---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-13T23:26:01Z
reviewed-against: 9f44314643967194b72ae6f7da66c789a00b2f06
diff-base: a2050318dd2ad77149c6bb284fed535a6800f731
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 10
scope: 41
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Five passes (security, reuse, quality, efficiency, simplicity) over the reopen for the `[constitutions.*]` validation gap. 0 MUST / 0 SHOULD / 0 low-confidence outstanding — but this was **not** a pass that found nothing. Two defects were found in the change under review and **fixed inside the pass**, in `9f443146`, which this HEAD contains, so neither is outstanding at the reviewed commit. (1) Reuse: `is_url_shaped` hand-rolled a scheme / userinfo / port / IPv6 split when the crate already depends on a URL parser and already uses it for the same question in `fetch-archive`'s SSRF screen; the hand-rolled version accepted `https://:abc`, a port with no host. Every case was then verified against the parser rather than reasoned about, and two test *expectations* proved wrong rather than the implementation — `file://` is not URL-shaped, and `https:///acme/gov` is accepted with host `acme`. Both are now stated contracts in the scenario. (2) `BE-INPUT-011`: a TOML quoted alias may carry a newline, and a non-bare key is exactly what the first check rejects, so a raw rendering let committed config forge a second line in an operator-facing error; the rendering now escapes it while the carried field stays verbatim. Proven red before being trusted to pass.

Rules loaded: all 11 discovered files. Judged and not filed, with reasoning: `BE-ERR-001` — the error names the config path, which is the file the operator must edit and matches every sibling `PrimitiveError`; it is local CLI output, not a production response. `BE-LOG-002` / `CFG-ENV-006` — a rejected `repo` is echoed verbatim and could in principle carry userinfo credentials, but `CFG-ENV-005` is the rule on point for config validation and explicitly requires naming the offending value; the value is committed config shown to its own author. `QUAL-GROUND-001` — the bare-key grammar is encoded as a literal character class with no fail-loud binding to the TOML crate, which is compliant because the grammar is a contract this project owns and declares in `055`'s data-model. `BE-INPUT-004` — `path` gains no new filesystem use; the trust boundary stays where `config-sourced-paths-and-the-traversal-boundary` put it. `BE-INPUT-006` — the checks are single-pass over a local committed file with no regex, so no backtracking surface. `QUAL-CLAIM-001` — `validate` is total over the registry, and the change exists to remove a wrong-reason result.

**examined 10 of scope 41**, and what the other 31 are. Read in full: `runtime/src/schema/constitutions.rs`, `runtime/src/primitives/resolve_constitutions.rs`, `scenarios/the-constitutions-registry-validates-its-values.md`, `version`, `framework/commands/target.md`. Read in the entirety of the region that changed plus surrounding context: `runtime/src/primitives/mod.rs` (the whole `PrimitiveError` enum and `validate_slug`; the file is ~2300 lines), `runtime/CHANGELOG.md` (the 0.49.3 and 0.49.2 entries), `runtime/Cargo.toml` (version and `[dependencies]`), `framework/bootstrap/ductus.md` (§Validating the registry, the `constitutions.<alias>` schema, and the parity frontmatter; the file is 141KB), `specs/022-deterministic-runtime/spec.md` (frontmatter, section index, and all 31 acceptance criteria).

Not read, each with why. `framework/bootstrap/govern.md` — byte-identical mirror held by audit Family 21; `cmp` was run before and after the `cp` and both reported identical, which is grounds to believe it correct and is not a read. `.claude/commands/ductus/target.md` — generated from `framework/commands/target.md`, which was read in full; `gen-claude-commands.sh` re-ran and reported all 16 commands in sync. `.github/workflows/markdown-only-pipeline.yml` — **absent from the tree**; retired by `048-govern-acquired-runtime` and annotated on AC6/AC12, and it stays in scope because the plan lists it. `runtime/Cargo.lock` — a one-line version delta, verified instead by `cargo build --release --locked` succeeding after the `--offline` refresh. The remaining 26 are the plan's Affected Files at directory granularity (`runtime/src/interpreter/`, `runtime/src/mcp/`, `runtime/src/parser/`, `runtime/src/primitives/`, `runtime/src/schema/`, `runtime/tests/`, `runtime/tests/fixtures/`, `runtime/tests/golden/`, `runtime/tests/parity/`) plus pre-existing files this change does not touch (`README.md`, `framework/commands/{analyze,implement,plan,specify,status}.md`, `framework/runtime-tools.txt`, `runtime/.gitignore`, `runtime/legacy-prose-commands.txt`, `runtime/src/{io.rs,lib.rs,main.rs}`, `.github/workflows/{runtime,runtime-release}.yml`, `scripts/lint-procedure-parseability.sh`, `specs/022-deterministic-runtime/{plan.md,data-model.md,tasks.md}`) — 022's plan scopes the whole runtime, and this change is confined to two source files plus their error surface. Partial evidence about that remainder, which is not a read: the full suite passes across all 20 binaries (1344 lib + 1486 total), `cargo clippy -D warnings` is clean, and `scripts/audit/run-all.sh` passes after the commits, proven able to fail first by breaking the repo-root `version` file.

Scope measured on both bases after the reopen commit, per the campaign's step (5): natural base `a2050318` gives 14 modified-since / 41 scope; `--since HEAD` gives 0 / 33. The natural base was chosen — it covers the edits this pass made, which `HEAD` excludes by construction. Both collapse the 246 / 222 measured on the pre-reopen base on 2026-09-13, which is the window move step (5) predicts.

`022`'s `data-model.md` is deliberately unchanged: this adds no field and no result shape, `resolve-constitutions` has no entry there (its schema is canonical in `055`'s `data-model.md`), and adding a partial second home would be the drift §drift-prevention forbids. `055` is untouched and stays `done` — its AC6 scopes rejection to configuration time and is not falsified.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
