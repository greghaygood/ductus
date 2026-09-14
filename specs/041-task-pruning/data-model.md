# 041 — Task Pruning Data Model

Defines the structures the `prune-tasks` primitive owns: the in-memory
**segmentation** it builds from a `tasks.md`, the per-section
**classification**, the two reduction **modes**, and the primitive's
request/response **schema**. The **serialized** types — `PruneTasksArgs`, `PruneMode`, `Classification`,
`PruneGate`, `PruneAction`, `SizeSummary` and `PruneSection` — live in
`runtime/src/schema/primitives.rs` with `serde::{Serialize, Deserialize}`
derives; their serialized JSON is the stable host contract, consistent with
the primitive-schema convention in
[022 — Deterministic Runtime](../022-deterministic-runtime/data-model.md).

The parsing reuses the existing `tasks.md` machinery in
`runtime/src/primitives/mod.rs` — `detect_tasks_structure`, `parse_atx_heading`,
`iter_phase_ranges`, and the `checkbox::find_checkbox_line` helper — so
`prune-tasks` recognizes exactly the same task set as `read-tasks` and
`mark-task`. Nothing about the grammar is re-invented.

## Segmentation

A `tasks.md` is segmented into one ordered `Vec<Block>`; the preamble is
simply the first `Structure` block, not a separate field. Structure detection is shared: `detect_tasks_structure` yields
`Flat` (task headings at level 2, `## N.`) or `Phased` (task headings at
level 3, `### N.`, under `## …` phase containers). `task_level` is 2 or 3
accordingly.

```rust
enum PruneMode { KeepPending, Reset }

/// Private to `primitives/prune_tasks.rs` — never serialized. A block owns
/// its own lines, so keep-pending rebuilds the file by concatenating the
/// survivors rather than by slicing ranges out of the original.
enum Kind {
    /// The preamble, the `# …` heading, and any other non-task heading
    /// group. Always kept.
    Structure,
    /// Phased files only: a `## …` non-numeric heading. Kept iff a task
    /// section within it survives.
    Phase,
    /// A numbered task section. Dropped when spent.
    Task,
}

struct Block {
    kind: Kind,
    lines: Vec<String>,            // the block's own lines, verbatim
    number: String,                // "1", "12", … (Task only)
    heading: String,               // title text, sans the `N.` prefix
    phase: Option<String>,         // containing phase heading (phased only)
    checkbox_total: u32,           // task-list checkboxes in the section
    checkbox_checked: u32,         // of which are `[x]`
    governing_phase: Option<usize>, // index of the phase container, if any
}

enum Classification {
    /// >= 1 checkbox and every checkbox is checked. Removable.
    Spent,
    /// >= 1 checkbox and at least one is unchecked. Always preserved.
    Pending,
    /// Zero checkboxes (prose/structural task). Always preserved — never
    /// classified spent.
    NoCheckbox,
}
```

A section's checkboxes are counted with `checkbox::find_checkbox_line`,
which already excludes `- **Done when**:` lines (they are not `[ ]`/`[x]`
markers). A `Task` block's lines run up to the next heading whose level is
`<= task_level` — identical to `mark-task`'s `locate_task_range`.

**Classification rule.**

| Checkboxes present | All checked | Classification | Removable |
| --- | --- | --- | --- |
| ≥ 1 | yes | `Spent` | yes |
| ≥ 1 | no | `Pending` | no |
| 0 | — | `NoCheckbox` | no |

## Reduction modes

### keep-pending (default)

Output, in document order:

For each `Block`, in order:

- `Structure` (the preamble, the `# …` heading, any other non-task heading
  group) → **kept verbatim**.
- `Task` classified `Spent` → **dropped**.
- `Task` classified `Pending` or `NoCheckbox` → **kept verbatim** (its own
  already-checked boxes included; prune never edits a section's interior).
- `Phase` → kept **iff at least one `Task` it governs survives**; otherwise
  dropped so no empty phase container lingers.

Seams between kept blocks are normalized to a single blank line and the file
ends with exactly one trailing newline, so the result is `markdownlint`-clean.

When no section is `Spent`, the computed output equals the input: the
primitive sets `nothing-to-prune: true` and writes nothing even under
`apply: true`.

### reset (`--reset`)

Output is the feature's identity plus a canonical empty task body:

```text
<existing first H1 line>

<CANONICAL_EMPTY_TASKS_BODY>
```

`<existing first H1 line>` is the file's first `# …` heading (preserves
feature identity, e.g. `# 041 — Task Pruning Tasks`).
`<CANONICAL_EMPTY_TASKS_BODY>` is a constant embedded in the primitive equal
to `framework/templates/spec/tasks.md` with its own H1 line removed (the
intro line + the guidance comment). A unit test asserts the constant equals
that template body so the two never drift. This satisfies the acceptance
criterion "restores `tasks.md` to the template's initial state (heading plus
guidance comment)" without coupling the primitive to a template path at
runtime.

If the file has no `# …` heading, reset cannot preserve identity: the
primitive errors (`malformed-tasks`) and writes nothing. When the computed
reset output already equals the file content, `nothing-to-prune: true` and no
write occurs (idempotent).

## Primitive request/response schema

### `prune-tasks` — reduce a feature's `tasks.md`

Args:

```json
{ "feature": "041-task-pruning", "reset": false, "force": false, "apply": false }
```

- `feature` — feature directory under the configured spec-root.
- `reset` — `false` = keep-pending; `true` = full reset.
- `force` — override the `reset` status gate on a non-`done` spec.
- `apply` — `false` = preview (compute + classify, **no write**); `true` =
  write the reduced file atomically.

Result (a **compact summary — never the file body**):

```json
{
  "mode": "keep-pending",
  "applied": false,
  "gate": "not-applicable",
  "nothing-to-prune": false,
  "removed-count": 3,
  "kept-count": 2,
  "size-before": { "lines": 412, "bytes": 18234 },
  "size-after":  { "lines": 180, "bytes": 7920 },
  "sections": [
    { "number": "1", "heading": "Wire crate", "phase": "Phase A — Bootstrap",
      "classification": "spent",   "checkbox-total": 4, "checkbox-checked": 4, "action": "removed" },
    { "number": "2", "heading": "Add CLI",    "phase": "Phase A — Bootstrap",
      "classification": "pending", "checkbox-total": 3, "checkbox-checked": 1, "action": "kept" }
  ],
  "path": "specs/041-task-pruning/tasks.md"
}
```

- `mode` — `"keep-pending"` or `"reset"`, echoing the resolved mode.
- `applied` — whether a write happened (`false` on preview, on
  `nothing-to-prune`, and on a blocked reset).
- `gate` — `"not-applicable"` for keep-pending; for reset, `"allowed"`
  (status is `done`, or `force` supplied) or `"blocked-needs-force"` (status
  is not `done` and `force` absent). A blocked reset is a **domain outcome**,
  not an operational error: the primitive returns `applied: false`,
  `gate: "blocked-needs-force"`, and writes nothing. The command surfaces the
  refusal (name the status, point at keep-pending, mention `--reset --force`).
- `status` — the spec's frontmatter status, read from `spec.md` only when
  `reset` is true. On keep-pending the field is `None` and is **omitted from
  the JSON entirely** (`skip_serializing_if`) rather than serialized as
  `null`, which is why the example above carries no `status` key — a host
  must treat it as absent, not as a present null.
- `sections` — one compact record per task section: its identity, its
  classification, checkbox counts, and the `action` taken
  (`"removed"` | `"kept"`). Bounded by the task count; the section **bodies
  are never included**, which is the token-reduction contract that motivates
  the primitive doing its own write (per the runtime-eligibility resolution
  in [spec.md](spec.md)).

### Operational errors

Reported as `error` envelopes (not result fields); the primitive writes
nothing when any fires:

| Code | Condition |
| --- | --- |
| `feature-not-found` | feature directory absent under the spec-root |
| `tasks-file-missing` | feature directory exists but has no `tasks.md` (command directs the user to `/{project}:plan`) |
| `malformed-tasks` | file has no `# …` heading (reset cannot preserve identity) |
| `missing-spec-file` / `status-field-missing` | `reset` requested but `spec.md` is absent or its frontmatter has no `status` |

## Notes

- All paths are repo-relative with `/` separators on every platform.
- The write uses the shared `write_atomic` (tempfile + rename); a crash
  mid-write leaves `tasks.md` unchanged, and no backup sidecar is produced
  (recovery is git history).
- `size-before`/`size-after` report both line and byte counts so the command
  can render a "materially smaller" summary without seeing the content.
