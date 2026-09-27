---
section: "Rule Format"
---

# A-statement-carries-one-obligation-keyword

## Context

The rule format's severity classification makes a rule's Statement the source of its tier: "The Statement's RFC 2119 keyword determines the severity `/{project}:analyze` reports", MUST and MUST NOT blocking, SHOULD and SHOULD NOT non-blocking. The same table requires that "Rules MUST use exactly one of the four keywords in the Statement. Mixed keywords (e.g., "MUST … SHOULD") are not permitted; split such rules into two entries" (`specs/008-security-rules/data-model.md:111`–`:120`). `specs/017-derive-dont-ask/data-model.md:75`–`:84` restates the invariant for configuration rules.

The shipped rule files do not hold to it. Counted 2026-09-27 over the Statement block quote of every `### {ID}` section in `framework/rules/*.md`, with MUST NOT counted as MUST and SHOULD NOT as SHOULD, 16 of 192 rules carry both a MUST-family and a SHOULD-family keyword:

- `framework/rules/security-backend.md`: `BE-AUTHN-007`, `BE-AUTHN-010`, `BE-AUTHN-014`, `BE-DEPS-001`, `BE-ERR-002`, `BE-LOG-003`
- `framework/rules/security-frontend.md`: `FE-CSP-001`, `FE-CSP-003`, `FE-PII-003`, `FE-STORAGE-002`, `FE-XSS-008`
- `framework/rules/performance-frontend.md`: `FE-FONT-002`, `FE-LOAD-001`, `FE-LOAD-002`
- `framework/rules/api-backend.md`: `BE-ERRENV-001`, `BE-PAGE-001`

Every one is a `BE-`/`FE-` ID, whose format this spec's data model governs. No check enforces the invariant, which is how 16 violations reached the shipped files. Three Statements also use MAY, which is not one of the four keywords — `BE-PAGE-001` (one of the 16), `BE-INPUT-010`, and `BE-DATA-001` — counted over the same block quotes.

Surfaced while clarifying `060-exec-analyze-assesses-each-loaded-rule`, whose exec walker reads each rule's tier from its Statement. That spec settles the walker's reading of a mixed Statement as MUST-tier, so a mixed rule is assessed once and never in both tiers. It does not settle whether the rule files or the invariant is the thing to change.

## Behavior

Every shipped rule's Statement and the rule format's severity classification agree: a reader, or a tool, takes one tier from each Statement without a precedence rule the format does not state. Whichever way the open questions below resolve, no rule in `framework/rules/` is left violating the invariant as the data models state it.

## Edge Cases

- **Splitting keeps the existing ID on the obligation citations rely on.** Under ID stability an existing ID is never renumbered or reissued. A split keeps the existing ID on one half and mints the next free sequence number in its category for the other, so every existing citation still resolves to a rule.
- **Adopter copies.** The security rule files ship with the `update` strategy, so an adopter's unpinned copy receives the change on the next `/ductus` run. A pinned copy does not, and keeps the mixed Statements until its owner acts.
- **Two data models state the invariant.** A change to the invariant itself, rather than to the rule files, changes `specs/008-security-rules/data-model.md` and `specs/017-derive-dont-ask/data-model.md` together, or leaves them disagreeing.

## Open Questions

- Are the 16 mixed rules split into two entries each, as the invariant directs — a MUST rule and a SHOULD rule, one of them under a newly minted ID — or is the invariant relaxed to admit a SHOULD clause refining a MUST rule, with the format stating that such a rule is MUST-tier? Splitting mints up to 16 IDs and changes what adopters cite; relaxing changes two data models and makes the format match the reading `060-exec-analyze-assesses-each-loaded-rule` already adopted.
- Does a check enforce the resolved invariant, so a mixed Statement cannot reach a shipped rule file again? None does today, and the format's other integrity requirements (missing fields, malformed IDs, duplicate IDs) are stated as analyze blocks in this spec's Edge Cases.
- Is MAY permitted in a Statement? The severity classification names four keywords and assigns MAY no tier, and three shipped Statements use it (`BE-PAGE-001`, `BE-INPUT-010`, `BE-DATA-001`).

## Resolved Questions

*None yet.*
