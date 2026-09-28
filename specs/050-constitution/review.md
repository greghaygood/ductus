---
spec: 050-constitution
last-run: 2026-09-28T00:54:02Z
reviewed-against: 6072c266816079e07147b96ccdc2618d52100aa1
diff-base: d2af331b14e8232ed7041eff9957d9495639b7c5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 8
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

Re-review for tasks 26 and 27, which the completion gate's criteria check raised against AC2, AC3 and AC22. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, no stored decisions, no observations.

**What changed (6072c266).** Task 26 re-promotes W6, *never record anything durable in `review.md`*, which 054 had withdrawn from the constitution on 2026-08-31 along with the superseded-criterion rule it was promoted inside. It is now a bullet under §implement-phase, worded for the waiver and decision lists 058 made persistent. The `AGENTS.md` entry is a pointer to it, with its lead phrase byte-identical, and `plan.md`'s W6 row records the round trip. The in-substance feature-directory entry gains a §numbering pointer for its universal half. Task 27 states the rule-file destination in the spec's own §Classification, as AC22 says; before this it was only in `plan.md`'s §Classification.

**Base.** `diff-base` is d2af331b, the previous review's commit, passed with `--since`, on the precedent 008's re-reviews set: everything before it is covered by d2af331b's record. The derived default, e0a426a4, resolves 32 files. This base resolves 8.

**Scope.** 8 in scope, examined 5, each read in full: `framework/constitution.md`, read in full at session start, with the new bullet written and read in this pass; this spec's `spec.md`, `plan.md` and `tasks.md`; and `analysis.md`, 050's own record. Not counted:

- `AGENTS.md`: 217KB, read only at the two entries changed, `:47` and `:132`.
- `specs/inbox.md` and `specs/045-decision-state-drift-detection/spec.md`: plan-listed and unchanged, not read.

**Passes.** All 11 rule files were loaded. Security, reuse, efficiency and simplicity found no subject in constitution and spec prose. Quality checked the new text against its sources:

- The bullet's claim, that the report body is regenerated whole and only the waiver and decision lists carry forward, matches `framework/commands/review.md`: "The report is regenerated on every run — never appended", waivers preserved on survivors, and decisions persisting.
- Both pointers use fragments the constitution resolves: `#implement-phase`, and `#numbering-convention`, which is what the constitution's own §numbering links use.
- A 54-row check found every promoted, already-promoted, in-substance and rule-file row in `plan.md` §Classification resolving to an `AGENTS.md` entry that cites the constitution or `QUAL-DELEG-001`. The count was 52 before these tasks.
- The anchor set is unchanged, and the spec and plan heading counts are unchanged.

The full local surface is green: markdownlint, all six `scripts/lint-*.sh`, and `scripts/audit/run-all.sh`, whose promotion-coverage notice now counts 78 constitution-citing entries, up from 76, with 8 still unclassified. `cargo test --release --locked` passed 1713 with 0 failed.

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
