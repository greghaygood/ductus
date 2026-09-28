---
spec: 058-findings-route-at-discovery
last-run: 2026-09-28T15:13:59Z
reviewed-against: ff571435da2b3866ebf888b3bcbace320bfec6b9
diff-base: 446e4ced1c605a98a8ca784ed2ceb697e65059f0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 24
scope: 51
skipped-passes: []
reviewed-digest:
  data-model.md: eb5f2099c81634ab8f9dc5c7ff3073d2f6323dccf95c39a1cf1ec410d3b9b237
  scenarios/analysis-drift-judges-the-record-it-writes.md: e8d3c238c85ebff667fe7ce0e3cafd91022dc739f4ff3cb987ef1f663013577e
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: dc4b6385ca5563e56830b22831e40a707dfc162780da351c4074b825184b3ae7
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: c5c0bdca5d562145e041e80058782eeff480466fca4f5bba9097c589236774a3
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: ce363ad101864dcff01440b28b899e453793a0a7e3c7b25611fd0818eec62ec5
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 9
  undispositioned: 0
decisions:
  - key: "perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs`"
    outcome: discarded
    reason: bounded to two small files per spec; the decisions list is read apart from the record by design
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: a duplicate-key decision is pruned when its key expires or is decided again — `runtime/src/primitives/decisions.rs`"
    outcome: discarded
    reason: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: an all-empty decision entry is dropped on re-render — `runtime/src/primitives/decisions.rs`"
    outcome: discarded
    reason: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs`"
    outcome: discarded
    reason: "predates 058: moved from render_captured_plain, not introduced by it"
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: predates 058 on render_waivers_at; no behavior depends on it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs`"
    outcome: discarded
    reason: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "perf: check-artifacts' history revwalk runs on the async worker, contrary to server.rs's module doc (predates 058) — `runtime/src/mcp/server.rs`"
    outcome: discarded
    reason: predates 058 (ccdd3ac6), not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs`"
    outcome: discarded
    reason: predates 058, not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs`"
    outcome: discarded
    reason: predates 058; a doc-only omission, and each variant's own message names the file and the defect
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

Unrestricted re-review of 058, run to judge its stored decisions against their sources. The previous scoped pass (b273bb10) retained all 11 without reading the files they concern. Diff base is 446e4ced, the parent of 058's last reopen, so the window also carries the 016, 036 and 050 cycles that followed it. 0 MUST, 0 SHOULD, 0 low-confidence; 9 observations, all matched to stored discard decisions; not blocking.

Each stored decision was judged against the code as it stands, because every source file except prune_tasks.rs changed after its decision. Nine still hold and re-fire under their stored discards, each read at its site:

- validate_frontmatter.rs:265-290 loads each record, then read_decisions reads the same file again.
- decisions.rs:330-339 prunes every entry for an expired or re-decided key, duplicates included.
- write_review.rs:955-976 renders nothing for an all-empty entry.
- write_analysis.rs:529-537 strips the bullet marker itself rather than calling mod.rs:1539.
- Every render_list_entry caller passes an empty base (write_review.rs:880, decisions.rs:370).
- process_decisions.rs:113-124 and decisions.rs:216-229 fall back to an empty companion.
- check_artifacts.rs:423 walks history while server.rs:855-862 runs it inline, contrary to its module doc at lines 9-13.
- append_task.rs parses a heading by find('.') at 166-172 and by split_numbered_heading at 245.
- The # Errors docs of check_review_gate.rs, check_unfolded_specs.rs, dashboard.rs and prune_tasks.rs name neither variant split_frontmatter returns (mod.rs:817-818).

Two stored decisions expired because their findings no longer fire. The promotion-coverage notice now reports 0 unclassified after 050's third round (was 3). The routed exec-analyze decision's work landed with 060 (payload.rs:880-894 asks about each loaded rule once, in its own tier).

Examined 24 of the scope. The window's changed files were read in full as diffs: AGENTS.md, framework/commands/analyze.md, framework/constitution.md, framework/rules/quality-cross.md, framework/templates/spec/spec.md, 016's scenario and tasks.md, 036's data-model.md, spec.md and tasks.md, 050's plan.md and tasks.md, and 058's spec.md and tasks.md. These runtime files were read at the regions above: append_task.rs, check_artifacts.rs, check_review_gate.rs, dashboard.rs, mod.rs, process_decisions.rs, validate_frontmatter.rs, write_analysis.rs and write_review.rs. analyze_tally.rs was read at lines 1-247, all of its production code. Not opened:

- the generated .claude/commands/ductus/ mirrors (gen-claude-commands --check in sync);
- the plan-affected files unchanged in this window;
- the review and analysis records of 016, 036, 050 and 058, which write-review and write-analysis wrote this session;
- specs/inbox.md, whose only change is an item the operator logged during this session and has not committed.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision is pruned when its key expires or is decided again — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it
- simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **discarded**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs` — **discarded**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- perf: check-artifacts' history revwalk runs on the async worker, contrary to server.rs's module doc (predates 058) — `runtime/src/mcp/server.rs` — **discarded**: predates 058 (ccdd3ac6), not introduced by it
- reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs` — **discarded**: predates 058, not introduced by it
- convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs` — **discarded**: predates 058; a doc-only omission, and each variant's own message names the file and the defect

## Skipped passes

*None.*

## Unexamined governance

*None.*
