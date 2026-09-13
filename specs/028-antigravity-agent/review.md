---
spec: 028-antigravity-agent
reviewed-at: 2026-09-13T22:50:07Z
reviewed-against: 5a62a107e8c93fc2db4c45729ef5a166efdf5c1f
diff-base: 940ccee5382076c89a5e74ff49846448fbd343ae
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 10
skipped-passes: []
---

# Review — 028-antigravity-agent

## Summary

Backfill re-review under the examined/scope campaign. All five dimensions ran (security, reuse, quality, efficiency, simplicity) against 11 rule files discovered by `discover-rule-files`. 0 MUST / 0 SHOULD / 0 low-confidence **at this commit** — but that is a post-fix number, not a clean first pass: the pass found seven defects and every one was fixed before this review was recorded, in `9fc85a66` and `5a62a107`.

What was found and fixed. (1) **AC5 asserted behavior the tree contradicts.** It claimed ductus wires Antigravity via project-local `.agents/mcp_config.json`; `031-agent-mcp-wiring` reproduced that case against a spawning positive control and measured 0 server spawns and 0 MCP log references. Antigravity reads MCP servers only from home-level `~/.gemini/config/mcp_config.json`, and its registration `mechanism` is `surface-instruction`, so ductus writes no MCP file for it at all. The permission half (`mcp(ductus/*)` via `configure/antigravity.md:63`) still holds exactly, so AC5 is annotated **Half superseded** in the 046-AC22 style rather than rewritten — the wiring it describes is what this spec shipped in June, and rewriting it would record a fiction. AC11 named the same non-emitted file and carries a matching annotation. (2) Three body sites stated the superseded model as live fact (the Verified Antigravity Layout table, the Per-Agent Adoption MCP bullet, the divergences-table provenance note) and (3) the **MCP wiring + merge ownership** Resolved Question recorded a two-file decision 031 reversed — the class §drift-prevention *Decision resolution* names and nothing detects, since the prose carries none of the tokens an identifier sweep greps for. (4) **`framework/bootstrap/ductus.md`'s rendered completion message hardcoded the colon invocation for every agent.** The §Derived values Invocation row is layout-derived (`/{project}:<name>` claude-style, `/{project}-<name>` antigravity, `/{project}/<name>` opencode), so an Antigravity or OpenCode adopter finished `/ductus` and was handed six commands that do not exist on their agent — on the primary adoption path, as the last thing they read. `govern.md` re-synced byte-identical for audit Family 21. (5) `README.md:64` carried the same defect, listing Antigravity under the colon form. (6) `scripts/gen-configure-mcp.sh --help` printed `sed -n '2,21p'` while its per-host mapping had grown to line 24, so help output ended mid-sentence at "(covers" and dropped OpenCode entirely — the dangling-enumeration shape, verified by running it; its header also claimed the server prefix comes from "the adopter's `.mcp.json`", true only for Claude post-031. (7) `data-model.md` carried three stale claims: a two-valued `layout` enum where 032 added a third, the Profile-derived table still listing MCP-wiring as layout-derived, and a rows list omitting `opencode`.

Verified and holding: AC1 (§Derived values expresses all layout-derived dimensions and the procedure branches on `layout`, not agent name), AC2 (§Adding a new agent is the documented checklist), AC3 (`{config_dir}/skills/{project}-<name>/SKILL.md`, invoked `/{project}-{name}`), AC4 (`{config_dir}/rules/<name>.md`), AC6 (`configure/antigravity.md` plus the generator's emitted block, test D passing), AC7 (`ductus.md:792` — an Antigravity-only adoption ships no `CLAUDE.md`), AC8 (format isolation confirmed by reading both configure files: Claude carries `Bash(...)`/`Edit`/`mcp__ductus__*`, Antigravity only `command(...)`/`read_url(...)`/`mcp(ductus/*)`, with no leakage either way), AC9 (auto-detect lists registry entries whose `config_dir` exists), AC10 (README install table row). AC11's markdown half re-verified: `npx markdownlint-cli2` clean over 508 files.

Scope and coverage. `scope: 10`, `examined: 8`. Both bases were measured before choosing, per the campaign's step 5: the post-reopen natural base `940ccee5` gave 9 scope / 2 modified-since and `--since HEAD` gave 8 / 0. The natural base was taken because it covers the edits this pass made rather than excluding them — tighter *and* more honest — matching what 052 recorded. It has since resolved to 10 / 7 as this pass's own commits landed.

**Read in full (8):** `README.md`, `framework/bootstrap/configure/antigravity.md`, `framework/templates/project/gitignore`, `scripts/gen-configure-mcp.sh`, `scripts/tests/test-gen-configure-mcp.sh`, `specs/012-multi-agent-govern/spec.md`, `specs/028-antigravity-agent/data-model.md`, `specs/028-antigravity-agent/spec.md`.

**Not counted, and why.** `framework/bootstrap/ductus.md` (1182 lines) was read **in part** — the sections this spec governs: Instructions, Agent Registry, Derived values, MCP registration, Adding a new agent, Inputs, Pre-flight Checks, Agent Selection, Permission Setup, runtime auto-wiring, Pre-flight Phase, runtime detection State A, Shared Files, Per-Agent Scaffolding, Slash commands and cleanup, Antigravity layout, OpenCode layout, `ductus` self-installation, Placeholder Substitution, Post-Write Integrity Check, Post-Scaffolding Output, Idempotency, Directory Creation — roughly 600 of 1182 lines. Not opened: Pre-run Migrations, Project Configuration, File Fetching, Frontmatter Migration, Security Audit, Hook Installation, and the State-B runtime-acquisition detail, all owned by specs 048 / 027 / 019 / 040 / 018 rather than by 028. `framework/bootstrap/govern.md` was **not read**: it is the byte-identical mirror audit Family 21 pins, which I wrote with `cp` from `ductus.md` and proved identical with `cmp`. Believing it correct on that evidence and having read it are different claims, and `examined` is only the second.

Gates run at this commit: `cargo test --release --locked` (20 binaries, 1471 tests, 0 failures — `ductus.md` carries a `parity:` frontmatter block, so it is parity-relevant), `npx markdownlint-cli2` (508 files, 0 issues), `lint-procedure-parseability`, `shellcheck -S warning`, `gen-claude-commands.sh` (16 commands in sync), `derive-dependencies` (drift: false), and `scripts/audit/run-all.sh` after committing (exit 0, zero bytes — proven to redden earlier this session by breaking the repo-root `version` file, so the silence is evidence).

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
