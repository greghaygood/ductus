---
spec: 017-derive-dont-ask
reviewed-at: 2026-09-13T12:28:44Z
reviewed-against: 07901330d423c655f1954e7b6cf1ea8ba1d7147e
diff-base: 510eb25cfd96bc5ac2bcc714054c482a2c3cbfe1
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 49
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Reopened by the retired-filename sweep, which removed the two superseded-naming annotations 017 carried in `spec.md` and `data-model.md`. Reviewing that change found six further stale claims, all corrected in this run and all of the same class: a criterion or contract describing machinery that no longer exists. **AC12** claimed the pre-commit hook runs four generators — `gen-readme-table.sh` was retired with the generated table (AC10) and `gen-spec-deps.sh` was promoted to the `derive-dependencies` / `derive-references` primitives (AC23), so two of the four have not existed for several releases; the hook today runs `gen-configure-mcp`, `gen-claude-commands` and `gen-help-tables` plus those two primitives, verified against `.githooks/pre-commit`. **AC24** still specified the CI step as dry-run, which the spec body itself contradicts. All **four scenarios** carried present-tense MUST statements naming those two retired scripts, while `framework/rules/quality-cross.md`'s `QUAL-CLAIM-001` Source — which cites `generator-sync-claim-honesty` by name — already recorded the promotion, so the rule file and the contracts it cites disagreed. Each was annotated rather than rewritten: the requirements carried across to the primitives unchanged, so the text still binds, and a superseded *mechanism* is not a rename. No MUST or SHOULD violation against the loaded rules is outstanding.

**What this review read, and what it did not.** The five passes read 8 of the 45 in-scope files: `framework/rules/quality-cross.md`, 017's `spec.md` and `data-model.md`, all four scenarios, and `.githooks/pre-commit` (the file AC12's claim had to be checked against). The remaining 37 are `plan.md` Affected Files entries from 017's original 2026 implementation — templates, command sources and bootstrap files it has not touched since — and they were **not** re-read this run, so this report is evidence about 017's own artifacts and the hook, not about those. Eight of the 45 no longer exist at all (`constitution.md`, `framework/commands/capture.md`, `framework/commands/elaborate.md`, `framework/templates/spec/spec-and-plan.md`, `framework/bootstrap/hooks/install.sh`, `scripts/gen-readme-table.sh`, `scripts/gen-spec-deps.sh`, `specs/000-016/spec.md`), retired by later specs; that is the plan being a design record of what was affected then, which AC14 makes explicit, not drift to repair.

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
