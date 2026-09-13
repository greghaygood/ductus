---
title: "002-project-scaffolding — spec"
status: in-progress
dependencies: [000-slash-commands, 001-system-spec-templates]
tags: [templates, bootstrap]
review:
  last-run: 2026-09-13T12:56:02Z
  reviewed-against: 674925045d17b0843724db970bed4ff6fb457ef8
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  examined: 3
  scope: 3
  reviewed-digest: {}
  blocking: false
next-criterion: 8
analyze:
  last-run: 2026-09-06T14:12:55Z
  analyzed-against: 683a1e03c463c62ea644a4466acc5873eba0d1a4
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 3
  unexamined-by-reason:
    root-absent: 3
  blocking: false
---

# 002 — Project Scaffolding Templates

Templates for the project-level files that every governance-adopting project needs beyond the constitution, AGENTS.md, and spec templates.

## Problem

Bootstrapping a new project requires creating several files that follow governance conventions but are not currently provided as templates: a project README with a feature status table, a `.gitignore` that excludes claude settings but preserves commands, a `CLAUDE.md` with import directives, and a session state file. Projects like anvil created all of these independently.

## Behavior

Governance provides additional templates in the `framework/templates/project/` directory for project-level files.

### README.md template

A project README that includes:

- Project name and description placeholder
- Quick start commands section
- Getting started section that references `/{project}:configure` and `/{project}:status` as the onramp for new contributors
- Documentation links (constitution, AGENTS, system specs)
- Feature specs table (with columns for spec, status, dependencies, description)
- Pipeline overview with slash command references
- Slash command reference table
- Instructions for working on existing specs

### .gitignore template

A minimal baseline `.gitignore` for governance-adopting projects:

- Environment and secrets (`.env`, `.env.*`)
- Claude settings exclusion with commands exception (`.claude/*`, `!.claude/commands/`)
- IDE and OS files (`.vscode/`, `.idea/`, `.DS_Store`, etc.)

The template stays minimal — no language-specific patterns. Language-specific entries are populated during bootstrap (003) by fetching from github.com/github/gitignore based on the project's tech stack.

### CLAUDE.md template

A minimal file with `@import` directives:

- Imports constitution.md
- Imports AGENTS.md

## Acceptance Criteria

- [x] AC1: `framework/templates/project/project-readme.md` exists with placeholder sections for project name, quick start, getting started (referencing the configure and status commands), documentation links, feature table, pipeline overview, and slash command reference
- [x] AC2: `framework/templates/project/gitignore` exists with minimal sections for secrets, claude settings, IDE files, and OS files — no language-specific patterns
- [x] AC3: `framework/templates/project/claude-md.md` exists with `@import` directives
- [x] AC4: Each template uses `{project}` placeholder where the project name appears
- [x] AC5: The README template includes a feature table matching the format used by constitution's spec numbering convention
- [x] AC6: The .gitignore preserves `.claude/commands/` while excluding other `.claude/` contents
- [x] AC7: All markdown templates pass markdownlint

## Resolved Questions

- **README and setup command** — the README template must include a "Getting Started" section referencing `/{project}:configure` and `/{project}:status` as the onramp for new contributors.
- **Gitignore and language patterns** — the template stays minimal (secrets, claude settings, IDE, OS). Language-specific patterns are fetched from github.com/github/gitignore during bootstrap (spec 003) based on the project's tech stack.
- **Settings.local.json template** — no static template. The `setup` slash command handles creation and configuration of `.claude/settings.local.json` as the single source of truth for permissions.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
- [001-system-spec-templates](../001-system-spec-templates/spec.md)
