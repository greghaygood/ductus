---
spec: 029-bootstrap-runtime-autowire
diff-base: 0d251acc13bb7455c068a3d4e8ddf88224899939
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T02:24:07Z
reviewed-against: 6554ab025b820f8fa65de8fe18c20a73d4580b6d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 15
scope: 17
reviewed-digest:
  scenarios/archive-fetch-direct-codeload.md: 8d72afc1592a147d1326817d6a6c9f97f3ccad2621d99e6d9a918905187cd24f
  scenarios/project-inputs-asked-once.md: 300da5f046c71060e85aed09ffeb1d1779f09fffa8816e0a4fc375b4dcfb9960
  scenarios/runtime-probe-parity-audit.md: f03e440661b5aa0e0f0e373fe9be513e6500454009bcfe8e478a80da83fa78e6
  scenarios/state-a-deterministic-path-forcing.md: dc0389b97152fba961f1b3b115279cb120b9ed3d78c7fc5a43834d45a0939577
blocking: false
---

# Review — 029-bootstrap-runtime-autowire

## Summary

0 MUST / 0 SHOULD / 0 low-confidence across all five dimensions, examined 15 of 17 in scope on diff base `0d251acc` (12 modified-since, plan affecting 8). Backfill re-review under the `examined`/digest campaign; the record it replaces predates `ductus-v0.49.0` and carried neither field.

Both bases were measured after the pass's own fix commits and both returned inline, so the base was chosen on the merits rather than on reading friction. The pre-reopen natural base resolved 549 modified-since / 551 in scope at 117,508 bytes; the reopen collapsed it to 12 / 17 at 4,911 bytes. `--since HEAD` gave 0 / 8 and was declined for excluding the nine files this pass edited.

Two shipped models the spec still described as current had been reversed, and both were adopter-visible. **MCP wiring is per-agent, not layout-derived**: §MCP Wiring, AC6 and AC16 named a per-layout path out of `ductus.md` §Derived values, which deliberately carries no MCP row since 031 made discovery a per-agent property; 032 then added OpenCode as a second `write-file` agent, and Antigravity is `surface-instruction` — it gets no MCP file written at all, and its config is the home-level `~/.gemini/config/mcp_config.json`. The Signpost named one agent on each side of a split that holds two. **State B no longer aborts**: §Pre-flight Phase, §State B, AC11, AC15 and the Post-completion note described a single combined pre-flight abort before the archive fetch, which 048's scenario `state-b-continues-in-session` removed. AC11 asserted the opposite of shipped behaviour — no archive fetched, no scaffolding — while ticked. README was correct throughout and is what the corrections were checked against.

Four durable contracts, all edited ahead of this review so the digest covers the final text. `state-a-deterministic-path-forcing.md` instructed a State-A run to call `substitute-templates` and `merge-claude-md`, both retired by 022 task 68 and absent from `framework/runtime-tools.txt`. `runtime-probe-parity-audit.md` said Family 15 hard-codes three agents where the script carries four. `project-inputs-asked-once.md` named State-B wiring as an abort cause and State C as live. Fourteen section references across the three named sections of `ductus.md` without naming the file on the reference's own line, so each resolved as a claim about the constitution; all four artifacts plus `plan.md` now report no unresolved references.

`plan.md` left the Antigravity probe grammar recorded as an open implement-time decision. It was decided — `command(which)` ships in both the registry seed and `configure/antigravity.md`, and Family 15 guards the pairing.

Not examined, and why. `framework/bootstrap/ductus.md` is 1,182 lines / 145KB and was read in the regions this pass turned on — the Agent Registry, Derived values, MCP registration, Permission Setup, runtime auto-wiring, runtime detection with States A and B, MCP wiring, the self-update stale path, the Pre-flight abort, the Closing restart, the runtime tip, the pinned advisory and the archive fetch — but not end to end, so it is named here rather than counted. `framework/bootstrap/govern.md` is a byte-identical `cp` mirror of it, confirmed with `cmp` and held by audit Family 21; a generated mirror is named, never folded into the numerator.

Security: the scope carries no application code, so the security surface is the permission grammar. The canonical sets reason explicitly about it — `configure/claude.md` records why a wildcard must never precede a git subcommand and why a path-scoped write entry grants nothing, and both files keep deny-side wildcards deliberately broad. No finding. Reuse, efficiency and simplicity: the two audit families share `lib.sh` and answer different questions; no duplication or complexity worth a finding. Quality: assessed against all three `quality-cross.md` rules — no stub, no unowned external contract, and both families distinguish a skip from a clean result.

One finding was captured rather than fixed, because its principal half lies in a durable contract under a spec the campaign has closed and standing operator decision (a) binds: 022's `framework-list-dedup.md` says it records an open question that its own Open Questions section does not carry, and two shipped configure files send adopters to that empty home. Recorded in `specs/inbox.md` with its measurement and three candidate shapes priced.

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
