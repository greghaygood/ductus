---
spec: 026-framework-self-audit
reviewed-at: 2026-09-15T13:02:08Z
reviewed-against: 880e59a0906dbdbb273784d28079446d6f01d856
diff-base: c1b00ea8661acd9a55392c39947b1569773edfcd
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 39
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Reviewed the `/ductus:init` retirement (cc870e18) and the ductus-v0.49.7 release (880e59a0) against 026's window. Rule files: all 11 loaded via `discover-rule-files`. Five passes run — security, reuse, quality, efficiency, simplicity — 0 MUST, 0 SHOULD, 0 low-confidence.

Scope and what was NOT read. `examined: 8` of `scope: 39`, `diff-base c1b00ea8` (the parent of the reopen commit, so the pass's own edits are inside the window). This is a deliberately small numerator over a wide scope and the split is stated rather than implied. **Read end to end**: `AGENTS.md`, `framework/constitution.md`, `runtime/tests/mechanical_sweep_parity.rs`, `scripts/gen-claude-commands.sh`, `scripts/audit/manifest-parity.sh`, `scripts/audit/check-zero.sh`, `specs/026-framework-self-audit/scenarios/host-namespace-parity.md`, `version`.

**Seven in-scope paths do not exist and could not be read**, none folded into the numerator: `.claude/commands/ductus/init.md`, deleted by this change; the five `specs/004-tech-stack-selection/` artifacts, removed by this change; and `.github/workflows/markdown-only-pipeline.yml`, which predates this pass and was already absent. They stay in scope because `compute-review-scope` reads the plan's Affected Files.

**Read only in part, named rather than counted**: `specs/026-framework-self-audit/spec.md` — all 25 acceptance criteria were read through `read-spec` and the §Behavior §2 Manifest-parity block was read and rewritten here, but the file was not read whole; `framework/commands/audit.md` and `scripts/audit/README.md` were read only at their Family 2 entries, which is what this change could falsify — both list the family by name with no behavioural claim attached, so the header rewrite contradicts neither. The three `runtime/src/` doc comments corrected by 0.49.7 were read with their enclosing functions (`load_template`, `template_candidates`), which is how the corrected text was confirmed against behaviour rather than assumed.

**Not read at all, and named individually rather than summarised**: `.claude/commands/ductus/audit.md` (generated from `framework/commands/audit.md`, whose Family 2 entry was read; `scripts/gen-claude-commands.sh --check` reported in sync this pass — grounds to believe it correct, and not a read); `.github/workflows/runtime-release.yml`; `runtime/legacy-prose-commands.txt`; and seven sibling audit families this change does not touch — `adopter-shell-behavior.sh`, `cross-doc-consistency.sh`, `introducing-drift.sh`, `placeholder-roundtrip.sh`, `sibling-coupling.sh`, `ssot-invariants.sh`, `template-alignment.sh`. What covers them instead is `scripts/audit/run-all.sh`, which ran green after the commit and is the gate those files exist to serve; that is evidence about their behaviour, not a substitute for having read them.

Quality pass, verified rather than assumed. Family 2's installer sub-check is retired rather than deferred, and the script carries no code for it — the body implements the MCP permission half alone, so the header change removes a stale deferral note and deletes nothing executable. `check-zero.sh` drives `gen-claude-commands.sh --check`, which exits 0 with the file gone and `init.md` no longer in `expected`. The Family 19 / `check-review-gate` parity test is the one real defect this pass surfaced: it had been non-vacuous only because 020 carried a stale-but-sweep-exempt `data-model.md`, so reopening 020 emptied its subject. Fixed in 0.49.7 with a fixture-backed test that cannot go vacuous, proven failable by probe in both directions.

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
