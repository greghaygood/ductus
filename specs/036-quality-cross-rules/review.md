---
spec: 036-quality-cross-rules
last-run: 2026-09-28T15:10:44Z
reviewed-against: aac499d331586f2cc4a171497535a4b6df2076f2
diff-base: 47e29c72f8783e834aceb1c31b0fdd8849f5d57d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 9
scope: 12
skipped-passes: []
reviewed-digest:
  data-model.md: 93794b99d80a5e03fe22e3b991a19cbdcfa3b6e3d16b2c5000bcda3842b2212b
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 036-quality-cross-rules

## Summary

Scoped re-review of 036's reopen for QUAL-TEST-001, which 050's third round routed here (task 10), and for task 11, which corrects two sibling rules' claim that a SHOULD finding leaves done unblocked. Diff base is 47e29c72, the parent of the reopen commit. 0 MUST, 0 SHOULD, 0 low-confidence, no observations; not blocking.

Examined 9 of 12, each read in full as a diff over the window: framework/rules/quality-cross.md (also read whole, including the new QUAL-TEST section), specs/036-quality-cross-rules/spec.md, data-model.md and tasks.md, framework/commands/analyze.md, framework/constitution.md, AGENTS.md, and specs/050-constitution/plan.md and tasks.md, which are in this window because 050's third round was reopened alongside. The quality pass checked that QUAL-TEST-001's Statement keeps one tier (SHOULD and SHOULD NOT) and carries all four fields. It checked that its discriminators against QUAL-CLAIM-001 and QUAL-STUB-001 hold. It checked task 11's corrected sentence against check_artifacts.rs:665, where analyze reports a done spec with an outstanding SHOULD as drift, and against constitution §implement-phase. It also checked that the promoted §scenarios paragraph matches append_task.rs:94-118. lint-rule-ids reports 208 rule IDs.

Not opened: .claude/commands/ductus/analyze.md, generated from framework/commands/analyze.md (gen-claude-commands --check in sync); and framework/bootstrap/ductus.md and scripts/lint-rule-ids.sh, plan-affected but unchanged in this window. The new rule adds no manifest row, because quality-cross.md already ships, and the QUAL surface is already on the lint allowlist.

Local gate after the change: every framework-checks step and scripts/audit/run-all.sh exit 0; cargo fmt, clippy -D warnings and cargo test --release --locked exit 0 (1713 passed, 20 result lines for 20 targets).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
