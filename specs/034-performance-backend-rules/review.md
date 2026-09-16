---
spec: 034-performance-backend-rules
diff-base: 40d0a9fb537d0a9ff35833579358e7efbfff8848
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T12:41:39Z
reviewed-against: 0357294b85a852d24feca4c6289ac528380a4414
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 3
reviewed-digest: {}
blocking: false
---

# Review — 034-performance-backend-rules

## Summary

First review of 034 to record `examined` against a derived `scope`; the prior record predated those fields, so its `0/0/0` could not be distinguished from a run whose passes never fired. One defect found and fixed in this cycle: **AC6 named two cross-references the rule file does not make.** Its parenthetical claimed `performance-backend.md` cites `BE-INPUT-006` for input bounds and `BE-IDEMP` for retry-safe async; neither appears anywhere in the file. Checking whether the rules that would need them restate instead — which is what the criterion exists to prevent — `BE-PAYLOAD-001` bounds *response* size and cites `BE-PAGE`, and `BE-ASYNC-001` cites `BE-STATUS-001` for the async acknowledgment. No rule in the set reaches request-size limits or retry semantics, so there was nothing to cite and nothing was duplicated. The requirement held; the enumeration, carried over from the Boundaries section at clarify time, did not. AC6 now states the citations actually made (`BE-PAGE`, `BE-STATUS-001`, `CFG-CONST-003`) and why the other two were unnecessary.

**The other six criteria verified against the tree.** AC1: the file exists with the `-backend.md` suffix and the canonical `### {ID}` / Statement / Rationale / Verification schema. AC2: all 13 IDs use `BE-{CATEGORY}-{NNN}` over `QUERY`/`CACHE`/`POOL`/`PAYLOAD`/`ASYNC`, disjoint from `security-backend.md`'s eight and `api-backend.md`'s seven — checked by extracting all four category sets and intersecting them; `lint-rule-ids.sh` exits 0. AC3: the header declares all five. AC4: query, caching, pooling and payload each carry design-time-commitment Verification clauses rather than code-pattern greps. AC5: the eight MUSTs are exactly the DoS/exhaustion cases the Severity posture enumerates (unbounded query and result set, never-expiring cache, per-request connection, unsized pool, unbounded pool wait, unbounded response, request-blocking slow work); the five SHOULDs are the tunable trade-offs. AC7: one **Shared Files** manifest row in `framework/bootstrap/ductus.md`.

**What this review read: all 3 files in scope.** `specs/034-performance-backend-rules/spec.md`, `framework/rules/performance-backend.md` (the artifact every criterion asserts about), and `framework/bootstrap/ductus.md`'s manifest row for AC7. The category-disjointness check additionally extracted the ID sets from `security-backend.md` and `api-backend.md`, which are outside this scope. 034 has no scenarios and no data model, so its `reviewed-digest` is empty — taken and empty, which reads as current, rather than absent and unjudgeable as before.

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
