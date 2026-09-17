//! Canonical finding-severity tier sets.
//!
//! Three closed sets, one per vocabulary, and they are deliberately three
//! types rather than one. The field is spelled `severity` on all five of the
//! shapes that carry it, but the sets are not interchangeable — a review
//! finding is `must`/`should`, an analyze finding is the constitution's
//! four-tier Validation Severity scale, and the rule an `assessSpecQuality`
//! request assesses carries an RFC 2119 obligation level. Merging them into
//! one enum would make the cross-vocabulary error — an analyze tier emitted
//! into a review finding — representable in the very type introduced to
//! forbid it.
//!
//! **The defect these replace was not that a bad value existed; it was where
//! a bad value landed.** `write-review` bucketed findings with the severity
//! test last in an `if`/`else` chain whose `else` was the catch-all, so any
//! string that was not exactly `must` filed as a SHOULD, `blocking` was
//! written `false`, and the spec passed `check-review-gate`'s MUST check.
//! The reachable path is not a source typo: `severity` arrives from the
//! `performReview` extension point, written freehand by an LLM into JSON on
//! every review run. An unrecognized value must therefore **fail**, never
//! degrade to the permissive tier (spec 022 scenario
//! `severity-is-a-closed-set-not-a-string`).
//!
//! Follows the [`crate::schema::status`] precedent: one module owning a
//! closed set, consumed everywhere instead of hand-maintained copies.
//!
//! ## Case-insensitivity is preserved, deliberately
//!
//! Both comparisons these replace used `eq_ignore_ascii_case`, so `"MUST"`
//! was accepted before this change and is accepted after it. Narrowing to an
//! exact match would have been a behavior change for any host already
//! sending the RFC 2119 spelling — which is the spelling the rule files
//! themselves use — turning a working review run into a halted one over a
//! case difference that carries no ambiguity. Strictness here targets
//! *unrecognized* values, not capitalization. Serialization is always the
//! canonical lower-case form, so records stay byte-identical.

use std::fmt;
use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de};

/// Build the rejection message for an unrecognized tier.
///
/// Names the offending value and the full legal set. It cannot name the
/// *field* — a `Deserialize` impl has no access to the key it was reached
/// through — so the vocabulary is named instead, which identifies the field's
/// type and is what tells an operator whether a value crossed vocabularies or
/// was simply misspelled.
fn unrecognized(vocabulary: &str, found: &str, expected: &[&str]) -> String {
    format!(
        "unrecognized {vocabulary} severity {found:?} — expected one of: {}",
        expected.join(", ")
    )
}

/// Deserialize a tier from a string through its [`FromStr`], mapping the
/// parse failure onto serde's error type so the message survives.
fn deserialize_tier<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr<Err = String>,
{
    let raw = String::deserialize(deserializer)?;
    T::from_str(&raw).map_err(de::Error::custom)
}

/// Review finding tiers — the RFC 2119 obligation level a
/// `/{project}:review` finding carries.
///
/// `must` is blocking and `should` is advisory; there is no third value and
/// no default. [`crate::schema::primitives::ReviewFinding`] deliberately does
/// **not** derive `Default` for this reason — a finding with no severity is
/// not a meaningful value, and the derive is what made the empty string
/// constructible in the first place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewSeverity {
    /// Blocking. Counts toward `must-violations` and sets `blocking: true`.
    Must,
    /// Advisory. Counts toward `should-violations` and never sets `blocking`.
    Should,
}

impl ReviewSeverity {
    /// The legal set, in severity order, for messages and tests.
    pub const ALL: &'static [&'static str] = &["must", "should"];

    /// Canonical wire form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Must => "must",
            Self::Should => "should",
        }
    }

    /// Whether this tier blocks a spec from reaching `done`.
    ///
    /// The bucketing predicate, expressed once on the type rather than as an
    /// `else` arm: a tier that is not `Must` is `Should` by construction, so
    /// there is no third branch for an unrecognized value to fall into.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Must)
    }
}

/// Analyze finding tiers — the constitution's Validation Severity scale
/// (§text-first-artifacts).
///
/// Carried by [`crate::schema::primitives::FrontmatterFinding`] and
/// [`crate::schema::primitives::ArtifactFinding`]. Which condition earns
/// which tier is the constitution's to state, not this type's; see
/// `framework/commands/analyze.md` step 2 for the host's rendering rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AnalyzeSeverity {
    /// The artifact itself is malformed. Blocks pipeline advancement.
    HardFail,
    /// The artifact set is incomplete or inconsistent with itself. Blocks
    /// pipeline advancement; distinct from `HardFail` in that the file parses.
    Blocking,
    /// Reported, never gating.
    Advisory,
    /// An observation that is neither an error nor a warning.
    Informational,
}

impl AnalyzeSeverity {
    /// The legal set, in severity order, for messages and tests.
    pub const ALL: &'static [&'static str] =
        &["hard-fail", "blocking", "advisory", "informational"];

    /// Canonical wire form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardFail => "hard-fail",
            Self::Blocking => "blocking",
            Self::Advisory => "advisory",
            Self::Informational => "informational",
        }
    }

    /// Whether this tier prevents pipeline advancement. `HardFail` and
    /// `Blocking` both do — the distinction between them is what is wrong,
    /// not whether it gates.
    #[must_use]
    pub const fn is_gating(self) -> bool {
        matches!(self, Self::HardFail | Self::Blocking)
    }
}

/// Rule tiers for the `assessSpecQuality` extension point — the obligation
/// level of the *rule* being assessed, not of a finding against it.
///
/// [`Unspecified`](Self::Unspecified) is the fourth state and serializes to
/// the empty string, preserving the wire shape `severity_from_step_prose`
/// has always produced when a step's prose names no tier. It is a named
/// state rather than an absent one precisely so it cannot be confused with a
/// tier that failed to parse.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RuleSeverity {
    /// MUST-tier rule. Its findings join the Blocking tier.
    Must,
    /// SHOULD-tier rule. Its findings join the Advisory tier.
    Should,
    /// INFO-tier rule.
    Info,
    /// The step prose named no tier.
    #[default]
    #[serde(rename = "")]
    Unspecified,
}

impl RuleSeverity {
    /// The legal set, in severity order, for messages and tests. The empty
    /// string is included because it is a legal wire value, not an absence.
    pub const ALL: &'static [&'static str] = &["must", "should", "info", ""];

    /// Canonical wire form.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Must => "must",
            Self::Should => "should",
            Self::Info => "info",
            Self::Unspecified => "",
        }
    }
}

macro_rules! tier_conversions {
    ($ty:ty, $vocabulary:literal, [$(($variant:path, $wire:literal)),+ $(,)?]) => {
        impl FromStr for $ty {
            type Err = String;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                $(
                    if raw.eq_ignore_ascii_case($wire) {
                        return Ok($variant);
                    }
                )+
                Err(unrecognized($vocabulary, raw, <$ty>::ALL))
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserialize_tier(deserializer)
            }
        }
    };
}

tier_conversions!(
    ReviewSeverity,
    "review",
    [(Self::Must, "must"), (Self::Should, "should")]
);

tier_conversions!(
    AnalyzeSeverity,
    "analyze",
    [
        (Self::HardFail, "hard-fail"),
        (Self::Blocking, "blocking"),
        (Self::Advisory, "advisory"),
        (Self::Informational, "informational"),
    ]
);

tier_conversions!(
    RuleSeverity,
    "rule",
    [
        (Self::Must, "must"),
        (Self::Should, "should"),
        (Self::Info, "info"),
        (Self::Unspecified, ""),
    ]
);

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn every_variant_round_trips_through_its_wire_form() {
        for wire in ReviewSeverity::ALL {
            let parsed = ReviewSeverity::from_str(wire).unwrap();
            assert_eq!(parsed.as_str(), *wire);
        }
        for wire in AnalyzeSeverity::ALL {
            let parsed = AnalyzeSeverity::from_str(wire).unwrap();
            assert_eq!(parsed.as_str(), *wire);
        }
        for wire in RuleSeverity::ALL {
            let parsed = RuleSeverity::from_str(wire).unwrap();
            assert_eq!(parsed.as_str(), *wire);
        }
    }

    #[test]
    fn serialization_is_the_canonical_wire_string() {
        // Records must stay byte-identical across this change.
        assert_eq!(
            serde_json::to_value(ReviewSeverity::Must).unwrap(),
            serde_json::json!("must")
        );
        assert_eq!(
            serde_json::to_value(AnalyzeSeverity::HardFail).unwrap(),
            serde_json::json!("hard-fail")
        );
        assert_eq!(
            serde_json::to_value(RuleSeverity::Unspecified).unwrap(),
            serde_json::json!("")
        );
    }

    #[test]
    fn case_variants_are_accepted_and_normalize() {
        // Preserved on purpose: `eq_ignore_ascii_case` was the behavior
        // before this change, and the rule files spell the tier `MUST`.
        assert_eq!(
            ReviewSeverity::from_str("MUST").unwrap(),
            ReviewSeverity::Must
        );
        assert_eq!(
            ReviewSeverity::from_str("Should").unwrap(),
            ReviewSeverity::Should
        );
        assert_eq!(
            AnalyzeSeverity::from_str("HARD-FAIL").unwrap(),
            AnalyzeSeverity::HardFail
        );
        assert_eq!(ReviewSeverity::from_str("MUST").unwrap().as_str(), "must");
    }

    #[test]
    fn an_unrecognized_value_is_rejected_naming_the_value_and_the_set() {
        let err = ReviewSeverity::from_str("mandatory").unwrap_err();
        assert!(err.contains("\"mandatory\""), "{err}");
        assert!(err.contains("must, should"), "{err}");
        assert!(err.contains("review"), "{err}");
    }

    #[test]
    fn a_value_from_another_vocabulary_is_rejected_rather_than_demoted() {
        // The whole point: `blocking` is a legal analyze tier and an illegal
        // review one. Before this change it fell through to the SHOULD
        // bucket and wrote `blocking: false`.
        assert!(ReviewSeverity::from_str("blocking").is_err());
        assert!(AnalyzeSeverity::from_str("must").is_err());
    }

    #[test]
    fn an_empty_string_is_rejected_for_the_two_sets_that_do_not_admit_it() {
        assert!(ReviewSeverity::from_str("").is_err());
        assert!(AnalyzeSeverity::from_str("").is_err());
        // …and accepted for the one that does, as a named state.
        assert_eq!(
            RuleSeverity::from_str("").unwrap(),
            RuleSeverity::Unspecified
        );
    }

    #[test]
    fn deserialization_rejects_an_unrecognized_tier() {
        let err = serde_json::from_str::<ReviewSeverity>("\"mandatory\"").unwrap_err();
        assert!(err.to_string().contains("mandatory"), "{err}");

        // And accepts every legal one.
        assert_eq!(
            serde_json::from_str::<ReviewSeverity>("\"must\"").unwrap(),
            ReviewSeverity::Must
        );
    }

    #[test]
    fn blocking_and_gating_predicates_match_the_tiers() {
        assert!(ReviewSeverity::Must.is_blocking());
        assert!(!ReviewSeverity::Should.is_blocking());
        assert!(AnalyzeSeverity::HardFail.is_gating());
        assert!(AnalyzeSeverity::Blocking.is_gating());
        assert!(!AnalyzeSeverity::Advisory.is_gating());
        assert!(!AnalyzeSeverity::Informational.is_gating());
    }
}
