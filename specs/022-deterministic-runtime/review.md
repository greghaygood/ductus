---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-15T19:51:35Z
reviewed-against: e7ac89523d7ec0ddcae0d27c1fefd6d2790a89e8
diff-base: 6f8b787200f2915ba891c17b89205e2707cffa1d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 5
scope: 52
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Five passes over the part of the scope this change touches. **examined: 5 of 52, and that is the honest number** — this was a deliberate partial re-review, not a full pass over 022, and the unread paths are enumerated below rather than folded into the numerator.

READ IN FULL: runtime/src/primitives/check_promotion_coverage.rs; scripts/audit/promotion-coverage.sh; specs/022-deterministic-runtime/scenarios/the-promotion-coverage-line.md; framework/runtime-tools.txt; version.

NOT EXAMINED — 47 in-scope paths, named individually because a count alone would let a reader assume coverage this pass did not spend. Several were opened at the region this change touched and are still listed here, because reading a region is not reading a file: specs/022-deterministic-runtime/{spec.md, plan.md, tasks.md, data-model.md, scenarios/write-session-primitive.md}; scripts/audit/{run-all.sh, review-freshness.sh, README.md}; framework/commands/{audit.md, analyze.md, implement.md, plan.md, specify.md, status.md, target.md}; framework/bootstrap/{ductus.md, configure/claude.md, configure/auggie.md}; .claude/commands/ductus/{audit.md, configure.md}; runtime/src/{main.rs, lib.rs, io.rs, interpreter/mod.rs, mcp/server.rs, primitives/mod.rs, schema/primitives.rs, schema/registry.rs}; runtime/src/{interpreter/, mcp/, parser/, primitives/, schema/}; runtime/tests/{, fixtures/, golden/, parity/}; runtime/{CHANGELOG.md, Cargo.toml, Cargo.lock, .gitignore, legacy-prose-commands.txt}; scripts/lint-procedure-parseability.sh; README.md; .github/workflows/{runtime.yml, runtime-release.yml, markdown-only-pipeline.yml}.

022 carries 99 scenarios and a 1434-line data-model, and none of the 98 scenarios other than the one added here was re-read. The prior record was itself a digest repair (examined 12 of 87) whose own Summary said its scenarios were not re-read, so this record does not claim, and must not be read as, the first full five-pass review of 022.

WHAT THE PASSES COVERED. Security: no network, no credentials, no untrusted input — the new primitive reads two repo-relative paths through the shared resolver and writes nothing. Reuse: the comment- and fence-aware bullet grammar is shared with the inbox primitives rather than re-rolled, and Family 38 calls the primitive instead of reimplementing the count, which is what §runtime-boundary principle 3 requires and what QUAL-DELEG-001 governs — the top-level-bullet constraint is deliberately kept local, with the reason recorded at both the call site and in the data-model, because the shared helper trims indentation for a case where nesting does not occur. Quality: QUAL-CLAIM-001 turned on the new result — no-table is a state rather than a zero, a supplied-but-unreadable table is an error rather than the absent state, missing-sections and unmatched-keys are their own fields, and all four terms are reported rather than the difference alone. Efficiency: one pass over each of two files, no repeated scanning. Simplicity: no new config schema and no adopter-facing surface, which is why the notice landed in the maintainer-only audit rather than on the inbox-row surface.

ONE DEFECT FOUND BY THESE PASSES AND FIXED IN e7ac8952 rather than recorded: the coverage line Family 38 exists to render was invisible. run_check folded stderr into its captured output and discarded it on exit 0, so every family coverage notice — Family 19s included, written on every run since it shipped — never reached an aggregated run, and a clean run-all.sh printed nothing at all. Found by probe, not by reading. Fixed on both sides: run_check now passes stderr through, and review-freshness.sh emits its line to stderr like every sibling. A clean run prints 21 coverage lines instead of zero, and still exits 1 with two findings on a corrupted version pin.

Zero findings survive against this commit.

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
