---
spec: 020-code-review
last-run: 2026-09-25T18:23:57Z
reviewed-against: cbc1f042b85cb720d31f1c86139774a70719a946
diff-base: 2c488bf7d3d45e4177e5ad005d32d04fb88d646c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 15
skipped-passes: []
reviewed-digest:
  data-model.md: a3f182639b7be06ee9bcf4d38b8cf5021adbf14a07ee91c1425d46613abb35cf
  scenarios/review-flag-parsing-is-specified.md: 9f1a3dd82bab2b9622808c31dd2bb00f0c6f403effeb8bb496bb4b329496e9c7
  scenarios/waiver-expiry.md: a06cf9e1d638fefd25d2edcf9d38a7ad56fa0b7e74ab048e4e77ce2878df77ca
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 020-code-review

## Summary

Re-review for spec 058's cross-spec discharge (058 task 25). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. Two observations, both fixed in the run.

**What changed.** 058 removed `captured-issues` from the review record and replaced it with a `dispositions:` map and a `decisions:` list. `data-model.md`'s `review.md` table, YAML example, and body-sections table now match `write-review`'s render order: `dispositions` after `blocking`, `decisions` after `waivers`, no captured-issues section, and an Observations row that renders each observation beside its disposition. `spec.md` gains a 058 signpost after the merge paragraph (now past tense), and the 057 signpost notes the replacement.

**Observations, fixed.** (1) §Embedded artifacts said the pre-done gate reads nine things; 058 added two, so it reads eleven (checked against `ReviewGateBlock` in `runtime/src/schema/primitives.rs`). (2) `data-model.md` §Finding record said a waived finding's reason comes from spec frontmatter; waivers have lived in `review.md`'s frontmatter since 057. Both were one-phrase chores, confirmed before writing.

**Scope.** `diff-base` 2c488bf7, 15 in scope, examined **2**: this spec's `spec.md` and `data-model.md`, both read in full. The thirteen unread: `scenarios/waiver-expiry.md` is unchanged and names no 058 field; 057's `spec.md` and `data-model.md` are reviewed under 057; 058's `tasks.md` is 058's; the rest are this spec's historical Affected Files, unchanged since the diff base. The record shapes were checked against `write_review.rs`'s frontmatter render and `decisions::render`, and against the constitution's Audit records table.

**Passes.** Security, reuse, and efficiency had no subject: prose in one spec and its data model. Quality: each corrected row was checked against the code that renders it. Simplicity: `data-model.md` points at 058's data model for the two new fields' shapes rather than restating them.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the embedded-artifacts note counted nine gate checks; 058 added two — `specs/020-code-review/spec.md` — **fixed**
- convention: a waived finding's reason was said to come from spec frontmatter, where waivers have not lived since 057 — `specs/020-code-review/data-model.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
