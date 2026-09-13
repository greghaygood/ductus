---
spec: 052-spec-supersession-and-consolidation
reviewed-at: 2026-09-13T14:34:46Z
reviewed-against: 8c01d6f58ba65d6218e43301e8a384237fa49c3b
diff-base: 3f239479e2474111cd55991939343d350960cfd6
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 9
skipped-passes: []
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Re-review for the `examined`/`scope` backfill — 052's records predate `ductus-v0.49.0`, so they carried no `examined`, no `scope`, and neither digest.

**Scope, and what was actually read.** Both diff bases were measured before choosing. Before this pass reopened the spec, the natural base was 052's own `in-progress` parent (`2818a378`), resolving **321** files against a plan that affects 7 — the case AGENTS.md §Gotchas says the `--since HEAD` override exists for, and which resolved to exactly those 7. Reopening the spec made the override unnecessary: the reopen commit records a fresh `in-progress` transition, so the natural base moved forward to `3f239479` and the window is now **9** files — the 7 the plan affects plus the 2 this pass edited. That is the base recorded here; no override was passed.

All 9 were read in full, so `examined` is 9 of 9 and **no in-scope file went unread**. Every in-scope path exists in the tree; none names a file that has since been removed. The two large ones were read in ranges rather than sampled — `framework/bootstrap/ductus.md` (1182 lines) and `runtime/src/schema/primitives.rs` (5484 lines) — the latter in full because the plan lists it and a partial read would make this number a guess.

**Passes.** Rule files loaded through `discover-rule-files` (8 files, backend surface). The five dimensions ran against the two `runtime/*.rs` files as code and the seven markdown surfaces as contracts. Zero MUST, zero SHOULD. `retire-feature` orders its two refusals correctly — traversal validation, then the gated sequential refusal before anything touches the filesystem, then the ungated anti-stranding check — and `retired: false` is a named domain outcome rather than a bare success, which is `QUAL-CLAIM-001` satisfied by construction. Its tests pin both halves of the gate, including that opting in does not relax the anti-stranding guard and that fold's call shape is unchanged. No silent stubs (`QUAL-STUB-001`): every incomplete path errors or returns a named outcome. No unowned external contracts (`QUAL-GROUND-001`): the paths and formats are the project's own.

**Criteria.** All eleven verified against the tree, enumerations included. AC34 was the one that had gone false: it named the README as stating the one-spec/two-spec split, which `8bc18b95` put there and `82ff5066` later moved to `docs/slash-commands.md:47`, where it still names `fold` and `consolidate` as the two-spec pair. Only the location moved, so it was swept rather than annotated and the criterion stays true. AC25's two halves both hold — the manifest row at `ductus.md:886` and the only-command-that-removes-a-durable-artifact sentence at `consolidate.md:12`. AC39's discharge is 051's post-completion signpost at `051/spec.md:59`, correctly written as a blockquote so no dependency edge forms.

**Decisions read as live claims.** The plan's three §Trade-offs still hold: the refusal is gated rather than split in two, consolidation is a command rather than a `--into` flag on fold, and the known limitation that nothing verifies the target's coverage is still stated where an operator meets it. Both Resolved Questions hold — consolidation still compares nothing (`consolidate.md:30`), and the command is still adopter-facing (`scripts/maintainer-only-commands.txt` lists only `audit`). The scenario's first Edge Case had gone false and is corrected in the commit this review runs against: it offered clearing the session as "a third option neither command currently takes", which 052's own implementation commit adopted for consolidate.

**Observations.** Three documentation defects in the two runtime files, none mapping to a loaded rule and none blocking. All three are `runtime/` edits, which the release entry makes a version bump and a `ductus-v<version>` tag — beyond this session's one-spec scope, so they are captured to the inbox rather than fixed here.

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

- convention: `retire-feature`'s `# Errors` doc on `run` still states that `InvalidArgument` fires "when `feature` is not the branch-scoped form" unconditionally. Spec 052's `allow-sequential` gate made that conditional — the module doc above documents the gate at length, but the function's own `# Errors` block, which is what a caller reads, still describes pre-052 behaviour. One clause. Fixing it is a `runtime/` edit, so it carries the version bump and `ductus-v<version>` tag the release entry requires (captured during /ductus:review of 052) — `runtime/src/primitives/retire_feature.rs:46`
- convention: `ArtifactFinding::family`'s doc comment enumerates eight `check-artifacts` families and then ends mid-sentence on a dangling "or" with nothing after it. The primitive owns nine; `analyze-state-drift` is the omission, verified against the `family: "..."` literals in check_artifacts.rs. This is the same eight-for-nine defect `ductus-v0.49.1` fixed in the `check-artifacts` MCP tool description, at a second site that fix missed. A `runtime/` edit, so it carries the version bump and tag (captured during /ductus:review of 052) — `runtime/src/schema/primitives.rs:3412`
- convention: `CheckArtifactsResult::findings`'s doc says findings come "across the eight families" and its parenthetical lists eight, omitting `analyze-state-drift`; the primitive owns nine. Sibling of the `ArtifactFinding::family` omission and repaired in the same edit — an enumeration is a claim of its own, which is the drift class AGENTS.md §Gotchas records. A `runtime/` edit, so it carries the version bump and tag (captured during /ductus:review of 052) — `runtime/src/schema/primitives.rs:3457`

## Skipped passes

*None.*

## Unexamined governance

*None.*
