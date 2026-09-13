---
title: "003-bootstrap-automation — plan"
---

# 003 — Bootstrap Automation Plan

## Overview

Create eleven slash commands in `.claude/commands/ductus/`: ten standard pipeline commands copied from `commands/` templates with `{project}` replaced by `ductus`, plus one governance-specific `init.md` that scaffolds new projects. The standard commands give governance the same pipeline enforcement as adopting projects. The init command automates the manual bootstrap process from the README.

## Technical Decisions

### Standard commands are literal copies with placeholder replacement

Each of the ten command templates in `commands/` is copied to `.claude/commands/ductus/` with every occurrence of `{project}` replaced by `ductus`. No other modifications. This ensures this repo dogfoods the exact same commands adopting projects use. If a command template is updated later, the copy is re-derived from the template — by hand as designed here, and by `scripts/gen-claude-commands.sh` since the Trade-offs entry below was reversed. The copy-and-substitute shape survived that reversal; what changed is that a script does it over a directory glob rather than a contributor over a fixed list.

### Init command is governance-specific

The init command does not exist in `commands/` — it is unique to the governance repo. It lives alongside the standard commands at `.claude/commands/ductus/init.md` and is invoked as `/ductus:init`. It orchestrates file copying, placeholder replacement, and gitignore fetching as a single slash command prompt.

### Placeholder replacement in init uses find-and-replace

The init command instructs the agent to replace `{project}` with the user-provided project name in all copied files. This is the same approach adopting projects already use — literal string replacement, not a templating engine.

### Gitignore language patterns fetched at runtime

The init command fetches `.gitignore` patterns from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` for each primary language. The fetched content is appended below the governance template's entries, separated by a comment header identifying the language. If a fetch fails, the command reports the failure and continues with the minimal template.

### Session file path

Standard commands reference `.ductus/session.toml` for session state. (Original 003 design referenced `.claude/gov-session.json` under the `{cli-config-dir}/{project}-session.json` pattern from spec 000; consolidated onto the repo-root TOML in spec 022 task 40.)

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `.claude/commands/ductus/about.md` | Create | Pipeline overview (from template) |
| `.claude/commands/ductus/target.md` | Create | Set session target (from template) |
| `.claude/commands/ductus/status.md` | Create | Spec dashboard (from template) |
| `.claude/commands/ductus/setup.md` | Create | Configure permissions (from template) |
| `.claude/commands/ductus/specify.md` | Create | Create new spec (from template) |
| `.claude/commands/ductus/clarify.md` | Create | Resolve questions (from template) |
| `.claude/commands/ductus/plan.md` | Create | Create plan and tasks (from template) |
| `.claude/commands/ductus/implement.md` | Create | Execute tasks (from template) |
| `.claude/commands/ductus/analyze.md` | Create | Audit artifacts (from template) |
| `.claude/commands/ductus/next.md` | Create | Auto-advance phase (from template) |
| `.claude/commands/ductus/init.md` | Create | Scaffold new projects (governance-specific) |

## Trade-offs

### Considered: generating standard commands dynamically from templates

Rejected here, and **later adopted** — recorded per [§drift-prevention](../../framework/constitution.md#drift-prevention)'s *Decision resolution* rule, which fires when a previously-rejected option is taken up. The rejection rested on two premises that did not hold: that copying with replacement stays simple, and that this repo's project name is fixed. The command set grew from ten to sixteen, and 049 renamed the project to `ductus`, so hand-copied files drifted from the templates they came from. `scripts/gen-claude-commands.sh` now globs `framework/commands/*.md`, substitutes `{project}` and `{cli-config-dir}`, writes `.claude/commands/ductus/`, and deletes any file whose source is gone; the pre-commit hook runs it, and `/ductus:audit`'s check-zero precondition fails on a stale copy.

### Considered: a single `/ductus:work` command instead of ten standard commands

Rejected. Governance should use the same commands as adopting projects. A custom command would diverge from the dogfooding principle and miss bugs or friction in the templates.

### Considered: skipping the configure command for this repo

Rejected. Even though this repo already has a settings file, the configure command is part of the standard set. Keeping it maintains parity with adopting projects.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
