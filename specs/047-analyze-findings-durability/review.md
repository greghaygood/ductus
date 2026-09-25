---
spec: 047-analyze-findings-durability
last-run: 2026-09-25T23:39:46Z
reviewed-against: 39856db620fc3891be9e2d0bd757407fe5a85de5
diff-base: 92b4b18a5812e9c8fe9f1707f62acd052f0620c7
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 27
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 825f3b4cbe5d780f4d170ff0cbc2bea347d2dfc06c4aacc8c44504ed26f48cfd
  scenarios/analyze-run-durability.md: f186704c8fb9758c749a01ee92415dd0def86147f0e5ecad08f114638278b47c
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 047-analyze-findings-durability

## Summary

Re-review for 058 task 42, scoped to the contract that reopen changed. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. One observation, fixed in the run.

**What changed.** `scenarios/analyze-run-durability.md` gains a "Changed by 058" note under **The drift family**: `analyze-state-drift` left `check-artifacts`, because read during detection it judged the record the run was about to replace. `/{project}:analyze` now judges analyze-state drift from the record it writes, which always carries `last-run`, so the check reports a `done` spec whose new record is blocking. `spec.md` changed only in its status line.

**The observation.** Directly below the note, the grandfather section still said "The drift family exempts them". Task 42 corrected that same sentence in `scripts/audit/analyze-record-backlog.sh`, `framework/commands/audit.md`, and `scripts/audit/README.md`: since 058 the CI template's analyze-record check (`framework/templates/ci/adopter-generators.yml`, the `analyze-exempt` high-water mark) is what exempts them. The sentence now says the drift family exempted them and names the CI check as what exempts them since 058. The quality pass was re-run over the edited section and found nothing further.

**Scope, and what was not re-read.** `diff-base` 92b4b18a (the parent of 39856db6, where 047 re-entered `in-progress`). 27 files in scope; examined **2**: the scenario (the drift-family note and the grandfather section beside it) and `spec.md`, grep-checked for any other place naming the drift family or the grandfather rule (AC10 and AC11 name neither the family nor its exemption mechanism). **The rest of the scope was not re-read.** That includes this spec's other scenarios and the runtime, command, and audit files 058 task 42 changed, which 058's own review covers. The note was checked against the code at 39856db6: `check_artifacts.rs` emits no `analyze-state-drift`, and `framework/commands/analyze.md` §Analyze state drift judges the record written after step 19.

**Passes.** Security, reuse, and efficiency had nothing to check, because the changes are prose. Quality: the note matches the code, and the one stale sentence is fixed. Simplicity: the note states the change once and points to where the check now lives.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the grandfather section still names the drift family as what exempts done specs predating the record; since 058 the CI template's analyze-record check does — `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md:70` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
