---
spec: 052-spec-supersession-and-consolidation
last-run: 2026-09-30T17:26:34Z
reviewed-against: 00ba8b753531f7ea7f7695ee5ae8766e3e8a849e
diff-base: 628a89dbb3b01dbd068ce4344eedc97b630c819e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 13
skipped-passes: []
reviewed-digest:
  scenarios/stranded-session-after-removal.md: 4883250ac10f3dfad056127377b33924382c3e8fb5487e212fc1e2ccbb57b0e0
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "signpost: 051's post-completion note links 052 by its former title 'Spec supersession and consolidation' — `specs/051-branch-scoped-spec-numbering/spec.md:46`"
    outcome: discarded
    reason: a signpost written when 052 carried that title; the link resolves, and retitling it alone would reopen 051 for a label
    decided-at: 2026-09-30T17:26:34Z
    decided-by: andy@stone.dev
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Not blocking. A re-review of 052 after its reopen (0a1aad71), routed from 048's review because the previous record (a21e707a) described its seven unread files as "each unchanged since the 2026-09-13 full review" when all seven had changed since (`git log 8c01d6f5..6940509c`). That description was false, and this record replaces it. Natural base `628a89db`, the commit before the reopen, resolving 13 paths. Read in full, all five passes: `framework/commands/consolidate.md`, whose Purpose 446fbf2d rewrote and which AC25 names; `framework/commands/fold.md`; `runtime/src/primitives/retire_feature.rs`; `specs/052-spec-supersession-and-consolidation/spec.md` and `tasks.md`; and, as modified-since paths, `specs/041-task-pruning/tasks.md` and `specs/048-govern-acquired-runtime/spec.md`. AC7-AC12, AC25, AC34, AC42 and AC44 hold against the command and the primitive; AC39 holds against 051's post-completion note. No rule finding. Two observations were fixed in the run (00ba8b75), and one was discarded with its reason. Not read in full, named individually: `README.md` (only its `/consolidate` line was re-read here, though 048's review read the whole file earlier this session); `framework/bootstrap/ductus.md` (only the consolidate manifest row, line 822); `runtime/src/schema/primitives.rs` (only `RetireFeatureArgs` and `RetireFeatureResult`, though 048's review read the whole file this session); `specs/051-branch-scoped-spec-numbering/spec.md` (only the post-completion note AC39 names); and `specs/048-govern-acquired-runtime/analysis.md` and `review.md`, records 048's own runs wrote this session, which are no surface of 052. The durable contract `scenarios/stranded-session-after-removal.md` is unchanged since the previous record's digest. This is not a full five-pass review of every path in scope.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- doc: a_sequential_feature_is_refused_outright was documented 'this primitive can never remove one', false since 052's opt-in — `runtime/src/primitives/retire_feature.rs:247` — **fixed**
- claim: AC9 named the target's review: block, a record spec 057 moved to review.md — `specs/052-spec-supersession-and-consolidation/spec.md:48` — **fixed**
- signpost: 051's post-completion note links 052 by its former title 'Spec supersession and consolidation' — `specs/051-branch-scoped-spec-numbering/spec.md:46` — **discarded**: a signpost written when 052 carried that title; the link resolves, and retitling it alone would reopen 051 for a label

## Skipped passes

*None.*

## Unexamined governance

*None.*
