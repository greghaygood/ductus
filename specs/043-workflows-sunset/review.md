---
spec: 043-workflows-sunset
reviewed-at: 2026-09-13T13:25:13Z
reviewed-against: aca5720738988e6f23e97b44fcda541befc9f6eb
diff-base: aca5720738988e6f23e97b44fcda541befc9f6eb
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 7
scope: 22
skipped-passes: []
---

# Review — 043-workflows-sunset

## Summary

First review of 043 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. **All eleven criteria verified against the tree and every one holds** — no correction, so 043 never left `done`.

**What this review read: 7 of the 22 files in scope, and here is the other 15.** Read in full: `framework/constitution.md`, `framework/bootstrap/ductus.md`, `framework/commands/groom.md`, `framework/migrations.toml`, `framework/migrations/workflows-sunset.md`, `README.md`, and `AGENTS.md`. **Not examined:** `CHANGELOG.md` (searched for §Archived migrations, not read whole — it is a 2000+ line release log), `framework/commands/link.md`, `framework/templates/project/agents.md`, three `runtime/` files (`Cargo.toml`, `enforce_manifest.rs`, `schema/paths.rs`), two generated `.claude/commands/ductus/{groom,init}.md` copies, and four sibling specs (`004`, `005`, `010`, `019`) read only where 043's criteria reach into them. **And three paths that do not exist** — `framework/workflows/`, `framework/migrations/skills-to-workflows.md` and `workflow-filename-rename.md` — which is the point: AC1 and AC7 assert their absence, so the scope naming them is the plan recording what the implementation removed.

**Absence claims were checked as absence, not inferred.** AC1 asserts `framework/workflows/` does not exist *and* that no live artifact references it or its surfaces. The directory is gone, and a grep across `framework/`, `scripts/`, `runtime/src`, `.github/`, `docs/`, `README.md` and `AGENTS.md` returns only two classes of hit, both sanctioned: the migration entry and procedure, which **must** name the retired paths to remain auditable — §drift-prevention's load-bearing exception for a migration whose subject *is* the removal — and three `runtime/` source comments describing the history for a reader of the code that used to depend on it. AC2's claim is likewise about absence: `ductus.md` carries no recommendation flow, no registry row, no `[workflows]` schema, and its example TOML shows `[pinned]` and `[[review.disabled-rule-files]]` with no `[workflows]` neighbour.

**The enumeration checked term by term, which is where this campaign's defects have lived.** AC3 claims the migration's target paths cover "the scaffolded command directory's **22 known filenames (13 current + 9 legacy)**". The procedure's exact-set list carries 13 current names on one line and 9 legacy `{category}-{language}-{tool}.md` names on the next — counted individually, 13 and 9, totalling 22. The criterion is exact rather than approximately right. AC3's other four targets (`skills/`, `workflows/registry.json`, `framework/workflows/`, plus `framework/skills/`) are all present in `target_paths`, and the entry carries `id`, `introduced_in`, `sunset_after`, `summary` and `procedure_file` as required.

**How the rest were checked.** AC4, AC5, AC6: the procedure's six steps cover exact-set deletion with per-file reporting, pinned-path preservation reported as `pinned (kept):`, adopter-authored files surviving by exact-match construction, the `[workflows]` section removal preserving every other table byte-for-byte, and an idempotency check that exits silently when no target exists — which is AC6's no-op case stated as step 1. AC7: neither retired migration id appears in `migrations.toml`, both procedure files are gone from `framework/migrations/`, and `CHANGELOG.md` §Archived migrations carries both texts under their own headings with introduced/sunset versions. AC8: `005-workflows` carries the sunset note at the top of its body, links this spec, and its `status:` is still `done` — both halves of the criterion. AC9: the constitution's canonical-source map has no Workflow registry row. AC10: `scripts/audit/run-all.sh` exits 0, verified in this session with a positive control proving it fails when it should. AC11: `gvrn-v0.23.0` exists as a published tag, matching the entry's `introduced_in`; the criterion names the pre-049 tag scheme correctly, because that is the name the tag was actually published under.

**On the diff base.** No commit records 043 entering `in-progress`, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead. 043 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
