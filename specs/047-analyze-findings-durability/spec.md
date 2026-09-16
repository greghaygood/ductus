---
status: in-progress
dependencies: []
next-criterion: 16
---

# 047 — Analyze Findings Durability

`/{project}:analyze` persists the findings it produces, so an audit's results
survive the session that ran it.

> **Superseded in part by [057 — Analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md).** Two halves of this spec
> moved, and only one of them is a reversal. **The record's location changed:**
> the analyze record is no longer a block in `spec.md` frontmatter — it is the
> frontmatter of `analysis.md`, and the review record is `review.md`'s. Every
> criterion below that names "the spec's `analyze:` block" still holds as a
> requirement and now reads against that file; each is annotated in place.
> **And the Resolved Question that rejected an `analysis.md` artifact was
> reversed**, on grounds that only became available after 047 shipped — see its
> annotation under §Resolved Questions. What 047 delivered is unchanged: the
> inbox capture, the record's fields, the second gate, and the digest-based
> freshness check are all still live. 057 moved where the record is written and
> gave the findings a report of their own.

## Motivation

`/{project}:review` had a durable sink: it wrote `review.md` and stamped the
spec's `review:` frontmatter, so its findings stayed readable long after the
session ended and `check-review-gate` could act on them later.
`/{project}:analyze` had none. It was read-only by design, and its findings
existed only in the invoking session's output.

That asymmetry made a full audit the only way to recover what a previous audit
had already found. On 2026-08-03 a comprehensive sweep of this repo produced 21
`criterion-path-existence` findings; they were acted on in-session and never
written down, so nothing in git recorded that they existed. The user asked the
question that exposed it: *if I lost the session, that information would be
hidden until I run another comprehensive audit.*

The constitution already required capture — [§brownfield-inbox Automatic issue
capture](../../framework/constitution.md#automatic-issue-capture) said issues an
agent surfaces MUST be captured, not dropped, and named `/{project}:implement`
and `/{project}:review` as the surfacing gates. Two things were missing.
`/{project}:analyze` was not among those gates, and the section addressed an
agent noticing something *incidental to other work* — not a command whose
**primary output is findings**. A command that exists to produce findings and
then discards them was the failure the **Design
Principles** rule names directly: it relied on the operator remembering.

## Behavior

`/{project}:analyze` appends each surviving finding to `{specs-root}/inbox.md`
before it renders its report, using the same mechanical append `/{project}:log`
performs.

**What is captured.** Every finding the run produces that is still live when
the run ends — hard-fail, blocking, and advisory alike. A finding `--fix`
resolved in the same run is not captured: it no longer exists to record.
Informational entries (the unexamined-target `skipped` list, cross-service
reference unknowns) are **not** findings and are not captured; they report what
could not be examined rather than a defect to route.

**One bullet per finding**, in the auto-capture form §brownfield-inbox already
documents. It carries the family so `/{project}:groom` can route it, the
message so a reader can act without re-running the audit, and the citing
artifact's path so the finding can be located. The *missing subject* the
finding is about lives inside the message, not in `{path}` — see the dedup
discussion below, where that distinction is load-bearing:

```text
- [ ] {category}: {family} — {message} — {path} (captured during /{project}:analyze)
```

**Idempotent across runs.** Analyze is a read-only audit that operators re-run
freely, and a capture step that appended on every run would make the inbox grow
without bound while saying nothing new. Each append is guarded by a dedup
prefix built from the finding's identity — its category, its family, and its
message — so a finding already in the inbox is not appended twice. Re-running
analyze against an unchanged repo therefore leaves `inbox.md` byte-identical,
which preserves the command's read-only-by-default character in every respect
that matters: it still mutates no spec, no status, and no artifact the pipeline
reads.

The message is **part of** the key, not excluded from it. `ArtifactFinding`
carries `family`, `severity`, `message`, and `path` — and `path` is the *citing
artifact*, not the missing subject the finding is about. Keying on family plus
`path` therefore merges every finding a single `spec.md` produces: measured
against this repo, that collapses 21 findings into 8 keys, silently discarding
spec 012's `specs/errors.md` and `specs/events.md` because `specs/system.md`
was captured first, and one of spec 008's two rule files for the same reason.
The subject that distinguishes them appears only inside `message`, so the
message is what makes the key faithful. The cost is that re-wording a check's
message re-appends its findings once; losing distinct findings is the worse
failure, and this trade is measured rather than assumed.

**Capture precedes rendering.** The append happens before the report is
rendered, so a run interrupted between the two still leaves the findings
recorded. Rendering is the surfacing half of the contract, not the recording
half.

**Removal is the fixer's job, not analyze's.** Analyze never prunes inbox
items. A captured finding leaves the inbox the way every other item does —
`/{project}:groom` routes it, or the work that resolves it removes it. Analyze
re-adding a resolved finding is not a regression: if the finding fires again,
it is again true.

## Constitution amendment

[§brownfield-inbox Automatic issue capture](../../framework/constitution.md#automatic-issue-capture)
gains `/{project}:analyze` in its **Surface at completion** list, and its
opening scope widens from issues an agent notices incidentally to include the
findings a command produces as its primary output. The existing four bullets
are unchanged in substance.

## Acceptance Criteria

- [x] AC1: `framework/commands/analyze.md` carries a capture step that appends every surviving finding to the inbox via `append-inbox`, ordered before the report-render step, and the markdown-only reference documents the same behavior for the runtime-less path.
- [x] AC2: Each captured bullet names the finding's category, family, message, and citing artifact path, and marks the capture source as `/{project}:analyze`.
- [x] AC3: Re-running `/{project}:analyze` against an unchanged repo appends nothing — every bullet is dedup-guarded on category, family, and message — so `inbox.md` is byte-identical across consecutive runs.
- [x] AC4: Two findings of the same family citing the same `spec.md` but naming different missing paths are captured as two bullets, not merged into one.
- [x] AC5: A finding resolved by `--fix` within the same run is not captured.
- [x] AC6: Informational entries — the `skipped` unexamined-target list and cross-service reference unknowns — are not captured.
- [x] AC7: The constitution's **Surface at completion** bullet names `/{project}:analyze` alongside `/{project}:implement` and `/{project}:review`, and the section's scope covers a command whose primary output is findings.
- [x] AC8: The generated `{cli-config-dir}/commands/{project}/analyze.md` mirror matches its source, and the full markdown lint passes.
- [x] AC9: `/{project}:analyze` records every run in the spec's `analyze:` frontmatter block — `last-run`, `analyzed-against`, the three tier counts, `unexamined`, and a derived `blocking` — including on a clean run and an empty scope, so the record's absence is itself information. A spec whose frontmatter does not parse receives no record. **Relocated by 057**: the record is the frontmatter of `analysis.md`, written on every run. Every clause survives the move — the fields, the write-on-every-run rule, and the refusal to record into an unparseable spec — and one sharpens: *the record's absence is itself information* now means the **file's** absence, which is the never-analyzed state [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) declares, rather than a missing key inside a file that exists for other reasons.
- [x] AC10: `/{project}:implement`'s pre-done gate blocks a spec whose `analyze:` block is absent, carries a null `last-run`, reports `blocking: true`, or carries a record gone stale (AC13), ordered after every `review:` check and with no grandfather clause. Advisory findings and unexamined targets are reported in the gate's guidance and never block. **Relocated by 057**: the gate reads the record from `analysis.md` and the review checks it is ordered behind from `review.md`. The block conditions, the ordering and the absence of a grandfather clause are unchanged; 057 added a third state the gate must not collapse — an artifact that exists but carries no parseable record is **undeterminable**, never never-analyzed.
- [x] AC11: The exempt population — `done` specs predating the record — is counted by `/{project}:audit` Family 37 against a committed high-water mark, which fails when the backlog grows (the gate has no grandfather clause, so growth means it was bypassed) and reports a baseline that has gone slack. Backfilling the record was rejected: it would assert a run that nothing on disk substantiates.
- [x] AC12: `/{project}:review` renders exactly one `analyze` row in every run's stdout summary — never-analyzed, superseded, current, or freshness-undeterminable — from the same subject-set digest comparison the gate uses, so a review that has just written `review.md` reports superseded rather than current. The row is advisory: the command's exit code and the review record's `blocking` — in `review.md`'s frontmatter since 057, in the spec's `review:` block when this was written — are unchanged by it. It renders on an empty scope, on a dimension-restricted run, and once per feature under `--all`.
- [x] AC13: `check-review-gate` blocks a spec whose analyze record is stale, ordered after `not-analyzed` and `analyze-findings`, with the blocked message naming the changed paths and directing the operator to re-run `/{project}:analyze`. A record carrying no `analyzed-digest` is reported as undeterminable rather than passed silently or diffed by sha.
- [x] AC14: Both surfaces derive staleness from one subject set — every `.md` under `specs/{feature}/`, `review.md` included — excluding the spec's own `analyze:` frontmatter block and excluding a uniform §spec-lifecycle rename. One implementation and **one reference point** back both: the recorded digest of what the run read, compared to the same subject set now. The notice and the gate therefore return the same answer wherever they are asked, rather than sharing code across two reference points that could differ. **Relocated by 057**: the excluded record is `analysis.md`'s own frontmatter, and `spec.md` is digested **whole** — the block-surgery path for `spec.md` is gone rather than re-pointed. The reason the exclusion exists is unchanged and is what carried it across: a record written after its subjects are read can never digest itself.
- [x] AC15: `/{project}:analyze` records `analyzed-digest` — a per-path digest of the subject set as the run read it from disk, excluding the spec's own `analyze:` block — and `analyzed-against` is recorded as provenance only, never as the staleness basis. A subject that could not be read is recorded as unreadable rather than digested as empty. **Relocated by 057**, as AC14: the excision is `analysis.md`'s own record.

## Open Questions

*None — captured during specification.*

## Resolved Questions

- **Should analyze write a per-spec `analysis.md` artifact instead, mirroring `review.md`?** **Resolved: no — capture to the inbox.** The symmetry is tempting, but the two commands differ in a way that decides it: `review.md` exists because review is a **gate**, and its artifact carries state the pipeline reads back (`review.blocking`, consulted by `/{project}:implement`, `/{project}:analyze`, and CI). Analyze findings drove no gate and were read by no command when this was decided, so an artifact would be a second inbox that `/{project}:groom` does not walk — a parallel triage surface for items that already have one. **That premise was overtaken by this spec's own later criteria** (AC9-AC10): the `analyze:` frontmatter block now records each run and `check-review-gate` blocks `done` on a missing, failing or stale one. The decision is unaffected, and the distinction is what makes it hold — the gate reads the record's *counts*, which are frontmatter; an `analysis.md` would have held the findings' *content*, which still nothing reads. The durable home for that content is the inbox, as resolved here. The inbox is the framework's designated home for routable findings, and routing is exactly what these need. A new artifact would also mean a new primitive and a new file in all 47 spec directories, against a `/{project}:groom` workflow that already handles the job.
  - **Reversed by `057-analyze-artifact-and-record-relocation`: `analysis.md` exists.** Read as a live claim, the *no* above has gone false, and the argument that produced it has not — which is the distinction worth recording rather than sweeping. Its load-bearing premise was that the record could live in `spec.md` while only the findings' content needed a home, so the artifact would buy a second triage surface and nothing else. 057 retired that premise: two gate-bearing records in one file was the two-homes condition §drift-prevention rejects, and the review half had already grown a reconciliation check (`check-review-agreement`, audit Family 31) to hold the copies together. Giving each record the frontmatter of the artifact its own command writes leaves analyze needing an artifact, so the file follows the record rather than the findings. **The inbox stays the durable home for routable findings** — AC1-AC6 are untouched, `analysis.md` is overwritten wholesale on every run, and 057's AC13 forbids a `- [ ]` item anywhere in its body **mechanically**, so the parallel-triage-surface hazard this question named is the thing that criterion exists to prevent rather than a cost the reversal accepted. The cost this question priced correctly and 057 paid anyway: a new primitive (`relocate-audit-records`), a corpus-wide migration, and a second file in every spec directory.
- **Should capture be limited to advisory findings, since blocking ones already halt the pipeline?** **Resolved: no — capture every surviving finding.** Blocking findings do get acted on, but they are equally lost when a session ends before anyone acts, and a partially-audited repo is precisely where the record matters most. The distinction also would not survive contact with `--all`, where a blocking finding on one spec sits beside advisory findings on forty-six others. Severity raises salience, not routing — the constitution's own words.
- **Does appending to the inbox break analyze's read-only contract?** **Resolved: no, and the dedup guard is what makes that true.** `/{project}:analyze` is read-only with respect to the artifacts it audits — specs, plans, tasks, scenarios, frontmatter — and this change touches none of them. It writes only the backlog file whose documented purpose is receiving exactly this. Because each append is dedup-guarded, a second run against an unchanged repo writes nothing at all, so the command stays idempotent in the sense operators rely on. `--fix` remains the only flag that mutates a spec.
