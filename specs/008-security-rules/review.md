---
spec: 008-security-rules
scenario: x-frame-options-carries-one-strength
last-run: 2026-09-27T21:41:10Z
reviewed-against: e13765301207955bb901ba321d899fcfa9b391ad
diff-base: f76b222737b0c53d2a638e3aa5578f0574988475
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 9
skipped-passes: []
reviewed-digest:
  data-model.md: 4ab1f01634418baee8b7f06d1348ceacbfff8b665703782bfcb408ecb3cf89bb
  scenarios/a-statement-carries-one-obligation-keyword.md: 346b88a803f48662192dda28a853aa0d8bd879e1d2b4108f9a3a08f2bb81ecc2
  scenarios/x-frame-options-carries-one-strength.md: 8cf5c99537d8f9cf81233184b4b13d76f7c3bc66a0ce96f0effb075572e53fc4
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 008-security-rules

## Summary

Re-review for task 11, scenario `x-frame-options-carries-one-strength`, the route the previous review (f76b2227) made. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. One observation, fixed in the run. The previous review's one stored decision, the X-Frame-Options route, expired because its finding no longer fires.

**What changed.** `X-Frame-Options` is a SHOULD in both security rule files. `BE-API-001`'s minimum-required header table no longer lists it, and its Rationale points at the new `BE-API-012`. `BE-API-012` (SHOULD) mirrors `FE-CSP-008` for a backend that serves HTML without loading the frontend rules, and `FE-CSP-008` names it. That makes 207 shipped rules, none mixing tiers, with every citation in both files resolving (`check-rule-ids`, 0 missing) and the rule-section corpus tests green.

**Base.** `diff-base` f76b2227, the previous review's commit, passed with `--since`. That review recorded 0 MUST and 0 SHOULD over 23 files from base f0b59a9e, 18 of them read in full, and everything outside this delta is unchanged since. The derived base, the parent of 008's reopen commit, would re-cover the same 52-file window that review narrowed for the reason its Summary gives.

**Scope.** 9 in scope, examined 5: the five files changed since the base. For each, this review read its whole change since f76b2227 in context. All five were also read in full earlier the same session: the X-Frame-Options scenario while it was clarified, and the other four by f76b2227's review. The four not counted are plan-listed Affected Files with no change since the base: `framework/constitution.md` and `specs/008-security-rules/data-model.md`, both read in full by f76b2227's review, and `framework/bootstrap/ductus.md` and `framework/commands/analyze.md`, not read.

**Passes.** Loaded: api-backend, concurrency-backend, configuration-cross, observability-backend, performance-backend, quality-cross, reliability-backend, security-backend. Security and efficiency found nothing: the change is rule prose. Quality: `BE-API-012` parses SHOULD-tier with a Verification, and `BE-API-001`'s Statement still asks for "frame protection", which its CSP `frame-ancestors` row provides. A sweep for claims task 11 falsified found one, fixed in e1376530. Reuse: `BE-API-012` restates `FE-CSP-008` deliberately, as `BE-AUTHN-014` does `FE-STORAGE-002`, and says so in its Rationale, so it is not an observation. Simplicity found nothing.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the keyword scenario's Q1 measured cost said the two SHOULD-half citations are re-pointed to the new ID, which task 11 made false for the security-backend one — `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
