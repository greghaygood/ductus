---
spec: 009-scenario-targeting
reviewed-at: 2026-09-13T13:06:57Z
reviewed-against: 7339abcc7c12b74f2210031ab67cb3df309d7498
diff-base: 7339abcc7c12b74f2210031ab67cb3df309d7498
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 11
skipped-passes: []
---

# Review — 009-scenario-targeting

## Summary

First review of 009 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All eighteen criteria were verified against the tree, and **one turned out to be false** — a behaviour the framework lost in a rewrite and has not had since.

**What this review read: 6 of the 11 files in scope, and here is the other 5.** Read in full: `framework/commands/target.md`, `amend.md`, `clarify.md`, `status.md`, `implement.md`, and `framework/templates/spec/scenario.md`. **Not examined: the five `.claude/commands/ductus/*.md` copies** — generated from those same sources by `scripts/gen-claude-commands.sh` with `{project}`/`{cli-config-dir}` substituted, and held against them by the audit's check-zero precondition, which ran clean. Counting them would double every source and inflate the ratio.

**AC15 was false, and the cause is a regression rather than a rename.** The criterion requires `/target` to report that a feature has **no scenarios** when it has no `scenarios/` directory, distinctly from a slug matching no file. `target.md` had one branch for both — "list available scenarios and ask the user to choose" — so on a feature with no scenarios it lists an empty set, which reads as *your slug did not match one of these* when the truth is *there are none*. `resolve-feature` cannot separate them either: run against `001` (no directory) and `050` (seven scenarios, unmatched slug) it returns `exists: false` with an empty `section` both times. `git log -S` places the loss at `886b34b0`, 022's rewrite of the command under the parseable-step conventions — nothing removed the behaviour deliberately, so per AGENTS.md this is a **sweep to finish** and the criterion is left ticked and unedited; rewriting it to match the tree would have recorded a fiction. Restored host-side in `7339abc`, keyed on directory existence, so no runtime change and no version bump is owed. It is worth naming what this was an instance of: §design-principles' *a check that cannot run must never be indistinguishable from one that passed*, committed by the framework's own command.

**The other corrections** (`d1cf740`). The body's rename annotation goes — §drift-prevention sweeps a rename rather than annotating it. Swept: the `question` and `scenario` commands, both merged into `/{project}:amend` by 023; `validate` → `analyze`, which AC12 named; every `commands/` and `templates/` path to its framework/ home; and `spec-ref` → `section` (017), which also appeared in the target-display enumeration AC4 covers. Two illustrative blocks showed shapes the framework stopped writing: the session-file examples used camelCase keys (`scenarioPath`, `setAt`) where the 0.10.0 consolidation moved the file to TOML with kebab-case keys — so AC1's field name was wrong — and the scenario-template block showed a `**spec-ref:**` body line where the template has carried `section:` frontmatter since 017. AC18 is the one claim superseded rather than renamed: 012 replaced the per-agent `ductus/` bootstrap variants with a single registry-driven file, so there are no variants left to hold in parity; annotated with where the surviving obligation lives (Family 21).

**How the rest were checked.** AC1–AC4, AC13–AC16: `target.md` steps 1, 3, 7, 8 and 9 — the no-argument display, the `{feature}/{scenario-slug}` split, the `not-found` report, the session write, and the resolved-target display. AC5: the scenario template carries both question sections after Edge Cases. AC6, AC7, AC10: `amend.md`'s Target File Detection routes a scenario target's input to the scenario's own `## Open Questions`, its question route appends to the spec otherwise, and its scenario route calls `write-session` with the new slug. AC8, AC9: `clarify.md` runs steps 1, 6 and 13 on a scenario-targeted session and the full feature walk otherwise. AC11: `status.md` renders the `Scenario: {scenario} ({section})` preamble line. AC12: `specify.md` and `plan.md` read no scenario field, and `analyze.md` targets the feature — they ignore it as claimed. AC17: parity is the generator plus check-zero, run clean here.

**On the diff base.** No commit records 009 entering `in-progress`, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead. 009 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

**Checked and clean elsewhere.** `README.md`, `CLAUDE.md` and `docs/` were searched for mirrors of 009's claims — the session-file shape and scenario targeting — and carry none.

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
