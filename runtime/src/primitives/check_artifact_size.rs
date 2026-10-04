//! `check-artifact-size` — warn when a spec artifact is larger than an agent
//! can read in one call (spec 063).
//!
//! An agent's file reader returns only part of a file past a host-specific
//! cap, and a command that reads a spec artifact once, at setup, then works
//! from part of it without saying so — the failure §grounding's *A partial
//! read is not a read* names. This primitive measures a feature's subject
//! artifacts against a configured threshold and reports each one over it with
//! the fixes for its kind, so the warning arrives where fixing it is cheapest.
//! `/{project}:clarify` and `/{project}:plan` print the warnings and never gate
//! on them; `/{project}:analyze` records each as an advisory `artifact-size`
//! finding.
//!
//! **Bytes, not tokens.** The runtime has no tokenizer, and the cap that
//! matters differs per host and per host setting, so a file's length on disk
//! is the measurable proxy and the threshold is a heuristic the project may
//! set. The warning therefore says a file *may* not be read in one call,
//! never that it *will* be truncated.
//!
//! **Read-only.** Nothing is written in any mode.

use std::path::Path;

use crate::primitives::prune_tasks::read_status;
use crate::primitives::{PrimitiveError, Result, read_text, rel_path};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckArtifactSizeArgs, CheckArtifactSizeResult, DecisionsState, ReadSizeThreshold,
    ThresholdSource,
};

/// The threshold when `[artifacts] read-size-bytes` is unset: the round
/// number below the smallest file recorded past a host's read cap (spec 063,
/// Resolved Questions).
pub(crate) const DEFAULT_READ_SIZE_BYTES: u64 = 50_000;

/// The config section and key the threshold is read from.
const SECTION: &str = "artifacts";
const KEY: &str = "read-size-bytes";

/// Measure `args.feature`'s subject artifacts against the configured read
/// size.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` would escape the spec root.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
/// - [`PrimitiveError::MissingSpecFile`] / [`PrimitiveError::StatusFieldMissing`]
///   / [`PrimitiveError::MissingFrontmatter`] /
///   [`PrimitiveError::UnclosedFrontmatter`] / [`PrimitiveError::Yaml`] when
///   the spec status cannot be read: every fix's effect on a `done` spec turns
///   on it.
/// - [`PrimitiveError::Toml`] when the config file does not parse, as for
///   every config reader: the project's configuration is broken for every
///   command, and a size check is not the place to paper over that.
/// - [`PrimitiveError::Io`] when the config file exists but cannot be read.
pub fn run(args: &CheckArtifactSizeArgs, repo: &Path) -> Result<CheckArtifactSizeResult> {
    super::validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let status = read_status(&feature_dir, &root, &args.feature)?;
    let (threshold, threshold_notice) = resolve_threshold(repo)?;

    let mut notices = Vec::new();
    notices.extend(threshold_notice);
    Ok(CheckArtifactSizeResult {
        feature: args.feature.clone(),
        status,
        threshold,
        examined: 0,
        oversized: Vec::new(),
        skipped: Vec::new(),
        decisions: DecisionsState::Absent,
        notices,
        path: rel_path(&feature_dir.join("spec.md"), repo),
    })
}

/// The threshold to check at, and the notice an invalid setting owes.
///
/// The file is found through the config ladder every reader uses. The key is
/// read as a raw value so a wrong type is reported rather than failing the
/// parse: anything but a positive integer is invalid, and the check runs at
/// the default and says so. A non-table `artifacts` is invalid the same way,
/// since it cannot hold the key.
fn resolve_threshold(repo: &Path) -> Result<(ReadSizeThreshold, Option<String>)> {
    let default = |source| ReadSizeThreshold {
        bytes: DEFAULT_READ_SIZE_BYTES,
        source,
        rejected: None,
    };
    let (path, name) = paths::resolve_config(repo);
    if !path.is_file() {
        return Ok((default(ThresholdSource::Default), None));
    }
    let content = read_text(&path)?;
    let parsed: toml::Table =
        toml::from_str(&content).map_err(|source| PrimitiveError::Toml { path, source })?;
    let (setting, value) = match parsed.get(SECTION) {
        None => return Ok((default(ThresholdSource::Default), None)),
        Some(toml::Value::Table(section)) => match section.get(KEY) {
            None => return Ok((default(ThresholdSource::Default), None)),
            Some(value) => {
                if let toml::Value::Integer(bytes) = value
                    && let Ok(bytes) = u64::try_from(*bytes)
                    && bytes > 0
                {
                    return Ok((
                        ReadSizeThreshold {
                            bytes,
                            source: ThresholdSource::Config,
                            rejected: None,
                        },
                        None,
                    ));
                }
                (format!("[{SECTION}] {KEY}"), value)
            }
        },
        // Whatever it holds, a non-table `artifacts` cannot carry the key.
        Some(other) => (SECTION.to_string(), other),
    };
    let rejected = value.to_string();
    let notice = format!(
        "`{setting}` in {name} is {rejected}, not a positive whole number of bytes; \
         checked at the default of {DEFAULT_READ_SIZE_BYTES} bytes instead"
    );
    Ok((
        ReadSizeThreshold {
            bytes: DEFAULT_READ_SIZE_BYTES,
            source: ThresholdSource::Invalid,
            rejected: Some(rejected),
        },
        Some(notice),
    ))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use tempfile::TempDir;

    /// A repo holding one draft spec, and `config` at `.ductus/config.toml`
    /// when given.
    fn repo(config: Option<&str>) -> TempDir {
        let tmp = TempDir::new().unwrap();
        let feature = tmp.path().join("specs/001-a");
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(
            feature.join("spec.md"),
            "---\nstatus: draft\n---\n\n# 001 — A\n",
        )
        .unwrap();
        if let Some(body) = config {
            std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
            std::fs::write(tmp.path().join(".ductus/config.toml"), body).unwrap();
        }
        tmp
    }

    fn check(tmp: &TempDir) -> Result<CheckArtifactSizeResult> {
        run(
            &CheckArtifactSizeArgs {
                feature: "001-a".into(),
            },
            tmp.path(),
        )
    }

    #[test]
    fn no_config_file_is_the_default() {
        let result = check(&repo(None)).unwrap();
        assert_eq!(result.threshold.bytes, 50_000);
        assert_eq!(result.threshold.source, ThresholdSource::Default);
        assert!(result.threshold.rejected.is_none());
        assert!(result.notices.is_empty());
        assert_eq!(result.status, "draft");
    }

    #[test]
    fn an_unset_key_is_the_default() {
        for config in ["[paths]\nspecs-root = \"specs\"\n", "[artifacts]\n"] {
            let result = check(&repo(Some(config))).unwrap();
            assert_eq!(
                result.threshold.source,
                ThresholdSource::Default,
                "{config}"
            );
            assert_eq!(result.threshold.bytes, 50_000);
            assert!(result.notices.is_empty());
        }
    }

    #[test]
    fn a_positive_integer_is_used() {
        let result = check(&repo(Some("[artifacts]\nread-size-bytes = 120000\n"))).unwrap();
        assert_eq!(result.threshold.bytes, 120_000);
        assert_eq!(result.threshold.source, ThresholdSource::Config);
        assert!(result.notices.is_empty());
        // Lowering it is as valid as raising it.
        let result = check(&repo(Some("[artifacts]\nread-size-bytes = 1\n"))).unwrap();
        assert_eq!(result.threshold.bytes, 1);
    }

    #[test]
    fn every_invalid_shape_falls_back_to_the_default_and_says_so() {
        for (value, rejected) in [
            ("\"50kb\"", "\"50kb\""),
            ("50000.5", "50000.5"),
            ("0", "0"),
            ("-1", "-1"),
            ("[50000]", "[50000]"),
            ("true", "true"),
        ] {
            let config = format!("[artifacts]\nread-size-bytes = {value}\n");
            let result = check(&repo(Some(&config))).unwrap();
            assert_eq!(result.threshold.bytes, 50_000, "{value}");
            assert_eq!(result.threshold.source, ThresholdSource::Invalid, "{value}");
            assert_eq!(result.threshold.rejected.as_deref(), Some(rejected));
            assert_eq!(result.notices.len(), 1, "{value}");
            let notice = &result.notices[0];
            assert!(notice.contains(rejected), "{notice}");
            assert!(notice.contains("[artifacts] read-size-bytes"), "{notice}");
            assert!(notice.contains(".ductus/config.toml"), "{notice}");
            assert!(notice.contains("50000"), "{notice}");
        }
    }

    #[test]
    fn a_non_table_section_is_invalid() {
        let result = check(&repo(Some("artifacts = 50000\n"))).unwrap();
        assert_eq!(result.threshold.source, ThresholdSource::Invalid);
        assert_eq!(result.threshold.rejected.as_deref(), Some("50000"));
        assert!(
            result.notices[0].contains("`artifacts`"),
            "{}",
            result.notices[0]
        );
    }

    #[test]
    fn an_unparseable_config_file_is_an_error() {
        let err = check(&repo(Some("[artifacts\nread-size-bytes = 1\n"))).unwrap_err();
        assert!(matches!(err, PrimitiveError::Toml { .. }), "{err:?}");
    }

    #[test]
    fn a_missing_feature_is_refused() {
        let tmp = repo(None);
        let err = run(
            &CheckArtifactSizeArgs {
                feature: "999-none".into(),
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(
            matches!(err, PrimitiveError::FeatureNotFound { .. }),
            "{err:?}"
        );
    }
}
