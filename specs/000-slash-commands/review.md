---
spec: 000-slash-commands
diff-base: c1b00ea8661acd9a55392c39947b1569773edfcd
captured-issues: 0
skipped-passes: []
last-run: 2026-09-15T13:01:59Z
reviewed-against: 880e59a0906dbdbb273784d28079446d6f01d856
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 36
reviewed-digest:
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: b08bf71669de7980442f8246be334fac4440d43c1920eca95c770878f5c45386
  scenarios/dashboard-dependencies-column.md: e471757b1a1f7167129935144362f08c47c5da2dea585405a85f9ce2bcbc8f67
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/scenario-without-task-visibility.md: 912b16e99355b5fa17a6fd86fc4e640ffbe91375cf95394618c8954136e5960d
  scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
  scenarios/target-clear-flag.md: b7dd0b1df765a600403715a0012afacc9c13749122cd15d7d56ad2b69252eb52
  scenarios/validate-fix-mode.md: ce87d6a7473de97a20fb7cf4120a8a6e8bce588db355e7f1f39cd3baf40f447f
  scenarios/validation-gates.md: 07dc0f7d1d5c494503cca763dbfcfd75e497395621cf43d0b20b73b0048ef934
blocking: false
---

# Review — 000-slash-commands

## Summary

Reviewed the `/ductus:init` retirement (cc870e18) and the ductus-v0.49.7 release (880e59a0) against 000's window. Rule files: all 11 loaded via `discover-rule-files`. Five passes run — security, reuse, quality, efficiency, simplicity — 0 MUST, 0 SHOULD, 0 low-confidence.

Scope and what was NOT read. `examined: 8` of `scope: 36`, `diff-base c1b00ea8` (the parent of the reopen commit, so the pass's own edits are inside the window). **Read end to end**: `AGENTS.md`, `framework/constitution.md`, `runtime/tests/mechanical_sweep_parity.rs`, `scripts/gen-claude-commands.sh`, `scripts/audit/manifest-parity.sh`, `specs/000-slash-commands/spec.md`, `specs/000-slash-commands/scenarios/command-autocomplete-summary.md`, `version`.

**Sixteen in-scope paths do not exist and could not be read**, and none was folded into the numerator. Eleven are the pre-reorganization layout this spec's plan still lists — `commands/{about,analyze,clarify,implement,next,plan,setup,specify,status,target}.md` and `.claude/commands/ductus/init.md`, the last deleted by this very change. Five are `specs/004-tech-stack-selection/{spec,plan,tasks,review}.md` and its one scenario, removed by this change when 004 was consolidated into 003. They stay in scope because `compute-review-scope` reads the plan's Affected Files; the successor files were not substituted for them.

**Read only in part, named rather than counted**: `runtime/src/interpreter/payload.rs` and `runtime/src/primitives/mod.rs` (the template-resolution doc comments this release corrected, plus `load_template` / `template_candidates` in full, to confirm the corrected comments describe what the code does — they do: the first candidate is `{specs-root}/templates/{file}`, which `/ductus` ships via the Shared Files manifest rows and creates); `runtime/src/primitives/derive_boundary.rs` (the one test comment); `runtime/CHANGELOG.md` (the 0.49.7 entry written here, plus the 0.49.6 head); `runtime/Cargo.lock` (the single changed version line); `runtime/Cargo.toml`; and the five sibling `spec.md` files edited by the retirement (003, 020, 026, 040, 043), each read at its edited region rather than whole.

Reuse pass, one candidate considered and deliberately not filed. `commit_all` in the new fixture overlaps `init_git_repo` / `commit_staged` in `runtime/tests/parity.rs`. Not recorded as a finding: Rust integration tests compile as separate crates, so sharing needs a `tests/common/` module, and the two have different contracts — parity.rs freezes its signature because the commit OID lands in a golden payload, while this one only needs stability for a legible failure message. Fifteen lines against a shared module for two call sites with different determinism requirements.

Quality pass, verified rather than assumed. The generator's `--help` range moved with the deleted header line (`2,16p` → `2,15p`) and was confirmed by running `--help` and reading the rendered block. `--check` exits 0 and `scripts/audit/check-zero.sh` drives exactly that invocation. The prune loop removed `init.md` on the first write-mode run, which is the behaviour the rewritten AGENTS.md entry asserts. The relaxed corpus guard returns before the closing coverage line only on the empty-set path, and the new fixture-backed test was proven failable by probe — an inverted expectation gives `left: {} right: {alpha.md}` — after a first probe attempt that proved nothing because cargo exited 101 from the repo root without running.

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
