---
spec: 025-rule-opt-out
diff-base: fd5bb62ba9931a2794904a5d402e481a97b81013
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:24:13Z
reviewed-against: fd5bb62ba9931a2794904a5d402e481a97b81013
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 5
reviewed-digest: {}
blocking: false
---

# Review — 025-rule-opt-out

## Summary

First review of 025 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. **All nine criteria hold in substance**; one carried a citation that had silently come to point at the wrong rule.

**What this review read: the whole scope.** All five files — `framework/commands/review.md`, `framework/commands/status.md`, `framework/constitution.md`, `framework/bootstrap/ductus.md`, and this spec's own `spec.md` — read in full. Nothing in scope went unread.

**The defect: a line-number pointer into a living document.** AC9 cited "**AGENTS.md line 42**" for the rule that a `.ductus/config.toml` key is documented in the spec that owns it rather than retro-added to an earlier config spec. `AGENTS.md` has grown since; line 42 is now the runtime-release entry, so the citation resolves to a rule about tagging releases. The criterion's *requirement* was never in doubt — the schema is documented in this spec's body and in `review.md` §Inputs, exactly as claimed — but its citation had rotted. Replaced with the entry's own wording, which survives an edit above it. Two more of the same shape sat in `plan.md` and `tasks.md`, pointing at "lines 246–262" of `ductus.md` for the example TOML block, now near line 573. This is §drift-prevention's dead-reference failure in the one form a grep for the *name* can never find: the pointer's target moves without the pointer being touched.

**How each criterion was checked, against `review.md` rather than against this spec's own prose.** AC1: §Inputs documents `[[review.disabled-rule-files]]` as array-of-tables with required `file` (basename) and `reason`, and states the threshold as "trimmed length ≥ 16 Unicode codepoints" — the codepoint wording matters, since the criterion's whole point is that a non-ASCII reason is judged by visible length. AC2: both notice forms are present verbatim, including the `(no-op)` variant for a file stack detection would not have selected, and the whitespace-collapse rule for multi-line TOML. AC3: the unknown-file warning fires and is non-fatal; the criterion already carries a note recording that its wording generalised to "the rule-file directory", because that directory is `specs/rules/` in an adopter and `framework/rules/` here — I confirmed the current wording matches the note rather than the original. AC4: malformed entries warn with the indexed form and are not auto-removed, and `review.md` states that all four warning forms "do not affect the exit code" — which is the load-bearing half, since it is what keeps `blocking: true` meaning a MUST violation. AC5: the duplicate warning fires and only the first entry applies. AC6: `status.md`'s rendering step emits the `disabled rule files: …` line when the config is present and the array non-empty. AC7: the key is `.ductus/config.toml` state, not spec frontmatter, so nothing in `analyze.md`'s frontmatter tiers touches it. AC8: the filter matches on basename with no carve-out anywhere in the selection path — the absence is the claim, so it was checked by reading the whole filter rather than by finding a positive statement. AC9: the schema appears in `review.md` §Inputs and in `ductus.md` §Project Configuration, which also documents that `/ductus` does not itself read the key.

**One neighbouring fact confirmed while reading.** `ductus.md`'s example TOML no longer shows `[workflows]` — 043 sunset it — so the plan's description of the block's neighbours was stale in a second way, now noted rather than silently corrected.

**On the diff base.** No commit records 025 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 025 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
