---
spec: 059-project-in-a-repository-subdirectory
last-run: 2026-09-27T15:44:27Z
analyzed-against: c7ad37482e601cd6b9f24d1eba636937cefee2f4
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: 4b7c06a03b9a63b6007e4d5c31ee3173f67584a6df5098311cc807eb185c879e
  review.md: f053f9111f8b8862307c64be8f83fcf0459069dcdea246e11afea64c805adc36
  spec.md: 2d7f1198098c7361f1a89723470e4096f8e80964e112d3edd66e153c56dc67f7
  tasks.md: 2b4b24f03af4e11c39ec86bbede8ac8d9aac5cfeae6ab4e56f1c915f990647d0
blocking: false
dispositions:
  fixed: 9
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 059-project-in-a-repository-subdirectory

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 9 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- grounding — spec.md states that `Repository::open` does not search upward with no citation of git2's contract — `specs/059-project-in-a-repository-subdirectory/spec.md` — **fixed**
- grounding — spec.md states that `workdir_prefix` canonicalizes the work tree and the project root with no citation of the code — `specs/059-project-in-a-repository-subdirectory/spec.md` — **fixed**
- grounding — spec.md states that discovery finds the innermost repository with no citation of libgit2's discovery contract — `specs/059-project-in-a-repository-subdirectory/spec.md` — **fixed**
- grounding — spec.md states that `pre-commit` installs without setting `core.hooksPath` with no citation of pre-commit's install code — `specs/059-project-in-a-repository-subdirectory/spec.md` — **fixed**
- grounding — plan.md states that the `open` sites do not search upward with no citation of git2's contract — `specs/059-project-in-a-repository-subdirectory/plan.md` — **fixed**
- grounding — plan.md states that the manifest places the inner hook at `{project}/.githooks/ductus-pre-commit` with no citation of the manifest — `specs/059-project-in-a-repository-subdirectory/plan.md` — **fixed**
- grounding — plan.md states that `git diff --cached --name-only` names paths from the work-tree root with no citation of git-diff(1) — `specs/059-project-in-a-repository-subdirectory/plan.md` — **fixed**
- grounding — plan.md states that `pre-commit` installs into `.git/hooks` without setting `core.hooksPath` with no citation of pre-commit's install code — `specs/059-project-in-a-repository-subdirectory/plan.md` — **fixed**
- grounding — plan.md states that `git status --porcelain` reports the whole work tree with no citation of git-status(1) — `specs/059-project-in-a-repository-subdirectory/plan.md` — **fixed**
