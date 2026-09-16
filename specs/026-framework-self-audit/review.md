---
spec: 026-framework-self-audit
last-run: 2026-09-16T15:51:43Z
reviewed-against: 8e238dabc40e0d620c1c2e525832be254296430a
diff-base: ddfd95edc715fa17c24bb0a8273bf96c4814bb51
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 22
skipped-passes: []
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 035bb7ef52c236135d791f35c3ec407d7908c8d51a169483bce40bc26c7a46c8
  scenarios/family-10-migration-coverage.md: 8684e643e4ea938cbeeed6d1342aef27efc84a7bf7559b111b6a12cacacfa02b
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: e2f5e57dc51efec37f63fdc419d1c78ae4afb8319bcd41b604a2c22507af6296
  scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
  scenarios/family-19-review-freshness.md: 07adf4f5f7f0590e90c043d9254534f10f77fc717fb951bd7dfe2ff42a74d787
  scenarios/family-19-says-what-it-examined.md: 2ee343d1295dfc755ea19ae453e5edd61dfb7cdb3d1eecdd46865d54be48621d
  scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
  scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
  scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: f5f69dd3e566a0e07ef2825d5843ec76cf18203355c7c7ddff8737673dfadb09
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
---

# Review — 026-framework-self-audit

## Summary

Re-review for spec 057's retirement of audit Family 31, which reopened this spec to discharge 057's `cross-spec-impact:` entry. Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers (`process-waivers` returned empty in every bucket).

**What changed.** 057 merged the review record's two homes into `review.md`, which leaves Family 31 — the check that reconciled them — with an empty subject by construction: its subject was defined as the *intersection* of specs carrying both records. The primitive, its schema types, `scripts/audit/review-block-agreement.sh` and the family's registry entries were removed by 057's task 14. Here that is discharged three ways: a signpost at the top of `spec.md`, AC22 annotated rather than deleted, and `scenarios/family-31-review-block-agreement.md` deleted outright per §scenarios, which says an obsolete scenario is deleted rather than given a status. The worked example the scenario carried is preserved in the AC22 annotation — 031 and 041 both recording `should-violations: 1` in `spec.md` against their own `review.md`'s `0` for weeks, and the hand-moved waiver with no `review.waivers` entry that caused it — because that incident is the argument 057 acted on, and what is genuinely lost is stated with it: nothing now compares `examined` against a non-empty `scope`.

**Two things task 19 did not name, found by reading rather than by grep.** `scenarios/family-19-says-what-it-examined.md` is a contract on `review-freshness.sh`'s coverage line and still specified `G grandfathered (no review: block)` — the predicate task 20 re-pointed. Corrected to `(no review.md)` and verified character-for-character against the shipped line. The scenario gains what its own subject makes it the right home for: keyed on the block's absence, the relocation would have made that predicate true of every spec, so Family 19 would have examined nothing and exited **green** — this scenario's own failure mode arriving through the predicate rather than the exit code. Its "three siblings" list is annotated rather than rewritten, with the observation that one of the three has since been retired for exactly the fault it shamed Family 19 over. Second, `framework/commands/audit.md`'s Family 19 entry described the check as reading `review.reviewed-against`; corrected with `scripts/audit/README.md`'s matching line, in a separate commit because their subject is the documentation sweep rather than this spec.

**The registry surfaces were checked rather than assumed.** Family 28 holds `run-all.sh`'s registered set, `audit.md`'s enumerated set and `README.md`'s script list in agreement, and it is green — but a green parity check over three sets that all dropped the family together proves only that they agree. Each was read directly: `run-all.sh` has no `review-block` registration, `audit.md`'s numbered list runs 26, 30, 32-38 with no 31, and `README.md` carries no `review-block-agreement.sh` entry. Family numbers are permanent identifiers and 31 is not reused, matching what Behavior §3 already records for the retired Family 3.

**Scope.** `diff-base` ddfd95ed, 22 in scope. Examined **6 of 22**, and the sixteen unread are named rather than folded into the numerator, because most of this scope is the plan's Affected Files list rather than this window's changes. Read in full: this spec's `spec.md`, both of the scenarios above (one of them immediately before deleting it), 047's `analyze-run-durability.md` and `spec.md`, and 057's `tasks.md`. Read only in the regions this review's claims assert on: `framework/commands/audit.md` (the family registry and the Family 19, 26, 30, 32-38 entries), `framework/constitution.md` (§spec-lifecycle, §implement-phase, Frontmatter Schema, Validation Severity) and `scripts/audit/README.md` (the Family 19 entry) — each confirmed against the live file where a claim here rests on it, none read end to end. **Not read at all: nine `scripts/audit/*.sh` files** (`adopter-shell-behavior`, `check-zero`, `cross-doc-consistency`, `introducing-drift`, `manifest-parity`, `placeholder-roundtrip`, `sibling-coupling` beyond its header, `ssot-invariants`, `template-alignment`), the two `.github/workflows/*.yml` files, `runtime/legacy-prose-commands.txt`, and `.claude/commands/ductus/audit.md`, the generated mirror of a source that was not fully read either. None of them changed in this window; that is a reason to expect them clean, not evidence that they are, so they are counted as unexamined.

**Passes.** Security, reuse and efficiency had no subject — this window's changes are a file deletion and prose corrections. Quality carried the weight against `quality-cross.md`: the Family-19 coverage-line correction *is* a `QUAL-CLAIM-001` repair, since the contract as written would have been satisfied by a family reporting every spec grandfathered over zero examined. Simplicity: deleting the scenario rather than marking it superseded is the simpler of the two dispositions and the one §scenarios requires; the AC22 annotation carries the content that had to survive it.

**One judgement recorded because it decided what was not done.** Family 7 (sibling-spec coupling) went red during this work, naming 057 against 020, 047 and 026 as bundling candidates. Its subject is pairs of **non-`done`** specs, so the finding is an artifact of three specs being reopened at once and clears when they close. It was not suppressed with the `Why split from …` contract the family provides: the pairing is a scheduling coincidence, and recording a split rationale for it would put a permanent suppression in the corpus for a transient state.

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
