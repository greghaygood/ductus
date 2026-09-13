---
spec: 004-tech-stack-selection
reviewed-at: 2026-09-13T14:04:03Z
reviewed-against: 21c2f9efccb7459d98eb7a0f56ee0b79f1c24512
diff-base: 21c2f9efccb7459d98eb7a0f56ee0b79f1c24512
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 2
scope: 2
skipped-passes: []
---

# Review — 004-tech-stack-selection

## Summary

Clean: 0 MUST, 0 SHOULD, 0 low-confidence outstanding. Re-review for the examined/scope backfill; the prior record (2026-06-11) predated ductus-v0.49.0 and carried neither `examined` nor `reviewed-digest`.

Scope and what was read. `examined: 2` of `scope: 2` — both in-scope files read end to end: `.claude/commands/ductus/init.md` (199 lines) and `AGENTS.md` (143 lines of very long lines, ~126KB, read in 13 ranges earlier this session). No file went unread, so this Summary names none.

Diff base. `HEAD` was passed. 004's plan affects two files and the natural base resolves a window spanning every change since the spec closed; both were measured before choosing, per AGENTS.md §Gotchas. The empty window makes scope equal the plan's Affected Files, so numerator and denominator describe the same two files.

Rule selection. All 11 rule files loaded. None fires: the scope is one command-definition document and a contributor guide, in a project with no HTTP surface, no database and no browser. `CFG-CONST-003` was considered against init.md's example-option lists and does not apply — those are illustrative prompt choices, not operator-tunable values.

Acceptance criteria. All nine verified against the tree, enumerations included. AC1/AC4 against init.md's 4a→4b→4c ordering; AC2/AC3 against the two gate clauses ("ask only if project type is…"); AC5 against the "2–4 example choices plus Other and Skip" rule at input 4 — the framework-implies-language inference suppresses the language *question*, so AC5 governs the questions actually asked and is not contradicted by it; AC6 against the ten-row layer→role mapping; AC7 against "if all categories were skipped, leave the Tech Stack comment placeholder unchanged"; AC8 against the absence of any second language prompt; AC9 against the per-language `.gitignore` fetch. All nine hold. `check-artifacts` was clean with an empty `skipped` array.

One defect was found and fixed at source in 21c2f9ef, before this record was written. The `framework-implies-language` scenario — a durable requirement document — justified its central requirement with a mechanism 043 deleted, in the present tense: workflow recommendation "matches registry entries on `backend_language`", so omitting the row "would silently drop" the Ruby-triggered workflows. `framework/workflows/` and its registry are gone. The requirement is unchanged and still correct, so the rationale was annotated rather than the requirement rewritten — the disposition §drift-prevention prescribes for superseded behaviour, as distinct from a moved name. 043 signposted 004's `spec.md` when it landed but not the scenario one directory down, which is how it survived; the spec body's own signpost covers "body references… below" and a scenario is a separate file.

One observation is recorded rather than fixed, and captured to the inbox by this same call: 004's primary scope file is an orphan no sweep reaches, and has drifted across five specs' renames. Repairing it versus retiring it is an operator decision — it is measured in the observation rather than swept here, per the campaign's standing decision against piecemeal sweeps.

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

- convention: .claude/commands/ductus/init.md is an orphan that no sweep reaches, and it has drifted across five specs' renames. It is the one hand-maintained file under .claude/ (AGENTS.md §Gotchas) — no framework/commands/init.md source, so no generator rewrites it; and .claude/ is deliberately absent from AGENTS.md's sweep-target manifest, correctly, because every other file there is generated output. init is also absent from help.md, README.md, docs/slash-commands.md and scripts/maintainer-only-commands.txt (which lists only `audit`), and Family 16 anchors on framework/commands/*.md source files so it never sees init either. Measured 2026-09-13, five drifts: (1) it copies 6 of the 11 shipped rule files — concurrency-backend, observability-backend, performance-backend, quality-cross and reliability-backend are missing, so a scaffolded project silently gets a smaller rule set than /ductus installs; (2) it copies a spec-and-plan.md template that 023 deleted; (3) it writes the constitution to repo-root constitution.md, the pre-044 location, not .ductus/constitution.md; (4) it writes a new project's config to .govern.toml at two sites — a write to a fresh tree, not a read of a legacy tier, so it scaffolds straight into the layout the migrations exist to undo; (5) it creates .claude/{slug}-session.json, while the session file is .ductus/session.toml for every adopter, project-name-agnostic and TOML. It also never acquires or wires the runtime, which §runtime-boundary principle 3 now makes required. This is the drift §drift-prevention's Manifest discipline exists to prevent, and that rule names this exact pair as its example — "/ductus and /{project}:init both scaffold a project" — so the framework constitution ships adopters a reference to a command they never receive. Repair-vs-retire is an operator call, not a sweep: /ductus supersedes init for every adopter, and init's only remaining role is scaffolding a brand-new project locally. None of this falsifies a 004 criterion — all nine hold. — `.claude/commands/ductus/init.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
