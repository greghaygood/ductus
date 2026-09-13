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

- [ ] convention: backfill `examined`/`scope` and the freshness digests on the 45 spec records that predate `ductus-v0.49.0` — measured 2026-09-13, 49 of 55 review records carried no `examined` (so a `0/0/0` review is byte-identical to one whose five passes never fired), 46 no `reviewed-digest` and 45 no `analyzed-digest` (so the pre-`done` gate reports freshness as *undeterminable*, which does not block). Re-running `/ductus:review` then `/ductus:analyze` per spec is what writes the fields, and it is only honest if the passes actually read the resolved scope — this is NOT a mechanical backfill and must not be batch-written (see §Workflow's review-is-the-five-passes entry, and the 017/048 incident it records). Done so far: 001, 002, 006, 009, 011, 016, 017, 020, 022, 027, 034, 037, 038, 039, 050. Remaining 40, ordered by durable-contract size so the cheap ones clear first — zero-contract: 015, 018, 021, 024, 025, 043, 044, 046, 049, 054; then 033, 004, 003, 052, 010, 035, 007, 036, 014, 042, 012, 005, 019, 055, 028, 030, 032, 008, 031, 047, 029, 045, 041, 040, 013, 051, 023, 000, 048, and 026 last (22 scenarios, 1391 lines). Parked rather than done in the pass because it is per-spec judgment over ~13,000 lines, not a mechanical edit (captured during the 2026-09-13 systematic re-review)
- [ ] convention: grounding — a `done` spec corpus, two rule files and the runtime record another adopter project's name, against §Workflow's "never record another project's name in this repository — describe the shape instead". Measured 2026-09-13: ~100 occurrences. They split three ways and only the first is a straight sweep. (1) **Spec prose** — `000` (spec + plan, 6 hits), `001` (spec + plan, 3), `002` (spec + plan + tasks, 5), `017` (2 scenarios), `022` (6 scenarios), `036` (spec + plan + tasks, 4): each states *what kind of project* it derived from or was motivated by, so the name drops out with no loss of meaning. (2) **`framework/rules/quality-cross.md`** `QUAL-STUB-001`'s Rationale and `framework/migrations/{ductus-rename,session-file-consolidate}.md` — these **ship to adopters**, which makes the leak worse than in a spec body. (3) **`runtime/`** — source doc comments, `CHANGELOG.md`, and test fixtures that use the name as a *placeholder project* (`tests/fixtures/exec-auggie/`, `exec-opencode/`, `parity.rs`, `host.rs`): a fixture is the sanctioned form per that same entry, so whether these count is an operator call, and changing them is a `runtime/` edit that needs the version bump and `ductus-v<version>` tag. Note the entry's own warning: the replacement text varies per sentence, so this is **not** a uniform substitution and does not earn the mechanical-sweep exemption — each affected spec takes the back-edge (captured during /ductus:analyze of 001)
- [ ] convention: 049's rename swept the `govern`/`gvrn` *tokens* but not the pre-rename product name written as an English common noun, so 27 occurrences survive across 16 live artifacts — "the governance framework", "projects adopting governance", "the governance pipeline", "governance-adopting", "governance conventions", "the governance root", "the governance project". Measured 2026-09-13 over `specs/`, `framework/`, `README.md`, `docs/`. Affected: `000` (plan + tasks), `002`, `003`, `005` (plan), `006` (spec + plan — swept in place, that spec was open), `007`, `008`, `009` (spec + plan), `012`, `013` (spec + plan + data-model). This is the [§drift-prevention](../framework/constitution.md#drift-prevention) *behaviour-change needs a prose-claim sweep* case rather than the identifier one: the residue contains none of the tokens 049 grepped for, which is exactly why a path- and identifier-scoped sweep passed straight over it. `001` was swept incidentally while open for its own correction, so the count above excludes it; the replacement varies by sentence ("the framework", "`ductus`", or a rephrase), so this is **not** a uniform substitution and each affected spec takes the back-edge (captured during /ductus:analyze of 006)
- [ ] bug: four artifacts disagree on whether a brownfield `draft` may carry **zero** acceptance criteria, and the two execution paths answer differently — which [§runtime-boundary](../framework/constitution.md#runtime-boundary) forbids ("the two paths share one contract; neither wraps the other"). `framework/commands/analyze.md` §Spec integrity lists "Acceptance criteria section exists with at least one checkbox item" as **blocking**, unscoped by status, so on the markdown-only path a skeleton spec with a placeholder comment and no criteria fails the gate. The runtime disagrees by omission: `check-artifacts` implements no minimum-criterion family at all (its eight families are artifact-completeness, task-consistency, scenario→task, review-drift, analyze-drift, scenario-questions, decision-drift, criterion-path-existence and criterion-labels), so the same spec passes clean. Meanwhile `framework/commands/specify.md` tells authors to "leave the section with a placeholder comment if no criteria are known yet", `specs/011-brownfield-process/spec.md` §Capture edge cases says "No acceptance criteria known at all — valid. The Acceptance Criteria section can be empty at `draft`", and `framework/templates/spec/spec.md` scopes the requirement to the *clarify* gate ("required before /{project}:clarify will advance the spec") — which is the only one of the four consistent with §brownfield-process's "sparse acceptance criteria are expected and valid". The constitution itself is silent on zero-at-draft: §spec-requirements requires the section, the §readiness-check requires non-empty criteria at `planned`. Deciding which reading wins is an operator call, not a sweep — it sets whether the brownfield entry path 011 exists to enable is gated on day one. Found 2026-09-13 verifying 011's AC7, which is *not* itself false (it says "without requiring **comprehensive** criteria", and sparse is not zero) (captured during /ductus:analyze of 011)
