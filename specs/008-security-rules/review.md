---
spec: 008-security-rules
reviewed-at: 2026-09-14T00:37:41Z
reviewed-against: b3f322adb078fd208860ff00ea4ec1044b99452f
diff-base: e6f07be86659fb9782f585dcc28e3d9e359e63e2
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 7
scope: 20
skipped-passes: []
---

# Review — 008-security-rules

## Summary

Backfill re-review of 008 under the examined/scope campaign. The prior record (2026-07-21, `ba807cc5`) carried no `examined` and no `reviewed-digest`, so a `0/0/0` on it was byte-identical to a review whose five passes never fired. This run reads its scope and states what it did not.

**Scope and base.** Measured both legs after the step-4 commit, per the campaign's step (5). Pre-reopen the natural base `043a0345` resolved 83 modified-since / 88 in scope; the reopen collapsed it to **20 / 20** on base `e6f07be8` (the reopen commit's parent) against a plan affecting 6. `--since HEAD` gave **0 / 6** and was declined: it excludes by construction the twenty files this pass edited, which are the whole subject of the re-review. Both legs returned inline — no saved-output step, so the base was chosen on the merits.

**Examined 7 of 20, read in full:** `framework/constitution.md` (761 lines), `framework/rules/security-backend.md` (765 lines / all 73 rules), `framework/rules/security-frontend.md` (343 lines / all 32 rules), and all four of 008's own artifacts under review (`spec.md`, `plan.md`, `data-model.md`, `research.md`).

**The thirteen not counted, and why.** Two are mirrors and are named rather than folded into the numerator, per `AGENTS.md` §Workflow: `.claude/commands/ductus/analyze.md` is generated from `framework/commands/analyze.md`, which was read, and `scripts/gen-claude-commands.sh` re-ran in this pass reporting all 16 commands in sync; `framework/bootstrap/govern.md` is held byte-identical to `framework/bootstrap/ductus.md` by audit Family 21 and was re-mirrored with `cp` and proven with `cmp` in this pass. Believing those two correct and having read them are different claims, and only the second is `examined`.

Three were read in part, and the parts are named. `framework/bootstrap/ductus.md` — about 120 of 1182 lines: §Security Audit (brownfield) in full, the §Shared Files manifest tables, §Placeholder Substitution, and the stale-branch and spec-root input steps. `framework/commands/analyze.md` — about 130 of 391 lines: frontmatter, Purpose, Context, Scope Boundaries, all eighteen numbered Instructions steps, §Rules (blocking and advisory) in full, and the full H3 index of the markdown-only reference. `framework/templates/spec/spec.md` — the §Applicable Rules block only.

Eight are the non-008 rule files — `accessibility-frontend`, `api-backend`, `concurrency-backend`, `configuration-cross`, `observability-backend`, `performance-backend`, `performance-frontend`, `reliability-backend`. Every line this pass changed in each was read, in the file and again in the diff, and `api-backend.md`'s diff was read in full as the representative case; their Rationale prose was not. They are in scope because this pass edited them, not because 008 owns them.

**Why eight files outside 008's plan are in scope at all.** The `validate` → `/{project}:analyze` residue traces to 008's `data-model.md`, which the constitution's canonical-sources map designates THE source for security rule file format and Verification phrasing. It prescribed "instruction to the validate agent", and all eleven rule files copied that into 187 Verification fields. Spec 023 renamed the command and swept four token forms; the bare prose name was in none of them, so 202 current-usage references survived across 13 live files, 11 of which ship to adopters. Fixing the canonical source and leaving the files it seeded would have been the same partial sweep that produced the defect. None of the eight is a spec directory, so no spec reopened and no other review went stale — `write_review.rs` digests only `scenarios/*.md` and `data-model.md`.

**Passes.** Security: the subject is 008's own rule content, and all 105 rules were verified structurally — every one carries Statement, Rationale and Verification, IDs are unique, zero-padded from `001` per category with no gaps, and the BE/FE category sets match `data-model.md`'s table and Resolved Question 2's enumeration exactly. No rule's substance was weakened; the sweep touched only the sentence naming the enforcing command. Reuse: the ten rule-file headers were aligned to wording `quality-cross.md` (spec 036, the newest rule file) already carried, rather than minting an eleventh phrasing. Quality: the restored no-rule-files advisory in `analyze.md` closes a §design-principles violation — `loading rule files:` with an empty list rendered identically whether the directory held nothing or could not be read. Efficiency and Simplicity: N/A for a documentation pass; no indirection added, and every correction replaced a claim rather than annotating around one.

**Eleven defects, all fixed in this pass, none captured.** The inbox stands at 10, unchanged. Each was verified against the tree before being fixed: AC16's advisory was traced to `11aad341` (022 task 14) deleting it with no decision to drop it, so the source was fixed rather than the criterion; `BE-API-002` was checked against the file's first commit (`35502af0`) and has never been the TLS rule the spec cited; the `FE-XSS-002` mislabel was checked against `FE-XSS-001` and corrected in both 008 and the shipped `framework/templates/spec/spec.md`; the `specs/security-{backend,frontend}.md` path against `framework/migrations/rule-files-relocate.md`; "Backend rules apply to all projects" against spec 033's `[rules] surfaces` filter; the `NNN-*` grammar against §numbering and spec 051; the "verbatim" category claim against all 15 shipped headings; and plan.md's GraphQL and supply-chain deferrals against `BE-INPUT-014` and `BE-DEPS-002`/`-004`/`-005`.

**Tech-stack alignment.** This project's own code surface is markdown, bash, YAML and Rust — no backend service and no frontend application — so the backend and frontend rule files are *shipped content* reviewed as content, not enforceable constraints on the framework's own source. `discover-rule-files` reports all eleven; `configuration-cross.md` and `quality-cross.md` are the two that bind this repo's own artifacts, and both were considered: no new constant, env var or operator-tunable literal is introduced, and `QUAL-CLAIM-001` is what the restored advisory serves rather than violates.

**Gate.** Full local surface, run after committing: markdownlint (509 files, 0 issues), the six framework lints, `scripts/tests/*.sh`, shellcheck, the three generators plus `derive-dependencies`/`derive-references` with a clean tree after, `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked` at 1487 tests across 20 binaries. `scripts/audit/run-all.sh` exits 0 — proven meaningful by first reddening Family 20 with a bad `version` value and restoring it.

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
