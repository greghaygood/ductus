---
spec: 041-task-pruning
diff-base: 8de657817c5ac0eabc675011e4923da7fce1aa90
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T12:40:49Z
reviewed-against: 5538c5c99b769a1ac2c381166d1af4465c8ef648
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 22
reviewed-digest:
  data-model.md: 6fd2f2ac88690e649f39d60fba2c936f844b6699d2eade755547cd966bed7393
blocking: false
---

# Review — 041-task-pruning

## Summary

Backfill pass over a review record written before `ductus-v0.49.0`: no `examined`, no `scope`, no `reviewed-digest`, so freshness read as undeterminable and a run that skipped its passes would have been byte-identical to one that did not. All five passes ran against the resolved scope. No MUST, SHOULD or low-confidence findings.

**Scope and base.** Both measured after this pass's first correction commit and recorded before choosing. The natural base `8de65781` resolves 6 modified-since / 22 in scope against a plan affecting 20; `--since HEAD` gave 0 / 20 at the moment the base was chosen. The modified-since figure was 5 when the base was selected and is 6 as recorded here, because a later correction in this same pass added a sixth file — the count is evidence about the commit this review names, not about the selection. The natural base was taken because it covers the files this pass edited, which HEAD excludes by construction. The pre-reopen natural base `5ec4b7c3` resolved **805 modified-since / 805 in scope at 154,617 bytes** — over the MCP output cap, so that leg was run through the CLI and read with `jq`. The reopen collapsed it to 1,579 bytes, so the deciding leg returned inline: the friction is a property of the pre-reopen window only, which is now the fifth pass running to find that.

**examined 8 of 22.** Read in full: this spec's `spec.md`, `plan.md` and `data-model.md`; `framework/commands/prune.md`; `framework/runtime-tools.txt`; `runtime/legacy-prose-commands.txt`; `runtime/Cargo.toml`; and `runtime/src/primitives/prune_tasks.rs`, all 631 lines including its 11 inline tests. Named rather than counted, each with what was relied on instead: `.claude/commands/ductus/prune.md` is a generated mirror of a source read in full and the generator re-ran in this pass reporting all 16 command copies in sync; five runtime sources — `interpreter/mod.rs`, `main.rs`, `mcp/server.rs`, `parser/mod.rs`, `primitives/mod.rs` — were read only at their `prune-tasks` registration sites; `schema/primitives.rs` only at the seven prune types; `runtime/tests/mcp.rs` at its module contract and function inventory; `README.md` at the `/prune` row, whose anchor into `docs/slash-commands.md` was resolved against the real heading; `framework/commands/help.md` and `scripts/gen-help-tables.sh` at their prune entries; the two `framework/bootstrap/configure/` files at their allow-blocks plus the generator that writes them; and `runtime/CHANGELOG.md` was not opened at all.

**Five corrections, all committed ahead of this review so the digest covers them.** The largest is a criterion the shipped command source contradicts: AC11 claimed the markdown-only fallback "reaches identical bytes", unqualified, while `prune.md`'s own reset bullet records that the reset body is compiled into the primitive and pinned to `framework/templates/spec/tasks.md`, so a project that has customized its own tasks template diverges. That case is reachable — the **Shared Files** manifest ships that template to adopters at `specs/templates/tasks.md` — and the same `prune.md` section promises byte-for-byte parity two paragraphs above the bullet that denies it. The behaviour is deliberate and unchanged: the plan weighed reading the template at runtime and rejected it, and an adopter tree holds no `framework/` copy for the markdown-only host to read, so the claim was the defect. Corrected in AC11, in the Runtime-eligibility resolved question, and in `prune.md`.

The other four are enumeration and shape drift. `data-model.md` depicted a `Segmentation` struct and a `LineRange` that have **zero** definitions in the tree, and a `Block` enum where the realized type is a private struct carrying a `kind` discriminant with a third variant the depiction had no room for — under an opening claim that "the types live in `runtime/src/schema/primitives.rs`", true of the seven serialized types and false of the two it had just drawn. It also showed the keep-pending result as `"status": null` where `skip_serializing_if` omits the key entirely, in a document that calls its serialized JSON the stable host contract. Both documents named `iter_phase_ranges` among the helpers the primitive reuses; it appears zero times there and `append_task.rs` is its only consumer. The plan's Affected Files listed an `mcp.rs` edit that never landed. And `plan.md` still spelled `gov:init`, the pre-049 namespace.

**Verified by probe rather than by reading**, on scratch features moved to `/tmp` afterwards rather than deleted: `read-tasks` returns zero tasks and `append-task` numbers from 1 against a template-state file whose guidance comment embeds three `## N.` example headings; `--reset` on an `in-progress` spec returns `blocked-needs-force` and writes nothing, `--force` flips it to `allowed`; reset restores the template's 31 lines; keep-pending took 335 to 247 bytes, dropped the spent section, kept the half-done one with its checked box and kept a checkbox-free section; a second run reports `nothing-to-prune` and does not write; the pruned output lints clean and leaves no sidecar; and all three documented error paths write nothing.

**Checked before filing, and correctly not a finding:** `prune-tasks` is absent from `configure/antigravity.md` and `configure/opencode.md`. Reading the generator rather than the output shows `gen-configure-mcp.sh` writes those layouts a single `mcp(ductus/*)` wildcard, so no per-tool line exists there by design.

Security, reuse, efficiency and simplicity found nothing to report. `validate_no_traversal` guards the feature name before any path is joined; every error path returns before a write; the write is the shared atomic tempfile-and-rename and the source file's own line ending is preserved, so pruning a CRLF checkout does not silently convert it. The primitive hand-rolls no grammar — it reuses `detect_tasks_structure`, `parse_atx_heading`, `split_numbered_heading`, `SkipScanner` and `checkbox::find_checkbox_line`, which is what keeps its task set identical to `read-tasks` and `mark-task`. The spec status is read only when `reset` is set, and the section records carry identity and counts but never a line of the file, which is the token-reduction contract AC11 exists for.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
