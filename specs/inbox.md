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

- [ ] `framework/commands/analyze.md` (Finding dispositions) defines **Fixed** as a chore — a mechanical edit adding no durable requirement — and **Routed** as a write to the groom tree's home, but `write-analysis` rejects a `routed` finding whose re-check no longer produces it (`runtime/src/primitives/write_analysis.rs` test at line 855: `gone_but_routed.live = false` → error). So a finding resolved by a confirmed route that removes it from the re-check (e.g. a body edit on the spec in hand) must be recorded `fixed`, which the command source's definitions contradict; the primitive's error is the only statement of the rule. Likely home: 058-findings-route-at-discovery, which owns the dispositions.
