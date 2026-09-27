---
spec: 008-security-rules
last-run: 2026-09-27T21:44:24Z
analyzed-against: 7d106937b5ddfa719392e7ff1c8d4cacb4e70f4e
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  data-model.md: 4ab1f01634418baee8b7f06d1348ceacbfff8b665703782bfcb408ecb3cf89bb
  plan.md: c897e7279cd5e1c78b0b98dca56c883e33e2b0ebb1d85385ab97873a7767d4f2
  research.md: d6f1450b6485bbb5ee823b2daabf5bdb146dbd34f5662f9a41baf0c3ff3ff68a
  review.md: 98187474d85780bcf8cf30fe9386931cb272273526128b34e34a320a75e9213f
  scenarios/a-statement-carries-one-obligation-keyword.md: 346b88a803f48662192dda28a853aa0d8bd879e1d2b4108f9a3a08f2bb81ecc2
  scenarios/x-frame-options-carries-one-strength.md: 8cf5c99537d8f9cf81233184b4b13d76f7c3bc66a0ce96f0effb075572e53fc4
  spec.md: a74df303a73222f51c005cd632d43548a33a395e42dcd2c9fa4dda44b61a29f1
  tasks.md: 79007311238e4c9f3c0e0382e6072c7a7752719b68dfa5e3bb386131f94dd9d7
blocking: false
dispositions:
  fixed: 3
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 3 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- grounding — plan.md:156 states the shipped rule counts as 73 backend and 32 frontend; the files carry 79 and 36 — `specs/008-security-rules/plan.md` — **fixed**
- grounding — plan.md:172 states governance has no programmatic tooling; the runtime parses the rule files — `specs/008-security-rules/plan.md` — **fixed**
- grounding — plan.md:139 cites spec 043-command-consolidation as owning the workflow registry; no such spec exists, and 043-workflows-sunset retired the registry — `specs/008-security-rules/plan.md` — **fixed**
