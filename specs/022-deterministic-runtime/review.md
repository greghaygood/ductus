---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-14T01:42:53Z
reviewed-against: 0e4be7590037cb4cd5e5cad4f59814ecf0bcd80e
diff-base: a2050318dd2ad77149c6bb284fed535a6800f731
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 12
scope: 87
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

**This is a digest refresh after a mechanical canonical-record sync, not a full re-review of 022, and the record should be read as exactly that.** `ductus-v0.49.4` added `examined` to `CheckRuleIdsResult`, and `data-model.md` is the canonical registry of primitive result shapes, so the `check-rule-ids` example gained the field. That is a §spec-lifecycle mechanical sync — 022 stays `done` — but it changed a durable contract, so `reviewed-digest` no longer matched and `/{project}:audit` Family 19 blocked the release gate. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. No waivers.

**What was re-read, and what was not.** Examined **12 of 87**. The scope is 63 modified-since on base `a2050318`, which spans everything committed since 022's last reopen — three unrelated backfill passes and this release. Counted only files read end to end: `framework/constitution.md` (761 lines), `AGENTS.md` (157), `README.md` (314), `framework/commands/analyze.md` (399), `runtime/src/schema/services.rs` (250), and the eight 031 and 047 artifacts this session's earlier passes read in full.

Named rather than folded into the numerator:

- `specs/022-deterministic-runtime/data-model.md` (1349 lines) — read in the sections bearing on this change: the primitive result-shape registry around `check-rule-ids`, plus a corpus grep confirming no sibling artifact states that shape (`spec.md` and three scenarios name the primitive, all describing behavior this change does not touch). Not read end to end, so not counted.
- `runtime/src/primitives/check_rule_ids.rs` (443), `check_artifacts.rs`, `derive_references.rs` and `schema/primitives.rs` — read and reviewed **only in the regions this release changed**, each of which was proven red before its fix. The rest of those files was not re-read.
- 022's `spec.md`, `tasks.md` and its **95 scenarios** (4855 lines) — not re-read at all. Nothing in this change reaches them.
- The remaining ~70 scope files, which are the other passes' subjects rather than this one's.

**Why the split is stated this way rather than resolved by reading more.** `specs/inbox.md` records the standing judgement that a full 022 re-review is *its own unit, not a doc fix batched into a release* — 022 is the largest spec in the corpus at roughly 7500 lines across its own artifacts. Spending that here would have made a three-line release into the corpus's biggest review, and claiming it without spending it is the conflation `examined` exists to prevent. The honest third option is this one: refresh the digest, record a truthful numerator, and say which question this record does and does not answer.

**The three fixes, reviewed in full.** Each is a `QUAL-CLAIM-001` repair in the runtime's own machinery, and each regression was demonstrated failing before its fix. `check-rule-ids` now reports `examined`, so a real miss and an empty rule-file list are no longer byte-identical — the primitive that enforces rule citations previously yielded blocking findings against a correct spec. `ships_to_adopter` now normalizes its own candidate; the characterization matters and is corrected here, because `criterion-path-existence` passes an already-trimmed span and was **never** reachable by that defect, while `check-orphaned-references` passed the raw target and was the only live path. And `Services::duplicate_repos` now groups by the same `normalize_repo` the reference harvester keys on, because the defect was two notions of "the same repo" rather than a missing check. Verified through the built binary rather than the MCP tools: this repo's `check-orphaned-references` reports `findings: 0`, down from four.

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
