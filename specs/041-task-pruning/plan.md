# 041 — Spec Directory Pruning Plan

Implements [041 — Spec Directory Pruning](spec.md).

## Overview

Ship `/{project}:prune` as a thin command over two deterministic runtime
primitives: `prune-tasks`, which reduces `tasks.md`, and `prune-plan`, which
finds the `plan.md` sections outside the design record and removes the ones
the operator confirms. Both do all the mechanical work — parse, classify,
gate, and write atomically — and return compact summaries that never carry a
file body. The command owns judgment only: resolve the session target (or walk
the corpus under `--all`), render the preview, move a plan section's durable
pieces to their homes, take each confirmation, and call each apply pass.
`/{project}:analyze` reports plan sections outside the record through
`prune-plan`'s preview, so one classification and one finding key serve both
commands. The rule behind the plan half — a plan records the design as it
stands — lands in the constitution and the plan template. Parsing reuses the
existing `tasks.md` machinery so `prune-tasks` sees exactly the task set
`read-tasks` and `mark-task` see. See [data-model.md](data-model.md) for both
segmentations, both classifications, and the request/response schemas.

## Technical Decisions

### Reuse the `tasks.md` parser — no new grammar

`prune-tasks` builds its segmentation from the shared helpers in
`runtime/src/primitives/mod.rs`: `detect_tasks_structure` (Flat → task level
2, `## N.`; Phased → task level 3, `### N.` under `## …` containers),
`parse_atx_heading`, `split_numbered_heading`, `SkipScanner`, and
`checkbox::find_checkbox_line`. Phase tracking is inline rather than through
`iter_phase_ranges`, because the empty-phase-drop rule needs the governing
phase as an index into the block list rather than as a line range.
A task section's line range terminates at the next heading whose level is
`<= task_level` — the same rule `mark-task`'s `locate_task_range` uses. This
guarantees the section-boundary grammar the spec deferred to the plan matches
every other tasks primitive; no separate parser is introduced.

### Segmentation → classification → rebuild

The `run` function performs three passes: (1) segment the file into a
preamble and an ordered list of phase-heading / task-section blocks; (2)
classify each task section — `Spent` (≥1 checkbox, all checked), `Pending`
(any unchecked), `NoCheckbox` (zero checkboxes, always preserved); (3) rebuild
the output per mode. Classification counts checkboxes via
`checkbox::find_checkbox_line`, which already skips `- **Done when**:` lines.
The classification table and block model are specified in
[data-model.md](data-model.md).

### One primitive, two modes, an `apply` flag

`prune-tasks` takes `feature`, `reset: bool`, `force: bool`, `apply: bool`,
plus `all: bool` (see *`--all` repeats a one-spec reduction per spec*).
`apply: false` is a pure preview — it computes the segmentation,
classification, and gate decision and returns the compact summary (counts,
per-section classification, size before/after) **without writing and without
ever returning the file body**. `apply: true` recomputes and writes the
reduced `tasks.md` via the shared `write_atomic` (tempfile + rename). Keeping
the body inside the runtime on both passes is the token-reduction contract
that motivated making the primitive do its own write (resolved
runtime-eligibility question). The command calls preview → renders summary →
confirms → calls apply; the confirmation gate is preserved without
round-tripping the file through model context. `prune-plan` follows the same
preview → apply shape.

### keep-pending vs reset output

- **keep-pending** (`reset: false`): emit the preamble verbatim, keep every
  `Pending`/`NoCheckbox` section verbatim, drop every `Spent` section, and in
  phased files drop a `## …` phase container that has no surviving task
  section (no empty phases linger). Seams normalize to a single blank line
  with one trailing newline so the result is `markdownlint`-clean. When no
  section is spent the output equals the input → `nothing-to-prune: true`, no
  write.
- **reset** (`reset: true`): emit the file's existing first `# …` heading
  followed by a canonical empty-tasks body — a constant equal to
  `framework/templates/spec/tasks.md` with its own H1 removed (the intro line
  and guidance comment). A unit test asserts the constant matches that template
  body so they never drift. A file with no `# …` heading fails
  `malformed-tasks` and writes nothing. The constant carries the template's
  whole guidance comment, its handoff-note guidance (below) included.

### Status gate for `--reset`

When `reset` is true the primitive reads `spec.md` frontmatter `status`.
`status == done` (or `force: true`) → `gate: "allowed"`, proceed.
Otherwise → `gate: "blocked-needs-force"`, `applied: false`, no write — a
**domain outcome** carried in the result, not an operational error (matching
the `mod.rs` convention that domain results ride the struct). keep-pending
never reads `spec.md` (`status: null`, `gate: "not-applicable"`). The command
surfaces a blocked reset by naming the status, pointing at the keep-pending
default, and mentioning `--reset --force`. Under `--all` the gate is applied
per spec, and `force` alongside `all` is refused before anything is read.

### The design record is the plan template's `##` headings

`prune-plan` classifies a `##` section as design record when its heading,
trimmed and compared case-insensitively, is one of the plan template's own
`##` headings: Overview, Technical Decisions, Affected Files and Trade-offs,
always present, and Data Model, Open Questions Resolved and Cross-spec impact,
optional. The set is a constant compiled into the primitive, pinned to
`framework/templates/spec/plan.md` by a unit test that parses the template's
`##` headings and asserts equality — the same pinning the reset body uses, and
the same bounded divergence: an adopter who customizes the template sees their
own set only on the markdown-only path. The template carries Trade-offs and
the three optional sections, each optional one marked to be omitted when it
does not apply, so it names every section `/{project}:plan` fills, Data Model
and Trade-offs among them (`framework/commands/plan.md` §Create the plan
step 2).

### `prune-plan` — fence-aware sections, digest-guarded removal

Segmentation feeds every line through `SkipScanner`
(`runtime/src/primitives/mod.rs`, `struct SkipScanner`), so a `##` inside a
fenced block or an HTML comment is not structure. Both occur: this
repository's 60 plans hold 6 `##` lines inside fenced blocks and 3 inside
HTML comments, across 4 plans (010, 026, 027, 058), and a scan that counted
them as sections would propose removing text that is not a section at all.
A section is a level-2 heading from `parse_atx_heading` and every line up to
the next heading at level 2 or above outside a skipped region; `###` and
deeper stay inside their parent, and a `#` heading opens a structural block
that is always kept. Lines before the first heading are the preamble and are
always kept.

Each section outside the record is reported with its `heading`, its `ordinal`
(position among all `##` sections), `lines`, `bytes`, a sha256 `digest` of the
section's text, its `finding` (below), and `decided`; the result adds the
spec's `status` and `sections-examined`, the count of `##` sections read, so
an empty list over no sections reads differently from one over sections all
in the record. Preview returns only this summary. Apply takes
`remove: [{heading, digest}]` and removes exactly those sections; if any
listed section's current digest differs, or no section carries that heading,
the whole apply is refused as the domain outcome `stale-sections` and nothing
is written, because the host's moves were judged against the text it read.
An apply with nothing listed, a list without an apply, and a listed
design-record heading are refused before anything is read, the last because
prune never removes a design-record section. Seams normalize as
keep-pending's do, through a renderer the two primitives share in `mod.rs`,
and the write is `write_atomic`. `apply` and `remove` default, as
`prune-tasks`' `apply` does (`PruneTasksArgs.apply`, `#[serde(default)]`), so
a dispatch with neither argument is always a preview.

A feature with no `plan.md` is the domain outcome `missing`, not an error:
`/{project}:analyze` dispatches the preview on every spec, a spec below
`planned` has no plan, and the exec walker halts on a primitive error — so an
error would stop every exec analyze of a `draft` or `clarified` spec, the
`analyze-basic` fixture among them.

### One finding key serves the advisory and prune

`plan_section_finding(heading)` returns the family `plan-record` and the
message `plan.md §{heading} is outside the design record`. `write-analysis`
keys a finding `{family} — {message}` (`runtime/src/primitives/write_analysis.rs`,
`finding_key`), so the stored key is fully determined by the heading.
`prune-plan` reads `analysis.md`'s stored decisions with `read_decisions`
(`runtime/src/primitives/decisions.rs`) and sets `decided: true` for a section
whose key has a stored `discarded` decision, compared with `same_key`. A
decision analyze re-matched by judgment keeps its original key (058's
`decision-key`), so a key that no longer matches is left `decided: false` for
the host to match the way `/{project}:analyze`'s stored-decision step does,
before it proposes the section.

### The reopen trigger is computed from the diff

`prune-plan`'s result carries `reopen-required` on every call against a
`done` spec, and omits it otherwise — absent is not computed, never `false`.
It is true when any design-record section's text differs from the same
section in `plan.md` at HEAD, or when `tasks.md` adds an unchecked checkbox —
holds an unchecked checkbox line, trimmed, more times than it does at HEAD,
so a box moved or re-indented adds nothing — the two diff-visible triggers
the scenario fixes. A preview computes it over the tree as it stands:
removing a section outside the record changes no design-record section, so
a reopen that edits made before the run already require is known before the
host writes anything. An apply computes it over the tree it leaves, whether
or not the removal was refused, because the host's moves are already on
disk, and before its own write, so an apply that cannot read HEAD writes
nothing. Both read HEAD through one shared reader, promoted from the private
`read_blob_at_head` in `runtime/src/primitives/check_stuck.rs` into `mod.rs`
and built on `ProjectRepository`, so a project in a repository subdirectory
resolves correctly (the 059 convention). An artifact absent at HEAD is not a
trigger: only a `done` spec is reopened, and a `done` spec's artifacts are
committed. The command, not the primitive, performs the reopen with
`set-status` (`from: done`), so status mutation stays in the one primitive
that guards it.

### `--all` repeats a one-spec reduction per spec

Both primitives take `all: bool`; exactly one of `feature` and `all` is
required, and anything else is refused with the existing `MissingArgument` /
`InvalidArgument` variants. The walk is `list_feature_dirs` in
`feature_dir_cmp` order — sequential and branch-scoped directories alike. The
result carries `examined`, a `features` list with one entry per spec that has
something to report — each beside its `feature` — and `skipped` naming each
feature without the artifact (`no-tasks-file`, `no-plan-file`); a spec with
nothing to report is omitted and counted in `examined`. MCP requires a tool's
output schema to be an object, so each result is one struct with two
flattened, optional halves — the single-feature summary, on the wire exactly
as before, or the walk — and because the halves share one key space no key
may sit in both, which is why `prune-plan`'s section count is
`sections-examined`. A `prune-tasks` walk lists each spec as a compact line —
gate, status, applied, counts, sizes and path — rather than its summary: the
per-section records would carry every task section in the corpus — 842 of
them and 161,841 bytes over this repository's 60 specs, past the MCP output
cap and against the token-reduction contract — where the lines come to 15,373
bytes. The
line carries `status` for every spec, keep-pending included, because only a
`done` spec can be reopened and the corpus preview prices each row by it. A
`prune-plan` walk keeps full summaries, since the host judges each section by
heading. `prune-tasks` with `all` and `apply` writes every feature's
reduction under one confirmation; `prune-plan` with `all` is preview-only and
refuses `apply`, because each plan section is a per-spec judgment. This is the batch shape
`/{project}:analyze --all` already has — the same one-spec operation repeated
per spec, never one operation spanning two — so it does not contradict
`docs/slash-commands.md`'s rule that a two-spec operation is its own command;
that page's wording is sharpened to draw the distinction.

### Command layer — judgment, moves, and confirmation

`/{project}:prune` resolves the session target (erroring to
`/{project}:target` when none) unless `--all` is given, maps
`--reset`/`--force`/`--all` to the primitive args, and calls both previews.
It renders one preview: per artifact, the classification and size
before → after, plan sections kept because `analysis.md` decided them, and —
under `--all` — one row per spec with its status. The `tasks.md` reduction is
confirmed through `gate-confirm` and applied. Then, per undecided plan
section, the host reads the section with its own file tools, proposes where
each durable piece goes (a Technical Decisions entry, `AGENTS.md` for
knowledge that holds for this project alone, a pending task) and what is
dropped, and confirms the moves and the removal together, naming the reopen
the moves imply on a `done` spec. A reopen the preview reports — edits made
before the run already require it — is named in the preview, before
anything is written. On confirmation the host writes the moves and calls
`prune-plan` apply with that section's heading and digest. On a `done` spec
the command calls `set-status` once the run has written to it and the reopen
is known: after its first write when the preview reported one, else after
the first apply that does. An apply that reports a reopen nothing named — a
move whose effect the host misjudged — is asked about before the flip,
never performed silently. A declined section is skipped
for that run, and the prompt names an analyze discard as the lasting keep.
The write surface is `tasks.md`, `plan.md`, `AGENTS.md` for moved knowledge,
and status through `set-status`; knowledge that holds more widely is named
for the operator to route where §drift-prevention's *Shared knowledge stays
in git* sends it, and never written by prune. A missing `tasks.md`
stops the run with the "run `/{project}:plan`" directive only when there is
no plan either; with a plan, the plan half runs and the missing task list is
reported.

### `/{project}:analyze` reports every section outside the record

A new analyze step — step 16, after the grounding scan and before the stored
decisions, the one position that leaves steps 1–15 and every citation of them
unchanged — invokes `prune-plan` against the feature, with no `apply`, on a
spec at `planned` or later, and records **every** section it reports as an
advisory finding with the result's family and message — decided sections
included. Reporting only undecided sections would expire their stored
discards: `process-decisions` drops a decision whose key does not fire on an
unrestricted run (`runtime/src/primitives/process_decisions.rs`, the
`expired` outcome), after which prune would propose the section again. Firing
every section lets the stored-decision step match the decided ones, so they
count as discarded and nothing is asked. The exec walker binds a step's
arguments from its context (`runtime/src/interpreter/mod.rs`, the `call!`
macro), so the step dispatches with the feature and no `apply` — a preview.
The exec walker's analyze tally counts each section on a `planned`-or-later
spec as advisory (`runtime/src/interpreter/analyze_tally.rs`), as it counts
every detection step's findings, so an exec record states them. With the step
at 16, analyze's steps 17–21 are the ones numbered 16–20 before it, and every
present-tense citation of them names the new number.

### Constitution, templates and docs

- **Constitution.** §plan-phase states that a plan records the design as it
  stands, that the design record is the template's `##` sections and the only
  place a plan's claims live, and where every other kind of content goes.
  §tasks-phase's closing sentence stops saying `/{project}:prune` touches
  neither `plan.md` nor `data-model.md`, and says handoff notes go on the
  pending task they concern. §implement-phase's *nothing durable goes in a
  review record's body* gains the clause that a triage is not a record: each
  finding is dispositioned in the run, an undecided one is recorded as
  undispositioned and re-detected, and planned fixes are tasks.
- **Templates.** The plan template gains its sections (above) and a guidance
  comment on what stays out of a plan. The tasks template's guidance comment
  says a task body may carry working notes, that an ordering note goes on the
  task that must wait, and that a note is prose, never a checkbox. Both ship
  to adopters with the `update` strategy (`framework/bootstrap/ductus.md`,
  *ductus-owned shared files*), so no migration is needed.
- **Command sources.** `framework/commands/prune.md` is rewritten for the
  spec-directory scope, stored discards, the reopen, and `--all`, with a
  markdown-only reference for plan segmentation. `framework/commands/analyze.md`
  gains the new step and a `plan-record` section in its markdown-only
  reference. `scripts/gen-help-tables.sh` carries prune's new description.
- **Docs.** `docs/slash-commands.md` sharpens the flag-versus-command rule and
  rewrites the `/prune` entry, whose "One spec." and single-artifact scope go
  false; `docs/analyze.md` names the new advisory; `README.md`'s `/prune`
  line follows.

### Error taxonomy

`TasksFileMissing { root, feature }` and `MalformedTasks { path, reason }`
serve `prune-tasks`; `prune-plan` adds no variant, since a missing plan is its
`missing` outcome. `FeatureNotFound` is reused for a missing feature dir;
`MissingSpecFile` / `StatusFieldMissing` when a status read fails;
`MissingArgument` / `InvalidArgument` for `prune-plan`'s `remove`/`apply`
pairing and a listed design-record section, and for the `feature`/`all`
exclusivity, `force` with `all`, and `apply` with `all` on `prune-plan`;
`Yaml` for an `analysis.md` decisions list that does not parse; and `Git` for
a HEAD the reopen trigger cannot read. A digest mismatch is the
`stale-sections` domain outcome, not an error. Each error writes nothing.

### Runtime wiring (fully-wired primitives)

`prune-tasks` is wired through all seven registration sites so it is callable
from the CLI, MCP, and the `ductus exec` walker, and `prune-plan` takes the
same seven:

1. `schema/primitives.rs` — the Args / Result / section-record / enum types
   (kebab-case serde, `clap::Args` on the args struct) + a round-trip serde
   test per primitive.
2. `primitives/prune_tasks.rs`, `primitives/prune_plan.rs` — the `run`
   functions + inline `#[cfg(test)]`.
3. `primitives/mod.rs` — `pub mod` lines, the error variants, the promoted
   HEAD-blob reader, and the block renderer both primitives share.
4. `main.rs` — args import, `Command` arm, dispatch.
5. `mcp/server.rs` — the name in `TOOL_NAMES` + a `#[tool]` async method.
6. `interpreter/mod.rs` — the `dispatch_primitive` arm.
7. `schema/registry.rs` — the `PRIMITIVE_REGISTRY` entry, which
   `parser/mod.rs`'s `PRIMITIVE_NAMES` aliases.

An eighth site is `prune-plan`'s alone: its arm in the exec analyze tally.
Then add `prune-plan` to `framework/runtime-tools.txt` and run
`scripts/gen-configure-mcp.sh` followed by `scripts/gen-claude-commands.sh`.
`runtime/tests/mcp.rs` holds the manifest set-equal to the registry, and
`main::tests::every_registry_primitive_has_a_clap_subcommand` pins the CLI.
Finish with a `runtime/CHANGELOG.md` `### Added` entry and a minor version
bump across the three version sites, released with a `ductus-v<version>` tag
once every affected spec is `done`.

### The title broadened; the slug did not

The spec is titled *Spec Directory Pruning*, for its spec-directory scope,
while its directory stays `041-task-pruning`: a rename is a corpus sweep that
buys nothing here.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/041-task-pruning/data-model.md` | Edit | `prune-plan` segmentation, classification and schema; `all` on both primitives |
| `specs/041-task-pruning/spec.md` | Edit | Body corrections and new criteria |
| `runtime/src/schema/primitives.rs` | Edit | `PrunePlanArgs`/`PrunePlanResult` types; `all` on `PruneTasksArgs`; round-trip tests |
| `runtime/src/schema/registry.rs` | Edit | `prune-plan` in `PRIMITIVE_REGISTRY` |
| `runtime/src/primitives/prune_tasks.rs` | Edit | `all` walk; reset constant follows the tasks template; status reader and size shared with `prune-plan` |
| `runtime/src/primitives/prune_plan.rs` | Create | Segmentation, classification, finding key, digest-guarded removal, reopen trigger |
| `runtime/src/primitives/mod.rs` | Edit | `pub mod prune_plan;`, the promoted HEAD-blob reader, the shared block renderer |
| `runtime/src/primitives/check_stuck.rs` | Edit | Calls the promoted HEAD-blob reader |
| `runtime/src/primitives/analyze_subjects.rs` | Edit | Its hex encoder shared for the section digest |
| `runtime/src/interpreter/analyze_tally.rs` | Edit | The `plan-record` advisory in an exec analyze record |
| `runtime/src/schema/status.rs` | Edit | Names the tally as a consumer of the `planned`-onward set |
| `runtime/src/main.rs` | Edit | CLI subcommand for `prune-plan` |
| `runtime/src/mcp/server.rs` | Edit | `TOOL_NAMES` entry + `#[tool]` method |
| `runtime/src/interpreter/mod.rs` | Edit | `dispatch_primitive` arm |
| `runtime/tests/golden/analyze-basic.jsonl` | Re-bless | Analyze's new step renumbers the walk |
| `specs/058-findings-route-at-discovery/scenarios/*.md`, `specs/058-findings-route-at-discovery/plan.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`, `specs/022-deterministic-runtime/scenarios/exec-analyze-derives-its-list-seeds.md` | Edit | Analyze's steps 16–20 are 17–21 |
| `specs/058-findings-route-at-discovery/review.md` | Refresh | The digest the renumbering stales |
| `framework/runtime-tools.txt` | Edit | Manifest entry `prune-plan` |
| `framework/bootstrap/configure/*.md` | Regenerate | MCP allow-blocks (via `gen-configure-mcp.sh`) |
| `framework/constitution.md` | Edit | §plan-phase rule; §tasks-phase sentence; §implement-phase triage clause |
| `framework/templates/spec/plan.md` | Edit | Trade-offs and optional sections; what-stays-out comment |
| `framework/templates/spec/tasks.md` | Edit | Working-notes guidance |
| `framework/commands/prune.md` | Edit | Spec-directory scope, discards, reopen, `--all` |
| `framework/commands/consolidate.md` | Edit | 041's broadened title in its link |
| `framework/commands/analyze.md` | Edit | The `prune-plan` step and `plan-record` reference |
| `scripts/gen-help-tables.sh` | Edit | Prune's description |
| `framework/commands/help.md` | Regenerate | Help table |
| `.claude/commands/ductus/*.md` | Regenerate | Materialized commands |
| `docs/slash-commands.md` | Edit | Flag-versus-command rule; `/prune` entry |
| `docs/analyze.md` | Edit | The `plan-record` advisory |
| `README.md` | Edit | `/prune` line |
| `AGENTS.md` | Edit | Runtime-ownership routing matches practice |
| `runtime/CHANGELOG.md`, `runtime/Cargo.toml`, `version` | Edit | Release |

## Trade-offs

- **Section is the atomic unit; interiors are never edited.** Rejected
  stripping completed checkboxes out of still-pending sections: it would force
  renumbering and intra-section reference rewrites, dragging judgment into a
  mechanical primitive, and would discard the working context a half-done
  section's checked items provide. Cost: a long-lived section with many
  completed boxes and one straggler stays large until that box is checked.
  Accepted — that residue is small and self-clearing.
- **Primitive writes; preview never returns the body.** Rejected a
  preview-only primitive that hands the proposed content back for the agent to
  write: it round-trips the whole file through model context twice, defeating
  the runtime's token-reduction purpose. Cost: the primitive owns a write path
  (more surface than a pure computation) and the confirmation becomes a
  two-call preview→apply dance. Accepted — the summary-only contract is the
  point. The host does read a plan section it is judging, with its own file
  tools; judgment needs the text, and no write carries it.
- **Reset re-emits a canonical body constant.** Rejected reading the tasks
  template at runtime (adds a template-path dependency and a missing-template
  failure mode) and rejected preserving the working preamble verbatim (a
  normal filled file has already stripped the guidance comment, so it wouldn't
  satisfy "restores … heading plus guidance comment"). Cost: a constant that
  must track the template, guarded by a drift test. The design-record heading
  set takes the same trade.
- **Empty phase containers are dropped in keep-pending.** Rejected preserving
  them (leaves noisy empty `## Phase …` headings). Cost: a phase the user
  intends to refill loses its heading; re-planning re-adds it. Accepted —
  keep-pending targets a lean working set.
- **A sibling primitive, not a wider `prune-tasks`.** Rejected folding plan
  reduction into `prune-tasks`: its name, its byte-parity criterion (AC11) and
  its result shape are all about one artifact, and a plan reduction's apply is
  a list of host-chosen removals rather than a mode. Cost: a second pass through
  the seven registration sites, and a second schema. Accepted.
- **The advisory rides `prune-plan`, not a `check-artifacts` family.** A new
  family would have to be registered in 022's data model, the canonical
  registry of check families, which is a durable contract there and would
  reopen 022 for a full re-review of 105 scenarios and a 1,529-line data
  model; feature-specific primitives already live with their feature
  (`prune-tasks` here, 062's session primitives in 062). Cost: the advisory is
  documented in `framework/commands/analyze.md` and this spec's data model
  rather than beside the other families. Accepted, and `AGENTS.md`'s routing
  entry is corrected to match.
- **Removal is digest-guarded.** Rejected removing by heading alone: a
  section edited between preview and apply would lose content the host never
  judged. Cost: a stale preview refuses the apply, and the run re-previews.
- **Known limitations.** Prune classifies tasks by checkbox state alone, so a
  section left fully checked but not actually merged (a mis-checked box) is
  treated as spent, matching every other tasks primitive's trust of checkbox
  state. `--reset --force` on a live spec can strand it with no runnable tasks
  until `/{project}:plan` repopulates — the deliberate, explicit escape hatch,
  not silent behavior. The design-record detector reads `##` sections only, so
  a journal nested under a design heading is not seen; the rule still applies
  there, and no heading test reaches it.

## Cross-spec impact

- **`052-spec-supersession-and-consolidation`** — its *One-spec and two-spec
  commands* said `prune` writes one spec, which `--all` falsified as written.
  052 now draws the distinction `--all` needs: a batch flag repeating a
  one-spec operation for every spec is not a two-spec operation, so `prune`
  stays among the one-spec commands. A `> **Signpost:**` there links back to
  this spec.
