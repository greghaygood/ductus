---
title: "001-system-spec-templates — spec"
status: in-progress
dependencies: []
tags: [templates, pipeline]
review:
  last-run: 2026-09-13T12:54:02Z
  reviewed-against: 4ba8a955a5c7a7b843b1bff2d9e188dbd166a712
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  examined: 3
  scope: 3
  reviewed-digest: {}
  blocking: false
next-criterion: 7
analyze:
  last-run: 2026-09-13T12:54:57Z
  analyzed-against: fa640bd978f62277eba20022f9e5b6494a0d66d8
  hard-fail: 0
  blocking-findings: 0
  advisory: 1
  unexamined: 0
  analyzed-digest:
    plan.md: f6f024c8bd9365842c615bc556b5e70dd654ad68fe7237d7b28eebd22fe5b4f6
    review.md: 292a2a4a3084fc69347b5df193ac1b27474a2f80b04789b1a96f7c9ab450626c
    spec.md: eaa52d569a2e5b67efb9b73ee579d430e2dd816503a21e4c6c3338483e8b90aa
    tasks.md: 27fba330e2e88fa2084352625386da19e50eb0e268608652abc2e10ac2415574
  blocking: false
---

# 001 — System Spec Templates

Templates for the cross-cutting system specs that the constitution references but does not provide: `system.md`, `errors.md`, and `events.md`.

## Problem

The constitution's spec phase defines a directory structure that includes `system.md`, `errors.md`, and `events.md` under `specs/`. The README tells adopters to "write `specs/system.md` describing your architecture" but provides no template or guidance on what sections to include. Adopter projects have built these from scratch, establishing patterns that should be reusable.

## Behavior

`ductus` provides three new templates in the `framework/templates/project/` directory. Each template has placeholder sections with comments explaining what to fill in, following the same pattern as existing templates (spec.md, plan.md, etc.).

### system.md template

Prompts adopters for architectural patterns that feature specs reference. Sections are prompts, not prescriptions — adopters include what applies and remove what doesn't:

- Configuration approach (environment variables, config files, etc.)
- Application lifecycle (startup sequence, initialization order)
- Request or message lifecycle (middleware chain, handler pattern)
- Multi-tenancy or scoping model (if applicable)
- Shared infrastructure packages/modules
- Module or component pattern (isolation rules, dependency injection)

### errors.md template

Covers error handling conventions:

- Error response format (JSON structure, fields)
- Error code naming convention
- HTTP status code mapping (or equivalent for non-HTTP)
- Validation error format (per-field details)
- Logging conventions for errors (severity mapping)
- Internal vs external error exposure rules

### events.md template

An event catalog — a registry of event types populated as features are built. Includes:

- Event catalog structure (how to document each event type)
- Event envelope or message format
- Subject or topic naming convention
- Publisher and subscriber documentation pattern
- A comment suggesting that projects consider specifying retry policy and dead-letter handling as dedicated feature specs

## Acceptance Criteria

- [x] AC1: `framework/templates/project/system.md` exists with placeholder sections for configuration, lifecycle, request flow, shared infrastructure, and module pattern
- [x] AC2: `framework/templates/project/errors.md` exists with placeholder sections for error format, code convention, status mapping, validation errors, and logging
- [x] AC3: `framework/templates/project/events.md` exists with placeholder sections for event catalog, envelope format, naming convention, and a comment suggesting retry/dead-letter as feature specs
- [x] AC4: Each template uses HTML comments with commented-out example content, consistent with existing template style
- [x] AC5: Templates are technology-agnostic — no language-specific code or framework references
- [x] AC6: Each template starts with a top-level heading and passes markdownlint

## Resolved Questions

- **Graceful shutdown in system.md** — too implementation-specific for a template. System.md sections are prompts for useful architectural patterns, not prescriptions. Shutdown behavior belongs in a feature spec if needed.
- **Retry policy and dead-letter in events.md** — keep in feature specs, not the catalog. Events.md is a registry. Include a comment suggesting projects consider specifying retry policy and dead-letter handling as dedicated feature specs.
- **Example content style** — use commented-out examples matching existing template style (spec.md, plan.md, etc.). Examples make templates self-documenting.
