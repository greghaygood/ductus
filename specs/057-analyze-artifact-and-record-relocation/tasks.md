# 057 — Analyze artifact and record relocation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Task 1 is first by constitutional rule, not by preference: §runtime-boundary
principle 4 requires a schema change to ship through the constitution with the
runtime following. Tasks 2-8 are the runtime; 9-12 the migration and the gates
around it; 13-14 the sweeps; 15-16 verification and the declared obligations.

## 1. Declare both record schemas in the constitution

- [ ] Add a record-schema subsection to `framework/constitution.md`'s **Frontmatter Schema** naming the owning artifact for each record and the fields it carries
- [ ] Leave the spec-file table at its existing five fields — the records were never in it
- [ ] State that `spec.md` carries neither block, so a residual one is a defect rather than an open-schema extra

- **Done when**: the constitution declares both records at their owning artifacts, and no part of the declaration is sourced from a runtime type.

## 2. Move the record types off `Frontmatter`

- [ ] Remove `review` and `analyze` from `Frontmatter` (`runtime/src/schema/primitives.rs:697`)
- [ ] Merge the review record's two former sides into one type: `blocking` and `waivers` from the block, `diff-base`, `captured-issues`, `skipped-passes` from the report
- [ ] Rename the review timestamp to `last-run`, retiring `reviewed-at`
- [ ] Parse each record from its owning artifact's frontmatter

- **Done when**: `cargo test` passes and every field named in `data-model.md` round-trips through its owning artifact with none dropped.

## 3. `write-review` stamps `review.md` only

- [ ] Drop the `spec.md` frontmatter write from `write_review.rs`
- [ ] Write the merged record to `review.md`'s frontmatter

- **Done when**: a review run modifies `review.md` and leaves `spec.md` byte-identical.

## 4. `write-analysis` writes the artifact

- [ ] Render `analysis.md` from a fixed skeleton — Summary, hard failures, blocking findings, advisory findings, unexamined targets
- [ ] Write the record to its frontmatter; drop the `spec.md` write
- [ ] Write on every run, including a clean run and an empty scope
- [ ] Emit no `- [ ]` item in any section

- **Done when**: a clean run and an empty-scope run each produce an `analysis.md` containing zero checkbox items, and `spec.md` is byte-identical across both.

## 5. Move the digest exclusions

- [ ] Excise `analysis.md`'s own record from the analyze digest; digest `spec.md` whole and delete the `analyze:`-block surgery path
- [ ] Leave the review digest subject set unchanged, and annotate in the code that the `spec.md` exclusion's stated rationale lapses once `write-review` stops touching it

- **Done when**: an analyze run immediately following itself reports current rather than stale, and the review digest's subject set is unchanged from before the relocation.

## 6. Point the gate at the new homes

- [ ] Replace the `frontmatter.review` / `frontmatter.analyze` reads in `check_review_gate.rs` (lines 134, 183) with a per-artifact read
- [ ] Report an absent artifact as never-run and an unparseable one as undeterminable — never collapse the second into the first
- [ ] Leave every downstream branch untouched

- **Done when**: a differential run of the gate over the pre- and post-relocation corpus produces identical verdicts for every spec, and the absent and unreadable cases are distinguishable in the output.

## 7. Validate each record at its owner

- [ ] Point `validate_frontmatter.rs` at the owning artifact for each record
- [ ] Report a `review:` or `analyze:` block still present in `spec.md` as a violation

- **Done when**: a spec carrying a residual block is reported, and a correctly migrated spec validates clean.

## 8. Follow the records through the remaining readers

- [ ] Update `read_spec.rs`, `dashboard.rs`, and `check_artifacts.rs`
- [ ] Confirm `/{project}:status` renders the same review and analyze state as before

- **Done when**: the status view and dashboard output match their pre-relocation values for the migrated corpus.

## 9. Add the `relocate-audit-records` primitive

- [ ] Read a spec's frontmatter, write each record to its owning artifact, remove both blocks from `spec.md`
- [ ] Invent no record for a spec that carries none
- [ ] Make a spec with no blocks left a no-op
- [ ] Leave body prose untouched, and halt if a run finds itself changing it

- **Done when**: running the primitive twice over a spec leaves the second run writing nothing, and a partially migrated corpus converges without duplicating or dropping a record.

## 10. Register the migration

- [ ] Add one `[[migrations]]` entry to `framework/migrations.toml` with `introduced_in` set to the release that first carries the primitive
- [ ] Write `framework/migrations/audit-record-relocate.md`, invoking the primitive per spec and skipping pinned files with one line naming each
- [ ] State the §spec-lifecycle case (c) rule: the sweep is a mechanical edit, so a `done` spec stays `done`
- [ ] Detect an adopter CI file still carrying the block-presence check and tell them to re-copy the template

- **Done when**: 027's audit family passes against the entry, and a `done` spec swept by the procedure is still `done`.

## 11. Replace the CI template's vacuous predicate

- [ ] Read the record from `review.md` and `analysis.md`
- [ ] Bound the exempt set with a committed high-water mark, in the form `scripts/audit/analyze-record-backlog.sh` already uses
- [ ] Remove the block-presence skip

- **Done when**: no predicate in the step is satisfiable by every spec in a migrated corpus, and the step fails when the exempt set grows.

## 12. Retire the reconciliation check, reconcile the baseline

- [ ] Delete `check_review_agreement.rs`, its schema types, and `scripts/audit/review-block-agreement.sh`
- [ ] Point `scripts/audit/analyze-record-backlog.sh` at the relocated record and reconcile `analyze-record-baseline.txt`
- [ ] Confirm the never-reviewed and never-analyzed cases remain distinguishable through task 6

- **Done when**: `/{project}:audit` passes with Family 31 removed and Family 37 counting the same population it counted before.

## 13. Ship the template and sweep this repo's corpus

- [ ] Remove both blocks from `framework/templates/spec/spec.md`
- [ ] Run the primitive across all spec directories under the spec root
- [ ] Run the full markdown lint over the result

- **Done when**: a newly created spec opens with `status` and `dependencies` alone, no `spec.md` in the corpus carries either block, and the lint is clean.

## 14. Sweep the documentation

- [ ] Update `framework/commands/review.md`, `analyze.md`, `implement.md`, `audit.md`, and `consolidate.md` — runtime steps **and** markdown-only references
- [ ] Update `docs/analyze.md` field-by-field, including the `analysis.md` skeleton
- [ ] Update `docs/slash-commands.md`, `docs/shared-constitution.md`, `README.md` (the `/analyze` entry states the old location outright), and `AGENTS.md`
- [ ] Regenerate the per-agent command mirrors

- **Done when**: the command-parity audit passes and every mirror matches its source.

## 15. Verify the sweep mechanically

- [ ] Search the repository for the pre-relocation shape — a `review:` or `analyze:` block addressed as `spec.md` frontmatter
- [ ] Confirm the only surviving hits are this spec's own Motivation and Resolved Questions

- **Done when**: the search returns no hit outside this spec, so the sweep is verified rather than asserted.

## 16. Discharge the declared cross-spec obligations

- [ ] Record in `047-analyze-findings-durability` that its resolved question is superseded on the record-location half, linking back here
- [ ] Record in `020-code-review` the change to its CI-gate mechanism, linking back here

- **Done when**: both specs link back to 057, clearing the `cross-spec-impact` entries that would otherwise block `done`.
