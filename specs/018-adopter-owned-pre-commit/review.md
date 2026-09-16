---
spec: 018-adopter-owned-pre-commit
diff-base: 91741dfb77a5e46e184257c2313afa9684f2da62
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:14:20Z
reviewed-against: 91741dfb77a5e46e184257c2313afa9684f2da62
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 7
reviewed-digest: {}
blocking: false
---

# Review — 018-adopter-owned-pre-commit

## Summary

First review of 018 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. **All thirteen criteria verified against the tree and every one holds** — no correction, so 018 never left `done`.

**What this review read: 5 of the 7 files in scope, and here is the other 2.** Read in full: `framework/bootstrap/ductus.md` (1181 lines), `framework/bootstrap/hooks/ductus-pre-commit`, `framework/bootstrap/hooks/pre-commit`, and this spec's `plan.md` and `tasks.md`. **Not examined:** `specs/017-derive-dont-ask/spec.md` — another spec's artifact, read only at the two places 018's criteria reach into it (the signpost block after its H1, and its `dependencies:` line); and `framework/bootstrap/hooks/install.sh`, which **does not exist**, because AC12 deleted it. A scope entry naming a file this spec's own criterion removed is the plan behaving as a design record, not drift.

**The seven skipped criteria, walked by hand.** `check-artifacts` reports six `not-a-live-claim` and one `ships-to-adopter` on this spec — the largest skip set in the zero-contract tier. Per AGENTS.md those are the manual worklist: a criterion in that bucket has had **nothing** check the substance of its claim, ever. Each was checked against the tree this run and each holds. The `not-a-live-claim` exemptions are correct in every case — AC12 says `install.sh` *is deleted*, AC1 and AC13 carry supersession annotations naming `.ductus/scripts/` as gone, and a path-existence family would report those backwards.

**How each criterion was checked.** AC1: the inner hook ships, carries `# managed-by: ductus` on **line 2** exactly, and orchestrates `derive-dependencies` / `derive-references` plus `git add` staging — the criterion's own annotation already records that the shell generators it named became primitives. AC2: the outer stub invokes `./.githooks/ductus-pre-commit` and carries no sentinel anywhere — checked by reading the whole file, since "no sentinel" is a claim about absence. AC3, AC4: the manifest rows sit at lines 768 and 785, which fall inside the `strategy: update` and `strategy: create` subsections respectively — the strategy is positional here, so the row alone would not have proved it. AC5: the ladder is four items, and the sentinel-detected branch is gone, with prose stating why. AC6: the manual-integration snippet names `./.githooks/ductus-pre-commit`. AC7: §Migration from spec-017 hook carries the three-part trigger, the tracked/untracked branch, both recovery paths, and the summary line. AC10: 017's signpost sits immediately after its H1 and names AC21–AC23. AC12: the file is gone, and no **live** artifact references it — the surviving mentions are all in 017's and 018's own bodies, plans and tasks, which are design records. AC13: `derive-dependencies` excludes blockquote-prefixed lines, and 017's `dependencies:` is `[]`; I confirmed the behaviour independently earlier today when a blockquote signpost added to 006 induced no edge.

**Three criteria I did not re-verify, and why that is stated rather than implied.** AC8, AC9 and AC11 are end-to-end claims about a real `/ductus` run against a sandbox adopter — migration path and fresh-install path. The plan names manual sandbox verification as their discharge criterion and `tasks.md` task 9 records both runs. Re-running a bootstrap was out of scope here, so what this review verified is the **procedure** they rest on — the ladder, the migration subsection, the manifest strategies and the `chmod`/`core.hooksPath` inlining — not the runs themselves. AGENTS.md is explicit that a real adopter run is the only test of composition this project has; nothing in this repo substitutes for it.

**The hook read as code, not just as a manifest entry.** `ductus-pre-commit` halts rather than skips on an unreachable runtime, and additionally probes each primitive's `--help` before calling it, so an old-but-present binary fails with an actionable message instead of a bare clap error. Its `set -e` idioms are the safe ones throughout — `[ -f "$f" ] || continue` rather than a trailing `&&`, and `{ grep … || true; }` around the staged-spec match so a no-match does not abort the run. The staged-spec regex matches the spec root by *shape* rather than by the literal `specs`, which is what keeps it correct for a project that renamed its spec tree.

**On the diff base.** No commit records 018 entering `in-progress`, so the natural derivation is empty and `write-review` would collapse the denominator to `scope: 0` under a non-zero `examined`. `HEAD` is passed instead. 018 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

**Checked and clean elsewhere.** `README.md` and `docs/` carry no description of the two-file hook model, so there is no mirror to drift. The shipped adopter CI template was checked because `tasks.md` records a boundary expansion into it: its error message no longer names the deleted `gen-spec-deps.sh` and correctly points at `ductus derive-dependencies --write`.

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
