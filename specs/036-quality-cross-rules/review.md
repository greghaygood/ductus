---
spec: 036-quality-cross-rules
last-run: 2026-09-28T00:27:35Z
reviewed-against: f6be2d8799a5663b19556cd2127ada02abfbb7a5
diff-base: 7a791a9800eb2bce4d365e27c7972b67b88d1785
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 6
skipped-passes: []
reviewed-digest:
  data-model.md: 22b7f0429322f20dbc1ab11dfe5f295e87248830c5bdf61f9cd903111007e109
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 036-quality-cross-rules

## Summary

Re-review for task 9, which the completion gate's criteria check raised. AC10 and task 8 put QUAL-DELEG-001's discriminator against QUAL-CLAIM-001 and QUAL-GROUND-001 in the Rationale, where both sibling rules state theirs, but it had shipped at the end of the Verification paragraph. f6be2d87 moves the sentence. The rule's ID, Statement and trigger are unchanged. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, no stored decisions, no observations.

**Base.** `diff-base` is 7a791a98, the previous review's commit, passed with `--since`. This follows the precedent 008's re-reviews set: everything outside this delta is covered by 7a791a98's record. The derived default, 6e3ef069, resolves 34 files. This base resolves 6.

**Scope.** 6 in scope, examined 5. Four were read in full: `framework/rules/quality-cross.md`, `scripts/lint-rule-ids.sh`, and this spec's `data-model.md` and `tasks.md`. The fifth, `analysis.md`, is 036's own record, read in full after it was written. For `quality-cross.md`, that means the whole file earlier in this session and the QUAL-DELEG section again after the edit. Not counted: `framework/bootstrap/ductus.md`, plan-listed and unchanged, read only at its manifest rows `:656`–`:659` in the previous review.

**Passes.** All 11 rule files were loaded. Security, reuse, efficiency and simplicity found no subject in a one-sentence move within one rule's prose. Quality checked the move. The sentence now follows QUAL-CLAIM-001's Rationale form ("This differs from … which governs …; this rule governs …"). The Verification paragraph now ends at "does not block `done`", as QUAL-GROUND-001's does. Nothing else in the repository quotes the moved text; a grep for "correct-looking substitution" finds only the rule and the prose summaries in the spec and data model. `scripts/lint-rule-ids.sh` exits 0 (11 files, 207 headings), markdownlint is clean, and `cargo test --release --locked` passed 1713 with 0 failed, since `framework/rules/**` is a runtime CI input.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
