//! The tier counts an exec `/{project}:analyze` walk records (spec 058, AC26).
//!
//! Interactively, the host tallies each detection step's findings into the
//! tier counts `write-analysis` records, by the tier each step assigns. The
//! exec walker has no host to do that, and its `write-analysis` dispatch once
//! bound no counts at all, so every exec record read 0/0/0 and fully examined
//! whatever detection had found: a clean result standing in for an unexamined
//! one, the conflation `QUAL-CLAIM-001` forbids. This is that tally, and the
//! `unexamined-by-reason` breakdown beside it: what the walk did not examine,
//! including the detection it never runs.
//!
//! It is taken at dispatch rather than read back from the walker context,
//! because the context merges results by bare key and several steps answer
//! under the same one (`findings`), each overwriting the last. Nothing is
//! itemized, so `write-analysis` counts every live finding undispositioned —
//! the walker has no operator to decide one.
//!
//! Each arm restates the tier its step in `framework/commands/analyze.md`
//! assigns, and each unexamined reason the class that file's Unexamined
//! targets section gives it. The procedure is the authority: a step whose
//! tiering changes changes here in the same commit.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::schema::severity::{AnalyzeSeverity, RuleSeverity};
use crate::schema::status::UNBLOCKING_STATUSES;

/// Steps 13–15 — cross-service references, `## Applicable Rules` citations,
/// and grounding — by the reason each is recorded under. They are detection
/// the procedure leaves to the host, and the walker no-ops host prose, so an
/// exec run examines none of them. Each is recorded once, on every run: whether
/// a step would have applied to this spec is itself something the walk never
/// looked at.
const HOST_DETECTION: [&str; 3] = [
    "references-not-checked",
    "applicable-rules-not-checked",
    "grounding-not-checked",
];

/// The running tier counts of one exec analyze walk.
#[derive(Debug)]
pub(crate) struct AnalyzeTally {
    hard_fail: u32,
    blocking: u32,
    advisory: u32,
    /// What the walk did not examine, by reason: the host detection it never
    /// runs, the targets its detection steps report they could not reach, and
    /// the assessments it asked about no rule.
    unexamined: BTreeMap<String, u32>,
}

impl AnalyzeTally {
    /// A tally for a walk that has examined nothing yet, carrying the host
    /// detection it will not run.
    pub(crate) fn new() -> Self {
        Self {
            hard_fail: 0,
            blocking: 0,
            advisory: 0,
            unexamined: HOST_DETECTION
                .iter()
                .map(|reason| ((*reason).to_string(), 1))
                .collect(),
        }
    }

    /// Count one detection primitive's result. `context` is the walker context
    /// the step ran against, where step 3 finds the spec's own status in step
    /// 1's `read-spec` result.
    pub(crate) fn record_primitive(
        &mut self,
        name: &str,
        result: &Value,
        context: &Map<String, Value>,
    ) {
        let items = |key: &str| {
            result
                .get(key)
                .and_then(Value::as_array)
                .map_or(&[][..], Vec::as_slice)
        };
        let count = |key: &str| u32::try_from(items(key).len()).unwrap_or(u32::MAX);
        match name {
            // Step 2: each finding in the tier its own condition carries.
            "validate-frontmatter" => self.by_severity(items("findings")),
            // Step 3: a missing dependency and every cycle are blocking; an
            // incompatible dependency is blocking once this spec is at
            // `clarified` or later.
            "traverse-deps" => {
                let status = context
                    .get("frontmatter")
                    .and_then(|frontmatter| frontmatter.get("status"))
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let past_draft = UNBLOCKING_STATUSES.contains(&status);
                for edge in items("dependencies") {
                    let holds = |key: &str| edge.get(key) == Some(&Value::Bool(true));
                    if !holds("exists") || (past_draft && !holds("compatible")) {
                        self.blocking += 1;
                    }
                }
                self.blocking += count("cycles");
            }
            // Step 4: an unresolved anchor is advisory.
            "resolve-anchor" => self.advisory += count("unresolved"),
            // Step 5: a missing citation is blocking and a deprecated one
            // advisory — but `missing` means something only against rule files
            // actually read. With none examined, nothing was checked: a
            // blocking finding raised from that would be against a correct
            // spec, so each citation is recorded unexamined instead.
            "check-rule-ids" => {
                if result.get("examined").and_then(Value::as_u64).unwrap_or(0) > 0 {
                    self.blocking += count("missing");
                } else {
                    self.record_unexamined("rule-citations-not-checked", count("citations"));
                }
                self.advisory += count("deprecated");
            }
            // Step 6: drift between body links and `dependencies:` is one
            // advisory finding.
            "derive-dependencies" => {
                if count("updated") > 0 {
                    self.advisory += 1;
                }
            }
            // Step 7: each lint violation is advisory.
            "lint-markdown" => self.advisory += count("violations"),
            // Step 8: each finding in its tier; each skipped target under its
            // reason, never as a finding.
            "check-artifacts" => {
                self.by_severity(items("findings"));
                for skipped in items("skipped") {
                    if let Some(reason) = skipped.get("reason").and_then(Value::as_str) {
                        self.record_unexamined(reason, 1);
                    }
                }
            }
            // Steps 9 and 10: project-level checks, advisory. A referrer
            // step 9 could not read is unexamined, never clean.
            "check-orphaned-references" => {
                self.advisory += count("findings");
                self.record_unexamined("referrer-unreadable", count("skipped"));
            }
            "check-unfolded-specs" => self.advisory += count("unfolded"),
            _ => {}
        }
    }

    /// What an `assessSpecQuality` request asks the host about, read before
    /// the request is handed off, for [`Self::record_assessment`].
    pub(crate) fn assessed(request: &Value) -> Assessed {
        let verification = request
            .pointer("/rule/verification")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if verification.trim().is_empty() {
            Assessed::Nothing
        } else {
            Assessed::Rule(rule_tier(request, "/rule/severity"))
        }
    }

    /// Steps 11 and 12: a failed `assessSpecQuality` assessment joins the
    /// blocking tier for a MUST rule and the advisory tier for a SHOULD rule.
    ///
    /// The tier is the finding's own when the host returned one, and
    /// otherwise the tier of the rule the walker put in the request: a
    /// failure the host reported without a finding still failed, and
    /// dropping it would record the rule as passed. Either is read through
    /// [`RuleSeverity`], case-insensitively, as validation accepts it. An
    /// INFO rule has no analyze tier to join, and an unspecified one comes
    /// from a step naming no tier, which steps 11 and 12 never are.
    ///
    /// A request that asked about no rule ([`Assessed::Nothing`]) examined
    /// nothing, so the step is recorded unexamined whatever the host
    /// answered. Counting its verdict would raise a finding against a spec
    /// nothing was checked against, the reasoning step 5 applies to a
    /// citation checked against no rule file.
    pub(crate) fn record_assessment(&mut self, assessed: Assessed, response: &Value) {
        let Assessed::Rule(assessed) = assessed else {
            self.record_unexamined("rule-assessments-not-checked", 1);
            return;
        };
        if response.get("passed") == Some(&Value::Bool(true)) {
            return;
        }
        match rule_tier(response, "/finding/severity").or(assessed) {
            Some(RuleSeverity::Must) => self.blocking += 1,
            Some(RuleSeverity::Should) => self.advisory += 1,
            Some(RuleSeverity::Info | RuleSeverity::Unspecified) | None => {}
        }
    }

    /// Bind the tally as `write-analysis`' tier arguments, over any the context
    /// carries: the walk's own detection is the only authority on them.
    pub(crate) fn bind(&self, bindings: &mut Map<String, Value>) {
        bindings.insert("hard-fail".into(), self.hard_fail.into());
        bindings.insert("blocking-findings".into(), self.blocking.into());
        bindings.insert("advisory".into(), self.advisory.into());
        bindings.insert(
            "unexamined-by-reason".into(),
            Value::Array(
                self.unexamined
                    .iter()
                    .map(|(reason, count)| serde_json::json!([reason, count]))
                    .collect(),
            ),
        );
    }

    /// Record `count` targets unexamined under `reason`; nothing when zero, so
    /// a step that reached everything adds no entry.
    fn record_unexamined(&mut self, reason: &str, count: u32) {
        if count > 0 {
            *self.unexamined.entry(reason.to_string()).or_default() += count;
        }
    }

    /// Count findings by the analyze severity each carries.
    fn by_severity(&mut self, findings: &[Value]) {
        for finding in findings {
            match finding
                .get("severity")
                .and_then(Value::as_str)
                .and_then(|tier| tier.parse().ok())
            {
                Some(AnalyzeSeverity::HardFail) => self.hard_fail += 1,
                Some(AnalyzeSeverity::Blocking) => self.blocking += 1,
                Some(AnalyzeSeverity::Advisory) => self.advisory += 1,
                Some(AnalyzeSeverity::Informational) | None => {}
            }
        }
    }
}

/// What an `assessSpecQuality` request put to the host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Assessed {
    /// A rule with a Verification to assess the spec by, and the rule's tier
    /// when the request names one.
    Rule(Option<RuleSeverity>),
    /// No rule with a Verification: the walker read no rule file, or none it
    /// read carries one, so the request asks the host to assess the spec
    /// against nothing.
    Nothing,
}

/// The rule tier at `pointer` in an `assessSpecQuality` payload.
fn rule_tier(payload: &Value, pointer: &str) -> Option<RuleSeverity> {
    payload
        .pointer(pointer)
        .and_then(Value::as_str)
        .and_then(|tier| tier.parse().ok())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use serde_json::json;

    fn bound(tally: &AnalyzeTally) -> Map<String, Value> {
        let mut bindings = Map::new();
        tally.bind(&mut bindings);
        bindings
    }

    /// The host detection an exec walk never runs, as the record lists it.
    fn host_detection() -> Value {
        json!([
            ["applicable-rules-not-checked", 1],
            ["grounding-not-checked", 1],
            ["references-not-checked", 1]
        ])
    }

    #[test]
    fn each_step_counts_in_the_tier_it_assigns() {
        let mut tally = AnalyzeTally::new();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "planned" }));
        tally.record_primitive(
            "validate-frontmatter",
            &json!({ "findings": [{ "severity": "hard-fail" }, { "severity": "blocking" }] }),
            &context,
        );
        tally.record_primitive(
            "traverse-deps",
            &json!({
                "dependencies": [
                    { "exists": false, "compatible": false },
                    { "exists": true, "compatible": false },
                    { "exists": true, "compatible": true }
                ],
                "cycles": [["a", "b"]]
            }),
            &context,
        );
        tally.record_primitive(
            "resolve-anchor",
            &json!({ "unresolved": [{}, {}] }),
            &context,
        );
        tally.record_primitive(
            "check-rule-ids",
            &json!({ "missing": ["X-1"], "deprecated": ["Y-1"], "examined": 3 }),
            &context,
        );
        tally.record_primitive(
            "derive-dependencies",
            &json!({ "updated": ["a", "b"] }),
            &context,
        );
        tally.record_primitive("lint-markdown", &json!({ "violations": [{}] }), &context);
        tally.record_primitive(
            "check-artifacts",
            &json!({
                "findings": [{ "severity": "blocking" }, { "severity": "advisory" }],
                "skipped": [{ "reason": "root-absent" }, { "reason": "root-absent" }]
            }),
            &context,
        );
        tally.record_primitive(
            "check-orphaned-references",
            &json!({ "findings": [{}], "skipped": [] }),
            &context,
        );
        tally.record_primitive(
            "check-unfolded-specs",
            &json!({ "unfolded": [{}] }),
            &context,
        );
        tally.record_assessment(
            Assessed::Rule(Some(RuleSeverity::Must)),
            &json!({ "passed": false, "finding": { "severity": "must" } }),
        );
        tally.record_assessment(
            Assessed::Rule(Some(RuleSeverity::Should)),
            &json!({ "passed": false, "finding": { "severity": "should" } }),
        );
        tally.record_assessment(
            Assessed::Rule(Some(RuleSeverity::Must)),
            &json!({ "passed": true }),
        );

        let bindings = bound(&tally);
        assert_eq!(bindings["hard-fail"], 1);
        // frontmatter 1 + two edges + a cycle + a missing citation + an
        // artifact finding + a MUST assessment.
        assert_eq!(bindings["blocking-findings"], 7);
        // anchors 2 + deprecated 1 + drift 1 + lint 1 + artifact 1 + orphan 1
        // + unfolded 1 + a SHOULD assessment.
        assert_eq!(bindings["advisory"], 9);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["grounding-not-checked", 1],
                ["references-not-checked", 1],
                ["root-absent", 2]
            ])
        );
    }

    /// Steps 13–15 are host prose the walker never runs, so a walk that
    /// detected nothing still records them unexamined rather than reading as
    /// fully examined.
    #[test]
    fn an_exec_walk_records_the_host_detection_it_never_runs() {
        let bindings = bound(&AnalyzeTally::new());
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["unexamined-by-reason"], host_detection());
    }

    /// Below `clarified` an incompatible dependency is allowed, and with no
    /// rule file read a citation is unexamined, not missing: nothing was
    /// checked, so nothing is blocking and the record says what went unchecked.
    #[test]
    fn a_draft_spec_raises_nothing_and_an_unread_rule_set_records_its_citations() {
        let mut tally = AnalyzeTally::new();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "draft" }));
        tally.record_primitive(
            "traverse-deps",
            &json!({ "dependencies": [{ "exists": true, "compatible": false }], "cycles": [] }),
            &context,
        );
        tally.record_primitive(
            "check-rule-ids",
            &json!({
                "citations": [
                    { "rule-id": "X-1", "found": false, "deprecated": false },
                    { "rule-id": "X-2", "found": false, "deprecated": false }
                ],
                "missing": ["X-1", "X-2"],
                "deprecated": [],
                "examined": 0
            }),
            &context,
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["grounding-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-citations-not-checked", 2]
            ])
        );
    }

    /// An unread rule set is a gap only when the spec cites a rule: with no
    /// citation there was nothing to check against it.
    #[test]
    fn an_unread_rule_set_with_no_citation_records_nothing() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-rule-ids",
            &json!({ "citations": [], "missing": [], "deprecated": [], "examined": 0 }),
            &Map::new(),
        );
        assert_eq!(bound(&tally)["unexamined-by-reason"], host_detection());
    }

    /// A referrer `check-orphaned-references` could not read is unexamined:
    /// its empty `findings` for that file is not a clean one.
    #[test]
    fn an_unreadable_referrer_is_recorded_unexamined() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-orphaned-references",
            &json!({
                "findings": [],
                "skipped": [
                    { "path": "CLAUDE.md", "reason": "file exists but could not be read as UTF-8 text" },
                    { "path": "AGENTS.md", "reason": "file exists but could not be read as UTF-8 text" }
                ]
            }),
            &Map::new(),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["grounding-not-checked", 1],
                ["references-not-checked", 1],
                ["referrer-unreadable", 2]
            ])
        );
    }

    /// Validation accepts a capitalized tier, so the tally does too; and a
    /// failed assessment with no finding counts in its rule's tier rather
    /// than vanishing.
    #[test]
    fn an_assessment_counts_whatever_its_case_and_with_or_without_a_finding() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            Assessed::Rule(None),
            &json!({ "passed": false, "finding": { "severity": "MUST" } }),
        );
        tally.record_assessment(
            Assessed::Rule(None),
            &json!({ "passed": false, "finding": { "severity": "Should" } }),
        );
        let failed = json!({ "passed": false });
        tally.record_assessment(Assessed::Rule(Some(RuleSeverity::Must)), &failed);
        tally.record_assessment(Assessed::Rule(Some(RuleSeverity::Should)), &failed);
        tally.record_assessment(Assessed::Rule(Some(RuleSeverity::Info)), &failed);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 2);
        assert_eq!(bindings["advisory"], 2);
    }

    /// A request assesses a rule only when it carries the rule's
    /// Verification: with none resolved, the walker sends an empty rule, and
    /// a rule id alone gives the host nothing to assess the spec by.
    #[test]
    fn the_assessed_rule_is_read_from_the_request() {
        let request = json!({ "rule": { "id": "X-1", "verification": "v", "severity": "should" } });
        assert_eq!(
            AnalyzeTally::assessed(&request),
            Assessed::Rule(Some(RuleSeverity::Should))
        );
        for rule in [
            json!({ "id": "", "verification": "", "severity": "must" }),
            json!({ "id": "X-1", "verification": "  ", "severity": "must" }),
        ] {
            assert_eq!(
                AnalyzeTally::assessed(&json!({ "rule": rule })),
                Assessed::Nothing,
                "{rule}"
            );
        }
    }

    /// An assessment of no rule examined nothing: whatever the host answers,
    /// failed with a MUST finding or passed, it counts in no tier and the
    /// step is recorded unexamined.
    #[test]
    fn an_assessment_of_no_rule_is_unexamined_whatever_the_host_answers() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            Assessed::Nothing,
            &json!({ "passed": false, "finding": { "severity": "must" } }),
        );
        tally.record_assessment(Assessed::Nothing, &json!({ "passed": false }));
        tally.record_assessment(Assessed::Nothing, &json!({ "passed": true }));
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["grounding-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-assessments-not-checked", 3]
            ])
        );
    }
}
