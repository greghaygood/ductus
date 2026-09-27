---
spec: 008-security-rules
scenario: a-statement-carries-one-obligation-keyword
last-run: 2026-09-27T21:19:32Z
reviewed-against: aaabeac83d5010822a76b2763332a6dcf54ec358
diff-base: f0b59a9eba1f17dbe584c25734e4728490c163f2
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 18
scope: 23
skipped-passes: []
reviewed-digest:
  data-model.md: 4ab1f01634418baee8b7f06d1348ceacbfff8b665703782bfcb408ecb3cf89bb
  scenarios/a-statement-carries-one-obligation-keyword.md: 30253c5d9973feb19132ecd42268a2c53268b1361c90574bd766adfa522ed0ca
  scenarios/x-frame-options-carries-one-strength.md: a8563493f3d5c96be5fc64563421da861a4bd22cfaeb8ef13488ddafe2197940
blocking: false
dispositions:
  fixed: 5
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "bug: BE-API-001 lists X-Frame-Options among its minimum-required headers at MUST tier while FE-CSP-008 states it as SHOULD — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/x-frame-options-carries-one-strength.md
    decided-at: 2026-09-27T21:19:32Z
    decided-by: andy@stone.dev
---

# Review — 008-security-rules

## Summary

Re-review for the reopen on scenario `a-statement-carries-one-obligation-keyword` (task 9) and the disposition task it produced (task 10). 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, no stored decisions. Six observations: five fixed in the run, one routed.

**What changed.** 14 of the 16 rule Statements carrying both MUST- and SHOULD-tier keywords were split into a MUST rule under the existing ID and a SHOULD rule under a new one: `BE-ERRENV-003`, `BE-PAGE-003`, `FE-LOAD-003`, `FE-LOAD-004`, `FE-FONT-003`, `BE-AUTHN-015`, `BE-AUTHN-016`, `BE-AUTHN-017`, `BE-ERR-004`, `BE-LOG-007`, `FE-XSS-009`, `FE-STORAGE-004`, `FE-CSP-008` and `FE-PII-004`. `BE-DEPS-001` and `FE-CSP-001` dropped a SHOULD clause their own MUST already implied. The format states the invariant in tier terms, with MAY as a tierless permission and without "one sentence", in 008's, 017's and 036's data models and the constitution's §rules summary. The runtime gained the corpus test `no_shipped_statement_mixes_tiers`, and `runtime.yml` now triggers on `framework/rules/**`.

**Base.** `diff-base` f0b59a9e, 060's `done` commit, passed with `--since` in place of the derived base f1fe19b1, the parent of 008's reopen commit 05f42d3a. 008 was reopened during 060's clarify, so the derived window held 060's whole implementation: 36 commits and 52 files, 30 of them covered by 060's review fd960cb1 and 022's reviews dba5175f and 5715ed7b. The narrowed window holds every 008 commit except the reopen itself, whose three files later commits modify again.

**Scope.** 23 in scope, examined 18, each read in full: the four edited rule files, `runtime/src/primitives/rule_sections.rs`, `.github/workflows/runtime.yml`, `framework/constitution.md` (read in full this session, one line changed since), 008's `data-model.md`, `research.md`, `spec.md`, `tasks.md` and both scenarios, 017's and 036's `spec.md` and `data-model.md`, and 050's new scenario. The five not counted: `AGENTS.md`, whose only change in the window is one project-only gotcha line from 9e4a0880, which was read; `framework/bootstrap/ductus.md` and `framework/commands/analyze.md`, this spec's historical Affected Files, unchanged in the window; 050's `spec.md`, whose only change is its status line, read as a diff; and 050's `tasks.md`, of which lines 180–233, holding the appended task 25, were read.

**Passes.** Loaded: api-backend, concurrency-backend, configuration-cross, observability-backend, performance-backend, quality-cross, reliability-backend, security-backend. Security and efficiency found nothing: the code in scope is one test module and a CI trigger list. Quality: every rule parses with a Verification and one tier, and every citation of the 16 IDs resolves (`check-rule-ids`, 0 missing). The full runtime suite passed at 767b5b89, and the rule-section tests, fmt and clippy passed again after the `tokens()` refactor. Reuse produced one observation, fixed. Simplicity found nothing.

**Observations.** Five were fixed with confirmation, in dccec94f and aaabeac8. The second commit corrected a line citation the first had made stale. One was routed: `BE-API-001` lists `X-Frame-Options` as minimum-required at MUST tier while `FE-CSP-008` states it as SHOULD. The disagreement predates the split. It is now scenario `x-frame-options-carries-one-strength`, with an open question, and task 11.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: the keyword scenario's Q2 resolution named the tier check as part of every_shipped_rule_is_assessable, which was built as no_shipped_statement_mixes_tiers, and cited rule_sections.rs lines that had moved — `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md` — **fixed**
- convention: the keyword scenario's Context described the pre-split state in the present tense, which the split made false — `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md` — **fixed**
- convention: spec.md's RFC 2119 keyword enumeration omitted MAY, which the rule format now permits as a tierless permission — `specs/008-security-rules/spec.md` — **fixed**
- convention: runtime.yml's header said framework edits stay off the workflow, while its command-file and rule-file paths trigger it — `.github/workflows/runtime.yml` — **fixed**
- reuse: the corpus check's mixes_tiers re-implemented RuleSection::tier's tokenization, so the two could drift — `runtime/src/primitives/rule_sections.rs` — **fixed**
- bug: BE-API-001 lists X-Frame-Options among its minimum-required headers at MUST tier while FE-CSP-008 states it as SHOULD — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/x-frame-options-carries-one-strength.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
