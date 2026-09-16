---
spec: 047-analyze-findings-durability
last-run: 2026-09-16T12:34:24Z
reviewed-against: 267070e4bcf15cb354fa20997e1b13d38f2650db
diff-base: 1f5539dbc98706aa8811c10cc484d59901eea9fa
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 12
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 4873de36588f18d253409758c41ab5d0b200c1a1655271c64aa5e8d98998a39f
  scenarios/analyze-run-durability.md: 287dacba26b62b8f72ba377b8f07f92063cdd28a351ede9568035dd0fd811046
blocking: false
---

# Review — 047-analyze-findings-durability

## Summary

Re-review for spec 057's record relocation, which reopened this spec to discharge its `cross-spec-impact:` entry. Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers (`process-waivers` returned empty in every bucket).

**What changed.** 057 moved the analyze record out of `spec.md` frontmatter into `analysis.md`'s, so every claim here that addressed it as a `spec.md` block was false. AC9, AC10, AC12, AC14 and AC15 are annotated in place rather than restated (the 009-AC18 / 046-AC22 disposition), a signpost at the top links back to 057, and `scenarios/analyze-record-freshness.md` — this spec's one durable contract — now names `analysis.md` in its subject set, records that the digest exclusion moved there and `spec.md` is digested whole, and adds the third state 057 introduced: an artifact that exists but carries no parseable record is undeterminable, never never-analyzed.

**The Resolved Question was reversed, not relocated, and that is recorded as a reversal.** *Should analyze write a per-spec `analysis.md` artifact?* resolved **no**, on the premise that the record could live in `spec.md` while only the findings' content needed a home. 057 retired that premise — two gate-bearing records in one file is the two-homes condition §drift-prevention rejects — so the file follows the record. The annotation states what survives (the inbox stays the durable home for routable findings; 057's AC13 forbids a checkbox anywhere in `analysis.md` *mechanically*, which is what keeps the parallel-triage-surface hazard the question named from arriving with the reversal) and what the reversal cost (a new primitive, a corpus migration, a second file per spec directory) rather than reading the old *no* as merely superseded.

**Scope.** `diff-base` 1f5539db, 12 in scope. Examined **9 of 12**, and the three unread are named rather than folded into the numerator: `framework/commands/analyze.md` (402 lines) was read only in its Review-state-drift and analyze-record sections, `framework/constitution.md` (834) only in §Frontmatter Schema and §Validation Severity — both confirmed against the live file where this spec's criteria assert on them, neither read end to end — and `.claude/commands/ductus/analyze.md` is the generated mirror of a source that was not fully read either, so it is doubly uncounted.

**Passes.** The security, reuse and efficiency passes had no subject: the window contains prose artifacts and one YAML workflow template. The quality and simplicity passes carried the weight, against `quality-cross.md`. The one `QUAL-CLAIM-001` candidate in the window — the CI template's audit-record gate treating an absent `blocking` field and an uppercase `NULL` timestamp as clean — is 020's surface, not this spec's; it was found by 020's pass in the same session, fixed at `267070e4`, and is therefore already absent from this scope. `QUAL-GROUND-001` applies to the annotations themselves and was enforced by re-deriving each claim from the runtime rather than from the spec: the subject-set and exclusion wording was checked against `analyze_subjects.rs`, and the undeterminable-vs-absent three-state wording against the constitution's §Frontmatter Schema → Audit records.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
