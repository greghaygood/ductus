---
spec: 032-opencode-agent
reviewed-at: 2026-09-14T00:18:43Z
reviewed-against: a5ca20970476a50563d11f811fffee2a079696a5
diff-base: 3c423743ca68c8d86db08582268c4923bb18f6fd
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 11
skipped-passes: []
---

# Review — 032-opencode-agent

## Summary

Five passes (security, reuse, quality, efficiency, simplicity) over the resolved scope of 11 files; examined 9. 0 MUST, 0 SHOULD, 0 low-confidence stand at this HEAD — seven defects were found and all seven were fixed in the pass, across commits 3c423743, 6350907f, c63c1b42 and a5ca2097. No finding mapped to a loaded rule (all are prose/contract drift, not code-path violations), so none is recorded as a rule finding, and none was captured to the inbox: every one lay inside the spec that was open, which AGENTS.md §Workflow's inbox rule says to fix rather than capture.

Diff base. Measured on all three bases before choosing, per AGENTS.md §Gotchas. Pre-reopen natural base 5fdd7390 (032's own planning commit) resolved 811 modified-since / 812 in scope against a plan affecting 9 — a window spanning months for a spec whose surface is nine files, which is the case the --since override exists for. `--since HEAD` gave 0 / 9. Neither was taken: the step-4 reopen moved the natural base forward to 3c423743 (the reopen commit's parent), collapsing it to 5 / 11. That is both tighter than the pre-reopen base and more honest than HEAD, because it covers the five files this pass edited, which HEAD excludes by construction. Recorded against 3c423743.

Read in full (9): README.md (314 lines), framework/bootstrap/configure/opencode.md (85), framework/bootstrap/ductus.md (all 1182), install.sh (339), scripts/gen-configure-mcp.sh (178), scripts/tests/test-gen-configure-mcp.sh (74), specs/028-antigravity-agent/spec.md (343), specs/032-opencode-agent/data-model.md (129), specs/032-opencode-agent/spec.md (296).

Not examined (2), named individually rather than folded into the numerator. framework/bootstrap/govern.md — the retired transitional bootstrap path, held byte-identical to ductus.md by audit Family 21; it was written by `cp` from ductus.md in this pass and proven with `cmp` (exit 0), which is grounds to believe it correct and is not a read. framework/templates/project/gitignore.md — absent from the tree; the file is `framework/templates/project/gitignore` with no extension, renamed by 8558ac8b long before this spec. It stays in scope because plan.md lists it, and is named here as absent rather than silently dropped. The installer references the real name correctly at §Shared files with conflict handling, so nothing is broken; only the plan's affected-files entry carries the retired name, and plan.md is not a durable contract.

Defects found and fixed. (1) 049's rename sweep half-applied to the OpenCode namespacing example: its map carried `command/gov/` to `command/ductus/`, so it rewrote the path but left the bare key `gov/specify` on the same line — six occurrences (spec.md x4, plan.md x1, ductus.md x1) asserting that a file under a `ductus/` subdirectory registers under a `gov/` key, false under the very subdirectory-namespacing rule the sentence illustrates. (2) data-model.md §Region 2 depicted the `bash` permission map with the broad `"*": "ask"` last; under OpenCode's last-matching-rule evaluation, which the note two lines below states, that shadows every allow and downgrades the `rm -rf *` deny to an ask. configure/opencode.md and spec.md both had the order right, so this was doc-vs-shipped drift with no behavioral surface. (3)-(7) Five layout-derived values stated as claude-style universals — the drift class AGENTS.md §Workflow's per-agent entry names, where the row-by-row walk covers what the scaffolding writes and not what the framework says: §Pinned `ductus.md` advisory enumerated two of three install paths (028 added the antigravity branch; 032 added the `opencode` layout and updated five sibling enumerations but missed this one); §Security audit summary rendered `/{project}:groom` with no per-agent rendering instruction, a sibling of the 028 defect in a block 028's pass did not reach; §Instructions steps 1 and 7 and §Project Configuration said the runtime resolves one command path where `host.rs`'s `command_file_candidates` resolves two; §Instructions step 9 named `{cli-config-dir}/commands/ductus.md` as the self-install entry, which for an OpenCode adopter is the wrong directory; and §Edge Cases keyed its settings-file bullet to the claude-style filename. Separately, scripts/gen-configure-mcp.sh's header said the invariant covers "both agents'" sources when there are four — user-facing, since `--help` prints those lines verbatim.

Examined and judged correct as written, recorded so the disposition is explicit rather than silent. data-model.md's `rules_file_note` quotes the registry value as "no second rules file." where the shipped row reads "no second rules file is needed." — a two-word paraphrase in a design record, identical in substance, so rewriting it would churn a durable contract for no reader benefit. data-model.md's 12-pattern bootstrap seed against the registry row's 19 is covered by the document's own statement that the exact bash allow/deny patterns are finalized at implement and the shape above is the contract. 028's AC1 and §Registry Generalization describe a two-layout registry, which is what 028 delivered; its post-032 signpost carries the extension and is intact and accurate. README's Quick start shows the colon form, but line 24 scopes that walkthrough to Claude Code explicitly and line 64 states all three invocation forms before the command list — 028's pass landed that and it holds. All 11 acceptance criteria were verified against the tree, enumerations included; all hold. §Out of Scope and the ten Resolved Questions were read as live claims: Resolved Q5 (generic JSON merge, not a `merge-permissions` extension) still holds — `merge_permissions.rs` has zero opencode references. `resolve-anchor` reports `unresolved: []`.

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
