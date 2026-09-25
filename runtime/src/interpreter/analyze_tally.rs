//! The tier counts an exec `/{project}:analyze` walk records (spec 058, AC26).
//!
//! Interactively, the host tallies each detection step's findings into the
//! tier counts `write-analysis` records, by the tier each step assigns. The
//! exec walker has no host to do that, and its `write-analysis` dispatch once
//! bound no counts at all, so every exec record read 0/0/0 and fully examined
//! whatever detection had found: a clean result standing in for an unexamined
//! one, the conflation `QUAL-CLAIM-001` forbids. This is that tally.
//!
//! It is taken at dispatch rather than read back from the walker context,
//! because the context merges results by bare key and several steps answer
//! under the same one (`findings`), each overwriting the last. Nothing is
//! itemized, so `write-analysis` counts every live finding undispositioned —
//! the walker has no operator to decide one.
//!
//! Each arm restates the tier its step in `framework/commands/analyze.md`
//! assigns. The procedure is the authority: a step whose tiering changes
//! changes here in the same commit.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

/// The running tier counts of one exec analyze walk.
#[derive(Debug, Default)]
pub(crate) struct AnalyzeTally {
    hard_fail: u32,
    blocking: u32,
    advisory: u32,
    /// Skipped targets by their closed reason, as `check-artifacts` reports them.
    unexamined: BTreeMap<String, u32>,
}

impl AnalyzeTally {
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
                let past_draft = matches!(status, "clarified" | "planned" | "in-progress" | "done");
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
            // actually read. With none examined, nothing was checked, and a
            // blocking finding raised from that would be against a correct spec.
            "check-rule-ids" => {
                if result.get("examined").and_then(Value::as_u64).unwrap_or(0) > 0 {
                    self.blocking += count("missing");
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
                        *self.unexamined.entry(reason.to_string()).or_default() += 1;
                    }
                }
            }
            // Steps 9 and 10: project-level checks, advisory.
            "check-orphaned-references" => self.advisory += count("findings"),
            "check-unfolded-specs" => self.advisory += count("unfolded"),
            _ => {}
        }
    }

    /// Steps 11 and 12: an `assessSpecQuality` finding joins the blocking tier
    /// for a MUST rule and the advisory tier for a SHOULD rule.
    pub(crate) fn record_assessment(&mut self, response: &Value) {
        if response.get("passed") == Some(&Value::Bool(true)) {
            return;
        }
        match response
            .pointer("/finding/severity")
            .and_then(Value::as_str)
        {
            Some("must") => self.blocking += 1,
            Some("should") => self.advisory += 1,
            _ => {}
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

    /// Count findings by the analyze severity each carries.
    fn by_severity(&mut self, findings: &[Value]) {
        for finding in findings {
            match finding.get("severity").and_then(Value::as_str) {
                Some("hard-fail") => self.hard_fail += 1,
                Some("blocking") => self.blocking += 1,
                Some("advisory") => self.advisory += 1,
                _ => {}
            }
        }
    }
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

    #[test]
    fn each_step_counts_in_the_tier_it_assigns() {
        let mut tally = AnalyzeTally::default();
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
            &json!({ "findings": [{}] }),
            &context,
        );
        tally.record_primitive(
            "check-unfolded-specs",
            &json!({ "unfolded": [{}] }),
            &context,
        );
        tally.record_assessment(&json!({ "passed": false, "finding": { "severity": "must" } }));
        tally.record_assessment(&json!({ "passed": false, "finding": { "severity": "should" } }));
        tally.record_assessment(&json!({ "passed": true }));

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
            json!([["root-absent", 2]])
        );
    }

    /// Below `clarified` an incompatible dependency is allowed, and with no
    /// rule file read a missing citation is nothing checked, not a finding.
    #[test]
    fn a_draft_spec_and_an_unread_rule_set_raise_nothing() {
        let mut tally = AnalyzeTally::default();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "draft" }));
        tally.record_primitive(
            "traverse-deps",
            &json!({ "dependencies": [{ "exists": true, "compatible": false }], "cycles": [] }),
            &context,
        );
        tally.record_primitive(
            "check-rule-ids",
            &json!({ "missing": ["X-1"], "deprecated": [], "examined": 0 }),
            &context,
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(bindings["unexamined-by-reason"], json!([]));
    }
}
