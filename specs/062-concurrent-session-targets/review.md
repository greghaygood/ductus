---
spec: 062-concurrent-session-targets
last-run: 2026-09-30T13:56:20Z
reviewed-against: 2d173c280c27da749dd83fd6eb63f442e57a9dce
diff-base: 904671d5ca6f44f169c68af4a6e91da3432f0230
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 38
skipped-passes: []
reviewed-digest:
  data-model.md: 9ce5458a99a6ba857f8778df877a8b2b0a2708037b23b05319588b4c06aaa10f
  scenarios/first-writers-create-the-ignore-file-once.md: 02149b44fd9c6d76d8e9d97af6235df746bf028cad34e693bb57302dea1b1f15
blocking: false
dispositions:
  fixed: 3
  routed: 0
  discarded: 0
  undispositioned: 0
waivers:
  - rule: CFG-ENV-001
    file: runtime/src/primitives/fetch_archive.rs
    reason: "Pre-existing in spec 048's fetch-archive: reqwest reads the proxy variables, and on Linux the certificate variables SSL_CERT_FILE and SSL_CERT_DIR, on each client build. Turning proxies off would break adopters behind a proxy, so the design is routed to 048's scenario fetch-archive-reads-its-proxy-once."
    waived-at: 2026-09-29T22:27:22Z
    waived-by: andy@stone.dev
decisions:
  - key: "contract: `/target X` and `--clear` overwrite a pending fold or consolidation notice undelivered, since target resolves only without an argument (AC21) — task 17 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: no command displays write-session's `peers`, so the writing session is never told who shares its feature (AC19) — task 17 — `framework/commands/target.md:44`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: fold's and consolidate's report steps name neither the other sessions re-pointed or cleared nor unreadable files (AC21, AC22) — task 17 — `framework/commands/consolidate.md:80`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: `ductus exec` consumes resolve-session's notices without emitting them (AC19, AC21) — task 18 — `runtime/src/interpreter/mod.rs:237`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no exec test runs with an identity set, so AC23 is untested where run_exec reads it — task 18 — `runtime/src/main.rs:586`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no test exercises the advisory lock itself; AC4's test passes without it — task 13 — `runtime/tests/concurrent_sessions.rs:178`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "simplicity: two enums represent the removal cause, and the recorded cause is never read back — `runtime/src/primitives/retarget_sessions.rs`"
    outcome: discarded
    reason: one match translates between them, and retarget-sessions, the only caller, validates the cause against the target, so the two cannot disagree
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "efficiency: every resolution parses every session file, and peek takes the exclusive lock — `runtime/src/session.rs:580`"
    outcome: discarded
    reason: the files are sub-kilobyte and bounded by seven days of sessions; the cost is negligible beside a command's own process spawn
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "input: session names have no length limit — `runtime/src/session.rs:71`"
    outcome: discarded
    reason: a name past the filesystem's name limit is not a realistic operator input, and its failure names the path rather than passing silently
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a used-at stamped in the future is never reported — `runtime/src/session.rs:429`"
    outcome: discarded
    reason: a future stamp still expires seven days after it, so nothing accumulates; a clock set ahead is outside the design
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing keeps its own civil-from-days copy — `runtime/src/primitives/inbox_standing.rs:208`"
    outcome: discarded
    reason: it predates 062 in spec 058's file, which 062 did not touch; consolidating date helpers is a refactor outside this spec
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: consolidate.md restates §concurrent-features' reasoning in its step and its markdown-only reference — `framework/commands/consolidate.md:77`"
    outcome: discarded
    reason: the restatement predates 062, and a step plus its markdown-only reference both describing the procedure is the command files' two-path convention
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "terminology: 'agent identity' (cli-config-dir) now sits beside 'session identity' — `framework/constitution.md:775`"
    outcome: discarded
    reason: "'agent identity' names cli-config-dir in ten passages that predate 062; renaming it is a terminology sweep outside this spec"
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76`"
    outcome: discarded
    reason: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a write overwrites the own record before storing the default and sweeping, so an error part-way loses the notice it replaced — task 20 — `runtime/src/session.rs:1256`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: /target X now fails at step 1 on a malformed own session file it used to repair by overwriting — task 20 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "security: with a proxy set, fetch-archive's connection is tunnelled and its SSRF address pin no longer holds (pre-existing) — `runtime/src/primitives/fetch_archive.rs:130`"
    outcome: routed
    target: specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "test: two lock tests rely on sleeps to reach the contended state — `runtime/src/session.rs:1452`"
    outcome: discarded
    reason: neither can fail falsely, and the deterministic timeout and fresh-repo lock tests carry the behavior
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "concurrency: the legacy layout still read-modify-writes the default without the lock — `runtime/src/session.rs:393`"
    outcome: discarded
    reason: "deliberate and recorded in the plan: creating .ductus/sessions/ beside a legacy file breaks 042's cutover rule, and it lasts only until /ductus migrates the repo"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29`"
    outcome: discarded
    reason: "truthful: the session did adopt that target before the write replaced it"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44`"
    outcome: discarded
    reason: each command stating its own procedure in full is the command files' self-contained convention
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: with a malformed shared default, /target X can no longer repair it and every exec walk halts at the seed — task 24 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "exec-path: target's continue-past-a-malformed-own-file path has no exec equivalent, and the step does not say so — task 24 — `runtime/src/main.rs:505`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "accuracy: 048's new scenario says REQUEST_METHOD makes the matcher ignore HTTP_PROXY; it ignores every proxy variable — task 25 — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md:21`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
---

# Review — 062-concurrent-session-targets

## Summary

Not blocking. A deliberate partial review over 062's reopen window (diff base `904671d5`, the commit before the 2026-09-30 reopen at `38b3cab6`), approved by the operator on the precedent of 058's and 052's partials. The five passes read the 8 files modified since that base, in full: `runtime/src/session.rs`, `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md`, `specs/062-concurrent-session-targets/spec.md`, `specs/062-concurrent-session-targets/tasks.md`, `specs/062-concurrent-session-targets/plan.md`, `specs/062-concurrent-session-targets/data-model.md`, `AGENTS.md` and `specs/041-task-pruning/tasks.md`. The window's behavior change is task 26: `lock_within` creates the sessions directory, takes the lock, and only then writes `.gitignore` when absent, so racing first writers create it once; `runtime` run 36665970819 passed on windows-latest, ubuntu-latest and macos-latest at `9a256750`. No rule finding in the window. Three observations were fixed in the run (`c30f4a58`): the data model and plan described the ignore file as created with the directory, the scenario's first edge case gained a test proven red by mutation, and 041's task 17 note was brought up to date. The CFG-ENV-001 waiver on `runtime/src/primitives/fetch_archive.rs` still applies: that file is unchanged since the pass that found it (`git diff 834b72a4..HEAD` is empty), so the finding stands without a re-read, and it is routed to 048. Stored decisions were retained, not re-matched, because their sources were not re-read (`restricted`). This is not a full five-pass review of 062's scope. `examined: 8` counts files against `scope: 38`, which counts scope entries: the 8 read are 8 of its 35 single-path entries, and the other 30 entries were not read this pass. Those are 27 single paths and 3 patterns covering 41 files (`.claude/commands/ductus/*.md`, 18 files; `framework/commands/{amend,analyze,clarify,consolidate,fold,groom,help,implement,plan,prune,review,specify,status,target}.md`, 14; `runtime/tests/golden/*.jsonl`, 9), 68 files in all, each unread: `.claude/commands/ductus/amend.md`, `.claude/commands/ductus/analyze.md`, `.claude/commands/ductus/audit.md`, `.claude/commands/ductus/clarify.md`, `.claude/commands/ductus/configure.md`, `.claude/commands/ductus/consolidate.md`, `.claude/commands/ductus/fold.md`, `.claude/commands/ductus/groom.md`, `.claude/commands/ductus/help.md`, `.claude/commands/ductus/implement.md`, `.claude/commands/ductus/link.md`, `.claude/commands/ductus/log.md`, `.claude/commands/ductus/plan.md`, `.claude/commands/ductus/prune.md`, `.claude/commands/ductus/review.md`, `.claude/commands/ductus/specify.md`, `.claude/commands/ductus/status.md`, `.claude/commands/ductus/target.md`, `.gitignore`, `README.md`, `docs/runtime.md`, `framework/bootstrap/configure/auggie.md`, `framework/bootstrap/configure/claude.md`, `framework/bootstrap/ductus.md`, `framework/commands/amend.md`, `framework/commands/analyze.md`, `framework/commands/clarify.md`, `framework/commands/consolidate.md`, `framework/commands/fold.md`, `framework/commands/groom.md`, `framework/commands/help.md`, `framework/commands/implement.md`, `framework/commands/plan.md`, `framework/commands/prune.md`, `framework/commands/review.md`, `framework/commands/specify.md`, `framework/commands/status.md`, `framework/commands/target.md`, `framework/constitution.md`, `framework/runtime-tools.txt`, `framework/templates/project/gitignore`, `runtime/Cargo.toml`, `runtime/src/interpreter/mod.rs`, `runtime/src/lib.rs`, `runtime/src/main.rs`, `runtime/src/mcp/server.rs`, `runtime/src/primitives/create_feature.rs`, `runtime/src/primitives/dashboard.rs`, `runtime/src/primitives/fetch_archive.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/primitives/resolve_session.rs`, `runtime/src/primitives/retarget_sessions.rs`, `runtime/src/primitives/write_session.rs`, `runtime/src/schema/paths.rs`, `runtime/src/schema/primitives.rs`, `runtime/src/schema/registry.rs`, `runtime/tests/common/mod.rs`, `runtime/tests/concurrent_sessions.rs`, `runtime/tests/golden/analyze-basic.jsonl`, `runtime/tests/golden/cross-service-basic.jsonl`, `runtime/tests/golden/ductus-basic.jsonl`, `runtime/tests/golden/implement-basic.jsonl`, `runtime/tests/golden/plan-basic.jsonl`, `runtime/tests/golden/review-basic.jsonl`, `runtime/tests/golden/specify-basic.jsonl`, `runtime/tests/golden/status-basic.jsonl`, `runtime/tests/golden/target-basic.jsonl`, `specs/010-agent-autonomy/spec.md`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: CFG-ENV-001 — fetch-archive reads the proxy and certificate variables on every call

- **File**: `runtime/src/primitives/fetch_archive.rs:113-139`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.
- **Finding**: Each call builds a new reqwest client without `.no_proxy()`; each build reads the proxy variables through hyper-util's system matcher and, on Linux, `SSL_CERT_FILE` and `SSL_CERT_DIR` through the certificate verifier, so the long-lived MCP server re-reads them per fetch.
- **Auto-fixable**: no
- **Suggested fix**: Waived here and routed to spec 048's scenario `fetch-archive-reads-its-proxy-once`.
- **Waived**: Pre-existing in spec 048's fetch-archive: reqwest reads the proxy variables, and on Linux the certificate variables SSL_CERT_FILE and SSL_CERT_DIR, on each client build. Turning proxies off would break adopters behind a proxy, so the design is routed to 048's scenario fetch-archive-reads-its-proxy-once.

## Observations

- record: 062's data-model.md and plan.md said the sessions .gitignore is created with the directory; the first lock holder writes it — `specs/062-concurrent-session-targets/data-model.md:10` — **fixed**
- test-gap: the scenario's first edge case, an existing .gitignore left as it is, had no test — `runtime/src/session.rs:312` — **fixed**
- record: 041 task 17's note predated 062 task 26's Windows verdict and the operator's decision on 048 — `specs/041-task-pruning/tasks.md:138` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
