---
spec: 041-task-pruning
last-run: 2026-10-01T18:58:23Z
analyzed-against: 10e760b9b2ea4f8a7ac0500dbaed76d4a23c031a
hard-fail: 0
blocking-findings: 0
advisory: 12
unexamined: 1
analyzed-digest:
  data-model.md: d1e04eeaa9f74921b26f7c3b5101c70832e7469dfc590f15e787d24cea604535
  plan.md: a2e0fd098231551f3bb9428ec664142753a400f463df9121f9dc1b699ec01246
  review.md: f30c42c1d93e5b307f9d0455520da1f80885c4e4cc591439b0c3478f45a198b9
  scenarios/plan-records-the-design-as-it-stands.md: 617feb2d97e676600074e386b656b15f9d9c632500aaf418d554e2d91364a4eb
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 2f1a33caf5c5c72e94c6e13c6652e0e8465162ffe2d1e9c158a0ce18e60b8c30
  spec.md: 99c6a6755d9ec7d074208f6c6d3fa21c70f3a14964312a0bdf53d17e8d3eb019
  tasks.md: a6d5b0961f3604f82a7590ba8de7d5e7dc4baf13f2db170beb39cbbd5abac512
unexamined-by-reason:
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 8
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

0 hard-fail, 0 blocking, 12 advisory; not blocking. 1 unexamined target(s). Dispositions: 8 fixed, 0 routed, 12 discarded, 0 undispositioned.

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

- rule-assessment — BE-INPUT-004: the feature argument is joined into file paths with no traversal check committed — `specs/041-task-pruning/plan.md` — **fixed**
- rule-assessment — BE-INPUT-008: spec.md frontmatter and analysis.md decisions are read as YAML with no safe-loading mode named — `specs/041-task-pruning/plan.md` — **fixed**
- rule-assessment — BE-PAYLOAD-001: the --all walk returns a corpus-sized list with no committed size bound — `specs/041-task-pruning/plan.md` — **fixed**
- grounding — plan.md says MCP requires a tool's output schema to be an object, uncited — `specs/041-task-pruning/plan.md` — **fixed**
- grounding — plan.md says the exec walker halts on a primitive error, uncited — `specs/041-task-pruning/plan.md` — **fixed**
- grounding — plan.md cites mark-task's locate_task_range and the list_feature_dirs walk with no path — `specs/041-task-pruning/plan.md` — **fixed**
- grounding — spec.md says /implement directs an empty task list back to /plan, which no implement step does — `specs/041-task-pruning/spec.md` — **fixed**
- grounding — spec.md says groom operates on a repo-level inbox.md, uncited — `specs/041-task-pruning/spec.md` — **fixed**
