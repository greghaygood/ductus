# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
     - A bug or omission inside the scope of the spec currently in progress does NOT belong
       here — it becomes a task on that spec's tasks.md. The inbox is for todos with no
       home yet; an in-progress spec is already the home, so an item logged here is routed
       straight back to it (constitution §brownfield-inbox, scope decides the destination).
     - Nothing a pipeline run finds is written here. /review, /analyze, and /implement fix,
       route, or discard their own findings in the run that found them (constitution
       §brownfield-inbox, Finding dispositions); this file holds what a person logs.
     - Write specs for areas being actively touched — let adoption spread naturally.
     - As specs are written, items migrate from here into spec updates or new scenarios.
     - Chores (project maintenance with no feature home — lint/formatting cleanup,
       dependency cleanup, repo hygiene) may be logged here too; /groom does them in the
       grooming pass and removes them. They clear when done, not by migrating to a spec.
     - The brownfield backlog drains toward empty as adoption completes; the file
       persists as long as people keep logging todos.
     - Status notes do NOT belong here. Every item must be routable by /groom to one of
       its five routes (rule, spec, scenario, chore, discard); a "where things stand"
       or "what to do next" note matches none of them, so it would be walked and
       re-discarded on every pass forever. Pipeline state is derived — read it from
       /status, tasks.md, and git — not narrated into the backlog.

     Format each item as a checkbox list entry, recorded with /log, with a brief
     description and any relevant context:
        `- [ ] {Brief description of the issue and any relevant context}`

     When an item is migrated, remove it from this list. -->

- [ ] Warn when a spec artifact exceeds the agent's single-read cap, and give every warning a fix an adopter can act on. Claude Code's Read returns only a partial first page past 25,000 tokens (measured 2026-09-28: 727 lines / ~54 KB of this repo's plan markdown; no line cap — 2,500 short lines read whole), so an adopter's 2,200-line plan.md is never read in full by /implement's single setup read (framework/commands/implement.md:111), and whether it is depends on the agent choosing to page. Check: advisory, byte-based (runtime has no tokenizer; default ~50 KB, configurable in .ductus/config.toml because the cap differs per host and per CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS), over spec.md, plan.md, tasks.md, data-model.md and scenarios (not research.md — no command reads it), run at /clarify, /plan's readiness check and /analyze — /clarify because splitting is cheapest at draft/clarified, and at done every fix except /prune or a discard reopens the spec. The message names only the fixes valid at the spec's current status: tasks.md → /prune (still over after pruning means the spec is too big); spec.md or plan.md → split the spec (can a slice reach done on its own?) or trim (plan: code sketches, Affected Files rows for finished work); data-model.md → split the spec or trim, never into sub-files (review staleness matches the exact name, runtime/src/primitives/analyze_subjects.rs:106); scenario → promote to its own spec (constitution §scenario-promotion); any file → discard in /analyze with a reason, keyed on the file's read-page count so it re-fires when the file grows past another page. Gaps a warning alone leaves: (1) no command splits a spec — /consolidate only merges, and splitting is the main fix for spec.md, plan.md and data-model.md; (2) no way to move existing spec content into a scenario — /amend only records new input, and its scenario route appends an implement task for behavior already built; (3) a large plan that really is one concern has no fix but paging or a discard (reading the plan by section would close it); (4) /plan tells the agent to put code snippets in the plan (framework/commands/plan.md:102), so trimmed plans regrow. Suggested scope: the warning, the status-aware message, the discard path and gap 4; gap 1 is the largest and may be its own spec.
