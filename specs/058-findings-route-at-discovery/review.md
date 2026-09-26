---
spec: 058-findings-route-at-discovery
last-run: 2026-09-26T18:25:59Z
reviewed-against: 226234c818191f13d54353891a73fdcddd5d4f21
diff-base: a523457ca91e1a5ea17b9c98a81634fc90f35de6
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 54
scope: 87
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
  routed: 1
  discarded: 10
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
  - key: "bug: with no rule file read, exec analyze assesses an empty rule at steps 11–12 and counts the host's verdict, so the record reads blocking beside rule-citations-not-checked, and a parity test pins it — `runtime/src/interpreter/analyze_tally.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
  - key: "convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs`"
    outcome: discarded
    reason: predates 058; a doc-only omission, and each variant's own message names the file and the defect
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

058's fourth review, over what changed since the third (`--since=a523457c`). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. 11 observations: 1 routed, 10 discarded (9 matched to stored decisions), none undispositioned.

**The third review's work landed.** Its two SHOULDs no longer fire. Its 21 routed decisions expired, each re-checked against the tree. Its 9 discards were re-observed and matched to their stored decisions. The promotion-coverage notice now lists 5 unclassified entries, not 3: the same issue, grown by "No dead code", the test-validity rule, and the mutation-testing gotcha.

**The five items committed in `113820e5` without host re-verification hold.** Each was checked by reading and by single-point mutations in the dev profile, restored afterwards: 10 mutations over 17 tests, and every targeted test failed.

- `UnclosedFrontmatter` is named apart from a missing block, and a closing fence ending the file is accepted. 14 primitives' `# Errors` docs name it.
- `read_recorded_list` reads a frontmatter-less record as empty, for every reader.
- `compute-review-scope`'s `modified-since` is project-relative and drops changes outside the project.
- Same-key findings with one outcome are each counted and stored once; any two outcomes for one key are refused.
- `remove-inbox-item` applies `append-inbox`'s newline-only rule.

**Routed.** Task 48. With no rule file read, exec analyze sends steps 11–12 an `assessSpecQuality` request for an empty rule and counts the host's verdict. Reproduced on `analyze-basic` without `framework/rules`: `blocking: true` beside `rule-citations-not-checked: 1`, and a parity test asserts that count. The tally is 058's (tasks 41 and 44).

**Discarded.** Four primitives whose `# Errors` docs name neither frontmatter variant (predates 058).

**Scope.** `diff-base` a523457c; 87 in scope (77 modified since, 10 plan-only); examined **54**. One reviewer read every 058 hunk of 33 runtime source and test files and 21 prose files: the command sources, docs, audit script and README, `AGENTS.md`, 022's and 058's changed artifacts, and 059's draft. **Not read:** the hunks of 051's `fde2e38f` and `5cf68693` in the 16 files only they touched, which 051's own review covers; the 7 `.claude` mirrors, which regenerate from their sources with no diff; and the 10 plan-affected entries unchanged since the third review, three of them glob strings rather than paths.

**Passes.** Security: no loaded backend rule fires on this code. Quality produced both new observations. Reuse, efficiency, and simplicity re-observed the stored discards and found nothing new.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- perf: validate-frontmatter still reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision still goes when its key expires or is re-decided — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is still dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain still strips bullet markers itself — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it
- other: the promotion-coverage notice now lists 5 unclassified AGENTS.md entries — `AGENTS.md` — **discarded**: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
- simplicity: render_list_entry still takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **discarded**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix still fall back on a validated companion — `runtime/src/primitives/process_decisions.rs` — **discarded**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- perf: check-artifacts' history revwalk still runs on the async worker (predates 058) — `runtime/src/mcp/server.rs` — **discarded**: predates 058 (ccdd3ac6), not introduced by it
- reuse: append_task.rs still parses a task heading two ways (predates 058) — `runtime/src/primitives/append_task.rs` — **discarded**: predates 058, not introduced by it
- bug: with no rule file read, exec analyze assesses an empty rule at steps 11–12 and counts the host's verdict, so the record reads blocking beside rule-citations-not-checked, and a parity test pins it — `runtime/src/interpreter/analyze_tally.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs` — **discarded**: predates 058; a doc-only omission, and each variant's own message names the file and the defect

## Skipped passes

*None.*

## Unexamined governance

*None.*
