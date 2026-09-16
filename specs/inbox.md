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

- [ ] convention: `validate-frontmatter` emits `severity: "blocking"` for every finding, including malformed YAML and a missing `status`, which the constitution's Validation Severity section classifies as **Hard fail** — so the primitive's vocabulary is coarser than the schema it enforces and a reader cannot tell the tiers apart — runtime/src/primitives/validate_frontmatter.rs (captured during 057-analyze-artifact-and-record-relocation)
- [ ] `cargo fmt` collapsed backslash line-continuations inside three user-facing message literals in `runtime/src/`, leaving runs of 18-19 spaces mid-sentence in text an operator reads: `check_artifacts.rs:549` (the artifact-unreadable finding, from `8c4ca744`) and `check_review_gate.rs:822` and `:827` (the two cross-spec-impact guidance strings, from `11fe1928`). Measured 2026-09-15 with a python scan over every string literal in `runtime/src/*.rs` (64 literals carry a 3+ space run; all but these are deliberate — YAML fixtures, indentation builders, whitespace-input tests). Four further instances in `check_review_agreement.rs` are excluded because spec 057 task 14 deletes that file. Outside 057's scope (that spec relocates records; these strings are about unreadable artifacts and cross-spec impact) and predating it, so not fixed inline. Destination is `runtime/` — a release — but the fix is three whitespace edits, so batch it into the next runtime tag rather than spending one on it.
