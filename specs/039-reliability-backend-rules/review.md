---
spec: 039-reliability-backend-rules
reviewed-at: 2026-09-13T12:43:39Z
reviewed-against: a616ed9b2df0b5ce2704c590dc1adca7e7303cc3
diff-base: a616ed9b2df0b5ce2704c590dc1adca7e7303cc3
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 2
scope: 2
skipped-passes: []
---

# Review — 039-reliability-backend-rules

## Summary

First review of 039 to record `examined` against a derived `scope`; the prior record predated those fields, so its `0/0/0` could not be distinguished from a run whose passes never fired. **One real gap found and fixed at the source.** AC6 claimed `reliability-backend.md` cites `CFG-*` for tunable config, and the file contained zero mentions of configuration — while mandating a bounded timeout on every outbound call (`BE-TIMEOUT-001`), a maximum attempt count with backoff and jitter (`BE-RETRY-001`), and a failure-rate threshold (`BE-BREAKER-001`). Every one of those is an operator-tunable value, the spec's own Boundaries section assigns them to `configuration-cross.md`, and the sibling set 034 cites `CFG-CONST-003` from `BE-POOL-001`/`BE-POOL-002` for precisely this. Fixed by adding the citation to the rule file rather than by weakening the criterion: `BE-TIMEOUT-001` and `BE-RETRY-001` now require the value to be a named constant per `CFG-CONST-003`. That is the difference between this finding and the cosmetic AC6 discrepancies in 034 and 038 — there, no rule reached the cited surface, so nothing was owed; here the rules reached it and said nothing.

**The other seven criteria verified against the tree.** AC1: the file exists with the `-backend.md` suffix and the canonical schema. AC2: the eight IDs span `TIMEOUT`/`RETRY`/`BREAKER`/`DRAIN`/`BULK`, disjoint from the three sibling sets — checked by intersecting the extracted category sets; `lint-rule-ids.sh` exits 0 after the edit. AC3: the header declares all five with their concerns. AC4: timeouts/deadlines, bounded retries, circuit breakers and graceful shutdown are each covered, and all eight Verification clauses are design-time commitments rather than code-pattern greps. AC5: exactly three MUSTs, matching the Severity posture's enumeration precisely — `BE-TIMEOUT-001` (unbounded downstream wait), `BE-RETRY-001` (retry storm), `BE-DRAIN-001` (dropped in-flight work on deploy). AC6 as above. AC7: one **Shared Files** manifest row. AC8: 034's forward-reference resolves here — 034's Boundaries defer deadlines, downstream timeouts, retries and circuit breakers, and `TIMEOUT`/`RETRY`/`BREAKER` land all four, with `BE-POOL-002` correctly left in the performance set and cited rather than moved.

**What this review read: both files in scope** — `framework/rules/reliability-backend.md` and `framework/bootstrap/ductus.md`'s manifest row; `framework/rules/performance-backend.md` was also consulted for the AC8 boundary check and the `CFG-CONST-003` citation pattern, outside this scope. `specs/039-reliability-backend-rules/spec.md` was read in full and is not counted, falling outside the resolved scope. The diff base is an empty window for the reason recorded in 037's review. 039 has no scenarios and no data model, so `reviewed-digest` is empty — taken and empty, which reads as current.

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
