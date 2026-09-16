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

- [ ] convention: `validate-frontmatter`'s severity vocabulary is coarser than the schema it enforces, and the mis-tiering now runs in **both** directions — `runtime/src/primitives/validate_frontmatter.rs`, `framework/commands/analyze.md` step 2 (captured during 057-analyze-artifact-and-record-relocation; re-measured 2026-09-16 at `ductus-v0.50.0`). **The original claim — "emits `blocking` for every finding" — is no longer true**: 057 task 8 added two `hard-fail` sites, so it is 13 blocking and 2 hard-fail across 15 constructed findings. **Eight emit `blocking` where the constitution's Validation Severity section says Hard fail**: `:37` (frontmatter not valid YAML), `:57` (frontmatter not a mapping), `:75` / `:82` / `:87` (status out of set / non-string / missing), `:98` / `:106` / `:111` (dependency entry non-string / dependencies non-list / missing). **Five are correctly `blocking`**: `:161` and `:172` (`folds-into` shape), `:201` and `:210` (`cross-spec-impact` shape), `:242` (a residual `review:`/`analyze:` block, which the constitution classifies Blocking explicitly and calls out as *not* a hard fail). **Two are correctly `hard-fail`**: `:264` and `:271` (a `review.md` / `analysis.md` that exists but carries no readable record). **The consumer is the other half and must move with it.** `framework/commands/analyze.md` step 2 states that the primitive emits every finding as `severity: blocking` and that the host therefore renders **all** frontmatter findings in the report's hard-fail tier. That sentence is now factually stale, and its rule mis-tiers in the opposite direction: the `:242` residual-block finding, which the constitution says is Blocking precisely because *the spec file itself parses*, is rendered as Hard fail. Fixing only the primitive would leave the host flattening it again. The `hard-fail` variant already exists and is already emitted, so nothing new is needed on the type; no code branches on the string — `hard_fail` elsewhere in `runtime/src` is `AnalyzeBlock`'s count field, unrelated.
- [ ] `cargo fmt` collapsed backslash line-continuations inside three user-facing message literals in `runtime/src/`, leaving runs of 18-19 spaces mid-sentence in text an operator reads: `check_artifacts.rs:549` (the artifact-unreadable finding, from `8c4ca744`) and `check_review_gate.rs:822` and `:827` (the two cross-spec-impact guidance strings, from `11fe1928`). Measured 2026-09-15 with a python scan over every string literal in `runtime/src/*.rs` (64 literals carry a 3+ space run; all but these are deliberate — YAML fixtures, indentation builders, whitespace-input tests). Four further instances in `check_review_agreement.rs` needed no fix — spec 057 task 14 deleted that file, and it is gone as of `ductus-v0.50.0`. Outside 057's scope (that spec relocates records; these strings are about unreadable artifacts and cross-spec impact) and predating it, so not fixed inline. All three sites re-verified present with their space runs on 2026-09-16 at `ductus-v0.50.0`; `check_artifacts.rs:549` is the `message: format!(` whose literal sits on the next line. Destination is `runtime/` — a release — but the fix is three whitespace edits, so batch it into the next runtime tag rather than spending one on it. **`ductus-v0.50.0` shipped 2026-09-16, so the batch is the next tag after it**, and the sibling item above shares that destination: the two are one release session, not two chores (AGENTS.md §Workflow, the grooming entry's rule 5).
