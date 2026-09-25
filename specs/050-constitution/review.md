---
spec: 050-constitution
last-run: 2026-09-25T18:29:44Z
reviewed-against: 79ddf0eb12df2d2bc3273436786fb7f0e93b9c42
diff-base: f5ddd6f01496fb7225af87ad65d2d6c5465c687e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 78
skipped-passes: []
reviewed-digest:
  scenarios/a-canonical-source-is-pointed-at-not-copied.md: 6936b866607a842ece8ebfd749223d4f7d98637200e8db87d85772702589e98a
  scenarios/a-declared-cross-spec-impact-gates-done.md: 17265775c8d619c2cad70f6ce7785c9734aaa3d08cf287785a6a91fde68ac981
  scenarios/a-measurement-states-its-method-and-units.md: 4c250a480c11e9788058f1012b647daa14aba2036d4a3baa9d0c1ba3c5965b2a
  scenarios/a-partial-read-is-not-a-read.md: 8d6851e20f8a2e9f083434a37cbf39ab3bca533ec819a8a258197e4afb797b07
  scenarios/a-retired-feature-leaves-no-spec.md: eadf56734b7018bdf20fc4c6b03d274f46c36bba6ba65ec43975a099eaaa98bb
  scenarios/a-retired-filename-leaves-a-decision-record.md: f84a3177744eb03a859d82a4017495c24bcf24466448b7c49ad1a253147ab581
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

Review of 050's reopen (c421d0e1, scenario `report-outcomes-not-edits`) together with spec 058's cross-spec discharge (058 task 26). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers. No observations.

**What changed.** Two durable contracts. `scenarios/report-outcomes-not-edits.md` is new: an agent's chat replies report outcomes rather than reproducing edits, with the rule as a §pipeline-boundaries bullet. The constitution carries that bullet once, and the scenario's edge cases (asked-for changes, approval gates, failures, locations, durable artifacts) match it. `scenarios/findings-route-by-scope.md` is annotated for 058: a signpost opens its Behavior, and its chore, no-spec-in-progress, and spanning-finding edge cases say what 058 superseded. `spec.md` annotates AC15 and opens its Acceptance Criteria with a 058 signpost. `plan.md`'s Family 38 row was re-keyed in 058 task 22 to `AGENTS.md`'s reworded machinery-observation entry, and that entry's lead phrase matches it.

**Scope.** `diff-base` f5ddd6f0, 78 in scope, examined **3**: `spec.md` and both changed scenarios, read in full. The window is wide because it opens at the commit before 050's reopen, which is also where 058's implementation began. Five of the 78 are 050's own files; of the rest, `framework/constitution.md` and `AGENTS.md` were checked only at the new §pipeline-boundaries bullet, §brownfield-inbox Finding dispositions, and the re-keyed entry, and everything else is 058's runtime, command, doc, and discharge work (plus 045's spec, a historical Affected File), reviewed under 058. Of 050's own, `plan.md` was checked only at the re-keyed row, and `tasks.md` is not reviewed.

**Passes.** Security, reuse, and efficiency had no subject: prose. Quality: each annotation was checked against the constitution section it names. Simplicity: annotation keeps the scenario's reasoning readable without restating 058.

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
