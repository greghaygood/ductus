# 057 — Analyze artifact and record relocation Plan

Implements [057 — Analyze artifact and record relocation](spec.md).

## Overview

Eight movements, in a forced order. The constitution declares the schema first
because [§runtime-boundary](../../framework/constitution.md#runtime-boundary)
principle 4 says schema changes ship through the constitution and the runtime
updates to match. Then the record types change owner, the writers move, the
readers follow, a new primitive sweeps the corpus under a registry migration,
the reconciliation check is deleted, and the documentation sweep closes it.

The work is mostly *relocation*, and the plan's discipline is keeping it that
way: two places invite a behavior change that would ride along unnoticed, and
both are decided against below.

## Technical Decisions

### The constitution moves first, and it declares more than it relocates

`framework/constitution.md`'s spec-file frontmatter table (lines 520-527) lists
`status`, `dependencies`, `references`, `next-criterion`, and
`cross-spec-impact`. `review:` and `analyze:` are **not there** — they have been
riding the Open-schema rule at line 533, "additional fields … permitted and
ignored by uninterested consumers." Two gate-bearing records, undeclared.

So AC21 is not a relocation within the document; it is a first declaration. The
Frontmatter Schema section gains a **record schema** subsection naming the
owning artifact for each record and the fields it carries, and the spec-file
table stays at the five fields it already had. Line 562 fixes the order: "Schema
changes ship through the constitution; the runtime MUST update to match. The
constitution MUST NOT import runtime types."

### `ReviewBlock` and `AnalyzeBlock` keep their shape and change their owner

`runtime/src/schema/primitives.rs:33` (`ReviewBlock`) and `:109`
(`AnalyzeBlock`) are already standalone types. The change is where they are
deserialized from: `Frontmatter` (`:697`) loses its `review` and `analyze`
fields, and each record is parsed from its own artifact's frontmatter.

`check_review_gate.rs` reads `frontmatter.review` at line 134 and
`frontmatter.analyze` at line 183, both from a single `spec.md` deserialize at
line 85. It gains one read per artifact. That is the whole reader change in the
gate; the branch logic downstream of those two bindings is untouched, which is
what makes AC4's differential check a real check rather than a restatement.

### One timestamp name: `last-run` on both records

The two records spell the same instant differently today —
`check_review_agreement.rs:50` names it: "The timestamp is the one pair spelled
differently on each side (`last-run` / `reviewed-at`), so pairs are keyed by
meaning, not by name." That primitive exists to reconcile the pair; with the
pair gone, the naming should not survive it.

Both records use `last-run`. Keeping `review.md`'s existing `reviewed-at` was
considered — it would leave adopters' existing files untouched in that one field
— and rejected: the migration rewrites both files wholesale regardless, so the
rename costs nothing extra, and two names for one concept is exactly what
§drift-prevention rejects.

### Two digest subject sets: one exclusion moves, one exclusion's *rationale* lapses

These are the two places a behavior change could ride along, and they are
decided differently.

**The analyze exclusion moves, and simplifies.** `AnalyzeBlock::analyzed_digest`
(`primitives.rs:129-132`) digests `spec.md` "with its own `analyze:` block
excised, because this record is written after the subjects are read and a digest
covering it could never match." With the record in `analysis.md`, the excision
moves there and `spec.md` is digested **whole** — the block-surgery path for
`spec.md` disappears rather than being re-pointed. This is AC7.

**The review exclusion stays, and its rationale is annotated as lapsed.**
`ReviewBlock::reviewed_digest` (`primitives.rs:52-55`) excludes `review.md` and
`spec.md` because "`write-review` touches both, so counting them would stale
every review the instant it was recorded." After AC3, `write-review` no longer
touches `spec.md` — so the stated reason lapses for that half. **The set is not
expanded in this change.** Adding `spec.md` to the review digest would make
every review stale on any spec-body edit: a genuine behavior change wearing a
relocation's clothes, and one AC4 forbids. The lapsed rationale is recorded in
the code comment and left to a follow-up that can argue it on its own merits.

### The corpus sweep is a primitive, invoked per spec

A new primitive — `relocate-audit-records` — reads a spec's frontmatter, writes
each record to its owning artifact, and removes both blocks from `spec.md`.
Idempotent by construction: a spec with no blocks left is a no-op, which is AC20.

**It merges; it does not refuse.** The first implementation treated an existing
artifact as a conflict and declined to write. A probe against real spec 047
showed why that is wrong: *every* pre-migration spec already has a `review.md`,
so refusal was the normal case and the primitive could not migrate the review
record at all. It now merges the block into the artifact's frontmatter, carrying
that artifact's existing report body **verbatim** — replacing a real review
report with a placeholder would destroy the findings the record is being moved
beside.

**A disagreement is named, never smoothed.** When both copies carry a key with
different values the spec block wins, because it is the copy every gate actually
read; the key is reported in the result's `disagreements` list as `file:key`.
This is the 031/041 shape directly, and a migration that resolved it quietly
would erase the only evidence the drift ever happened.

`framework/migrations/criterion-label-backfill.md` step 2 is the precedent and
the argument: its sweep is performed by the primitive "never by hand — 700
criteria across 49 specs in this repository's own backfill, where a hand edit is
a silent renumbering waiting to happen." The arithmetic here is 54 specs × two
records. §runtime-boundary principle 3 closes the other option: a shell script
parsing frontmatter has already failed.

### The migration must not reopen a `done` spec

Moving a frontmatter block changes no criterion text and no body prose, so it is
case (c) of §spec-lifecycle's mechanical-edit rule and the `done → draft`
back-edge does not fire. `criterion-label-backfill.md` step 3 states the same
rule for the same reason, down to its stopping condition: a run that finds
itself changing body prose is not this migration and must halt.

### The CI step borrows Family 37's baseline, because that predicate cannot go vacuous

`framework/templates/ci/adopter-generators.yml:111` skips a spec when
`grep -q '^review:'` fails — a predicate this relocation makes true of every
spec. `scripts/audit/analyze-record-backlog.sh:38-46` already solves the shape:
the backlog is held against a committed high-water mark
(`scripts/audit/analyze-record-baseline.txt`), "at or below it: clean … above
it: a finding, because the backlog **cannot legitimately grow**." The CI step
adopts that form, so emptying the subject cannot satisfy it.

### `check-review-agreement` is deleted, not re-pointed

Its subject is the intersection of two records of one fact. With one record
there is no intersection, and a check that compares a thing to itself reports
agreement forever — the false green its own header warns about. The primitive,
its schema types, and audit Family 31 go. The `single_sided` distinction it drew
(a block with no report, a report with no block) survives as AC5's
absent-versus-unreadable reporting on the gate.

### No dual-read window

Between a new runtime landing and the migration running, a gate would read the
new home, find nothing, and report never-reviewed. That window is bounded by a
single `/ductus` run: bootstrap's Pre-flight Phase acquires the pinned runtime
*before* Pre-run Migrations, as `criterion-label-backfill.md` records for its
own case. A dual-read deprecation window was rejected — it re-creates the
two-homes condition this spec exists to remove, and AC6 requires a residual
`spec.md` block to be a *violation*, which a dual-read would have to tolerate.

### Why the runtime work is sequenced additively

`Frontmatter` is a shared type, so removing its `review` and `analyze` fields
breaks every reader in one compile. The first task breakdown put that removal
early and asked each following task to pass `cargo test` — unreachable, because
the tree does not build again until the last of them lands.

Two call sites made it worse than a red tree. `process_waivers.rs:65` and
`invalidate_review.rs:94` parse their own minimal frontmatter structs rather
than `Frontmatter`, so they would have kept compiling, found no `review:` block,
and behaved as though every spec had zero waivers — a waiver losing its
structural existence, which is the precise shape of the 031 failure this spec is
built on.

So tasks 2-9 are ordered additively: the merged type and the loaders first
(additive), then the writers, then the readers, and the `Frontmatter` field
removal last. Each step compiles and tests. Tasks 3 and 4 deliberately leave the
`spec.md` block write in place until task 9 retires it — a refactoring sequence
internal to this change, not the shipped dual-read the Trade-offs section
rejects. The distinction is that nothing ships between task 3 and task 9.

**One correction to the data model from this sequencing.** `waivers` is listed
there as part of the merged review record, which is right as a schema claim, but
it is not a field on `ReviewBlock` and must not become one: three raw parsers
own it with `#[serde(flatten)]` to preserve an organization's custom fields, and
`schema/primitives.rs` carries no `serde_norway::Value` for the `JsonSchema`
derive to accept. Waivers change file in task 5 and stay open-schema.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Edit | Declare both record schemas and their owning artifacts (AC21); ships before the runtime change |
| `runtime/src/schema/primitives.rs` | Edit | Drop `review`/`analyze` from `Frontmatter`; rename the timestamp to `last-run`; add the merged review record fields (AC12) |
| `runtime/src/primitives/write_review.rs` | Edit | Write the record to `review.md` frontmatter only (AC3) |
| `runtime/src/primitives/write_analysis.rs` | Edit | Write `analysis.md` — record plus fixed skeleton — on every run (AC1, AC14, AC19) |
| `runtime/src/primitives/check_review_gate.rs` | Edit | Read each record from its owning artifact; absent vs unreadable (AC4, AC5) |
| `runtime/src/primitives/validate_frontmatter.rs` | Edit | Validate each record at its owner; residual `spec.md` block is a violation (AC6) |
| `runtime/src/primitives/relocate_audit_records.rs` | Create | The idempotent per-spec sweep (AC10, AC20) |
| `runtime/src/primitives/check_review_agreement.rs` | Delete | No intersection left to compare (AC15) |
| `runtime/src/primitives/check_artifacts.rs`, `dashboard.rs`, `read_spec.rs` | Edit | Follow the records to their new homes (AC8) |
| `framework/migrations.toml` | Edit | One entry, `procedure_file` pointing at the new procedure (AC16) |
| `framework/migrations/audit-record-relocate.md` | Create | The per-entry procedure, including the adopter CI-file detection (AC16, AC18) |
| `runtime/src/schema/registry.rs`, `framework/runtime-tools.txt`, `main.rs`, `mcp/server.rs`, `interpreter/mod.rs` | Edit | Primitive registration — three separate parity tests enforce all of them |
| `runtime/Cargo.toml`, `Cargo.lock`, `runtime/CHANGELOG.md` | Edit | 0.49.8 → 0.50.0; `introduced_in` must name the release that first carries the primitive |
| `framework/templates/ci/adopter-generators.yml` | Edit | Replace the vacuous grandfather with a baseline bound (AC17) |
| `framework/templates/spec/spec.md` | Edit | Template ships without the two blocks (AC2, AC25) |
| `scripts/audit/review-block-agreement.sh` | Delete | Family 31's shell entry point goes with its primitive (AC15) |
| `scripts/audit/analyze-record-backlog.sh`, `analyze-record-baseline.txt` | Edit | Count the exempt population from the relocated record (AC9) |
| `framework/commands/{review,analyze,implement,audit,consolidate}.md` | Edit | Both runtime steps and markdown-only references (AC23) |
| `docs/analyze.md`, `docs/slash-commands.md`, `docs/shared-constitution.md`, `README.md`, `AGENTS.md` | Edit | The documentation sweep (AC22, AC25) |
| `specs/*/spec.md`, `specs/*/review.md`, `specs/*/analysis.md` | Edit | The 54-spec corpus sweep, performed by the primitive (AC10, AC11) |

## Implementation notes

Recorded during tasks 1-12 so a session resuming this work does not rediscover
them.

**The write boundary is spec-dir only.** `derive-boundary` returns
`specs/057-analyze-artifact-and-record-relocation/**` and nothing else, because
the commit that created the spec directory changed nothing outside it. Every
remaining task writes outside that boundary, so `/{project}:implement` will
report an out-of-boundary edit and wait. The grant needed is the **Affected
Files** table above — `framework/`, `runtime/`, `scripts/audit/`, `docs/`,
`README.md`, `AGENTS.md`, the per-agent command mirrors, and the spec corpus.

**The MCP server runs the binary from `runtime/target/release/`.** Runtime
changes are invisible to the ductus tools in the current session until
`cargo build --release` **and** a session restart. This is harmless while the
corpus still carries the blocks; it matters before task 15's sweep, which must
run against the newly built binary.

**Three registration surfaces are parity-tested.** A new primitive needs an
entry in the clap `Command` enum + dispatch arm, `PRIMITIVE_REGISTRY`, and
`framework/runtime-tools.txt`. Each has its own failing test, so missing one is
loud rather than silent.

**CI denies clippy warnings** (`.github/workflows/runtime.yml:180`,
`clippy --release --all-targets -- -D warnings`), and `cargo fmt` will collapse
backslash line-continuations inside string literals into runs of spaces — check
any multi-line user-facing message after formatting.

**Tasks 18, 19 and 23 need a restarted session, and this is now proven rather
than predicted.** The MCP server still exposes `check-review-agreement`, which
task 14 deleted — so the server is running a binary older than this spec's
runtime work, and `write-review` / `write-analysis` called through it would
write **pre-relocation** records: the spec-block form, into `spec.md`. That is
the stale-*write* case `AGENTS.md` §Gotchas names as the worst one, because it
persists on disk and a later gate trusts it. Every remaining discharge task
re-runs those two commands, so all three wait on a restart. `cargo build
--release` is already done; only the session is stale.

**Four `cross-spec-impact:` entries, not two, and the prices differ.** The plan
declared 047 and 020. Implementation added two more, each discovered by a check
rather than by reading: **026** (task 14 deleted Family 31, which 026's AC22
and a scenario specify) and **022** (task 17's sweep found 10 lines across 6
durable contracts). Price them by destination before spending them: 047 and 026
and 022 all have hits in `scenarios/*.md` or `data-model.md`, so each costs a
reopen **plus a full five-pass re-review**; 020's hits are in `spec.md`,
`plan.md` and `data-model.md`, so it costs the re-review too. 022 is the
largest single unit in the corpus and that pass has never been spent.

**Audit Family 20 has been red on `main` since `783602c5`.** Tasks 11-12 bumped
`runtime/Cargo.toml` and `runtime/CHANGELOG.md` to 0.50.0 and left the repo-root
`version` file at 0.49.8. It is red in the *safe* direction — `/ductus` reads
the root file and 0.49.8 is published, so no adopter is halted — but the audit
is a hard release gate. Task 21 reconciles it **with** the tag, in one sitting:
a bumped pin with no tag is an outage, not a deferred release.

**One issue was captured to the inbox rather than fixed here:**
`validate-frontmatter` emits `severity: "blocking"` for every finding including
ones the constitution classes as Hard fail. Out of this spec's scope; it routes
through `/{project}:groom`.

## Resuming from here

Tasks 1-16, 20 and 22 are complete and committed; task 17's search has run and
its confirmation has not. What remains is task 17's second half plus four
discharge tasks, and `tasks.md` carries each one's steps. This section is the
part `tasks.md` cannot hold: the order, the mechanics, and the two things a
fresh session would otherwise measure again.

**Do the discharges before the release, and 057's own gates last.** Tasks 18,
19 and 23 each reopen a sibling spec; task 17's confirmation can only pass once
all three have landed, because the surviving hits *are* those specs; 057's own
`/{project}:review` and `/{project}:analyze` come after that, since both read a
corpus those tasks are still editing; and task 21 — the version pin and the tag
— is last of all, because a release is cut from completed work.

**The reopen mechanic is `set-status`, not `/{project}:amend`.** Each discharge
edits artifacts to reflect a decision already made here, with no new input to
classify, which is the case §spec-lifecycle sends to the status primitive
directly. `set-status --feature <slug> --from done --to in-progress`, make the
edits, re-run both audit commands, then `--from in-progress --to done`. The
guard is on both ends deliberately: a transition computed from a stale read
fails loudly instead of overwriting what is actually on disk.

**Every one of the four costs a full re-review, and that is not obvious from
the diff.** Each spec's hits land in `scenarios/*.md` or `data-model.md` — the
only two files `write_review.rs` digests — so the reopen stales
`reviewed-digest` and `check-review-gate` answers `review-stale` rather than
`analyze-stale`. Do not price these as prose fixes. 022 is the largest single
unit in the corpus and that pass has never been spent; `AGENTS.md` §Workflow
carries the standing judgement about it.

**Reopening 020 does not redden `mechanical_sweep_parity`.** `AGENTS.md`
asserted it did; that claim was disproved and corrected on 2026-09-16, with the
three derivations recorded in the entry itself. Do not route around it.

**Task 17's confirmation is a classifier, not a grep, and the allow-list is the
substance of it.** The pre-relocation shape is a record block addressed as
`spec.md` frontmatter — match either spelling of the block, or a dotted
accessor (`review.last-run`, `analyze.blocking`, `review.waivers`), within ~140
characters of `spec.md` / "spec frontmatter" / "the spec", in both orders.
Four families of hit are **correct and must survive**, so the confirmation is
that nothing outside them remains: this spec's own artifacts and
`runtime/CHANGELOG.md`; the migration entry and procedure, whose subject *is*
the move; the constitution's declaration that a residual block is a violation,
and `validate-frontmatter` implementing it; and past-tense accounts — the
`analyze_subjects` doc comments, `check_review_gate`'s fixture helper, and the
CI template comment naming what the step used to skip on. Classify every new
hit by **tense**, which is the test the whole sweep used.

**057 carries null records of its own.** The corpus sweep relocated this spec's
template block, so `review.md` and `analysis.md` exist here with `last-run:
null` — never-reviewed and never-analyzed, stated rather than implied. The
completion gate reads them correctly and will demand both; nothing needs
repairing first.

## Trade-offs

**Rejected: a dual-read deprecation window.** Reading the old location when the
new one is absent would remove the bounded window described above. It would also
reinstate two homes for one fact for the length of the deprecation, and force
AC6 to tolerate exactly the residual block it is written to flag.

**Rejected: keeping `reviewed-at` in `review.md`.** It would spare one field
from the migration's rewrite, in a file the migration rewrites anyway, at the
cost of keeping the two-names-one-instant condition that `check-review-agreement`
had to special-case.

**Rejected: expanding the review digest subject set now.** The exclusion's
stated rationale lapses for `spec.md` under AC3, and following that through
would be a defensible change — but not *this* change, and not silently inside
one. AC4's differential check is what makes the difference visible.

**Rejected: a shell script or hand edit for the corpus sweep.** §runtime-boundary
principle 3, and the 700-criteria precedent that produced it.

**Limitation: adopters must re-copy the CI template by hand.** `/ductus` does not
install CI files (`framework/templates/ci/adopter-generators.yml:20`), so AC18's
detection can only *tell* an adopter their copy is stale. An adopter who ignores
it keeps a step that passes vacuously — the failure this change is removing,
surviving in the one place the framework cannot reach.

**Limitation: the never-run window inside a `/ductus` run.** Bounded, self-healing,
and documented rather than engineered around.

**Limitation: `cross-spec-impact` blocks `done` until `047` and `020` link back.**
Declared deliberately in the spec frontmatter; the obligation is real work, not
an artifact of the declaration.
