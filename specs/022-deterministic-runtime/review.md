---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-15T16:29:08Z
reviewed-against: 23a5fd80c997e2863cafc3caca96155f7f200eef
diff-base: 36252b40de7ba71c197e1758aa7c8ae5d6875cbb
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 1
scope: 48
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

**A full 022 re-review was not spent here, and this record does not claim one.** The reopen was caused by a single token substitution in one scenario — an adopter's real service alias replaced with the `acme` placeholder this repository already uses — and 022 is the corpus's largest spec: a 500-line `spec.md`, a 1,351-line `data-model.md`, and 98 scenarios. `AGENTS.md` §Workflow records the standing judgement that a full 022 re-review is its own unit of work rather than something batched into a doc fix, and that claiming a pass you did not spend is exactly the conflation `examined` exists to prevent. So the numerator here is truthful and small: **1 of 48** in scope. (The denominator moved during this pass — it measured 47 before this pass's own review artifacts landed in the window and 48 when the record was written. That is the same measure-after-the-step-4-commit-but-not-measure-once behaviour `AGENTS.md` records; the figure here is the one the record carries.)

**Read in full:** `specs/022-deterministic-runtime/scenarios/derive-references-unstaged-drift-is-reported.md`, the durable contract this pass edited and the one file whose change staled this review. The substitution sits in its §Context, where the alias is an arbitrary config value illustrating a rename (`[services.api]` became `[services.acme-api]`, same `repo` URL). The scenario's argument — that `references:` derives from the body *and* the `[services]` registry, so a registry rename drifts specs nobody edited and staged-mode enumeration can never see them — is untouched by the substitution and was re-read end to end to confirm that.

**Read in part, named rather than counted:** `specs/022-deterministic-runtime/data-model.md` at its `unwritten` region only (lines ~1030–1060), to run one targeted cross-artifact check the scenario itself asserts — that the field is recorded as present on **both** derive primitives with the `derive-dependencies`-only scoping annotation removed. It is, with the reasoning captured. That is a verified claim, not a read of the file.

**Not read:** `specs/022-deterministic-runtime/spec.md`, `plan.md`, `tasks.md`, the other 97 scenarios, and the ~40 remaining in-scope entries (`runtime/src/` and `runtime/tests/` directory entries, workflow files, command sources, and the 14 files this pass edited in five sibling specs, each reviewed under its own spec). None of them changed in the window this review covers except the sibling-spec edits, and none was opened here.

**No defect was found in what was read.** The substitution preserves the scenario's shape and every claim it makes; the one cross-artifact assertion checked holds.

**Scope window.** `diff-base` is `36252b40`. The pre-reopen leg measured **88 modified-since / 112 in scope at 37,922 bytes** — materially narrower than the 144 / 167 at 144,932 bytes `AGENTS.md` records for 022, because its base moved forward in the 2026-09-14 batch reopen — and collapsed to **14 / 47 at ~3.4KB** after the reopen commit. Both legs returned inline, so the saved-file route the entry describes was not needed on either.

**Rule applicability, stated rather than implied.** The one file read is markdown prose. The code-pattern rule sets were loaded (all 11 via `discover-rule-files`) and had nothing in the read set to fire against — available, not examined-and-clean. This record is evidence about one scenario and one data-model region, and about nothing else under 022.

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
