---
title: "000-slash-commands — spec"
status: done
dependencies: []
tags: [commands, pipeline, templates]
review:
  last-run: 2026-08-17T12:17:15Z
  reviewed-against: ccdd3ac
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 16
analyze:
  last-run: 2026-09-13T18:05:00Z
  analyzed-against: d9c1d784df1524b86937efae2716adc8464e4990
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 0
  analyzed-digest:
    plan.md: d0dc9d9867d7a899e633f88a32232a63f959452ad06a1cc91dacd6af3d0ed6af
    review.md: 4fc8372f96f37f2ee91bcdfc6ec7e1a52db027d2fdb6488ca6af2ce63367160a
    scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
    scenarios/command-autocomplete-summary.md: 2d1bbd64b1ced5c5abba7e5160daa0ad9b9319998709e213aaa04ee01ace0ba0
    scenarios/criterion-route-after-draft.md: 80781ef6092c5f1072106a856b1664f3b2bfb9b72e48df61a1e91e5fd6b8c1d4
    scenarios/dashboard-dependencies-column.md: dee20040cb8baefeaf59a0283da0b1126f501d8173d70fb052aec6c38eef3a88
    scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
    scenarios/scenario-without-task-visibility.md: 912b16e99355b5fa17a6fd86fc4e640ffbe91375cf95394618c8954136e5960d
    scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
    scenarios/target-clear-flag.md: b559bb6b731140c83a6c0733013ccc5fc4a0bdc9a3ec3cab4c26c00528380c43
    scenarios/validate-fix-mode.md: a2b4b707e7705baab55683bf642c97c8e4e6e09036246dde9f2cdbf9396d6728
    scenarios/validation-gates.md: 8fc3c93217e0b2ce5cc23812db08f9e31aff8f2ea5d04bcb998528d4b9548d58
    spec.md: c1bacd18539dc509f3054f115f557f3be62a31e89f20b2a96a60a3a361b3e910
    tasks.md: 12dc83f2b8b99cfdeaf8978b0b9ff9a4d7f49126b596e05d15150fa7f7a40cf2
  blocking: false
---

# 000 — Slash Command Templates

Generic, project-agnostic slash command templates that operationalize the `ductus` development pipeline. Projects copy these commands into their `.claude/commands/{project}/` directory and customize the project name and any project-specific paths.

> **Note:** subsequent specs renamed several commands and removed one. References below to `about`, `setup`, and `next` reflect the original design; the current names are `help` (about), `configure` (setup, renamed by [012-multi-agent-govern](../012-multi-agent-govern/spec.md)), and the auto-advance `next` command was retired in favor of explicit pipeline gates surfaced by `/{project}:status` and `/{project}:target`.
>
> **Note:** [014-reclarify-backedge](../014-reclarify-backedge/spec.md) made `/{project}:amend` the owner of the `clarified` / `planned` / `in-progress` → `draft` back-edge (the command mutates spec status when recording a question on a non-`draft` spec), added a recovery path to `/{project}:clarify` for hand-edited inconsistent state (a non-`draft` spec with open questions in the body), and added overwrite-protection to `/{project}:plan` (existing `plan.md` / `tasks.md` / `data-model.md` are surfaced with a keep/replace prompt before any template copy). Gate-enforcement language below predates these changes; the current behavior of the three commands is documented in their command sources under `framework/commands/`.
>
> **Note:** path references below (`commands/`, `templates/`) reflect the original layout. The repository was later reorganized so command sources live in `framework/commands/` and templates in `framework/templates/{spec,project}/`. Adopting projects' destination paths did not change.

## Problem

The constitution defines the pipeline (spec, plan, tasks, implement) and the spec lifecycle (draft, clarified, planned, in-progress, done), but provides no interactive tooling to enforce it. Projects like anvil have built their own slash commands from scratch, duplicating the pipeline logic that should be shared.

## Behavior

`ductus` provides a set of `.md` command templates in a `commands/` directory. Each template uses a placeholder `{project}` that adopters replace with their project name. The commands enforce the pipeline gates defined in the constitution.

### Command Set

Ten commands organized into two groups:

#### Pipeline commands (run in order)

- **specify** — prompt qualifying questions to detect lightweight track, create spec (or spec-and-plan) from template, set as session target, add to README (the qualifying questions and the combined document were removed by `023-govern-refinement`; `specify` always creates `spec.md` today)
- **clarify** — resolve open questions, enumerate edge cases, verify acceptance criteria, advance draft to clarified
- **plan** — generate plan.md and tasks.md, run readiness check, advance clarified to planned
- **implement** — walk through tasks, write code/tests, verify acceptance criteria, advance planned to done

#### Utility commands

- **about** — print a fixed overview of the pipeline and command usage (no file reads)
- **target** — set the working feature for the session, persisted in a session file
- **status** — read-only dashboard of all specs, their status, artifacts, dependencies, and next actions
- **next** — auto-advance the targeted feature by running the appropriate pipeline command
- **validate** — read-only audit of artifacts for consistency, completeness, and cross-spec alignment
- **setup** — configure `.claude/settings.local.json` with permissions needed for commands to run

### Parameterization

Each command template must work for any project by replacing a single placeholder:

- `{project}` — the project name, used in command references (e.g., `/{project}:clarify`) and file paths (e.g., `.claude/commands/{project}/`)

Commands reference the session file as `.claude/{project}-session.json`.

### Session State

Commands share state through a session file that tracks the current working feature:

```json
{
  "feature": "{NNN-feature-name}",
  "path": "specs/{NNN-feature-name}",
  "setAt": "{ISO 8601 timestamp}"
}
```

### Gate Enforcement

Pipeline commands enforce gates before executing:

- **clarify** gate: spec must be at `draft`; if already `clarified` or later, report and stop
- **plan** gate: spec must be at `clarified`; if `draft`, direct to clarify first
- **implement** gate: spec must be at `planned` or `in-progress`; if earlier, direct to the right command

### Lightweight Track Detection

> **Superseded by [023](../023-govern-refinement/spec.md).** The lightweight track was removed: `framework/templates/spec/spec-and-plan.md` is deleted, the constitution's §lightweight-track section is gone, `specify` no longer asks qualifying questions, and no command source carries the two-filename fallback. The section below records what 000 delivered. The runtime still *reads* a legacy `spec-and-plan.md` so an unmigrated adopter resolves (`is_spec_path`), and `framework/migrations/spec-and-plan-sunset.md` records the rename.

As delivered, the `specify` command determined whether a feature qualified for the lightweight track by prompting the user with qualifying questions:

- Does this touch more than one module or package?
- Are there open questions or unknowns about the approach?
- Does it involve data model changes beyond trivial?
- Will it be more than ~50 lines of spec?

If all answers indicated "small and clear," specify created `spec-and-plan.md` from a combined template instead of `spec.md`. The `clarify` and `plan` commands detected which file existed and adapted: `clarify` worked on whichever file was present, and `plan` skipped plan creation if `spec-and-plan.md` already contained the plan section.

### Template References

Pipeline commands reference spec templates from the project's `specs/templates/` directory, not from `ductus`. Each project copies the `ductus` templates into their own `specs/templates/` during bootstrap.

## Why this spec is still `in-progress` (2026-08-16)

Every acceptance criterion is met and its review is current. The single thing
holding it out of `done` is the pre-done gate's scenario check: three open
questions remain across `criterion-route-after-draft` and
`scenario-without-task-visibility`, and a spec is not complete while its
scenarios carry questions ([§spec-lifecycle](../../framework/constitution.md#spec-lifecycle)).

**Operator decision, 2026-08-16: these were deliberately not resolved for the
`ductus-v0.28.0` release.** They predate that work, this spec is not one the
release closes, and resolving them requires design decisions rather than
mechanical cleanup. The release shipped without them by explicit choice, not by
oversight.

To close this spec, target each scenario and walk its questions:
`/{project}:target 000/criterion-route-after-draft` then `/{project}:clarify`,
and the same for `scenario-without-task-visibility`. A question that is deferred
rather than undecided belongs in that scenario's `## Resolved Questions` with its
trigger recorded — only `## Open Questions` entries count against the gate.

## Acceptance Criteria

- [x] AC1: Ten command template files exist in `commands/` directory
- [x] AC2: Each template uses `{project}` as the only project-specific placeholder
- [x] AC3: Pipeline commands (specify, clarify, plan, implement) enforce gates matching the constitution's spec lifecycle
- [x] AC4: The `about` command prints a self-contained guide without reading any files
- [x] AC5: The `target` command writes a session file and displays feature status
- [x] AC6: The `status` command scans all spec directories and displays a dashboard table
- [x] AC7: The `next` command maps current status to the correct pipeline command
- [x] AC8: The `validate` command checks spec integrity, artifact completeness, plan consistency, task consistency, dependencies, and cross-spec references
- [x] AC9: The `setup` command configures permissions for common operations (git, lint, file reads)
- [x] AC10: The `specify` command determines the next feature number, creates the spec directory, and updates README
- [x] AC11: Commands reference `specs/templates/` for templates (not `ductus` templates)
- [x] AC12: Commands reference `.claude/{project}-session.json` for session state
- [x] AC13: The `validate` command runs `npx markdownlint-cli2` on the feature's files as part of its checks
- [x] AC14: The `specify` command prompts qualifying questions and creates `spec-and-plan.md` for lightweight track features — **delivered, then removed by `023-govern-refinement`**: `specify` asks no qualifying questions and the combined template is deleted. Checked because 000 delivered it; the removal is 023's.
- [x] AC15: Pipeline commands detect and handle both `spec.md` and `spec-and-plan.md` — **delivered, then removed by `023-govern-refinement`**: its AC10 stripped the two-filename fallback from every command source. The runtime retains the legacy filename in `is_spec_path` so an unmigrated adopter still resolves, which is a read tier rather than the detection branch this criterion describes.

## Resolved Questions

- **Setup and web fetch permissions** — leave to project-specific customization. The setup command handles universal operations (git, lint, file reads). Projects add their own web fetch domains.
- **Validate and markdown lint** — validate includes a markdownlint check as part of its PASS/FAIL report. Lint compliance is a quality gate defined in the constitution.
- **Specify and dependencies** — specify accepts only a description. Dependencies are set during writing and clarifying, not at creation time.
- **Retire/archive command** — deferred. See [specs/README.md](../README.md#future-considerations). Projects can manually update status or delete directories.
- **Lightweight track handling** — the `specify` command detected lightweight track eligibility by prompting qualifying questions, created `spec-and-plan.md` when all answers indicated small and clear, and pipeline commands adapted based on which file existed. The decision was reversed by `023-govern-refinement`, which removed the track outright; see §Lightweight Track Detection above.
