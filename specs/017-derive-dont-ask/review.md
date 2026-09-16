---
spec: 017-derive-dont-ask
diff-base: 36252b40de7ba71c197e1758aa7c8ae5d6875cbb
captured-issues: 0
skipped-passes: []
last-run: 2026-09-15T16:27:44Z
reviewed-against: 23a5fd80c997e2863cafc3caca96155f7f200eef
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 56
reviewed-digest:
  data-model.md: d2430c84cd4a126802f4a0d45b7611e0afa957e368a0bac5ec5551c0d199389b
  scenarios/detect-dependency-cycles.md: 2c426d44ce0a3a3cf5e91160fdf84ac054795737b25979152f70e89fe86005ab
  scenarios/generator-sync-claim-honesty.md: 0ee16bda3f0a5c0658100fee3d28ddae8214bc0eca9d57261811077aae314264
  scenarios/skip-prose-cross-references.md: 1b5c4bd36e2ca760437d63b948706e5b575ecd450b0779d1d10440cc77d0e086
  scenarios/tracked-specs-not-worktree.md: ba42aee5fbee0dfed38fce985200b0ba5775ecd03113a81aa792b25a18383829
blocking: false
---

# Review — 017-derive-dont-ask

## Summary

Five passes over a post-reopen scope of 56 files, of which **7 were read in full** (`examined: 7`). Rule files loaded via `discover-rule-files`: all 11 under `framework/rules/`.

**Read in full:** `specs/017-derive-dont-ask/spec.md`, `data-model.md`, `plan.md`, and both in-scope scenarios (`detect-dependency-cycles.md`, `skip-prose-cross-references.md`); `.githooks/pre-commit`, the file every claim about which generators run had to be checked against; and `framework/rules/configuration-cross.md`, the rule file AC18 delivers.

**Not read, named individually.** `specs/017-derive-dont-ask/tasks.md` — in scope and not re-read this pass; it is spent task records, ephemeral per §tasks-phase, and rewriting a completed task's rationale would record a fiction. The 14 files carrying this pass's edits in five sibling specs (`001`, `002`, `022`, `036`, `046`) were read **only at their changed regions with surrounding context**, not in full; each is reviewed under its own spec. The remaining ~34 are `plan.md` Affected Files entries from 017's original implementation — templates, command sources and bootstrap files it has not touched since — not re-read. **Eight of the 56 do not exist at all** and are named as absent rather than unread: `constitution.md`, `framework/bootstrap/hooks/install.sh`, `framework/commands/capture.md`, `framework/commands/elaborate.md`, `framework/templates/spec/spec-and-plan.md`, `scripts/gen-readme-table.sh`, `scripts/gen-spec-deps.sh`, `specs/000-016/spec.md`. They stay in scope because the plan lists them; that is the plan being a design record of what was affected then, which AC14 makes explicit, not drift to repair.

Two scenarios outside this scope — `generator-sync-claim-honesty.md` and `tracked-specs-not-worktree.md` — have their digests refreshed by this write. Both are **byte-identical** to the digest the previous review recorded (verified by hashing: `0ee16bda…` and `ba42aee5…` match), and that review read them. They were **not** re-read here, and an unchanged hash is a reason to believe them correct, not a read.

**Three defects found and fixed in this pass** (`23a5fd80`, and the reopen commit `9c766f1a`), all the same class — a spec asserting in the present tense that machinery still exists:

1. **Q7's §Resolved Questions block** named four mechanisms in the present tense that are gone: the four-generator hook, `.ductus/scripts/gen-spec-deps.sh` shipping with `update` strategy and being pinnable, and `framework/bootstrap/hooks/install.sh`. AC12, AC21 and AC23 already carried that correction from the previous review; **Q7 states the same claims one tier upstream and was missed when they were made**. Verified against `.githooks/pre-commit`, which runs `gen-configure-mcp.sh`, `gen-claude-commands.sh` and `gen-help-tables.sh` plus `derive-dependencies` and `derive-references`. Annotated rather than rewritten — the decision carried across unchanged.
2. **Q8 resolved to leave the diligence principle in `AGENTS.md`** and "re-evaluate in a follow-up spec once a third or fourth application accumulates". That re-evaluation happened: `050-constitution` promoted it to the constitution's §design-principles, classified universal in that spec's plan. This is the *previously-rejected-option-adopted* class — nothing greps for it, the option's name never changed, and the text still read as a live decision. Annotated as discharged.
3. **`data-model.md` is the canonical source the constitution's map designates for `CFG-` rule-file format**, and it still prescribed Verification as an *"instruction to the validate agent"* in both the template block and the field table. `specs/008-security-rules/data-model.md` — the source the 202-reference sweep traced to — was corrected in exactly those two places and its twin was missed. Both canonical sources and all 11 rule files now spell `/{project}:analyze`; verified by grep.

**Deliberately not changed.** `023-govern-refinement`'s AC21 asserts no current-usage reference to `/validate`, `/ductus:validate`, `/{project}:validate` or `validate.md`; "the validate agent" contains none of those four tokens, so the criterion stays literally true — the token-list limitation is already recorded rather than a defect to sweep. `plan.md`'s `/validate` references are a design record naming the command as it was called when those decisions were taken, which is the correct treatment by tense. And `skip-prose-cross-references.md`'s Edge Case describing `update`-strategy shipping is covered by that file's existing file-level supersession blockquote, the disposition the previous review chose and recorded.

**Scope window.** `diff-base` is `36252b40`; the pre-reopen leg measured **262 modified-since / 291 in scope at 54,922 bytes** before the flip and collapsed to **15 / 56 at ~4.2KB** after the reopen commit, both legs inline. The pre-reopen figure is recorded here because it is not recoverable once the base moves.

**Rule applicability, stated rather than implied.** The scope holds no source file, so the code-pattern rule sets (`quality-cross`'s three, and the backend/frontend authn, input, XSS, CSP, query, pooling, concurrency, reliability, observability and accessibility rules) had nothing in scope to fire against. They were loaded and available, not examined-and-clean.

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
