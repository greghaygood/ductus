---
spec: 048-govern-acquired-runtime
last-run: 2026-09-28T13:27:19Z
analyzed-against: 113a30f66a2a45cd1ea66b9834032ea97b5ee14e
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 7
analyzed-digest:
  data-model.md: aaabca49a7dd3b0e8c5a3fe311f77ba06c98b5f3d106cbc19c3c846d9fcd9a33
  plan.md: cbadd9ee50ddab1c9b7cf5caa0f3f1ac1b6433f192454e6fe74a749eb59596a2
  review.md: 8e16158b3867e4b7bf09c681910f2f2ea1d2073f0519544c7844b71c00623434
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 0c5984d011f5d35b3685f05bc083e01d9aaa7bac4a31c0949e1d5bcd1f00627b
  scenarios/state-a-version-checks-the-pin.md: 40a4cbaf5dab4b5ee1127c4e808f70bbd08433415e952011ceb7c419a5e24e23
  scenarios/state-b-continues-in-session.md: b4d9b7da7ad0d9486268cff26ec375aa64d35a9639a5828a03cdde92da756ddd
  spec.md: f9be44241c519fe6860ba86e7996770458da8173ea073e1c934237c992aacb35
  tasks.md: 18351012539afba44c73fdd3ae5c8cbc6c543c44fbff4cf07c27ca0c879ddad6
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 2
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 048-govern-acquired-runtime

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 7 unexamined target(s). Dispositions: 1 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 2

## Fixed in this run

- grounding — spec.md's store-cost paragraph asserted how auggie and antigravity register their MCP server without citing a source — `specs/048-govern-acquired-runtime/spec.md` — **fixed**
