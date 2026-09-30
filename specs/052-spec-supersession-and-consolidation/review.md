---
spec: 052-spec-supersession-and-consolidation
last-run: 2026-09-30T03:11:24Z
reviewed-against: 6940509c1ad4fc3087e7178989ebf18ca18ae6c0
diff-base: 9a6c79b4ca91367fb73f24f19e88d21d208ba392
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 8
skipped-passes: []
reviewed-digest:
  scenarios/stranded-session-after-removal.md: 4883250ac10f3dfad056127377b33924382c3e8fb5487e212fc1e2ccbb57b0e0
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Deliberate partial re-review of 052 after its reopen (7ef83f5f) to discharge 041's cross-spec impact. examined: 1 of 8, and that is the honest number. The run exists to give 052's review record the dispositions map it predates, and to review the one file the reopen changed. It is not a full pass. The previous record (examined 9 of 9, 0/0/0, at 8c01d6f5) remains the last full five-pass review of 052.

BASE. The natural base, 9a6c79b4, is the commit before 052's reopen. It resolves 8 paths: the one file modified since, plus the 7 plan-affected entries.

READ IN FULL, all five passes: specs/052-spec-supersession-and-consolidation/spec.md. The reopen changed two things. The One-spec and two-spec commands paragraph now splits commands by how many specs one operation writes, and says a batch flag repeating a one-spec operation (analyze --all, prune --all) leaves that scope intact. A Signpost blockquote links back to 041. The existing link to 041 also names its broadened title. Both claims hold against the tree. framework/commands/analyze.md's --all scans every feature directory and reports per feature. 041's prune-tasks and prune-plan walks write, or preview, one spec at a time. /fold and /consolidate still write two specs in one operation. AC34 still holds against docs/slash-commands.md, whose rule says the same thing.

NOT EXAMINED, named individually, each unchanged since the 2026-09-13 full review: runtime/src/schema/primitives.rs (its retire-feature arguments); runtime/src/primitives/retire_feature.rs; framework/commands/consolidate.md; framework/commands/fold.md; framework/bootstrap/ductus.md; README.md; specs/051-branch-scoped-spec-numbering/spec.md. consolidate.md did change once in the window, at line 18, where 041's link text took the broadened title in 9a6c79b4. That change is the base commit itself, so it sits outside this window.

WHAT THE PASSES COVERED. The subject is prose, so security, efficiency and simplicity have nothing to act on. Reuse: the paragraph states the rule, docs/slash-commands.md mirrors it, and the signpost points at the spec that caused the change rather than restating it. Quality: each claim matches the commands it names (QUAL-CLAIM-001). No observations were raised, so the dispositions map counts nothing. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking.

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
