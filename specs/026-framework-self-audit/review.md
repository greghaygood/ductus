---
spec: 026-framework-self-audit
last-run: 2026-09-28T13:21:20Z
reviewed-against: f3d9fb69aa765f6d446803b221a7315b6566ebff
diff-base: e4c2d1cb4518dce7ebde2bfddb470e35b982103e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 16
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
  scenarios/family-36-self-url-resolution.md: 23716b0f5c3810890bab089c65258008842869e35616a71b86b663ed231bde18
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 026-framework-self-audit

## Summary

Reopen for 061 (Family 36's main-URL rationale and slug source). Default diff base: the parent of this reopen's done -> in-progress commit, so the window is the reopen itself; compute-review-scope reports scope 16 from the plan's Affected Files, of which spec.md (the status flip) and scenarios/family-36-self-url-resolution.md (the restatement and signpost) were modified since the base. Five passes (security, reuse, quality, efficiency, simplicity) against all 11 rule files discover-rule-files selected, each read in full, plus AGENTS.md read in full. 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations. examined: 2 of 16 -- spec.md and the scenario, both read in full; the scenario's two restated bullets were checked against scripts/audit/self-url-resolution.sh as committed at a654662a (read in full, outside this scope), whose slug grep now keys on the codeload tar.gz URL and whose report was byte-identical before and after that change. The only durable contract changed is this one scenario; the other twenty are unchanged in the window. NOT read against this reopen, named rather than counted, all unchanged in this window: .claude/commands/ductus/audit.md (generated), .github/workflows/runtime-release.yml, framework/commands/audit.md, runtime/legacy-prose-commands.txt, scripts/audit/{adopter-shell-behavior,check-zero,cross-doc-consistency,introducing-drift,manifest-parity,placeholder-roundtrip,sibling-coupling,ssot-invariants,template-alignment}.sh (runtime-release.yml and audit.md were edited by 061 tasks 6-7 before this window). ABSENT but in scope because the plan lists it: .github/workflows/markdown-only-pipeline.yml, removed by 048.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
