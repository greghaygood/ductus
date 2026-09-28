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

- [ ] Warn when a spec artifact exceeds the agent's single-read cap. Claude Code's Read returns only a partial first page past 25,000 tokens (measured 2026-09-28: 727 lines / ~54 KB of this repo's plan markdown; no line cap — 2,500 short lines read whole), so an adopter's 2,200-line plan.md is never read in full by /implement's single setup read (framework/commands/implement.md:111), and whether it is depends on the agent choosing to page. Proposed: an advisory (non-blocking), byte-based threshold (runtime has no tokenizer; default ~50 KB, configurable in .ductus/config.toml because the cap differs per host and per CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS) checked by /analyze and /plan's readiness check over spec.md, plan.md, tasks.md, data-model.md and scenarios (not research.md — no command reads it); an oversized spec.md or plan.md points at splitting the spec (can a slice reach done on its own?), an oversized tasks.md at /prune.
