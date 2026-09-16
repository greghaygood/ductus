---
spec: 037-observability-backend-rules
diff-base: e9b511ff8e22bef2bc45926e45b6c63a05405402
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T12:42:21Z
reviewed-against: e9b511ff8e22bef2bc45926e45b6c63a05405402
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 2
reviewed-digest: {}
blocking: false
---

# Review — 037-observability-backend-rules

## Summary

First review of 037 to record `examined` against a derived `scope`; the prior record predated those fields, so its `0/0/0` could not be distinguished from a run whose passes never fired. No MUST or SHOULD violation; all seven criteria verified against the tree rather than taken as banked, and nothing needed correcting.

**How each was checked.** AC1: `framework/rules/observability-backend.md` exists with the `-backend.md` suffix and the canonical `### {ID}` / Statement / Rationale / Verification schema. AC2: the seven IDs span `METRIC`/`TRACE`/`HEALTH`, and intersecting those against `security-backend.md`'s eight categories, `api-backend.md`'s seven and `performance-backend.md`'s five confirms disjointness; `lint-rule-ids.sh` exits 0. AC3: the header declares all three by name. AC4: every one of the seven Verification clauses is a design-time commitment phrased against a spec or plan, not a code-pattern grep — checked by extracting each clause and matching its phrasing, which is the stronger reading of the criterion than confirming the three subjects are merely present. AC5: exactly two MUSTs, and they are precisely the pair the Severity posture names — `BE-HEALTH-001` (no readiness probe ships silent bad deploys) and `BE-TRACE-001` (broken trace-context propagation makes distributed failures undebuggable); the other five are SHOULD. AC6: `BE-LOG-006` is cited for the logging-correlation seam and `CFG-CONST`/`CFG-ENV` for tunable config, with no restatement of either. AC7: one **Shared Files** manifest row in `framework/bootstrap/ductus.md`.

**What this review read: both files in scope** — `framework/rules/observability-backend.md` and `framework/bootstrap/ductus.md`'s manifest row. `specs/037-observability-backend-rules/spec.md` was read in full as well and is deliberately not counted, since it falls outside the resolved scope and counting it would make the ratio overstate coverage.

**On the diff base.** No commit in this history records 037 entering `in-progress`, so the natural derivation yields an empty base. That is recorded here as `HEAD`, an empty window, which resolves the scope to the plan's Affected Files — the honest description of what this review covers. The alternative, basing on the parent of 037's first commit, resolves an 800-file window spanning every change since June and would describe a review nobody performed. 037 has no scenarios and no data model, so `reviewed-digest` is empty: taken and empty, which reads as current, rather than absent and unjudgeable as before.

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
