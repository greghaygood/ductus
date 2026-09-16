---
spec: 020-code-review
diff-base: ed2092a85afb3c1da6fa5b393f6afbc7f85e2e15
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T01:57:59Z
reviewed-against: fc70afc6db98029dc8ebd35e666be17eed7c1a70
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 15
reviewed-digest:
  data-model.md: 005b900eab99d081cfe9b6c643998cdda2ccb452048c0292e9ffb3097a300766
  scenarios/review-flag-parsing-is-specified.md: 9f1a3dd82bab2b9622808c31dd2bb00f0c6f403effeb8bb496bb4b329496e9c7
  scenarios/waiver-expiry.md: 6b7309dd2c803a6fd1ecf5a6a92f8895011d787b334452ce5fb7f73274c77068
blocking: false
---

# Review — 020-code-review

## Summary

Re-run 2026-09-13 for the embedded-snapshot removal. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. No waivers.

**What changed.** §Embedded artifacts held three frozen copies of the command sources this spec delivered — `framework/commands/review.md` in full (429 lines), plus the required edits to `implement.md` and `analyze.md`. All three are replaced by a short account of what 020 delivered, pointing at the live sources. `spec.md` goes 803 → 275 lines. Under [§drift-prevention](../../framework/constitution.md#drift-prevention)'s *Canonical sources*, replacing a copy with a pointer changes what the document asserts, so this is a factual correction and took the back-edge rather than the mechanical-sweep exemption.

**Scope.** The natural base collapsed from 114 modified-since / 118 in scope to **4 / 15** once the correction commit recorded a fresh `in-progress` transition; `--since HEAD` gave 0 / 11 and was declined for excluding this pass's own edits. Examined **8 of 15**.

Named rather than counted: `framework/commands/review.md` (896 lines) and `framework/commands/implement.md` (187) were read only in the regions this pointer asserts — the five dimensions, rule loading, waiver flow and report shape in the first; the pre-`done` gate in the second — each confirmed against the live file before the claim was written, but neither read end to end. `.claude/commands/ductus/review.md` is the generated mirror of a source that was not fully read either, so it is doubly uncounted. `framework/templates/spec/spec.md`, `framework/templates/ci/adopter-generators.yml` and `specs/050-constitution/spec.md` were not read. And **`framework/templates/spec/spec-and-plan.md` does not exist** — 023's lightweight-track sunset deleted it; it stays in scope because this spec's plan lists it, and is named here as absent rather than silently dropped.

**Three defects found by removing the copy, all fixed in the pass.** Deleting a section is not only a deletion: the corpus had to be re-read for what pointed at it. `data-model.md` — an authoritative-shapes document — said the shapes are referenced by "the spec body and embedded `framework/commands/review.md` artifact", naming a section that no longer exists. `plan.md`'s affected-files table said the **embedded artifact in spec.md is the canonical content**, which is the inversion that produced the snapshot in the first place and is exactly backwards under the canonical-sources rule. Both corrected.

**The third is independent of the removal and is the more consequential one.** `data-model.md` claims to hold authoritative shapes for the `review:` block and `review.md` frontmatter, and both tables had fallen behind what the runtime writes: the `review:` block was missing `examined`, `scope` and `reviewed-digest`, and the `review.md` frontmatter was missing `captured-issues`, `examined` and `scope`. This spec's **own AC15** asserts that "`review.md` and the spec's `review:` block both record `examined`", so the authoritative record contradicted its own acceptance criterion. Verified against a record written minutes earlier rather than against the code. Six fields added.

**AC1–AC15 were checked against the tree and all hold**, including the ones the removal could plausibly have falsified: none of the fifteen asserts that §Embedded artifacts exists, which is the check the cross-spec-signpost lesson requires before deleting a section a sibling criterion might depend on.

**One in-scope file corrected under a different spec.** `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md` is the durable contract that records this instance, and it recorded it in the present tense — "its §Embedded artifacts section **is** a frozen copy", "the snapshot **sits** inside a code fence". Both went false on removal. Corrected by tense with the discharge dated, which reopened 050; leaving a false present-tense claim inside the scenario that governs canonical-copy drift would have been self-refuting.

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
