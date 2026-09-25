---
spec: 058-findings-route-at-discovery
last-run: 2026-09-25T21:51:08Z
reviewed-against: 9f48fe2b5d072b6b7e1f52dc7671dfd1f25848e2
diff-base: 5a1d410b75d6086bf34842d15cb83ca2a94ad024
must-violations: 0
should-violations: 4
low-confidence: 0
examined: 59
scope: 113
skipped-passes: []
reviewed-digest:
  data-model.md: d6d37afc89bb2896d14746b731b67ac61a36e938d06e8793dd835b6b010c5821
  scenarios/analysis-drift-judges-the-record-it-writes.md: 41a7eb07068616dd6f78df38034db7ce84d33ec6d8b3ef84a98d87fcb0706a99
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: dc4b6385ca5563e56830b22831e40a707dfc162780da351c4074b825184b3ae7
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 5ecc0bf8478bdff8c51a4ed3678ef6283824ce94f5438be1f4a1306bb0d85422
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: 2a6f546f7758f7bd227f4cd0eca3206820dc75181d3268ed760a9338d62680d7
blocking: false
dispositions:
  fixed: 0
  routed: 18
  discarded: 5
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
  - key: "bug: libgit2's buffer blame misplaces a deletion that follows an insertion, so the inbox age can read a removed item's date — `runtime/src/primitives/inbox_standing.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: a shallow clone reports its boundary commit's date as the inbox's oldest, though the docs say undeterminable — `runtime/src/primitives/inbox_standing.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: a project in a subdirectory of its git repo always reads inbox age undeterminable, because the blame path is not workdir-relative (predates 058) — `runtime/src/primitives/inbox_standing.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: invalidate-review no longer nulls captured-issues, which 64 pre-058 records still carry — `runtime/src/primitives/invalidate_review.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "simplicity: the performReview response reuses ReviewObservation, so exec validates a disposition it then strips, and a malformed one fails the run — `runtime/src/schema/extensions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "simplicity: dead code (unused Default on AnalysisFinding, AnalysisTier, InboxStanding, InboxState; unreachable branches in needs_quote, process-decisions, and to_ref) — `runtime/src/schema/primitives.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: a stored decision is compared three ways (flattened key in process-decisions, raw key in merge, full ref in the in-run conflict check) — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: stale runtime docs (restricted and fired, AppendTaskResult.appended, append-inbox dedup in primitives/mod.rs, write-analysis Errors, yaml_string, render_extra_field, decisions::render, dispatch_blocking) — `runtime/src/schema/primitives.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "design: write-analysis refuses a prior analysis.md whose frontmatter is missing or unparseable, so re-running analyze no longer repairs a damaged record — `runtime/src/primitives/write_analysis.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "test: a decision-key naming no stored decision and an all-empty decision entry are untested; two inbox and dedup-title tests cannot fail on the behavior they name — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "design: analyze step 17 and the constitution reopen only another done spec, so a task routed onto the done spec in hand never reopens it — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: 058's data model and plan are stale (disposition-drift per record, restricted on any unexamined target, never-pruned entries, the new-decision test, inbox_standing.rs doc-only) — `specs/058-findings-route-at-discovery/data-model.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: 022's append-inbox entry still says bullet scanning dedups — `specs/022-deterministic-runtime/data-model.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: docs/analyze.md restates the excluded-by-construction class list and omits the unresolved-constitution restriction — `docs/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: the brownfield audit promises a discard that analyze refuses for a blocking security gap — `framework/bootstrap/ductus-procedure.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "convention: the markdown-only empty-scope step skips dispositioning observations — `framework/commands/review.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "design: analyze-state-drift judges the prior analysis.md, so on a done spec it never clears within a run and only --fix ends it — `runtime/src/primitives/check_artifacts.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: exec analyze fails at validate-frontmatter on a real session, whose path is the spec directory (predates 058) — `runtime/src/interpreter/mod.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "other: the promotion-coverage notice lists 3 unclassified AGENTS.md entries — `AGENTS.md`"
    outcome: discarded
    reason: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

058's second review. 0 MUST, 4 SHOULD, 0 low-confidence, not blocking; the four SHOULDs hold `done` until fixed. No waivers. 23 observations: 18 routed, 5 discarded, none undispositioned.

**The SHOULDs.** QUAL-CLAIM-001: `ductus exec analyze` never binds the tier counts, so its record reads 0/0/0 and fully examined whatever detection found. Reproduced with two live blocking findings, it makes AC26's exec clause false; task 41. QUAL-GROUND-001, three times: U+FFFE and U+FFFF pass `is_line_hazard` and are written raw, after which the reader refuses the whole record; tab-then-`#` is written unquoted and read back truncated, so the decision expires the run after it is made; and the inbox age blames raw working-tree bytes, so a committed CRLF inbox under `core.autocrlf` always reads age undeterminable, a regression from task 32. Each was reproduced; task 39.

**The first review's routes landed.** Its 30 routed decisions expired, each re-checked fixed, and its SHOULD (control characters in a decision) no longer fires. Its four discards were re-observed and matched to their stored decisions.

**Routed.** Task 39: the three QUAL-GROUND-001 fixes; the inbox age's libgit2 deletion misplacement, shallow-clone date, and subdirectory path (the last predates 058); `invalidate-review` keeping `captured-issues`; the `performReview` response type; dead defaults and unreachable branches; one equality rule for a stored decision; stale runtime docs; `write-analysis` refusing a damaged record; missing tests. Task 40: analyze step 17's reopen scope, 058's stale data model and plan, 022's `append-inbox` dedup wording, `docs/analyze.md`'s restated class list, the brownfield audit's discard promise, and the markdown-only empty-scope step. Task 41: the exec tally and the session-path failure (predates 058). A new scenario, `analyze-state-drift-judges-the-record-it-writes`, with task 42: analyze-state drift loops on a done spec exactly as disposition drift did. Discarded: the promotion-coverage notice.

**Scope.** `diff-base` 5a1d410b, 113 in scope, examined **59**. Four parallel reviewers read, in full or at every 058 hunk: 22 runtime source modules, 5 test files, and 3 goldens; the constitution, 8 command files, both configure permission lists, the adoption-audit procedure, the migration and its registry entry, the inbox template, and `runtime-tools.txt`; `docs/analyze.md`, `docs/slash-commands.md`, `README.md`, `AGENTS.md`, and `specs/inbox.md`; and 058's spec, data model, plan, prior review, and four newest scenarios. The host re-checked every claim recorded here against the code, most by reproduction. Not read: the six discharged specs' files (022's were reviewed today under its own spec), the `.claude` mirrors (confirmed byte-identical to their sources after substitution), the plan table's three pattern rows, 058's `tasks.md` beyond tasks 32–38, and `version` (unchanged).

**Passes.** Security: no trigger in the backend rule files fires on this code. Quality produced the four SHOULDs and the bug observations; reuse, efficiency, and simplicity produced the rest.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — exec analyze records a clean, fully examined run whatever detection found

- **File**: `runtime/src/interpreter/mod.rs:694-704`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: Nothing in the exec walker binds write-analysis's tier counts or unexamined, so they default to zero and undispositioned is 0 − 0. Reproduced: a planned spec with no plan.md or tasks.md gave two blocking check-artifacts findings, and exec wrote blocking-findings: 0, blocking: false, undispositioned: 0, and 'every target was examined'. The zeros predate 058, but AC26's exec clause, plan.md, and this binding's comment assert that exec counts every live finding undispositioned.
- **Auto-fixable**: no
- **Suggested fix**: Tally each analyze step's results into the tier counts at dispatch, by the tiering the step states, and its skipped targets into unexamined-by-reason, and bind them to write-analysis. Routed to 058 task 41.

### SHOULD: QUAL-GROUND-001 — U+FFFE and U+FFFF pass is_line_hazard and leave a decision record unreadable

- **File**: `runtime/src/primitives/mod.rs:1234-1236`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract in a way that fails loudly when the assumption is wrong (a typed or generated binding, a schema/migration reference, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption.
- **Finding**: is_line_hazard covers is_control() plus U+2028/U+2029, but the YAML reader also refuses the noncharacters U+FFFE and U+FFFF. Both pass validate_single_line and flatten_line and are written raw by yaml_string; the reader then refuses the whole record ('control characters are not allowed', reproduced), so every writer and the gate refuse it until it is repaired by hand.
- **Auto-fixable**: yes
- **Suggested fix**: Add U+FFFE and U+FFFF to is_line_hazard, and test that a decision carrying each is refused or re-reads cleanly. Routed to 058 task 39.

### SHOULD: QUAL-GROUND-001 — a decision value carrying tab-then-# is written unquoted and read back truncated

- **File**: `runtime/src/primitives/write_review.rs:990-1003`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract in a way that fails loudly when the assumption is wrong (a typed or generated binding, a schema/migration reference, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption.
- **Finding**: needs_quote treats only ' #' as a comment start, but YAML also opens a comment at tab-then-#, and reparses_as_nonstring checks the parsed type rather than the value. Reproduced: a stored key 'grounding — plan.md cites issue<TAB>#12 as closed' reads back as 'grounding — plan.md cites issue' and is expired by the next run's process-decisions; a discard reason loses its tail the same way.
- **Auto-fixable**: yes
- **Suggested fix**: Quote any value whose unquoted form does not re-parse to the identical string, replacing the ' #' test and the type-only check; test tab-then-#. Routed to 058 task 39.

### SHOULD: QUAL-GROUND-001 — a committed CRLF inbox under core.autocrlf always reads age undeterminable

- **File**: `runtime/src/primitives/inbox_standing.rs:97-100`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract in a way that fails loudly when the assumption is wrong (a typed or generated binding, a schema/migration reference, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption.
- **Finding**: blame_buffer is handed the raw working-tree bytes, assuming they equal the committed blob. Under core.autocrlf or eol=crlf the working file is CRLF and the blob LF, so every line diffs as uncommitted and the oldest date is lost. Reproduced on a clean, fully committed inbox; the reviewer confirmed that a build from before task 32 dates it.
- **Auto-fixable**: no
- **Suggested fix**: Date each working-tree bullet by its text against HEAD's blame, with a trailing \r trimmed. Routed to 058 task 39.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: libgit2's buffer blame misplaces a deletion that follows an insertion, so the inbox age can read a removed item's date — `runtime/src/primitives/inbox_standing.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: a shallow clone reports its boundary commit's date as the inbox's oldest, though the docs say undeterminable — `runtime/src/primitives/inbox_standing.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: a project in a subdirectory of its git repo always reads inbox age undeterminable, because the blame path is not workdir-relative (predates 058) — `runtime/src/primitives/inbox_standing.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: invalidate-review no longer nulls captured-issues, which 64 pre-058 records still carry — `runtime/src/primitives/invalidate_review.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- simplicity: the performReview response reuses ReviewObservation, so exec validates a disposition it then strips, and a malformed one fails the run — `runtime/src/schema/extensions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- simplicity: dead code (unused Default on AnalysisFinding, AnalysisTier, InboxStanding, InboxState; unreachable branches in needs_quote, process-decisions, and to_ref) — `runtime/src/schema/primitives.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: a stored decision is compared three ways (flattened key in process-decisions, raw key in merge, full ref in the in-run conflict check) — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: stale runtime docs (restricted and fired, AppendTaskResult.appended, append-inbox dedup in primitives/mod.rs, write-analysis Errors, yaml_string, render_extra_field, decisions::render, dispatch_blocking) — `runtime/src/schema/primitives.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- design: write-analysis refuses a prior analysis.md whose frontmatter is missing or unparseable, so re-running analyze no longer repairs a damaged record — `runtime/src/primitives/write_analysis.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- test: a decision-key naming no stored decision and an all-empty decision entry are untested; two inbox and dedup-title tests cannot fail on the behavior they name — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- design: analyze step 17 and the constitution reopen only another done spec, so a task routed onto the done spec in hand never reopens it — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 058's data model and plan are stale (disposition-drift per record, restricted on any unexamined target, never-pruned entries, the new-decision test, inbox_standing.rs doc-only) — `specs/058-findings-route-at-discovery/data-model.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 022's append-inbox entry still says bullet scanning dedups — `specs/022-deterministic-runtime/data-model.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: docs/analyze.md restates the excluded-by-construction class list and omits the unresolved-constitution restriction — `docs/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: the brownfield audit promises a discard that analyze refuses for a blocking security gap — `framework/bootstrap/ductus-procedure.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: the markdown-only empty-scope step skips dispositioning observations — `framework/commands/review.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- design: analyze-state-drift judges the prior analysis.md, so on a done spec it never clears within a run and only --fix ends it — `runtime/src/primitives/check_artifacts.rs` — **routed** to `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`
- bug: exec analyze fails at validate-frontmatter on a real session, whose path is the spec directory (predates 058) — `runtime/src/interpreter/mod.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- other: the promotion-coverage notice lists 3 unclassified AGENTS.md entries — `AGENTS.md` — **discarded**: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
- perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision is pruned when its key expires or is decided again — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it

## Skipped passes

*None.*

## Unexamined governance

*None.*
