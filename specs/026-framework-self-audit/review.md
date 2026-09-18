---
spec: 026-framework-self-audit
scenario: family-18-marker-list-parity
last-run: 2026-09-18T00:19:27Z
reviewed-against: 7340a41d16af2b90fec56ff362c5a2da3952a9c9
diff-base: 1ccd8fdf7fbcd8f32352dc6c86fb95150880516c
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 13
scope: 30
skipped-passes: []
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 035bb7ef52c236135d791f35c3ec407d7908c8d51a169483bce40bc26c7a46c8
  scenarios/family-10-migration-coverage.md: 8684e643e4ea938cbeeed6d1342aef27efc84a7bf7559b111b6a12cacacfa02b
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 8d48555ad2ec2a7983bfa8dac24c6edf9dcbc051037ae6b00c9ec5e26f767e4d
  scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
  scenarios/family-19-review-freshness.md: 07adf4f5f7f0590e90c043d9254534f10f77fc717fb951bd7dfe2ff42a74d787
  scenarios/family-19-says-what-it-examined.md: 2ee343d1295dfc755ea19ae453e5edd61dfb7cdb3d1eecdd46865d54be48621d
  scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
  scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
  scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: f5f69dd3e566a0e07ef2825d5843ec76cf18203355c7c7ddff8737673dfadb09
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
---

# Review — 026-framework-self-audit

## Summary

Five passes over Family 18's new `18e` arm, which binds the negated-creation predicate's two word lists across the same three restatements 18a-18c bind the phrase list. **0 MUST, 0 SHOULD outstanding.**

One finding was surfaced by these passes and fixed in `a0036cbd`, before this record: 18e skipped a consumer whose derivation came back empty. That is a silent pass in precisely the shape this family exists to refuse — a restatement that still exists but no longer parses reads as agreement — and it is the fail-open 18a was written to avoid. It now emits, and the arm is proven by reformatting `analyze.md`'s bullet so its groups no longer parse while the bullet itself survives.

**Grounding** — an exit code is not evidence a check ran, so 18e was verified by injected drift in four directions, not by the clean baseline: a word dropped from `CREATION_VERBS` (caught, with the stale declared length reported alongside), a word added to `analyze.md` alone, the canonical section renamed (the fail-closed empty derivation), and the unparseable-but-present restatement above. The restored baseline exits 0. **Reuse** — 18b and 18e parsed the same Rust array shape with a copy of the literal regex each; that is now one `rust_literals` helper. **Simplicity** — 18e reuses 18c's structural filter (only parenthesised groups that are entirely comma-separated code spans) rather than inventing a second convention, and compares the two word lists as one union, matching 18a-18c's stated position that the contract is the set and the grouping is editorial. **Security** — read-only, no writes outside the contract, unchanged. **Quality** — deliberately no count arm: the predicate's prose states no count, and that choice is recorded in the script, the scenario and the README rather than left implicit.

**What these passes read: 13 of 30 in-scope files** — the audit family and its README entry, the runtime constants it binds, the three restatements, and the spec artifacts carrying the contract. **Not read:** the other family scripts under `scripts/audit/`, `run-all.sh`, `framework/commands/audit.md`, 026's `spec.md`, `plan.md` and its other scenarios, and the CI workflows. This change adds one arm to one family and touches none of them.

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
