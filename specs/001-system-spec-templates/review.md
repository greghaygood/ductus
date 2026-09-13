---
spec: 001-system-spec-templates
reviewed-at: 2026-09-13T12:54:02Z
reviewed-against: 4ba8a955a5c7a7b843b1bff2d9e188dbd166a712
diff-base: 4ba8a955a5c7a7b843b1bff2d9e188dbd166a712
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 3
scope: 3
skipped-passes: []
---

# Review — 001-system-spec-templates

## Summary

First review of 001 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All six criteria were verified against the tree rather than banked, and three of them did not hold as written — corrected in `4ba8a95` before this review ran.

**What this review read: all three files in scope** — `framework/templates/project/system.md`, `errors.md`, and `events.md`, each in full. Nothing in scope went unread. `spec.md`, `plan.md` and `tasks.md` were read as well and are deliberately not counted, since they fall outside the resolved scope and counting them would make the ratio overstate coverage.

**The correction.** AC1–AC3 named `templates/{system,errors,events}.md`, a path that stopped resolving when `3fc76b7`/`8558ac8` reorganized the repo around `framework/`. Only the files moved, so under §drift-prevention's retired-filename rule the criteria were swept rather than annotated, which leaves each one true. The body's `> **Note:**` was that annotation and was removed: it preserved the retired name permanently, while the fact it carried — that adopter destination paths are unchanged — is recorded canonically in the **Shared Files** manifest. `plan.md`'s Affected Files table carried the same dead paths, which is why the scope resolved to three nonexistent files before the fix. `plan.md`'s Trade-offs section deliberately keeps `templates/`: it records the layout decision, the one place the rule leaves a retired name standing.

**How each criterion was checked.** AC1: `system.md` carries Configuration, Application Lifecycle, Request Lifecycle, Shared Infrastructure and Module Pattern — all five the criterion names, each a prompt with a commented example. AC2: `errors.md` carries Error Response Format, Error Codes, Status Mapping, Validation Errors and Logging — all five. AC3: `events.md` carries Envelope Format, Naming Convention and Event Catalog, and its header comment makes the retry/dead-letter suggestion verbatim; the enumeration was checked term by term rather than the requirement alone, which is what caught the Task-9 specs' AC6 drift earlier in this campaign. AC4: all three use `<!-- -->` with commented-out example content, and comparing against the other templates confirms the "consistent with existing template style" half — 11 of the 13 files under `framework/templates/` use the same device. AC5: the examples are JSON payloads, env-var names and HTTP status codes — a wire format and a protocol, not a language or a framework; `errors.md` hedges "or equivalent" on status mapping and `system.md` says "request or message", so the agnosticism is deliberate rather than incidental. AC6: each file opens with a single top-level heading, and `markdownlint-cli2` exits 0.

**On the diff base.** No commit in this history records 001 entering `in-progress`, so the natural derivation yields an empty base, and `write-review` would collapse the denominator to `scope: 0` — the incoherent record this campaign exists to stop writing. `HEAD` is passed instead: an empty window by construction, which resolves the scope to the plan's Affected Files and makes numerator and denominator describe the same three files. 001 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current, rather than absent and unjudgeable as before.

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
