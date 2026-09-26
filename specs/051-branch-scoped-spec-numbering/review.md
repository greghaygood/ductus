---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-09-26T19:34:55Z
reviewed-against: 3bd4841aed864c0f3572c3b283c48a471f1e7f67
diff-base: a523457ca91e1a5ea17b9c98a81634fc90f35de6
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 22
scope: 92
skipped-passes: []
reviewed-digest:
  data-model.md: 416e488447e1fb195e85babd92882b3d5eefa1cb5e3e2876809a59f487420818
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: a59ee126c7a769f8625c9030f42ded19c3d9486c9cce99e10386ad6cdcf324d6
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
blocking: false
dispositions:
  fixed: 4
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "convention: 050's scenario a-declared-cross-spec-impact-gates-done says the gate holds its category for any undischarged fold; since task 26 it holds only a fold this tree can perform — `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`"
    outcome: discarded
    reason: the sentence's point, that an undischarged obligation is not a candidate for done, still holds, and the constitution it restates carries the narrowed wording; not worth a 050 reopen and full re-review
    decided-at: 2026-09-26T19:34:55Z
    decided-by: andy@stone.dev
---

# Review — 051-branch-scoped-spec-numbering

## Summary

051's review over its window since it re-entered `in-progress` (default `diff-base` a523457c): task 26 and its scenario, a fold owed to another tree no longer holding `done`. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. 5 observations: 4 fixed, 1 discarded, none undispositioned. This record replaces one written before 058, which carried `captured-issues` and no `dispositions:` map.

**Task 26 holds.** `pending_fold_block` blocks only when `fold_target_resolves` finds the `folds-into` target in this tree. That is fold-back's own test: the name screened by `parse_feature_dir`, and a directory holding `spec.md`. An absent target passes and the later checks decide. `check-unfolded-specs` reports a `done` staging spec with its status, `retire-feature` folds one, and `/{project}:status`'s "not in this tree" agrees with the gate wherever it renders, since `dashboard` refuses a feature directory with no `spec.md`. Three single-point mutations of `fold_target_resolves` in the dev profile, restored byte-identical afterwards, each failed at least one of the five gate tests: the target always resolving, never resolving, and a bare directory counting. The `retire-feature` and `check-unfolded-specs` tests were not mutated: each pins that its primitive does not check status, so there is no code to remove. The cross-spec-impact gate's `target-missing` block is unchanged, as the scenario's Resolved Question argues. The rule is restated in the constitution (the `done` row, the branch-scoped paragraph, §cross-spec-impact, §numbering), 051's AC35, body and two Resolved Questions, `implement.md`, `fold.md`, `status.md`, the spec template, the user docs, the two MCP descriptions, the gate, dashboard and schema docs, and 022's data model and two scenarios. `implement-basic`'s golden moved by its fixture shas only.

**Fixed, by operator decision.** Four statements task 26 left stale or wrong:

- The new scenario named `check-unfolded-specs`' test as the gate's. That primitive resolves no target; the gate applies fold-back's.
- `framework/commands/specify.md` still said a branch-scoped directory never reaches `done`. It now says it is discharged by fold-back, whatever its status. The command tests (`parity`, `specify_command`) pass, and the regenerated mirror changed only `specify.md`.
- 051's plan recorded that `check-review-gate` never checks whether the fold target resolves. The decision is annotated as narrowed, as the plan's line on the gate already was.
- The plan repeated one sentence about `retire-feature` twice. This predates task 26.

**Discarded, by operator decision.** 050's scenario `a-declared-cross-spec-impact-gates-done` says the gate holds its category for any undischarged fold. Since task 26 it holds only a fold this tree can perform, but the sentence's point still holds and the constitution carries the narrowed wording.

**Scope.** 92 in scope (84 modified since the base, 8 plan-only); examined **22**: every hunk of `fde2e38f`, `5cf68693` and `e9a22e18` in the 22 files they touched outside `.claude/`, which includes 022's data model and two scenarios and 058's `tasks.md` header. **Not read:** the four `.claude` mirrors `5cf68693` regenerated, which match their sources after regeneration; the hunks of 058's and 022's commits in the window, which 058's fourth and fifth reviews and 022's review read; and the 8 plan-affected entries unchanged since the base, one a glob string.

**Passes.** Security: the fold target is screened by `parse_feature_dir` before it is joined to a path. Reuse: the gate calls the shared feature-directory predicate. Quality produced the five observations. Efficiency and simplicity found nothing.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: a-fold-owed-to-another-tree-does-not-hold-done said the gate resolves the target by check-unfolded-specs' test, which resolves no target; the gate applies fold-back's — `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md` — **fixed**
- convention: specify.md said a branch-scoped directory never reaches done, false since task 26 — `framework/commands/specify.md` — **fixed**
- convention: 051's plan recorded that check-review-gate never checks whether the fold target resolves, false since task 26 — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- convention: 051's plan repeated one sentence about retire-feature twice (predates task 26) — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- convention: 050's scenario a-declared-cross-spec-impact-gates-done says the gate holds its category for any undischarged fold; since task 26 it holds only a fold this tree can perform — `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md` — **discarded**: the sentence's point, that an undischarged obligation is not a candidate for done, still holds, and the constitution it restates carries the narrowed wording; not worth a 050 reopen and full re-review

## Skipped passes

*None.*

## Unexamined governance

*None.*
