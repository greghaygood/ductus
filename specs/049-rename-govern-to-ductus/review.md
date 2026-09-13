---
spec: 049-rename-govern-to-ductus
reviewed-at: 2026-09-13T13:40:49Z
reviewed-against: 2266ed4361305b3bdd0b6d8b41c89b6127c05adc
diff-base: 2266ed4361305b3bdd0b6d8b41c89b6127c05adc
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 21
skipped-passes: []
---

# Review — 049-rename-govern-to-ductus

## Summary

First review of 049 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. **One SHOULD found and fixed, not waived** — `must-violations: 0`, `should-violations: 0` here because the finding was resolved at source in `2266ed4` before this record was written, which is the disposition §implement-phase requires rather than carrying it to `done`.

**What this review read: 6 of the scope in full.** `framework/constitution.md`, `README.md`, `framework/migrations.toml`, `framework/migrations/ductus-rename.md`, `runtime/src/host.rs`, and `runtime/Cargo.toml`. **Read in part:** `runtime/src/schema/paths.rs` (its `config_path` / `session_path` / `newest_existing` ladder, which is what the criteria turn on, not the whole 628 lines). **Not examined:** `framework/bootstrap/govern.md` — the retired transitional alias audit Family 21 holds byte-identical to `ductus.md`, which *was* read in full — plus `.mcp.json`, `.github/workflows/runtime-release.yml`, `framework/migrations/govern-dir-consolidate.md`, and `specs/022-deterministic-runtime/`. **Seven scope entries are glob patterns** rather than paths (`framework/commands/*.md`, `runtime/src/**`, `scripts/audit/*.sh`, `specs/NNN-*/**` …) — the plan's Affected Files was authored with wildcards, so they resolve to nothing and are neither files read nor files missing.

**The quality pass over the two runtime files in scope found a `QUAL-CLAIM-001` SHOULD.** `Host::load_session_cli_config_dir` returned `None` for every failure of the per-contributor session file — missing, unreadable, or malformed — while its sibling `load_host_block` logged a warning to stderr on exactly the same parse failure. The asymmetry is the defect: a corrupt session file was indistinguishable from an absent one, so resolution fell through to the default `.claude`, and the mistake surfaced far from its cause as a "command file not found" for an agent the contributor does in fact use. That is the rule's own shape — a fully-implemented path whose output overstates what it verified — and the fallback chain is what makes it quiet rather than loud. Fixed: a missing file stays silent, since not having one is the ordinary state, while a file that exists and does not parse now names itself and the parse error. Behaviour is otherwise unchanged and the existing `malformed_session_falls_back_to_legacy_then_default` test still passes untouched.

**AC1 was violated in `runtime/src/`, which the criterion names explicitly.** `host.rs`'s module doc and four function docs said the runtime reads `project` from `.govern.toml` and `cli-config-dir` from `.govern.session.toml` — naming the **legacy tier as the current source**. The code has resolved through `schema::paths`' newest-wins ladder (`.ductus/` → `.govern/` → the pre-042 root files) since this very spec, so only the prose lagged. This is the case AGENTS.md names as where a superseded rationale hides best: the diff that falsified the comment is the same diff a reviewer reads it inside of. `schema/paths.rs` also carried a `/gov:review` reference under the retired command namespace. Both swept.

**AC1's surviving occurrences were measured, not eyeballed.** 324 `govern`/`gvrn` matches remain across the artifact set the criterion names. Sampling them by category: the English verb ("rules that **govern** this project", "a class of behavior the framework should **govern**"), spec-directory slugs that genuinely contain the word (`048-govern-acquired-runtime` is a real path), the migration procedures and registry entries whose subject *is* the rename and which §drift-prevention requires to name both sides, `framework/bootstrap/govern.md` as the retired alias Family 21 pins, and the config-resolution ladder's legacy tiers in the command bodies. Each is load-bearing or historical; none is a live artifact asserting current behaviour under the old name — after the two `runtime/src/` fixes above, which were.

**How the rest were checked.** AC2: the sweep was a uniform substitution and no spec drifted by it — `check-artifacts` reports clean across the corpus. AC3, AC4, AC11: `ductus-rename.md`'s procedure carries the idempotency check as step 1 (six tells, exit silently if none), the convergence rule applied to every move, and the registry-ordering argument that makes a pre-042 adopter converge in a single run — `govern-dir-consolidate` then `constitution-relocate` then this, each hop's output being the next hop's input, with step 3 stating exactly why following the earlier procedures literally would otherwise leave a dangling reference. AC5: the runtime reads `.ductus/` first and falls back, so an adopter who upgrades the binary before re-running the bootstrap is not broken — verified in `paths.rs`'s `newest_existing`, not just asserted in the migration prose. AC6, AC12: published tags and CHANGELOG entries are left as written, and the retired crate is deprecated rather than yanked. AC7: the audit's parity families pass with none disabled — `run-all.sh` exits 0, verified this session with a positive control proving it fails when it should. AC8: `cargo test --release --locked` is green across 20 binaries and the generated command copies regenerate clean. AC9: `README.md` describes acquiring, registering and invoking under the new name only — zero old-name matches. AC10: the contributor-local checklist is in both `AGENTS.md` and the constitution's §drift-prevention. AC13: the version series continued rather than restarting, and Family 20 passes across all three sites, now at 0.49.1.

**On the diff base.** No commit records 049 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 049 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
