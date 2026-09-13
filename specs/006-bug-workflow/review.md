---
spec: 006-bug-workflow
reviewed-at: 2026-09-13T13:01:47Z
reviewed-against: 3b5bcd011801267c17aac17e6f85002b22808757
diff-base: 3b5bcd011801267c17aac17e6f85002b22808757
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 10
scope: 17
skipped-passes: []
---

# Review — 006-bug-workflow

## Summary

First review of 006 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All nineteen criteria were verified against the tree, and one real defect in a shipped adopter file was found and fixed.

**What this review read: 10 of the 17 files in scope, and here is the other 7.** Read in full: `framework/constitution.md`, `framework/commands/amend.md`, `groom.md`, `analyze.md`, `help.md`, `status.md`, `README.md`, `framework/templates/spec/scenario.md`, `framework/templates/spec/spec.md`, and `framework/templates/project/inbox.md`. **Not examined: five `.claude/commands/ductus/*.md` files** (`amend`, `analyze`, `groom`, `help`, `status`) — these are generated copies that `scripts/gen-claude-commands.sh` renders from the `framework/commands/` sources already read here, with `{project}` and `{cli-config-dir}` substituted; the audit's check-zero precondition fails if they drift, and it was run clean. Counting them would double every source file and inflate the ratio. **And two paths that no longer exist**: `commands/next.md` and `.claude/commands/ductus/next.md`, retired with the `/next` command — they remain in the plan's Affected Files as the record of what the implementation touched, so the scope names them and nothing could read them.

**The defect, in a file adopters receive.** `help.md` §Key Concepts stated the bug decision tree as three steps — no spec, ambiguous spec, clear spec. The constitution's canonical §bug-handling has four: the **rules tier** is step 1, added by 016. The constitution and `README.md` were swept then; this copy was not, so the shipped glance guide has been routing readers past the first and most consequential branch. Fixed in `3b5bcd0`. It was reachable only by reading `help.md` against its canonical source — the sentence carries none of the tokens an identifier sweep would grep for, which is §drift-prevention's prose-claim case exactly.

**The corrections to 006 itself** (`3c4fe76`). The spec carried three `> **Note:**` blocks annotating renames; §drift-prevention says a rename is swept, not annotated. One had gone stale in the worst available way — it read "`/ductus:groom` is now `/ductus:groom`", residue from a substitution that landed in a sentence it did not fit. Swept: `/ductus:scenario` → `/ductus:amend`, `/ductus:about` → `/ductus:help`, `triage` → `inbox` for artifact and command alike, the scenario frontmatter field `spec-ref` → `section`, and every pre-reorg path. `/ductus:next` is the one case that is **not** a rename — retired outright — so AC16 takes an annotation instead, which is the distinction §drift-prevention draws and the convention 023's own AC32 set.

**How each criterion was checked.** AC1: the scenario template carries `section` frontmatter, Context, Behavior and Edge Cases — the field is `section`, not the `spec-ref` the criterion named, so the enumeration was corrected. AC2: the inbox template is a flat checkbox list with migration rules in its comment block. AC3: the spec template's comment block names `specs/{NNN-feature-name}/scenarios/`. AC4–AC6: §bug-handling carries the decision tree, §scenarios defines the scenario lifecycle, and §spec-phase's directory diagram shows `scenarios/{slug}.md` — each read in the constitution rather than assumed. AC7–AC9: `groom.md` walks each item through the tree, routes it, and calls `remove-inbox-item` once it lands. AC10: `help.md` documents `/amend`, `/groom`, scenario conventions and the bug workflow — and checking the *content* of that last item is what surfaced the three-step tree. AC11–AC14: `amend.md`'s scenario route writes `scenarios/{slug}.md` via `create-scenario`, appends a linked task via `append-task`, halts when no session target is set, and walks the bug decision tree before drafting. AC15: `status.md`'s dashboard table carries a Scenarios column of per-spec counts. AC16: annotated — the command is gone. AC17: `analyze.md`'s scenario-consistency family checks a still-present scenario task is complete at `done`. AC18: `README.md` §"Bugs are unwritten scenarios" carries the workflow and the conventions. AC19: `markdownlint-cli2` exits 0 over all 512 files.

**On the diff base.** No commit records 006 entering `in-progress`, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead — an empty window by construction, resolving the scope to the plan's Affected Files. 006 has no scenarios and no data model, so `reviewed-digest` is `{}`: taken and empty, which reads as current.

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
