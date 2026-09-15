---
spec: 036-quality-cross-rules
reviewed-at: 2026-09-15T19:06:55Z
reviewed-against: b40aafc277e1971c6bd1e1b1afbe9403edbab20b
diff-base: 6cf0f3cc8ba180878a217cbb2666a4a3e37b5b5d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 7
skipped-passes: []
---

# Review — 036-quality-cross-rules

## Summary

Five passes over the 7-file scope resolved at diff-base 6cf0f3cc, the parent of this reopen's first commit. Examined 6 of 7 in full: AGENTS.md (read in 16 byte-bounded ranges, the whole file), framework/rules/quality-cross.md, scripts/lint-rule-ids.sh, and 036's data-model.md, spec.md and tasks.md. NOT examined: framework/bootstrap/ductus.md — 814 lines / 110,417 bytes, in scope because plan.md lists it. Only its Shared Files manifest region was opened, to confirm the framework/rules/quality-cross.md -> specs/rules/quality-cross.md row AC7 asserts and that Family 35 tests the destination against; the other ~800 lines were not read and this review makes no claim about them. It is named rather than counted, per the examined-counts-what-was-read rule.

Security: no code changed; the pass covered the one artifact that leaves this repository, framework/rules/quality-cross.md, for leaked paths, credentials or maintainer-local context. None. Reuse: QUAL-DELEG-001 cites QUAL-CLAIM-001 and QUAL-GROUND-001 for the discriminator rather than restating them, and data-model.md points at 008's schema rather than copying it. Quality: QUAL-CLAIM-001 turned on lint-rule-ids.sh, which is in scope and unchanged by this task — it already fails loudly on an empty rule-file set and reports examined counts to stderr, so it satisfies the rule it is adjacent to. Efficiency and simplicity: the rule lands as a fourth category in an existing file rather than a new file or spec, which is the routing §rules prescribes, and the 050 signpost is a blockquote so it discharges the cross-spec-impact declaration without inducing a dependency edge.

Three defects were found by these passes and FIXED in b40aafc2 rather than recorded, per the fix-inside-the-spec-you-have-open rule; all three were introduced by this task's own first commit. (a) Family 35 red — the rule file ships to specs/rules/ while the constitution lands at .ductus/constitution.md, so its Source line's repo-relative links dangled in an adopter tree; re-spelled in prose to match the two sibling rules, probed red-before and green-after. (b) The spec's Added-categories section enumerated GROUND and CLAIM and omitted DELEG. (c) A markdown link to 050 in that section induced a false 036 -> 050 dependency edge; re-spelled as a backticked slug, drift true before and false after. Zero findings survive against the reviewed commit.

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
