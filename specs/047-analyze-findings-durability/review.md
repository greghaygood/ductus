---
spec: 047-analyze-findings-durability
diff-base: c52596803bfa98e62cf44f524982a6bd231aa8d1
captured-issues: 0
skipped-passes: []
last-run: 2026-09-14T01:14:10Z
reviewed-against: 1024110d4661812a811ceb51c5c51e09c75516ef
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 8
reviewed-digest:
  scenarios/analyze-record-freshness.md: b79ba39909899e6f8662e8882e04b54e66c04cd17febae782af1443cdd05091d
  scenarios/analyze-run-durability.md: 287dacba26b62b8f72ba377b8f07f92063cdd28a351ede9568035dd0fd811046
blocking: false
---

# Review — 047-analyze-findings-durability

## Summary

Re-run 2026-09-13 as the examined/scope backfill pass. The prior record (2026-09-07) predated `ductus-v0.49.0` and carried no `examined`, no `scope` and no `reviewed-digest`, so its `0/0/0` was indistinguishable from a run whose five passes never fired. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. No waivers: `process-waivers` reports 0 applied, 0 expired, 0 retained.

**Scope, and what was actually read.** The natural diff base collapsed from 315 modified-since / 315 in scope to **4 / 8** once this pass's correction commit recorded a fresh `in-progress` transition. `--since HEAD` gave 0 / 4 and was declined: it excludes by construction the four files this pass edited. Examined **7 of 8** — every in-scope file was read end to end except one, which is named rather than folded into the numerator:

- `.claude/commands/ductus/analyze.md` is the **generated mirror** of `framework/commands/analyze.md`, which was read in full (399 lines). `scripts/gen-claude-commands.sh` ran in this session and reported all sixteen commands in sync against a clean `git status`, so the mirror is provably identical to a source that was read. Believing it correct and having read it are different claims, and `examined` is only the second.

`framework/constitution.md` is counted: it was read in full at session start — 761 lines across four contiguous ranges — as `/{project}:target` step 4 requires.

**Rule coverage.** All 11 rule files loaded and all 105 IDs enumerated. None has a subject in a scope that is one constitution, one command source, one generated mirror and five spec artifacts, with no code. `QUAL-CLAIM-001` was assessed closely rather than waved past, because this spec's entire subject *is* that rule turned on the pipeline: the analyze record exists so that a run which never happened cannot read as a clean one, `unexamined` is recorded even at zero because "a zero that was computed and a field that was never written are not the same claim", and the `skipped` list keeps `clean` narrow. The command source applies the rule correctly at every point it invokes it, including the restored `No rule files found` advisory. Compliance, not violation.

**Four corrections landed in this pass, three of them in durable contracts.** The largest is §drift-prevention's *a previously-rejected option is adopted*, occurring entirely inside one spec: `scenarios/analyze-run-durability.md` resolved "should the gate also check analyze freshness?" with **not yet**, and its sibling `scenarios/analyze-record-freshness.md` then built exactly that gate — answering the learned-ignored objection by removing its cause, replacing the sha diff with a content comparison against `analyzed-digest`. The freshness scenario cites `022`'s scenario as where the superseded reasoning lives and never mentions the sibling under its own spec carrying the identical decision, so the bullet stood as a live "not yet" over a shipped gate (AC13-AC15, and `check_review_gate.rs`'s `AnalyzeStale`). Two count words went stale alongside it: `check-review-gate` "gains two block reasons" where it carries three, and the review `analyze` row rendered "in all three states" where it renders four — a scenario contradicting itself, since its own Edge Cases describe the fourth. The spec's first Resolved Question rejected an `analysis.md` partly because "analyze findings drive no gate and are read by no command", which this spec's own AC9-AC10 falsified; the decision survives because the gate reads the record's *counts* while the rejected artifact would have held finding *content*.

**Verified rather than assumed, on the two criteria most likely to be taken on trust.** AC12's four-state enumeration was checked against the **rendered** labels in `framework/commands/review.md`, not against the `RecordFreshness` variants, which are named differently (`never-run` / `current` / `stale` / `undeterminable`) — comparing the wrong pair would have manufactured a defect. AC10's "ordered after every `review:` check" was read from `run()`'s control flow rather than from source line order, which puts `ReviewStale` at line 423 *after* `AnalyzeStale` at 376 and reads as a contradiction until you notice those are helper definitions, not call sites. AC11 was checked by running Family 37 directly: 54 done specs examined, backlog **0** against baseline **0**, so the grandfathered set is fully drained and the ratchet closed.

**One in-scope observation that is not this spec's to fix.** `framework/commands/analyze.md`'s §Spec integrity still lists "Acceptance criteria section exists with at least one checkbox item" as blocking and unscoped by status, which is the subject of a standing inbox item carrying an operator decision (2026-09-13) that zero criteria are valid at `draft`. It is a single markdown edit that reopens no spec, but it is that item's work rather than this pass's, and bundling it would make this commit non-uniform across two unrelated concerns.

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
