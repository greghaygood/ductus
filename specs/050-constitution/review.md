---
spec: 050-constitution
reviewed-at: 2026-09-13T17:58:00Z
reviewed-against: 4703223d8ca930c77118645c61ca7e7730799f97
diff-base: ad086a0d9358c5f50c7640fc9c1cdbedb462c665
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 6
skipped-passes: []
---

# Review — 050-constitution

## Summary

Five passes over 6 of 6 in-scope files — nothing unread. **Re-recorded.** The previous record (`4ef49b83`, examined 8/8) carried a Summary asserting that the `AGENTS.md` mirror's duplicated back-edge clause had been removed. The edit was in the working tree and never staged, so at that sha the clause was still present: the claim was true of the tree and false of the commit it was recorded against. `4703223d` lands the trim and this record is written against it. Kept as a re-record rather than an amendment because a review record is evidence about a commit, and a Summary describing uncommitted work is the same failure one layer up from the one this backfill campaign exists to clear — AGENTS.md §Workflow's *edit, review, commit together records the HEAD from before the edits landed*.

**Scope.** Base `ad086a0d`, the parent of the reopen commit: 6 files — `AGENTS.md` and `specs/050-constitution/spec.md` modified since, plus 050's four plan-affected artifacts. The scenario file left scope on this pass because it was unchanged since the base and is not plan-affected; it was read in full and reviewed in the prior record, whose `reviewed-digest` still covers it. Every in-scope path exists; none is absent.

**What this pass added to 050.** `a-canonical-source-is-pointed-at-not-copied`, stating under §drift-prevention's *Canonical sources* that referencing means a pointer and that a reproduction is never one. The existing MUST — "reference the canonical source rather than restate it" — is satisfied on its face by a verbatim copy, which is the gap: 020's §Embedded artifacts runs lines 242–802 of an 802-line spec against a live source now at 894 lines, and **nothing could report it**, because the snapshot sits in a code fence that no link check, anchor resolver or audit family reads.

**Criteria re-verified against the tree, enumerations included.** All nineteen hold. Four were checked because this change could have falsified them. **AC3** (a mirror states nothing normative of its own) is the one that failed on the first attempt and is now satisfied: grepping the rule's phrasing finds one normative statement in the constitution and one pointer in `AGENTS.md`. **AC5** (adopter-neutral wording): the new text cites the constitution, git history and generic tooling categories — "link check, anchor resolver, structural audit" — and no path that exists only in this repository, which is what the reword test requires. **AC10** (anchors undisplaced): the rule is a paragraph inside an existing subsection, so no `<!-- §anchor -->` moved; `resolve-anchor` returns 28 references on the spec and 8 on the scenario, `unresolved: []` on both. **AC12** (the pin does not move): `version`, `runtime/Cargo.toml` and `runtime/CHANGELOG.md` untouched, Family 20 clean — the constitution reaches adopters by the Shared Files manifest row, not by a tag.

**050's own §Trade-offs read as live claims.** All five hold, and this change conforms to two rather than contradicting them: "promoted rules lose their war stories" — the constitution carries the adopter-neutral rule while the `AGENTS.md` mirror keeps the 020 incident; "bullets are less citable than sections" — the rule landed inside §drift-prevention rather than as a new section, per the clarify walk's resolution.

**Rule set.** 11 files loaded. Every one verifies against code, or against a spec or plan introducing shared state, an outbound call, a config value, a metric, API surface or UI. This change introduces none — it is governance prose in a document — so no Verification trigger fires. The passes ran and were empty rather than skipped, and the reuse pass is the one with a real surface here: it is what caught the duplicated clause both times.

**The observation from the prior record stands and is not re-captured.** The adopter-project-name inbox item enumerates 000, 001, 002, 017, 022 and 036 — exactly the `anvil` spread, correct for that name — while a second name, `svc-zmc-api`, was never measured and lives in 045 and 046. It was appended to the inbox by the prior `write-review` call; re-appending here would duplicate it, which the idempotence guard exists to prevent.

**Verification at this HEAD.** The constitution reaches two parity goldens (`implement-basic`, `target-basic`), so the full runtime suite was run rather than assumed: 20 binaries invoked, 20 reported, 0 failures, no re-bless needed. clippy `-D warnings` and `cargo fmt --check` clean; six `lint-*.sh` green; the 37-family self-audit green, and previously proven to fail on a perturbed subject before its silent zero-byte pass was trusted; markdownlint over 513 files. `derive-dependencies` reports no drift — the scenario's link to 020 induces no edge, confirmed by running it rather than assuming scenarios are exempt.

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
