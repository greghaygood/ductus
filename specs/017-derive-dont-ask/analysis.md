---
spec: 017-derive-dont-ask
last-run: 2026-09-28T00:12:33Z
analyzed-against: a42351706b70e73d3907d943a8e88d3816989461
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 10
analyzed-digest:
  data-model.md: bc0dbdffb2f2dc1409cbae6c53d9acc9b30c77bb7671ac1909ebbe1071adc01b
  plan.md: 371921636aa901292a365fd571854ce16a585733c5d0da8fe72a00bf235814ec
  review.md: 5977fea2707e9a246e3ee2b6ea7c114680e6c0d123e915f4d8c952272c4718c6
  scenarios/detect-dependency-cycles.md: 2c426d44ce0a3a3cf5e91160fdf84ac054795737b25979152f70e89fe86005ab
  scenarios/generator-sync-claim-honesty.md: 0ee16bda3f0a5c0658100fee3d28ddae8214bc0eca9d57261811077aae314264
  scenarios/skip-prose-cross-references.md: 1b5c4bd36e2ca760437d63b948706e5b575ecd450b0779d1d10440cc77d0e086
  scenarios/tracked-specs-not-worktree.md: ba42aee5fbee0dfed38fce985200b0ba5775ecd03113a81aa792b25a18383829
  spec.md: ca9ca894a64c4691151943de7fb975b7f5c87502bc3760b1229265d054358762
  tasks.md: 375377a3d6f43f25b3f0565d366a534d5841744f5db3653b31993ad0b57b37a7
unexamined-by-reason:
  no-readable-state: 1
  not-a-live-claim: 9
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 10 unexamined target(s). Dispositions: 1 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- no-readable-state: 1
- not-a-live-claim: 9

## Fixed in this run

- grounding — spec.md's CI safety net paragraph and AC24 stated the current CI behavior — generators run for real, then the tree is compared — without citing the workflows that do it — `specs/017-derive-dont-ask/spec.md` — **fixed**
