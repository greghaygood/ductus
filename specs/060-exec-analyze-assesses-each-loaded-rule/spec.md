---
status: draft
dependencies: [022-deterministic-runtime, 058-findings-route-at-discovery]
next-criterion: 5
---

# 060 — Exec analyze assesses each loaded rule

A `ductus exec` `/{project}:analyze` assesses each rule in the tier the rule carries and counts each verdict once, and its record names every loaded rule it did not ask the host about.

## Motivation

Steps 11 and 12 of `/{project}:analyze` request a semantic assessment for every loaded MUST-tier and SHOULD-tier rule whose Verification trigger fires against the spec. Interactively the host walks each rule. The exec walker sent one `assessSpecQuality` request per step, as [022](../022-deterministic-runtime/spec.md)'s single-shot extension points specify, and `resolve_assessed_rule` (`runtime/src/interpreter/payload.rs:883`) filled it with one rule: the first cited rule whose Verification resolved, else the first rule in the loaded files that carried one. The request's tier came from the step's prose (`severity_from_step_prose`, `runtime/src/interpreter/payload.rs:867`), not from the rule.

Both steps therefore asked about the same rule, once as `must` and once as `should`: the `analyze-basic` golden's `req-1` and `req-2` both carry `CFG-CONST-001`. [058](../058-findings-route-at-discovery/spec.md)'s task 41 made the exec record count the host's verdicts (`AnalyzeTally::record_assessment`, `runtime/src/interpreter/analyze_tally.rs:181`), and 058's fifth review found two consequences:

- One rule's verdict counted twice. An exec run over `analyze-basic`, whose one rule both steps assess, recorded `blocking-findings: 1` and `advisory: 1` for that single rule.
- The other loaded rules went unasked and unrecorded. This repository's 11 rule files held 192 rule sections with a Verification paragraph (counted 2026-09-26). An exec run asked the host about one rule per step, never evaluated which rules' triggers fire, and named none of the rest in `unexamined-by-reason`, so the record read as though steps 11 and 12 had examined every rule. That is the overstatement `QUAL-CLAIM-001` forbids.

058's task 48 had already recorded a request carrying no rule under `rule-assessments-not-checked` rather than counting its verdict. That covered a step with no rule to ask about, not a step with more rules than its one request could carry.

## Acceptance Criteria

- [ ] AC1: An exec analyze assesses each rule in the tier the rule itself carries, never in a tier taken from the step alone, so no rule is assessed in both step 11 and step 12
- [ ] AC2: Each rule an exec analyze assesses contributes its verdict to the record at most once
- [ ] AC3: Every loaded rule an exec analyze did not ask the host about is recorded in `unexamined-by-reason`, under a reason `framework/commands/analyze.md`'s Unexamined targets and 022's data model classify, so the record never reads as though steps 11 and 12 examined a rule they did not
- [ ] AC4: An exec run over `analyze-basic` counts `CFG-CONST-001`'s verdict once, and a test with two or more loaded rules shows each one either assessed or recorded unexamined

## Open Questions

- Does the walker send one request per rule, which changes 022's single-shot extension-point contract and moves the `analyze-basic` golden, or keep one request per step and record the rules it could not ask about as unexamined? The first examines more; the second changes no protocol.
- Where does the walker read a rule's tier? The rule files state it in each rule's Statement prose (MUST or SHOULD), not in a field, and `resolve_assessed_rule` reads none.
- Does the exec walker evaluate a rule's Verification trigger, as steps 11 and 12 say, or ask about every loaded rule and let the host answer that a trigger does not fire?
