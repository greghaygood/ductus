---
spec: 008-security-rules
last-run: 2026-09-27T21:54:15Z
reviewed-against: b0b60c6266609bfb215f6457fc0f8841707cce28
diff-base: 6f43a9d9fa22cd52647685cc69dfa01744505bd8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 10
skipped-passes: []
reviewed-digest:
  data-model.md: 4ab1f01634418baee8b7f06d1348ceacbfff8b665703782bfcb408ecb3cf89bb
  scenarios/a-statement-carries-one-obligation-keyword.md: 346b88a803f48662192dda28a853aa0d8bd879e1d2b4108f9a3a08f2bb81ecc2
  scenarios/x-frame-options-carries-one-strength.md: 8cf5c99537d8f9cf81233184b4b13d76f7c3bc66a0ce96f0effb075572e53fc4
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 008-security-rules

## Summary

Re-review for task 12, which the completion gate's criteria check raised: AC18 and AC21 were found unmet against the tree. 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers, no stored decisions. One observation, fixed in the run.

**What changed.** `framework/commands/analyze.md`'s Rules section again reports a malformed rule file and a duplicate rule ID as Blocking findings, with the file's rules withheld from steps 11 and 12. Those are the two messages 008 and 016 specified, which 022's parseable rewrite (11aad341) dropped with no recorded decision. The section now also states the exec reduction: the walker checks no rule-file integrity. Also in the window: analyze's three grounding fixes to `plan.md` (7d106937), its record (c48608a7), and the task-12 tick.

**Base.** `diff-base` 6f43a9d9, the previous review's commit, passed with `--since`, on the same reasoning as that review: everything outside this delta is covered by 6f43a9d9's and f76b2227's records.

**Scope.** 10 in scope, examined 4: `framework/commands/analyze.md`, `specs/008-security-rules/plan.md`, `analysis.md` and `tasks.md`. For each, this review read its whole change since the base in context, and the Rules section of `analyze.md` in full. The six not counted: `.claude/commands/ductus/analyze.md`, the generated mirror, regenerated after each edit, whose Rules section was compared with the source's and differs only by the generator's `/{project}:` → `/ductus:` substitution; and five plan-listed Affected Files unchanged since the base. Of those five, `framework/rules/security-backend.md`, `framework/rules/security-frontend.md`, `framework/constitution.md` and `specs/008-security-rules/data-model.md` were read in full earlier this session, and `framework/bootstrap/ductus.md` only in its Shared Files manifest (lines 645–662 and 559).

**Passes.** Loaded: api-backend, concurrency-backend, configuration-cross, observability-backend, performance-backend, quality-cross, reliability-backend, security-backend. Security, reuse and efficiency had no subject: the change is command prose and spec records. Quality: the restored text's claims hold against the tree. 11aad341 removed both messages, `runtime/src/interpreter/analyze_tally.rs:182`–`:187` records the two reasons it names, and `runtime/src/primitives/rule_sections.rs` reads no Rationale and detects no duplicate. It raised one observation, fixed in b0b60c62: since MAY became a tierless keyword, "no RFC 2119 keyword" missed a MAY-only Statement, so three places now say "no MUST or SHOULD keyword". The wording avoids a "-tier" token inside step 11, whose prose the exec walker's `severity_from_step_prose` reads. The runtime suite passed after each command-file edit: 1713 passed, 0 failed. Simplicity found nothing.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: analyze.md described the rules recorded unexamined as those whose Statement carries no RFC 2119 keyword, missing a MAY-only Statement, which carries one and still has no tier — `framework/commands/analyze.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
