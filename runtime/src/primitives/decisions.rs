//! Stored finding decisions — the `decisions:` list both audit records carry
//! beside their counts (spec 058).
//!
//! A finding routed or discarded in one run fires again in the next, because
//! detection is stateless: a routed finding keeps firing until the routed work
//! lands, and a discarded one keeps firing forever. Without a stored decision
//! every re-run would ask again, and a routed finding would come back
//! undispositioned and block `done` again. So each routed or discarded
//! decision is stored in the record of the command that made it, and a later
//! run counts a matching finding under the stored outcome without asking.
//!
//! The lifecycle mirrors review waivers (`framework/commands/review.md`,
//! §Per-run waiver processing): matched while the key fires, expired when an
//! unrestricted run no longer produces it, retained when the run did not
//! evaluate its source. A malformed entry is reported and never pruned.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::Result;
use crate::schema::primitives::{DecisionOutcome, DecisionRef};

/// The frontmatter key both records store decisions under.
pub(crate) const DECISIONS_KEY: &str = "decisions";

/// One stored decision with every field optional, so a malformed entry is a
/// reportable notice rather than a whole-record parse failure, and so a
/// re-render preserves the full entry. Unknown adopter-authored fields are
/// captured by `extra` and re-emitted verbatim, the §text-first-artifacts
/// open-schema rule the waiver list already follows (`RawWaiverFull`).
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub(crate) struct RawDecision {
    #[serde(default)]
    pub(crate) key: Option<String>,
    #[serde(default)]
    pub(crate) outcome: Option<String>,
    #[serde(default)]
    pub(crate) target: Option<String>,
    #[serde(default)]
    pub(crate) reason: Option<String>,
    #[serde(default, rename = "decided-at")]
    pub(crate) decided_at: Option<String>,
    #[serde(default, rename = "decided-by")]
    pub(crate) decided_by: Option<String>,
    #[serde(flatten)]
    pub(crate) extra: BTreeMap<String, serde_norway::Value>,
}

impl RawDecision {
    /// The first thing that makes this entry unusable, named as the field to
    /// fix, or `None` when it is well-formed.
    ///
    /// An outcome's companion is required with it — a route names its target,
    /// a discard its reason — because an entry missing it records a decision
    /// nobody can audit, which is no better than no decision.
    pub(crate) fn defect(&self) -> Option<&'static str> {
        let blank = |value: &Option<String>| value.as_deref().unwrap_or("").trim().is_empty();
        if blank(&self.key) {
            return Some("key");
        }
        let Some(outcome) = self.parsed_outcome() else {
            return Some("outcome");
        };
        if blank(&self.decided_at) {
            return Some("decided-at");
        }
        if blank(&self.decided_by) {
            return Some("decided-by");
        }
        match outcome {
            DecisionOutcome::Routed if blank(&self.target) => Some("target"),
            DecisionOutcome::Discarded if blank(&self.reason) => Some("reason"),
            _ => None,
        }
    }

    /// The recorded outcome, when it is one of the two a decision can store.
    fn parsed_outcome(&self) -> Option<DecisionOutcome> {
        match self.outcome.as_deref().map(str::trim) {
            Some("routed") => Some(DecisionOutcome::Routed),
            Some("discarded") => Some(DecisionOutcome::Discarded),
            _ => None,
        }
    }

    /// The entry as a [`DecisionRef`], when it is well-formed.
    pub(crate) fn to_ref(&self) -> Option<DecisionRef> {
        if self.defect().is_some() {
            return None;
        }
        let outcome = self.parsed_outcome()?;
        Some(DecisionRef {
            key: self.key.clone().unwrap_or_default(),
            outcome,
            target: match outcome {
                DecisionOutcome::Routed => self.target.clone(),
                DecisionOutcome::Discarded => None,
            },
            reason: match outcome {
                DecisionOutcome::Discarded => self.reason.clone(),
                DecisionOutcome::Routed => None,
            },
        })
    }
}

/// The decisions recorded in `feature_dir/file`'s frontmatter.
///
/// An absent file or key is an empty list; a list that will not parse is an
/// error, never an empty list (see [`super::read_recorded_list`]).
pub(crate) fn read_decisions(feature_dir: &Path, file: &str) -> Result<Vec<RawDecision>> {
    super::read_recorded_list(feature_dir, file, DECISIONS_KEY)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn write(dir: &Path, file: &str, frontmatter: &str) {
        std::fs::write(
            dir.join(file),
            format!("---\n{frontmatter}---\n\n# Record\n"),
        )
        .unwrap();
    }

    #[test]
    fn an_absent_file_or_key_is_an_empty_list() {
        let dir = tempfile::tempdir().unwrap();
        assert!(
            read_decisions(dir.path(), "analysis.md")
                .unwrap()
                .is_empty()
        );
        write(dir.path(), "analysis.md", "hard-fail: 0\n");
        assert!(
            read_decisions(dir.path(), "analysis.md")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_stored_decision_round_trips_with_its_extras() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "review.md",
            "decisions:\n\
             \u{20} - key: \"perf: a() is called in a loop — `src/a.rs`\"\n\
             \u{20}   outcome: routed\n\
             \u{20}   target: specs/031-x/scenarios/loop.md\n\
             \u{20}   decided-at: 2026-09-25T14:40:00Z\n\
             \u{20}   decided-by: dev@example.com\n\
             \u{20}   ticket: OPS-12\n",
        );
        let decisions = read_decisions(dir.path(), "review.md").unwrap();
        assert_eq!(decisions.len(), 1);
        let decision = &decisions[0];
        assert_eq!(decision.defect(), None);
        assert_eq!(
            decision.extra.get("ticket"),
            Some(&serde_norway::Value::String("OPS-12".into())),
            "an adopter-authored field must survive the parse"
        );
        assert_eq!(
            decision.to_ref(),
            Some(DecisionRef {
                key: "perf: a() is called in a loop — `src/a.rs`".into(),
                outcome: DecisionOutcome::Routed,
                target: Some("specs/031-x/scenarios/loop.md".into()),
                reason: None,
            })
        );
    }

    #[test]
    fn an_unparseable_list_is_an_error_not_an_empty_list() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "analysis.md", "decisions: not-a-list\n");
        assert!(
            read_decisions(dir.path(), "analysis.md").is_err(),
            "a list that silently read as empty would re-ask every settled question"
        );
    }

    #[test]
    fn each_missing_companion_is_its_own_defect() {
        let routed_without_target = RawDecision {
            key: Some("k".into()),
            outcome: Some("routed".into()),
            decided_at: Some("2026-09-25T00:00:00Z".into()),
            decided_by: Some("dev@example.com".into()),
            ..RawDecision::default()
        };
        assert_eq!(routed_without_target.defect(), Some("target"));

        let discarded_without_reason = RawDecision {
            outcome: Some("discarded".into()),
            reason: Some("  ".into()),
            ..routed_without_target.clone()
        };
        assert_eq!(discarded_without_reason.defect(), Some("reason"));

        let unknown_outcome = RawDecision {
            outcome: Some("fixed".into()),
            ..routed_without_target.clone()
        };
        assert_eq!(
            unknown_outcome.defect(),
            Some("outcome"),
            "fixed is never stored: a fixed finding stops firing"
        );

        let no_author = RawDecision {
            target: Some("specs/031-x/tasks.md".into()),
            decided_by: None,
            ..routed_without_target
        };
        assert_eq!(no_author.defect(), Some("decided-by"));
        assert_eq!(no_author.to_ref(), None);
    }
}
