---
section: "Follow-on scenarios"
---

# Severity-is-a-closed-set-not-a-string

## Context

Every finding the runtime produces carries `severity` as a bare `String`: `ReviewFinding` (`schema/primitives.rs:417`), `FrontmatterFinding` (`:1313`) and `ArtifactFinding` (`:3669`). The tiers themselves are spelled as inline literals — 16 occurrences in `check_artifacts.rs`, 15 in `validate_frontmatter.rs`, 3 in `schema/extensions.rs`, 2 in `schema/primitives.rs` — with no constant, no enum, and no validation on deserialize. The doc comments name the legal values; nothing enforces them.

**The field holds two different closed sets under one type.** `ReviewFinding` uses `must` / `should`; `FrontmatterFinding` and `ArtifactFinding` use `hard-fail` / `blocking` / `advisory` / `informational`. They share a name and a `String`, so nothing prevents a value from one vocabulary being emitted into the other.

**On the review side the failure direction is permissive, which is what makes this more than hygiene.** `write_review.rs:118-128` buckets findings with the severity test *last* in an if/else chain whose `else` is the catch-all:

- a finding is `waived` if a waiver matches, else `low` if confidence is low, else `must` if `severity` equals `"must"` case-insensitively, **else `should`**.

`blocking` is then `must_n > 0` (`:130`). So any severity string that is not exactly `"must"` — a typo, a trailing space, a value borrowed from the analyze vocabulary — is silently filed as a SHOULD, `blocking` is written `false`, and the spec passes `check-review-gate`'s MUST check. `finding_rank` (`:402`) derives its ordering from the identical test, so a misspelled MUST also *loses* cross-pass dedup to a correctly-spelled SHOULD, and the surviving copy is the weaker one.

The reachable path is not a source typo. `findings` is `write-review`'s argument, supplied by the host from the `performReview` extension point — the severity string is written freehand by an LLM into JSON on every review run. An unvalidated permissive default sits directly on the path that gates `done`.

**The analyze side is the same root cause with a milder consequence**, and the asymmetry is worth recording rather than flattening. Nothing in `runtime/src` branches on `FrontmatterFinding.severity` or `ArtifactFinding.severity` — both results are serialized straight out to the host, which renders per-finding tiers by the prose in `framework/commands/analyze.md` step 2. An unrecognized tier there is mis-rendered, not silently downgraded past a gate. That absence of a branch is exactly why [`frontmatter-severity-tiers-on-both-sides`](frontmatter-severity-tiers-on-both-sides.md) could correct eight wrong tiers with a string substitution — and equally why those eight wrong tiers survived months undetected.

## Behavior

**A severity tier is a closed set, bound so an unrecognized value cannot be constructed, deserialized, or emitted.** The two vocabularies stay distinct rather than being merged into one enum — they are different tier systems belonging to different commands, and collapsing them would make the cross-vocabulary error representable in the type that exists to forbid it.

- The review tiers (`must`, `should`) and the analyze tiers (`hard-fail`, `blocking`, `advisory`, `informational`) each become their own type, serialized to exactly the strings they carry today, so `review.md` and `analysis.md` are byte-identical across the change and no consumer's parsing moves.
- Deserialization of an out-of-set value is an **error**, not a default. This is the load-bearing half: the current behavior's defect is not that a bad value exists but that it resolves to the permissive tier. A rejected payload names the offending value and the field; it never silently degrades to `should` or to `advisory`.
- `write_review`'s bucketing and `finding_rank` branch on the bound value rather than on a string comparison, so the `else`-is-`should` catch-all disappears by construction rather than by a guard someone has to remember.
- The inline literals across `check_artifacts.rs`, `validate_frontmatter.rs` and `schema/extensions.rs` are replaced by the bound values at every site.
- 022's `data-model.md` result-shape registry records the two closed sets and points at the constitution's [§text-first-artifacts](../../../framework/constitution.md#text-first-artifacts) Validation Severity subsection for the analyze tiers' meaning rather than restating the assignment.

The extension-point contract is where the guarantee has to hold, because that is the reachable path: a `performReview` response carrying an unrecognized severity is rejected with the value named, and the run halts rather than recording a review whose counts silently understate what the passes found.

## Edge Cases

- **A rejected extension response is worse than a demoted finding only if it is silent.** Halting on an unrecognized severity turns a silently-wrong review into a loudly-failed one, which is the intended trade — but the error must name the offending value, the field and the finding, or an operator cannot tell a model typo from a contract change.
- **Case-insensitivity is current behavior and is a decision, not an accident.** Both existing comparisons use `eq_ignore_ascii_case`, so `"MUST"` is legal today. The bound type either preserves that (accepting case variants on deserialize, emitting canonical lowercase) or narrows it — narrowing is a behavior change for any host already sending `"MUST"`, so it needs to be stated either way rather than fallen into.
- **Waived and low-confidence bucketing sit ahead of the severity test and are unaffected.** A waiver match and a low-confidence tier both short-circuit before severity is read, so this change cannot move a finding out of those buckets.
- **The analyze tiers already have a fifth informal member.** `check_artifacts` emits `blocking` and `advisory` only; `hard-fail` and `informational` arrive from other producers and from the host's rendering. The closed set is the constitution's four, so binding it will surface any producer that was relying on a value outside them.
- **No shipped record changes shape.** Because the serialized strings are unchanged, existing `review.md` and `analysis.md` records deserialize under the new types — except one that already carries an out-of-set value, which now fails loudly. That is the check working, and the corpus should be swept for such a record before the change lands rather than discovering it at a gate.
- **This is a `runtime/` change and therefore a release.** Per AGENTS.md §Workflow the tag is cut from completed work: implement, review, analyze, 022 back to `done`, and only then bump the three version sites and push `ductus-v<version>`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
