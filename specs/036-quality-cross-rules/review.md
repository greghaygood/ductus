---
spec: 036-quality-cross-rules
reviewed-at: 2026-09-13T18:52:58Z
reviewed-against: 8384deb0b245e53e974f7578b6f13c4efa39603e
diff-base: 71770bb400891a307303c70e5e7226f8877f4ad1
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 7
skipped-passes: []
---

# Review — 036-quality-cross-rules

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes at this HEAD. Not blocking. Three defects were found during the run and all three were fixed at source before this record was written (`f2f426f1`, `8384deb0`), so the zero counts describe the reviewed HEAD rather than a clean first pass.

**Examined: 6 of 7 in-scope files read in full.** Read completely: `framework/constitution.md` (761 lines, in four ranges), `framework/rules/quality-cross.md` (45), `scripts/lint-rule-ids.sh` (99, reviewed as code), `specs/036-quality-cross-rules/spec.md` (73), `plan.md` (63), `data-model.md` (58). **Not read in full: `framework/bootstrap/ductus.md` (1182 lines / 141KB)** — §Shared Files (lines 745–813) was read completely, because that is the section 036 owns a change to, and the whole file was swept by grep for rule-file rows and for count words. The remaining ~1100 lines of bootstrap procedure — pre-flight, runtime detection, migrations, project configuration, file fetching, frontmatter migration, per-agent scaffolding, hook installation, placeholder substitution, edge cases — were **not** examined. 036's surface in that file is one manifest row and one note, both verified; the rest is unrelated installer procedure and is unread. `examined` is 6 rather than 7 for that reason.

**Scope, measured on three bases before choosing.** Pre-reopen natural base: 744 files (from the operator brief; not re-measurable once the base moved). `--since HEAD`: base `f2f426f1`, `modified-since` empty by construction, scope 4. Post-reopen natural base — **chosen** — base `71770bb4`, `modified-since` 4, scope 7. The post-reopen base is both tighter than 744 and more honest than `HEAD`, because it *covers* the edits this pass made rather than excluding them.

**Defect 1 — the `QUAL` surface was never registered in the constitution's canonical-sources table.** §drift-prevention carries a row for the `BE-`/`FE-` registration (008) and for `CFG-` (017), and instructs that a new kind of fact referenced from multiple documents name its canonical source there. `QUAL` is referenced from `framework/rules/quality-cross.md`, from `scripts/lint-rule-ids.sh`'s source-of-truth comment, and twice from the constitution itself — §design-principles and §grounding both cite `QUAL-CLAIM-001` by name and send the reader to §rules. 036's plan had considered the constitution and concluded "no constitution edit", reasoning correctly about §rules (which does not enumerate rule-ID surfaces as a closed list) and never reaching the canonical-sources table. 021 and 045 each added their own row this way; 036 is the one that missed it. Row added; the plan's conclusion corrected with it.

**Defect 2 — a false claim in the spec body.** §Added categories said the four `CLAIM` instances included "one of which was an adopter report". None did. Confirmed twice against history: at `890b7f6e` (2026-08-02), when that line was written, the rule's Source named four instances all observed in `ductus`'s own tooling on 2026-08-01/02, and the review recorded that same day describes them as one confirmed instance, one confirmed sibling and two unassessed — no adopter among them. The adopter case is a later *fifth* instance that reached the Source on 2026-08-27 (`b05e1df5`). The line now points at the Source rather than restating it, which also keeps the instance record in one place.

**Defect 3 — `lint-rule-ids.sh` could not distinguish examining everything from examining nothing.** The script globs `framework/rules/*.md` under `shopt -s nullglob`, so a missing or relocated rules directory yields an empty array; on bash >= 4.4 — which `ubuntu-latest` runs, and this lint is a `framework-checks.yml` step — expanding an empty array under `set -u` is not an error, so the loop would not run and the script would exit 0 having read no rule file. On bash 3.2 it instead dies with `files[@]: unbound variable`, loud but naming a bash internal condition rather than the missing subject. Verified here on 3.2; the >= 4.4 half is the documented behaviour change and was **not** executed locally, since only 3.2 is installed on this machine — stated as such rather than asserted. This is `QUAL-CLAIM-001` — the rule 036 itself introduced — in a script 036 modified, and constitution §design-principles states the same thing as a MUST for a check that cannot run. Fixed: an empty file set now exits 1 naming the subject, and every run reports `examined N rule file(s), M rule ID heading(s)` on stderr (the Family 26 shape, where the count is the guard). Proved in four states — clean (exit 0, 11 files / 191 IDs), empty subject (exit 1 with a diagnostic), malformed + duplicate IDs (both still caught), and `--help` (the `sed 2,19p` range untouched).

**Criteria verified against the tree, enumerations included.** All nine hold. Every path named in a criterion resolves, including the two — `performance-frontend.md` and `security-backend.md` in AC7 — that `criterion-path-existence` can never see, since neither carries an interior slash; `check-artifacts` reported `clean: true` with an empty `skipped`, which is a fact about backticks and slashes, not about claims. AC3's lint was proved red on a malformed ID before its exit 0 was trusted. AC4's header declares all three categories (`STUB`, `GROUND`, `CLAIM`), matching `analyze.md`'s own statement of them. AC5's three-part discriminator and four exemptions match the Resolved Question clause for clause. AC8's two claims were confirmed at source: `/{project}:review` has a quality pass (step 5), and `analyze.md` itself frames its grounding check's truth half as "`/{project}:review`'s job against code", which is the counterpart relationship AC8 asserts. AC9's promotion criterion matches `analyze.md`'s advisory path exactly (5+ instances on two consecutive runs). The data-model's grammar claim is exact — the harvester requires a category of >= 2 characters (`check_rule_ids.rs:94`) while the lint allows one, and the narrowest shipped category is 3, so the divergence is latent as stated.

**AC7's parenthetical was examined and judged still true.** It says the row is "slotted between `performance-frontend.md` and `security-backend.md`"; `reliability-backend.md` (spec 039, later) now sits between them. The row is still positionally between the two named, the table is alphabetical, and the criterion asserts placement rather than adjacency — so it is not falsified and takes no edit.

**Decisions re-read as live claims.** All four Resolved Questions and all five plan trade-offs still hold, and their premises with them: `--waive` and `[[review.disabled-rule-files]]` both still exist (the MUST-vs-SHOULD resolution rests on them); `-cross.md` is still kept unconditionally by the surface filter (`ductus.md:751`) and still one of exactly three suffixes; the "ship `STUB` alone" resolution explicitly anticipated the later `GROUND` and `CLAIM` promotions, so the spec is coherent with what happened rather than stale. The one decision that had gone stale was the plan's manifest-count instruction (bump six -> seven), which implementation deliberately reversed by removing the count; task 3 records that and the plan now says so, so no later reader restores a number that was dropped on purpose.

**Mirrors checked.** `README.md` states the three `quality-cross.md` failure modes and the three suffixes correctly, and its rule-file table carries 11 rows matching 11 files on disk. `analyze.md:306` names the three categories exactly. The two `ductus.md` / `govern.md` hits on a rule-file count sit inside a past-incident narrative where the number was correct at the time — off-limits to a sweep by the tense test, and left alone.

**Pass-by-pass.** Security: the only code in scope is `lint-rule-ids.sh` — fixed glob under a quoted `$ROOT`, every expansion quoted, no network, no `eval`, arguments other than `-h/--help` rejected with exit 2; the one unquoted expansion (`$id_re` inside `[[ =~ ]]`) is required to be unquoted there. Nothing found. Reuse: the data-model cites 008's schema rather than restating it, the rule file cites `BE-SCHEMA-002` and §grounding rather than restating them, and the constitution row added here is a pointer, not a copy. Quality: defect 3 above. Efficiency: 11 greps and an O(n^2) `seen` scan over ~30 IDs per file — negligible, nothing found. Simplicity: nothing overengineered; the guard is two lines and a comment.

**Checked and deliberately not filed.** Two things were examined and judged not findings rather than missed. (1) `QUAL-GROUND-001` names "a config key" as an external contract while `CFG-ENV-003` already mandates fail-fast startup validation for required env vars — but `QUAL-GROUND-001`'s own exemption list excludes "a contract the project itself owns and defines in-repo", which is exactly an application's own env var, so the two are disjoint by construction and AC6 owes no citation. (2) The lint tolerates a heading carrying a trailing title (`### QUAL-STUB-001 — Silent stubs`) although its header states the schema reserves `###` for the bare ID; all 191 headings in the corpus are bare, the runtime harvester tolerates the same shape so the two agree, and the truncation is deliberate per the code's own comment — not worth acting on today, so not carried as inbox debt either.

Deterministic corroboration at this HEAD: `npx markdownlint-cli2` clean over 513 files; six lints, both script tests and `shellcheck -S warning` exit 0; generators and both derivations left the tree clean; `scripts/audit/run-all.sh` exit 0, proved red first with a Family 21 probe so its silence is evidence; `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test --release --locked` all exit 0 with 20 of 20 test binaries reporting ok — including both parity suites, so the constitution edit staled no golden.

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
