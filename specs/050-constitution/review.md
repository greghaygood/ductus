---
spec: 050-constitution
last-run: 2026-09-28T00:39:38Z
reviewed-against: b2ab256509cc662c2ed40522f97af135a2bb7edd
diff-base: e0a426a4a58c052e403d8b73929e23dc863ee34f
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 30
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
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 050-constitution

## Summary

Re-review after the reopen at d3f5ff4a, which added the scenario `a-status-commit-holds-only-the-transition` and its task 25. b2ab2565 implements it. §spec-lifecycle's restore bullet now states the condition its advice to commit a status transition as its own step depends on: when the commit is made, the working copy of the spec file holds the transition and nothing else. It also states the reason, which is that the pre-commit hook restages each staged spec file whole from the working tree, and the remedy. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, no stored decisions, no observations.

**Base.** `diff-base` is e0a426a4 (d3f5ff4a^), the derived default. No narrower base exists. 050's work in the window is d3f5ff4a and b2ab2565, and every commit between them belongs to 008, 017 or 036, so a base late enough to drop those commits would also drop d3f5ff4a. There is no earlier review of this reopen to narrow from; 050's previous review (79ddf0eb) predates it.

**Scope.** 30 in scope: 27 modified since the base, unioned with 5 plan-listed Affected Files, two of which overlap. Examined 4, each read in full: `framework/constitution.md`, read in full at session start, with the changed bullet at `:203` written and re-read in this pass; this spec's `spec.md` and `tasks.md`; and `scenarios/a-status-commit-holds-only-the-transition.md`. Not counted, named by group:

- `AGENTS.md`: read only at a3b514c5's one added bullet, verified in 017's review. Its §spec-lifecycle mirror at `:78` was also read, and it points at the section rather than restating it, so it needs no change.
- Three plan-listed files unchanged since the base, not read: `specs/050-constitution/plan.md`, `specs/inbox.md` and `specs/045-decision-state-drift-detection/spec.md`.
- 13 files of 008's work, covered by 008's records (f76b2227, 6f43a9d9, 7a8760fb): `.claude/commands/ductus/analyze.md`, `.github/workflows/runtime.yml`, `framework/commands/analyze.md`, `framework/rules/security-backend.md`, `framework/rules/security-frontend.md`, `runtime/src/primitives/rule_sections.rs`, and 008's `analysis.md`, `plan.md`, `review.md`, `spec.md`, `tasks.md` and two scenarios.
- 017's `spec.md`, `data-model.md`, `review.md` and `analysis.md`: covered by 017's records (e584179d, 8241d3b0).
- 036's `spec.md`, `tasks.md`, `review.md` and `analysis.md`, and `framework/rules/quality-cross.md`: covered by 036's records (7a791a98, 5b953340, cf4ab36e).

**Passes.** All 11 rule files were loaded. Security, reuse, efficiency and simplicity found no subject in a constitution-prose change.

Quality checked the new text against the scenario and against the code it describes. It meets the scenario's Behavior: the condition, the reason stated with the rule, and both remedies. It meets all four Edge Cases, including the one where the parked copy predates the transition and has to be re-applied. The hook claim matches both hooks: `.githooks/pre-commit:121`–`:130` and the shipped `framework/bootstrap/hooks/ductus-pre-commit:157`–`:170` take the staged spec files, run `label-criteria` on each, then `git add` it from the working tree. No command source or `AGENTS.md` entry gives transition-commit advice the new text contradicts. The one sentence the edit moved, on stash, restore and hard reset, still belongs to the restore hazard it follows.

The full local surface is green: markdownlint, all six `scripts/lint-*.sh`, and `scripts/audit/run-all.sh`, whose promotion-coverage notice lists 8 unclassified entries, as before, and is informational. `cargo test --release --locked` passed 1713 with 0 failed.

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
