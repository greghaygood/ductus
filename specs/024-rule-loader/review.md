---
spec: 024-rule-loader
diff-base: bbb1f4169533a355263e85fc98d5d62add4bb953
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:23:05Z
reviewed-against: bbb1f4169533a355263e85fc98d5d62add4bb953
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 11
reviewed-digest: {}
blocking: false
---

# Review — 024-rule-loader

## Summary

First review of 024 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All nine criteria verified against the tree; **AC4 was half false**, and two body claims AGENTS.md had already flagged were corrected.

**What this review read: 8 of the 11 files in scope, and here is the other 3.** Read in full: `framework/commands/review.md`, `analyze.md`, `groom.md`, `framework/constitution.md`, `framework/bootstrap/ductus.md`, `specs/README.md`, `scripts/lint-rule-filenames.sh`, and `framework/rules/quality-cross.md` via the rule-file load. **Not examined:** `framework/commands/implement.md` — read only at the one line 024 touched, its Reference list naming `configuration-cross.md`; `.claude/commands/ductus/review.md`, a generated copy of a source read here; and **two paths that do not exist** — `framework/rules/configuration.md`, which AC4 itself renamed away, and `.github/workflows/markdown-only-pipeline.yml`, retired by 048.

**AC4's second half no longer held.** It records the `configuration.md` → `configuration-cross.md` rename as filed under `specs/README.md` **§Past Renames**, and asserts done-spec bodies "stay as written". 023's `living-specs` scenario deleted that section *and* removed the exemption behind the second clause. The rename is still recorded in `specs/README.md` — in the prose that replaced §Past Renames, which names this rename explicitly — so the requirement survives and only the mechanism moved; the criterion now says so.

**Two present-tense claims AGENTS.md had already adjudicated.** 024's spec and plan asserted that done spec bodies **are** frozen archaeology — 020 "is frozen archaeology", done-spec bodies "are frozen … and stay as written". AGENTS.md names these two files specifically as the last surviving present-tense instances and calls the fix a meaningful edit. Both are now past-tense accounts of what was done under the rule as it then stood, followed by the rule as it is — the form AGENTS.md marks correct for 016, 018 and 023.

**A correction I made and then had to undo.** Rewriting that bullet, I first cited 020 by backticked slug to avoid minting a dependency edge. That was wrong in this direction: 020 was already a declared dependency and a real one, since 024 rewrites the rule selection inside the command 020 created. De-linking removed a true edge rather than avoiding a false one, and `derive-dependencies` duly dropped it. Relinked in `e76c80a`; `dependencies:` is back to `[020-code-review, 023-govern-refinement]`. The backtick rule is for citations that are not dependencies, and this was the other case.

**A stale claim in a live artifact, found by reading the scope.** `specs/README.md`'s first Design Decision placed all templates in `templates/` at the repo root — a directory gone since the `framework/` reorganization. Its adopter-side half was correct and is kept: the manifest still lands spec templates at `{project}/specs/templates/`, so only the source moved. Fixed in `bbb1f41`.

**How each criterion was checked.** AC1: the constitution carries §rules → *Filename suffix* with the three-suffix closed set, and `lint-rule-filenames.sh` enforces it — read in full, and it correctly names `framework-checks.yml` as its consumer rather than the workflow 048 retired. AC2: `review.md`'s selection is the `discover-rule-files` derivation plus a suffix walk; the only surviving mention of `security-backend.md` is an adopter-notes pinning *example*, not a selection criterion, which is the distinction the criterion turns on. AC3: the three files load — confirmed by running the discovery, which returned all eleven including `api-backend.md`, `accessibility-frontend.md` and `performance-frontend.md`. AC5: `review.md` §Notes for adopters keeps the `AGENTS.md` fallback narrowed to files outside the rule directory, naming `docs/rules/internal-api.md`. AC6: the unrecognized-suffix path is load-for-every-stack plus the exact one-line warning, with "never silent skip" stated. AC7: the `loading rule files: …` notice is emitted — observed directly in this run's discovery output. AC8: §Notes for adopters describes the derivation rather than the retired `AGENTS.md`-reference rule. AC9: `analyze.md` uses the same discovery and loads **every** file regardless of stack, with the reason given — citation verification spans surfaces.

**On the diff base.** No commit records 024 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 024 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
