---
spec: 058-findings-route-at-discovery
last-run: 2026-09-28T14:50:29Z
reviewed-against: b273bb103050ef77129484919cf6167b3960fa99
diff-base: 446e4ced1c605a98a8ca784ed2ceb697e65059f0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 38
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
  discarded: 0
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
  - key: "other: the promotion-coverage notice lists 3 unclassified AGENTS.md entries — `AGENTS.md`"
    outcome: discarded
    reason: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
    decided-at: 2026-09-25T21:51:08Z
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
  - key: "bug: exec analyze's steps 11 and 12 ask the host about the same rule, each under its step's tier, so one rule's verdict counts twice (the request shape predates 058; task 41 began counting it) — `runtime/src/interpreter/payload.rs`"
    outcome: routed
    target: specs/060-exec-analyze-assesses-each-loaded-rule/spec.md
    decided-at: 2026-09-26T19:06:56Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

Scoped re-review of 058's reopen for the analyze record's fixed/routed rule (task 49, groomed from the inbox), against diff base 446e4ced, the parent of the reopen commit. The window also carries 016's reopen, so both changes are in scope. 0 MUST, 0 SHOULD, 0 low-confidence, no observations; not blocking.

Examined 11 of 38. Read in full as diffs over the window: framework/commands/analyze.md, framework/constitution.md, framework/templates/spec/spec.md, specs/058-findings-route-at-discovery/spec.md and tasks.md, specs/016-cross-cutting-rules/spec.md, tasks.md and scenarios/applicable-rules-consistency-check.md, and specs/inbox.md. runtime/src/primitives/write_analysis.rs was read at lines 260-360 and 840-880, where the rule the prose now states lives (line 294 refuses any outcome but fixed for a finding gone from the re-check; live decisions only at 346-351). runtime/src/interpreter/analyze_tally.rs was read at lines 1-247, all of its production code (249-546 is the test module), to judge the recorded QUAL-CLAIM-001 waiver: every not-examined state is now recorded under its own reason since 060 landed (9dcace0e), so the rule no longer fires there and the waiver expired. Not opened: .claude/commands/ductus/analyze.md, generated from framework/commands/analyze.md, which was read, with gen-claude-commands --check reported in sync; and the 26 plan-affected files unchanged in this window, reviewed at 058's prior review (dad2bd4f).

The 11 stored decisions are retained untouched (process-decisions restricted), because this scoped pass did not read the files they concern. That includes the routed one: payload.rs:880-894 shows 060 asks about each loaded rule once, in its own tier, so its routed work has landed, and the next unrestricted review prunes it.

Local gate after the change: framework-checks steps, shellcheck, parseability and scripts/audit/run-all.sh exit 0; cargo fmt, clippy -D warnings and cargo test --release --locked exit 0 (1713 passed, 20 result lines for 20 targets).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
