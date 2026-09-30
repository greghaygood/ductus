# 041 — Task Pruning Plan

Implements [041 — Task Pruning](spec.md).

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
  `malformed-tasks` and writes nothing. This reopen edits the tasks template's
  guidance comment (handoff notes, below), so the constant is updated in the
  same change — the drift test fails until it is.

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
own set only on the markdown-only path. The template gains Trade-offs and the
three optional sections, each optional one marked to be omitted when it does
not apply, so it names every section `/{project}:plan` fills
(`framework/commands/plan.md` §Create the plan step 2 fills Data Model and
Trade-offs, which the template omits today).

### `prune-plan` — fence-aware sections, digest-guarded removal

Segmentation feeds every line through `SkipScanner`
(`runtime/src/primitives/mod.rs`, `struct SkipScanner`), so a `##` inside a
fenced block or an HTML comment is not structure. Both occur: this
repository's 60 plans hold 6 `##` lines inside fenced blocks and 3 inside
HTML comments, across 4 plans (010, 026, 027, 058), and a scan that counted
them as sections would propose removing text that is not a section at all. A section is a level-2 heading from `parse_atx_heading` and
every line up to the next level-2 heading outside a skipped region; `###` and
deeper stay inside their parent. Lines before the first `##` are the preamble
and are always kept.

Each section outside the record is reported with its `heading`, its `ordinal`
(position among all `##` sections), `lines`, `bytes`, a sha256 `digest` of the
section's text, its `finding` (below), and `decided`. Preview returns only
this summary. Apply takes `remove: [{heading, digest}]` and removes exactly
those sections; if any listed section's current digest differs, or no section
carries that heading, the whole apply is refused as the domain outcome
`stale-sections` and nothing is written, because the host's moves were judged
against the text it read. Seams normalize as keep-pending's do, and the write
is `write_atomic`. `apply` defaults to `false`, as `prune-tasks`' does
(`PruneTasksArgs.apply`, `#[serde(default)]`), so a dispatch with no `apply`
argument is always a preview.

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

`prune-plan`'s apply result carries `status` and `reopen-required`.
`reopen-required` is true when any design-record section's text differs from
the same section in `plan.md` at HEAD, or when `tasks.md` holds more unchecked
checkboxes than it does at HEAD — the two diff-visible triggers the scenario
fixes. Both read HEAD through one shared reader, promoted from the private
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
result carries `examined`, a `features` list whose entries are the
single-feature summary plus `feature`, and `skipped` naming each feature
without the artifact (`no-tasks-file`, `no-plan-file`). `prune-tasks` with
`all` and `apply` writes every feature's reduction under one confirmation;
`prune-plan` with `all` is preview-only and refuses `apply`, because each
plan section is a per-spec judgment. This is the batch shape
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
each durable piece goes (a Technical Decisions entry, `AGENTS.md`, a pending
task) and what is dropped, and confirms the moves and the removal together,
naming the reopen the moves imply on a `done` spec. On confirmation the host
writes the moves, calls `prune-plan` apply with that section's heading and
digest, and — when the spec is `done` and `reopen-required` is true — calls
`set-status`. If `reopen-required` reports a reopen the confirmation did not
name (uncommitted design-record edits made before the run), the command asks
before flipping rather than reopening silently. A declined section is skipped
for that run, and the prompt names an analyze discard as the lasting keep.
The write surface is `tasks.md`, `plan.md`, `AGENTS.md` for moved knowledge,
and status through `set-status`; knowledge true for every project is named
for the operator to route and never written by prune. A missing `tasks.md`
stops the run with the "run `/{project}:plan`" directive only when there is
no plan either; with a plan, the plan half runs and the missing task list is
reported.

### `/{project}:analyze` reports every section outside the record

A new analyze step invokes `prune-plan` against the feature, with no `apply`,
on a spec at `planned` or later, and records **every** section it reports as
an advisory finding with the result's family and message — decided sections
included. Reporting only undecided sections would expire their stored
discards: `process-decisions` drops a decision whose key does not fire on an
unrestricted run (`runtime/src/primitives/process_decisions.rs`, the
`expired` outcome), after which prune would propose the section again. Firing
every section lets the stored-decision step match the decided ones, so they
count as discarded and nothing is asked. The exec walker binds a step's
arguments from its context (`runtime/src/interpreter/mod.rs`, the `call!`
macro), so the step dispatches with the feature and no `apply` — a preview.
Inserting the step renumbers analyze's later steps, which moves the
`analyze-basic` parity golden; it is re-blessed filtered to that golden and
its diff read line by line.

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
serve `prune-tasks`; a new `PlanFileMissing { root, feature }` is its sibling
for `prune-plan`. `FeatureNotFound` is reused for a missing feature dir;
`MissingSpecFile` / `StatusFieldMissing` when a status read fails; and
`MissingArgument` / `InvalidArgument` for the `feature`/`all` exclusivity,
`force` with `all`, and `apply` with `all` on `prune-plan`. A digest mismatch
is the `stale-sections` domain outcome, not an error. Each error writes
nothing.

### Runtime wiring (fully-wired primitives)

`prune-tasks` is wired through all seven registration sites so it is callable
from the CLI, MCP, and the `ductus exec` walker, and `prune-plan` takes the
same seven:

1. `schema/primitives.rs` — the Args / Result / section-record / enum types
   (kebab-case serde, `clap::Args` on the args struct) + a round-trip serde
   test per primitive.
2. `primitives/prune_tasks.rs`, `primitives/prune_plan.rs` — the `run`
   functions + inline `#[cfg(test)]`.
3. `primitives/mod.rs` — `pub mod` lines, the error variants, and the
   promoted HEAD-blob reader.
4. `main.rs` — args import, `Command` arm, dispatch.
5. `mcp/server.rs` — the name in `TOOL_NAMES` + a `#[tool]` async method.
6. `interpreter/mod.rs` — the `dispatch_primitive` arm.
7. `schema/registry.rs` — the `PRIMITIVE_REGISTRY` entry, which
   `parser/mod.rs`'s `PRIMITIVE_NAMES` aliases.

Then add `prune-plan` to `framework/runtime-tools.txt` and run
`scripts/gen-configure-mcp.sh` followed by `scripts/gen-claude-commands.sh`.
`runtime/tests/mcp.rs` holds the manifest set-equal to the registry, and
`main::tests::every_registry_primitive_has_a_clap_subcommand` pins the CLI.
Finish with a `runtime/CHANGELOG.md` `### Added` entry and a minor version
bump across the three version sites, released with a `ductus-v<version>` tag
once every affected spec is `done`.

### This spec's body

Implementing the scenarios corrects the body they supersede: the *Scope
confirmation* resolution, the Behavior section's "the command's scope is
`tasks.md` only", AC1, and the *Framework consistency* section, which gains
`plan.md`'s classification beside `tasks.md`'s. New criteria for the three
scenarios are added unlabelled and labelled by `label-criteria`. The title
broadens from *Task Pruning*; the directory slug stays.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `specs/041-task-pruning/data-model.md` | Edit | `prune-plan` segmentation, classification and schema; `all` on both primitives |
| `specs/041-task-pruning/spec.md` | Edit | Body corrections and new criteria |
| `runtime/src/schema/primitives.rs` | Edit | `PrunePlanArgs`/`PrunePlanResult` types; `all` on `PruneTasksArgs`; round-trip tests |
| `runtime/src/schema/registry.rs` | Edit | `prune-plan` in `PRIMITIVE_REGISTRY` |
| `runtime/src/primitives/prune_tasks.rs` | Edit | `all` walk; reset constant follows the tasks template |
| `runtime/src/primitives/prune_plan.rs` | Create | Segmentation, classification, finding key, digest-guarded removal, reopen trigger |
| `runtime/src/primitives/mod.rs` | Edit | `pub mod prune_plan;`, `PlanFileMissing`, the promoted HEAD-blob reader |
| `runtime/src/primitives/check_stuck.rs` | Edit | Calls the promoted HEAD-blob reader |
| `runtime/src/main.rs` | Edit | CLI subcommand for `prune-plan` |
| `runtime/src/mcp/server.rs` | Edit | `TOOL_NAMES` entry + `#[tool]` method |
| `runtime/src/interpreter/mod.rs` | Edit | `dispatch_primitive` arm |
| `runtime/tests/golden/analyze-basic.jsonl` | Re-bless | Analyze's new step renumbers the walk |
| `framework/runtime-tools.txt` | Edit | Manifest entry `prune-plan` |
| `framework/bootstrap/configure/*.md` | Regenerate | MCP allow-blocks (via `gen-configure-mcp.sh`) |
| `framework/constitution.md` | Edit | §plan-phase rule; §tasks-phase sentence; §implement-phase triage clause |
| `framework/templates/spec/plan.md` | Edit | Trade-offs and optional sections; what-stays-out comment |
| `framework/templates/spec/tasks.md` | Edit | Working-notes guidance |
| `framework/commands/prune.md` | Edit | Spec-directory scope, discards, reopen, `--all` |
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
  a list of host-chosen removals rather than a mode. Cost: a seventh
  registration set and a second schema. Accepted.
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
