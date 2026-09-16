---
spec: 047-analyze-findings-durability
last-run: 2026-09-16T16:06:53Z
reviewed-against: f31e83063c111fad72c93d9cc2d1737eb7943aa6
diff-base: 2a4779c0104d26c62f75acceb311ee61c8e822cd
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 3
scope: 8
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 825f3b4cbe5d780f4d170ff0cbc2bea347d2dfc06c4aacc8c44504ed26f48cfd
  scenarios/analyze-run-durability.md: 7719db590c1b78447332523a3725c1d05faa64abb0950d43aedeff50ed3648a2
blocking: false
---

# Review — 047-analyze-findings-durability

## Summary

Third recording for spec 057, and the third is the AC7 fix rather than the relocation. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers.

**Why a third.** Pass one (task 18) corrected the record-location claims in `spec.md` and `analyze-record-freshness.md`. Pass two (task 19) caught `analyze-run-durability.md`, which the location classifier could not see because it names no file. This one follows a **code** change: recording the analyses for those very passes surfaced that 057's AC7 was only half-met — `subject_digest` excised `analysis.md`'s frontmatter and digested its body, which the same `write-analysis` call rewrites, so any run whose report changed staled itself and converged only on a second identical run. Measured on `020-code-review`. The fix excludes `analysis.md` from the subject set outright, and this spec's `analyze-record-freshness` scenario is where that rule lives.

**The correction is to the rule's history, not just its statement.** The subject-set list drops `analysis.md`, and the paragraph tracing the exclusion now records both widenings with the reason that forced each: block surgery on `spec.md` while the record lived there, then the whole file once the record moved and excising only its frontmatter proved insufficient. The reason is identical across all three forms, which is what carried it — a record written after its subjects are read can never digest itself. `review.md` stays a subject, and the new unit test `the_review_record_remains_a_subject` pins that asymmetry rather than leaving it to prose.

**Scope.** `diff-base` 2a4779c0, 8 in scope, examined **3**: this spec's `spec.md`, `tasks.md` and `analyze-record-freshness.md`, all read in full. The five unread are named. `framework/constitution.md` (Frontmatter Schema, Validation Severity), `framework/commands/analyze.md` (the record-writing step) and `022`'s `data-model.md` and `spec.md` (the digest section and signpost) were read only in the regions this review's claims rest on, each confirmed against the live file and none read end to end. `.claude/commands/ductus/analyze.md` is the generated mirror of a source that was not fully read either.

**Passes.** Security, reuse and efficiency had no subject — prose in one durable contract. Quality, against `quality-cross.md`, is the pass that produced the underlying finding rather than this correction: a record that stales itself is a gate with a false positive, and the plan's own text says that is the gate people route around. Simplicity: the whole-file exclusion is the smaller rule than a partial excision, and it deleted `strip_record_frontmatter` along with the path it existed for.

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
