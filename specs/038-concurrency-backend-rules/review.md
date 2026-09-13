---
spec: 038-concurrency-backend-rules
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

# Review — 038-concurrency-backend-rules

## Summary

First review of 038 to record `examined` against a derived `scope`; the prior record predated those fields, so its `0/0/0` could not be distinguished from a run whose passes never fired. One defect found and fixed: **AC6 named a cross-reference the rule file does not make.** It claimed `concurrency-backend.md` cites `BE-POOL-*` for pool interaction; the file does not mention pools or connections anywhere, so there was nothing to cite and — the thing the criterion actually guards — nothing was restated either. The criterion now states the citations delivered (`BE-IDEMP`, `CFG-*`) and why the third was unnecessary. This is the same shape found in 034 and 039 this cycle; in 039 it was a real gap, here it is not.

**The other six criteria verified against the tree.** AC1: the file exists with the `-backend.md` suffix and the canonical schema. AC2: the eight IDs span `RACE`/`LOCK`/`TXN`/`COORD`, disjoint from `security-backend.md`, `api-backend.md` and `performance-backend.md` — checked by extracting all four category sets and intersecting; `lint-rule-ids.sh` exits 0. AC3: the header declares all four with their concerns. AC4: shared-state races, locking/deadlock and transaction isolation are each covered, and all eight Verification clauses are design-time commitments phrased against a spec or plan rather than code-pattern greps. AC5: the four MUSTs are the corruption hazards — `BE-RACE-001` (unguarded shared mutable state), `BE-TXN-002`, `BE-COORD-001` and `BE-COORD-002` (fencing tokens and delivery semantics); the contextual choices, optimistic-vs-pessimistic locking and isolation-level selection, are SHOULD. Note the Resolved Questions enumerate the MUSTs by *concern* (three) while the file carries four *rules*, because `COORD` contributes two — that is the enumeration being coarser than the rule set, not a mismatch. AC7: one **Shared Files** manifest row.

**What this review read: both files in scope** — `framework/rules/concurrency-backend.md` and `framework/bootstrap/ductus.md`'s manifest row. `specs/038-concurrency-backend-rules/spec.md` was read in full and is not counted, falling outside the resolved scope. The diff base is an empty window for the reason recorded in 037's review: no commit records 038 entering `in-progress`, and basing on its first commit's parent would resolve hundreds of files and describe a review nobody performed. 038 has no scenarios and no data model, so `reviewed-digest` is empty — taken and empty, which reads as current.

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
