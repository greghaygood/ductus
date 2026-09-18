---
spec: 045-decision-state-drift-detection
last-run: 2026-09-18T00:19:20Z
reviewed-against: 7340a41d16af2b90fec56ff362c5a2da3952a9c9
diff-base: 1ccd8fdf7fbcd8f32352dc6c86fb95150880516c
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 13
scope: 26
skipped-passes: []
reviewed-digest:
  data-model.md: c2d0f42b02c5bbd6f6d35932f68cc0a41534defd0ea3484aa7e3c073564564bc
blocking: false
---

# Review — 045-decision-state-drift-detection

## Summary

Five passes over the sixth exemption group this spec's `criterion-path-existence` family gained: a criterion asserting a path was never created is exempted whole, recorded as `not-a-live-claim`, and stated as AC19. **0 MUST, 0 SHOULD outstanding.**

The finding this fixes was reported from an adopter running 0.52.0 and is exactly backwards in the sense this spec's data-model already names: the path is absent, which is precisely what the criterion asserts. It could not be a fifteenth phrase — matching is a flat `contains`, so `was created` would exempt positive delivery claims and blind the family, while `was never created` misses the reported word order. Both directions are proven by probe against a fixture reproducing the criterion.

**Grounding** — the recorded decision at `spec.md:124` rejects checking a removal-phrased criterion by *inverting* it, and the implementation it rejected was also clause-scoped. Verified that this is not that decision re-adopted: there the clause scoping decided attribution, which is the semantic judgment §runtime-boundary places at an extension point; here it decides only whether one further exemption applies, the exemption stays whole-criterion, and the fourteen phrases are still matched across the whole criterion — so an appended annotation suppresses exactly what it suppressed before, and the six findings that returned under the rejected design cannot return under this one. The distinction is recorded in the scenario rather than left for a reader to re-derive. **Security / efficiency** — nothing reached; the predicate is one allocation-free pass over a criterion's words. **Reuse** — the canonical table stays the single source; `analyze.md` restates it because adopters have no copy, and 18e now binds the two. **Simplicity** — this is a precision fix only: no criterion that was silently exempted becomes a finding. **Quality** — AC19 states the clause scoping, which is the half that could regress silently.

**What these passes read: 13 of 26 in-scope files** — the runtime change, the three restatements, the audit family, and the spec artifacts carrying the contract. **Not read:** `framework/constitution.md` beyond §recommendations and a targeted sweep for marker-list claims (it carries none), `runtime/CHANGELOG.md`, `runtime/Cargo.toml`, `runtime/src/mcp/server.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/schema/primitives.rs`, 022's `data-model.md`, `spec.md` and three sibling scenarios, and 026's `spec.md`. None is touched by this change.

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
