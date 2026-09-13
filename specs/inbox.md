# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
     - A bug or omission inside the scope of the spec currently in progress does NOT belong
       here — it becomes a task on that spec's tasks.md. The inbox is for findings with no
       home yet; an in-progress spec is already the home, so an item logged here is routed
       straight back to it (constitution §brownfield-inbox, scope decides the destination).
     - Write specs for areas being actively touched — let adoption spread naturally.
     - As specs are written, items migrate from here into spec updates or new scenarios.
     - Chores (project maintenance with no feature home — lint/formatting cleanup,
       dependency cleanup, repo hygiene) also live here; /groom recognizes them and leaves
       them in place. They clear when done, not by migrating to a spec.
     - The brownfield backlog drains toward empty as adoption completes; incidental
       capture is ongoing, so the file persists while work keeps surfacing issues.
     - Status notes do NOT belong here. Every item must be routable by /groom to one of
       its five routes (rule, spec, scenario, chore, discard); a "where things stand"
       or "what to do next" note matches none of them, so it would be walked and
       re-discarded on every pass forever. Pipeline state is derived — read it from
       /status, tasks.md, and git — not narrated into the backlog.

     Format each item as a checkbox list entry with a brief description and any relevant
     context. Three forms are in use:

     1. Manual entry (via /log) — the simple form below:
        `- [ ] {Brief description of the issue and any relevant context}`

     2. Auto-captured finding (an agent recorded this automatically while working a task,
        per §brownfield-inbox Automatic issue capture). Lead with a category so /groom can
        route it, and include a source pointer:
        `- [ ] {category}: {summary} — {file:line or area} (captured during {NNN-feature})`
        Categories: security, leak (memory/resource), convention, bug, perf, other.
        Security issues and leaks are the highest-priority captures.

     3. Audit finding written by /ductus — stricter form (see
        specs/008-security-rules/spec.md): `- [ ] {Rule ID}: {artifact} does not address — {summary}`.

     When an item is migrated, remove it from this list. -->

- [ ] convention: backfill `examined`/`scope` and the freshness digests on the 45 spec records that predate `ductus-v0.49.0` — measured 2026-09-13, 49 of 55 review records carried no `examined` (so a `0/0/0` review is byte-identical to one whose five passes never fired), 46 no `reviewed-digest` and 45 no `analyzed-digest` (so the pre-`done` gate reports freshness as *undeterminable*, which does not block). Re-running `/ductus:review` then `/ductus:analyze` per spec is what writes the fields, and it is only honest if the passes actually read the resolved scope — this is NOT a mechanical backfill and must not be batch-written (see §Workflow's review-is-the-five-passes entry, and the 017/048 incident it records). Done so far: 016, 017, 020, 022, 027, 034, 037, 038, 039, 050. Remaining 45, ordered by durable-contract size so the cheap ones clear first — zero-contract: 001, 002, 006, 009, 011, 015, 018, 021, 024, 025, 043, 044, 046, 049, 054; then 033, 004, 003, 052, 010, 035, 007, 036, 014, 042, 012, 005, 019, 055, 028, 030, 032, 008, 031, 047, 029, 045, 041, 040, 013, 051, 023, 000, 048, and 026 last (22 scenarios, 1391 lines). Parked rather than done in the pass because it is per-spec judgment over ~13,000 lines, not a mechanical edit (captured during the 2026-09-13 systematic re-review)
