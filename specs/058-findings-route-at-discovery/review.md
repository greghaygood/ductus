---
spec: 058-findings-route-at-discovery
last-run: 2026-09-25T20:05:21Z
reviewed-against: 7c96a52df937598beb403ad10e472f9ebda18921
diff-base: 5a1d410b75d6086bf34842d15cb83ca2a94ad024
must-violations: 0
should-violations: 1
low-confidence: 0
examined: 51
scope: 108
skipped-passes: []
reviewed-digest:
  data-model.md: 5251a99cb5c27d3c2cdd97e82208ba86db9eb3fc1abca6f5c5a2253ba9719608
  scenarios/analysis-drift-judges-the-record-it-writes.md: 6a6b72c34a125d727e815fcf12175dee726bf1612d77398384e843a9ece50afb
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: dc4b6385ca5563e56830b22831e40a707dfc162780da351c4074b825184b3ae7
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: 2a6f546f7758f7bd227f4cd0eca3206820dc75181d3268ed760a9338d62680d7
blocking: false
dispositions:
  fixed: 0
  routed: 30
  discarded: 4
  undispositioned: 0
decisions:
  - key: "bug: exec analyze binds check-orphaned-references' findings into write-analysis and fails — `runtime/src/interpreter/mod.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: exec records a disposition the performReview response supplies, with no gate-confirm — `runtime/src/interpreter/mod.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: decisions::merge stores duplicate entries for same-key findings — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: decisions::merge re-stamps a re-matched decision whose companion text differs; the data model matches on key and outcome — `runtime/src/primitives/decisions.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: a blank decision-key or run timestamp is stored as an entry the reader calls malformed — `runtime/src/primitives/write_review.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: AnalysisFinding.tier defaults to advisory, so an untagged blocking finding can be stored discarded — `runtime/src/schema/primitives.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: write-analysis flattens newlines in stored keys, so a key can miss the host's fired key — `runtime/src/primitives/write_analysis.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: dedup-title hand-rolls the heading and checkbox grammars and never matches a title ending in # — `runtime/src/primitives/append_task.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: invalidate-review keeps a stale dispositions map beside last-run: null — `runtime/src/primitives/invalidate_review.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: the inbox age pairs working-tree bullet lines with HEAD's blame — `runtime/src/primitives/inbox_standing.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "bug: the no-file inbox row says the file is missing when it exists but cannot be read — `runtime/src/primitives/dashboard.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard runs git blame on the async MCP worker instead of dispatch_blocking — `runtime/src/mcp/server.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "simplicity: dead or inert code (Dispositions::total, --decided-by CLI flags, process-decisions' fired flag and dead branch, show_untracked_content) — `runtime/src/schema/primitives.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: stale runtime docs (append-inbox CLI help names dedup-by-prefix; a dashboard doc comment sits on the wrong item) — `runtime/src/main.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "reuse: write-review and write-analysis duplicate the dispositions block, the disposition rendering, and the decision conversion — `runtime/src/primitives/write_analysis.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "test: no MCP test for process-decisions and no round-trip tests for the new schema types — `runtime/tests/mcp.rs`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: 058's data model says MCP and the CLI ignore dedup-prefix; they refuse it — `specs/058-findings-route-at-discovery/data-model.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: 022's data model lacks process-decisions and disposition-drift, and says dedup-prefix is ignored — `specs/022-deterministic-runtime/data-model.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: the /analyze section of docs/slash-commands.md and README's Analyze bullet describe the pre-058 contract — `docs/slash-commands.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: an AGENTS.md corollary still says pipeline passes grow the inbox — `AGENTS.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: review.md's Blocking semantics misstates the gate order — `framework/commands/review.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: implement.md reopens a routed spec unconditionally and forbids reading the spec a disposition routes to — `framework/commands/implement.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: analyze.md's unparseable-decisions wording contradicts the spec, and its markdown-only frontmatter section omits the decisions hard fail — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: the constitution's chore paragraph and exec sentence contradict the commands — `framework/constitution.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: groom's tree ends a finding in the inbox and lacks the unpinned-rule-file discard — `framework/commands/groom.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: status.md's markdown-only path gives no inbox-age fallback without git — `framework/commands/status.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/tasks.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "design: disposition drift on a done spec's own analysis.md never converges on a re-run — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/scenarios/analysis-drift-judges-the-record-it-writes.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "design: run-level restricted keeps analyze decisions forever on specs whose unexamined count is never zero — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/scenarios/only-unreadable-targets-retain-decisions.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "design: analyze keys use host-worded messages, so those findings are asked about again every run — `framework/commands/analyze.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "design: --auto pauses on every disposition task although the walk says record and keep working — `framework/commands/implement.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/scenarios/auto-records-disposition-tasks-without-pausing.md
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
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
---

# Review — 058-findings-route-at-discovery

## Summary

058's own review. 0 MUST, 1 SHOULD, 0 low-confidence, not blocking. No waivers. 34 observations: 30 routed, 4 discarded, none undispositioned.

**The SHOULD (QUAL-GROUND-001).** A control character or YAML line break (ESC, U+2028) in a stored decision's key, target, or reason is written unquoted, and the YAML reader then refuses the whole record, reproduced as 'control characters are not allowed'. Every writer and the gate refuse until the file is repaired by hand. It is outstanding in this record, and 058 task 32 fixes it.

**Routed to new 058 tasks.** Task 32 covers correctness defects in 058's runtime, the most serious that `ductus exec analyze` fails whenever `check-orphaned-references` has a finding, because `write-analysis` binds that step's `findings` from the walker context. The others: exec honors a `performReview`-supplied disposition without a gate; `merge` stores same-key duplicates and matches on companion text rather than key and outcome; a blank key or timestamp is stored malformed; `tier` defaults to advisory; the writers' newline policies diverge; `dedup-title` hand-rolls its grammar; `invalidate-review` keeps a stale map; the inbox age misaligns working-tree lines with HEAD's blame; the no-file row mislabels an unreadable file; and `dashboard` blames on the async worker. Task 33 covers dead code, stale runtime docs, duplication, and missing tests. Task 34 covers stale prose, including 022's data model, which lacks `process-decisions` and `disposition-drift`, and 058's and 022's claim that MCP and the CLI ignore `dedup-prefix` (they refuse it).

**Routed to new 058 scenarios**, each with a task, the design chosen by the operator: `analysis-drift-judges-the-record-it-writes` (drift on a done spec's own analysis.md never converged); `only-unreadable-targets-retain-decisions` (run-level `restricted` kept decisions forever on 20 of 55 specs); `analyze-findings-match-decisions-by-host-judgment` (host-worded messages re-asked every run); `auto-records-disposition-tasks-without-pausing`.

**Scope.** `diff-base` 5a1d410b, 108 in scope, examined **51**. Four parallel reviewers read the full 058 diff of: 16 primitive modules; the schema, registry, MCP server, CLI, and interpreter; 5 test files and 3 goldens; `runtime-tools.txt` and the two configure permission lists; the constitution; 8 command files; the adoption-audit procedure; the migration and its registry entry; the inbox template and this repo's inbox; `docs/analyze.md`, `docs/slash-commands.md`, `README.md`, and `AGENTS.md`. Each claim reported here was re-checked against the code, and the SHOULD and the exec binding were confirmed by reproduction or by the walker's merge rule. The 57 unread are the six discharged specs' files (each reviewed under its own spec), the 10 generated `.claude` mirrors (regeneration reports them in sync), 058's own spec artifacts, two plan-table rows, and `version` (unchanged).

**Passes.** Security: `security-backend.md`'s triggers are spec and plan commitments, and none fires on this code; the YAML-injection hazards are filed under quality. Reuse, quality, efficiency, and simplicity produced the observations above.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-GROUND-001 — a control character or YAML line break in a decision value is written unquoted and leaves the record unreadable

- **File**: `runtime/src/primitives/decisions.rs:202-210`
- **Rule**: Code whose correctness depends on an external contract it does not own — a database schema, another service's API shape, a config key, a file or wire format — SHOULD bind to that contract in a way that fails loudly when the assumption is wrong (a typed or generated binding, a schema/migration reference, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption.
- **Finding**: decisions::render writes key, target, and reason through write_review::yaml_string, which quotes only for its needs_quote hazards and a non-string re-parse; the writers screen those values only for \n and \r. An ESC in a reason is emitted plain, and the YAML reader then refuses the whole record ('control characters are not allowed', reproduced), so every writer and the gate refuse until the file is repaired by hand. serde_json-based quoting would not escape U+2028 either.
- **Auto-fixable**: no
- **Suggested fix**: Reject C0/C1 controls and U+0085/U+2028/U+2029 in validate_single_line, flatten them in write_analysis's single_line, and add a test that stores and re-reads a decision carrying them. Routed to 058 task 32.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: exec analyze binds check-orphaned-references' findings into write-analysis and fails — `runtime/src/interpreter/mod.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: exec records a disposition the performReview response supplies, with no gate-confirm — `runtime/src/interpreter/mod.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: decisions::merge stores duplicate entries for same-key findings — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: decisions::merge re-stamps a re-matched decision whose companion text differs; the data model matches on key and outcome — `runtime/src/primitives/decisions.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: a blank decision-key or run timestamp is stored as an entry the reader calls malformed — `runtime/src/primitives/write_review.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: AnalysisFinding.tier defaults to advisory, so an untagged blocking finding can be stored discarded — `runtime/src/schema/primitives.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: write-analysis flattens newlines in stored keys, so a key can miss the host's fired key — `runtime/src/primitives/write_analysis.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: dedup-title hand-rolls the heading and checkbox grammars and never matches a title ending in # — `runtime/src/primitives/append_task.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: invalidate-review keeps a stale dispositions map beside last-run: null — `runtime/src/primitives/invalidate_review.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: the inbox age pairs working-tree bullet lines with HEAD's blame — `runtime/src/primitives/inbox_standing.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- bug: the no-file inbox row says the file is missing when it exists but cannot be read — `runtime/src/primitives/dashboard.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- perf: dashboard runs git blame on the async MCP worker instead of dispatch_blocking — `runtime/src/mcp/server.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- simplicity: dead or inert code (Dispositions::total, --decided-by CLI flags, process-decisions' fired flag and dead branch, show_untracked_content) — `runtime/src/schema/primitives.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: stale runtime docs (append-inbox CLI help names dedup-by-prefix; a dashboard doc comment sits on the wrong item) — `runtime/src/main.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- reuse: write-review and write-analysis duplicate the dispositions block, the disposition rendering, and the decision conversion — `runtime/src/primitives/write_analysis.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- test: no MCP test for process-decisions and no round-trip tests for the new schema types — `runtime/tests/mcp.rs` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 058's data model says MCP and the CLI ignore dedup-prefix; they refuse it — `specs/058-findings-route-at-discovery/data-model.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: 022's data model lacks process-decisions and disposition-drift, and says dedup-prefix is ignored — `specs/022-deterministic-runtime/data-model.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: the /analyze section of docs/slash-commands.md and README's Analyze bullet describe the pre-058 contract — `docs/slash-commands.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: an AGENTS.md corollary still says pipeline passes grow the inbox — `AGENTS.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: review.md's Blocking semantics misstates the gate order — `framework/commands/review.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: implement.md reopens a routed spec unconditionally and forbids reading the spec a disposition routes to — `framework/commands/implement.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: analyze.md's unparseable-decisions wording contradicts the spec, and its markdown-only frontmatter section omits the decisions hard fail — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: the constitution's chore paragraph and exec sentence contradict the commands — `framework/constitution.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: groom's tree ends a finding in the inbox and lacks the unpinned-rule-file discard — `framework/commands/groom.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- convention: status.md's markdown-only path gives no inbox-age fallback without git — `framework/commands/status.md` — **routed** to `specs/058-findings-route-at-discovery/tasks.md`
- design: disposition drift on a done spec's own analysis.md never converges on a re-run — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/scenarios/analysis-drift-judges-the-record-it-writes.md`
- design: run-level restricted keeps analyze decisions forever on specs whose unexamined count is never zero — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/scenarios/only-unreadable-targets-retain-decisions.md`
- design: analyze keys use host-worded messages, so those findings are asked about again every run — `framework/commands/analyze.md` — **routed** to `specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md`
- design: --auto pauses on every disposition task although the walk says record and keep working — `framework/commands/implement.md` — **routed** to `specs/058-findings-route-at-discovery/scenarios/auto-records-disposition-tasks-without-pausing.md`
- perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- convention: a duplicate-key decision is pruned when its key expires or is decided again — `runtime/src/primitives/decisions.rs` — **discarded**: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
- convention: an all-empty decision entry is dropped on re-render — `runtime/src/primitives/decisions.rs` — **discarded**: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
- reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it

## Skipped passes

*None.*

## Unexamined governance

*None.*
