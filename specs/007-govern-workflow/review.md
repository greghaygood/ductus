---
spec: 007-govern-workflow
diff-base: bd98776290f0e5ed072cf0bb0dce0f5affd340d6
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T18:15:56Z
reviewed-against: 2e63261d7398ad41b3d6a974749a95ac98ba2466
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 25
reviewed-digest:
  scenarios/ductus-self-update-precheck.md: c03909715a9bdd56b2c97a8d9bf89e970d5d8077afa61cb62b8bd3ad8998fbfe
blocking: false
---

# Review — 007-govern-workflow

## Summary

Five passes over a 25-file scope of which **8 still exist**. Read in full: 7. Partially examined: 1, named below. Absent: 17, named below. Backfill re-review — the 2026-05-10 record carried no `examined`, no `scope` and no `reviewed-digest`, the oldest in the corpus.

**Scope, all three bases measured before choosing.** Pre-reopen natural base `3d7c50be` resolved **828** files. `--since HEAD` resolved **19** (plan-affected only, excluding this pass's edits). The **post-reopen natural base `bd987762`** resolves **25** and is what this records: the union of the plan's 19 affected files and the 7 this pass touched.

**Seventeen in-scope paths no longer exist, and stay in scope because the plan lists them.** They are not unread — there is nothing to read. Thirteen `commands/*.md` (`about`, `analyze`, `clarify`, `commit-push`, `implement`, `next`, `plan`, `scenario`, `setup`, `specify`, `status`, `target`, `triage`) moved to `framework/commands/` and several were renamed or retired; `ductus/ductus.md` and `ductus/ductus-auggie.md` were replaced by the single registry-driven `framework/bootstrap/ductus.md`; `specs/007-govern-workflow/scenarios/govern-self-update-precheck.md` is the pre-rename name of a file this pass renamed. The last entry is not a path at all — `.claude/commands/ductus/*.md` is a **glob** written into an Affected Files table, so no check could ever have resolved it.

**One file partially examined, stated here rather than counted as read.** `framework/bootstrap/ductus.md` is 1182 lines / 141 KB. I read **lines 1–249 in full** — the frontmatter parity block, all ten §Instructions steps, the four-row §Agent Registry, both §Derived values tables, §MCP registration, §Adding a new agent, §Inputs, §Pre-flight Checks, §Agent Selection, §Permission Setup, §ductus runtime auto-wiring, §Pre-flight Phase, and §ductus runtime detection through State B — plus targeted reads of §Project Configuration's `rules.surfaces`, the Shared Files rule-file surface filter, the `.gitignore` merge steps, and the pre-existing-hook migration note. **I did not read roughly lines 250–1182** continuously: Runtime acquisition, Pointer materialization, MCP wiring, the Self-Update Check, Pre-run Migrations, Collect Project Inputs, the Shared Files manifest table, Per-Agent Scaffolding, Slash command cleanup, Security Audit, Post-Scaffolding Output, Re-Run Behavior and Session state. That is enough to ground every claim 007 makes about the installer and **not** enough to be a five-pass review of the installer itself; recording `examined: 8` would have been the falsification this campaign exists to remove, so it is 7.

**Criteria verified against the tree, enumerations included — three were false and are now annotated.** **AC1** named `ductus.md` variants in a `ductus/` directory; neither exists, superseded twice (012's single registry-driven installer, then the layout move). **AC3**'s parenthetical named two agents where the registry carries four across three layouts; the requirement holds, the enumeration is superseded. **AC14** said adding a CLI needs only a new variant file; §Adding a new agent (read at lines 97–106) specifies a registry row, an MCP registration row, a `configure/{key}.md`, a README snippet, and — on a new layout — §Derived values branches. All three annotated rather than rewritten: the claims were superseded, not renamed, and restating them would credit 007 with 012's work. `009`'s AC18 carries the same annotation for the same supersession.

**Why nothing had ever caught AC1 — a gap now recorded in `AGENTS.md` §Gotchas.** `check-artifacts` reported `clean: true` with an **empty** `skipped` array. `is_path_like` requires an *interior* `/` after trailing slashes are stripped, so `ductus/` reduces to `ductus` and `ductus.md` never had one: both dead paths were rejected as candidates despite correct backticks, producing neither a finding nor a skip. The existing §Gotchas entry said only that unbackticked paths are invisible; backticks turn out to be necessary but not sufficient.

**A second blind spot, same pass.** `tasks.md` task 6 referenced `scenarios/ductus-self-update-precheck.md` while the file on disk was `govern-self-update-precheck.md` — 049's sweep rewrote the task text, the plan and the scenario's own title but could not rename a file. Invisible in both directions: `scenario-consistency` is **skipped for `done` specs** by design, and **no family checks the reverse direction**, that a task's named scenario exists. File renamed; the reference resolves.

**Decisions read as live claims.** The plan's §Trade-offs rejected "a single ductus file with CLI prompt" because the user had already chosen their CLI by installing into a specific directory — and that is the shipped design, reversed by 012. The premise dissolved with the two-file model supporting it. Annotated in place; left alone it argues against the design that won. The scenario's signpost justified preserving its pre-fix body "per the constitution's frozen-archaeology rule", deleted by 023 — reworded in place, signpost intact, per inbox item 46's explicit *reword, never delete*.

**Mirrors checked.** `README.md`, `AGENTS.md`, `CLAUDE.md`, `docs/` and `framework/templates/` carry no copy of 007's superseded claims. `.claude/commands/ductus/init.md` was read in full and confirms all five drifts its inbox item measured, so the operator's retire decision rests on accurate evidence.

**Rule set.** 11 files loaded. Their Verification steps target code, or specs and plans introducing shared state, outbound calls, config values, metrics, API surface or UI. 007's change here is annotation and naming in markdown artifacts; no trigger fires. The reuse pass is the one with a live surface and it caught nothing new beyond the duplication already annotated.

**Verification at this HEAD.** 20 test binaries invoked and 20 reported, 0 failures; six `lint-*.sh` green; markdownlint over 513 files; the 37-family self-audit green — and it **caught a defect I introduced**: the first AC1 annotation cited 012 as a markdown link, which `derive-dependencies` harvests into an edge, closing a cycle with 012's existing dependency on 007. Replaced with a backticked slug; `derive-dependencies` now reports no drift and 007's `dependencies:` is unchanged at `[003-bootstrap-automation]`.

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
