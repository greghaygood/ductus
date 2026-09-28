---
spec: 050-constitution
last-run: 2026-09-28T15:11:13Z
reviewed-against: dff464a543dfbf0973d991613be8fa5f82fe4351
diff-base: ba689d02ce82ee7ab3073501a5c768ff6a1621e7
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 10
scope: 14
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

Scoped re-review of 050's reopen for the third classification round (task 28), which classifies the 9 AGENTS.md entries the promotion-coverage notice reported unclassified. Diff base is ba689d02, the parent of the reopen commit. 0 MUST, 0 SHOULD, 0 low-confidence, no observations; not blocking.

Examined 10 of the scope, each read in full as a diff over the window: specs/050-constitution/plan.md, tasks.md and spec.md, framework/constitution.md, AGENTS.md, framework/rules/quality-cross.md, framework/commands/analyze.md, and specs/036-quality-cross-rules/spec.md, data-model.md and tasks.md, which are in this window because 036 was reopened alongside for the rule this round routed there. The quality pass checked several things:

- Each third-round key resolves to its entry: promotion-coverage reports 129 classified, 0 unclassified, and no unmatched keys.
- The promoted §scenarios paragraph is the only normative statement of its rule; its distinctive phrase occurs once in framework/constitution.md and not in AGENTS.md (AC3).
- The paragraph states the two no-write shapes append_task.rs:94-118 actually has, correcting the entry's drifted phased-file claim.
- The constitution's anchor set is byte-identical to HEAD before the round (AC10).
- 050's cross-spec-impact on 036 is still discharged by 036's extended signpost.

Not opened: .claude/commands/ductus/analyze.md, generated from its source (gen-claude-commands --check in sync); specs/045-decision-state-drift-detection/spec.md and specs/inbox.md, plan-affected but unchanged in this window; and specs/036-quality-cross-rules/review.md, a record write-review wrote this session, of which only the frontmatter was read to verify the recorded sha.

Local gate after the change: every framework-checks step and scripts/audit/run-all.sh exit 0; cargo fmt, clippy -D warnings and cargo test --release --locked exit 0 (1713 passed, 20 result lines for 20 targets).

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
