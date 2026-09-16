# 057 — Analyze artifact and record relocation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Task 1 is first by constitutional rule, not by preference: §runtime-boundary
principle 4 requires a schema change to ship through the constitution with the
runtime following. Tasks 2-8 are the runtime; 9-12 the migration and the gates
around it; 13-14 the sweeps; 15-16 verification and the declared obligations.

## 1. Declare both record schemas in the constitution

- [x] Add a record-schema subsection to `framework/constitution.md`'s **Frontmatter Schema** naming the owning artifact for each record and the fields it carries
- [x] Leave the spec-file table at its existing five fields — the records were never in it
- [x] State that `spec.md` carries neither block, so a residual one is a defect rather than an open-schema extra

- **Done when**: the constitution declares both records at their owning artifacts, and no part of the declaration is sourced from a runtime type.

## 2. Add the merged record type and the record loaders

- [x] Extend `ReviewBlock` with the report-only scalars it must now carry: `spec`, `diff-base`, `captured-issues`, `skipped-passes`
- [x] Leave `waivers` untyped and open-schema — three raw parsers own it with `#[serde(flatten)]` extras, and `schema/primitives.rs` carries no `serde_norway::Value` for the `JsonSchema` derive to choke on
- [x] Add loaders that read each record from its owning artifact, returning **absent**, **unreadable**, and **present** as three distinct states
- [x] Leave `Frontmatter`'s `review` and `analyze` fields in place — this task is additive and nothing reads the loaders yet

- **Done when**: `cargo test` passes and the loaders return each of the three states against a fixture that exercises all three.

## 3. `write-review` writes the whole record into `review.md`

- [x] Render the merged record into `review.md` frontmatter: rename `reviewed-at` to `last-run`, and add `blocking`, `reviewed-digest`, `reviewed-unreadable`, and `waivers`
- [x] Leave the `spec.md` block write in place for now — retired in task 9, so the tree stays green in between

- **Done when**: `cargo test` passes and a review run produces a `review.md` whose frontmatter deserializes into the merged record with no field dropped.

## 4. `write-analysis` writes `analysis.md`

- [x] Render `analysis.md`: merged record frontmatter plus the fixed skeleton — Summary, hard failures, blocking findings, advisory findings, unexamined targets
- [x] Write it on every run, including a clean run and an empty scope
- [x] Emit no `- [ ]` item in any section
- [x] Leave the `spec.md` block write in place for now

- **Done when**: `cargo test` passes, and a clean run and an empty-scope run each produce an `analysis.md` containing zero checkbox items.

## 5. Move the waiver parsers to `review.md`

- [x] Repoint the raw waiver parsers in `process_waivers.rs`, `write_review.rs`, and `invalidate_review.rs` at `review.md`
- [x] Preserve the `#[serde(flatten)]` extras, so an organization's custom waiver fields survive the move

- **Done when**: `cargo test` passes and a waiver carrying org-custom fields round-trips through `review.md` byte-identical.

## 6. Migrate the gate and the artifact checks to the loaders

- [x] Replace the `frontmatter.review` / `frontmatter.analyze` reads in `check_review_gate.rs` (lines 134, 183) and `check_artifacts.rs` (583, 626) with the task-2 loaders
- [x] Report an absent artifact as never-run and an unparseable one as undeterminable — never collapse the second into the first
- [x] Leave every downstream branch untouched

- **Done when**: a differential run of the gate over the pre- and post-relocation corpus produces identical verdicts for every spec, and the absent and unreadable cases are distinguishable in the output.

## 7. Move the digest exclusions

- [x] Excise `analysis.md`'s own record from the analyze digest; digest `spec.md` whole and delete the `analyze:`-block surgery path
- [x] Leave the review digest subject set unchanged, and annotate in the code that the `spec.md` exclusion's stated rationale lapses once `write-review` stops touching it

- **Done when**: an analyze run immediately following itself reports current rather than stale, and the review digest's subject set is unchanged from before the relocation.

## 8. Validate each record at its owner

- [x] Point `validate_frontmatter.rs` at the owning artifact for each record
- [x] Report a `review:` or `analyze:` block still present in `spec.md` at the **Blocking** tier the constitution now declares for it
- [x] Report a `review.md` or `analysis.md` that exists but carries no parseable record as a hard fail

- **Done when**: a spec carrying a residual block is reported as Blocking, and a correctly migrated spec validates clean.

## 9. Retire the `spec.md` writes and drop the `Frontmatter` fields

- [x] Remove the `spec.md` block writes from `write_review.rs`, `write_analysis.rs`, and `invalidate_review.rs`
- [x] Remove `review` and `analyze` from `Frontmatter`, and fix the fixture at `primitives.rs:4421`
- [x] Point `check_review_agreement.rs` at its own empty-subject guidance path rather than letting it compare a record to itself — it is deleted in task 14, and a self-comparison in the interim is the false green its header warns about

- **Done when**: `cargo test` passes and no code path reads or writes either record in `spec.md` frontmatter.

## 10. Follow the records through the remaining readers

- [x] Update `read_spec.rs` and `dashboard.rs`
- [x] Confirm `/{project}:status` renders the same review and analyze state as before

- **Done when**: the status view and dashboard output match their pre-relocation values for the migrated corpus.

## 11. Add the `relocate-audit-records` primitive

- [ ] Read a spec's frontmatter, write each record to its owning artifact, remove both blocks from `spec.md`
- [ ] Invent no record for a spec that carries none
- [ ] Make a spec with no blocks left a no-op
- [ ] Leave body prose untouched, and halt if a run finds itself changing it

- **Done when**: running the primitive twice over a spec leaves the second run writing nothing, and a partially migrated corpus converges without duplicating or dropping a record.

## 12. Register the migration

- [ ] Add one `[[migrations]]` entry to `framework/migrations.toml` with `introduced_in` set to the release that first carries the primitive
- [ ] Write `framework/migrations/audit-record-relocate.md`, invoking the primitive per spec and skipping pinned files with one line naming each
- [ ] State the §spec-lifecycle case (c) rule: the sweep is a mechanical edit, so a `done` spec stays `done`
- [ ] Detect an adopter CI file still carrying the block-presence check and tell them to re-copy the template

- **Done when**: 027's audit family passes against the entry, and a `done` spec swept by the procedure is still `done`.

## 13. Replace the CI template's vacuous predicate

- [ ] Read the record from `review.md` and `analysis.md`
- [ ] Bound the exempt set with a committed high-water mark, in the form `scripts/audit/analyze-record-backlog.sh` already uses
- [ ] Remove the block-presence skip

- **Done when**: no predicate in the step is satisfiable by every spec in a migrated corpus, and the step fails when the exempt set grows.

## 14. Retire the reconciliation check, reconcile the baseline

- [ ] Delete `check_review_agreement.rs`, its schema types, and `scripts/audit/review-block-agreement.sh`
- [ ] Point `scripts/audit/analyze-record-backlog.sh` at the relocated record and reconcile `analyze-record-baseline.txt`
- [ ] Confirm the never-reviewed and never-analyzed cases remain distinguishable through task 6

- **Done when**: `/{project}:audit` passes with Family 31 removed and Family 37 counting the same population it counted before.

## 15. Ship the template and sweep this repo's corpus

- [ ] Remove both blocks from `framework/templates/spec/spec.md`
- [ ] Run the primitive across all spec directories under the spec root
- [ ] Run the full markdown lint over the result

- **Done when**: a newly created spec opens with `status` and `dependencies` alone, no `spec.md` in the corpus carries either block, and the lint is clean.

## 16. Sweep the documentation

- [ ] Update `framework/commands/review.md`, `analyze.md`, `implement.md`, `audit.md`, and `consolidate.md` — runtime steps **and** markdown-only references
- [ ] Update `docs/analyze.md` field-by-field, including the `analysis.md` skeleton
- [ ] Update `docs/slash-commands.md`, `docs/shared-constitution.md`, `README.md` (the `/analyze` entry states the old location outright), and `AGENTS.md`
- [ ] Regenerate the per-agent command mirrors

- **Done when**: the command-parity audit passes and every mirror matches its source.

## 17. Verify the sweep mechanically

- [ ] Search the repository for the pre-relocation shape — a `review:` or `analyze:` block addressed as `spec.md` frontmatter
- [ ] Confirm the only surviving hits are this spec's own Motivation and Resolved Questions

- **Done when**: the search returns no hit outside this spec, so the sweep is verified rather than asserted.

## 18. Discharge the declared cross-spec obligations

- [ ] Record in `047-analyze-findings-durability` that its resolved question is superseded on the record-location half, linking back here
- [ ] Record in `020-code-review` the change to its CI-gate mechanism, linking back here

- **Done when**: both specs link back to 057, clearing the `cross-spec-impact` entries that would otherwise block `done`.
