//! `retarget-sessions` — after a fold or consolidation removes a feature
//! directory, re-point or clear every session that names it (spec 062).
//!
//! §concurrent-features forbids leaving a session pointing at a removed
//! directory. Before spec 062 there was one session, and `/{project}:fold`
//! re-targeted it with `write-session` while `/{project}:consolidate` read it
//! by hand to decide whether to clear it. With a target per process, the
//! sessions held by other processes in the same working tree are reachable
//! too, so the rule extends to all of them — and that sweep, the only
//! operation that writes another process's target, lives here rather than as
//! a mode of `write-session`, which keeps meaning "write *my* session".

use std::path::Path;
use std::time::SystemTime;

use crate::primitives::{PrimitiveError, Result, rel_path, validate_no_traversal};
use crate::schema::primitives::{
    RetargetCause, RetargetSessionsArgs, RetargetSessionsResult, SessionTarget,
};
use crate::session::{self, Identity, RemovalCause};

const PRIMITIVE: &str = "retarget-sessions";

/// Execute the `retarget-sessions` primitive against `repo`, as the process
/// whose environment this runtime inherited.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] / [`PrimitiveError::MissingArgument`]
/// when the arguments do not describe exactly one of a fold with a new target
/// or a consolidation with `clear`; [`PrimitiveError::InvalidPath`] on a
/// traversing path; [`PrimitiveError::Io`] on a failed lock, read or write.
pub fn run(args: &RetargetSessionsArgs, repo: &Path) -> Result<RetargetSessionsResult> {
    let identity = session::identity_from_process_env()?;
    run_as(args, repo, identity.as_ref(), SystemTime::now())
}

/// Retarget as `identity` at `now`: the one path [`run`] and the tests share.
pub(crate) fn run_as(
    args: &RetargetSessionsArgs,
    repo: &Path,
    identity: Option<&Identity>,
    now: SystemTime,
) -> Result<RetargetSessionsResult> {
    let (to, cause) = validate(args)?;
    let outcome = session::retarget(repo, identity, &args.from, to.as_ref(), cause, now)?;
    Ok(RetargetSessionsResult {
        retargeted: outcome.retargeted,
        cleared: outcome.cleared,
        unreadable: outcome
            .unreadable
            .iter()
            .map(|path| rel_path(path, repo))
            .collect(),
    })
}

/// A fold names a new target and does not clear; a consolidation clears and
/// names none. Anything else describes neither removal.
fn validate(args: &RetargetSessionsArgs) -> Result<(Option<SessionTarget>, RemovalCause)> {
    let invalid = |argument: &str, reason: &str| PrimitiveError::InvalidArgument {
        primitive: PRIMITIVE.into(),
        argument: argument.into(),
        reason: reason.into(),
    };
    let missing = |argument: &str, reason: &str| PrimitiveError::MissingArgument {
        primitive: PRIMITIVE.into(),
        argument: argument.into(),
        reason: reason.into(),
    };

    if args.from.trim().is_empty() {
        return Err(missing("from", "name the removed feature directory"));
    }
    for value in [&args.path, &args.scenario_path].into_iter().flatten() {
        validate_no_traversal(value)?;
    }
    if args.scenario.is_some() != args.scenario_path.is_some() {
        let absent = if args.scenario.is_some() {
            "scenario-path"
        } else {
            "scenario"
        };
        return Err(missing(
            absent,
            "`scenario` and `scenario-path` are supplied together",
        ));
    }

    match args.cause {
        RetargetCause::Fold => {
            if args.clear {
                return Err(invalid(
                    "clear",
                    "a fold re-targets sessions at the upstream spec; only a consolidation clears",
                ));
            }
            let (Some(feature), Some(path)) = (&args.feature, &args.path) else {
                let absent = if args.feature.is_none() {
                    "feature"
                } else {
                    "path"
                };
                return Err(missing(
                    absent,
                    "a fold needs the upstream target: supply `feature` and `path`",
                ));
            };
            Ok((
                Some(SessionTarget {
                    feature: feature.clone(),
                    path: path.clone(),
                    scenario: args.scenario.clone(),
                    scenario_path: args.scenario_path.clone(),
                }),
                RemovalCause::Fold,
            ))
        }
        RetargetCause::Consolidate => {
            if !args.clear {
                return Err(missing(
                    "clear",
                    "a consolidation clears the sessions naming the removed spec",
                ));
            }
            if args.feature.is_some() || args.path.is_some() || args.scenario.is_some() {
                return Err(invalid(
                    "feature",
                    "a consolidation clears; it names no new target",
                ));
            }
            Ok((None, RemovalCause::Consolidate))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::primitives::resolve_session;
    use crate::primitives::write_session;
    use crate::schema::primitives::{SessionSource, WriteSessionArgs};
    use std::fs;
    use std::time::{Duration, UNIX_EPOCH};
    use tempfile::tempdir;

    fn now() -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_790_683_200)
    }

    fn fold(from: &str, to: &str) -> RetargetSessionsArgs {
        RetargetSessionsArgs {
            from: from.into(),
            cause: RetargetCause::Fold,
            feature: Some(to.into()),
            path: Some(format!("specs/{to}")),
            scenario: None,
            scenario_path: None,
            clear: false,
        }
    }

    fn consolidate(from: &str) -> RetargetSessionsArgs {
        RetargetSessionsArgs {
            from: from.into(),
            cause: RetargetCause::Consolidate,
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            clear: true,
        }
    }

    fn target_as(repo: &Path, name: &str, feature: &str) {
        let args = WriteSessionArgs {
            feature: Some(feature.into()),
            path: Some(format!("specs/{feature}")),
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: false,
        };
        let who = Identity::named(name).unwrap();
        write_session::run_as(&args, repo, Some(&who), now()).unwrap();
    }

    #[test]
    fn a_fold_moves_every_session_on_the_folded_spec() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        target_as(tmp.path(), "a", "1234.1-b");
        target_as(tmp.path(), "b", "1234.1-b");
        let acting = Identity::named("a").unwrap();
        let result = run_as(&fold("1234.1-b", "055-a"), tmp.path(), Some(&acting), now()).unwrap();
        assert_eq!(result.retargeted, ["default", "a", "b"]);

        let b = Identity::named("b").unwrap();
        let resolved = resolve_session::run_as(tmp.path(), Some(&b), now()).unwrap();
        assert_eq!(resolved.target.unwrap().feature, "055-a");
    }

    #[test]
    fn a_consolidation_clears_every_session_on_the_removed_spec() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        target_as(tmp.path(), "a", "058-gone");
        let result = run_as(&consolidate("058-gone"), tmp.path(), None, now()).unwrap();
        assert_eq!(result.cleared, ["default", "a"]);
        let a = Identity::named("a").unwrap();
        let resolved = resolve_session::run_as(tmp.path(), Some(&a), now()).unwrap();
        assert_eq!(resolved.source, SessionSource::Cleared);
    }

    #[test]
    fn nothing_naming_the_spec_is_a_clean_no_op() {
        let tmp = tempdir().unwrap();
        let result = run_as(&consolidate("058-gone"), tmp.path(), None, now()).unwrap();
        assert_eq!(result, RetargetSessionsResult::default());
        assert!(!tmp.path().join(".ductus").exists(), "nothing created");
    }

    #[test]
    fn a_fold_without_a_target_or_with_clear_is_refused() {
        let mut no_target = fold("x", "y");
        no_target.feature = None;
        no_target.path = None;
        assert!(matches!(
            validate(&no_target),
            Err(PrimitiveError::MissingArgument { .. })
        ));

        let mut fold_clear = fold("x", "y");
        fold_clear.clear = true;
        assert!(matches!(
            validate(&fold_clear),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
    }

    #[test]
    fn a_consolidation_without_clear_or_with_a_target_is_refused() {
        let mut no_clear = consolidate("x");
        no_clear.clear = false;
        assert!(matches!(
            validate(&no_clear),
            Err(PrimitiveError::MissingArgument { .. })
        ));

        let mut with_target = consolidate("x");
        with_target.feature = Some("y".into());
        assert!(matches!(
            validate(&with_target),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
    }

    #[test]
    fn traversing_paths_and_a_half_scenario_are_refused() {
        let mut traversal = fold("x", "y");
        traversal.path = Some("../elsewhere".into());
        assert!(matches!(
            validate(&traversal),
            Err(PrimitiveError::InvalidPath { .. })
        ));

        let missing_argument = |args: &RetargetSessionsArgs| match validate(args) {
            Err(PrimitiveError::MissingArgument { argument, .. }) => argument,
            other => panic!("expected MissingArgument, got {other:?}"),
        };
        let mut half = fold("x", "y");
        half.scenario = Some("s".into());
        assert_eq!(missing_argument(&half), "scenario-path");
        let mut other_half = fold("x", "y");
        other_half.scenario_path = Some("specs/y/scenarios/s.md".into());
        assert_eq!(missing_argument(&other_half), "scenario");
        let mut no_path = fold("x", "y");
        no_path.path = None;
        assert_eq!(missing_argument(&no_path), "path");

        let mut empty = consolidate(" ");
        empty.clear = true;
        assert!(validate(&empty).is_err());
    }
}
