---
spec: 050-constitution
reviewed-at: 2026-09-14T01:58:21Z
reviewed-against: fc70afc6db98029dc8ebd35e666be17eed7c1a70
diff-base: ed2092a85afb3c1da6fa5b393f6afbc7f85e2e15
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 8
skipped-passes: []
---

# Review — 050-constitution

## Summary

Re-run 2026-09-13. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. No waivers.

**Why this pass happened, and it was not 050's own work.** `020-code-review` discharged its embedded-snapshot item by replacing a 561-line frozen copy of `framework/commands/review.md` with a pointer. `scenarios/a-canonical-source-is-pointed-at-not-copied.md` is the durable contract that records that instance as the motivating case for the pointer-not-copy rule — and it recorded it in the **present tense**: "its §Embedded artifacts section **is** a frozen copy", "**70% of the spec is** a copy of another file", "the snapshot **sits** inside a code fence". All three went false the moment the section was removed.

Corrected by tense, with the discharge and its date recorded, and the account otherwise intact — the rule outlives its instance, and this scenario is the record of why the rule exists. Deleting the narrative would have destroyed exactly the decision record [§drift-prevention](../../framework/constitution.md#drift-prevention) preserves; leaving it in the present tense would have left a false claim inside the one scenario that governs canonical-copy drift, which is self-refuting. This is the tense rule the frozen-archaeology sweep established: a sentence describing what *is* gets fixed, one describing what *was* keeps its account.

**Scope.** The natural base collapsed from 147 modified-since / 149 in scope to **4 / 8** once the correction commit recorded a fresh `in-progress` transition; `--since HEAD` gave 0 / 5 and was declined for excluding this pass's own edits. Examined **6 of 8** — `specs/045-decision-state-drift-detection/spec.md` and `specs/050-constitution/plan.md` were not read, and nothing in this change reaches either.

**AC1–AC19 checked against the tree; all hold.** None asserts anything about 020's snapshot, so the correction falsifies no criterion here. AC12 in particular still holds and was re-verified rather than assumed: it requires that this spec leave the repo-root `version` pin, `runtime/Cargo.toml` and `runtime/CHANGELOG.md` untouched. All three moved today for `ductus-v0.49.4` — but under `022`'s runtime fixes, not under this spec, and this pass touched one scenario file. Family 20 is clean.

**A note on what this pass is evidence for.** The change here is one paragraph of tense in one scenario. The five passes were run against a scope of eight files of which six were read, and the finding count is a real zero rather than an unexamined one — but the honest reading of this record is that it re-verified a narrow correction and this spec's criteria, not that it re-derived 050's whole subject. The promotion work 050 owns is unchanged since its 2026-09-13 review, and the second promotion round remains an open inbox item and its own unit.

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
