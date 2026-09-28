---
spec: 036-quality-cross-rules
last-run: 2026-09-28T00:23:33Z
reviewed-against: 262adbcb2104709bcd8437a9b031d7e81a500c71
diff-base: 6e3ef069d23b26d0f8a620dffe1273c48f17f781
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 31
skipped-passes: []
reviewed-digest:
  data-model.md: 22b7f0429322f20dbc1ab11dfe5f295e87248830c5bdf61f9cd903111007e109
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 036-quality-cross-rules

## Summary

Re-review after the reopen at d52f79da, which restated the inherited severity invariant in tier terms (767b5b89: `data-model.md`'s § Severity and ID-stability invariants, plus a signpost in `spec.md`). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, and no stored decisions: the previous record predated dispositions. One observation, fixed as a chore in 262adbcb.

**Base.** `diff-base` is 6e3ef069 (d52f79da^), the derived default. As in 017's review, no narrower base exists. 036's change landed in 767b5b89 together with 008's rule-file rewrite, and every later commit in the window follows it, so any base that keeps 036's change also keeps all of 008's. The window has grown since 017's review by 017's own re-review commits.

**Scope.** 31 in scope: 28 modified since the base, unioned with 4 plan-listed Affected Files, one of which overlaps. Examined 5, each read in full: this spec's `spec.md` and `data-model.md`, `framework/rules/quality-cross.md`, `scripts/lint-rule-ids.sh`, and `framework/constitution.md`, read in full at session start and used here as the normative source. This spec's `plan.md` and `tasks.md` were also read in full, but they are outside the scope. Not counted, named by group:

- `framework/bootstrap/ductus.md`: plan-listed and unchanged since the base. Read only at its Shared Files manifest rows, `:656`–`:659`, to confirm AC7's placement; the rest of the file was not read and this review makes no claim about it.
- `AGENTS.md`: read only at a3b514c5's one added bullet, verified in 017's review against `runtime/src/interpreter/payload.rs:886`–`:894`.
- 17 files of 008's work, covered by 008's records (f76b2227, 6f43a9d9, 7a8760fb): `.claude/commands/ductus/analyze.md`, `.github/workflows/runtime.yml`, `framework/commands/analyze.md`, the four rule files `api-backend.md`, `performance-frontend.md`, `security-backend.md` and `security-frontend.md`, `runtime/src/primitives/rule_sections.rs`, and 008's nine spec files. Of these, only 008's `data-model.md` § Severity classification was read for this pass, as the source 036's invariant points to.
- 017's `spec.md`, `data-model.md`, `review.md` and `analysis.md`: 017's re-review and closure, covered by its own records (e584179d, 8241d3b0).
- 050's `spec.md`, `tasks.md` and `scenarios/a-status-commit-holds-only-the-transition.md`: d3f5ff4a's amend, left to 050's review.

**Passes.** All 11 rule files were loaded. Security, reuse, efficiency and simplicity found no subject in 036's change, which is spec prose and a data model; the window's code change is 008's. Reuse considered one question and raised nothing. `data-model.md`'s severity paragraph restates the inherited invariant in one clause, but it names `specs/008-security-rules/data-model.md` as the source, and the file's opening paragraph points at 008 for the schema.

Quality found that the changed claims hold. All four QUAL Statements are single-tier: STUB is MUST, and GROUND, CLAIM and DELEG are SHOULD. The data model's grammar note matches the parser and the harvester (`runtime/src/primitives/rule_sections.rs:77`, `runtime/src/primitives/check_rule_ids.rs:95`). `scripts/lint-rule-ids.sh` exits 0 over 11 files and 207 headings, and `scripts/lint-rule-filenames.sh` exits 0.

Quality raised one observation, on the signpost's figure of 66. It was re-derived by taking each rule's first block quote and counting Statements that pair MUST with MUST NOT, or SHOULD with SHOULD NOT, and carry no keyword of the other tier. That gives 66 of 192 at d178bdda, the clarify commit that recorded it, matching 008's scenario, and 68 of 207 at HEAD, because two split-off halves now pair MUST with MUST NOT. The figure was right but unbounded, so the signpost now names the corpus it was counted over.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the 008 signpost recorded a count of 66 single-tier Statements without the corpus it was counted over, which re-derives as 68 at HEAD — `specs/036-quality-cross-rules/spec.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
