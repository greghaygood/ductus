---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-15T02:17:36Z
reviewed-against: e9d345ae25fc2762b342e7c97e3df77f458fc555
diff-base: 950bcc76b45bf3b3c9b5b67779e7a8bc6e57c253
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 18
scope: 60
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s) — recorded **after** two MUST-class defects this pass found in its own work and fixed, not instead of them. Five passes over the reopened window (base `950bcc76`, 31 modified-since / 60 in scope at 5,645 bytes, collapsed 26x from the pre-reopen 144 / 167 at 144,932 bytes; `--since HEAD` gave 0 / 33 and was declined for excluding this pass's own edits). Rule files: 11 discovered and loaded.

**Both defects were introduced by this pass and found by probe, not by reading.** Delegating `derive_dependencies::leading_slug` and `derive_references::is_spec_slug` to the shared `parse_feature_dir` widened the *form* correctly and widened the *charset* by accident, because `parse_sequential` deliberately leaves a sequential slug's charset open — it recognizes directories already on disk, while these call sites parse untrusted link text and write it verbatim into frontmatter. (1) A body containing `[x](../001-evil], status: done, [y/spec.md)` rendered `dependencies: [001-evil], status: done, [y, …]`, splicing a second key onto the line. (2) Fixing that by terminating the candidate on `/` alone then dropped a *legitimate* edge: `](../002-b)`, a directory-only sibling link, yielded `002-b)` and harvested nothing — the same silent-drop class the pass exists to fix. Both are closed by scanning to the first byte outside the feature-name charset, which restores the predecessor's boundary exactly while leaving the form rule widened; both are pinned by regression tests carrying the probes' own inputs. Neither was visible in this corpus — `derive-dependencies` reports `drift: false, updated: 0` across all 54 specs before and after, because no spec here carries a directory-only sibling link.

**Examined 18 of 60, and what was not read is named rather than folded into the numerator.** Read in full or over their whole changed surface: `derive_dependencies.rs`, `derive_references.rs`, `write_review.rs`, `write_analysis.rs`, `check_artifacts.rs`, the two changed structs in `schema/primitives.rs`, the three new scenarios, and eight edited contracts (`unreadable-scenario-is-reported`, `framework-list-dedup`, `criterion-path-existence-family`, `adopter-generator-promotion`, `apply-manifest-substitution-contract`, `cli-config-dir-per-contributor`, `dashboard-primitive`, `orphaned-reference-check`). **Not counted:** `.claude/commands/ductus/analyze.md` is a generated mirror — the generator was re-run and reported all 16 commands in sync, which is grounds to believe it correct and is not a read; `framework/commands/analyze.md` was read at steps 16–17 only; `data-model.md` (1,349 lines) at its changed entries only; `spec.md` and `tasks.md` at frontmatter and appended tasks only; `Cargo.lock` as a four-line version diff; the six remaining edited scenarios at their edited regions only. **In scope and absent:** `.github/workflows/markdown-only-pipeline.yml`, removed by 048 — it stays in scope because the plan lists it. Several other scope entries are directories from the plan's Affected Files rather than files.

**Also verified rather than assumed:** the `empty-scope` change leaves the honest empty review byte-identical (the rendering branch is untouched and still selects the empty-scope Summary); `write-analysis`'s new `captured-issues` renders a count, never the issue text, so it carries no injection surface of its own; and `find_spec_segment` bounds its slug with `/` on both sides and requires a `spec.md` tail, so `derive_references` needed the charset guard but no boundary change.

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
