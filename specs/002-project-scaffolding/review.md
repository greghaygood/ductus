---
spec: 002-project-scaffolding
diff-base: 674925045d17b0843724db970bed4ff6fb457ef8
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T12:56:02Z
reviewed-against: 674925045d17b0843724db970bed4ff6fb457ef8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 3
reviewed-digest: {}
blocking: false
---

# Review — 002-project-scaffolding

## Summary

First review of 002 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All seven criteria were verified against the tree, and three did not hold as written — corrected in `6749250` before this review ran.

**What this review read: all three files in scope** — `framework/templates/project/project-readme.md`, `gitignore`, and `claude-md.md`, each in full. Nothing in scope went unread. `spec.md`, `plan.md` and `tasks.md` were read as well and are deliberately not counted, since they fall outside the resolved scope.

**The corrections.** AC1–AC3 named `templates/*`, retired at the framework/ reorg, and the body carried the annotation §drift-prevention says to remove rather than keep. Two further defects were reachable only by checking each criterion's *enumeration* — the failure mode this campaign found in three of the four Task-9 rule specs. AC1 required Getting Started to reference the **setup** command: no such command exists, `framework/commands/` has `configure`, and the template already says `/{project}:configure`. The requirement is unchanged and the name moved, so the criterion is swept rather than annotated. And the References note credited `scripts/gen-spec-deps.sh` with deriving `dependencies:` — that script was deleted by 022's adopter-generator-promotion and the work is the `derive-dependencies` primitive, so it was a stale behavioural claim rather than a dead path alone.

**How each criterion was checked.** AC1: the template carries `# {project}`, Quick Start, Getting Started, Documentation, a Feature Specs table, Development Pipeline and a Slash Commands table — all seven items the criterion enumerates, each checked by name. AC2: `gitignore` carries the four sections named — secrets (`.env`, `.env.*`), Claude settings, IDE, OS — and no language-specific pattern survives anywhere in the file; the later per-agent and ductus-state blocks it has grown are neither language-specific nor outside the criterion's claim. AC3: both `@import` lines are present, now naming `.ductus/constitution.md`. AC4: `project-readme.md` uses `{project}` at every point the project name appears; the other two contain no project name, so the criterion is satisfied rather than vacuous. AC5: the Feature Specs table's columns are Spec / Status / Dependencies / Description, with the Spec column carrying the `NNN-slug` form §numbering defines. AC6: `.claude/*` followed by `!.claude/commands/` — the exclusion and its exception, in that order, which is what makes the negation work. AC7: `markdownlint-cli2` exits 0 over the repo; `gitignore` is not markdown, and the criterion is scoped to "all markdown templates", so the two `.md` files are its whole subject.

**On the diff base.** No commit records 002 entering `in-progress`, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead — an empty window by construction, resolving the scope to the plan's Affected Files. 002 has no scenarios and no data model, so `reviewed-digest` is `{}`: taken and empty, which reads as current.

**Not corrected here, and why.** The body names a specific adopter project as prior art, against the standing rule that this repository never records another project's name. That is not local to 002 — it is ~100 occurrences spanning spec prose, two files that ship to adopters, and the runtime — so it is recorded as one measured inbox item rather than swept piecemeal, since a partial sweep would leave the measurement stale and the replacement text varies per sentence.

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
