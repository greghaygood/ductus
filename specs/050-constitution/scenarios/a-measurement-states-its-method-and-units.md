---
section: "Follow-on scenarios"
---

# A-measurement-states-its-method-and-units

## Context

Seven `AGENTS.md` entries in this repository independently arrived at one rule, and not one of them states it as the rule. Each states a corollary of it, in the context of the gotcha that produced it:

- *a recorded measurement can encode a constraint you have not found yet — re-derive at the recorded figure's own commit before concluding the record is wrong*;
- *when two greps in one session disagree, re-derive both*;
- *a measurement is a claim, and its scope is part of the claim*;
- *a count is evidence about the day it was taken* (stated twice, in two unrelated entries);
- *the 19-versus-20 test-binary case was a units disagreement rather than decay*;
- *an inbox item's enumeration of what it touches decays toward under-estimating*.

The inbox item that captured this counted **six**; re-deriving the set from `AGENTS.md` found **seven**, which is the rule failing on the very record that proposed it.

The cost is a re-derivation each time. In one session, four recorded figures were met as fresh puzzles: a candidate generator whose recorded marker list provably could not match either instance it was written from; **51** constitution-citing entries against the **47** a coverage formula needed, the first counting anchor-bearing *lines in the file* and the second *rule-bearing bullets in four sections*; the 19-versus-20 binary count; and a spec's `unexamined` overwritten from a measured **3** to a bare **0**. **Every one resolved to a difference of method, units, or an overwrite — and none to the record having decayed.** The reflex on meeting a disagreement is to trust the newer figure, which would have been wrong all four times.

Two existing rules sit adjacent and neither covers it. [§grounding](../../../framework/constitution.md#grounding)'s *Cite what you consulted* requires naming the **source** — the path, the query, the command — and says nothing about how the number was counted or what it counts. [§design-principles](../../../framework/constitution.md#design-principles)' *a check that cannot run must never look like one that passed* covers a check that did not run; this is the harder case where the measurement **did** run, honestly, with its denominator left unstated. The words `units`, `denominator` and `method` appear nowhere in the constitution.

## Behavior

[§grounding](../../../framework/constitution.md#grounding)'s **Rules** list gains a bullet, placed immediately after *Cite what you consulted* because it extends that rule rather than standing alone — the bullets-not-sections default this spec's §Resolved Questions sets, which also keeps AC10 satisfied by adding no anchor.

- **A recorded measurement states its method and its units, or it is not re-derivable.** Citing the source covers where the number came from; the number additionally carries *how* it was counted and *what* it counts.
- **The failure it prevents is a misread disagreement.** Without method and units, a later reader who measures the same subject and gets a different answer cannot separate a figure that has **decayed** from one taken by a **different method**, and the reflex — trust the newer figure — is wrong in exactly the cases where the older measurement was the better-informed one.
- **So state the denominator, the population, and the boundary.** *Rule-bearing bullets across four named sections* and *anchor-bearing lines in the whole file* are two honest measurements of one document that legitimately disagree; so are *test binaries* and *reported result lines*. Neither pair is an error, and neither may be substituted for the other.
- **On a disagreement, re-derive at the recorded figure's own commit first.** That separates decay from method at the cost of one command. When it does not explain the gap, diff the field across its own history — a value can have been correct and then overwritten with a worse one, which is the direction nothing resists.
- **A measurement whose method is unstated is an unmeasured gap wearing a number**, which [§design-principles](../../../framework/constitution.md#design-principles) already places in the task column rather than the caveat column.

The rule ships to adopters because the population is universal, not this repository's. Every adopter records `examined`, `scope` and `unexamined` on review and analyze blocks, and writes counts into review and analyze Summaries; the rule survives dropping every name particular to this project.

## Edge Cases

- **This is a new rule, not a promotion.** The seven `AGENTS.md` entries are evidence that the rule was needed, not seven copies of it awaiting consolidation — each states a situational corollary inside a gotcha whose subject is something else, and stripping those corollaries would destroy project-specific records while gaining nothing. Only the entry that states the rule *generally* — *a recorded measurement can encode a constraint you have not found yet* — is reduced to a pointer at the canonical source, per [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *referencing means a pointer, never a copy*. It is therefore outside the second promotion round's count, which is keyed to the classification tables in this spec's `plan.md`.
- **One of the six does carry a general clause, and it stays — recorded here rather than left to be rediscovered.** The `§`-reference entry concludes *a measurement is a claim, and its scope is part of the claim*, which generalises. It is not reduced to a pointer because it is the moral that entry draws from **its own** three scoping errors, so it is evidence for the rule rather than a second statement of it, and [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s pointer rule targets a **reproduction of canonical content**, not an aphorism a narrative arrives at. The bound matters: reducing every sentence that gestures at a general principle would be unbounded, and would strip the worked instances that make the corollaries usable where a contributor meets them. The line to hold is a restatement of *what the rule requires* — method and units — which none of the six makes.
- **Discharging it by adding an eighth `AGENTS.md` entry would commit the condition it describes.** Seven scattered statements of one rule is the defect; an eighth is that defect committed again, and it would also be the population error [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *Shared knowledge stays in git* now rules out — a universal rule routed to a project-only file because of what kind of thing it is.
- **Nothing enforces this, and that is stated rather than implied.** No check reads a number in prose and asks what it counts. A gate could compare `examined` against `scope` but cannot know whether a Summary's *1 of 47* used the same denominator the primitive did — which is the exact shape that has already produced a contradiction inside a single `write-review` call. Like the rest of §grounding it is a governed requirement, cited by review and analyze when a claim rests on an unstated method, and a rule implying enforcement it does not have would be the defect it describes.
- **It does not require a measurement where none is owed.** A qualitative claim carries no denominator and needs none. The rule binds a *number offered as evidence* — a count, a ratio, a size, a coverage figure — not every sentence that happens to contain a digit.
- **A stated method can still be wrong, and the rule does not claim otherwise.** Stating the denominator makes a disagreement diagnosable rather than correct; the instrument may be miscalibrated, as the candidate generator whose marker list matched neither of its own confirmed instances was. What the rule buys is that the next reader can *find* that, instead of choosing between two bare numbers.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
