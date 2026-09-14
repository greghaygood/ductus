---
title: "000-slash-commands — scenario: validate-fix-mode"
spec-ref: "000-slash-commands — Command Set / validate"
tags: []
---

# Validate Fix Mode

## Context

The `validate` command detects issues like unchecked checkboxes on completed items, but only reports them. The user must manually fix each one. For checkbox state mismatches — where the completion status is clear from context (e.g., spec is `done` but acceptance criteria are still `- [ ]`) — the fix is mechanical and safe to automate.

## Behavior

> **Superseded by `017-derive-dont-ask`.** The mode below shipped on 2026-04-06 and
> 017 *"remove[d] --fix mode entirely (and the flag from argument-hint)"* in its own
> commit message. A `--fix` flag exists again today, on `/{project}:analyze`, but it is
> a different flag: `framework/commands/analyze.md` §Scope Boundaries states the command
> *"never mutates an artifact it audits"*, and the flag's only writes are guarded
> `done → in-progress` reverts on two triggers — a `review:` block drifted to blocking,
> and unresolved scenario open questions — each emitting a non-silent notice. No checkbox
> is auto-corrected and no lint run follows a fix. The section below records what 000
> delivered. The `--all` flag is the part that survived: it still scans every feature
> directory under the configured spec root.

- The `validate` command accepts an optional `--fix` argument (or equivalent signal in the command args).
- In fix mode, after running all checks, validate automatically corrects fixable issues instead of just reporting them.
- Fixable issues:
  - Acceptance criteria checkboxes in specs with status `done` — update `- [ ]` to `- [x]`
  - Task checkboxes in `tasks.md` where all sub-items are marked `- [x]` — update the parent `- [ ]` to `- [x]`
  - Scenario-linked tasks where the spec status is `done` — update `- [ ]` to `- [x]`
- Non-fixable issues (report only, do not auto-correct):
  - Specs with status `in-progress` — cannot determine which criteria are truly met without verification
  - Missing artifacts (no plan, no tasks) — structural issues require human decisions
  - Lint failures — require manual correction
- Fix mode displays each correction it makes, showing the file, line, and change.
- The `--all` flag scans every feature directory under `specs/`. Without `--all`, validate operates on a single feature (from argument or session target).
- `--all` and `--fix` can be combined: `--all --fix` scans and fixes all features.
- After applying fixes, run `npx markdownlint-cli2` on modified files.

## Edge Cases

- If no fixable issues are found, report "No fixes needed" and exit cleanly.
- If validate is run without `--fix`, behavior is unchanged — report only.
