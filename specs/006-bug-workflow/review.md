---
spec: 006-bug-workflow
diff-base: 80b817ecc2aaf578e01f7baaa05904c0a2ad2614
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:09:21Z
reviewed-against: 80b817ecc2aaf578e01f7baaa05904c0a2ad2614
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 10
scope: 17
reviewed-digest: {}
blocking: false
---

# Review — 006-bug-workflow

## Summary

Re-review of 006 after restoring the cross-spec signpost that this campaign's own earlier commit removed. No MUST or SHOULD violation. All nineteen criteria re-verified against the tree.

**What this review read: 10 of the 17 files in scope, and here is the other 7.** Read in full: `framework/constitution.md`, `framework/commands/amend.md`, `groom.md`, `analyze.md`, `help.md`, `status.md`, `README.md`, `framework/templates/spec/scenario.md`, `framework/templates/spec/spec.md`, and `framework/templates/project/inbox.md`. **Not examined: five `.claude/commands/ductus/*.md`** — generated copies of the sources above, held against them by the audit's check-zero precondition, which ran clean; counting them would double every source. **And two paths that no longer exist** — `commands/next.md` and `.claude/commands/ductus/next.md`, retired with the `/next` command, which the plan still lists as the record of what the implementation touched.

**What changed since the previous run** (`80b817e`). `3c4fe76` removed three `> **Note:**` blocks from this spec. Two were genuine rename annotations and stay removed. The third was not an annotation at all — it recorded that `triage` was renamed to `inbox` by 011, which is a **§cross-spec-impact signpost**, and 011's AC14 is discharged by its presence. Removing it falsified that criterion silently: no check verifies cross-spec signposts, `check-artifacts` stayed clean on both specs, and 011 would have kept reading as green. §drift-prevention's rule against annotating a rename carries exactly one exception — "the retired name surviving only in the few references that record the decision to change it" — and this is that reference. Restored as a blockquote so `derive-dependencies` induces no edge (confirmed: `dependencies` stayed `[]`), matching the convention 007 uses for its signpost from the same spec.

**How each criterion was checked** is unchanged from the previous run and is not restated here in full: AC1–AC3 against the three templates, AC4–AC6 against the constitution's §bug-handling, §scenarios and §spec-phase, AC7–AC9 against `groom.md`, AC10 against `help.md`, AC11–AC14 against `amend.md`, AC15 against `status.md`, AC16 annotated (the `/next` command was retired), AC17 against `analyze.md`'s scenario-consistency family, AC18 against `README.md`, and AC19 by `markdownlint-cli2` over 512 files.

**The defect the previous run fixed stays fixed.** `help.md` §Key Concepts gave the bug decision tree as three steps where the constitution has four; the rules tier 016 added was missing from a file adopters receive. Corrected in `3b5bcd0` and re-read here.

**On the diff base.** No commit records 006 entering `in-progress` from its original implementation, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead. 006 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
