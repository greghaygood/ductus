# 057 — Analyze artifact and record relocation Data Model

The two audit records, at their new homes. Field shapes are carried over from
`runtime/src/schema/primitives.rs` — `ReviewBlock` at line 33, `AnalyzeBlock` at
line 109 — with the changes each table's Notes section names.

## Review record — `specs/{feature}/review.md` frontmatter

| Field | Type | Source | Notes |
| --- | --- | --- | --- |
| `spec` | string | `review.md` | Feature slug. Unchanged. |
| `last-run` | ISO-8601 UTC, nullable | merged | Was `last-run` in the block and `reviewed-at` in the report — one instant, two names. `last-run` survives. |
| `reviewed-against` | sha, nullable | both (identical) | Provenance only, never the staleness basis. |
| `diff-base` | sha, nullable | `review.md` | Report-only field; carried into the merged record. |
| `must-violations` | u32 | both (identical) | |
| `should-violations` | u32 | both (identical) | |
| `low-confidence` | u32 | both (identical) | |
| `captured-issues` | u32 | `review.md` | Report-only field. |
| `examined` | u32 | both (identical) | |
| `scope` | u32 | both (identical) | |
| `skipped-passes` | list of strings | `review.md` | Report-only field. |
| `reviewed-digest` | map path → sha256 | `spec.md` block | Subject set unchanged: `scenarios/*.md` and `data-model.md`, excluding `review.md` and `spec.md`. See the plan on why the `spec.md` exclusion's rationale lapses without the set changing. |
| `blocking` | bool | `spec.md` block | Block-only field. Derived, not authored. |
| `waivers` | list | `spec.md` block | Block-only field. The one AC12 field most easily lost in a merge, because only one side ever had it. |

**Notes.** Six fields were byte-identical across the two records and collapse to
one. Five existed on exactly one side and are carried, not dropped — that is the
whole of AC12. Absence of the file is the never-reviewed state (AC5); the file
is never written empty to signal it.

## Analyze record — `specs/{feature}/analysis.md` frontmatter

| Field | Type | Notes |
| --- | --- | --- |
| `spec` | string | Feature slug, mirroring `review.md`'s. New field. |
| `last-run` | ISO-8601 UTC, nullable | Unchanged name. |
| `analyzed-against` | sha, nullable | Provenance only. |
| `analyzed-digest` | map path → sha256 | The staleness basis. Subject set is every `.md` under the feature directory, `review.md` included — now excising **`analysis.md`'s own record** rather than `spec.md`'s `analyze:` block, so `spec.md` is digested whole. |
| `analyzed-unreadable` | list of strings | Subjects that exist but could not be read. Recorded rather than digested as empty. |
| `hard-fail` | u32 | |
| `blocking-findings` | u32 | Named for the tier, not for the derived flag below. |
| `advisory` | u32 | Recorded, never gated on. |
| `unexamined` | u32 | The field that makes a clean run honest: clean-with-nothing-skipped and clean-with-something-skipped are two states. |
| `unexamined-by-reason` | map reason → count | |
| `captured-issues` | integer | Findings this run appended to the inbox. Beside `advisory` because the pair is the point: how many findings the run produced, and how many it actually recorded. |
| `blocking` | bool | Derived. |

**Notes.** No field is added or removed by the relocation; the record is the one
`write-analysis` already produces. What changes is the file it lands in and the
excision the digest performs.

## `analysis.md` body

A fixed skeleton, rendered whole on every run (AC14), mirroring `review.md`:

```text
# Analysis — {feature}

## Summary
## Hard failures
## Blocking findings
## Advisory findings
## Unexamined targets
## Captured issues
```

**Notes.** No section carries `- [ ]` items — AC13, checked mechanically, by
stripping any checkbox marker a captured bullet arrives with rather than
trusting the caller not to send one. A checkbox is what would make this a
triage queue, and routing belongs to the inbox.

`## Captured issues` exists because it is the only section that can carry
finding *text*. The writer receives per-tier counts plus one list of captured
inbox bullets, and those bullets record `family — message — path` without
recording which tier produced them — so they cannot be split across the three
tier sections, and a body without this section would restate the frontmatter and
stop there.

## `Frontmatter` — `specs/{feature}/spec.md`

| Field | Change |
| --- | --- |
| `status` | unchanged |
| `dependencies` | unchanged |
| `tags` | unchanged |
| `folds-into` | unchanged |
| `cross-spec-impact` | unchanged |
| `review` | **removed** |
| `analyze` | **removed** |

**Notes.** `Frontmatter` at `primitives.rs:697` loses two `Option<…>` fields. The
open-schema rule means a residual block in an unmigrated tree deserializes
harmlessly — which is why AC6 makes `validate-frontmatter` report it explicitly
rather than relying on the type to reject it.
