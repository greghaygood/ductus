---
spec: 010-agent-autonomy
reviewed-at: 2026-09-13T16:22:03Z
reviewed-against: 76e5aeb5dada16ade73307ce40bbd8d3f65163f7
diff-base: c39f376f4347ff71424796fea18d23f94c03cf4a
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 21
scope: 25
skipped-passes: []
---

# Review — 010-agent-autonomy

## Summary

All five passes ran over the scope resolved from `c39f376f..HEAD` (the post-reopen natural base). **Examined 21 of 25 in scope.** The four unexamined are unexamined because they no longer exist on disk, and they stay in scope because `plan.md`'s Affected Files still lists them: `framework/skills/`, `framework/skills/registry.json`, `framework/skills/templates/*.md` (renamed to `framework/workflows/` by this spec's own AC14, then deleted outright by `043-workflows-sunset`) and `specs/005-skills-and-plugins/` (renamed to `specs/005-workflows/` by AC14; the successor directory's four artifacts are separately in scope and were all read in full). Nothing was skipped for convenience — every path that resolves was read.

Three of the 21 are generated command copies (`.claude/commands/ductus/{implement,plan,configure}.md`). Their sources — `framework/commands/implement.md`, `framework/commands/plan.md`, `framework/bootstrap/configure/claude.md` — were read in full, and each copy was then verified byte-identical to its source under the two documented substitutions (`{project}` → `ductus`, `{cli-config-dir}` → `.claude`); `scripts/gen-claude-commands.sh` re-run this session reported all 16 commands in sync. The fourth `.claude` file, `init.md`, is hand-maintained with no source and was read directly.

**Base selection.** Three measurements were taken before choosing. The pre-reopen natural base (`b693482c`) resolved **491** files; `--since HEAD` resolved **21**; the post-reopen natural base resolved **25**. The post-reopen base is recorded: it is tighter than the window that spans months of unrelated history, and unlike `HEAD` it *includes* the five files this pass edited rather than excluding them, so the denominator covers the work being reviewed.

**Scope composition.** The scope is entirely markdown — prose, prompt sources, templates and specs. No `runtime/*.rs` is in it, so the quality pass ran against artifacts as artifacts rather than as code. The security pass had two real subjects and both hold: `framework/bootstrap/configure/claude.md`'s canonical allow set places no wildcard before a git subcommand (the shape that once approved arbitrary execution) while the deny set correctly keeps its leading wildcards, and `framework/bootstrap/ductus.md`'s runtime acquisition verifies the sha256 sidecar before installing anything, treats a missing sidecar as failure rather than a skip, applies path-traversal protection per archive entry, and halts rather than falling through to "latest". `CFG-CONST-003` was assessed against the stuck-detection threshold of 3 and does not fire: the spec and plan both record it as deliberately not operator-tunable, and the rule's subject is operator-tunable values in business logic.

**Five defects were found by this pass and fixed in `76e5aeb5` before this review ran — none is carried.** They are recorded here because the counts below describe the tree as it now stands, and a bare `0/0/0` would not say what was found.

1. **AC11's enumeration was false.** It names four cost levers; `framework/constitution.md` §Cost levers names two. 010 delivered all four in `fc68cf1c`; `017-derive-dont-ask` removed the `[simple]` lever in `fc737946` and `023-govern-refinement` removed the lightweight track in `dd8f7b41`. Annotated superseded-in-part — the same disposition AC7, AC8 and AC14 already carry — since 010 delivered the enumeration as specified and neither later spec declares 010.
2. **The shipped constitution claimed `ductus` has no runtime.** §Cost levers read *"require a runtime `ductus` does not have"* — true when 010 wrote it, false since spec 022, and contradicted by §runtime-boundary in the same document, in a file every adopter receives. Fixed at source: the decision it supports is unchanged and better founded, because principle 2 forbids that runtime from calling an LLM, so it can never observe a token count.
3. **AC9, §Stuck detection and Resolved Question 3 all described a mechanism the plan declined.** Each said the step reads `git log` for *"affected paths"*. `plan.md` §Trade-offs settled the opposite at plan time — *"Stuck detection reads `tasks.md` commits, not affected-files commits"* — and `tasks.md` task 3, `framework/commands/implement.md` §Stuck-detection details and `check_stuck.rs` all read `tasks.md`. Corrected rather than annotated: nothing superseded this, the anticipated mechanism was never built.
4. **The scenario contradicted the command source.** `implement-offers-the-next-step.md`'s `ductus exec` edge case claimed the next-step offer "takes the same extension round trip the walk's other confirmations use". `framework/commands/implement.md` step 8 carries no `llm:` extension marker — only steps 5 and 11 do — and documents the opposite: a linear walk that renders the offer and continues on a fresh invocation. The scenario bullet was written at `07db968b`, the implementation at `5eb31327`; the reconciliation never happened. A scenario is a durable contract, so it was corrected to match.
5. **The plan's rename trade-off argued from a reversed decision.** "Cross-spec rename has broad blast radius" still justified itself with "the spec directory name stays" — reversed mid-implement two sections above, where a blast-radius check found only seven referring files, and AC14 records `specs/005-workflows/` as delivered.

**Swept in the same commit, no annotation, requirement unchanged.** AC5 named `commands/` and AC6 named `ductus/` — the first a directory reorganised into `framework/commands/`, the second sweep residue where 049's token pass rewrote `govern/` (the bootstrap installer variants, now `framework/bootstrap/`) onto a path that never existed. Both criteria state requirements that still hold, so per §drift-prevention they are swept rather than annotated. The same applies to 32 occurrences of the pre-rename product name used as a common noun across `spec.md`, `plan.md` and `tasks.md`, and to a dead `§lightweight-track` anchor in `tasks.md` that `resolve-anchor` now reports clean. The diff was checked for `X → X` substitution damage and for removed `> **Note:**` blockquotes; neither is present, and AC14's `framework/skills/` → `framework/workflows/` rename mapping was deliberately left carrying the retired name on both sides.

**Verified criteria.** All fourteen were read against the tree, enumerations included. AC1–AC4 hold (six capabilities, six verdicts, declines carrying rationale, no platform requirement introduced by any of them). AC5, AC6 hold after the sweep. AC7 and AC8 confirmed still superseded — `[simple]` appears nowhere under `framework/`, and `framework/commands/plan.md` read in full carries no proposal step. AC9 holds as corrected. AC10's five named gates all still fire in `framework/commands/implement.md` §Flags. AC11 holds as annotated. AC12 verified in `framework/templates/project/agents.md:80` — the `## Skills` section is present, optional, empty by default, with the activation-condition example. AC13 verified in `framework/constitution.md` §concurrent-features. AC14 verified from both sides: `specs/005-workflows/` exists, 005's AC16 carries the reciprocal signpost back to 010, 005's task 6 records the rename complete, and `specs/013-text-first-artifacts/plan.md` carries the one-row path update.

**Decisions re-read as live claims**, per §drift-prevention's decision-resolution rule. The plan's six trade-offs and five non-goals were each checked against current behavior: the fixed threshold of 3 still holds (`check-stuck` takes it as a required argument with no default; `implement.md` supplies 3 and documents it as not configurable), commit-but-do-not-push still holds (`implement.md` §Flags), no slash command takes a `--feature` flag, no execution log exists, no `[complex]` tier exists, and the `AGENTS.md` template carries no worktree note — correct, since the plan placed it in the constitution deliberately. The one that had gone false is defect 5 above.

The prior record for this spec, written 2026-08-17, carried no `examined`, no `scope` and no `reviewed-digest` — the pre-`0.49.0` shape, in which a review that read none of its scope and one that read all of it are byte-identical. This run records all three.

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
