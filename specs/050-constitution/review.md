---
spec: 050-constitution
last-run: 2026-09-28T13:29:20Z
reviewed-against: a99314b536e687292b80a70be0f1bf51b825bfff
diff-base: 545795e3353da654ba3abba7d9653ce6ca4ccc94
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 6
skipped-passes: []
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: 6936b866607a842ece8ebfd749223d4f7d98637200e8db87d85772702589e98a
  scenarios/a-declared-cross-spec-impact-gates-done.md: 17265775c8d619c2cad70f6ce7785c9734aaa3d08cf287785a6a91fde68ac981
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 8d6851e20f8a2e9f083434a37cbf39ab3bca533ec819a8a258197e4afb797b07
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
  scenarios/a-status-commit-holds-only-the-transition.md: 0d3fb1a701bc4b3112cfc2e54bb488df060c75fceeb0dc6c6b39205eb01d4438
  scenarios/completion-claims-carry-no-caveats.md: 2b2e43f4cea73ba9c81dc21848b5c668e1db5bf6a5e3becd7467ce025c4179c4
  scenarios/findings-route-by-scope.md: a29ba00826b6109390c701ebd3eabdd8a1bd687f972f77339e57b629c6122226
  scenarios/governance-is-multi-source.md: ae59aca7a049317806297839a73cf335eeb3764db97999a7ddc0b12103e6ffeb
  scenarios/knowledge-routes-by-population-not-by-kind.md: 0e934ea65cbaf2bf31acc4f45c461bfb868be45deb5aa9fa6601c59b8c003c5c
  scenarios/report-outcomes-not-edits.md: caf13342d7fe0a5af4108024cf424ab921030c896f0743307cf5111be2e163ab
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

Reopen for 061 (how a constitution-only change reaches adopters). Default diff base: the parent of this reopen's done -> in-progress commit, so the window is the reopen itself; compute-review-scope reports scope 6 from the plan's Affected Files, of which spec.md and plan.md were modified since the base. Five passes (security, reuse, quality, efficiency, simplicity) against all 11 rule files discover-rule-files selected, each read in full. 0 MUST, 0 SHOULD, 0 low-confidence. examined: 4 of 6 -- spec.md and plan.md, both read in full after their edits, and AGENTS.md and framework/constitution.md, both read in full this session in their current state (AGENTS.md after 061 task 8's edits; the constitution is unchanged by 061). One observation, fixed in the run and committed at a99314b5 before this record: AC12's closing clause said a constitution-only change never moves the pin, which 061's release policy made false for delivery; the criterion's own claim still holds and the clause is annotated. The twelve scenarios are unchanged in this window, so the durable-contract digest is unchanged. NOT read against this reopen, named rather than counted, unchanged in this window: specs/045-decision-state-drift-detection/spec.md and specs/inbox.md.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: AC12's closing clause said a constitution-only change never moves the pin, which 061's release policy made false for delivery — `specs/050-constitution/spec.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
