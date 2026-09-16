---
spec: 045-decision-state-drift-detection
diff-base: 877eab5f675d09724a65390ca2ee3c161b8081f9
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T02:39:20Z
reviewed-against: de98121ca2fb53c7a226e5c31463d46c0fff30cc
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 10
scope: 19
reviewed-digest:
  data-model.md: 18ef73385daae1bdb2e92eec3bbdb16109ac66c6509cc8d7903e6dd515ee2438
blocking: false
---

# Review — 045-decision-state-drift-detection

## Summary

Backfill pass over a review record written before `ductus-v0.49.0`: it carried no `examined`, no `scope` and no `reviewed-digest`, so freshness read as undeterminable and a run that skipped its passes would have been byte-identical to one that did not. All five passes ran against the resolved scope. No MUST, SHOULD or low-confidence findings.

**Scope and base.** Both bases were measured after this pass's own correction commit and recorded before choosing. The natural base `877eab5f` resolves 5 modified-since / 19 in scope against a plan affecting 17; `--since HEAD` (`de98121c`) gives 0 / 17. The natural base was taken because it covers the five files this pass edited, which HEAD excludes by construction. The pre-reopen natural base was `c27a08f4` at 397 / 402 over a 96,366-byte payload; the reopen collapsed it to 1,997 bytes, so both legs returned inline and the base was chosen on the merits rather than on which one was readable.

**examined 10 of 19.** Read in full: this spec's `spec.md`, `plan.md` and `data-model.md`; `framework/commands/analyze.md`; `framework/constitution.md`; `runtime/Cargo.toml`; and 022's four scenarios carrying this spec's runtime work. Named rather than counted, each with what was relied on instead: `.claude/commands/ductus/analyze.md` is a generated mirror of a source read in full, and the generator re-ran in this pass reporting all 16 command copies in sync; `runtime/src/primitives/check_artifacts.rs` was read across its module header and the whole of both 045 families with their helpers, but not its test module at lines 1631-3518; `runtime/src/primitives/mod.rs` was read at `split_blocks` and `MarkdownBlock` only, `runtime/src/schema/primitives.rs` at `SkippedTarget` and `ArtifactFinding` only, and `runtime/src/mcp/server.rs` at the `check-artifacts` tool description only; `specs/022-deterministic-runtime/data-model.md` was read at its `check-artifacts` section only; and `runtime/CHANGELOG.md`, 022's `spec.md` and 022's `tasks.md` were not opened at all.

**Corrections this pass made**, committed in `de98121c` ahead of this review so the digest covers them. The canonical `data-model.md` called its `SkippedTarget` reason set closed at six where the runtime emits seven — the missing `not-a-live-claim` being the one this spec's own analyze record counts, and the one `analyze.md` and 022's registry both already carried, making the canonical copy the outlier. `ArtifactFinding` was cited at `primitives.rs:2189` against a declaration that sits at `:3430`. A `§Behavior` reference named no document on its own line, so it read as a claim about the constitution. AC18, the Motivation table and the data-model's worked example each attributed both of the originating case's deletions to `531e3ea`, which deleted only `framework/workflows/registry.json`; `scripts/audit/registry-equivalence.sh` went in `3ff65445`. `plan.md` restated the path grammar in four bullets where the canonical table has six, and now points at it. And `analyze.md` gained the third `review-state-drift` condition that task 15 shipped and never documented — the omission that had left that section's own closing paragraph referring to a count no bullet introduced.

**Verified rather than assumed.** The six tells, their classes and their order match `TELLS` exactly; the fourteen non-assertion phrases match `NON_ASSERTION_MARKERS`; the path grammar matches `is_path_like` clause for clause; all four exempt contexts are implemented, the blockquote in `split_blocks` and the code span in `contains_outside_code`; AC18's fixture pins both paths in their present-tense form; and AC10's shared promotion criterion is present in `analyze.md`. Reuse and simplicity are sound by design rather than by accident — `inline_code_spans` is promoted once and shared by both families, criteria come from `read-spec` rather than a second section walker, `list_scenario_files` supplies the scanned set, and `SkipScanner` is deliberately left alone so the four task parsers that share it do not change how they read a quoted line. Efficiency likewise: the adopter-destination manifest is read once per feature rather than per candidate.

One thing examined and judged not a finding: the path grammar admits a parent-directory segment, since `is_path_like` rejects a leading `/` but not a `..` component, so a candidate could reach `repo.join` above the repo root. The operation there is `exists` alone and never an open, the sibling family guards the case that does open files through lexical containment plus a symlink test, and a sweep of every backticked span in every spec's acceptance criteria found zero such candidates.

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

- convention: 022's `check-artifacts` registry has fallen behind shipped behaviour in three measured places, and the runtime's own module doc in a fourth. `specs/022-deterministic-runtime/data-model.md:1011` says *Eight families* where nine ship, and `analyze-state-drift` (added by 047) appears nowhere in that file — 0 occurrences. The same line glosses `review-state-drift` as two conditions where three ship; the third is the non-zero `should-violations` check 045's task 15 added, which this pass documented in `analyze.md`. And `specs/022-deterministic-runtime/scenarios/criterion-path-existence-family.md`'s final Edge Case still says in the present tense that an adopter-layout path whose top-level segment also exists here *still flags*, which `ships_to_adopter` (landed in `6e37efa5`, under 022 itself) changed to a `ships-to-adopter` skip. `runtime/src/primitives/check_artifacts.rs`'s module doc carries the same two-condition `review-state-drift` gloss, and quotes 045's AC18 with the single-commit attribution this pass corrected. Cost, which is what sets the disposition: the first three are one 022 reopen, and the scenario is a durable contract, so they oblige a full five-pass re-review of the corpus's largest spec — standing operator decision (a) keeps that out of this pass, and the campaign item already records a full 022 re-review as its own unit. The fourth is a `runtime/` doc-comment edit and therefore carries a version bump and a `ductus-v<version>` tag. — `specs/022-deterministic-runtime/data-model.md:1011`

## Skipped passes

*None.*

## Unexamined governance

*None.*
