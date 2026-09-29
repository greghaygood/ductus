# 062 — Concurrent session targets Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Session core: identity, storage, lock

- [x] Bump `rust-version` in `runtime/Cargo.toml` from 1.88 to 1.89 for `std::fs::File::lock`
- [x] Add `SESSIONS_DIR` and `SESSIONS_LOCK` to `runtime/src/schema/paths.rs` beside `SESSION_FILE`
- [x] Move `derive_slug` from `runtime/src/primitives/create_feature.rs` to a shared `pub(crate)` home in `runtime/src/primitives/mod.rs`; `create-feature` calls the shared one
- [x] Create `runtime/src/session.rs` and register it in `runtime/src/lib.rs`: `identity_from_env(lookup)` (`DUCTUS_SESSION` → platform table with `CLAUDE_CODE_SESSION_ID` → none; empty is unset; slug-sanitized; empty after sanitizing is an `InvalidArgument` naming the variable)
- [x] Per-process record type per the data model; parse errors name the file
- [x] Lock helper over `.ductus/sessions/.lock`; directory creation writes `.ductus/sessions/.gitignore` containing `*`
- [x] Legacy-layout guard: when the active session file is a legacy tier, the core treats the process as unidentified
- [x] Unit tests with injected lookups: precedence, same-name sharing, empty-is-unset, sanitizing, refusal, Claude Code variable, legacy guard

- **Done when**: `cargo test` covers identity resolution for AC12, AC13 and AC14, and the legacy guard, all passing; `cargo clippy` is clean at the new MSRV.

## 2. Session core: resolve, write, sweep

- [x] `resolve(repo, identity)`: the five sources (`own`, `adopted`, `default`, `cleared`, `none`); `used-at` refresh; adoption notice; co-target notice against `seen-peers`; pending removal notice delivered and removed; malformed own file is an error, never adoption
- [x] `write_target` / `write_clear` / host-config: own file plus default per the plan; `seen-peers` set at write time; peers returned
- [x] Expiry sweep after target and clear writes: remove per-process files idle more than seven days, never the caller's own, re-reading `used-at` under the lock; report unparseable files without deleting them
- [x] Unit tests: adoption and pin (a later write by another identity leaves the pinned target alone), empty default pins nothing, cleared never adopts, co-target notice fires once and again only when the peer set changes, expiry boundary, a target used after the sweep began is kept, malformed own file errors, unidentified process reads and writes the default only, default always holds the latest change, `cli-config-dir` preserved through every write

- **Done when**: unit tests covering AC6, AC15, AC16, AC17, AC18, AC19, AC20 and AC22 pass against the core directly.

## 3. `write-session` through the core

- [ ] Rewrite `runtime/src/primitives/write_session.rs` to delegate to the core, taking the identity from the environment at the edge; keep the three write shapes and their validation
- [ ] Extend `WriteSessionResult` in `runtime/src/schema/primitives.rs` with `identity`, `own-path`, `peers`, `expired`, `unreadable`
- [ ] Keep the default's bytes identical to today's for an unidentified write, so the existing `write_session` tests and the parity byte-equality check still pass unchanged

- **Done when**: the existing `write_session` tests pass unmodified, new tests cover the identified target and clear writes, and AC3's single-agent shape is asserted byte-for-byte against the default.

## 4. `resolve-session` primitive

- [ ] Create `runtime/src/primitives/resolve_session.rs`, with args and result in `runtime/src/schema/primitives.rs` per the data model
- [ ] Register it in `runtime/src/schema/registry.rs`, `runtime/src/mcp/server.rs`, the CLI in `runtime/src/main.rs`, and the interpreter dispatch in `runtime/src/interpreter/mod.rs`
- [ ] Add `resolve-session` to `framework/runtime-tools.txt` and its permission entry to `framework/bootstrap/configure/claude.md` and `framework/bootstrap/configure/auggie.md`

- **Done when**: `ductus resolve-session` and the MCP tool return the data model's shape; `scripts/lint-tool-coverage.sh` and `scripts/audit/run-all.sh` pass.

## 5. `retarget-sessions` primitive

- [ ] Create `runtime/src/primitives/retarget_sessions.rs`: re-target or clear every per-process file and the default naming `from`, under the lock; pending notice on every file but the caller's own; argument validation per the data model
- [ ] Register it everywhere `resolve-session` was registered in task 4, including `framework/runtime-tools.txt` and both permission lists
- [ ] Tests: fold re-targets every session naming the spec and the default; consolidate clears them; sessions naming other features are not written; the caller gets no notice and the others do; unparseable files are reported

- **Done when**: tests covering AC5 and AC21 pass, and the audit families from task 4 pass.

## 6. `dashboard` and the exec seed through the core

- [ ] `load_session_target` in `runtime/src/primitives/dashboard.rs` resolves through the core; the payload gains `identity`, `source` and `notices`; `rendered-markdown` shows the session label on the target line and each notice beneath it
- [ ] The exec seed in `runtime/src/main.rs` resolves through the core instead of reading the session path
- [ ] Regenerate any golden stream under `runtime/tests/golden/` that the seed change alters, and confirm each diff is only the seed

- **Done when**: dashboard tests cover AC8, the exec seed test covers AC23's two cases, and `cargo test` passes including the walker goldens.

## 7. Cross-process integration tests

- [ ] Create `runtime/tests/concurrent_sessions.rs`, spawning the built binary with per-child environments (`DUCTUS_SESSION`, `CLAUDE_CODE_SESSION_ID`, neither)
- [ ] Two identities: a target write in one leaves the other's resolved target unchanged, for a plain target write and for each side-effect shape the commands use (AC1); each resolves its own feature (AC2)
- [ ] Concurrent writers: many children with distinct identities write at once, and each own file holds its own target (AC4)
- [ ] Two children with only `CLAUDE_CODE_SESSION_ID` set, to different values, hold separate targets (AC13); two with neither share the default (AC15)

- **Done when**: the integration tests for AC1, AC2, AC4, AC13 and AC15 pass under `cargo test`.

## 8. Gitignore

- [ ] Add `/.ductus/sessions/` to the managed block in `framework/templates/project/gitignore`, with a comment, beside `/.ductus/session.toml`
- [ ] Add `/.ductus/sessions/` to this repository's `.gitignore`

- **Done when**: after running two identified sessions in this repository, `git status` shows nothing under `.ductus/` (AC7), and the manifest and template audit families pass.

## 9. Command files

- [ ] Replace hand reads of the session file with "Invoke `resolve-session`; display its notices; markdown-only: read the shared default (§concurrent-features)" in `amend`, `analyze`, `clarify`, `groom`, `implement`, `plan`, `prune`, `review`, `specify` and `target` under `framework/commands/`
- [ ] `fold` step 13 and `consolidate` step 6 invoke `retarget-sessions`; consolidate's step drops its hand read of the session
- [ ] `status` describes the target line's session label, the notices, and the markdown-only derivation; `help` updates the Session target glossary entry
- [ ] Mirror every edit to `.claude/commands/ductus/`

- **Done when**: no file under `framework/commands/` instructs a hand read of the session file outside a markdown-only branch; the mirrors are byte-identical; `scripts/audit/run-all.sh` passes (AC24's command-side statement).

## 10. Constitution and the 010 signpost

- [ ] Rewrite §concurrent-features in `framework/constitution.md`: per-process targets and identity; the shared default and pin on first resolution; `git worktree` kept for isolating edits, with the one-branch-per-worktree limit; the two open bounds; the markdown-only reduction; removal extended to every session in the working tree, keeping the teammate bound
- [ ] Add a post-completion note to `specs/010-agent-autonomy/spec.md` §Parallel milestones and Resolved Question 5 linking to this spec

- **Done when**: AC10, AC24 and AC25 hold in the constitution text; 010 carries the back-link that discharges 062's declared impact; anchors resolve (`resolve-anchor`); markdownlint is clean.

## 11. Verification sweep

- [ ] `cargo build --release`, `cargo test`, `cargo clippy --all-targets` under `runtime/`
- [ ] `scripts/audit/run-all.sh`
- [ ] Manual check in this repository: two Claude Code sessions targeting different specs each keep their own through `/ductus:target` and `/ductus:status`, and a `retarget-sessions` call against a scratch spec both targets re-points or clears both and notifies the other session

- **Done when**: every command above passes, and each acceptance criterion maps to a passing test or a verified text change.
