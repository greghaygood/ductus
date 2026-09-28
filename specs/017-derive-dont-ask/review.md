---
spec: 017-derive-dont-ask
last-run: 2026-09-28T00:02:48Z
reviewed-against: 5456df5c1d5bfac6e232c5546a63c5b5921d0ae4
diff-base: 6e3ef069d23b26d0f8a620dffe1273c48f17f781
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 64
skipped-passes: []
reviewed-digest:
  data-model.md: bc0dbdffb2f2dc1409cbae6c53d9acc9b30c77bb7671ac1909ebbe1071adc01b
  scenarios/detect-dependency-cycles.md: 2c426d44ce0a3a3cf5e91160fdf84ac054795737b25979152f70e89fe86005ab
  scenarios/generator-sync-claim-honesty.md: 0ee16bda3f0a5c0658100fee3d28ddae8214bc0eca9d57261811077aae314264
  scenarios/skip-prose-cross-references.md: 1b5c4bd36e2ca760437d63b948706e5b575ecd450b0779d1d10440cc77d0e086
  scenarios/tracked-specs-not-worktree.md: ba42aee5fbee0dfed38fce985200b0ba5775ecd03113a81aa792b25a18383829
blocking: false
dispositions:
  fixed: 1
  routed: 3
  discarded: 0
  undispositioned: 0
decisions:
  - key: "convention: AC25 said recording a review observation is what writes it to the inbox, which 058 stopped; its guarantee holds and only the mechanism is dated — `specs/017-derive-dont-ask/spec.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/spec.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
  - key: "convention: data-model prescribed `## {Category Name}` rule-file headings while configuration-cross.md and 008's corrected twin schema use `## {ID prefix} — {Category Name}` — `specs/017-derive-dont-ask/data-model.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/data-model.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
  - key: "convention: data-model said hook scripts use the `.sh` extension; git invokes hooks by name and none has ever had one — `specs/017-derive-dont-ask/data-model.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/data-model.md
    decided-at: 2026-09-28T00:02:48Z
    decided-by: andy@stone.dev
---

# Review — 017-derive-dont-ask

## Summary

Re-review after the reopen at d52f79da, which restated the configuration rule format in tier terms (767b5b89: `data-model.md`'s Statement row and severity invariant, plus a signpost in `spec.md`). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, and no stored decisions: the previous record predated dispositions. Four observations. Three were routed as body edits on this spec and one was fixed as a chore, all in 5456df5c.

**Base.** `diff-base` is 6e3ef069 (d52f79da^), the derived default. The only narrower candidate, `--since=d52f79da`, measured an identical modified-since set of 26 files. No base can narrow further. 017's change landed in 767b5b89 together with 008's rule-file rewrite, and every later commit in the window follows it, so any base that keeps 017's change also keeps all of 008's. Unlike 008's re-reviews, there is no earlier review of this reopen to narrow from: 017's previous review (23a5fd80) predates it.

**Scope.** 64 in scope: 26 modified since the base, unioned with 43 plan-listed Affected Files, 5 of which overlap. Examined 6, each read in full: this spec's `spec.md`, `data-model.md`, `plan.md` and `tasks.md`; `framework/rules/configuration-cross.md`, the rule file whose format the data model governs; and `framework/constitution.md`, read in full at session start by `/ductus:target` and used here as the review's normative source. Not counted, named by group:

- `AGENTS.md`: read only at a3b514c5's one added bullet. Its claims were verified against `runtime/src/interpreter/payload.rs:886`–`:894` and `framework/commands/analyze.md` steps 11–12.
- 17 files of 008's work, covered by 008's records (f76b2227, 6f43a9d9, 7a8760fb). They are `.claude/commands/ductus/analyze.md`, `.github/workflows/runtime.yml`, `framework/commands/analyze.md`, the four rule files `api-backend.md`, `performance-frontend.md`, `security-backend.md` and `security-frontend.md`, `runtime/src/primitives/rule_sections.rs`, and 008's `analysis.md`, `data-model.md`, `plan.md`, `research.md`, `review.md`, `spec.md`, `tasks.md` and two scenarios. Of these, only 008's `data-model.md` § Severity classification, the section 017 now points to, was read for this pass.
- 036's `spec.md` and `data-model.md`: the same restatement, left to 036's own review.
- 050's `spec.md`, `tasks.md` and `scenarios/a-status-commit-holds-only-the-transition.md`: d3f5ff4a's amend, left to 050's review.
- 27 plan-listed Affected Files unchanged since the base, not re-read. They are the six spec templates, ten command sources, `CLAUDE.md`, `README.md`, `scripts/gen-help-tables.sh`, `scripts/install-hooks.sh`, `.githooks/pre-commit`, `framework/bootstrap/hooks/pre-commit`, `framework/bootstrap/ductus.md`, the two `configure/` files, and both generator workflows. Only the first three lines of the hook files were read, for observation 3.
- Eight absent files, in scope only because the plan lists them: `constitution.md`, `framework/bootstrap/hooks/install.sh`, `framework/commands/capture.md`, `framework/commands/elaborate.md`, `framework/templates/spec/spec-and-plan.md`, `scripts/gen-readme-table.sh`, `scripts/gen-spec-deps.sh` and `specs/000-016/spec.md`.

This spec's four scenarios are outside the scope. All four were read in full, and each hashes to the digest the previous review recorded.

**Passes.** All 11 rule files were loaded. Security, reuse, efficiency and simplicity found no subject in 017's change, which is spec prose and a data model; the window's code change is 008's. Reuse considered one question and raised nothing: `data-model.md:84` restates the tier rule, but the constitution's canonical-source map makes this data model the home of the CFG format, so it has to state it, and it names 008 as canonical for the shared part. Quality found that the changed claims hold. All 11 CFG Statements are single-tier MUST. The pointer to 008's § Severity classification resolves and states how a split assigns IDs. The signpost's link resolves and adds no dependency edge. Quality also raised four observations about claims elsewhere in 017's own artifacts:

1. AC25's inbox half was superseded by 058, which declared no impact on 017. The criterion was annotated rather than rewritten.
2. The data model's category-heading form had drifted from the shipped rule file since 041b8ccc, and from 008's corrected twin.
3. The data model claimed a `.sh` extension for hook scripts.
4. A bare "validate" remained in four lines. The seven `/validate` mentions that record decisions under the name they were taken with were left alone.

After the edits the residue grep, the heading counts, markdownlint and `derive-dependencies` were all clean.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: AC25 said recording a review observation is what writes it to the inbox, which 058 stopped; its guarantee holds and only the mechanism is dated — `specs/017-derive-dont-ask/spec.md` — **routed** to `specs/017-derive-dont-ask/spec.md`
- convention: data-model prescribed `## {Category Name}` rule-file headings while configuration-cross.md and 008's corrected twin schema use `## {ID prefix} — {Category Name}` — `specs/017-derive-dont-ask/data-model.md` — **routed** to `specs/017-derive-dont-ask/data-model.md`
- convention: data-model said hook scripts use the `.sh` extension; git invokes hooks by name and none has ever had one — `specs/017-derive-dont-ask/data-model.md` — **routed** to `specs/017-derive-dont-ask/data-model.md`
- convention: the retired command name survived as a bare "validate" in four lines of spec.md and data-model.md — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
