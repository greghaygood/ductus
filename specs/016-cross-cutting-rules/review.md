---
spec: 016-cross-cutting-rules
last-run: 2026-09-28T14:53:58Z
reviewed-against: d7b53be08dae2d7d8c4b50853def6268d5f6bbf7
diff-base: c37e5723b893e5b9730349dec40da1c961b3bc99
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 15
skipped-passes: []
reviewed-digest:
  scenarios/applicable-rules-consistency-check.md: eac42f86ea49840c67eb395c8993822a3c61f581314db66d6cdde79d613f3056
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 016-cross-cutting-rules

## Summary

Scoped re-review of 016's reopen, which exempts code-pattern rules from the Applicable Rules cited-but-does-not-fire check (task 10, surfaced by 061's analysis). Diff base is c37e5723, the parent of the reopen commit. 0 MUST, 0 SHOULD, 0 low-confidence; 1 observation, fixed; not blocking.

Examined 8 of the scope. Read in full as diffs over the window: framework/commands/analyze.md, framework/templates/spec/spec.md, specs/016-cross-cutting-rules/scenarios/applicable-rules-consistency-check.md, spec.md and tasks.md, and specs/058-findings-route-at-discovery/spec.md and tasks.md, which are in the window because 058's cycle followed this reopen. framework/constitution.md is unchanged in this window, and its one changed bullet (058's) was read. The exemption's claim was checked against framework/rules/quality-cross.md, which is outside this scope. There, 3 of the 4 rules' Verification reads "flags a code path" and QUAL-DELEG-001's reads "flags a change in scope". That is the observation, fixed at d7b53be0 by rewording both occurrences to "flagging code in scope"; lint, the 13 parity tests, parseability and the self-audit re-ran green afterwards.

Not opened: .claude/commands/ductus/analyze.md and .claude/commands/ductus/groom.md, which are generated from their framework/commands/ sources (gen-claude-commands --check in sync). framework/commands/groom.md, specs/008-security-rules/spec.md and specs/016-cross-cutting-rules/plan.md are plan-affected but unchanged since 016's prior review. specs/058-findings-route-at-discovery/review.md and analysis.md are records written this session by write-review and write-analysis; only their frontmatter was read, to verify the recorded sha.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the code-pattern exemption says each quality-cross rule's Verification flags a code path, but QUAL-DELEG-001's flags a change in scope — `specs/016-cross-cutting-rules/scenarios/applicable-rules-consistency-check.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
