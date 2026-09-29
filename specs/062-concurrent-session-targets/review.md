---
spec: 062-concurrent-session-targets
last-run: 2026-09-29T23:13:22Z
reviewed-against: b7771f72d8eecdbd6ca201af658ae77e3f51df2c
diff-base: 25c946eaf75727634cd849c8579baac00f4e4eb6
must-violations: 2
should-violations: 1
low-confidence: 0
examined: 27
scope: 81
skipped-passes: []
reviewed-digest:
  data-model.md: a0de31e159f154b7544c6ccb13be4a430d6c328a4745750ffdb0b244081b5201
blocking: true
dispositions:
  fixed: 3
  routed: 3
  discarded: 0
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

Blocking. The second review's findings are resolved (tasks 20–23): writes name an unparseable own record and never lose a notice part-way, session names are held to visible ASCII, the inventory lists the proxy variables, and the identity-capture test holds in any environment. This pass read the in-scope files changed since the second review's head `ce1cfc1e`; the others are unchanged since earlier passes examined them, and their decisions are retained. Two MUST findings remain, both in the inventory: its `REQUEST_METHOD` row understates the CGI guard, and it omits the certificate variables the HTTP client reads on Linux (task 25). A write still treats a malformed shared default as absent (SHOULD, task 24), which together with step 1 running on every `/target` means a malformed default can no longer be repaired and halts every `ductus exec` walk — routed to task 24. The per-fetch proxy read stays waived and routed to spec 048.

## MUST violations (blocking)

### MUST: CFG-ENV-002 — The REQUEST_METHOD row understates the CGI guard

- **File**: `docs/runtime.md:48`
- **Rule**: The inventory MUST describe each variable's purpose, declare whether it is required or optional, and provide a safe placeholder or default value.
- **Finding**: The row says that with `REQUEST_METHOD` set the HTTP client ignores `HTTP_PROXY`. hyper-util 0.1.20's matcher returns no HTTP proxy, no HTTPS proxy and an empty NO_PROXY whenever it is set (`matcher.rs:304-311`), so every proxy variable is ignored and fetch-archive's https downloads go direct.
- **Auto-fixable**: yes
- **Suggested fix**: Say that with REQUEST_METHOD set the client ignores every proxy variable above. Task 25.

### MUST: CFG-ENV-002 — The inventory omits the certificate variables the HTTP client reads on Linux

- **File**: `docs/runtime.md:39`
- **Rule**: A single canonical inventory of every environment variable the application reads MUST be maintained alongside the source code.
- **Finding**: On Linux and other non-Apple Unix, reqwest's certificate verifier loads roots through `rustls_native_certs::load_native_certs`, which reads `SSL_CERT_FILE` and `SSL_CERT_DIR` and trusts only what they name when either is set — on each fetch, since fetch-archive builds a client per call. They decide which authorities fetch-archive trusts, the same kind of client configuration as the proxy variables the inventory lists, and its carve-out does not separate them.
- **Auto-fixable**: no
- **Suggested fix**: List SSL_CERT_FILE and SSL_CERT_DIR, and make the carve-out name what it excludes and why. Task 25.

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — A write treats a malformed shared default as absent

- **File**: `runtime/src/session.rs:1218-1220`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: `write` reads the default through `read_default_lenient`, so a default that does not parse becomes an empty record: the write replaces it, drops its `cli-config-dir`, and returns a result indistinguishable from a clean replace — while `retarget` reports the same file and `resolve` errors on it (AC22).
- **Auto-fixable**: no
- **Suggested fix**: Report a malformed default in `unreadable`, as replaced. Task 24.

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: CFG-ENV-001 — fetch-archive reads the proxy variables on every call

- **File**: `runtime/src/primitives/fetch_archive.rs:113-139`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.
- **Finding**: Each call builds a new reqwest client without `.no_proxy()`, and each build reads the proxy variables, so the long-lived MCP server re-reads them per fetch.
- **Auto-fixable**: no
- **Suggested fix**: Waived here and routed to spec 048's scenario `fetch-archive-reads-its-proxy-once`.
- **Waived**: Pre-existing in spec 048's fetch-archive: reqwest reads the proxy variables on each client build. Turning proxies off would break adopters behind a proxy, so the design is routed to 048's scenario fetch-archive-reads-its-proxy-once.

## Observations

- regression: with a malformed shared default, /target X can no longer repair it and every exec walk halts at the seed — task 24 — `framework/commands/target.md:29` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- exec-path: target's continue-past-a-malformed-own-file path has no exec equivalent, and the step does not say so — task 24 — `runtime/src/main.rs:505` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- accuracy: 048's new scenario says REQUEST_METHOD makes the matcher ignore HTTP_PROXY; it ignores every proxy variable — task 25 — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md:21` — **routed** to `specs/062-concurrent-session-targets/tasks.md`
- accuracy: the unreadable docs said 'could not parse', but the sweep also reports a file whose used-at cannot be read — `runtime/src/schema/primitives.rs:3001` — **fixed**
- accuracy: data-model's allowlist paragraph omitted control characters and backslash, and read 'The one names' — `specs/062-concurrent-session-targets/data-model.md:146` — **fixed**
- drift: the exec expired line hardcoded 'seven days' instead of deriving it from IDLE_EXPIRY — `runtime/src/interpreter/mod.rs:887` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
