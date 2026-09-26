---
status: draft
dependencies: [058-findings-route-at-discovery]
next-criterion: 5
---

# 059 — Project in a repository subdirectory

Every primitive that asks git about a spec's history names the spec's path from the git work tree, so a project that sits in a subdirectory of its repository reads the same history it would at the repository root.

## Motivation

A project need not sit at its repository's root. Git names every path in its trees, its index, and its diffs from the work tree, while a project names its specs from the project root; the two differ by the project's directory inside the repository. [058](../058-findings-route-at-discovery/spec.md) found two readers that did not convert, `compute-review-scope` and the inbox age, and fixed both through one helper, `workdir_prefix` (`runtime/src/primitives/mod.rs:1298`, called at `compute_review_scope.rs:144` and `inbox_standing.rs:187`).

058's third review found four more with the same defect, older than 058, and its task 46 routed them here. Each discovered the repository and then looked up a spec-root-relative path in its trees, so in a subdirectory project it found no history for the spec:

- `check-stuck` looked for the spec's `in-progress` commit under `{specs-root}/{feature}/spec.md` (`runtime/src/primitives/check_stuck.rs:41-45`). It found none, counted no commits, and never reported a spec stuck.
- `check-artifacts`' scenario-to-task family searched history for `{specs-root}/{feature}/tasks.md` to learn whether a `done` spec's scenario ever had a task (`runtime/src/primitives/check_artifacts.rs:419-431`). It found no revision, so a scenario whose task had been pruned was reported as never having had one: the finding that walk exists to suppress.
- `diff-cross-spec` and `derive-boundary` looked for the first commit touching `{specs-root}/{feature}/` (`runtime/src/primitives/diff_cross_spec.rs:55-60`, `runtime/src/primitives/derive_boundary.rs:45-57`). They found none and answered as for a spec never committed: cross-spec impact unknown, and a boundary of the spec directory alone.

## Acceptance Criteria

- [ ] AC1: `check-stuck` finds a subdirectory project's `in-progress` commit and counts the commits touching its `tasks.md` since then, as it does at the repository root
- [ ] AC2: `check-artifacts` finds a pruned task in a subdirectory project's `tasks.md` history and raises no scenario-to-task finding for its scenario
- [ ] AC3: `diff-cross-spec` and `derive-boundary` find a subdirectory project's first spec-directory commit, and report every path project-relative, dropping paths outside the project, as `compute-review-scope` does
- [ ] AC4: Each primitive above resolves its history paths through `workdir_prefix`, and each has a test against a project in a subdirectory of its repository

## Open Questions

- Do the other git readers share the defect? `analyze_subjects` (`runtime/src/primitives/analyze_subjects.rs:303`) and the interpreter's payload builder (`runtime/src/interpreter/payload.rs:1122`) also call `Repository::discover` without `workdir_prefix`, and `check-corpus-links` opens the project root itself as the repository (`runtime/src/primitives/check_corpus_links.rs:133`), which a subdirectory project's root is not. None was examined for it.
