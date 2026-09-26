---
spec: 058-findings-route-at-discovery
last-run: 2026-09-26T13:34:14Z
reviewed-against: 21514ed5bb31d27ad890670fccd1cdde24869f07
diff-base: 5a1d410b75d6086bf34842d15cb83ca2a94ad024
must-violations: 0
should-violations: 2
low-confidence: 0
examined: 67
scope: 128
skipped-passes: []
reviewed-digest:
  data-model.md: ea7df679ce300ccbc3e49f8f827f346a2c36b6cf77927a1ebc71704bef9225bd
  scenarios/analysis-drift-judges-the-record-it-writes.md: 41a7eb07068616dd6f78df38034db7ce84d33ec6d8b3ef84a98d87fcb0706a99
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: dc4b6385ca5563e56830b22831e40a707dfc162780da351c4074b825184b3ae7
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 5ecc0bf8478bdff8c51a4ed3678ef6283824ce94f5438be1f4a1306bb0d85422
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: 2a6f546f7758f7bd227f4cd0eca3206820dc75181d3268ed760a9338d62680d7
blocking: false
dispositions:
  fixed: 0
  routed: 21
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
  - key: "other: the promotion-coverage notice lists 3 unclassified AGENTS.md entries — `AGENTS.md`"
    outcome: discarded
    reason: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
    decided-at: 2026-09-25T21:51:08Z
    decided-by: andy@stone.dev
  - key: "bug: remove-inbox-item refuses U+2028, U+0085, and ESC text that append-inbox accepts, so that bullet cannot be removed — `runtime/src/primitives/mod.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: process-decisions refuses a frontmatter-less analysis.md that write-analysis treats as empty and overwrites — `runtime/src/primitives/process_decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: an unclosed prior analysis.md is refused as frontmatter missing, and no test pins the refusal — `runtime/src/primitives/write_analysis.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: two observations sharing a key, one routed and one undispositioned, are accepted and counted under both — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: the exec tally drops an assessment whose severity is capitalized, and a failed assessment carrying no finding — `runtime/src/interpreter/analyze_tally.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "test: no test covers the walker feeding assessSpecQuality responses into the tally — `runtime/src/interpreter/mod.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: dedup-title merges two out-of-spec findings sharing a summary, because the path lives in the task body — `runtime/src/primitives/append_task.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: repeated inbox texts pair front-first while remove-inbox-item removes the first, so the survivor takes the removed bullet's date — `runtime/src/primitives/inbox_standing.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: compute-review-scope returns an empty scope with no guidance for a project in a subdirectory of its repo (predates 058) — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "design: check-orphaned-references' skipped referrers never reach unexamined-by-reason — `runtime/src/interpreter/analyze_tally.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "reuse: the exec analyze binding and writeCode's excerpts resolve the constitution differently — `runtime/src/interpreter/payload.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "simplicity: PassObservation's unused Default and Eq, a re-parse of validated observations, a hand-copied status list, and a companion check written twice — `runtime/src/schema/extensions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: stale runtime docs (Errors sections, malformed-entry claims, pass_observations placement, mcp.rs three-fields doc, already_done_block's split doc, check_artifacts' mid-list paragraph, analyze_subjects' readers, AppendTaskResult.task_number) — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
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
  - key: "convention: review.md says malformed and duplicate decisions are never pruned, though the runtime drops an all-empty entry and a duplicate with its key; its waiver passage is stale the same way — `framework/commands/review.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: 058's plan is stale (the invalidate_review row, six reopened specs for seven, check_analyze_drift, write-review at step 9) — `specs/058-findings-route-at-discovery/plan.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: 058's data-model line citations are stale, and spec.md's edge case names analyze's dedup key — `specs/058-findings-route-at-discovery/data-model.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "bug: two 058 scenarios say exec reports drift; exec records it and reports nothing — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "other: four places call Family 37's set the exact population the CI template exempts; it is a subset — `framework/commands/audit.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: analyze.md's markdown-only list of checks omits analyze state drift, and docs/analyze.md carries its own reason-class table — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "other: status.md says a shallow clone makes the inbox age undeterminable, which holds only when every bullet is behind the cut — `framework/commands/status.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: docs/slash-commands.md and README.md's /review entries do not describe observation dispositions (AC19) — `docs/slash-commands.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

058's third review. 0 MUST, 2 SHOULD, 0 low-confidence, not blocking. The two SHOULDs hold `done` until fixed. No waivers. 30 observations: 21 routed, 9 discarded (5 matched to stored decisions), none undispositioned.

**The SHOULDs.** Both are QUAL-CLAIM-001, both reproduced, both routed to task 44.

- A shallow clone whose inbox has bullets on both sides of its cut reports the oldest date after the cut as the queue's oldest: 2022 for a queue the full clone dates 2020. Task 39 fixed only the case where every bullet is behind the cut.
- `ductus exec analyze` still records "every target was examined". It never runs steps 13–15, and a `check-rule-ids` that read no rule file drops its `missing` list: with `rule-files = []`, a cited unknown rule went uncounted (2 blocking became 1) under `unexamined: 0`.

**The second review's work landed.** Its four SHOULDs no longer fire. Its 18 routed decisions expired, each re-checked as fixed; two had residuals, recorded here as new items (the shallow-clone SHOULD, and the plan row for `invalidate_review.rs`). Its five discards were re-observed and matched to their stored decisions.

**Routed.**

- Task 44 (runtime):
  - both SHOULDs;
  - `remove-inbox-item` refusing text `append-inbox` accepts;
  - `process-decisions` refusing a frontmatter-less `analysis.md` that the writer overwrites, and the unclosed-frontmatter refusal (misnamed and untested);
  - same-key observations with routed and undispositioned outcomes counted under both;
  - capitalized assessment severities dropped from the exec tally, with no test of that path;
  - `dedup-title` merging distinct findings that share a summary;
  - repeated inbox texts misdated after a removal;
  - `compute-review-scope` in a subdirectory project (predates 058);
  - orphan-check skips missing from `unexamined-by-reason`;
  - two constitution lookups;
  - four cleanups and eight doc corrections.
- Task 45 (prose):
  - `review.md`'s never-pruned claim;
  - 058's stale plan, data-model, and spec edge case;
  - two scenarios saying exec reports drift;
  - Family 37's "exact population" wording;
  - `analyze.md`'s check list and `docs/analyze.md`'s second class table;
  - `status.md`'s shallow-clone wording;
  - the `/review` docs that AC19 requires.

**Discarded.** Four items: `render_list_entry`'s always-empty `base`; `describe`'s fallback on a validated companion, since `DecisionRef` is a wire type; `check-artifacts`' revwalk on the async worker (predates 058); and `append_task.rs`'s two heading grammars (predates 058).

**Scope.** `diff-base` 5a1d410b; 128 in scope; examined **67**. Four parallel reviewers each read their files in full or at every 058 hunk:

- 29 runtime source and test files, and 3 goldens;
- the constitution, 9 command sources, both configure lists, the adoption-audit procedure, the migration and its registry, the inbox template, and `runtime-tools.txt`;
- `docs/analyze.md`, `docs/slash-commands.md`, `README.md`, `AGENTS.md`, and the Family 37 script and README;
- 058's spec, data model, plan, prior review, and five newest scenarios;
- 3 `.claude` mirrors, confirmed identical to their sources after substitution.

The host re-checked every recorded claim, the bugs by reproduction. **Not read:** the discharged specs' files (022, 047, and 054 were reviewed under their own specs this session), the other `.claude` mirrors, 058's `tasks.md` beyond tasks 39–45, `specs/inbox.md`, and `version` (unchanged).

**Passes.** Security: no trigger in the backend rule files fires on this code. Quality produced both SHOULDs and the bugs; reuse, efficiency, and simplicity produced the rest.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — a shallow clone reports the oldest date after its cut as the inbox's oldest when older bullets sit behind it

- **File**: `runtime/src/primitives/inbox_standing.rs:110-153`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: Boundary hunks are skipped (`if shallow && hunk.is_boundary() { continue; }`), and the `filter_map`/`min` then drops the bullets they held, so whenever one bullet lands after the cut the row names a confident date that understates the queue's age. Reproduced: origin bullets from 2020 and 2022, `git clone --depth 2`, and `dashboard` reports `oldest 2022-01-01`; the full clone reports 2020-01-01. The function's doc, `InboxStanding.oldest`, and 022's data model say a shallow clone is undeterminable.
- **Auto-fixable**: no
- **Suggested fix**: Return `None` when any surviving bullet maps to a line behind a shallow cut, and test a mixed-depth shallow clone. Routed to 058 task 44.

### SHOULD: QUAL-CLAIM-001 — exec analyze records a fully examined run when detection steps never ran or no rule file was read

- **File**: `runtime/src/interpreter/analyze_tally.rs:71-140`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: The tally binds only `check-artifacts`' skipped targets to `unexamined-by-reason`. Steps 13–15 (cross-service references, Applicable Rules citations, grounding) are host prose the walker never runs, and a `check-rule-ids` that read zero rule files has its `missing` list dropped. Reproduced on `analyze-basic` with an unknown `CFG-CONST-999` cited: with the rule file seeded, 2 blocking; with `rule-files = []`, 1 blocking. Both records read `unexamined: 0` and 'every target was examined'. The module doc says this tally removes that conflation.
- **Auto-fixable**: no
- **Suggested fix**: Record an unexamined reason for each detection step the walker cannot run and for a `check-rule-ids` that examined no rule file while citations exist, each classified in `analyze.md`'s Unexamined targets. Routed to 058 task 44.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- perf: validate-frontmatter still reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision still goes when its key expires or is re-decided — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is still dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain still strips bullet markers itself — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it
- other: the promotion-coverage notice still lists 3 unclassified AGENTS.md entries — `AGENTS.md` — **discarded**: a notice by design, never a gate; 050 makes classification rounds deliberate rather than standing
- bug: remove-inbox-item refuses U+2028, U+0085, and ESC text that append-inbox accepts, so that bullet cannot be removed — `runtime/src/primitives/mod.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: process-decisions refuses a frontmatter-less analysis.md that write-analysis treats as empty and overwrites — `runtime/src/primitives/process_decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: an unclosed prior analysis.md is refused as frontmatter missing, and no test pins the refusal — `runtime/src/primitives/write_analysis.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: two observations sharing a key, one routed and one undispositioned, are accepted and counted under both — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: the exec tally drops an assessment whose severity is capitalized, and a failed assessment carrying no finding — `runtime/src/interpreter/analyze_tally.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- test: no test covers the walker feeding assessSpecQuality responses into the tally — `runtime/src/interpreter/mod.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: dedup-title merges two out-of-spec findings sharing a summary, because the path lives in the task body — `runtime/src/primitives/append_task.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: repeated inbox texts pair front-first while remove-inbox-item removes the first, so the survivor takes the removed bullet's date — `runtime/src/primitives/inbox_standing.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: compute-review-scope returns an empty scope with no guidance for a project in a subdirectory of its repo (predates 058) — `runtime/src/primitives/compute_review_scope.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- design: check-orphaned-references' skipped referrers never reach unexamined-by-reason — `runtime/src/interpreter/analyze_tally.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- reuse: the exec analyze binding and writeCode's excerpts resolve the constitution differently — `runtime/src/interpreter/payload.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- simplicity: PassObservation's unused Default and Eq, a re-parse of validated observations, a hand-copied status list, and a companion check written twice — `runtime/src/schema/extensions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: stale runtime docs (Errors sections, malformed-entry claims, pass_observations placement, mcp.rs three-fields doc, already_done_block's split doc, check_artifacts' mid-list paragraph, analyze_subjects' readers, AppendTaskResult.task_number) — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **discarded**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs` — **discarded**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- perf: check-artifacts' history revwalk runs on the async worker, contrary to server.rs's module doc (predates 058) — `runtime/src/mcp/server.rs` — **discarded**: predates 058 (ccdd3ac6), not introduced by it
- reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs` — **discarded**: predates 058, not introduced by it
- convention: review.md says malformed and duplicate decisions are never pruned, though the runtime drops an all-empty entry and a duplicate with its key; its waiver passage is stale the same way — `framework/commands/review.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 058's plan is stale (the invalidate_review row, six reopened specs for seven, check_analyze_drift, write-review at step 9) — `specs/058-findings-route-at-discovery/plan.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 058's data-model line citations are stale, and spec.md's edge case names analyze's dedup key — `specs/058-findings-route-at-discovery/data-model.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: two 058 scenarios say exec reports drift; exec records it and reports nothing — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- other: four places call Family 37's set the exact population the CI template exempts; it is a subset — `framework/commands/audit.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: analyze.md's markdown-only list of checks omits analyze state drift, and docs/analyze.md carries its own reason-class table — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- other: status.md says a shallow clone makes the inbox age undeterminable, which holds only when every bullet is behind the cut — `framework/commands/status.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: docs/slash-commands.md and README.md's /review entries do not describe observation dispositions (AC19) — `docs/slash-commands.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
