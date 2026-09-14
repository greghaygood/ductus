---
spec: 040-configurable-specs-dir
reviewed-at: 2026-09-14T17:23:24Z
reviewed-against: 58124bbf3864074c30307e7b8af90462ee07e598
diff-base: 20c49dc25a44960624dcba76548f5d6b47b28fee
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 5
scope: 28
skipped-passes: []
---

# Review — 040-configurable-specs-dir

## Summary

Clean across all five passes — 0 MUST, 0 SHOULD, 0 low-confidence — over a scope of 28, of which **4 were read in full** plus one directory entry read to its 040-relevant extent (see the coverage boundary below). This is a backfill pass: the prior record predates `ductus-v0.49.0`, so it carried no `examined`, no `scope` and no `reviewed-digest`, and the pre-`done` gate reported its freshness as undeterminable.

**Both diff bases measured after the step-4 commit, and the natural base taken.** Natural base `20c49dc2` (the reopen commit's parent): 3 modified-since / 28 in scope, plan-affected 25, payload 2,033 bytes. `--since HEAD` (`58124bbf`): 0 modified-since / 25 in scope, 1,700 bytes — declined, because it excludes by construction the three files this pass edited. Measured before the reopen, 040's natural base was **478 in scope at 120,712 bytes**, the first window in the campaign that exceeds the MCP output cap outright; the step-4 commit collapsed it by a factor of 59. Both legs returned inline post-reopen, so the CLI-and-`jq` route was needed only for the pre-reopen measurement — the eighth pass running to confirm that friction is a property of the pre-reopen window and never touches the leg that decides the base.

**Four corrections, all made in the pass; nothing captured, so the inbox held at 11.** Each was a decision the tree had already overtaken, and no check could see any of them.

**AC11 was falsified by this spec's own task 12, inside the commit its own review was recorded against.** It promises the constitution carries "a single one-line note"; `830e42a0` wrote **two** paragraphs into §spec-phase — the descriptive note plus the imperative substitution rule the six deleted command blockquotes had been supplying — and added a canonical-sources row. The sibling scenario prescribed exactly that and preserved AC11's substantive holding, which still verifies; nobody updated its literal one. The prior review Summary records "AC11's substantive holding is preserved exactly" without noticing the other half had moved, so the tell sat in the record for four weeks. Annotated rather than restated, in the 009-AC18 / 046-AC22 style.

**The strict `[A-Za-z0-9_-]` charset was justified at three sites by a consumer that no longer exists.** The spec says the name must stay regex-safe because "the bash generators interpolate it into `grep`/`awk` regexes". Spec 022's `adopter-generator-promotion` replaced those generators with the `derive-dependencies` and `derive-references` primitives and deleted the `lib/specs-root.sh` helper they shared, and `framework/bootstrap/hooks/ductus-pre-commit` now matches the spec root by **shape** rather than interpolating the configured name — so no shell consumer interpolates it. The outlier was the correct copy: `runtime/src/schema/paths.rs` already carries both the retirement and the three reasons the strictness is kept, while the spec pointed the reader at the 040 review for the retired one. Those surviving reasons are now what the spec states.

**`substitute-templates` is named as a live full-path primitive at three sites** (spec §Resolution behavior, its matching Resolved Question, and `plan.md`). 022 retired it and it is absent from `framework/runtime-tools.txt`.

**A durable contract still prescribes a mechanism its sibling retired.** `scenarios/command-prose-resolves-spec-root.md` §Behavior offers the blanket note — the `specify.md:60` blockquote placed near the top of each command's Instructions — as one of two live means. `spec-root-rule-stated-once` deleted all six; `git grep 'Spec-root resolution' -- framework/commands/` is empty and eight command sources now carry a `§spec-phase (spec-root resolution)` Reference-line pointer instead. The requirement is unchanged — a command acting on a spec-root path still resolves it — so only the carrier was corrected.

`plan.md` additionally described the two deleted generators and the retired markdown-only opt-in CI job in the present tense; 048 replaced that job's invariant with the acquisition invariant, exercised by `.github/workflows/runtime-acquisition.yml`.

**Verified rather than assumed.** AC13 was checked by **probe**, not by reading: a scratch repo configured `specs-root = "governance"` returns `feature directory not found: governance/040-foo` from `read-spec`, `set-status`, `read-tasks` and `traverse-deps` — the criterion's own example string, with no hardcoded prefix. AC3's charset matches `validate_specs_root` character for character. AC7 holds under the current pointer mechanism, confirmed by the empty corpus-wide grep above plus the six Reference lines read individually. AC10's "one shared helper" holds across **35** call sites; `resolve-references` reaches it through `paths::specs_dir` rather than `Paths::load` and was nearly misfiled as an exception before the second entry point was read. AC6 holds — `.claude/commands/ductus/init.md` scaffolds `inbox.md`, `rules/` and the shared docs under `{spec-root}`. Task 3's claim that the two lints need no change holds: `lint-rule-ids.sh` walks `framework/rules/`, and `lint-frontmatter.sh` is ductus-CI-only with no manifest row.

**The efficiency finding that was measured and then dropped.** `specs_dir` re-reads the project config on every call, and `resolve_references` calls it per reference rather than per invocation. Measured rather than filed: this corpus carries **0** specs with a `references:` block and **0** registered services, the bound is a spec's cross-service link list either way, and per-checkout resolution is required for correctness because each service may configure its own root. Every other call site hoists once per invocation. Not a finding at any severity.

**Coverage boundary — what was read, and what was not.** Read in full: `framework/constitution.md` (761 lines), and 040's own `spec.md`, `plan.md` and `scenarios/command-prose-resolves-spec-root.md`. The scope entry `runtime/src/schema/` is a **directory of ten files**; `paths.rs` is the one the plan's row names as this spec's subject and its entire non-test portion (315 lines) plus its full 20-test inventory were read, while the other nine carry zero `specs_root` or `specs_dir` references — verified by grep, not assumed. That entry is counted; the following are **not**, and none was folded into the numerator. `framework/bootstrap/ductus.md` (141KB) — only §Collect Project Inputs' spec-root step, the update-mode check, the config schema block and the manifest substitution note were read. `README.md` — §Configuration only. `.claude/commands/ductus/init.md` — its spec-root sites only. The eleven `runtime/src/primitives/*.rs` and `interpreter/payload.rs` entries — each read only at its resolution call site and the surrounding error path, which is the whole of what this spec changed in them, but not the whole file. `runtime/tests/` — the renamed-root coverage was enumerated by test name (`runtime/tests/specs_root_override.rs`, 352 lines, 13 integration tests, including `error_messages_name_the_configured_root`) and the bodies were not read. `scripts/lint-frontmatter.sh` and `scripts/lint-rule-ids.sh` — their walk targets only. `scripts/tests/` — listed; it holds two files and neither is 040's, because the renamed-root generator fixture task 3 records went with the generators it exercised.

**Five in-scope paths do not resolve, and each stays in scope because `plan.md` lists it.** `constitution.md` at the repo root never existed here — task 7 records it as an adopter-only artifact. `framework/commands/*.md` and `specs/002-…` are a glob and an elided row rather than paths. `scripts/gen-spec-deps.sh` and `scripts/gen-cross-service-refs.sh` were deleted by 022's promotion, which is the defect this pass corrected.

**Known and already routed, deliberately not re-captured.** `.claude/commands/ductus/init.md` writes a new project's config to the pre-042 root `.govern.toml` and never acquires the runtime. That is one of five drifts an existing inbox item measures, and the operator decided on 2026-09-13 to **retire** the file rather than repair it, which is a 050 back-edge. Re-filing it here would duplicate a live item against the idempotent-capture rule.

Spent `tasks.md` entries naming `substitute_templates`, the two generators and the opt-in CI are left exactly as written: rewriting a completed task's rationale records a fiction about why the work was done, and §tasks-phase makes the file ephemeral regardless.

Checks run against the committed tree: `lint-markdown` on the feature directory (clean), `derive-dependencies` (drift false, 54 examined), `resolve-anchor` on all three edited files (`unresolved: []`, with the four `qualified` references on the AC11 line each confirmed by hand against its target heading).

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
