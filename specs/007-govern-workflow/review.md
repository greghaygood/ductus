---
spec: 007-govern-workflow
last-run: 2026-09-28T13:12:56Z
reviewed-against: e3ef49741eee957955fd5f6d0646aa026f1f1021
diff-base: aa5a95a18c4a4e7236cf50bfe535336b7b705151
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 20
skipped-passes: []
reviewed-digest:
  scenarios/ductus-self-update-precheck.md: 1ad54d483e9606fb73440abaec7df167f22f13cd35ca2c251df1e6c3d2c46686
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 007-govern-workflow

## Summary

Reopen for 061 (the self-update's source and the fetch model). Default diff base: the parent of this reopen's done -> in-progress commit, so the window is the reopen itself; compute-review-scope reports 2 paths modified since it (spec.md and scenarios/ductus-self-update-precheck.md). Five passes (security, reuse, quality, efficiency, simplicity) against all 11 rule files discover-rule-files selected, each read in full, plus AGENTS.md read in full. 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations. examined: 2 of 20 in scope -- spec.md and the scenario, both read in full after the edit. The pass checked the two signposts against framework/bootstrap/ductus.md as committed at 1d696f53 (Source resolution, the Small fetch at {raw-ref}, {framework-root}) and confirmed each claim; the corrected superseded-in-part note no longer asserts that the per-file fetch workflow is current. NOT read, named rather than counted: the .claude/commands/ductus/*.md glob (18 generated mirrors, unchanged in this window) and framework/bootstrap/ductus.md (read in full earlier this session and edited by 061 tasks 1-3, unchanged in this window). ABSENT but in scope because the plan lists them, all from the pre-reorganization layout: commands/{about,analyze,clarify,commit-push,implement,next,plan,scenario,setup,specify,status,target,triage}.md, ductus/ductus.md, ductus/ductus-auggie.md, and .claude/commands/ductus/init.md.

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
