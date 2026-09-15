---
spec: 026-framework-self-audit
reviewed-at: 2026-09-15T18:36:19Z
reviewed-against: 1110d6fc9634a77b5fa09b53f3341beef902cfa7
diff-base: 6014d53caae11bcba77bc868829234207e49b459
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 18
scope: 20
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Five passes over the resolved scope. **`examined: 18` of `scope: 20`**, and the two not counted are named individually rather than folded into the numerator.

**`.claude/commands/ductus/audit.md`** — the generated mirror of `framework/commands/audit.md`, which was read in full. `scripts/gen-claude-commands.sh` was re-run in this pass and reported all 18 command copies regenerated, so there are good grounds to believe it correct; that is confidence, not a read, and `examined` is only the second. **`.github/workflows/markdown-only-pipeline.yml`** — **absent**. It stays in scope because `plan.md` lists it, and its absence is grounded rather than assumed: 026's own §Resolved Questions records that `048-govern-acquired-runtime` removed that workflow and the `/audit` step now runs as `(h) Framework self-audit` in `.github/workflows/framework-checks.yml`.

The other 18 were read in full this session, including all eleven rule files `discover-rule-files` reports under `selected`, loaded before the passes ran.

**Security.** No source in scope but shell. The change adds six alternatives to a literal-built regex in `rename-sweep-residue.sh`; no input crosses a boundary, nothing is eval'd on user data, and `shellcheck -S warning` is clean over all 55 tracked scripts. The surface-specific rule sets (security-backend/frontend, api-backend, concurrency, observability, reliability, performance, accessibility, configuration-cross) verify design-time commitments this change makes none of — read and found to have no subject here, which is a different statement from checked-and-passed.

**Quality — `QUAL-CLAIM-001`, the rule this change is most exposed to, since the family *is* a claim-checker.** Three checks. The widened family still reports its examined-file count on stderr and still treats a degenerate scan as a finding rather than a pass, so the property that made it honest is unchanged. The header states its own limit outright: a ditransitive sentence (*"give ductus its due"*) would be a false positive, measured absent across the 584 tracked markdown files as of today. And the correction it carries is of exactly this shape — the family previously exited 0 over 514 files with live residue present, which is a check that could not see what it exists to find, and the record now says so instead of retaining *"exactly the 8 real sites, no others"*.

**Reuse.** The construction count is now stated in five places — script header, scenario, AC17, `framework/commands/audit.md`, `scripts/audit/README.md`. That is the framework's existing shape for all 37 families and Family 28 mechanically holds only the family-number registration, not the descriptions, so the prose copies are held by discipline. The disposition taken was the sweep `AGENTS.md` prescribes for a canonical set: all five were located and updated in one pass, and the task records that as a subtask rather than leaving it to be rediscovered.

**Simplicity.** `POSSESSIVES` is a separate variable rather than six more alternatives inside `FOLLOWERS`. Functionally identical; kept separate because the scenario and both registries now describe *three* constructions and a reader matching prose to code should find three lists.

**Two things the probe found that reading did not, recorded because they are the evidence the widening is calibrated rather than guessed.** Probing the **narrowed** direction — not the widened one — is what caught both: `my` had to be dropped from the possessive class because `README.md` documents adoption as `/ductus my-project`, so the front page puts the name before `my` in correct prose; and the inbox item recording the ninth site had quoted the defect in `*italics*` without the double quotes the family's stripper protects, so the widened run reported the item describing the bug. Neither was predictable by reading the pattern.

**Calibration, re-derived rather than quoted.** `govern its` occurs exactly once at `9da4a7ae^` — the `api-backend.md` site — and `their` / `our` / `your` / `his` / `her` zero times each. The whole closed class ships anyway because the existing lists are grammar-calibrated, not frequency-calibrated: at that same commit three of the seven followers (`whether`, `these`, `those`) and seven of the eight modals had zero instances and are listed regardless. That measurement is what makes adding the plural consistent with precedent rather than speculative.

**Pre-reopen window, measured before the flip because it is not recoverable after it:** base `c1b00ea8`, **81 modified-since / 93 in scope at 52,446 bytes**. Post-reopen: **8 / 20 at 4,234 bytes** on `6014d53c`, a 12x byte reduction, both legs inline.

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
