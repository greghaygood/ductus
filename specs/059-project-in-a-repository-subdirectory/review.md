---
spec: 059-project-in-a-repository-subdirectory
last-run: 2026-09-27T15:36:19Z
reviewed-against: aa06bf6985d530ebcae788a47904f3f6c68ee670
diff-base: cea5a4137d8d0096977d6466574cc54d0c4f1850
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 33
scope: 37
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 28
  routed: 0
  discarded: 17
  undispositioned: 0
decisions:
  - key: "design: the rename exemption's evidence spans the whole repository, so another project's identical rewrite counts toward its two-file test — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: A rename sweep is repository-wide by design and a monorepo's projects share one history; narrowing the evidence to the project changes the mechanical-sweep exemption's contract, which 059 did not change.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "bug: a committed rename sweep can exempt an uncommitted edit to the same subject — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: Predates 059 and concerns how the mechanical-sweep exemption weighs uncommitted work — a contract question for the spec that owns the exemption, not a path conversion.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "convention: the corpus scope matches `.md` case-sensitively where the repository scope does not — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: discarded
    reason: Predates 059; which files each scope examines is `check-corpus-links`' own contract, and changing either direction changes what the check reports.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "perf: the `check-stuck` and scenario history walks visit every commit in the repository and do not follow renames — `runtime/src/primitives/check_stuck.rs`"
    outcome: discarded
    reason: "Cost and history reach, not a conversion defect: both walks answer correctly for the project, and bounding them by path or following renames changes what each walk is defined to read."
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "convention: tests that expect no repository assume the temp directory has no enclosing work tree — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: discarded
    reason: Predates 059 and is shared with other test modules; a ceiling would need process-wide environment in a parallel suite, and CI's temp directory has no enclosing work tree.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "design: `ProjectRepository::discover` searches upward without a ceiling, so a project inside an unrelated repository that ignores it finds no specs — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: "This is git's own resolution: the repository that contains a directory is the one git reports, and overriding it would disagree with every git command the operator runs."
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "security: error messages print the caller's value raw — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: Predates 059; the values go to stderr, which is not a log, so BE-INPUT-011's conditions do not hold.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "bug: a non-UTF-8 component in the project's path from the work tree renders lossily and matches nothing — `runtime/src/primitives/mod.rs`"
    outcome: discarded
    reason: "No such directory can be created on the platforms this is developed and tested on, so a guard could not be shown failing, and the callers already report `examined: 0`."
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "design: amend's reconcile pass offers untracked scenarios but not staged ones — `framework/commands/amend.md`"
    outcome: discarded
    reason: Predates 059 and is the reconcile pass's own contract; 059 changed only how its paths are named.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "convention: 022's `adopter-generator-promotion.md` quotes the root project's `.githooks` literal — `specs/022-deterministic-runtime/scenarios/adopter-generator-promotion.md`"
    outcome: discarded
    reason: The literal describes the case its resolved question was decided for, and the reasoning — local git config never travels with a clone — holds for any project.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "bug: the criterion-path grammar accepts `../` and scenario finding paths hard-code a lowercase `.md` — `runtime/src/primitives/check_artifacts.rs`"
    outcome: discarded
    reason: Predates 059; both are contracts of `check-artifacts`' criterion-path and scenario families, and neither is a path conversion.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "simplicity: three diff-delta walks convert and filter paths in near-identical code — `runtime/src/primitives/diff_cross_spec.rs`"
    outcome: discarded
    reason: The walks read different sides of a delta, so one helper would need a mode per caller; that is not a mechanical merge.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "design: an unparseable config gives `derive-references` an empty registry — `runtime/src/primitives/derive_references.rs`"
    outcome: discarded
    reason: "Predates 059 and is the registry contract of the spec that owns cross-service references; `registered-services: 0` already reports it."
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "perf: `first_commit_for_prefix` reads past its first match, and each listing discovers the repository up to three times per run — `runtime/src/primitives/derive_boundary.rs`"
    outcome: discarded
    reason: git2 reports an aborted walk as an error, so an early stop would have to be told apart from a real failure; the repeated discovery is negligible beside the walk.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "security: the review payload ships an in-scope file whatever its name, where the writeCode payload refuses secret-named and ignored files — `runtime/src/interpreter/payload.rs`"
    outcome: discarded
    reason: Predates 059; the reviewing host reads the same tree with its own tools, and whether review withholds a file it is reviewing is the review payload's contract.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "perf: `read_repo_file` canonicalizes the project root once per rule file — `runtime/src/interpreter/payload.rs`"
    outcome: discarded
    reason: N is the rule-file count, so the cost is negligible.
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
  - key: "design: an unreadable plan claims nothing in `derive-routing-candidates`, so its skip can name the wrong cause — `runtime/src/primitives/derive_routing_candidates.rs`"
    outcome: discarded
    reason: "Behaviour 059 kept on purpose: routing candidates are advice and the source still lands in `skipped`; the reason text is that primitive's contract."
    decided-at: 2026-09-27T15:36:19Z
    decided-by: andy@stone.dev
---

# Review — 059-project-in-a-repository-subdirectory

## Summary

Review of 059 over the whole in-progress window. No MUST or SHOULD violation stands: the passes raised one MUST and six SHOULD findings, and each was fixed in the run with a test shown failing without its fix.

**Diff base and scope.** The derived base is `cea5a413`, the parent of the commit 059 entered `in-progress` at. The scope is the union of the plan's 18 Affected Files and the 37 files modified since, 37 in all, and it includes 018's re-review artifacts, 017's spec and 022's scenario, which tasks 14 and 17 corrected.

**What this review read: 33 of the 37 files in scope.**

- **Read in full by the lead reviewer (16):** `AGENTS.md`, `framework/bootstrap/ductus-procedure.md`, both shipped hooks, `runtime/Cargo.toml`, `version`, `runtime/src/primitives/derive_routing_candidates.rs`, 017's `spec.md`, 018's five artifacts, and 059's `spec.md`, `plan.md` and `tasks.md`.
- **Read in full by six read-only review agents, each file with its 059 diff (17):**
  - `payload.rs` and `mod.rs`;
  - `check_artifacts.rs`, `check_corpus_links.rs`, `check_stuck.rs` and `analyze_subjects.rs`;
  - `compute_review_scope.rs`, `derive_boundary.rs`, `derive_dependencies.rs`, `derive_references.rs` and `diff_cross_spec.rs`;
  - the Family 22 audit script, the adopter CI template, `amend.md`, `implement.md`, the `implement-basic` golden, and 022's `adopter-generator-promotion.md`.
- **Not examined (4):**
  - `.claude/commands/ductus/amend.md` and `implement.md`, which `scripts/gen-claude-commands.sh` generates from the two command sources; the generator reports no drift.
  - `runtime/CHANGELOG.md`, of which only the `0.54.1` section this spec wrote was read.
  - `runtime/Cargo.lock`, of which only its diff, the version line, was read.

**Rule files.** `quality-cross.md`, `security-backend.md`, `configuration-cross.md` and `performance-backend.md` were read in full and applied. The three frontend files have no surface in scope. The `Verification` triggers of `api-backend.md`, `concurrency-backend.md`, `observability-backend.md` and `reliability-backend.md` were scanned: each names an endpoint, shared mutable state, a deployable service or an outbound call, and no file in scope introduces one.

**Rule findings, fixed in the run (tasks 20 and 21):**

- `BE-INPUT-004` (MUST, low confidence, predates 059): the `writeSpecBody` payload's `read_existing_section` joined the context's `feature` and `path` with no containment.
- `QUAL-CLAIM-001` (SHOULD):
  - A project path containing `*`, `?` or `[` made the `diff-cross-spec` and untracked-listing pathspecs globs that matched nothing, and each reported clean. 059 introduced this.
  - A found repository whose index or status could not be read listed nothing.
  - An unreadable `plan.md` read as a plan naming no files.
- `QUAL-GROUND-001` (SHOULD): `implement.md` parsed `git diff --stat`, which abbreviates a long path to `.../`, and `amend.md` parsed `git status --short`, which follows `status.relativePaths` and `color.status`.

The most serious defect the passes found is an observation rather than a rule finding: a 059 regression in the exec payload's gitignore guard. It asked git about the plan's spelling of a path, so an absolute spelling of an ignored file inside a subdirectory project reached git as `proj//<path>` and was shipped. Both guards now ask about the canonical target, named from the project root.

**Considered and not recorded:** `QUAL-CLAIM-001` on `check-artifacts`' scenario history walk, which flags nothing and records no skipped target when history cannot be consulted. 000's `scenario-without-task-visibility` requires that failure to emit nothing, so the fail-safe direction is the owning contract rather than a claim over an unexamined subject. The fix in the run widened that same direction to unreadable revisions and shallow clones.

**Re-run over the fixed tree.** The passes were re-run over the fix diff since the review began, `2fc8ccd5..aa06bf69`. They found only four comments whose old line breaks had survived in-place edits, reflowed in `aa06bf69`. Every behaviour change is covered by a test mutated to fail without it, and the pre-commit hook ran fmt, clippy and the full suite on each commit.

**Observations: 45.**

- 28 fixed: stale docs, dead code, duplicated helpers, and the Family 22 and CI-annotation defects.
- 17 discarded, each with its reason. They concern behaviour of the pipeline's own machinery that 059 did not change, or costs too small to act on, and each is a contract question for the spec that owns it. The discards are also recorded on 059's task 21.

059 has no scenarios, so the review digest covers no scenario files.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- security: the exec payload's secret and gitignore guards asked about the plan's spelling of a path, so an absolute spelling of an ignored file in a subdirectory project reached git as `proj//<path>` and was shipped (a 059 regression), and `.`, `..` and `//` spellings misfired — `runtime/src/interpreter/payload.rs` — **fixed**
- security: a harmless-named symlink to `.env` or to an ignored file passed both guards, which judged the link's name while the read followed it to the target — `runtime/src/interpreter/payload.rs` — **fixed**
- convention: the `out-of-repo` label and the payload's "repo root" wording mean the project root, and the discovery-failure docs named only a missing repository — `runtime/src/interpreter/payload.rs` — **fixed**
- simplicity: `read_write_boundary` repeated `string_array` with its key fixed — `runtime/src/interpreter/payload.rs` — **fixed**
- convention: `diff-cross-spec`'s module doc equated its result with the pre-`--relative` git form, and its step numbers predated implement's renumbering — `runtime/src/primitives/diff_cross_spec.rs` — **fixed**
- simplicity: `diff-cross-spec` matched in git's names and converted afterwards, leaving the inbox conversion untested; it now converts first, as `derive-boundary` does, with a subdirectory inbox test — `runtime/src/primitives/diff_cross_spec.rs` — **fixed**
- simplicity: dead `bytes` binding kept alive by `let _ = bytes` — `runtime/src/primitives/derive_references.rs` — **fixed**
- convention: the subdirectory test's `[services.api]` config had no effect on anything it asserted — `runtime/src/primitives/derive_references.rs` — **fixed**
- simplicity: local `commit_all` and `write` test helpers duplicated `git_fixture`'s in four test modules — `runtime/src/primitives/check_stuck.rs` — **fixed**
- convention: an intra-doc link to `PrimitiveError::Io` did not resolve, and three `# Errors` sections omitted `InvalidPath` — `runtime/src/primitives/derive_dependencies.rs` — **fixed**
- convention: `ProjectRepository::discover`'s doc named `GIT_WORK_TREE`, which discovery never reads (plan.md carried the same premise), and omitted the `inbox_standing` exception — `runtime/src/primitives/mod.rs` — **fixed**
- convention: a stray `read_text` doc line opened `line_ending_of`'s doc, and `list_feature_dirs`' doc sat on `is_spec_path` — `runtime/src/primitives/mod.rs` — **fixed**
- perf: the `--staged` listing diffed the whole index against HEAD rather than the spec tree — `runtime/src/primitives/mod.rs` — **fixed**
- convention: `subject_digest`'s doc described an excision the code no longer makes, and two references named the retired `is_durable_contract` — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- convention: root-absolute link comments said the repository root where the code resolves against the project root — `runtime/src/primitives/check_corpus_links.rs` — **fixed**
- bug: the repository scope counted a conflicted file once per stage, and its empty-scope guidance named the spec root it never walked — `runtime/src/primitives/check_corpus_links.rs` — **fixed**
- simplicity: `collect_tracked_markdown` and `list_tracked_specs` repeated one index walk, now `tracked_project_paths` — `runtime/src/primitives/mod.rs` — **fixed**
- convention: `find_in_progress_commit` never said its path is named from the work tree — `runtime/src/primitives/check_stuck.rs` — **fixed**
- bug: the scenario history walk skipped a revision it could not read or decode and walked a shallow clone as complete, so it could flag a scenario whose task lived there; it rescanned an unchanged `tasks.md` per commit — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- convention: the module doc, one test name and two test comments described scenario and family behaviour the code no longer has — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- bug: the test helper's unread `_review` parameter left two review-drift tests vacuous; both are reseeded and fail without their behaviour — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- simplicity: `run` re-read the review record and scenario scan `read-spec` already held, and skip deduplication was written four times — `runtime/src/primitives/check_artifacts.rs` — **fixed**
- convention: Family 22 gave all three missing Case 4 calls the staged-listing fix, though only `label-criteria` depends on it — `scripts/audit/adopter-shell-behavior.sh` — **fixed**
- bug: Family 22's fixture commits ran under the user's global git config, so a `commit.gpgsign` there failed a case for a reason not the hook's — `scripts/audit/adopter-shell-behavior.sh` — **fixed**
- bug: Case 2's stub rewrote on the hook's `--help` probe, so its derivation assertion could pass with the real `--write --staged` call removed — `scripts/audit/adopter-shell-behavior.sh` — **fixed**
- convention: Family 22's header described one fixture and one hook, and the fixture-failure block repeated four times — `scripts/audit/adopter-shell-behavior.sh` — **fixed**
- bug: under a `working-directory` default the audit-record gate's annotations named paths from the project root, which GitHub resolves from the repository root — `framework/templates/ci/adopter-generators.yml` — **fixed**
- convention: four comments edited during the fixes kept their old line breaks mid-sentence — `runtime/src/primitives/mod.rs` — **fixed**
- design: the rename exemption's evidence spans the whole repository, so another project's identical rewrite counts toward its two-file test — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: A rename sweep is repository-wide by design and a monorepo's projects share one history; narrowing the evidence to the project changes the mechanical-sweep exemption's contract, which 059 did not change.
- bug: a committed rename sweep can exempt an uncommitted edit to the same subject — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: Predates 059 and concerns how the mechanical-sweep exemption weighs uncommitted work — a contract question for the spec that owns the exemption, not a path conversion.
- convention: the corpus scope matches `.md` case-sensitively where the repository scope does not — `runtime/src/primitives/check_corpus_links.rs` — **discarded**: Predates 059; which files each scope examines is `check-corpus-links`' own contract, and changing either direction changes what the check reports.
- perf: the `check-stuck` and scenario history walks visit every commit in the repository and do not follow renames — `runtime/src/primitives/check_stuck.rs` — **discarded**: Cost and history reach, not a conversion defect: both walks answer correctly for the project, and bounding them by path or following renames changes what each walk is defined to read.
- convention: tests that expect no repository assume the temp directory has no enclosing work tree — `runtime/src/primitives/check_corpus_links.rs` — **discarded**: Predates 059 and is shared with other test modules; a ceiling would need process-wide environment in a parallel suite, and CI's temp directory has no enclosing work tree.
- design: `ProjectRepository::discover` searches upward without a ceiling, so a project inside an unrelated repository that ignores it finds no specs — `runtime/src/primitives/mod.rs` — **discarded**: This is git's own resolution: the repository that contains a directory is the one git reports, and overriding it would disagree with every git command the operator runs.
- security: error messages print the caller's value raw — `runtime/src/primitives/mod.rs` — **discarded**: Predates 059; the values go to stderr, which is not a log, so BE-INPUT-011's conditions do not hold.
- bug: a non-UTF-8 component in the project's path from the work tree renders lossily and matches nothing — `runtime/src/primitives/mod.rs` — **discarded**: No such directory can be created on the platforms this is developed and tested on, so a guard could not be shown failing, and the callers already report `examined: 0`.
- design: amend's reconcile pass offers untracked scenarios but not staged ones — `framework/commands/amend.md` — **discarded**: Predates 059 and is the reconcile pass's own contract; 059 changed only how its paths are named.
- convention: 022's `adopter-generator-promotion.md` quotes the root project's `.githooks` literal — `specs/022-deterministic-runtime/scenarios/adopter-generator-promotion.md` — **discarded**: The literal describes the case its resolved question was decided for, and the reasoning — local git config never travels with a clone — holds for any project.
- bug: the criterion-path grammar accepts `../` and scenario finding paths hard-code a lowercase `.md` — `runtime/src/primitives/check_artifacts.rs` — **discarded**: Predates 059; both are contracts of `check-artifacts`' criterion-path and scenario families, and neither is a path conversion.
- simplicity: three diff-delta walks convert and filter paths in near-identical code — `runtime/src/primitives/diff_cross_spec.rs` — **discarded**: The walks read different sides of a delta, so one helper would need a mode per caller; that is not a mechanical merge.
- design: an unparseable config gives `derive-references` an empty registry — `runtime/src/primitives/derive_references.rs` — **discarded**: Predates 059 and is the registry contract of the spec that owns cross-service references; `registered-services: 0` already reports it.
- perf: `first_commit_for_prefix` reads past its first match, and each listing discovers the repository up to three times per run — `runtime/src/primitives/derive_boundary.rs` — **discarded**: git2 reports an aborted walk as an error, so an early stop would have to be told apart from a real failure; the repeated discovery is negligible beside the walk.
- security: the review payload ships an in-scope file whatever its name, where the writeCode payload refuses secret-named and ignored files — `runtime/src/interpreter/payload.rs` — **discarded**: Predates 059; the reviewing host reads the same tree with its own tools, and whether review withholds a file it is reviewing is the review payload's contract.
- perf: `read_repo_file` canonicalizes the project root once per rule file — `runtime/src/interpreter/payload.rs` — **discarded**: N is the rule-file count, so the cost is negligible.
- design: an unreadable plan claims nothing in `derive-routing-candidates`, so its skip can name the wrong cause — `runtime/src/primitives/derive_routing_candidates.rs` — **discarded**: Behaviour 059 kept on purpose: routing candidates are advice and the source still lands in `skipped`; the reason text is that primitive's contract.

## Skipped passes

*None.*

## Unexamined governance

*None.*
