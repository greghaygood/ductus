---
spec: 054-remove-supersession
reviewed-at: 2026-09-13T13:44:25Z
reviewed-against: 089ba25279c50730340592d9da3b40421ae08279
diff-base: 089ba25279c50730340592d9da3b40421ae08279
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 11
scope: 37
skipped-passes: []
---

# Review — 054-remove-supersession

## Summary

First review of 054 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All twenty-six criteria verified against the tree; **one pinned a count that has since moved**.

**What this review read: 11 of the 37 files in scope, and how the rest were treated.** Read in full: `framework/constitution.md`, `framework/commands/analyze.md`, `consolidate.md`, `help.md`, `specify.md`, `framework/bootstrap/ductus.md`, `framework/runtime-tools.txt`, `README.md`, `AGENTS.md`, `docs/slash-commands.md`, and `runtime/src/schema/paths.rs`'s resolver. **Checked for the specific property the criteria assert, not read line by line:** the nine `runtime/src/` files — `main.rs`, `mcp/server.rs`, `interpreter/{mod,payload}.rs`, `primitives/{mod,read_spec,validate_frontmatter,check_artifacts}.rs`, `schema/{extensions,primitives,registry}.rs`. 054 is a **removal** spec, so what its criteria claim about those files is *absence*, and absence is verified by exhaustive search rather than by reading — each was searched for every removed identifier, and each of the five registration sites AGENTS.md names returned zero. Reading them as new code would not have tested the claim. **Not examined:** `framework/bootstrap/govern.md` (the alias Family 21 pins byte-identical to `ductus.md`, which was read), the two `configure/` permission files, `scripts/audit/README.md`, `readme-command-parity.sh`, `gen-help-tables.sh`, `runtime/CHANGELOG.md`, `Cargo.toml`, the `specify-basic` golden, `two_spec_commands.rs`, 026's scenario, and 052's spec. **Four scope entries do not exist** — `supersede.md`, both removed primitives, and 053's directory — which is exactly what AC1, AC7 and AC18 assert.

**AC10 pinned a count that moved.** It claims `check-artifacts` "runs **eight** residual deterministic families". The runtime emits **nine**: `analyze-state-drift`, `artifact-completeness`, `criterion-labels`, `criterion-path-existence`, `link-adjacent-drift`, `review-state-drift`, `scenario-consistency`, `scenario-open-questions`, `task-consistency`. `analyze-state-drift` arrived later, with the durable `analyze:` record. The *requirement* AC10 actually states is untouched and holds exactly — `supersession-reciprocity` is returned by no code path, and `analyze.md`'s enumeration and count match the implementation, both saying nine. Only the literal number was a snapshot, which is what a count inside a criterion always is.

**AC13 was checked and deliberately not "fixed".** It requires every hardcoded command count in `ductus.md` and `govern.md` to read sixteen rather than seventeen, and both files contain the string "17 commands". That string sits inside a **past-incident narrative** — the account of 370 literal placeholders shipping across "the constitution, all 17 commands, 5 rule files and 2 templates" — where seventeen was the correct count at the time. The installer manifest itself carries exactly **16** slash-command rows, which is what the criterion is about. Rewriting the narrative to say sixteen would falsify an incident record, the same damage an earlier sweep did to a rename mapping elsewhere in this corpus.

**The absence claims, verified as absence.** A single search across `framework/`, `runtime/src/`, `scripts/`, `docs/`, `README.md`, `AGENTS.md` and the installed command directory for `supersede.md`, `/{project}:supersede`, `--supersedes`, `supersedes:`, `§supersession-annotations`, `write-supersession-annotation`, `read-supersession-pair`, `supersession-reciprocity`, `classifyClaims` and `blockquote_cites` returns **zero live references** — AC1, AC2, AC19, AC23 in one pass. AC7's five registration sites (`runtime-tools.txt`, `registry.rs`, `main.rs`, `interpreter/mod.rs`, `mcp/server.rs`) each return zero, which is the de-registration discipline AGENTS.md records as five sites of which two are found only by tests. AC18: 053's directory is gone and the surviving `053-` mentions are narrative inside 054's own plan — a design record of what was consolidated, not inbound pointers.

**How the rest were checked.** AC3, AC4, AC26: `specify.md` accepts no `--supersedes` and offers no supersession answer; `consolidate.md`'s decision table asks whether the earlier spec still describes something true; `docs/slash-commands.md` states the two-spec rule naming `/fold` and `/consolidate` with no exception. AC5: the constitution carries no `§supersession-annotations`, no supersession bullet in §spec-requirements, and no `supersedes` row in the frontmatter schema — checked against the table directly, since an absent row is easy to assume. AC6: `check-corpus-links` runs clean in the pre-commit hook on every commit this session, and `resolve-anchor` reports no unresolved reference. AC11, AC15: `cargo test --release --locked` is green across 20 binaries with clippy `-D warnings` clean, and `run-all.sh` exits 0 — verified this session with a positive control proving it fails when it should. AC12: `help.md` carries sixteen project-namespaced command rows outside the Bootstrap category, and neither it nor `gen-help-tables.sh` names supersede. AC16: `migrations.toml` gained no entry — the installed command is removed by the existing slash-command cleanup instead. AC21: the surviving ordinary-English uses of "supersede" name no capability. AC22, AC25: 052 is `done` and retains its consolidation content; `validate-frontmatter` reports `clean` on this corpus with no supersession path left to fire.

**On the diff base.** No commit records 054 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 054 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
