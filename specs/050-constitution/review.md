---
spec: 050-constitution
reviewed-at: 2026-09-13T12:33:13Z
reviewed-against: d2bbb3cb66d299d15bb4f465fd08a80184036168
diff-base: 832aeb5d3088ffaf142d903ae375a9fae5f8bad3
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 15
scope: 87
skipped-passes: []
---

# Review — 050-constitution

## Summary

Tasks 15–17 delivered §drift-prevention's retired-filename rule and the corpus pass it governs. The rule was corrected mid-implementation: the first version said a ticked acceptance criterion is annotated rather than rewritten, which is wrong for a rename — the requirement a criterion states is unchanged when only the file's name moved, and an annotation preserves the retired name in the corpus permanently, which is the residue the rule exists to remove. Annotation is now reserved for a criterion whose *behaviour* was superseded. AC19 records the rule and was verified clause by clause against `framework/constitution.md` rather than asserted. No MUST or SHOULD violation against the loaded rules is outstanding; all 28 `§` references in the spec resolve.

**The sweep, and why it reopened only three specs.** 490 occurrences of four retired names — `.govern.toml`, `.govern.session.toml`, `.govern/`, `.governance.toml` — were judged individually across the spec corpus; 346 were substituted across 65 files and the rest deliberately kept. The survivors are categorical, not accidental: the runtime's resolution-ladder tiers, the migration chain that must name what it migrates, `managed_roots`' historical roots, one dated 2026-05-22 observation of a command actually run, and 042/049 — the specs that *are* the decision record of the rename. Every touched file is line-for-line with per-line token counts asserted, so the diff satisfies all three parts of `mechanical_sweep`'s test and the exemption held: 016, 017 and 027 reopened only because each carried a **reworded** line (a dead README pointer, two removed annotations), and each has since been re-reviewed, re-analysed and returned to `done`. That the exemption held was verified empirically, not assumed — Family 19 reported 022 stale before the commit and clean after, which is the check answering about committed state.

**Verification.** The full local gate ran green on the committed tree: `cargo test --release --locked` (20 test binaries, 0 failures), `cargo clippy -D warnings`, `cargo fmt --check`, `markdownlint-cli2` over 512 files, the six lint scripts, `scripts/tests/*.sh`, `shellcheck -S warning` over 55 files, all three generators, both frontmatter derivations followed by a clean `git status`, and `scripts/audit/run-all.sh` across all 37 families — run again after committing, since several families read git history.

**What this review read, and what it did not.** The scope is 86 files because 050 performed the sweep, and every one of the 490 occurrences in it was examined at occurrence level with surrounding context — that judgment *was* the work. Full end-to-end reads number 15: `framework/constitution.md`, `AGENTS.md`, `framework/rules/quality-cross.md`, `runtime/src/primitives/mechanical_sweep.rs` (which settled the clarification's open question), 050's `spec.md`, `tasks.md` and its new scenario, 017's `spec.md` and four scenarios, 016's `spec.md`, and 027's `spec.md`. The remaining 71 files were read only around their occurrences, and 050's own `plan.md` was not re-read this run. The 18 surviving occurrences in `specs/*/review.md` were left alone deliberately: `write-review` regenerates those files wholesale, so they clear on each spec's next review rather than by hand.

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
