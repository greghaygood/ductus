---
spec: 046-scenario-open-question-visibility
reviewed-at: 2026-09-13T13:27:44Z
reviewed-against: 444c51ae9cc16910bae086b071d1919d7e0238d4
diff-base: 444c51ae9cc16910bae086b071d1919d7e0238d4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 4
scope: 14
skipped-passes: []
---

# Review — 046-scenario-open-question-visibility

## Summary

First review of 046 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All thirty-two criteria verified against the tree; **one was half false**, and tracing it surfaced a stale durable contract in a neighbouring spec.

**What this review read: 4 of the 14 files in scope in full, and here is the other 10.** Read in full: `framework/commands/analyze.md`, `status.md`, `target.md`, and `framework/constitution.md` — between them the subjects of roughly two-thirds of the criteria. **Read in part, at the sites the criteria name:** `framework/commands/implement.md` (its pre-`done` gate ordering), `runtime/src/primitives/check_review_gate.rs` (`scenario_question_block` in full, not the 2000-line file), `check_artifacts.rs` (its family documentation and the `artifact-unreadable` records), and `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md` (its Context and ownership split). **Not examined:** `runtime/src/primitives/{create_scenario,dashboard,read_spec}.rs`, `specs/022-deterministic-runtime/data-model.md`, `scenario-question-parser-fix.md`, and this spec's own `tasks.md`.

**AC22 was half false, and the halves had genuinely diverged.** It claims an unreadable or malformed scenario "never blocks the `done` gate **and** never produces a blocking finding". The **gate** half is exactly right and verified at source: `scenario_question_block` returns `None` on an empty question list, with the reasoning in the code — "this gate must not fail closed on its own inability to read". The **finding** half was reversed by `8c4ca74`, which made `artifact-unreadable` a **blocking** `check-artifacts` finding on a `done` spec; `check_artifacts.rs`'s own header names `scenario-open-questions` as the motivating family, because an unreadable scenario contributed no questions and, as a skip, no finding — so a scenario carrying unresolved questions that would not parse passed the gate *this spec exists to build*. Annotated rather than rewritten: the claim was true as delivered, and the gate half still is. AC32 carried the looser form of the same claim ("blocks nothing"), narrowed here to the pre-`done` gate, which is what it remains true of.

**The same reversal left a durable contract stale, and that is logged rather than swept.** `022/scenarios/unreadable-scenario-is-reported.md` states "**Nothing gains a block**" and, of the skipped record, "**It is not a finding: the file is an unknown, not a defect**". The first is still correct; the second is now contradicted by the shipped runtime and by `analyze.md`'s own Unexamined-targets exception. A scenario is a durable requirement document, so this is a stale behavioural claim inside a `done` spec's artifact — a meaningful edit taking 022's back-edge and staling its `reviewed-digest`, which is why it is an inbox item with the exact wording and the gate/finding split recorded, rather than folded into this pass. The split matters: a blanket sweep would destroy the half that is right.

**How the rest were checked.** AC7: §spec-lifecycle's `done` row names "no scenario under the spec carries unresolved open questions", and §readiness-check says "the spec body's **and** those carried by any scenario under it" — both halves the criterion requires. AC8–AC10: `target.md` §Scenario open questions displays the total, names every carrying scenario in case-insensitive filename order, recommends no specific one with the reason stated, and overrides the next step to scenario-targeted clarify. AC11–AC13: `status.md` suffixes the existing Scenarios column with `({N} open)` and leaves it a bare count at zero, overrides Next Action, and renders the scenario callout **independently** of the recovery callout — which is AC13's case, where only the cell is exclusive. AC3, AC31: `clarify.md` surfaces scenario questions but resolves none on the feature-targeted path. AC14, AC15, AC23: `analyze.md`'s family is blocking at `done` and advisory otherwise, produces nothing for a feature with no scenarios or no questions section, and carries **no grandfather rule** — stated there in contrast to review-state drift. AC16–AC18, AC26, AC27: the parser reads `## Open Questions` only, excludes `## Resolved Questions`, HTML comments and fenced blocks, and skips both placeholder forms; the gate's own doc comment states why an exemptible section was refused. AC20, AC21, AC24, AC25: the gate check is ordered ahead of the `review:` checks with the reason recorded, its message names every carrying scenario via `scenarios.join(", ")` with no cap, and its guidance offers both exits — resolve, or move to Resolved Questions with the trigger. AC28, AC29: the runtime half lives in 022's scenarios, which back-link this spec and state the ownership split explicitly, while the constitution amendments landed here.

**On the diff base.** No commit records 046 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 046 has no scenarios of its own and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
