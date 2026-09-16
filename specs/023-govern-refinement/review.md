---
spec: 023-govern-refinement
diff-base: 3792cb908d99c412d7ae0a76080e108a66266c50
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T21:26:57Z
reviewed-against: fc8205efbff00731d8ce4eb5713be9c16e2b3292
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 42
reviewed-digest:
  scenarios/configure-dedup-permissions.md: 61f154523cf65426a4b56fdc38662e8d7f822bace8418ae30952322d7e627036
  scenarios/configure-inert-write-path-entries.md: fb2b3383713428e9006dff42edbb89a6bab99995f612161eefee69412c757f9d
  scenarios/configure-permission-pattern-safety.md: 6c6f71741c7fdfcd96b9f6017b649eb6396dd0edc6b90944372f3078d12b18f0
  scenarios/configure-retires-formerly-canonical-entries.md: 0abbef1ab46fd11f9b428fb0436fb9f87da6dc10c8d3759ad8701b909d3b0311
  scenarios/extend-existing-scenario-task.md: 61622769bcd38f91e2e6cf8aab86dab3f408df20beabdc747cbde2dbba7c36bf
  scenarios/living-specs.md: 3c88dc74f397eefbe50213c92698c6a1093747158f651f6f7a8b215a2b556c69
blocking: false
---

# Review — 023-govern-refinement

## Summary

*Why this ran at all.* 023's record was a **partial** pre-`ductus-v0.49.0` shape rather than a bare one: `examined` was present while `scope` and `reviewed-digest` were absent, so the numerator stood with no denominator and freshness was undeterminable. `analyzed-digest` was already present. This is the corpus backfill campaign's unit for 023.

*Scope, and both bases.* Pre-reopen the natural base `043a0345` resolved **155** modified-since / **180** in scope at **98,918 bytes** — measured before step 4 and recorded on the campaign item, because the reopen moves the base to the reopen commit's parent and the window is unrecoverable afterwards. Note that figure against the **73,353** the same item recorded for 023 earlier the same day: it grew **25KB inside one session**, the same quoted-figure decay 051 showed at 15KB, and `043a0345` is the base 008 also resolved against, so every commit to `main` widens it until 023 reopens. The step-4 commit collapsed it to **3 modified-since / 42 in scope at 2,921 bytes** on base `3792cb90`, which returned inline. `--since HEAD` gave **0 / 39** and was declined for excluding, by construction, the three files this pass edited.

*Five in-scope paths do not resolve, and four of them are this spec's own delivery.* `framework/commands/capture.md`, `framework/commands/elaborate.md`, `framework/commands/validate.md` and `framework/templates/spec/spec-and-plan.md` are absent because AC1, AC4, AC5 and AC20 **deleted** them — their absence is the criterion holding, not a broken path. `docs/introduction.md` was removed on 2026-08-30, which AC28 already records in place. All five stay in scope because the plan lists them.

*What was examined: 4 of 42.* Read end to end: `specs/023-govern-refinement/spec.md`, `specs/023-govern-refinement/plan.md`, and `framework/constitution.md`. Read as implementation and module documentation in full, with its `#[cfg(test)]` module not read: `runtime/src/primitives/create_scenario.rs`.

*What was not examined, named individually.* `runtime/src/primitives/append_task.rs` was read only at `run()`'s argument-validation front matter, where its input rules live, and not through its 1,100-line test module. `framework/commands/amend.md`, `framework/bootstrap/configure/claude.md`, `framework/commands/help.md`, `status.md`, `target.md`, `analyze.md` and `review.md` were each read only at the sections carrying a criterion — the classifier and reconcile pass, the canonical allow set and its retired list, the five category tables, the two Status → next action tables, and the two `description:` frontmatter lines — not in full. `framework/bootstrap/ductus.md`, `README.md`, `AGENTS.md`, `specs/README.md`, `framework/templates/project/agents.md`, `project-readme.md`, `framework/commands/{clarify,groom,implement,plan,specify}.md`, `framework/runtime-tools.txt`, `scripts/gen-configure-mcp.sh`, `gen-help-tables.sh`, `lint-frontmatter.sh`, `.githooks/pre-commit`, `runtime/CHANGELOG.md`, `runtime/Cargo.toml`, `runtime/src/primitives/mod.rs`, `specs/022-deterministic-runtime/spec.md` and `scenarios/ask-consolidation.md`, and `specs/023-govern-refinement/tasks.md` were **not opened**; where a criterion made a claim about one, the claim was settled by a targeted grep or a count against the tree, which is weaker than a read and is why none is counted. Two scope entries are **directories** rather than files — `runtime/src/mcp/` and `runtime/tests/` — and neither was walked. The five 023 scenarios other than the one this pass edited were read: `living-specs.md` and `configure-permission-pattern-safety.md` in full, the other three in part.

*The `skipped` array here is the inverse of 051's, and that is the lesson.* `check-artifacts` reported `clean: true` with **24** skips — the richest in the remaining set — where 051 reported an empty array. **59 of the 182** backticked spans across the 40 criteria are visible to `criterion-path-existence`, and 17 of those resolve to nothing. All 182 were walked by hand. Most of the 17 are correct: four are deletions this spec delivered, two are globs, and three are `Write(...)` permission entries rather than paths. **Four were the repairable shape** — spec-relative `scenarios/{slug}.md` citations inside 023's own criteria, which the family resolves against the **repo root** and therefore records as `root-absent` skips. Spelled `specs/023-govern-refinement/scenarios/…` they resolve, so AC33, AC34, AC36 and AC40 are now checked rather than skipped. That is 040's repair applied a second time, and *0 of N visible* versus *59 of N visible* turn out to be two different situations wearing the same `clean: true`.

*Five defects, all corrected.* **AC12** asserts `Edit`/`Write({cli-config-dir}/{project}-session.json)`; both halves are superseded and one of them by this spec — 022 task 40 moved the path to `.ductus/session.toml`, and **AC34 below removed the `Write(path)` entries**, because Claude Code matches file permissions against `Edit` rules only. **AC24** pins a description *exactly*, and 022's `7cfc42e3` rewrote its tail: `Read-only.` became a clause naming the two things `Read-only.` had stopped being true about — the command now writes an `analyze:` record, and `--fix` reverts a drifted `done` spec. **AC30's `status.md` half was never true**, including on the day it shipped: `git show 670a3181:framework/commands/status.md` — this spec's own rename commit — already reads `done (spec is complete)`, because 022 task 12 had rewritten that table first. The divergence is correct and was left alone rather than swept, because `/{project}:status` renders a view while `/{project}:target` recommends an action. **The Help-category Resolved Question recorded its *final* set naming `/validate`** — the command this spec renames four sections above — and the set has since grown in two more categories; the decision it settled survives, the single-row rationale beneath it does not. **AC21** now records that the prose form was swept: its four token forms are still absent and were never the problem, while 202 occurrences of the bare English verb were found and swept in 008's pass on 2026-09-13.

*A rejected option whose premise was overtaken.* `plan.md` §Trade-offs deferred a `spec-and-plan.md` residue check to "an `/audit` command (deferred per the inbox)". That command shipped with spec 026, and `scripts/audit/introducing-drift.sh` is the check described, carrying four of this spec's renamed tokens. `spec-and-plan.md` is deliberately absent from it for a **stronger** reason than the out-of-scope one recorded — it is a retired *filename* the project still reads, so a token catalog would fire on the pre-commit hooks' `(spec|spec-and-plan)\.md` alternation, on `scripts/lint-frontmatter.sh`, on `is_spec_path`, and on the `spec-and-plan-sunset` migration entry that records the rename. The deferral is now a settled decision with its reason.

*023's §-reference share is discharged, with two deliberate exceptions.* `§three-cycles` is a constitution **subsection** and carries no `<!-- § -->` marker, so `plan.md` now cites the marked section containing it; the scenario's `§Pre-run Migrations` now names `framework/bootstrap/ductus.md` on the reference's own line, which is what makes a cross-document reference qualify. `resolve-anchor` reports `unresolved: []` across `spec.md`, `plan.md` and all six scenarios. The **two in `tasks.md` are left as they are**, and that is a decision rather than an omission: they sit in spent task entries, which §tasks-phase makes ephemeral and `AGENTS.md` places off-limits, and `§lightweight-track` names the section its own task deleted — rewriting it to resolve would destroy the record of the deletion.

*Verified rather than assumed.* AC14–AC16's "every tool in `runtime-tools.txt`" was checked by set comparison rather than by the count that first looked wrong: the file has 75 non-blank lines but **55 tool names** once its 20 comment lines are excluded, and `claude.md` carries exactly 55 `mcp__ductus__` entries with an empty diff. Reading the entry before filing is what kept that from being a finding. Also confirmed against the tree: the constitution carries zero `lightweight-track`, `spec-and-plan`, `/capture` or `/elaborate` occurrences (AC2, AC3); no command source retains the detection fallback (AC10); `specify.md` prompts no qualifying questions (AC6); `amend.md` carries the classifier, the `flip` override, both back-edges and the reconcile pass with its slug-match rule (AC7–AC9, AC33); `gen-help-tables.sh` builds from `analyze.md` (AC22); `merge_permissions.rs` implements `revoke` with `ConflictingRevoke` (AC37); and both permission-shape families exist (AC35, AC39).

*Findings.* None. Zero MUST, zero SHOULD, zero low-confidence across the five passes over the four files read. `create-scenario` and `append-task` are notably hardened rather than merely correct — traversal and slug validation before any filesystem touch, embedded-newline refusal with the argument named, and a `MissingArgument` refusal where an earlier version silently doubled the slug. No observation is recorded: nothing found here lies outside the spec that was open, and the inbox stands at 12 with the standing rule that a pass adding an item without retiring one is the default that has to be argued for.

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
