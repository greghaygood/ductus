---
spec: 050-constitution
reviewed-at: 2026-09-15T18:22:57Z
reviewed-against: 84218ec14fb01780ebc1c907f3b5e229896212d0
diff-base: a9bf694567eeb66d3c4fbbef874754b71f780f65
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 8
skipped-passes: []
---

# Review — 050-constitution

## Summary

Five passes over all 8 in-scope files, each read in full this session: `AGENTS.md` (180 lines, in 15 byte-bounded ranges), `framework/constitution.md` (765 lines, in 8), `specs/045-decision-state-drift-detection/spec.md`, and 050's `spec.md`, `plan.md`, `tasks.md`, the new `scenarios/a-measurement-states-its-method-and-units.md`, plus `specs/inbox.md`. `examined: 8` of `scope: 8` — nothing was folded in on confidence rather than reading. All 11 rule files `discover-rule-files` reports under `selected` were loaded in full before the passes ran.

Scope is markdown only — no source file is in it — so the surface-specific rule sets (security-backend/frontend, api-backend, concurrency, observability, reliability, performance-backend/frontend, accessibility, configuration-cross) verify design-time commitments this change makes none of: it introduces no endpoint, no credential, no persisted data, no configuration key, no UI. That is stated as a bounded claim, not as a clean sweep: those rules were read and found to have no subject here, which is different from having been checked and passed.

`quality-cross.md` is the file with a live subject, and `QUAL-CLAIM-001` is the rule the change is most exposed to, since its own content is about claims. Three checks: the constitution bullet states plainly that its disposition rests on re-derivation rather than on any gate; the scenario's Edge Cases state outright that nothing enforces the rule and *why* a gate could not — a comparison of `examined` against `scope` cannot know whether a Summary's denominator matches the primitive's; and the coverage figures written into `plan.md` and `specs/inbox.md` carry their method inline (top-level bullets per level-two heading, in-scope sections only) rather than as bare numbers. A rule requiring stated method whose own landing recorded unstated numbers would have been the defect it describes.

Reuse: verified by search rather than assumed, before writing. `units`, `denominator` and `method` appear nowhere in the constitution; §grounding's *Cite what you consulted* binds the **source** only; §design-principles' check-that-cannot-run governs a check that did not run, not a measurement that ran with an unstated denominator. So this is a gap, not a second copy. On the other side of the same rule, the one `AGENTS.md` entry that stated it generally is reduced to a pointer rather than left as a parallel statement, per §drift-prevention's *referencing means a pointer, never a copy*.

Simplicity: one bullet in an existing section, no new heading and no new `<!-- §anchor -->`, which is 050 §Resolved Questions' bullets-not-sections default and leaves AC10's anchor set untouched.

Two things this pass changed that were not in the task as written, both recorded where they bind rather than here. The entry reduced to a pointer is row 22 of `plan.md`'s `#### Second round — promote`, so the new rule **discharges** one of that round's 26 in substance; the row moved to `#### Second round — already promoted in substance` and the round now reads 25 / 1 / 2 / 16, still summing to 44, with `specs/inbox.md` carrying the same delta. And one of the six corollary entries left in place does carry a general clause (*a measurement is a claim, and its scope is part of the claim*); the decision to keep it, and the line that decides such cases, is recorded in the scenario's Edge Cases rather than left for the next reader to re-derive.

Two measurements re-derived rather than quoted, both agreeing with the record: `AGENTS.md` holds **119** rule-bearing bullets (Workflow 49, Gotchas 59, Boundaries 3, Design Principles 8), and the inbox item's claim of *six* entries stating the rule is an undercount — there are **seven**, all seven verified to carry the attributed fragment. The undercount is noted in the scenario as the rule failing on its own proposing record.

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
