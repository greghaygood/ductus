---
spec: 058-findings-route-at-discovery
last-run: 2026-09-26T19:06:56Z
reviewed-against: dad2bd4f0fc7bec0e6076dbbb628cc07ed46f73a
diff-base: e52df370246768f81a50854c433c368c3dc17340
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 36
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
waivers:
  - rule: QUAL-CLAIM-001
    file: runtime/src/interpreter/analyze_tally.rs
    reason: the unasked rules trace to exec analyze's single-rule request (022, ef1686eb); the fix is parked in draft spec 060-exec-analyze-assesses-each-loaded-rule by operator decision
    waived-at: 2026-09-26T19:06:56Z
    waived-by: andy@stone.dev
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

058's fifth review, over what changed since the fourth (`--since=e52df370`): task 48 (`907d643c`) and the plan's release-scope fix (`dad2bd4f`). 0 MUST, 0 SHOULD outstanding, 0 low-confidence, not blocking. One SHOULD waived. 11 observations: 1 routed, 10 discarded (all 10 matched to stored decisions), none undispositioned.

**The fourth review's work landed.** Its routed decision expired. With no rule file read, `AnalyzeTally::assessed` classes the request `Nothing`, and `record_assessment` records it under `rule-assessments-not-checked` and counts no tier. `analyze_with_no_rule_directory_records_its_citations_unexamined` asserts `blocking-findings: 0`, `advisory: 0` and `blocking: false` beside `rule-assessments-not-checked: 2`. The new reason is classified *could not be read* in `analyze.md`, 022's data model, and `AnalyzeBlock`'s doc, the only places the set is enumerated. `assessed` keys on the Verification rather than the id, which matches `resolve_assessed_rule`'s placeholder: a cited id whose Verification does not resolve. The eight `analyze_tally` tests, the walker test, and the parity test pass against this tree. The fourth review's 10 discards were re-observed and matched: the file each is anchored to is byte-unchanged since `e52df370`, and the promotion-coverage notice still lists 5 entries.

**Waived.** `QUAL-CLAIM-001` on `analyze_tally.rs`. Steps 11 and 12 assess every loaded rule of their tier, but the exec walker asks the host about one rule per step, and the record names none of the rest unexamined. The single-rule request is 022's (`ef1686eb`); 058's task 41 began counting its verdicts. Waived by operator decision, with the fix parked in draft spec `060-exec-analyze-assesses-each-loaded-rule`.

**Routed.** Both steps ask about the same rule under each step's tier: the golden's `req-1` and `req-2` both carry `CFG-CONST-001`. An exec run over `analyze-basic` recorded `blocking-findings: 1` and `advisory: 1` for that one rule. Routed by operator decision to 060, created in this run through `/ductus:specify`'s procedure.

**Scope.** `diff-base` e52df370; 36 in scope (9 modified since, 27 plan-only); examined **8**. One reviewer read every hunk since the fourth review in `analyze_tally.rs` (read in full), `interpreter/mod.rs`, `schema/primitives.rs`, `tests/parity.rs`, `framework/commands/analyze.md`, 022's `data-model.md`, and 058's `plan.md` and `tasks.md`, and read `payload.rs`'s request builder for context. **Not read:** the `.claude` analyze mirror, which matches its source byte for byte after placeholder substitution; and the 27 plan-affected entries unchanged since the fourth review, three of them glob strings rather than paths.

**Passes.** Security: no loaded backend rule fires on this change. Quality produced the waived SHOULD and the routed observation. Reuse, efficiency, and simplicity found nothing new in the change.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: QUAL-CLAIM-001 — exec analyze records steps 11 and 12 as examined when each asked about one loaded rule

- **File**: `runtime/src/interpreter/analyze_tally.rs:181-194`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: Steps 11 and 12 assess every loaded rule of their tier whose Verification fires. The exec walker asks the host about one rule per step (`resolve_assessed_rule`), and `record_assessment` records nothing for the rest, so `unexamined-by-reason` reads as though every rule was examined. This repository loads 192 rules with a Verification.
- **Auto-fixable**: no
- **Suggested fix**: Record the loaded rules a step did not ask about under a could-not-be-read reason, or ask the host about each rule; draft spec 060 carries both options.
- **Waived**: the unasked rules trace to exec analyze's single-rule request (022, ef1686eb); the fix is parked in draft spec 060-exec-analyze-assesses-each-loaded-rule by operator decision

## Observations

- perf: validate-frontmatter still reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision still goes when its key expires or is re-decided — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is still dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain still strips bullet markers itself — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it
- other: the promotion-coverage notice still lists 5 unclassified AGENTS.md entries — `AGENTS.md` — **discarded**: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
- simplicity: render_list_entry still takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **discarded**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix still fall back on a validated companion — `runtime/src/primitives/process_decisions.rs` — **discarded**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- perf: check-artifacts' history revwalk still runs on the async worker (predates 058) — `runtime/src/mcp/server.rs` — **discarded**: predates 058 (ccdd3ac6), not introduced by it
- reuse: append_task.rs still parses a task heading two ways (predates 058) — `runtime/src/primitives/append_task.rs` — **discarded**: predates 058, not introduced by it
- convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks still name neither frontmatter variant under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs` — **discarded**: predates 058; a doc-only omission, and each variant's own message names the file and the defect
- bug: exec analyze's steps 11 and 12 ask the host about the same rule, each under its step's tier, so one rule's verdict counts twice (the request shape predates 058; task 41 began counting it) — `runtime/src/interpreter/payload.rs` — **routed** to `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
