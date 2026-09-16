---
spec: 031-agent-mcp-wiring
diff-base: dd8ce2a3d8c4b5df6ba16a6e3791fe75d9062e73
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T01:07:04Z
reviewed-against: f98ed3ed82b4a430473e3d3484bd8146b2d88ffb
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 10
reviewed-digest:
  data-model.md: beece633e378d2fb138ce082c8586779d8689fcedc7dbedc4db07162a9c05161
  scenarios/antigravity-mcp-verification.md: a89235da2c892bceac8befda8f988b829c755778e73eea62ba33e436f9008666
blocking: false
---

# Review — 031-agent-mcp-wiring

## Summary

Re-run 2026-09-13 as the examined/scope backfill pass. The prior record (2026-08-28) predated `ductus-v0.49.0` and carried no `examined` and no `reviewed-digest`, so its `0/0/0` was indistinguishable from a run whose five passes never fired. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking.

**Scope, and what was actually read.** The natural diff base collapsed from 813 modified-since / 814 in scope to **3 / 10** once this pass's correction commit recorded a fresh `in-progress` transition. `--since HEAD` gave 0 / 8 and was declined: it excludes by construction the three files this pass edited. Examined **8 of 10**. The two that were not read end to end are named here rather than folded into the numerator:

- `framework/bootstrap/ductus.md` (1182 lines) was read in full through line 385 — §Instructions, §Agent Registry, §Derived values, §MCP registration, §Adding a new agent, §Permission Setup, §Pre-flight Phase, §ductus runtime detection with States A and B, §Runtime acquisition, §Pointer materialization — plus §MCP wiring, and targeted reads at the self-update comparison, the Antigravity skill scaffolding, the self-install path and the closing-restart notice. That range carries the whole of this spec's subject. Lines 386–1182 (archive fetch, migrations, per-agent scaffolding, placeholder substitution) were not read end to end, so the file is not counted as examined.
- `framework/commands/{target,status,analyze,implement,audit,specify,plan,ask}.md` is a brace literal that resolves to no file on disk, and one of the eight names it spells, `ask.md`, was renamed to `amend.md` in `7dd6698f` on 2026-06-22 — five days after this spec was created. It stays in scope because the plan lists it. Task 8's requirement was verified another way: the phrase it replaced is absent repo-wide, and the host-generic statement now lives once in the constitution §runtime-host-integration plus `framework/bootstrap/ductus.md` line 22, rather than in eight command preambles.

**Rule coverage.** All 11 rule files were loaded and all 105 IDs enumerated. None has a subject in this scope, which is markdown plus one TOML registry with no code: the backend and frontend families have nothing to bind to, and `QUAL-STUB-001` / `QUAL-GROUND-001` / `QUAL-CLAIM-001` govern code paths. `QUAL-CLAIM-001` was checked rather than assumed inapplicable, against this spec's own verification scenario: `antigravity-mcp-verification` ran a positive control that spawned before concluding from a negative result, and recorded the quota-exhaustion confound together with why it cannot affect the outcome. That is the rule's own distinction, met.

**The SIMPLICITY waiver expired, and the expiry is correct rather than incidental.** It excused a finding that `scope` was descriptive metadata derivable from `target`, on the grounds that only `mechanism` drove State-B branching. That is no longer true: §MCP wiring now splits on `scope` directly — a `project-committed` target names the repo-relative pointer, a `user-global` or `home-level` target names the absolute store path — so `scope` carries behavior and the simplicity finding does not fire. `process-waivers` ran unrestricted with no dimension skipped and reported it expired, 0 applied and 0 retained. The waiver is gone because what it excused is gone.

**Five corrections landed in this pass**, all factual, none a rename. The surfaced Auggie registration command was stale in three places (AC7, the Auggie-registration Resolved Question, and `data-model.md`), all quoting `--command ductus` — a bare command that `048-govern-acquired-runtime` stopped resolving when it moved the runtime into the ductus-owned store; `framework/migrations/runtime-store-path.md` exists to repoint adopters off exactly that form. §Out of Scope deferred an Antigravity skills/settings verification, claimed it was tracked, and pointed at an "Open Question below" that is a Resolved Question — nothing tracked it, and `028-antigravity-agent` had already performed that verification on 2026-06-09, eight days before this spec was created. `data-model.md`'s "Server entry shape (unchanged across all agents)" went false when `032-opencode-agent` added an agent using an `mcp` map of typed local-server entries. 032 also added a fourth row to the per-agent descriptor introduced here while signposting only 028, so this spec now carries the matching Signpost (post-032). Seven dead `§` references in `plan.md` and `data-model.md` were qualified; `tasks.md` keeps four deliberately, since §tasks-phase makes it ephemeral and prunable and its entries are spent.

**One finding in scope that is not this spec's to fix.** `specs/029-bootstrap-runtime-autowire/spec.md` §MCP Wiring, AC6 and AC16 still state the superseded layout-derived model as current ("`.mcp.json` for `claude-style`, `{config_dir}/mcp_config.json` for `antigravity`"), and its Signpost (post-031) — which this spec's task 4 correctly placed — sits about 60 lines above them and says only `write-file` agents are Claude, which 032 made false. 029 is the third unit on the backfill campaign's Remaining list, so it is already queued; editing it here would reopen it, stale its review, and duplicate the work its own pass does.

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
