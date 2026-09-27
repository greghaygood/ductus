---
spec: 018-adopter-owned-pre-commit
last-run: 2026-09-27T14:14:09Z
reviewed-against: 3534aa173293f50899a3f44c2331a9367cb573e9
diff-base: 23f664154577f13b3bac280c65ff38a4a3b5fbf2
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 9
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 1
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "convention: 017's post-018 signpost and AC21 point at `framework/bootstrap/ductus.md` §Hook Installation, which 056 moved to `ductus-procedure.md` — `specs/017-derive-dont-ask/spec.md`"
    outcome: routed
    target: specs/059-project-in-a-repository-subdirectory/tasks.md
    decided-at: 2026-09-27T14:14:09Z
    decided-by: andy@stone.dev
---

# Review — 018-adopter-owned-pre-commit

## Summary

Re-review of 018 after its reopen for spec 059's cross-spec correction (`done -> in-progress` at `b1617519`). No MUST violation and, after one fix made in this run, no SHOULD violation.

**Diff base and scope.** The derived base is `23f66415`, the parent of the reopen commit, so the window holds this reopen's own commits and nothing older. The plan's Affected Files named `framework/bootstrap/ductus.md` for §Hook Installation, which spec 056 moved to `framework/bootstrap/ductus-procedure.md`; `a8653715` added the procedure file to the table before the passes ran, so the scope covers the file AC5, AC6, AC7 and AC12 name.

**What this review read: 8 of the 9 files in scope.** Read in full: `framework/bootstrap/ductus-procedure.md`, `framework/bootstrap/ductus.md` (all 814 lines), `framework/bootstrap/hooks/ductus-pre-commit`, `framework/bootstrap/hooks/pre-commit`, `specs/017-derive-dont-ask/spec.md`, and this spec's `spec.md`, `plan.md` and `tasks.md`. **Not examined:** `framework/bootstrap/hooks/install.sh`, which does not exist — AC12 deleted it, and the plan still lists it as the file it deleted. All eleven rule files under `framework/rules/` were read in full. The scope is bootstrap prose and two bash hooks, so no frontend rule and none of the backend design-time rules (api, concurrency, observability, performance, reliability) has a trigger here; the rules that bear on it are `quality-cross.md` and the shell and transport rules in `security-backend.md` (argument-vector invocation, no `curl -k`, digest verification), which the hooks and the acquisition steps satisfy.

**One SHOULD found and fixed in the run — `QUAL-CLAIM-001`, `framework/bootstrap/ductus-procedure.md` §Hook Installation.** Since 018 dropped spec 017's "existing `.githooks/pre-commit` not from `/ductus`" conflict case, the ladder reported `pre-commit hook installed` or `already wired up` over an outer hook of the project's own that never invokes `ductus-pre-commit` — the manifest's `create` pass leaves such a file in place — so ductus's passes never ran while the run reported success, and case 5 activated a hook the project had not activated. 018's own §Migration text said such a file gets "skip wiring, manual integration snippet"; the ladder did not implement it. Fixed as 018 task 12 in `c686a81f` and `3534aa17`: a precondition on cases 1, 2 and 5 skips wiring with the snippet when no non-comment line of `.githooks/pre-commit` names `ductus-pre-commit`, stated in new AC14. The quality pass re-ran over the changed sections and the precondition was walked against real hook contents from history — the current stub and the pre-059 stub proceed, a spec-017-era file and a project's own hook are skipped — with the `ductus-rename` migration re-pointing an older `govern-pre-commit` invocation before the ladder runs.

**Observations.** Two stale claims in 018's body were fixed in the same commits. 017's signpost and AC21 still point at `ductus.md` §Hook Installation; that correction is already 059's task 17, so it is routed there rather than made twice.

**Criteria.** AC14 is new and unticked; `/ductus:implement`'s completion gate verifies it and every ticked criterion against the tree, including the 059 narrowing annotations on AC2, AC5, AC6, AC9 and AC12. AC8, AC9 and AC11 are end-to-end claims about sandbox `/ductus` runs that this review did not re-run; it verified the procedure they rest on.

018 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: 018's body named a `framework/bootstrap/hooks/pre-commit-stub` (or similar) that never shipped, and said `framework/bootstrap/hooks/pre-commit` ships the inner body today — both false since 018 made that path the outer stub — `specs/018-adopter-owned-pre-commit/spec.md` — **fixed**
- convention: 017's post-018 signpost and AC21 point at `framework/bootstrap/ductus.md` §Hook Installation, which 056 moved to `ductus-procedure.md` — `specs/017-derive-dont-ask/spec.md` — **routed** to `specs/059-project-in-a-repository-subdirectory/tasks.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
