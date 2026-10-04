---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-04T17:25:58Z
reviewed-against: c38053d73dc39b2670a492f56ff2cc69a99abc39
diff-base: 27b7df2735d5842e2f87314111643cc01107810e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 29
scope: 34
skipped-passes: []
reviewed-digest:
  data-model.md: 5e75019313bc78379427ac5eaef318d638b53e916cda686a55bdc338c3e2ef6c
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 063-oversized-artifacts-warn-with-a-fix

## Summary

Five passes over 063's 34-entry scope at c38053d7, against the diff base 27b7df27 (the parent of the in-progress flip, 4f7e6aae), with all eleven rule files under framework/rules/ loaded; the three frontend files match nothing in scope. Read: runtime/src/primitives/check_artifact_size.rs, 063's spec, plan, data model and tasks, AGENTS.md and 058's review.md in full, and every other in-scope file as its diff against the base — 29 of 34. Not read, 5 of 34: the four generated .claude/commands/ductus/ mirrors (the pre-commit hook regenerated them and reported all 16 commands in sync) and framework/bootstrap/govern.md (byte-identical to framework/bootstrap/ductus.md by cmp, which was read). The quality pass found one QUAL-CLAIM-001 SHOULD in check-artifact-size's scenario listing: an entry the listing returned as an error, and a scenario whose name is not valid UTF-8, were dropped without a trace, so the result read as fully examined. It was fixed in c38053d7 as 063 task 10, each half pinned by a test proven to fail under the mutation that removes it, and the changed file was re-read after the fix, so this record describes the fixed tree. No MUST or SHOULD violation remains, and nothing outside the loaded rules was observed.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
