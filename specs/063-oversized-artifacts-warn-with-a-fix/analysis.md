---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-04T18:18:21Z
analyzed-against: 58e6be16018c5fbe2ca17e341a4fc6695c4bb084
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 0
analyzed-digest:
  data-model.md: 5e75019313bc78379427ac5eaef318d638b53e916cda686a55bdc338c3e2ef6c
  plan.md: f63041caa10846cf32aeffd5c82ff3080e3546a15586e550a839eaf369f9e863
  review.md: 21581682622b20ffc6daac4a107bea52f14bbc93c0f4f3bc517a1f52a53b74d0
  spec.md: fb805071cf3031784765c227666cebbdb887dd980443c545304e5849302c2531
  tasks.md: 5dfe5cdf9f8071379289e05433cc309a7a4d38bc0faa2cf1b3245dc513195c40
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 2
  undispositioned: 0
decisions:
  - key: grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source
    outcome: discarded
    reason: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics"
    outcome: discarded
    reason: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 0 unexamined target(s). Dispositions: 2 fixed, 0 routed, 2 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- grounding — spec.md says CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS changes Claude Code's read cap, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
- grounding — plan.md justifies its stat over a read with a 117 KB data model it does not name — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **fixed**
