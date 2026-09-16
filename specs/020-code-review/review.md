---
spec: 020-code-review
last-run: 2026-09-16T12:34:24Z
reviewed-against: 267070e4bcf15cb354fa20997e1b13d38f2650db
diff-base: 1f5539dbc98706aa8811c10cc484d59901eea9fa
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 16
skipped-passes: []
reviewed-digest:
  data-model.md: a375bcf75753b408e0b3bb7afb218d6b5dcf5dbcc06beebfd30c9fcae833be10
  scenarios/review-flag-parsing-is-specified.md: 9f1a3dd82bab2b9622808c31dd2bb00f0c6f403effeb8bb496bb4b329496e9c7
  scenarios/waiver-expiry.md: a06cf9e1d638fefd25d2edcf9d38a7ad56fa0b7e74ab048e4e77ce2878df77ca
blocking: false
---

# Review — 020-code-review

## Summary

Re-review for spec 057's record relocation, which reopened this spec to discharge its `cross-spec-impact:` entry. Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers (`process-waivers` returned empty in every bucket).

**Three defects were found by the quality pass and fixed in this pass rather than recorded as findings**, so they are absent from the counts above by being gone at `reviewed-against` rather than by never having existed. All three were in `framework/templates/ci/adopter-generators.yml`'s audit-record gate — the surface AC5 names as the third gate mechanism, rewritten by 057 — and all three were the same shape that step was rewritten to remove: a record with no `blocking` field read as `blocking: false`; `last-run: NULL` or `Null` read as a real timestamp; `blocking: True` read as not-blocking. Each exits 0 silently, in the one check whose whole subject is hand-edited frontmatter, which is precisely how such a record arises. `blocking` is **required** on both records by the constitution's §Frontmatter Schema and the gate turns on it, so absent is a state the step could not examine and not a clean one — `QUAL-CLAIM-001`. Proven by probe in **both** directions rather than by reading: four fixtures run against the pre-fix step exit 0 with zero errors on all three, against the fixed step exit 1 naming the record, and the clean control still exits 0. Fixed at `267070e4`. Nothing in `cargo test`, `run-all.sh` or the markdown lint covers this template, so the probe in that commit message is the only evidence there is, and it is stated rather than implied.

**What else changed.** 057 merged the run record's two homes into `review.md`, renaming `reviewed-at` to `last-run`, and replaced this spec's CI predicate. AC4, AC5, AC8 and AC15 are annotated in place rather than restated; a signpost at the top records both halves and links back to 057. AC15's `examined` / `scope` half survives intact and its **enforcement** half is gone with audit Family 31 — recorded as a real loss rather than smoothed over, with the observation that the family could not have carried the claim far anyway, since one `/ductus:review` call wrote both sides from the same values.

**`data-model.md` had two tables for one record, and that is the drift this spec's own AC15 contradicted.** The `review:` block's table was missing `examined`, `scope` and `reviewed-digest`; `review.md`'s was missing `captured-issues`, `examined` and `scope`. They collapse to one table, re-derived field-by-field from `write_review.rs`'s render order rather than from either prior table — including `reviewed-unreadable`, which neither had. The body-sections table gained `Captured issues`, `Observations` and `Unexamined governance`, stale since 047, 022 and 055 respectively; that is outside 057's scope and was fixed here because the reopen was already spent and the document claims to be authoritative.

**One claim in `plan.md` was written wrong and corrected before commit**, recorded because the correction is the useful half. Its retired `awk-parsed YAML … replaced by a deterministic runtime check in v2` limitation was first replaced with *the step stays inline because an adopter's CI has no guaranteed runtime* — false: the `derive` step two above it invokes `.ductus/bin/ductus` directly. The grounded reason is that no primitive answers this step's question. `check-review-gate` gates the `in-progress → done` transition and returns early on a spec already at `done`, which is every spec this step examines, so aiming it here would report `passed` over the whole corpus; `dashboard`'s per-spec payload carries no record fields either. Gate and step are complements, not one wrapping the other. Both were checked against the source before the sentence was rewritten.

**Scope.** `diff-base` 1f5539db, 16 in scope. Examined **9 of 16**, and the seven are named rather than folded into the numerator. Read only in the regions this review's claims assert on: `framework/commands/implement.md` (the pre-done gate, step 5 in full — its nine checks were confirmed consistent with the relocation), `framework/commands/review.md` (922 lines; the Instructions, Flags, Scope Boundaries and markdown-only steps 1–5), `framework/commands/analyze.md` (402; §Review state drift), `framework/constitution.md` (834; §Frontmatter Schema and §Validation Severity). Not read: `README.md` and `.claude/commands/ductus/review.md`, the generated mirror of a source that was not fully read either. And **`framework/templates/spec/spec-and-plan.md` does not exist** — 023's lightweight-track sunset deleted it; it stays in scope because this spec's plan lists it, and is named here as absent rather than silently dropped, as the previous review also did. Its `plan.md` row now says so.

**Passes.** Security had no subject beyond the CI template's embedded `python3`, which takes the spec root through an env var rather than shell interpolation (commented as deliberate) and reads only files in its own checkout. Reuse: the template's hand-rolled `frontmatter()` reader is a fourth frontmatter parser in a repo whose runtime parses frontmatter for a living, which AGENTS.md §Design Principles names — assessed and **not** a finding, on the grounding above that no primitive exposes this step's subject. Efficiency: one `listdir` and two small reads per spec, linear. Simplicity: the gate's branches map one-to-one onto the three record states and none is redundant.

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
