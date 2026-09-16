---
spec: 016-cross-cutting-rules
diff-base: 510eb25cfd96bc5ac2bcc714054c482a2c3cbfe1
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T12:30:07Z
reviewed-against: 19a236adfd41281319991094525535ded7586e91
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 17
reviewed-digest:
  scenarios/applicable-rules-consistency-check.md: 654067cf29c934cccf9a88046d9aedf9b61728af9c5353037bb45ff4adf7d145
blocking: false
---

# Review — 016-cross-cutting-rules

## Summary

Reopened by a factual correction rather than by the filename sweep. 016's §Edge Cases bullet cited *"the README's 'Pinning files with …' section"*, and the README carries no such section under any name — it was restructured, and `[pinned]` is now documented under §Configuration. The pointer was dead before the sweep touched the filename inside it, which is why it could not ride the sweep's exemption: repointing rewords the line, and a reworded line is a factual correction that takes the back-edge. It now names §Configuration, which `resolve-anchor` classifies as a qualified reference. No MUST or SHOULD violation against the loaded rules is outstanding.

**Criterion verification.** All eight ticked criteria were re-checked against the tree rather than taken as banked: AC1's `<!-- §rules -->` marker is present in `framework/constitution.md`; AC2's fourth decision-tree route is in §bug-handling; AC3's section in `framework/commands/analyze.md` is `### Rules (blocking and advisory)`, renamed from "Security rules" as required; AC4's `## Applicable Rules` section is in `framework/templates/spec/spec.md`; AC5's `rule` route is in `framework/commands/groom.md`'s decision tree; AC6's signpost is at the top of `specs/008-security-rules/spec.md` and links to 016; AC7 and AC8 re-verified by running `markdownlint-cli2` and the deterministic analyze families clean. Every `§` reference in the spec resolves — 11 of them, 0 unresolved.

**What this review read, and what it did not.** The five passes read 7 of the 17 in-scope files: 016's `spec.md`, `framework/rules/quality-cross.md`, and the five artifacts AC1–AC6 assert against (`framework/constitution.md`, `framework/commands/analyze.md`, `framework/commands/groom.md`, `framework/templates/spec/spec.md`, `specs/008-security-rules/spec.md`). The rest are 016's `plan.md` and `tasks.md`, the two generated `.claude/commands/ductus/` copies, and 017's and 027's artifacts — the latter pulled in because they were modified in the same window by the sweep, not because 016 bears on them; they carry their own reviews. None of those was re-read this run.

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
