---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-09-27T19:20:13Z
reviewed-against: 75eee29ca52cd9a87ad8da3fa3b0ea3c9fdc5757
diff-base: c6a7c74c0f653d3904052319dc8ae76abaccc80e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 27
scope: 28
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: discarded
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-09-27T19:20:13Z
    decided-by: andy@stone.dev
---

# Review — 060-exec-analyze-assesses-each-loaded-rule

## Summary

Full five-pass review of 060 against the 11 selected rule files and AGENTS.md. The window since `c6a7c74c` (the parent of the in-progress commit `be0dacc3`) holds 28 files, every one also modified, and the passes read 27: the runtime diffs (`runtime/src/interpreter/mod.rs`, `payload.rs` and `analyze_tally.rs`, the new `runtime/src/primitives/rule_sections.rs`, `check_rule_ids.rs` and `primitives/mod.rs`, and the doc comments in `runtime/src/schema/extensions.rs`, `primitives.rs` and `severity.rs`), the tests and fixtures (`runtime/tests/parity.rs`, `walker.rs`, the `analyze-basic` rule file, stdin and golden), `framework/commands/analyze.md`, the release files (`version`, `runtime/Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`), 022's changed documents and records, and 060's own `spec.md` and `tasks.md`. `.claude/commands/ductus/analyze.md` is not counted: it is generated from `framework/commands/analyze.md`, which was read in full, and the generator was re-run at task 8 and left no diff. The scope includes 022's exec-clarify fan-out (task 125), which landed in this window. 0 MUST, 0 SHOULD, 0 low-confidence. Three observations. Two were fixed at `7967bbb3` with the operator's confirmation: a failure whose finding named `info` or the empty tier counted in no tier (the new test failed before the fix), and unused derives on `LoadedRules` and `RuleSection`. The third, a per-request re-read of the spec file, was discarded with its reason. The fallback fix is in the `0.54.2` changelog at `75eee29c`. The quality and simplicity passes were re-run over the fixed code; the full suite passes (1711) and clippy is clean.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: a failed assessment whose finding named info or the empty tier, both of which validation accepts, counted in no tier, so the record read as though the rule had passed — `runtime/src/interpreter/analyze_tally.rs` — **fixed**
- convention: LoadedRules derived Clone and Debug, and RuleSection Clone, PartialEq and Eq, that nothing used — dead code under AGENTS.md's no-dead-code rule (also runtime/src/primitives/rule_sections.rs) — `runtime/src/interpreter/payload.rs` — **fixed**
- perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs` — **discarded**: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured

## Skipped passes

*None.*

## Unexamined governance

*None.*
