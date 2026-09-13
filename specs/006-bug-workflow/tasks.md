---
title: "006-bug-workflow — tasks"
---

# 006 — Bug Workflow Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Create scenario and inbox templates

- [x] Create `framework/templates/spec/scenario.md` with a `section` frontmatter field, Context, Behavior, and Edge Cases sections
- [x] Create `framework/templates/project/inbox.md` with flat inbox format and migration rules
- [x] Update `framework/templates/spec/spec.md` to reference the `scenarios/` directory convention

Done when: all three template files exist with correct structure, and `spec.md` template mentions scenarios.

## 2. Update constitution

- [x] Add bug handling section with the decision tree (three branches: no spec, ambiguous spec, clear spec)
- [x] Add scenario lifecycle documentation (scenarios as first-class artifacts, directory convention, when to create vs. not)
- [x] Update the spec phase file structure to include `scenarios/` subdirectory

Done when: `framework/constitution.md` includes bug handling, scenario lifecycle, and updated file structure showing `scenarios/`.

## 3. Create `/ductus:amend` command

- [x] Create `framework/commands/amend.md` template: requires active session target, confirms target, walks decision tree, creates scenario file in `scenarios/`, appends task to `tasks.md`
- [x] Handle edge cases: no session target, no `tasks.md`, duplicate scenario name, parent spec is `done`
- [x] Create `.claude/commands/ductus/amend.md` by copying template and replacing `{project}` with `gov`

Done when: both command files exist, `/ductus:amend` creates scenario files under the correct feature's `scenarios/` directory and appends linked tasks to `tasks.md`.

## 4. Create `/ductus:groom` command

- [x] Create `framework/commands/groom.md` template: reads `specs/inbox.md`, walks each item through the decision tree, migrates items to specs or scenarios, removes resolved items
- [x] Handle edge cases: `specs/inbox.md` does not exist, `specs/inbox.md` is empty
- [x] Create `.claude/commands/ductus/groom.md` by copying template and replacing `{project}` with `gov`

Done when: both command files exist, `/ductus:groom` processes inbox items and migrates them.

## 5. Update existing command templates

- [x] Update `framework/commands/help.md` to document `/amend`, `/groom`, scenario conventions, and bug workflow
- [x] Update `framework/commands/status.md` to display scenario counts per spec
- [x] Update `commands/next.md` to suggest `/amend` as a next action when appropriate (the `/next` command was retired by a later spec; this records what was done at the time)
- [x] Update `framework/commands/analyze.md` to check that scenario-linked tasks are complete

Done when: all four command templates include the new functionality.

## 6. Re-derive dogfooded command copies

- [x] Re-derive `.claude/commands/ductus/help.md` from updated `framework/commands/help.md` (replace `{project}` with `gov`)
- [x] Re-derive `.claude/commands/ductus/status.md` from updated `framework/commands/status.md`
- [x] Re-derive `.claude/commands/ductus/next.md` from updated `commands/next.md` (both retired with the command)
- [x] Re-derive `.claude/commands/ductus/analyze.md` from updated `framework/commands/analyze.md`

Done when: all four dogfooded copies match their templates with `{project}` replaced by `gov`.

## 7. Update README

- [x] Add bug workflow and scenario conventions documentation to `README.md`
- [x] Update the feature specs table with correct status for 006-bug-workflow

Done when: `README.md` documents the bug workflow and the feature table reflects current statuses.

## 8. Final lint and verification

- [x] Run `npx markdownlint-cli2` on all new and modified files
- [x] Verify all acceptance criteria from the spec are addressed by the tasks above

Done when: all files pass lint and every acceptance criterion maps to a completed task.
