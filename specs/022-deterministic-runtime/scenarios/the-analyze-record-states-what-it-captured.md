---
section: "Follow-on scenarios"
---

# The-analyze-record-states-what-it-captured

## Context

[§brownfield-inbox](../../../framework/constitution.md#brownfield-inbox) requires that a findings-producing command *record* what it found rather than only printing it, and names `/{project}:analyze` as the case: *"A findings-producing command that discards its findings is the failure the Design Principles rule names directly."*

The command does both halves in two separate places. Step 16 appends each surviving finding to the inbox; step 17 invokes `write-analysis`, which records how many findings the run *produced* as `advisory`. Nothing held the two against each other. A run that recorded `advisory: 5` and appended nothing was **byte-identical on disk** to one that captured all five — same block, same counts, same `blocking: false`.

The review side is not exposed this way, and the asymmetry is instructive rather than accidental: `write-review` already records `captured-issues`, `compute-review-scope` already *derives* the set from inbox bullets in the diff window, and `check-review-agreement` holds `review.md` against its own frontmatter. Analyze has no counterpart to the last of those, because it writes **no report artifact** for one to compare against — so the record was the only place the information could live, and the information was not in it.

Measured 2026-09-13: every per-spec run reconciled, and the one divergence was a **batch** run (`7963f134`, 50 records totalling `advisory: 21`, touching `inbox.md` not at all) that was deliberate and whose findings later got a durable home. So this is not a report of lost content. It is that the same skip performed carelessly would look exactly the same.

## Behavior

**`write-analysis` accepts `captured-issues` and records the count** in the `analyze:` block, beside `advisory`. The two numbers together say what the run produced and what it landed, which is what makes a divergence legible.

**They are not required to agree, and forcing them would be a defect.** `append-inbox`'s `dedup-prefix` guard legitimately suppresses a re-append, so a correct re-run against an unchanged repo captures fewer than it found — `advisory: 5` beside `captured-issues: 0` is the **expected** second run, not an error. What the field buys is that the case where they disagree for a *bad* reason is visible rather than invisible.

**Recorded rather than derived, deliberately.** Counting inbox bullets inside the primitive would make `captured-issues == advisory` look like an invariant when the dedup guard means it is not, and would re-introduce the conflation from the other side: a legitimate zero and a careless zero would again be indistinguishable. The honest field is the one that states what the caller did.

## Edge Cases

- A clean run records `advisory: 0` and `captured-issues: 0` — both computed, neither absent. A zero that was computed and a field never written are not the same claim, which is the rule `unexamined` already carries in this block.
- A re-run against an unchanged repo records a non-zero `advisory` beside `captured-issues: 0`, because every append deduped. Expected, and the reason no gate is built on equality.
- `--fix` resolved findings are not captured and are not counted as advisory either, so the pair stays coherent.
- The field is recorded, never gated on: no check reads it yet. It is the durable half that makes an analyze-side agreement check writable later, which could not be written while the information did not exist.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
