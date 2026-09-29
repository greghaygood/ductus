---
spec: 062-concurrent-session-targets
last-run: 2026-09-29T22:31:00Z
reviewed-against: ce1cfc1e58f4e43cbd199c1e8a14d0d2c28210c5
diff-base: 25c946eaf75727634cd849c8579baac00f4e4eb6
must-violations: 2
should-violations: 2
low-confidence: 1
examined: 55
scope: 78
skipped-passes: []
reviewed-digest:
  data-model.md: 0479b7baec8556d0c3799a6bd5c6257f6048398f604af0fa37451e74b0dc6532
blocking: true
dispositions:
  fixed: 12
  routed: 3
  discarded: 4
  undispositioned: 0
waivers:
  - rule: CFG-ENV-001
    file: runtime/src/primitives/fetch_archive.rs
    reason: "Pre-existing in spec 048's fetch-archive: reqwest reads the proxy variables on each client build. Turning proxies off would break adopters behind a proxy, so the design is routed to 048's scenario fetch-archive-reads-its-proxy-once."
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
---

# Review — 062-concurrent-session-targets

## Summary

Blocking. The first review's nine findings are resolved (tasks 13–19): the session lock is bounded and taken by every write, the identity variables are captured once at startup, names written into sessions are validated, resolutions name unreadable peer files, and commands and `ductus exec` deliver every notice and write result. This re-review read the in-scope files changed since the pre-review head `216b66b0`; the others are unchanged since the first review examined them, and its decisions on them are retained. Two MUST findings remain: the slug part of a sequential feature name is still checked only by a denylist, so bidi and zero-width characters pass, and the new environment-variable inventory omits the proxy variables reqwest reads. Both have fix tasks (21, 22), as do the two SHOULD findings (20, 23). The per-fetch proxy read in spec 048's `fetch-archive` is waived here and routed to 048, which is reopened with a scenario.

## MUST violations (blocking)

### MUST: BE-INPUT-002 — A sequential feature name's slug is checked only by a character denylist

- **File**: `runtime/src/primitives/mod.rs:2300-2315`
- **Rule**: Input validation MUST use allowlists (define what is acceptable) for constrained inputs. Denylists (block known-bad patterns) MUST NOT be used as the sole defense.
- **Finding**: `parse_feature_dir` leaves a sequential directory's slug unconstrained on purpose, and `validate_session_feature` then applies only the denylist `!is_control && !is_whitespace && != '/' && != '\\'`. `char::is_control` covers category Cc alone, so U+202E (right-to-left override) and U+200B (zero-width space) pass and reach other sessions' notices and dashboard `Target:` lines, where they reorder or hide text.
- **Auto-fixable**: no
- **Suggested fix**: Allow only visible ASCII (`is_ascii_graphic`) with no path separator. Task 21.

### MUST: CFG-ENV-002 — The environment-variable inventory omits the proxy variables

- **File**: `docs/runtime.md:37-47`
- **Rule**: A single canonical inventory of every environment variable the application reads MUST be maintained alongside the source code. The inventory MUST describe each variable's purpose, declare whether it is required or optional, and provide a safe placeholder or default value.
- **Finding**: The inventory says it is the one inventory of the variables the runtime reads, and that all three are read once at startup, but `fetch-archive` builds a reqwest client per call with the system proxy on, which reads `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY` and `NO_PROXY` (and their lowercase forms). They change where the runtime connects and are listed nowhere; `main`'s comment claiming every variable is read there once is false for the same reason.
- **Auto-fixable**: no
- **Suggested fix**: List the proxy variables with purpose and default, and state which variables are captured at startup and which the HTTP client reads per fetch. Task 22.

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — A write treats a malformed own record as having no notice

- **File**: `runtime/src/session.rs:1228-1240`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: `write` reads the own record with `ProcessRecord::load(..).ok()`, so a missing file and one that does not parse both yield no notice, and the write then overwrites the malformed file without naming it anywhere in its result — while `resolve` reports the same file as an error (AC22).
- **Auto-fixable**: no
- **Suggested fix**: Report an unparseable own record in `unreadable`, as replaced. Task 20.

### SHOULD: QUAL-TEST-001 — The capture test cannot fail where no identity is set

- **File**: `runtime/src/session.rs:1375-1381`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: The test asserts `process_identity()` is none. A per-call environment read also returns none when neither identity variable is set — CI's state — so the test fails only in a shell that carries one.
- **Auto-fixable**: no
- **Suggested fix**: Re-run the test as a child test process with an identity set. Task 23.

## Low-confidence findings

### LOW-CONFIDENCE: BE-RETRY-001 — The lock poll retries at a fixed interval without jitter

- **File**: `runtime/src/session.rs:362-380`
- **Rule**: Automatic retries MUST be bounded by a maximum attempt count, spaced with exponential backoff plus jitter, and limited to idempotent operations.
- **Finding**: `lock_within` retries `try_lock` every 10 ms until the deadline: bounded and idempotent, but fixed-interval. The rule's concern is retry storms against a downstream dependency; the contenders here are a few local processes polling a kernel lock held for milliseconds.
- **Auto-fixable**: no
- **Suggested fix**: Record at LOCK_RETRY why the rule's backoff does not apply. Task 23.

## Waived findings

### WAIVED: CFG-ENV-001 — fetch-archive reads the proxy variables on every call

- **File**: `runtime/src/primitives/fetch_archive.rs:113-139`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.
- **Finding**: Each call builds a new reqwest client without `.no_proxy()`, and each build reads the proxy variables through hyper-util's system matcher, so the long-lived MCP server re-reads them per fetch.
- **Auto-fixable**: no
- **Suggested fix**: Waived here and routed to spec 048's scenario `fetch-archive-reads-its-proxy-once`.
- **Waived**: Pre-existing in spec 048's fetch-archive: reqwest reads the proxy variables on each client build. Turning proxies off would break adopters behind a proxy, so the design is routed to 048's scenario fetch-archive-reads-its-proxy-once.

## Observations

- exec: the exec stream omitted write-session's expired sessions and each peer's scenario — `runtime/src/interpreter/mod.rs:857` — **fixed**
- doc: the shared test helper said every spawning test builds its command there, but concurrent_sessions.rs builds its own — `runtime/tests/common/mod.rs:17` — **fixed**
- doc: validate_args linked the test helper run_with_now — `runtime/src/primitives/write_session.rs:111` — **fixed**
- accuracy: the lone-agent claim omitted the restart case, where the earlier session shows as a co-target until it expires — `framework/constitution.md:766` — **fixed**
- contract: --clear displayed expired and unreadable sessions but not the notices a clear write delivers — `framework/commands/target.md:32` — **fixed**
- clarity: a bare (AC22) in fold.md and consolidate.md read as the command's own spec's criterion — `framework/commands/fold.md:79` — **fixed**
- clarity: groom's step 7 pointer did not mention what to display from the write — `framework/commands/groom.md:47` — **fixed**
- accuracy: 'only its own writes change its target' omitted fold and consolidation retargets — `specs/062-concurrent-session-targets/data-model.md:31` — **fixed**
- contract: data-model.md did not state the name allowlist task 15 added — `specs/062-concurrent-session-targets/data-model.md:137` — **fixed**
- contract: data-model.md's cleared list omitted the default label — `specs/062-concurrent-session-targets/data-model.md:145` — **fixed**
- drift: plan.md's Affected Files lacked fetch_archive.rs, and its edges paragraph omitted the primitives' run — `specs/062-concurrent-session-targets/plan.md:53` — **fixed**
- drift: README.md's pointer to docs/runtime.md did not name the environment-variable inventory — `README.md:216` — **fixed**
- quality: a write overwrites the own record before storing the default and sweeping, so an error part-way loses the notice it replaced — task 20 — `runtime/src/session.rs:1256` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- regression: /target X now fails at step 1 on a malformed own session file it used to repair by overwriting — task 20 — `framework/commands/target.md:29` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- security: with a proxy set, fetch-archive's connection is tunnelled and its SSRF address pin no longer holds (pre-existing) — `runtime/src/primitives/fetch_archive.rs:130` — **routed** to `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md`
- test: two lock tests rely on sleeps to reach the contended state — `runtime/src/session.rs:1452` — **discarded**: neither can fail falsely, and the deterministic timeout and fresh-repo lock tests carry the behavior
- concurrency: the legacy layout still read-modify-writes the default without the lock — `runtime/src/session.rs:393` — **discarded**: deliberate and recorded in the plan: creating .ductus/sessions/ beside a legacy file breaks 042's cutover rule, and it lasts only until /ductus migrates the repo
- ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29` — **discarded**: truthful: the session did adopt that target before the write replaced it
- reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44` — **discarded**: each command stating its own procedure in full is the command files' self-contained convention

## Skipped passes

*None.*

## Unexamined governance

*None.*
