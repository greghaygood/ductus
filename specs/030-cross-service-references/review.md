---
spec: 030-cross-service-references
reviewed-at: 2026-09-13T23:59:36Z
reviewed-against: bf37241d8ae0a0f07bb4079096353cc1307978e8
diff-base: 527c052dfb356e5951939126609fc84934dcd93f
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 15
scope: 24
skipped-passes: []
---

# Review — 030-cross-service-references

## Summary

Backfill re-review of 030 under the examined/scope campaign, run against the post-reopen natural diff base `527c052d` (24 in scope, 4 modified-since). The pre-reopen natural base `40c43175` resolved 810 modified-since / 814 in scope — the widest gap in the corpus — and `--since HEAD` gives 21 / 0; the reopen collapsed the window to a denominator that covers this pass's own edits, which `HEAD` excludes by construction, so the natural base was taken.

**Examined 15 of 24 in scope**, read in full unless noted: `.github/workflows/generators.yml`, `AGENTS.md`, `framework/commands/link.md`, `framework/commands/status.md`, `framework/constitution.md`, `runtime/src/schema/services.rs`, `runtime/src/primitives/resolve_references.rs` (all non-test code; its 209 remaining lines are unit tests), `runtime/tests/fixtures/cross-service-basic` (all three fixture specs), `runtime/tests/golden/cross-service-basic.jsonl`, `runtime/tests/parity/cross-service/expected.txt`, `scripts/install-hooks.sh`, and this spec's four artifacts.

**Not examined, and why.** `README.md` and `framework/commands/analyze.md` — only their cross-service sections were read (README §Cross-service references and the `[services]` config row; analyze.md step 13 and §Cross-service references); the rest of both files is outside this feature's surface. `framework/bootstrap/configure/*.md` (4 files) — inspected only for the `/{project}:link` permission shape, not read through. `runtime/src/mcp/server.rs` — only the `resolve-references` and `derive-references` tool descriptions. `runtime/src/primitives/mod.rs` (3240 lines) and `runtime/src/schema/primitives.rs` (5490) — not read; both are shared multi-primitive files where 030 owns a small slice, and nothing in this pass turned on them. `framework/runtime-tools.txt` — header read and both cross-service entries confirmed present; the remaining tool list not read.

**Two in-scope paths no longer exist** and stay in scope because the plan lists them: `.github/workflows/markdown-only-pipeline.yml`, deleted by `2cca7d6d` when 048 replaced the opt-in invariant with the acquisition invariant, and `scripts/gen-cross-service-refs.sh`, deleted by `f4c3bbd3` when 022's `adopter-generator-promotion` made it the `derive-references` primitive. Both removals are what this pass's corrections record.

**Corrections made in the pass** (committed `7a796cbc`, `bf37241d`): AC10 annotated half-superseded — its two-paths half is intact and held by `runtime/tests/cross_service.rs`, which asserts the runtime records equal the golden and the markdown-only capture byte-for-byte, while its "`ductus` is never a prerequisite, and the no-runtime CI job exercises the fallback end-to-end" clause was reversed by 048; `data-model.md` and the scenario no longer name the retired shell generator as the live harvester, and the scenario's root-awareness gap is stated in the past tense because `derive_references.rs` implements the two-tier matcher and task 13 is ticked; `plan.md` carries a signpost for both reversed premises rather than being rewritten.

**Verified against the tree, not inferred.** AC1–AC9, AC11 and AC12 each hold: the five outcomes in `resolve_references::classify` match the spec, `data-model.md` and `analyze.md` exactly; the golden exercises all five; `references:` is absent-when-empty and never touches `dependencies:`; §spec-lifecycle case (b) carries the non-reopening carve-out; and the empty-registry path (`Services::from_toml_str` on an absent table, `load_services` on an absent file, and `status.md`'s omit-the-readout rule) gives a single-service adopter no behavior change and no new configuration. The plan's anticipated per-agent `/{project}:link` permission entries proved unnecessary rather than missing — the command dispatches no primitive and `Edit(.ductus/config.toml)` already covers its write. Every surviving occurrence of the retired generator name elsewhere in the tree (`framework/migrations/`, `framework/migrations.toml`, `quality-cross.md`'s incident narrative) is load-bearing under §drift-prevention's retired-filename rule and was deliberately left.

**0 MUST / 0 SHOULD / 0 low-confidence.** One observation is recorded and captured to the inbox: a registry-identity mismatch whose fix lands in `runtime/` and therefore belongs in the next release batch. Local gate green after committing — markdownlint, six framework lints, all three generators plus both derivations with a clean tree, `cargo test --release --locked` at 20/20 binaries and 1487 passed / 0 failed, and `scripts/audit/run-all.sh` clean; the audit was proven able to fail first by reddening Family 20 against a broken `version` file, then restored.

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

- bug: two aliases whose [services] repo values differ only by a trailing slash or a trailing .git are one service to the harvester and two to the duplicate detector, so an ambiguous registry resolves to a confidently wrong status with no signal. derive_references::normalize_repo strips both suffixes and keys the registry by the normalized URL, so the alphabetically-later alias silently overwrites the earlier one (registered-services drops 2 -> 1); Services::duplicate_repos groups by the raw string and returns no duplicate, and its duplicate_repos_detected test covers only the byte-identical case. Reproduced in a scratch fixture: a body link written against <https://github.com/acme/api> harvested under a zzz-mirror alias and resolved outcome ok / status draft from the mirror checkout, where the authoritative api checkout reads done. data-model.md:32 calls a duplicate repo a registry-validation finding and link.md:64 promises a warning at registration, so this is the one shape both miss. Fix is ~5 lines (normalize in duplicate_repos, or detect the collision in load_registry) but lands in runtime/, which carries a version bump and a ductus-v<version> tag — recorded here rather than fixed in this pass so it batches into the next release, per the campaign's standing decision (c) on runtime-destined findings. The documentation half was fixed in the pass: data-model.md now states that identity is the normalized repo. — `runtime/src/schema/services.rs:67`

## Skipped passes

*None.*

## Unexamined governance

*None.*
