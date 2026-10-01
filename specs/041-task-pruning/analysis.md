---
spec: 041-task-pruning
last-run: 2026-10-01T22:08:36Z
analyzed-against: 7f1688a2cbbe12b68d8e541b6f5fe3f029c6e214
hard-fail: 0
blocking-findings: 0
advisory: 12
unexamined: 1
analyzed-digest:
  data-model.md: d1e04eeaa9f74921b26f7c3b5101c70832e7469dfc590f15e787d24cea604535
  plan.md: 57401c526f70fe60dfe826c5ab139bf1eef02ac1cc0c14dd2a1a2b5e386b4853
  review.md: 3762ffcbbce67e89077c57651713fc7844455b4d42583de5a15f80a4a4ffa42f
  scenarios/plan-records-the-design-as-it-stands.md: a50174ff23b21bc5475d52cb6f289f06085066fae8c7151fbf1be8eaa727221b
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 9f59c1ae42c42cc57e4f7a86379aefed593080ddbe40a0bbeea9e11c59bbb604
  spec.md: 24f644fd65c2b77ca97dcddc57dc0798714dd6e0e4555d0fcb863b83c2825080
  tasks.md: 827503ba4343c00b59a983a16585d2092d0bcffd70d949458ade8b3da9a0533e
unexamined-by-reason:
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 12
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-METRIC-001: the prune-plan MCP handler commits to no rate, error or duration metrics"
    outcome: discarded
    reason: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command
    outcome: discarded
    reason: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks
    outcome: discarded
    reason: motivation describing how task lists grow, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says ductus has no command to reclaim that space
    outcome: discarded
    reason: motivation stating the gap this spec filled; /prune now exists by this spec
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness
    outcome: discarded
    reason: the reason for an alignment this spec delivered, held by AC13 and its tests
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says an adopter tree holds only its own copy of the tasks template
    outcome: discarded
    reason: the bounded divergence AC11 records; prune.md's markdown-only reference states it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: "grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command"
    outcome: discarded
    reason: a measurement this run re-derived (2026-10-01) naming the files it counts
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says a done spec's artifacts are committed
    outcome: discarded
    reason: the premise of the trigger's absent-at-HEAD rule, a property of the pipeline's completion gate
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says the --all batch shape is the one /analyze --all already has
    outcome: discarded
    reason: a comparison to a command in this repository, named by its invocation
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says a filled tasks.md has already stripped the guidance comment
    outcome: discarded
    reason: rationale for a rejected alternative in Trade-offs
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines
    outcome: discarded
    reason: the rationale of a decision, dated by this run to when it was made
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says every other tasks primitive trusts checkbox state
    outcome: discarded
    reason: a known limitation stated in Trade-offs, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
---

# Analysis — 041-task-pruning

## Summary

0 hard-fail, 0 blocking, 12 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 routed, 12 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-METRIC-001: the prune-plan MCP handler commits to no rate, error or duration metrics — `specs/041-task-pruning/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
- grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command — `specs/041-task-pruning/plan.md` — **discarded**: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
- grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks — `specs/041-task-pruning/spec.md` — **discarded**: motivation describing how task lists grow, not a claim the design rests on
- grounding — spec.md says ductus has no command to reclaim that space — `specs/041-task-pruning/spec.md` — **discarded**: motivation stating the gap this spec filled; /prune now exists by this spec
- grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness — `specs/041-task-pruning/spec.md` — **discarded**: the reason for an alignment this spec delivered, held by AC13 and its tests
- grounding — spec.md says an adopter tree holds only its own copy of the tasks template — `specs/041-task-pruning/spec.md` — **discarded**: the bounded divergence AC11 records; prune.md's markdown-only reference states it
- grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command — `specs/041-task-pruning/plan.md` — **discarded**: a measurement this run re-derived (2026-10-01) naming the files it counts
- grounding — plan.md says a done spec's artifacts are committed — `specs/041-task-pruning/plan.md` — **discarded**: the premise of the trigger's absent-at-HEAD rule, a property of the pipeline's completion gate
- grounding — plan.md says the --all batch shape is the one /analyze --all already has — `specs/041-task-pruning/plan.md` — **discarded**: a comparison to a command in this repository, named by its invocation
- grounding — plan.md says a filled tasks.md has already stripped the guidance comment — `specs/041-task-pruning/plan.md` — **discarded**: rationale for a rejected alternative in Trade-offs
- grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines — `specs/041-task-pruning/plan.md` — **discarded**: the rationale of a decision, dated by this run to when it was made
- grounding — plan.md says every other tasks primitive trusts checkbox state — `specs/041-task-pruning/plan.md` — **discarded**: a known limitation stated in Trade-offs, not a claim the design rests on

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
