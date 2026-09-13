---
spec: 033-rule-surface-setting
reviewed-at: 2026-09-13T13:55:00Z
reviewed-against: 6fe820ee7a0443b14f7dc9b7bf6995063ebee2ce
diff-base: 6fe820ee7a0443b14f7dc9b7bf6995063ebee2ce
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 6
skipped-passes: []
---

# Review — 033-rule-surface-setting

## Summary

Clean: 0 MUST, 0 SHOULD, 0 low-confidence outstanding. Re-review for the examined/scope backfill, against a corpus record that predated ductus-v0.49.0 and therefore carried neither `examined` nor `reviewed-digest`.

Scope and what was read. `examined: 6` of `scope: 6` — every in-scope file was read end to end, in ranges, with no gaps: README.md (314 lines), framework/bootstrap/ductus.md (1181), framework/commands/analyze.md (391), framework/commands/review.md (892), specs/020-code-review/spec.md (802), specs/024-rule-loader/spec.md (89). No file went unread, so this Summary names none.

Diff base. `HEAD` was passed deliberately. The natural base (`find_in_progress_commit`) is f67cdde6, 2026-06-28, resolving an 806-file window spanning every change since June — a review nobody performed. 033's plan affects six files. Both were measured before choosing, per AGENTS.md §Gotchas; `HEAD` makes the window empty by construction, so the scope resolves to the plan's Affected Files and numerator and denominator describe the same six files.

Rule selection. All 11 rule files loaded (`discover-rule-files`, no `[rules] surfaces` set here). quality-cross.md and configuration-cross.md were read in full — they are the cross-cutting sets and the ones this spec's own contract cites (`CFG-ENV-003` fail-fast, `QUAL-CLAIM-001`). The nine surface-specific sets were read at ID-and-Statement level and produce no findings by construction over this scope: they govern HTTP contracts, auth, database queries, XSS/CSRF, WCAG, metrics and retries, and the scope is six markdown documents in a project with no HTTP surface, no database and no browser. `CFG-ENV-007` (documented, consistent precedence) is satisfied — `[rules] surfaces` documents config-file → stack derivation → all-surfaces in four places.

Acceptance criteria. All ten verified against the tree, not against the prior review. AC1/AC9 against runtime/src/primitives/discover_rule_files.rs (`VALID_SURFACES`, `validate_surface_member`, `resolve_surfaces`' unset branch); AC2/AC4/AC5 against ductus.md §Collect Project Inputs item 4; AC3/AC5/AC8 against §Shared Files' surface filter and `[pinned]`; AC6 against review.md §Behavior step 5 and §Inputs; AC7 against analyze.md's load-all note; AC10 against the config schema block, review.md §Inputs, analyze.md and README. All ten hold, so 033 did not reopen. `check-artifacts` was clean with an empty `skipped` array — nothing was left for a hand-check.

Three defects were found and fixed at source in 6fe820ee, before this record was written; the counts above state what is outstanding, which is nothing. (1) ductus.md requires every AskUserQuestion to carry 2–4 example choices and then listed them for four of the five inputs — Rule surfaces, this spec's own input, was never added when fafb52b1 introduced it. (2) ductus.md cited "State B step 2" for the permission write; that was correct at spec 031, when State B had three steps, and 048 prepended two, making it step 4. `check-step-references` cannot see it — the reference sits outside the `## Instructions` list, so it lands in `references-out-of-subject: 106`, counted rather than resolved. (3) review.md said `write-review` writes six frontmatter fields plus waivers; `render_review_yaml` writes four more — `scope` and `reviewed-digest` always, `examined` when stated, `reviewed-unreadable` when a contract is unreadable. All three are the same shape: a prose enumeration that went stale while the set it names grew.

Two observations are recorded rather than fixed, both outside this spec's scope and both measured; they are captured to the inbox by this same call.

Not swept, deliberately: 020's §Embedded artifacts (lines 242–802) is a verbatim snapshot of review.md as delivered, and ductus.md's "17 commands" sits inside a past-incident narrative where seventeen was correct. Both are records of what was, not claims about what is.

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

- convention: three live artifacts disagree on a rule file's required fields. framework/constitution.md §rules says four (ID, Statement, Rationale, Verification); framework/commands/analyze.md's rule-integrity check says three (Statement, Rationale, Verification — ID is checked separately as the level-3 heading, so this is self-consistent); framework/bootstrap/ductus.md §Security Audit says "the same integrity checks /{project}:analyze uses" and then names four (Statement, Rationale, Verification, Source). Measured 2026-09-13: Source is present on 73/73 security-backend and 32/32 security-frontend rules, so the bootstrap claim is true for its own subject (it loads only those two files) and no behaviour differs today; it is false about analyze.md, and configuration-cross carries Source on only 7 of 11 rules, so the stricter reading would treat that file as unloadable if the audit's subject ever widened. Deciding which enumeration is canonical belongs to 008/026, not 033. — `framework/bootstrap/ductus.md:834`
- convention: present-tense "frozen archaeology" claims survive in six specs, and AGENTS.md's entry on them now points at two files that were already fixed. The rule was deleted from the constitution by 023's living-specs scenario, so a present-tense claim that it applies is false. Measured 2026-09-13: specs/020-code-review/spec.md:177, specs/016-cross-cutting-rules/spec.md:69 and :84, specs/016-cross-cutting-rules/plan.md:81 and :111, specs/018-adopter-owned-pre-commit/spec.md:155 (a ticked AC) and :170, specs/019-config-decisions/plan.md:89 and :98, specs/019-config-decisions/tasks.md:69, specs/017-derive-dont-ask/plan.md:7, :114, :215 and :235 (which also cites a constitution anchor, §done-specs-are-frozen-archaeology, that no longer exists), and specs/007-govern-workflow/scenarios/govern-self-update-precheck.md:9 (inside a cross-spec signpost blockquote — reword, never delete). AGENTS.md line 62 names specs/024-rule-loader/spec.md and plan.md as "the last surviving present-tense instances"; both were corrected on 2026-09-13 and now read past-tense, so that pointer sends a reader somewhere there is nothing to find — the same failure the entry itself records about its previous pointer. Correctly past-tense and off-limits to the sweep: all of 023 (the record of the removal), 024's review.md, 050's plan.md:191, and runtime/CHANGELOG.md. Each affected spec takes the back-edge, so this is a per-spec correction, not a uniform substitution. — `AGENTS.md:62`

## Skipped passes

*None.*

## Unexamined governance

*None.*
