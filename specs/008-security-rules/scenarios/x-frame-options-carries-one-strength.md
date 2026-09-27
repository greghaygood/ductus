---
section: "Two Rule Files"
---

# X-frame-options-carries-one-strength

## Context

The two security rule files state the `X-Frame-Options` header at different strengths. `BE-API-001` in `framework/rules/security-backend.md` is MUST-tier, and its Verification lists `X-Frame-Options: DENY` (or `SAMEORIGIN`) among "the minimum required headers and their values" for HTML responses. `FE-CSP-008` in `framework/rules/security-frontend.md` states the same header as a SHOULD: a clickjacking fallback for browsers that do not honor CSP `frame-ancestors`, which `FE-CSP-003` requires at MUST.

The disagreement predates the split that minted `FE-CSP-008`. Its clause was `FE-CSP-003`'s SHOULD half before `a-statement-carries-one-obligation-keyword` split it off. It surfaced in 008's review of that split on 2026-09-27, when both files were read in full.

A project loading both files is told the header is required and advisory at once. A missing `X-Frame-Options` is reported twice: as a blocking finding under `BE-API-001` and an advisory one under `FE-CSP-008`. In `/{project}:review` the two do not merge, because its cross-pass dedup keys on the rule ID.

## Behavior

The two rule files state one strength for `X-Frame-Options`, and a project that loads both is told the same thing by each. The rule that carries the header's obligation names the other for context rather than restating it at a different strength.

## Edge Cases

- **A backend-only project.** It loads `security-backend.md` without `security-frontend.md`, so whatever `BE-API-001`'s table says is the whole of what it is told. Whichever strength is chosen has to be stated there, not only in the frontend rule.
- **Adopter copies.** Both files ship with the `update` strategy. An unpinned copy receives the change on the next `/ductus` run, and a pinned copy keeps the disagreement until its owner acts.

## Open Questions

- Which strength does `X-Frame-Options` carry: MUST, as `BE-API-001`'s minimum-required header table says, or SHOULD, as `FE-CSP-008` says? A MUST keeps the header blocking for every HTML-serving project, including one whose CSP `frame-ancestors` already covers every browser it targets. A SHOULD means `BE-API-001`'s table stops listing the header as minimum-required and points at `FE-CSP-008`. That leaves a backend-only project, which does not load the frontend file, with the header named but no rule requiring it.

## Resolved Questions

*None yet.*
