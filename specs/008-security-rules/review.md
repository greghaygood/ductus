---
spec: 008-security-rules
last-run: 2026-09-25T18:06:29Z
reviewed-against: d644580a57b9dfa55531ed215a6af4fa17ac0ac0
diff-base: 52de7ca211ade2801b52e8dd96a8b30d6a569ccd
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 12
skipped-passes: []
reviewed-digest:
  data-model.md: c56519ec36023061bd87268dfae78612cc2ebb239ae4ac34b2b0abd0838a869d
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 008-security-rules

## Summary

Re-review for spec 058's cross-spec discharge (058 task 24). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. One observation, fixed in the run. No durable contract changed: `data-model.md` is untouched, so `reviewed-digest` is unchanged.

**What changed.** 058 removed the brownfield security audit's inbox write. `spec.md` §Brownfield Adoption is rewritten to the shipped audit: it reports each gap under its spec, writes nothing, and hands off to `/ductus:analyze`, whose run dispositions the gap. The section opens with a blockquote signpost to 058. The trigger and audit logic are unchanged; §Finding format drops the checkbox, §Idempotency and §Reporting describe a run that writes nothing, and the rejected-alternative paragraph records why the inbox argument was reversed. AC22, AC25, AC26, and AC27 are annotated in place rather than rewritten.

**Observation, fixed.** `plan.md` §"Brownfield audit lives in ductus" still described the inbox sink. With confirmation, it gained a blockquote naming the change and pointing at `spec.md` §Brownfield Adoption; the plan's steps stay as the design first built.

**Scope.** `diff-base` 52de7ca2, 12 in scope, examined **1**: this spec's `spec.md`, read in full. The eleven unread: five are 047's and 058's discharge files, reviewed under their own specs; `framework/rules/security-backend.md`, `security-frontend.md`, `framework/constitution.md`, `framework/commands/analyze.md`, and `framework/bootstrap/ductus.md` are this spec's historical Affected Files, unchanged by this discharge; `data-model.md` is unchanged. The shipped audit the rewrite describes lives in `framework/bootstrap/ductus-procedure.md` §Security Audit (brownfield) and §Security audit summary, outside scope; those sections were read to check the rewrite against them.

**Passes.** Security, reuse, and efficiency had no subject: prose in one spec. Quality: each rewritten claim was checked against the procedure it describes. Simplicity: §Reporting describes the summary block instead of quoting it, so the procedure stays its one verbatim home.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: plan.md's brownfield-audit section still described the inbox write 058 removed — `specs/008-security-rules/plan.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
