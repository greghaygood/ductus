---
spec: 011-brownfield-process
diff-base: b195ba16754c62ca10f7ee7a79f2b80096595765
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:10:23Z
reviewed-against: b195ba16754c62ca10f7ee7a79f2b80096595765
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 16
reviewed-digest: {}
blocking: false
---

# Review — 011-brownfield-process

## Summary

First review of 011 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All nineteen criteria verified against the tree; **two did not hold** and one more surfaced a contradiction that is not this spec's to settle.

**What this review read: 7 of the 16 files in scope, and here is the other 9.** Read in full: `framework/constitution.md`, `framework/commands/groom.md`, `help.md`, `specify.md`, `framework/templates/project/inbox.md`, `README.md`, and `AGENTS.md`. **Not examined, and why each:** `framework/bootstrap/ductus.md` — 1181 lines / 141 KB, read only by targeted search for this spec's subject (the `triage` → `inbox` rename, which survives there as an ordinary English verb and never as the artifact or command name); two generated `.claude/commands/ductus/{groom,help}.md` copies, held against their sources by check-zero; `specs/006-bug-workflow/spec.md`, read in full during 006's own review earlier today; and **five paths that no longer exist** — `commands/capture.md`, `.claude/commands/ductus/capture.md`, `commands/triage.md`, `.claude/commands/ductus/triage.md`, `ductus/ductus-auggie.md` and `sdd-context.md`, which the plan lists as the record of what the implementation touched.

**AC10 was false.** It asserts inbox items "never remain standalone". The framework later added the **chore** route, under which an item with no feature home deliberately *does* stay an inbox checkbox until it is done and removed — §bug-handling's durability test, and §brownfield-inbox's "a chore … stays an inbox item however close to the current work it surfaced". The §Inbox body carried the same claim plus "the goal is for `specs/inbox.md` to eventually be empty and deleted", which §brownfield-inbox now contradicts outright: the brownfield backlog drains, the file persists, because incidental capture is ongoing. Body rewritten; the criterion annotated rather than rewritten, since the claim did hold for the requirement gaps this spec's inbox carried.

**AC18's subject no longer exists.** `sdd-context.md` was removed by the framework/ reorganization (`3fc76b7`) with no successor; the brownfield process it required is documented in the constitution and `README.md`, which AC17 and AC19 already cover. Superseded rather than renamed, so annotated.

**AC14 was falsified by this campaign and is now repaired.** It requires 006 to carry a signpost recording the `triage` → `inbox` rename. `3c4fe76` — an earlier commit of this same backfill — removed that note from 006 along with two genuine rename annotations. Nothing caught it: no check verifies cross-spec signposts, and both specs stayed green. §drift-prevention's rule against annotating a rename preserves exactly one exception, "the few references that record the decision to change it", and a §cross-spec-impact signpost is that reference. Restored in `80b817e` as a blockquote, so `derive-dependencies` induces no edge.

**A damaged decision record, repaired.** §Rename from triage read "`specs/inbox.md` → `specs/inbox.md`", and `tasks.md` "rename `specs/inbox.md` to `specs/inbox.md` if needed" — an earlier sweep had substituted the new name onto the *left-hand side* of the very mapping that records what the old name was. This is the second instance in this campaign, after 006's "`/ductus:groom` is now `/ductus:groom`".

**How the rest were checked.** AC1–AC6: `specify.md` accepts both input richnesses with sparse criteria valid, writes from the spec template, lists its reads in Scope Boundaries with no source-code read, creates no scenario, calls `write-session` at step 10, and checks naming conflicts. AC7: holds — it says "without requiring **comprehensive** criteria", and sparse is not zero. AC8, AC9: §brownfield-process's incremental-growth list. AC11: `groom.md` step 2 and its Step 2 reference both route a no-covering-spec item to `/{project}:specify`. AC12, AC16, AC17: the `§scenario-promotion`, `§cross-spec-impact` and `§brownfield-process` markers all resolve in the constitution. AC13: the rename is complete — `triage` survives in the bootstrap only as an English verb. AC15: 007 carries its signpost. AC19: `README.md` §Brownfield adoption.

**Recorded, not decided.** Verifying AC7 surfaced a four-way contradiction over whether a `draft` may carry *zero* criteria: `analyze.md` makes it blocking and unscoped by status, `check-artifacts` implements no such family so the runtime path passes the same spec, and `specify.md` plus this spec's own body both say an empty section is valid. The two execution paths therefore answer differently, which §runtime-boundary forbids. It is logged to the inbox rather than resolved here — it sets whether the brownfield entry path is gated on day one, which is an operator call.

**On the diff base.** No commit records 011 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 011 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
