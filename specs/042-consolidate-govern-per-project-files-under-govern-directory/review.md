---
spec: 042-consolidate-govern-per-project-files-under-govern-directory
diff-base: 322418501529128c0be4d4a44850659ec1637d3a
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T19:35:53Z
reviewed-against: 8fb42bfedd552a975c88d6c11e4d3acc2cb1a3a9
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 16
scope: 30
reviewed-digest:
  scenarios/provenance-tags-name-resolved-config-path.md: aef26b78a9e3eab0f62f3947d24b08214c94809f3f77c480c9fb5049fba22296
  scenarios/runtime-doc-strings-name-active-paths.md: 5799c0f7c23e431c48a83c8d2a4696beded3861629e767b5fc1fa8e67e06c527
blocking: false
---

# Review — 042-consolidate-govern-per-project-files-under-govern-directory

## Summary

Backfill review under the examined/scope + freshness-digest campaign. The previous record (2026-07-23, against 50cc0702) predated ductus-v0.49.0, so it carried no `examined`, no `scope` and no `reviewed-digest` — a record no gate could evaluate.

SCOPE AND WHAT WAS ACTUALLY READ. Three diff bases were measured before choosing: the pre-reopen natural base 2b32d415 resolved 773 scope / 767 modified-since; `--since HEAD` resolved 25 / 0; the post-reopen natural base 32241850 resolves 30 / 8. The post-reopen base was chosen — it collapses the 767-file window and, unlike HEAD, its window *contains* this pass's own eight edits rather than excluding them.

`examined: 16` of 30 scope entries. Read in full: `.ductus/config.toml`, `.gitignore`, `.shellcheckrc`, `AGENTS.md` (151 long lines, in ranges), `README.md`, `framework/bootstrap/hooks/ductus-pre-commit`, `framework/constitution.md` (761 lines), `framework/migrations.toml`, `framework/migrations/govern-dir-consolidate.md`, `framework/templates/ci/adopter-generators.yml`, `framework/templates/project/gitignore`, `runtime/src/schema/paths.rs` (628), `scripts/audit/README.md`, `scripts/audit/{fixture-session-shape,consolidation-pair,migration-coverage}.sh` (all three), and 042's own `plan.md` and `spec.md`.

The other 14 entries are named individually, because believing a file correct and having read it are different claims. FOUR were read in full EXCEPT their `#[cfg(test)]` modules — every line of implementation, no test code: `runtime/src/primitives/write_session.rs` (1-322 of 807), `runtime/src/primitives/resolve_references.rs` (1-176 of 465), `runtime/src/primitives/migrate_session_file.rs` (1-163 of 355), `runtime/src/host.rs` (1-204 of 344). SIX were read only in the regions carrying 042's subject: `framework/bootstrap/ductus.md` (1182 lines — read the Pre-run Migrations loop, §Project Configuration write policy, the Shared Files manifest and the session/config path prose at :945/:975), `framework/commands/{implement,clarify,amend,plan,target,specify,analyze,link,review,status,groom,prune,help}.md` (2772 lines across 13 — `target.md` read in full, all 13 read at every config/session path mention), `framework/bootstrap/configure/{claude,auggie,antigravity,opencode}.md` (`claude.md` read in full, the other three only at their path and permission entries), `runtime/src/main.rs` (the exec walker session seed, :515-555), `runtime/src/primitives/dashboard.rs` (`load_config`, `load_session_target` and the provenance-tag render), `runtime/src/primitives/discover_rule_files.rs` (`load_ductus_toml` and the disabled-rule-file filter). FOUR went unread: `runtime/src/interpreter/mod.rs` (1756 lines — grepped and confirmed it holds no config/session resolution call site, which is the only reason it is in scope at all), `runtime/tests/{parity.rs,exec_subprocess.rs,specs_root_override.rs,cross_service.rs}` (1696), `runtime/tests/fixtures/*/` (12 directories — enumerated, and their session/config files inspected to confirm the legacy-root layout the plan deliberately keeps as fallback proof, but the fixture contents were not read), and `specs/inbox.md` (read at the backfill item and the four items bearing on this spec, not end to end).

CRITERIA. All 16 walked against the tree, enumerations included. `check-artifacts` reported `clean: true` with four skipped paths; all four were walked by hand, since a skip means nothing has ever substantively checked the claim. AC1/AC3/AC4 already carried supersession annotations from 8e404b78 — but that sweep was `criterion-path-existence`, which only sees a backticked path with an interior slash, so it reached exactly those three and stopped. The annotation set was incomplete: AC12 (every `scripts/…` generator reference across five named surfaces, plus `lib/specs-root.sh` sourcing — the `root-absent` skip) and AC15's generators clause are superseded by the same 022 change and are now annotated, as are §Target layout, §Scripts, and the Resolved Question that claimed `.ductus/scripts/` *is* the structural marker of an adopter-facing script while `AGENTS.md` now records the opposite. AC2, AC5-AC11, AC13, AC14 and AC16 verified and hold: the three-tier resolvers and their exhaustive subset tests in `paths.rs`, the no-`sunset_after` registry entry, the converge-on-collision procedure, `active_path`'s write policy with the bootstrap's once-per-run config resolution at `ductus.md:473`, the `/.ductus/session.toml` anchor in both gitignores, and no stale root-path claim in README/AGENTS/CLAUDE/constitution/docs.

AC7's audit half was verified by proving the check can fail, not by watching it pass: `run-all.sh` prints nothing on a clean run, so `govern-dir-consolidate`'s `procedure_file` was pointed at a missing path, Family 10 emitted both invariants AC7 names (orphan procedure file, broken procedure reference) and `run-all.sh` exited 1; restored and re-run clean.

BOTH SCENARIOS HOLD. `runtime-doc-strings-name-active-paths`: every user-visible surface — `main.rs`'s clap docs, the MCP tool descriptions in `mcp/server.rs`, and the schema argument docs — names `.ductus/…` as canonical with the legacy path only as a fallback. `provenance-tags-name-resolved-config-path`: `discover_rule_files.rs` and `dashboard.rs` both render `{config_name}` from a single `resolve_config` probe, tested on both layouts, and the doc mirrors in `review.md`/`status.md` carry the tag *shape* rather than a hardcoded filename.

FIVE PASSES, ON THE CODE. The path-resolution core is sound. `newest_existing` / `active_path` differ only in the empty-chain fallback and say why; `TIERS` as a fixed-size array makes an empty chain unrepresentable rather than silently resolving to the repo root; `resolve_config` exists specifically so the read and its provenance tag cannot straddle a concurrent migration (`BE-RACE-001`); and the `subsets()` helper tests all seven non-empty tier combinations for read and write. One hypothesis was checked and disproved rather than filed: `render_toml`'s `.expect("infallibly")` in `migrate_session_file.rs` looked reachable via `ValueAfterTable` when a table-valued key sorts before a scalar one, but a throwaway probe against this crate's `toml = "1"` shows the serializer reorders scalars before tables, so the claim holds. Separately, `migrate-session-file` writing the hardcoded newest tier rather than the active file is correct and documented in the spec's Resolved Questions; the registry's `introduced_in` ordering makes the apparent hazard (an adopter on `.govern/` having it overwritten) unreachable, since such an adopter's `last_applied` already exceeds 0.10.0.

DEFECTS FOUND AND FIXED IN THIS PASS, all committed before this review was recorded (eaf4365f, 8fb42bfe). Four dead references to the generators 022 retired, measured across the whole live-artifact set at 26 occurrences in 12 files of which 22 are load-bearing: `framework/templates/project/gitignore` (SHIPPED — copied into every adopter repo on their next `/ductus` run), `AGENTS.md`'s `mirror_source_mode` rationale (which named the retired generators as the manifest's executable; the rule is live and its subject is now the two shipped hooks), `scripts/audit/README.md` (claimed Family 22 runs `.ductus/scripts/**`; it runs the shipped hook and nothing else), and `.shellcheckrc`. Plus one superseded rationale in `runtime/src/schema/paths.rs`: `validate_specs_root` justified its charset by the shipped bash generators interpolating the name unescaped into regexes — both halves false since 022, and the adopter hook now matches the spec root by shape deliberately. The constraint is kept with the reasons that outlived that one. Doc comments only; no version bump, because a rustdoc comment reaches no adopter and neither clap help nor the MCP descriptions were touched.

NOT FIXED HERE, CAPTURED INSTEAD. One measured cross-spec pattern is already on `specs/inbox.md` and is not repeated as an observation: the `.govern/` middle tier spec 049 added to the resolution ladder was never propagated to the sites that describe it — 59 prose/doc lines across 21 files carry a legacy filename, ~30 across 13 files stale either as a two-tier ladder (including SHIPPED `framework/commands/review.md:213-214`, whose sibling `status.md:65` gets it right) or as a legacy tier named as *the* file where the code calls the ladder. Two members are functional rather than prose: Family 12 (`fixture-session-shape.sh:48`) cannot see a fixture at `.govern/session.toml` while claiming to verify "every fixture session file", and Family 17 (`host-namespace-parity.sh:70`) resolves two tiers while its comment says it mirrors `Host::load`'s three. It was logged as one item rather than swept because it spans files outside 042's scope entirely (`schema/services.rs`, `schema/mod.rs`, `schema/primitives.rs`, `primitives/mod.rs`, `merge_managed_block.rs`, `cross_service.rs`), and correcting only 042's share would leave the corpus more inconsistent than it was found; the load-bearing occurrences are enumerated on the item so the next pass does not sweep the migration records, the legacy-layout fixtures, or the past-incident narrative. The one observation recorded below is the shipped CI template's trigger filter.

0 MUST, 0 SHOULD, 0 low-confidence outstanding: everything above is either fixed at source in this pass or captured where the pipeline will surface it again. Gate run after every commit — markdownlint, six lints, two shell test scripts, shellcheck, the three generators and both derivations with no drift, `run-all.sh` (proven able to fail), and the full cargo suite at 20 binaries / 0 failed.

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

- convention: the shipped adopter CI template `framework/templates/ci/adopter-generators.yml` hardcodes `paths: ['specs/**', '.githooks/**']` in its `on.pull_request` trigger, while every other part of the same file carefully resolves the configured `[paths] specs-root` (spec 040). An adopter who renamed their spec root gets a workflow that never triggers on a spec change, so both gates it carries — the generator-sync diff check and the review-blocking gate on done specs — silently never run, and the PR shows no check rather than a failing one. This is the defect the file's own comment at :56-60 records having already fixed once in the *enumeration* half ("It used to hardcode `find specs`, which on a renamed root enumerated nothing, left the failure counter at zero, and exited 0"); the trigger half was not carried across, and it is the AGENTS.md §Workflow case about a test whose inputs fall outside the paths that trigger its workflow. GitHub Actions `paths:` filters are static YAML and cannot interpolate a config value, so the repair is a design call rather than a substitution — broaden the glob, drop the filter so it always runs on PRs, or state the constraint in the template header so an adopter who renames knows to edit it. Measured during /ductus:review of 042, 2026-09-13; not fixed in that pass because the gap belongs to spec 040 and the fix is not mechanical. — `framework/templates/ci/adopter-generators.yml:27`

## Skipped passes

*None.*

## Unexamined governance

*None.*
