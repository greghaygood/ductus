# 059 — Project in a repository subdirectory Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tasks 1–9 are runtime work, verified through the built binary rather than the MCP tools, which answer with the binary the session started on. Every new test is shown to fail with its conversion removed before it is trusted.

## 1. `ProjectRepository` and the subdirectory fixture

- [x] Add `ProjectRepository { repository, prefix }` with `discover`, `to_git`, and `to_project` to `runtime/src/primitives/mod.rs` beside `workdir_prefix`, building the prefix as `history_prefix` does, and erring when `workdir_prefix` returns `None`
- [x] Unit tests: an empty prefix at the root; `proj/` in a subdirectory; `to_project` returns `None` for a path outside the project and for a sibling that shares the prefix's spelling without its `/` (`project-other/x`); a symlinked project resolves the prefix of its target
- [x] Add a `#[cfg(test)]` fixture that builds a repository with the project at `proj/` (its own `.ductus/config.toml`) and one shared `commit_all`
- [x] Move `compute_review_scope.rs` onto the helper and delete `history_prefix`; its existing subdirectory and root tests stay green

- **Done when**: the helper's tests pass, `compute_review_scope` has no private prefix code, and `cargo test --release --locked` passes.

## 2. `check-stuck` (AC1)

- [x] Open through `ProjectRepository`; build `spec_rel` and `tasks_rel` with `to_git` (`check_stuck.rs:41-45`)
- [x] Test: in a subdirectory project, the `in-progress` commit is found and the commits touching `tasks.md` since it are counted, the same numbers the root-layout test asserts

- **Done when**: the new test passes and fails with `to_git` removed.

## 3. `check-artifacts` scenario-to-task walk (AC2)

- [x] Open through `ProjectRepository` in `ever_tasked_slugs`; build the `tasks.md` path with `to_git` (`check_artifacts.rs:419-420`)
- [x] Test: a `done` subdirectory spec whose scenario's task was pruned from `tasks.md` raises no scenario-to-task finding, and a scenario that never had a task still does

- **Done when**: both directions pass, and the first fails with `to_git` removed.

## 4. `diff-cross-spec` and `derive-boundary` (AC3)

- [ ] `diff_cross_spec.rs`: open through the helper; `to_git` on `spec_prefix`, `root_prefix`, `inbox_rel`, and the pathspec; `to_project` on every reported path, dropping those outside the project
- [ ] `derive_boundary.rs`: open through the helper; `to_git` on `spec_prefix`; `to_project` on every changed path before `zone_glob`, dropping those outside the project; the spec glob and guidance stay project-relative
- [ ] Tests, each in a subdirectory project with a sibling project changed in the same window: the first spec-directory commit is found; every reported path is project-relative; the sibling project's paths are absent

- **Done when**: the tests pass for both primitives, and each fails with its conversion removed.

## 5. Exec payload gitignore guard (AC5)

- [ ] `payload.rs:1122`: open through the helper; `is_gitignored` asks about `to_git(rel)`
- [ ] Test: a file the project's own `.gitignore` ignores through an anchored pattern, under a name the basename secret check does not catch, is refused with the `.gitignore` pattern
- [ ] Test: a project file that a pattern anchored at the work-tree root would match if the path were read from the root is not refused
- [ ] Test: a plan path escaping the project is still refused as out-of-repo

- **Done when**: all three pass, and the first two fail with `to_git` removed.

## 6. Index-scoped spec listing (AC6)

- [ ] `list_tracked_specs`, `list_untracked_specs`, `list_staged_specs` (`mod.rs:2193`, `:2227`, `:2266`): open through the helper; `to_git` on the status pathspec; `to_project` on every index, status, and delta path before `is_spec_path`
- [ ] `derive_dependencies.rs` tests in a subdirectory project with a tracked spec and an untracked draft: `--write` never rewrites the draft; the draft is reported in `untracked-skipped`; `--staged` rewrites exactly the staged spec
- [ ] The same three assertions for `derive_references.rs`

- **Done when**: the tests pass for both generators, and each fails with the conversion removed.

## 7. Rename exemption (AC7)

- [ ] `exempt_renames` (`analyze_subjects.rs:303`): open through the helper; look each candidate up as `to_git(path)`; the returned set stays project-relative
- [ ] Test: in a subdirectory project, a uniform repo-wide rename leaves both the review and the analyze record current, and a real change to a subject still reads stale

- **Done when**: both directions pass, and the first fails with `to_git` removed.

## 8. `check-corpus-links --scope repository` (AC8)

- [ ] `collect_tracked_markdown` (`check_corpus_links.rs:132`): open through the helper; `to_project` on each index entry, skipping those outside the project
- [ ] Reword the repository-scope guidance (`check_corpus_links.rs:86-87`) to name both causes: no repository contains the project, or its index could not be read
- [ ] Test: a repository with a broken link outside the project and a clean project examines only the project's tracked markdown, names each file project-relative, and reports no broken link

- **Done when**: the test passes and fails with `to_project` removed.

## 9. Runtime gate

- [ ] `cargo fmt --check`, `cargo clippy --release --locked -- -D warnings`, and `cargo test --release --locked` in `runtime/`, each exit status read on the line that ran it
- [ ] Re-grep `runtime/src` for `Repository::discover` and `Repository::open` outside test modules: only `ProjectRepository::discover` and `inbox_standing` remain
- [ ] Build the release binary; note in the commit that the MCP server must be restarted on it before any review or analysis of 059

- **Done when**: all three commands exit 0 and the re-grep shows no unconverted reader.

## 10. The hook runs from the project root (AC9, hook half)

- [ ] `framework/bootstrap/hooks/ductus-pre-commit`: replace the toplevel `cd` (`:47-48`) with the project root from `${BASH_SOURCE[0]}`; add `--relative` to the staged-spec listing, and update the comments that describe both
- [ ] `framework/bootstrap/hooks/pre-commit`: invoke the inner hook by its own location rather than `./.githooks/ductus-pre-commit` from the toplevel
- [ ] `scripts/audit/adopter-shell-behavior.sh`: add a fixture with the project at `proj/`, run the outer stub and the inner hook from the work-tree root, and assert the stub runtime ran, the staged spec was labelled and re-staged, and the hook exited 0
- [ ] Show the new fixture failing against the current hook before trusting it

- **Done when**: Family 22 passes with the new fixture, the fixture fails against the pre-change hook, and the existing root fixtures still pass.

## 11. `/ductus` wires the hooks directory named from the work tree (AC9, wiring half)

- [ ] `framework/bootstrap/ductus-procedure.md` §Hook Installation: compute `P` with `git rev-parse --show-prefix` and `H = {P}.githooks`; run the ladder against `H`; add the pre-059 rewire case; look for third-party markers at the work-tree root too; add the pre-059 outer-stub precondition with its warning
- [ ] Update the two-files description (`ductus-procedure.md:233-234`) and the manual integration snippet (`./{P}.githooks/ductus-pre-commit`)
- [ ] Walk the ladder by hand against each spec edge case (root project, subdirectory project, pre-059 value, someone else's `.githooks`, third-party marker at the work-tree root, pre-059 outer stub, second project in one repository) and confirm each reaches the stated branch

- **Done when**: every edge case in the spec maps to exactly one ladder branch, and a root project's walk is unchanged.

## 12. CI template (AC10)

- [ ] `framework/templates/ci/adopter-generators.yml`: add a commented job-level `defaults.run.working-directory`, with a comment saying a project that is not at the repository root sets it to its path

- **Done when**: the template parses as YAML with the default commented out and with it uncommented, and every `run` step's paths resolve from the working directory.

## 13. Prose-claim sweep

- [ ] Re-grep live artifacts for `core.hooksPath`, `show-toplevel`, and `./.githooks/ductus-pre-commit`; classify each hit as corrected here, historical (a past-tense record, a changelog, a migration), or out of scope with its reason in the plan
- [ ] Grep for claims that a project sits at its repository's root, by meaning, across `README.md`, `framework/bootstrap/`, and `framework/commands/`

- **Done when**: every hit is corrected or classified, and no present-tense claim of the old wiring remains outside spec 018.

## 14. Discharge the impact on 018

- [ ] Reopen `018-adopter-owned-pre-commit` (`done → in-progress`) through the status primitive
- [ ] Replace its embedded outer stub (`spec.md:30-44`) with a pointer to `framework/bootstrap/hooks/pre-commit`; replace the ladder (`:52-61`) and snippet (`:63-75`) with pointers to `framework/bootstrap/ductus-procedure.md` §Hook Installation; add a signpost linking back to 059
- [ ] Read 018's ticked criteria against the new behavior; annotate any the change supersedes rather than rewriting it
- [ ] Run `/ductus:review` and `/ductus:analyze` on 018 and return it to `done` through its gate

- **Done when**: 018 is `done`, its body links to 059, and `check-review-gate` on 059 reports the impact discharged.

## 15. Release `0.54.1`

- [ ] Bump `version` and `runtime/Cargo.toml:3` to `0.54.1`; refresh `runtime/Cargo.lock` with one build without `--locked`
- [ ] Add a `runtime/CHANGELOG.md` section covering tasks 1–9
- [ ] Run audit Family 20

- **Done when**: Family 20 passes and the changelog section names every runtime change. The tag is cut after 059 and 018 are `done`.
