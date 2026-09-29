---
spec: 062-concurrent-session-targets
last-run: 2026-09-29T20:47:19Z
reviewed-against: 216b66b0a96b726735456a1e53561847c3b50e6b
diff-base: 25c946eaf75727634cd849c8579baac00f4e4eb6
must-violations: 5
should-violations: 4
low-confidence: 0
examined: 62
scope: 67
skipped-passes: []
reviewed-digest:
  data-model.md: f8bd48568cda1b81bcbc565cb14b55f4501f679f2312ebeb73c547e424202dc9
blocking: true
dispositions:
  fixed: 10
  routed: 6
  discarded: 8
  undispositioned: 0
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
---

# Review — 062-concurrent-session-targets

## Summary

Blocking. The session core holds its cross-process lock without a time limit and skips it on the unidentified path while `.ductus/sessions/` does not yet exist; the identity variables are read per call and appear in no inventory; and `retarget-sessions` writes caller-supplied names into other sessions unchecked. Each MUST and SHOULD finding has a fix task on `tasks.md` (13–16). Of the observations, the contract gaps where a command drops a notice or a write result the spec promises are routed to tasks 17–18, the mechanical corrections were fixed in this run, and the rest are discarded with their reasons. Not examined: `specs/010-agent-autonomy/analysis.md` and `review.md`, 010's own audit records, which 062's changes to 010 left in place.

## MUST violations (blocking)

### MUST: BE-TIMEOUT-001 — The session lock waits without bound

- **File**: `runtime/src/session.rs:299-313`
- **Rule**: Every outbound network call, downstream-service call, or blocking I/O operation MUST be made under a bounded timeout — no call waits unboundedly for a response.
- **Finding**: `lock()` calls the blocking `File::lock()` with no deadline, and every identified resolve and write goes through it. A holder stalled mid-section (SIGSTOP or Ctrl-Z of a terminal `ductus` call, a debugger, a hung filesystem) hangs every other agent's `resolve-session` and `write-session` indefinitely; the MCP handlers for the three session tools also call their primitives directly inside `async fn` rather than through `dispatch_blocking`, so each wait holds an async worker.
- **Auto-fixable**: no
- **Suggested fix**: Poll `File::try_lock` with a short backoff until a named `SESSION_LOCK_TIMEOUT` expires, then return an error naming `.ductus/sessions/.lock`; route the three session handlers through `dispatch_blocking`. Task 13.

### MUST: BE-TXN-002 — Unidentified writes and retargets skip the lock before the sessions directory exists

- **File**: `runtime/src/session.rs:1078-1090`
- **Rule**: A concurrent read-modify-write of persisted state MUST prevent lost updates — via an optimistic version/ETag check, a `SELECT … FOR UPDATE` row lock, or an atomic conditional write; a bare read-then-write under concurrency is silent data corruption.
- **Finding**: For an unidentified process, `write` (and `retarget`, line 985) takes `lock_if_shared`, which returns no lock while `.ductus/sessions/` is absent, then reads, modifies and replaces the shared default unguarded. An identified process whose first `lock()` creates the directory in that window read-modify-writes the same default concurrently, so a host-config write's `cli-config-dir` or a target write's default target is silently lost; a retarget in the window can leave a new per-process file naming the removed spec.
- **Auto-fixable**: no
- **Suggested fix**: Take `lock()` in `write` and `retarget` on every call; keep `lock_if_shared` for the read-only `peek`. Task 13.

### MUST: CFG-ENV-001 — The session identity variables are read on every primitive call

- **File**: `runtime/src/session.rs:134-136`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.
- **Finding**: `identity_from_process_env` calls `std::env::var` each time, and `write_session::run`, `resolve_session::run`, `retarget_sessions::run` and `dashboard::run` each call it per invocation, so the long-lived MCP server re-reads `DUCTUS_SESSION` and `CLAUDE_CODE_SESSION_ID` on every tool call, and `ductus exec` reads them once for the seed and again per session primitive it dispatches.
- **Auto-fixable**: no
- **Suggested fix**: Read the identity once at startup in `main` and cache it; primitives read the cached value. Task 14.

### MUST: CFG-ENV-002 — DUCTUS_SESSION is added with no environment-variable inventory

- **File**: `runtime/src/session.rs:33-47`
- **Rule**: A single canonical inventory of every environment variable the application reads MUST be maintained alongside the source code. The inventory MUST describe each variable's purpose, declare whether it is required or optional, and provide a safe placeholder or default value.
- **Finding**: 062 introduces the operator-set `DUCTUS_SESSION` and reads `CLAUDE_CODE_SESSION_ID`, but the repository keeps no canonical inventory of the variables the runtime reads — they are declared here, `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` privately in `fetch_archive.rs`, and described only in constitution prose — so an operator cannot find the runtime's configuration surface in one place.
- **Auto-fixable**: no
- **Suggested fix**: Add one inventory listing all three variables with purpose, required/optional and default. Task 14.

### MUST: BE-INPUT-002 — retarget-sessions writes unvalidated names into other sessions

- **File**: `runtime/src/primitives/retarget_sessions.rs:60-135`
- **Rule**: Input validation MUST use allowlists (define what is acceptable) for constrained inputs. Denylists (block known-bad patterns) MUST NOT be used as the sole defense.
- **Finding**: `validate` checks only `path` and `scenario-path` for traversal; `from`, `feature` and `scenario` are accepted as any string, then stored in every matching per-process file and rendered into other sessions' notices and dashboard `Target:` and `Notice:` lines — a newline in one forges extra lines in another agent's display. Feature directory names and scenario slugs have a grammar (`parse_feature_dir`, `validate_slug`) that is not applied.
- **Auto-fixable**: no
- **Suggested fix**: Check `from` and `feature` with `parse_feature_dir` and `scenario` with `validate_slug` before any session is touched; apply the same to `write-session`. Task 15.

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — resolve and peek drop unreadable peer files silently

- **File**: `runtime/src/session.rs:738-754`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: `due_notices` discards the unreadable list `peers_of` returns, and `Resolution` has no field to carry it, so `resolve-session` and the dashboard answer an empty notice list both when no other session targets the feature and when a peer's file could not be parsed — while `write-session` and `retarget-sessions` report the same case, and AC22 requires such a file to be reported by name.
- **Auto-fixable**: no
- **Suggested fix**: Carry `unreadable` through `Resolution` into `ResolveSessionResult` and the dashboard payload. Task 16.

### SHOULD: QUAL-TEST-001 — The writer-is-never-swept test passes with the guard deleted

- **File**: `runtime/src/session.rs:1574-1586`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: `write` stamps the writer's own record with `used-at = now` before `sweep` runs, so the writer is never idle when swept and the test's assertion holds whether or not `sweep` excludes the caller's own file; the behavior the test names is untested, and the guard cannot change the outcome from any current caller.
- **Auto-fixable**: no
- **Suggested fix**: Drive `sweep` directly with a stale own file, or remove the unreachable guard and rename the test. Task 16.

### SHOULD: AGENTS.md#gotchas:session-identity-in-tests — An in-process walk dispatches the environment-reading resolve-session

- **File**: `runtime/tests/walker.rs:831-915`
- **Rule**: How to apply: a test that spawns the binary removes both variables (`ductus_command` in `runtime/tests/parity.rs` and `runtime/tests/exec_subprocess.rs`; `runtime/tests/concurrent_sessions.rs` sets them explicitly per child), and an in-process test calls a primitive's unidentified seam (`run_as(…, None, …)`), never its environment-reading `run`.
- **Finding**: prune.md's step 1 now dispatches `resolve-session`, which the in-process `Walker` binds to `resolve_session::run`, reading the identity from the test process itself; in a Claude Code shell the walk runs as an identified process and takes the lock in its tempdir, and an unsanitizable `DUCTUS_SESSION` fails the test for a reason unrelated to its name. The interpreter has no way to pass an identity in.
- **Auto-fixable**: no
- **Suggested fix**: Read the identity once at startup (task 14); an in-process caller that never initialized it is unidentified by construction.

### SHOULD: AGENTS.md#gotchas:backtick-dispatch — exec dispatches target's conditional resolve-session on every walk

- **File**: `framework/commands/target.md:29`
- **Rule**: How to apply: in a numbered `## Instructions` step, backtick a primitive name only when the step is meant to *dispatch* it.
- **Finding**: Step 1 applies only when there is no argument, but `ductus exec` dispatches any backticked primitive in a numbered step, and `runtime/tests/golden/target-basic.jsonl` shows `resolve-session` dispatched on an argument-supplied walk; the walker merges the result without emitting it, so an identified caller's pending notices are consumed unseen.
- **Auto-fixable**: no
- **Suggested fix**: Make step 1 resolve and display notices whatever the argument, so the dispatch is intended (task 17), and have exec emit the notices it consumes (task 18).

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- contract: `/target X` and `--clear` overwrite a pending fold or consolidation notice undelivered, since target resolves only without an argument (AC21) — task 17 — `framework/commands/target.md:29` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- contract: no command displays write-session's `peers`, so the writing session is never told who shares its feature (AC19) — task 17 — `framework/commands/target.md:44` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- contract: fold's and consolidate's report steps name neither the other sessions re-pointed or cleared nor unreadable files (AC21, AC22) — task 17 — `framework/commands/consolidate.md:80` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- contract: `ductus exec` consumes resolve-session's notices without emitting them (AC19, AC21) — task 18 — `runtime/src/interpreter/mod.rs:237` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- test-gap: no exec test runs with an identity set, so AC23 is untested where run_exec reads it — task 18 — `runtime/src/main.rs:586` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- test-gap: no test exercises the advisory lock itself; AC4's test passes without it — task 13 — `runtime/tests/concurrent_sessions.rs:178` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- accuracy: §concurrent-features and the spec's intro said a lone operator sees no change, but on Claude Code that agent is identified — `framework/constitution.md:766` — **fixed**
- stale-claim: amend's scenario-route write step described a single-file write and omitted the markdown-only reduction — `framework/commands/amend.md:206` — **fixed**
- stale-claim: the bootstrap's Session state section said there is no session state beyond the one file — `framework/bootstrap/ductus.md:881` — **fixed**
- broken-anchor: the spec linked `#runtime-host-integration`, a comment marker rather than a heading slug — `specs/062-concurrent-session-targets/spec.md:79` — **fixed**
- drift: the DashboardResult.session_notices doc said the dashboard consumed the notices — `runtime/src/schema/primitives.rs:1850` — **fixed**
- contract: retarget-sessions' error kinds and argument names disagreed with its doc comment and data-model.md — `runtime/src/primitives/retarget_sessions.rs:60` — **fixed**
- contract: data-model.md said write-session's identity and own-path are null when unidentified, where the result omits them — `specs/062-concurrent-session-targets/data-model.md:85` — **fixed**
- quality: the exec seed paired a process's own target with the shared default's set-at — `runtime/src/main.rs:517` — **fixed**
- reuse: the identity-clearing ductus_command test helper was copied per file, and three tests spawned the binary without it — `runtime/tests/common/mod.rs` — **fixed**
- simplicity: file_identity_key recomputed the stem that session_files already holds — `runtime/src/session.rs:1018` — **fixed**
- simplicity: two enums represent the removal cause, and the recorded cause is never read back — `runtime/src/primitives/retarget_sessions.rs` — **discarded**: one match translates between them, and retarget-sessions, the only caller, validates the cause against the target, so the two cannot disagree
- efficiency: every resolution parses every session file, and peek takes the exclusive lock — `runtime/src/session.rs:580` — **discarded**: the files are sub-kilobyte and bounded by seven days of sessions; the cost is negligible beside a command's own process spawn
- input: session names have no length limit — `runtime/src/session.rs:71` — **discarded**: a name past the filesystem's name limit is not a realistic operator input, and its failure names the path rather than passing silently
- quality: a used-at stamped in the future is never reported — `runtime/src/session.rs:429` — **discarded**: a future stamp still expires seven days after it, so nothing accumulates; a clock set ahead is outside the design
- reuse: inbox_standing keeps its own civil-from-days copy — `runtime/src/primitives/inbox_standing.rs:208` — **discarded**: it predates 062 in spec 058's file, which 062 did not touch; consolidating date helpers is a refactor outside this spec
- reuse: consolidate.md restates §concurrent-features' reasoning in its step and its markdown-only reference — `framework/commands/consolidate.md:77` — **discarded**: the restatement predates 062, and a step plus its markdown-only reference both describing the procedure is the command files' two-path convention
- terminology: 'agent identity' (cli-config-dir) now sits beside 'session identity' — `framework/constitution.md:775` — **discarded**: 'agent identity' names cli-config-dir in ten passages that predate 062; renaming it is a terminology sweep outside this spec
- exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76` — **discarded**: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does

## Skipped passes

*None.*

## Unexamined governance

*None.*
