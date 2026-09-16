---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-09-16T16:14:45Z
reviewed-against: edffb74ae4f7c302e5e2ac7e5827fb8bd7e81098
diff-base: a2a698730d15690b273eaa25d089aadaee51c42d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 37
scope: 278
skipped-passes: []
reviewed-digest:
  data-model.md: bd1df805fef2ac01f573d300d4783c13238ba296e80553cbb769c31b5c79c473
blocking: false
---

# Review — 057-analyze-artifact-and-record-relocation

## Summary

The spec's own review. Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers (`process-waivers` empty in every bucket). `compute-review-scope` reports **2** inbox additions in this review window and the record reads `captured-issues: 0` — the two count different things and both are right. The field counts what *this run* appended, which is none; the window's two items were captured by earlier sessions of this same spec's work and stand in `specs/inbox.md` (the `validate-frontmatter` severity-vocabulary item and the `cargo fmt` message-literal item, both out of this spec's scope and recorded as such). They are absent from the Captured issues section below because the CLI surface does not accept them, which is a gap in the primitive's argument coverage rather than a claim about the window — stated here so the section's *None* is not read as the window being empty.

**Scope, and why the numerator is 37 of 278.** 165 of the 278 are the corpus sweep's own output — `spec.md`, `review.md` and `analysis.md` for ~55 specs — and they were **verified mechanically rather than read**, which is the disposition AC10 and AC11 ask for and the reason the sweep is a primitive at all: `framework/migrations/criterion-label-backfill.md` records that a hand edit across a corpus is a silent renumbering waiting to happen, and reading 165 migration outputs to confirm a uniform transformation is the same error facing the other way. What stands behind them instead: `markdownlint-cli2` clean across 574 files, `check-corpus-links` `broken: []` over 483, `derive-dependencies` `drift: false, cycles: []` over 55, the 36-family self-audit green apart from the known Family 20 version pin, and a purpose-written corpus scan that recomputed every done spec's analyze digest and reported its mismatching paths by frequency. Two scope rows are not files at all — `framework/commands/{review,analyze,implement,audit,consolidate}.md` and `specs/*/spec.md` are plan-table rows carried through verbatim.

The **37 examined** are: `framework/templates/ci/adopter-generators.yml` and `framework/templates/spec/spec.md`; `runtime/src/primitives/analyze_subjects.rs`; 022's seventeen touched scenarios; 020's `spec.md`, `plan.md`, `data-model.md` and `waiver-expiry.md`; 047's `spec.md` and both scenarios; 026's `spec.md` and both scenarios (one of them immediately before deleting it); and this spec's `spec.md`, `plan.md` and `tasks.md`. Read in **regions only and not counted**: `framework/constitution.md` (Frontmatter Schema, Validation Severity, §spec-lifecycle, §implement-phase), `framework/commands/implement.md` (the completion gate, step 5 in full), `analyze.md`, `review.md` and `audit.md`, `framework/migrations/audit-record-relocate.md`, `022`'s `data-model.md`, and `write_review.rs` / `check_review_gate.rs` / `check_artifacts.rs` / `schema/primitives.rs` in the functions this spec's claims rest on. Not read at all: 20 runtime test fixtures, 9 tests and goldens, the 6 generated mirrors, `README.md`, `docs/*`, and the two `configure/*.md` permission files.

**What this pass changed rather than confirmed.** Three things were found here and fixed before this record was written, and each is stated past-tense with what holds it.

**AC7 was half-met.** `subject_digest` excised `analysis.md`'s frontmatter and digested its body — which the same `write-analysis` call rewrites — so any run whose report changed staled itself and converged only on a second identical run. Measured on 020, fixed by excluding `analysis.md` from the subject set outright (it is the command's own output, exactly as `review.md` is the review's), and confirmed end to end: a single run on 047 and on 022 now leaves `check-review-gate` at `passed: true`, and neither digest carries an `analysis.md` key. Two tests that pinned the old contract were rewritten to pin the new one rather than deleted — one of them pinned the defect itself.

**The CI gate had three false-green paths.** A record with no `blocking` field read as `false`; `last-run: NULL` read as a real timestamp; `blocking: True` read as not-blocking. All three in the step AC17 delivers, and all three the shape it was rewritten to remove. Proven by probe in both directions: four fixtures exit 0 with zero errors against the pre-fix step and exit 1 naming the record against the fixed one, with the clean control still passing. Nothing automated covers that template, so the probe is the only evidence and the commit says so.

**The documentation sweep had missed the constitution.** §spec-lifecycle and §implement-phase both still placed the review record in `spec.md` frontmatter — in the document that ships to every adopter and that §drift-prevention makes the source the other copies point at. Task 17's classifier had *reported* both; the file was waved into the allow-list on the strength of its two correct hits without the other two being read, which is AC26's check failing in the reader rather than in the pattern.

**AC26 is met by classification, not by an empty search, and that is the criterion's own shape.** 118 location hits survive, all classified: 17 in this spec's artifacts, 8 in `runtime/CHANGELOG.md`, 2 in the migration entry and procedure, 3 in the constitution's residual-block violation and `validate_frontmatter` implementing it, and 88 past-tense accounts, spent `tasks.md` entries and annotated criteria. **The limit is recorded with the result:** a classifier keyed on `spec.md` / "spec frontmatter" / "the spec" cannot see a sentence that names the record and no file, and that class hid 047's whole `analyze-run-durability` Behavior section and most of 022's 41 corrections — against an estimate of 10. An enumeration is a claim, and a pattern that cannot fire on a class is `QUAL-CLAIM-001` wearing a regex.

**Passes.** Security: the one executable surface in the window is the CI step's embedded `python3`, which takes the spec root through an env var rather than shell interpolation and reads only its own checkout. Reuse: the relocation removed a whole check (`check-review-agreement`) rather than re-pointing it, and `is_analyze_subject` reuses `ANALYSIS_RECORD_FILE` rather than a second literal; deleting `strip_record_frontmatter` removed a helper whose only call site the fix made unreachable. Quality, against `quality-cross.md`: this is the rule family the whole spec turns on, and the three fixes above are its three shapes — a gate that could not judge exiting 0, a record that staled itself, and a green classifier over an unread file. Efficiency: one fewer file hashed per analyze digest. Simplicity: one record with one home, one timestamp spelling, and one fewer audit family.

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
