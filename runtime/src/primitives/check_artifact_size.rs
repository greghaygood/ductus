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

use std::path::{Path, PathBuf};

use crate::primitives::prune_tasks::read_status;
use crate::primitives::{PrimitiveError, Result, read_text, rel_path, scenario_name_cmp};
use crate::schema::paths;
use crate::schema::primitives::{
    ArtifactFix, ArtifactKind, CheckArtifactSizeArgs, CheckArtifactSizeResult, DecisionsState,
    FixKind, OnDone, OversizedArtifact, ReadSizeThreshold, SkippedTarget, ThresholdSource,
};

/// The finding family `/{project}:analyze` records an oversized artifact
/// under, and the family a skipped subject is listed under.
pub(crate) const FAMILY: &str = "artifact-size";

/// The skip reason for a subject that exists but could not be measured — the
/// reason `check-artifacts` and `write-analysis` already count as could not be
/// read.
const UNREADABLE: &str = "artifact-unreadable";

/// The fixed subjects, in subject order. Scenarios follow them. `research.md`,
/// `review.md`, `analysis.md` and any file the project added are never
/// subjects: no command reads the first, and the runtime regenerates the two
/// records whole on every run, so no fix applies to them (spec 063).
const FIXED_SUBJECTS: [(&str, ArtifactKind); 4] = [
    ("spec.md", ArtifactKind::Spec),
    ("plan.md", ArtifactKind::Plan),
    ("tasks.md", ArtifactKind::Tasks),
    ("data-model.md", ArtifactKind::DataModel),
];

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

    let mut skipped = Vec::new();
    let mut examined: u32 = 0;
    let mut oversized = Vec::new();
    for (path, kind) in subjects(&feature_dir, repo, &mut skipped) {
        let rel = rel_path(&path, repo);
        let Some(bytes) = measure(&path, &rel, &mut skipped) else {
            continue;
        };
        examined += 1;
        let pages = bytes.div_ceil(threshold.bytes).max(1);
        if pages >= 2 {
            oversized.push(oversized_artifact(
                rel,
                kind,
                bytes,
                pages,
                threshold.bytes,
                &status,
            ));
        }
    }

    let mut notices = Vec::new();
    notices.extend(threshold_notice);
    Ok(CheckArtifactSizeResult {
        feature: args.feature.clone(),
        status,
        threshold,
        examined,
        oversized,
        skipped,
        decisions: DecisionsState::Absent,
        notices,
        path: rel_path(&feature_dir.join("spec.md"), repo),
    })
}

/// Every subject that exists, in subject order. A missing fixed subject is
/// left out silently: at `draft` a missing `plan.md` is a state, not a gap in
/// the check. A `scenarios/` directory that exists but cannot be listed is
/// one skipped entry, never an empty list, since a listing nobody could read
/// says nothing about what it holds.
fn subjects(
    feature_dir: &Path,
    repo: &Path,
    skipped: &mut Vec<SkippedTarget>,
) -> Vec<(PathBuf, ArtifactKind)> {
    let mut found: Vec<(PathBuf, ArtifactKind)> = FIXED_SUBJECTS
        .iter()
        .map(|(name, kind)| (feature_dir.join(name), *kind))
        .filter(|(path, _)| path.symlink_metadata().is_ok())
        .collect();
    let scenarios = feature_dir.join("scenarios");
    if !scenarios.is_dir() {
        return found;
    }
    let Ok(entries) = std::fs::read_dir(&scenarios) else {
        skip(skipped, rel_path(&scenarios, repo));
        return found;
    };
    let mut names: Vec<String> = entries
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| {
            Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        })
        .filter(|name| !scenarios.join(name).is_dir())
        .collect();
    names.sort_by(|a, b| scenario_name_cmp(a, b));
    found.extend(
        names
            .into_iter()
            .map(|name| (scenarios.join(name), ArtifactKind::Scenario)),
    );
    found
}

/// A subject's length on disk, or `None` after recording it skipped when it
/// cannot be opened for reading or is not a regular file. The length is the
/// file's metadata, read without the content: the bytes an agent's reader
/// receives, line endings and multi-byte characters included, at the cost of
/// one stat rather than a read of the very file that may be too large.
fn measure(path: &Path, rel: &str, skipped: &mut Vec<SkippedTarget>) -> Option<u64> {
    let measured = std::fs::File::open(path)
        .and_then(|file| file.metadata())
        .ok()
        .filter(std::fs::Metadata::is_file)
        .map(|metadata| metadata.len());
    if measured.is_none() {
        skip(skipped, rel.to_string());
    }
    measured
}

fn skip(skipped: &mut Vec<SkippedTarget>, path: String) {
    skipped.push(SkippedTarget {
        family: FAMILY.into(),
        reason: UNREADABLE.into(),
        path,
    });
}

/// The record for one subject over the threshold: its fixes, the warning the
/// commands print, and the finding message `/{project}:analyze` records.
fn oversized_artifact(
    path: String,
    kind: ArtifactKind,
    bytes: u64,
    pages: u64,
    threshold: u64,
    status: &str,
) -> OversizedArtifact {
    let done = status == "done";
    let fixes: Vec<ArtifactFix> = fix_plan(kind)
        .into_iter()
        .map(|(fix, text)| ArtifactFix {
            fix,
            text: text.to_string(),
            on_done: done.then(|| on_done(fix)),
        })
        .collect();
    let listed: Vec<String> = fixes
        .iter()
        .map(|fix| match fix.on_done {
            Some(effect) => format!("{} ({})", fix.text, effect_text(effect)),
            None => fix.text.clone(),
        })
        .collect();
    let warning = format!(
        "`{path}` is {bytes} bytes, over the {threshold}-byte read size ({pages} read pages), \
         so an agent may not read it in one call. Fixes: {}.",
        listed.join("; ")
    );
    let message = finding_message(&path, bytes, pages, threshold);
    OversizedArtifact {
        path,
        kind,
        bytes,
        pages,
        fixes,
        warning,
        message,
        decided: false,
        decision_key: None,
    }
}

/// The `artifact-size` finding message. Its format is a contract: the
/// decided rule parses a stored key back under it, so the numbers carry no
/// separators and the path is the only backticked span.
pub(crate) fn finding_message(path: &str, bytes: u64, pages: u64, threshold: u64) -> String {
    format!("`{path}` is {bytes} bytes, {pages} read pages at the {threshold}-byte read size")
}

/// The manual split route: no command splits a spec, so the text names the
/// steps and only commands that exist.
macro_rules! split_route {
    () => {
        "split the spec when a slice can reach done on its own, by hand: create the new \
         spec with /{project}:specify, move the content into it, and link it from this one \
         (no command splits a spec)"
    };
}

const SPLIT: &str = split_route!();

/// A `tasks.md` still over the threshold once pruned means the spec itself is
/// too big, so the spec's own fixes follow the prune.
const SPLIT_AFTER_PRUNE: &str = concat!(
    "if it is still over the read size after pruning, the spec itself is too big: ",
    split_route!()
);

/// The fixes for `kind`, in the spec's order.
fn fix_plan(kind: ArtifactKind) -> Vec<(FixKind, &'static str)> {
    match kind {
        ArtifactKind::Tasks => vec![
            (
                FixKind::Prune,
                "prune its spent task sections with /{project}:prune",
            ),
            (FixKind::Split, SPLIT_AFTER_PRUNE),
            (FixKind::Trim, "or trim spec.md"),
        ],
        ArtifactKind::Spec => vec![(FixKind::Split, SPLIT), (FixKind::Trim, "trim spec.md")],
        ArtifactKind::Plan => vec![
            (FixKind::Split, SPLIT),
            (
                FixKind::Trim,
                "trim plan.md: its code sketches, and the Affected Files rows for finished work",
            ),
        ],
        ArtifactKind::DataModel => vec![
            (FixKind::Split, SPLIT),
            (
                FixKind::Trim,
                "trim data-model.md, never into sub-files: review staleness tracks \
                 data-model.md by its exact name",
            ),
        ],
        ArtifactKind::Scenario => vec![(
            FixKind::Promote,
            "promote the scenario to its own spec with /{project}:specify (§scenario-promotion)",
        )],
    }
}

/// What `fix` does to a `done` spec (§spec-lifecycle): pruning `tasks.md`
/// changes no claim; splitting and promoting remove requirements from what the
/// spec asserts; a trim does only when it changes a claim.
fn on_done(fix: FixKind) -> OnDone {
    match fix {
        FixKind::Prune => OnDone::Never,
        FixKind::Split | FixKind::Promote => OnDone::Always,
        FixKind::Trim => OnDone::IfClaimChanges,
    }
}

fn effect_text(effect: OnDone) -> &'static str {
    match effect {
        OnDone::Never => "does not reopen this done spec",
        OnDone::Always => "reopens this done spec",
        OnDone::IfClaimChanges => "reopens this done spec unless the trim changes no claim",
    }
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

    /// A repo at `status` checking at a 100-byte threshold, holding `files`
    /// (path under the feature, byte count) filled with `x`.
    fn sized(status: &str, files: &[(&str, usize)]) -> TempDir {
        let tmp = repo(Some("[artifacts]\nread-size-bytes = 100\n"));
        let feature = tmp.path().join("specs/001-a");
        std::fs::write(
            feature.join("spec.md"),
            format!("---\nstatus: {status}\n---\n\n# 001 — A\n"),
        )
        .unwrap();
        for (path, bytes) in files {
            let path = feature.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "x".repeat(*bytes)).unwrap();
        }
        tmp
    }

    fn paths_of(result: &CheckArtifactSizeResult) -> Vec<&str> {
        result.oversized.iter().map(|o| o.path.as_str()).collect()
    }

    fn fix_kinds(artifact: &OversizedArtifact) -> Vec<FixKind> {
        artifact.fixes.iter().map(|f| f.fix).collect()
    }

    #[test]
    fn only_the_subjects_are_examined() {
        // Every non-subject is far over the threshold; none is reported.
        let tmp = sized(
            "draft",
            &[
                ("research.md", 5_000),
                ("review.md", 5_000),
                ("analysis.md", 5_000),
                ("notes.md", 5_000),
                ("scenarios/nested/deep.md", 5_000),
                ("scenarios/a.txt", 5_000),
            ],
        );
        let result = check(&tmp).unwrap();
        assert!(result.oversized.is_empty(), "{:?}", paths_of(&result));
        assert!(result.skipped.is_empty());
        // Only spec.md exists among the subjects.
        assert_eq!(result.examined, 1);
    }

    #[test]
    fn a_missing_subject_is_neither_reported_nor_skipped() {
        let tmp = sized("draft", &[("tasks.md", 50)]);
        let result = check(&tmp).unwrap();
        assert_eq!(result.examined, 2, "spec.md and tasks.md");
        assert!(result.oversized.is_empty());
        assert!(result.skipped.is_empty(), "{:?}", result.skipped);
    }

    #[cfg(unix)]
    #[test]
    fn an_unreadable_subject_is_skipped_never_clean() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = sized("draft", &[("plan.md", 5_000), ("scenarios/a.md", 10)]);
        let feature = tmp.path().join("specs/001-a");
        let plan = feature.join("plan.md");
        std::fs::set_permissions(&plan, std::fs::Permissions::from_mode(0o000)).unwrap();
        // A broken symlink is a scenario that exists and cannot be opened.
        std::os::unix::fs::symlink(feature.join("gone.md"), feature.join("scenarios/b.md"))
            .unwrap();
        if std::fs::File::open(&plan).is_ok() {
            return; // running as root: permissions do not bind
        }
        let result = check(&tmp).unwrap();
        std::fs::set_permissions(&plan, std::fs::Permissions::from_mode(0o644)).unwrap();
        let skipped: Vec<(&str, &str, &str)> = result
            .skipped
            .iter()
            .map(|s| (s.family.as_str(), s.reason.as_str(), s.path.as_str()))
            .collect();
        assert_eq!(
            skipped,
            [
                (
                    "artifact-size",
                    "artifact-unreadable",
                    "specs/001-a/plan.md"
                ),
                (
                    "artifact-size",
                    "artifact-unreadable",
                    "specs/001-a/scenarios/b.md"
                ),
            ]
        );
        assert!(result.oversized.is_empty());
        assert_eq!(result.examined, 2, "spec.md and scenarios/a.md");
    }

    #[cfg(unix)]
    #[test]
    fn an_unlistable_scenarios_directory_is_skipped() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = sized("draft", &[("scenarios/a.md", 5_000)]);
        let scenarios = tmp.path().join("specs/001-a/scenarios");
        std::fs::set_permissions(&scenarios, std::fs::Permissions::from_mode(0o000)).unwrap();
        let listable = std::fs::read_dir(&scenarios).is_ok();
        let result = check(&tmp);
        std::fs::set_permissions(&scenarios, std::fs::Permissions::from_mode(0o755)).unwrap();
        if listable {
            return; // running as root
        }
        let result = result.unwrap();
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(result.skipped[0].path, "specs/001-a/scenarios");
        assert!(result.oversized.is_empty());
    }

    #[test]
    fn the_size_is_the_bytes_on_disk() {
        let tmp = sized("draft", &[]);
        // 60 CRLF lines of one character and a multi-byte dash: 3 bytes a
        // line plus 3 for the dash, counted as written.
        let body = format!("{}—", "x\r\n".repeat(60));
        let plan = tmp.path().join("specs/001-a/plan.md");
        std::fs::write(&plan, &body).unwrap();
        let result = check(&tmp).unwrap();
        assert_eq!(result.oversized.len(), 1);
        assert_eq!(result.oversized[0].bytes, 183);
        assert_eq!(result.oversized[0].bytes, body.len() as u64);
    }

    #[test]
    fn a_file_at_the_threshold_is_not_reported_and_one_byte_over_is() {
        let at = check(&sized("draft", &[("plan.md", 100)])).unwrap();
        assert!(at.oversized.is_empty());
        let over = check(&sized("draft", &[("plan.md", 101)])).unwrap();
        assert_eq!(over.oversized.len(), 1);
        assert_eq!(over.oversized[0].pages, 2);
        let three = check(&sized("draft", &[("plan.md", 201)])).unwrap();
        assert_eq!(three.oversized[0].pages, 3);
    }

    #[test]
    fn subjects_come_in_subject_order_with_scenarios_last() {
        let tmp = sized(
            "draft",
            &[
                ("data-model.md", 500),
                ("tasks.md", 500),
                ("plan.md", 500),
                ("scenarios/B.md", 500),
                ("scenarios/a.md", 500),
            ],
        );
        std::fs::write(
            tmp.path().join("specs/001-a/spec.md"),
            format!("---\nstatus: draft\n---\n\n{}", "x".repeat(500)),
        )
        .unwrap();
        let result = check(&tmp).unwrap();
        assert_eq!(
            paths_of(&result),
            [
                "specs/001-a/spec.md",
                "specs/001-a/plan.md",
                "specs/001-a/tasks.md",
                "specs/001-a/data-model.md",
                "specs/001-a/scenarios/a.md",
                "specs/001-a/scenarios/B.md",
            ]
        );
        assert_eq!(result.examined, 6);
    }

    #[test]
    fn each_kind_names_the_fixes_from_the_spec_table() {
        let tmp = sized(
            "draft",
            &[
                ("plan.md", 500),
                ("tasks.md", 500),
                ("data-model.md", 500),
                ("scenarios/a.md", 500),
            ],
        );
        let result = check(&tmp).unwrap();
        let by_kind = |kind| result.oversized.iter().find(|o| o.kind == kind).unwrap();
        assert_eq!(
            fix_kinds(by_kind(ArtifactKind::Tasks)),
            [FixKind::Prune, FixKind::Split, FixKind::Trim]
        );
        assert_eq!(
            fix_kinds(by_kind(ArtifactKind::Plan)),
            [FixKind::Split, FixKind::Trim]
        );
        assert_eq!(
            fix_kinds(by_kind(ArtifactKind::DataModel)),
            [FixKind::Split, FixKind::Trim]
        );
        assert_eq!(
            fix_kinds(by_kind(ArtifactKind::Scenario)),
            [FixKind::Promote]
        );
        let tasks = by_kind(ArtifactKind::Tasks);
        assert!(tasks.fixes[0].text.contains("/{project}:prune"));
        assert!(
            tasks.fixes[1]
                .text
                .contains("still over the read size after pruning")
        );
        let plan = by_kind(ArtifactKind::Plan);
        assert!(plan.fixes[1].text.contains("code sketches"));
        assert!(plan.fixes[1].text.contains("Affected Files"));
        let data_model = by_kind(ArtifactKind::DataModel);
        assert!(data_model.fixes[1].text.contains("never into sub-files"));
        let scenario = by_kind(ArtifactKind::Scenario);
        assert!(scenario.fixes[0].text.contains("its own spec"));
        // No fix is a discard: only /{project}:analyze offers one.
        for artifact in &result.oversized {
            assert!(
                artifact.fixes.iter().all(|f| !f.text.contains("discard")),
                "{}",
                artifact.warning
            );
        }
    }

    #[test]
    fn spec_md_is_split_or_trimmed() {
        let tmp = sized("draft", &[]);
        std::fs::write(
            tmp.path().join("specs/001-a/spec.md"),
            format!("---\nstatus: draft\n---\n\n{}", "x".repeat(500)),
        )
        .unwrap();
        let result = check(&tmp).unwrap();
        assert_eq!(result.oversized[0].kind, ArtifactKind::Spec);
        assert_eq!(
            fix_kinds(&result.oversized[0]),
            [FixKind::Split, FixKind::Trim]
        );
    }

    #[test]
    fn the_effect_on_done_is_named_only_at_done() {
        let files = [("plan.md", 500), ("tasks.md", 500), ("scenarios/a.md", 500)];
        for status in ["draft", "clarified", "planned", "in-progress"] {
            let result = check(&sized(status, &files)).unwrap();
            for artifact in &result.oversized {
                assert!(
                    artifact.fixes.iter().all(|f| f.on_done.is_none()),
                    "{status}"
                );
                assert!(!artifact.warning.contains("reopen"), "{}", artifact.warning);
            }
        }
        let result = check(&sized("done", &files)).unwrap();
        let effects: Vec<(FixKind, Option<OnDone>)> = result
            .oversized
            .iter()
            .flat_map(|o| o.fixes.iter().map(|f| (f.fix, f.on_done)))
            .collect();
        assert_eq!(
            effects,
            [
                (FixKind::Split, Some(OnDone::Always)),
                (FixKind::Trim, Some(OnDone::IfClaimChanges)),
                (FixKind::Prune, Some(OnDone::Never)),
                (FixKind::Split, Some(OnDone::Always)),
                (FixKind::Trim, Some(OnDone::IfClaimChanges)),
                (FixKind::Promote, Some(OnDone::Always)),
            ]
        );
        let tasks = &result.oversized[1].warning;
        assert!(
            tasks.contains("(does not reopen this done spec)"),
            "{tasks}"
        );
        assert!(tasks.contains("(reopens this done spec)"), "{tasks}");
        assert!(
            tasks.contains("(reopens this done spec unless the trim changes no claim)"),
            "{tasks}"
        );
    }

    #[test]
    fn warnings_say_may_never_will_and_name_only_real_commands() {
        let files = [
            ("plan.md", 500),
            ("tasks.md", 500),
            ("data-model.md", 500),
            ("scenarios/a.md", 500),
        ];
        for status in ["draft", "done"] {
            let result = check(&sized(status, &files)).unwrap();
            assert_eq!(result.oversized.len(), 4);
            for artifact in &result.oversized {
                let warning = &artifact.warning;
                assert!(warning.contains("may not read it in one call"), "{warning}");
                let words = warning
                    .split(|c: char| !c.is_alphanumeric())
                    .map(str::to_lowercase);
                assert!(words.clone().all(|w| w != "will"), "{warning}");
                // Every command named is one that exists.
                for (_, rest) in warning
                    .match_indices("/{project}:")
                    .map(|(i, _)| (i, &warning[i + "/{project}:".len()..]))
                {
                    assert!(
                        rest.starts_with("specify") || rest.starts_with("prune"),
                        "{warning}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_warning_and_message_name_the_size_threshold_and_pages() {
        let result = check(&sized("draft", &[("plan.md", 250)])).unwrap();
        let plan = &result.oversized[0];
        assert_eq!(
            plan.message,
            "`specs/001-a/plan.md` is 250 bytes, 3 read pages at the 100-byte read size"
        );
        assert!(plan.warning.starts_with(
            "`specs/001-a/plan.md` is 250 bytes, over the 100-byte read size (3 read pages)"
        ));
        assert!(!plan.decided && plan.decision_key.is_none());
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
