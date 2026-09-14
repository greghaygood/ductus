---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-09-14T21:19:18Z
reviewed-against: a31a7ee2cd66c54862a2a6c73e8baa9d2e165e83
diff-base: 03f9a9c9c4b1ecc65cdbad76abe9a10a4351e39e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 28
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

*Why this ran at all.* 051's review record predated `ductus-v0.49.0` — no `examined`, no `scope`, no `reviewed-digest` — so a run whose five passes never fired would have been byte-identical to this one, and its analyze record carried no `analyzed-digest`, leaving freshness undeterminable on both. This is the corpus backfill campaign's unit for 051.

*Scope, and both bases.* Pre-reopen, the natural base `eb52e24a` resolved **430** modified-since / **435** in scope at **131,994 bytes** — over the MCP output cap. That figure is cited from the campaign item rather than re-measured: the reopen moves the base to the reopen commit's parent, so the pre-reopen window is not recoverable once step 4 lands. The step-4 commit collapsed it to **6 modified-since / 28 in scope at 2,259 bytes** on base `03f9a9c9`, which returned inline. `--since HEAD` gave **0 / 22** and was declined for excluding, by construction, the six files this pass edited. One in-scope path does not resolve and stays in scope because the plan lists it: `framework/bootstrap/*`, a glob rather than a path. The seven other known-absent entries the campaign item records were in the pre-reopen window and are absent from this one.

*What was examined: 6 of 28.* Read end to end: `specs/051-branch-scoped-spec-numbering/spec.md`, its `data-model.md`, `scenarios/rewrites-preserve-line-endings.md`, and `framework/constitution.md`. Read as implementation and module documentation in full, with their `#[cfg(test)]` modules read only in part: `runtime/src/primitives/check_unfolded_specs.rs` and `runtime/src/primitives/retire_feature.rs`.

*What was not examined, named individually.* `runtime/src/primitives/rewrite_spec_links.rs`, `mod.rs`, `resolve_feature.rs`, `validate_frontmatter.rs` and `create_feature.rs` were each read only in the regions bearing on this spec's claims — the feature-form parse and comparator, identifier matching, the `folds-into` shape check, and identifier sanitization — and their claims were settled by probe against a scratch mixed corpus rather than by reading forward from the source. `runtime/tests/mixed_corpus.rs` was read for its four corpus-enumeration tests only. `framework/commands/fold.md` and `specify.md` were read at the steps carrying AC13, AC18, AC24 and AC28, not in full. Not opened at all: `framework/commands/analyze.md`, `framework/runtime-tools.txt`, `framework/templates/spec/spec.md`, `runtime/src/interpreter/payload.rs`, `runtime/src/mcp/server.rs`, `runtime/src/parser/mod.rs`, `runtime/src/primitives/dashboard.rs`, `runtime/src/schema/extensions.rs`, `runtime/src/schema/primitives.rs`, and `scripts/gen-help-tables.sh` — the generator was run rather than read, and reported in sync. The three shell surfaces this pass edited — `.githooks/pre-commit`, `framework/bootstrap/hooks/ductus-pre-commit` and `scripts/lint-frontmatter.sh` — were read only at the spec-matching alternation and its comment, so none is counted. Two scenarios outside this scope, `fold-target-checked-before-the-rewrite.md` and `the-numbering-grammar-reaches-every-surface.md`, were read in full for the criteria walk and are named here because they are not in the denominator.

*The empty `skipped` array, which is this spec's trap.* `check-artifacts` reported `clean: true` with `skipped: []`, and **0 of the 37** backticked spans in the criteria are visible to `criterion-path-existence` — worse than 041's 0 of 22. All 37 were walked by hand. The reason none is visible turns out to be sharper than 041's: not one of the 37 is a path claim at all. They are identifiers, grammars, statuses, frontmatter keys and command names — `1234.1-{slug}`, `max + 1`, `^[a-z0-9]+(?:-[a-z0-9]+)*$`, `folds-into`, `/ductus:review` — each rejected by `is_path_like` on braces, whitespace, a colon, a leading slash, the `NNN` token, or a missing interior slash. So there is nothing here for the family to check even in principle, and no correction could bring one under it. The verification burden is entirely on claim substance, which is where the four defects were.

*Four defects, all corrected in `a31a7ee2`.* **AC5 and AC15** were falsified by the same root cause, recorded as an observation below rather than fixed, because the fix is a `runtime/` change routing through 022. **AC16's** second clause — resolving `1234` `never a sequential spec` — was falsified by this spec's own commit `599f0ef8`, which made three digits a *minimum* so the formatter and the predicate would agree past 999 and made `1234-slug` legal in the same change; resolving `1234` against a root holding both now returns `ambiguous`, verified by probe, which is what the spec's own edge-case list already prescribes. **`data-model.md`** still carried `NNN = three ASCII digits` as the canonical grammar, the rule `599f0ef8` replaced — the constitution was amended in that commit and the spec's own canonical record was not. **A Resolved Question** argued `folds-into` needs no validation carve-out while AC32, task 53 and `validate_folds_into` all require one: an option this spec rejected and then adopted inside itself. Also corrected: §Interaction described the pre-051 predicate in the present tense and named `feature_number`, removed by this spec, and the line-endings scenario claimed every CRLF checkout is damaged *today* after the shared `line_ending_of` sweep shipped.

*A check that was cited and never written.* AC15 named `runtime/tests/spec_path_shape.rs` as what holds the shell alternations in agreement with `parse_feature_dir`. It does not exist and `git log --all` shows it never did, yet `.githooks/pre-commit`, `framework/bootstrap/hooks/ductus-pre-commit` and `scripts/lint-frontmatter.sh` each cited it — three live artifacts asserting a check a reader would rely on instead of looking. What actually holds the **shipped** hook is audit Family 22, which drives it against fixtures named `1000-thousandth` and `1234.1-staged`: behavioural evidence, and stronger than the regex comparison the citation promised. The dogfooded hook and the frontmatter lint are held by nothing, and all three comments now say which case they are in.

*Verified rather than assumed.* Twenty-four of the thirty-four criteria were settled by probe in a scratch repo at `/tmp` rather than by reading forward from the source: identifier sanitization (`PROJ-1111` to `proj-1111.1-`, `1111-PROJ` to `1111-proj.1-`, `a.b` to `a-b.1-`, case-folding to one namespace), the per-identifier counter, both creation refusals and their inverse guard, sequential numbering unaffected by branch-scoped directories, `folds-into` surviving both frontmatter generators without becoming a dependency, an absent fold target producing no finding, the rename re-pointing all eight `folds-into` fields, `retire-feature` refusing an absent target with the directory left in place, the dashboard's pending-fold row, and the pre-`done` gate blocking with `blocked-by: pending-fold`. The three scenarios were each checked against the tree: the fold-target check is in `rewrite_spec_links` ahead of the corpus-wide rewrite as its scenario requires, and the line-endings sweep shipped whole — a shared `line_ending_of`, plus `runtime/tests/line_ending_discipline.rs`, which enumerates the code rather than a list and records in its own header the per-function gap it cannot close.

*Also checked.* `resolve-anchor` reports `unresolved: []` across all eight artifacts, so 051 owes no share of the 62 unresolved references outside `spec.md`. Every spec numbered after this one was checked for a reversal of something here: 052's two post-completion notes, on the spec body and the `retire-feature` row, both still hold — 054 removed only 052's supersession half, and `allow-sequential` still exists with consolidation as its only caller. The local gate was run after committing, not before: `cargo test` exit 0 with 20 `test result:` lines against 20 binaries, `cargo fmt`, `cargo clippy -D warnings`, six framework lints, `shellcheck`, all three generators reporting in sync, `markdownlint` clean over 509 files, and `scripts/audit/run-all.sh` exit 0 — the last proven trustworthy by first reddening Family 20 with a wrong `version` value and watching it exit 1.

*Findings.* None. Zero MUST, zero SHOULD, zero low-confidence across the five passes over the six files read. One observation is recorded below and appended to the inbox by this same write, so the report and the backlog cannot disagree.

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

- bug: the two frontmatter generators each carry their own sibling-link grammar at exactly three digits, so a body link to a branch-scoped or four-digit spec yields no dependencies: edge. `derive_dependencies::leading_slug` tests `bytes[..3].iter().all(is_ascii_digit) && bytes[3] == b'-'`, and `derive_references` (`:280-289`) carries the same test as an independent copy. `parse_feature_dir` — the single membership rule spec 051 established — accepts both directory forms and three-or-more digits, so the two disagree with it in the same two ways. Proven by probe 2026-09-14 in a scratch repo at /tmp: a spec whose body linked `../1234.1-staged/spec.md` and `../1000-thousandth/spec.md` derived `dependencies: []` while links in the other direction harvested normally; `derive-dependencies` reported `examined: 3`, so the enumeration is right and only the target grammar is wrong. Effects: a dependency on a branch-scoped spec is silently absent, `traverse-deps` inherits the gap through the frontmatter it reads, and `scan_line` is `pub(crate)` for the pre-`done` cross-spec-impact back-link matcher, so a declared impact on such a spec could never be discharged. Not reachable in this corpus, which holds no branch-scoped and no four-digit directory. This falsifies 051's AC5 (`none of them treats a branch-scoped directory as a non-spec`) and AC15 (`no consumer that can call it carries a copy`), both of which were annotated in `a31a7ee2` rather than left standing. Cost, which is why it is recorded and not fixed here: it is a change to a shared parser under `runtime/src/`, so per AGENTS.md §Workflow it routes to `022-deterministic-runtime` as a scenario — reopening the corpus's largest spec and obliging a full five-pass re-review — and carries a version bump plus a `ductus-v<version>` tag in the same sitting. Route it with the other 022-blocked items rather than alone; decision (c)'s lesson is that a batch is one release and singletons are never. Smallest fix: have both call the shared grammar instead of re-deriving it, which is also what AC15 asks for. (captured during /ductus:review of 051-branch-scoped-spec-numbering) — `runtime/src/primitives/derive_dependencies.rs:189`

## Skipped passes

*None.*

## Unexamined governance

*None.*
