---
spec: 027-bootstrap-migration-registry
reviewed-at: 2026-09-13T12:31:12Z
reviewed-against: 2e93663a90b6344a5cf63d4f9bc15b1424d40479
diff-base: 510eb25cfd96bc5ac2bcc714054c482a2c3cbfe1
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 23
skipped-passes: []
---

# Review — 027-bootstrap-migration-registry

## Summary

Reopened by the retired-filename sweep. AC6, AC23 and AC24 carried superseded-naming annotations added before the rule was settled; after the substitution they would have read *"`.ductus/config.toml` no longer exists"*, so they were removed and the paragraph that existed to explain them was rewritten. The criteria now name `.ductus/config.toml` directly — the requirement each states is unchanged, since only the file's name moved. Removing an annotation rewords a line, which is why 027 took the back-edge rather than riding the sweep's exemption. No MUST or SHOULD violation against the loaded rules is outstanding.

**The load-bearing check for this spec specifically.** 027 owns the migration chain, and the retired name is *deliberately* live inside it — a migration must name the file it migrates *from*. Verified that the sweep did not touch it: `framework/migrations/governance-config-rename.md` still names `.governance.toml` and `.govern.toml` four times each, `framework/migrations.toml` still carries its 12 entries, and `scripts/audit/migration-coverage.sh` (Family 10) exits 0 standalone. AC6's subject — the `[migrations]` section in the config schema — is documented in `framework/bootstrap/ductus.md` §Project Configuration and at its three call sites, verified against the file.

**One pre-existing defect found and fixed.** `§Project-level consistency` at `spec.md:39` did not resolve: the line named the command `/{project}:analyze` rather than a markdown document, so `resolve-anchor` read it as a claim about the constitution, where no such marker exists. It has been unresolved since 027 closed. Fixed by naming `framework/commands/analyze.md` on the reference's own line — the document match is line-scoped, so a name on the preceding line does not qualify it, which the first attempt at this fix demonstrated. All three of the spec's anchors now resolve.

**What this review read, and what it did not.** The five passes read 9 of the 23 in-scope files: 027's `spec.md`, `framework/rules/quality-cross.md`, `framework/migrations.toml`, `framework/migrations/governance-config-rename.md`, `framework/bootstrap/ductus.md` (the §Project Configuration and Pre-run Migrations regions AC6 asserts against), `scripts/audit/migration-coverage.sh` by execution, and the three artifacts the anchor and criterion checks resolved against. The remaining 14 are the other five migration procedure files, `CHANGELOG.md`, the generated `.claude/` copy, and 016's and 017's artifacts — in scope only because the sweep modified them in the same window, and each carries its own review. None was re-read this run.

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
