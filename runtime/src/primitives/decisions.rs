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

use crate::primitives::{PrimitiveError, Result, flatten_line};
use crate::schema::primitives::{
    DecisionOutcome, DecisionRef, Disposition, DispositionOutcome, Dispositions,
};

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

/// The decision `disposition` stores under `key`, or `None` for a fixed or
/// undispositioned finding, which stores nothing.
///
/// The one conversion both writers use, so an observation and an analyze
/// finding store their decisions under one policy: the key and the companion
/// text pass through [`flatten_line`], the same normalization
/// `process-decisions` applies to the keys a host fires, so a stored key and a
/// fired key for the same finding agree.
pub(crate) fn decision_for(disposition: &Disposition, key: &str) -> Option<DecisionRef> {
    let companion = |value: &Option<String>| value.as_deref().map(flatten_line);
    let (outcome, target, reason) = match disposition.outcome {
        DispositionOutcome::Routed => (
            DecisionOutcome::Routed,
            companion(&disposition.target),
            None,
        ),
        DispositionOutcome::Discarded => (
            DecisionOutcome::Discarded,
            None,
            companion(&disposition.reason),
        ),
        DispositionOutcome::Fixed | DispositionOutcome::Undispositioned => return None,
    };
    Some(DecisionRef {
        key: flatten_line(key),
        outcome,
        target,
        reason,
    })
}

/// Append a record's `dispositions:` map — always, all four counts, because
/// its absence has a meaning: a record without it predates dispositions, and
/// the gate reads that absence as a record to re-run. Shared by both writers,
/// so the two records carry one shape.
pub(crate) fn render_dispositions(block: &mut String, counts: Dispositions) {
    use std::fmt::Write as _;
    let _ = writeln!(block, "dispositions:");
    let _ = writeln!(block, "  fixed: {}", counts.fixed);
    let _ = writeln!(block, "  routed: {}", counts.routed);
    let _ = writeln!(block, "  discarded: {}", counts.discarded);
    let _ = writeln!(block, "  undispositioned: {}", counts.undispositioned);
}

/// The suffix a report renders beside a finding for its disposition — fixed,
/// routed with its target in a code span, discarded with its reason, or
/// undispositioned, each outcome in bold — with the companion text passed
/// through `plain`, each writer's own normalization for its report body.
pub(crate) fn disposition_suffix(
    disposition: &Disposition,
    plain: impl Fn(&str) -> String,
) -> String {
    let companion = |value: &Option<String>| plain(value.as_deref().unwrap_or(""));
    match disposition.outcome {
        DispositionOutcome::Fixed => "**fixed**".to_string(),
        DispositionOutcome::Routed => format!("**routed** to `{}`", companion(&disposition.target)),
        DispositionOutcome::Discarded => {
            format!("**discarded**: {}", companion(&disposition.reason))
        }
        DispositionOutcome::Undispositioned => "**undispositioned**".to_string(),
    }
}

/// Why [`merge`] refused a run's decisions.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum MergeError {
    /// A new decision exists and no `decided-by` was supplied.
    DecidedByMissing,
    /// Two findings sharing this key were given different decisions. One key
    /// is one finding and gets one disposition (spec 058), so the writer
    /// refuses rather than choosing between them.
    ConflictingDecisions(String),
}

impl MergeError {
    /// The refusal as the writer's `InvalidArgument`: `primitive` names the
    /// writer, `list` the argument its findings arrived in, and `noun` one of
    /// them, for the message.
    pub(crate) fn into_error(self, primitive: &str, list: &str, noun: &str) -> PrimitiveError {
        let (argument, reason) = match self {
            Self::DecidedByMissing => (
                "decided-by".to_string(),
                format!("required when {noun} is newly routed or discarded"),
            ),
            Self::ConflictingDecisions(key) => (
                list.to_string(),
                format!(
                    "two entries share the key `{key}` but were given different decisions; \
                     one key is one finding and gets one disposition"
                ),
            ),
        };
        PrimitiveError::InvalidArgument {
            primitive: primitive.into(),
            argument,
            reason,
        }
    }
}

/// The decisions recorded in `feature_dir/file`'s frontmatter.
///
/// An absent file or key is an empty list; a list that will not parse is an
/// error, never an empty list (see [`super::read_recorded_list`]).
pub(crate) fn read_decisions(feature_dir: &Path, file: &str) -> Result<Vec<RawDecision>> {
    super::read_recorded_list(feature_dir, file, DECISIONS_KEY)
}

/// The `decisions:` list a writer records.
///
/// Every stored entry survives unless this run expired it or decided its key
/// afresh; a malformed entry always survives, because an entry that cannot be
/// read cannot be proven dead. This run's decisions are first collapsed to one
/// per key — two findings sharing a key are one finding (spec 058) — and each
/// is then either a **match**, a well-formed stored entry with the same key and
/// outcome, kept byte-for-byte with its original companion, `decided-at`, and
/// `decided-by`; or **new**, stamped with `decided_at` and `decided_by`.
/// Matching on key and outcome rather than on the companion's wording is the
/// data model's rule: a host that restates a matched discard's reason, or
/// normalizes a route's target path, has not made a new decision.
///
/// # Errors
///
/// - [`MergeError::ConflictingDecisions`] when two of this run's decisions
///   share a key but differ, since one key gets one disposition.
/// - [`MergeError::DecidedByMissing`] when a new decision exists and
///   `decided_by` is absent or blank: a decision nobody can attribute is not
///   auditable, so the writer refuses rather than storing one.
pub(crate) fn merge(
    stored: Vec<RawDecision>,
    expired: &[DecisionRef],
    decided: &[DecisionRef],
    decided_at: &str,
    decided_by: Option<&str>,
) -> std::result::Result<Vec<RawDecision>, MergeError> {
    let mut once: Vec<&DecisionRef> = Vec::new();
    for decision in decided {
        match once.iter().find(|seen| seen.key == decision.key) {
            Some(seen) if *seen != decision => {
                return Err(MergeError::ConflictingDecisions(decision.key.clone()));
            }
            Some(_) => {}
            None => once.push(decision),
        }
    }
    let fresh: Vec<&DecisionRef> = once
        .into_iter()
        .filter(|decision| {
            !stored.iter().any(|entry| {
                entry.to_ref().is_some_and(|stored| {
                    stored.key == decision.key && stored.outcome == decision.outcome
                })
            })
        })
        .collect();
    let author = decided_by.map(str::trim).filter(|who| !who.is_empty());
    if !fresh.is_empty() && author.is_none() {
        return Err(MergeError::DecidedByMissing);
    }
    let dropped = |entry: &RawDecision| {
        let Some(well_formed) = entry.to_ref() else {
            return false; // malformed entries are never pruned
        };
        expired.iter().any(|gone| gone.key == well_formed.key)
            || fresh.iter().any(|new| new.key == well_formed.key)
    };
    let mut merged: Vec<RawDecision> = stored.into_iter().filter(|entry| !dropped(entry)).collect();
    for decision in fresh {
        merged.push(RawDecision {
            key: Some(decision.key.clone()),
            outcome: Some(
                match decision.outcome {
                    DecisionOutcome::Routed => "routed",
                    DecisionOutcome::Discarded => "discarded",
                }
                .to_string(),
            ),
            target: decision.target.clone(),
            reason: decision.reason.clone(),
            decided_at: Some(decided_at.to_string()),
            decided_by: author.map(str::to_string),
            extra: BTreeMap::new(),
        });
    }
    Ok(merged)
}

/// Append the `decisions:` list to a rendered record, known fields first and
/// every adopter-authored extra after them, verbatim — the waiver list's
/// rendering rules, shared rather than copied.
pub(crate) fn render(block: &mut String, decisions: &[RawDecision]) {
    use std::fmt::Write as _;
    if decisions.is_empty() {
        return;
    }
    let _ = writeln!(block, "{DECISIONS_KEY}:");
    for decision in decisions {
        let fields = [
            ("key", decision.key.as_deref()),
            ("outcome", decision.outcome.as_deref()),
            ("target", decision.target.as_deref()),
            ("reason", decision.reason.as_deref()),
            ("decided-at", decision.decided_at.as_deref()),
            ("decided-by", decision.decided_by.as_deref()),
        ];
        let mut first = true;
        let mut indent = || {
            let indent = if first { "  - " } else { "    " };
            first = false;
            indent
        };
        for (key, value) in fields {
            if let Some(value) = value {
                let _ = writeln!(
                    block,
                    "{}{key}: {}",
                    indent(),
                    super::write_review::yaml_string(value)
                );
            }
        }
        for (key, value) in &decision.extra {
            super::write_review::render_extra_field(block, indent(), key, value);
        }
    }
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

    fn stored(key: &str, outcome: &str, companion: &str) -> RawDecision {
        RawDecision {
            key: Some(key.into()),
            outcome: Some(outcome.into()),
            target: (outcome == "routed").then(|| companion.to_string()),
            reason: (outcome == "discarded").then(|| companion.to_string()),
            decided_at: Some("2026-09-01T00:00:00Z".into()),
            decided_by: Some("first@example.com".into()),
            extra: BTreeMap::new(),
        }
    }

    fn decided(key: &str, outcome: DecisionOutcome, companion: &str) -> DecisionRef {
        DecisionRef {
            key: key.into(),
            outcome,
            target: (outcome == DecisionOutcome::Routed).then(|| companion.to_string()),
            reason: (outcome == DecisionOutcome::Discarded).then(|| companion.to_string()),
        }
    }

    #[test]
    fn a_matching_decision_keeps_its_original_stamp() {
        let merged = merge(
            vec![stored("k", "discarded", "noise")],
            &[],
            &[decided("k", DecisionOutcome::Discarded, "noise")],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(
            merged[0].decided_by.as_deref(),
            Some("first@example.com"),
            "a re-matched decision is the same decision, not a new one"
        );
    }

    #[test]
    fn a_changed_decision_replaces_the_stored_one() {
        let merged = merge(
            vec![stored("k", "discarded", "noise")],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Routed,
                "specs/031-x/tasks.md",
            )],
            "2026-09-25T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].outcome.as_deref(), Some("routed"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
        assert_eq!(
            merged[0].decided_at.as_deref(),
            Some("2026-09-25T00:00:00Z")
        );
    }

    #[test]
    fn expired_entries_drop_and_malformed_ones_never_do() {
        let malformed = RawDecision {
            key: Some("broken".into()),
            ..RawDecision::default()
        };
        let merged = merge(
            vec![stored("gone", "routed", "specs/031-x/tasks.md"), malformed],
            &[decided(
                "gone",
                DecisionOutcome::Routed,
                "specs/031-x/tasks.md",
            )],
            &[],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].key.as_deref(), Some("broken"));
    }

    #[test]
    fn a_new_decision_without_an_author_is_refused() {
        assert_eq!(
            merge(
                Vec::new(),
                &[],
                &[decided("k", DecisionOutcome::Discarded, "noise")],
                "2026-09-25T00:00:00Z",
                Some("  "),
            ),
            Err(MergeError::DecidedByMissing)
        );
    }

    #[test]
    fn a_rendered_list_reads_back_to_the_same_entries() {
        let dir = tempfile::tempdir().unwrap();
        let mut original = stored(
            "perf: a() loops — `src/a.rs`",
            "routed",
            "specs/031-x/tasks.md",
        );
        original.extra.insert(
            "ticket".into(),
            serde_norway::Value::String("OPS-12".into()),
        );
        let mut block = String::new();
        render(&mut block, std::slice::from_ref(&original));
        write(dir.path(), "review.md", &block);
        assert_eq!(
            read_decisions(dir.path(), "review.md").unwrap(),
            vec![original]
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

    /// The data model's rule: a stored decision matches on key and outcome. A
    /// host restating a matched discard's reason in other words has not made
    /// a new decision, so the original stamp — and its wording — stand, and
    /// no `decided-by` is needed.
    #[test]
    fn a_rematch_with_reworded_companion_text_keeps_the_original() {
        let merged = merge(
            vec![stored("k", "discarded", "noise")],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Discarded,
                "just noise, really",
            )],
            "2026-09-25T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(merged, vec![stored("k", "discarded", "noise")]);
    }

    /// A key decided with a different outcome is a new decision and replaces
    /// the stored one.
    #[test]
    fn a_changed_outcome_is_a_new_decision() {
        let merged = merge(
            vec![stored("k", "discarded", "noise")],
            &[],
            &[decided(
                "k",
                DecisionOutcome::Routed,
                "specs/001-x/tasks.md",
            )],
            "2026-09-25T00:00:00Z",
            Some("second@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].outcome.as_deref(), Some("routed"));
        assert_eq!(merged[0].decided_by.as_deref(), Some("second@example.com"));
    }

    /// Two findings sharing a key are one finding: the same decision twice is
    /// stored once, and two different decisions for one key are refused.
    #[test]
    fn this_runs_decisions_collapse_by_key_and_conflicts_are_refused() {
        let twice = decided("k", DecisionOutcome::Discarded, "noise");
        let merged = merge(
            Vec::new(),
            &[],
            &[twice.clone(), twice],
            "2026-09-25T00:00:00Z",
            Some("dev@example.com"),
        )
        .unwrap();
        assert_eq!(merged.len(), 1);

        assert_eq!(
            merge(
                Vec::new(),
                &[],
                &[
                    decided("k", DecisionOutcome::Discarded, "noise"),
                    decided("k", DecisionOutcome::Routed, "specs/001-x/tasks.md"),
                ],
                "2026-09-25T00:00:00Z",
                Some("dev@example.com"),
            ),
            Err(MergeError::ConflictingDecisions("k".into()))
        );
    }

    /// The shared conversion flattens the key and companion the way
    /// `process-decisions` flattens a fired key, so the two agree.
    #[test]
    fn decision_for_flattens_line_hazards() {
        let disposition = Disposition {
            outcome: DispositionOutcome::Discarded,
            target: None,
            reason: Some("false\u{1b}positive".into()),
        };
        let decision = decision_for(&disposition, "grounding — split\nmessage").unwrap();
        assert_eq!(decision.key, "grounding — split message");
        assert_eq!(decision.reason.as_deref(), Some("false positive"));
        let fixed = Disposition {
            outcome: DispositionOutcome::Fixed,
            ..Disposition::default()
        };
        assert_eq!(decision_for(&fixed, "k"), None);
    }
}
