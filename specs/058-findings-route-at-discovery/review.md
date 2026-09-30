---
spec: 058-findings-route-at-discovery
last-run: 2026-09-30T02:37:41Z
reviewed-against: 4ec75a72efd4650e4cf4af9f7fbfa2d37b4e19e8
diff-base: 4073733d2b1214971b02f2a870d15a8b15fc83b0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 37
skipped-passes: []
reviewed-digest:
  data-model.md: eb5f2099c81634ab8f9dc5c7ff3073d2f6323dccf95c39a1cf1ec410d3b9b237
  scenarios/analysis-drift-judges-the-record-it-writes.md: 33f0948bc3e3b1de7f92abbd822ebb1de03ad08f2aa24c1509236ff7bd0baf7a
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: d14bea60a4dfdba81edc5180b77367629154699b6808fda0739ab8a902776660
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: e975a37075b739bcd087c8683f166b6955ce9bcc42c28f64e79bf9f49003f803
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: 567f083c76eb4a23906c3c1ca95620b3799d5d5d766bc0df5ecc49facccc35eb
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

Deliberate partial re-review of 058 after 041's analyze-step renumber (4ec75a72), which rewrote analyze step numbers in four of 058's scenarios and one line of its plan. examined: 4 of 37, and that is the honest number. This is a digest refresh for that sweep, not a full pass over 058. The previous record (examined 24 of 51, at ff571435) remains the last full five-pass review.

BASE. The natural base 446e4ced resolves 147 paths, 128 of them modified since, almost all 062's and 041's work, which has nothing to do with 058. The change under re-review is exactly the sweep commit, so the base is overridden to its parent, 4073733d, which resolves 37: the 29 plan-affected entries plus the 8 files the sweep touched.

READ IN FULL, all five passes: specs/058-findings-route-at-discovery/scenarios/analysis-drift-judges-the-record-it-writes.md; specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md; specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md; specs/058-findings-route-at-discovery/scenarios/only-unreadable-targets-retain-decisions.md. Each step citation was checked against framework/commands/analyze.md as renumbered: 17 processes stored decisions and sets restricted, 18 fixes and routes, 19 re-checks, 20 writes the record, 21 renders. Every citation names the step it meant before the renumber. The sweep changed the numbers and nothing else, so no claim moved.

READ AT THE CHANGED LINE ONLY, not counted: specs/058-findings-route-at-discovery/plan.md (line 274); specs/022-deterministic-runtime/scenarios/exec-analyze-derives-its-list-seeds.md (line 9); specs/060-exec-analyze-assesses-each-loaded-rule/spec.md (line 33); specs/060-exec-analyze-assesses-each-loaded-rule/plan.md (line 70); framework/commands/analyze.md (read in full while making the renumber in 4073733d, but it sits in scope only through a plan-affected glob, and it is 041's change rather than 058's).

NOT EXAMINED, named individually: .claude/commands/ductus/*.md; AGENTS.md; docs/analyze.md; framework/bootstrap/ductus-procedure.md; framework/commands/{review,implement,amend,groom,log,help,status}.md; framework/constitution.md; framework/migrations.toml; framework/runtime-tools.txt; framework/templates/project/inbox.md; runtime/src/interpreter/analyze_tally.rs; runtime/src/primitives/append_task.rs; runtime/src/primitives/check_artifacts.rs; runtime/src/primitives/check_review_gate.rs; runtime/src/primitives/compute_review_scope.rs; runtime/src/primitives/dashboard.rs; runtime/src/primitives/diff_cross_spec.rs; runtime/src/primitives/inbox_standing.rs; runtime/src/primitives/invalidate_review.rs; runtime/src/primitives/mod.rs; runtime/src/primitives/process_decisions.rs; runtime/src/primitives/validate_frontmatter.rs; runtime/src/primitives/write_analysis.rs; runtime/src/primitives/write_review.rs; runtime/src/schema/extensions.rs; runtime/src/schema/primitives.rs; runtime/src/schema/registry.rs; runtime/tests/mcp.rs; specs/{008,020,022,047,050,054,057}-*/; version. None of these changed in the window this record covers.

STORED DECISIONS. The nine discard decisions are retained unchanged: their sources are runtime files this pass did not read, so none was evaluated and none can expire. This run raised no observation, so its dispositions map counts nothing.

WHAT THE PASSES COVERED. The subject is markdown prose, so security, efficiency and simplicity have nothing to act on. Reuse: the scenarios cite the procedure's steps rather than restating them. Quality: each numbered claim matches the procedure it cites (QUAL-CLAIM-001). 0 MUST, 0 SHOULD, 0 low-confidence; not blocking.

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
