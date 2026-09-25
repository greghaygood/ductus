---
spec: 047-analyze-findings-durability
last-run: 2026-09-25T14:33:17Z
reviewed-against: aea28e92da8ddda1cef10f1869e98ccc4f824bc8
diff-base: 52de7ca211ade2801b52e8dd96a8b30d6a569ccd
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 7
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 825f3b4cbe5d780f4d170ff0cbc2bea347d2dfc06c4aacc8c44504ed26f48cfd
  scenarios/analyze-run-durability.md: 69a78c9c7ba564c943f130195384de5d1d020eea7cf4ed747e9f1770edaddd07
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 047-analyze-findings-durability

## Summary

Re-review for spec 058's cross-spec discharge (058 task 24). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. One observation, fixed in the run.

**What changed.** 058 reversed this spec's capture half: `/ductus:analyze` no longer appends findings to the inbox, and each live finding is fixed, routed, or discarded in the run that finds it. `spec.md` gains a second signpost naming the reversal and what survives (the run record, the second gate, the digest freshness check, AC9–AC15); §Behavior and §Constitution amendment are annotated rather than rewritten, since they are the design 058 replaced; AC1–AC7 are annotated in place; each Resolved Question gains a 058 sub-bullet; and both `#automatic-issue-capture` links now point at `#finding-dispositions`.

**Observation, fixed.** The `analyze-run-durability` scenario's Resolved Questions still said the inbox is where the findings live and that `--fix` is the only path that mutates an audited artifact. Both were falsified by 058. With confirmation, each gained a 058 sub-bullet, matching `spec.md`. That edit changes a durable contract, so this record's digest covers it.

**Scope.** `diff-base` 52de7ca2, 7 in scope, examined **2**: this spec's `spec.md` and `scenarios/analyze-run-durability.md`, both read in full. The five unread: `specs/008-security-rules/spec.md` is 008's own discharge and is reviewed there; `specs/058-findings-route-at-discovery/tasks.md` is 058's; `framework/constitution.md`, `framework/commands/analyze.md`, and its `.claude/` mirror are this spec's historical Affected Files, unchanged since the diff base, and their current text was checked only in the regions the annotations cite (Finding dispositions; analyze.md's scope boundary and steps 16–20). `analyze-record-freshness.md` was not re-read; its one capture-era mention describes review's output before 047 and is history.

**Passes.** Security, reuse, and efficiency had no subject: prose in one spec and one scenario. Quality, against `quality-cross.md`: each annotation was checked against the shipped behavior it claims. Simplicity: annotation over rewrite keeps the replaced design readable without restating 058.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: two Resolved Questions in analyze-run-durability still stated capture-era behavior 058 falsified — `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
