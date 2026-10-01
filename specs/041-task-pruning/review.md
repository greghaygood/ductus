---
spec: 041-task-pruning
last-run: 2026-10-01T22:06:13Z
reviewed-against: c9ccf64c1bc3b511392c36ec597ce746bdc8fd1b
diff-base: 38112a1d4f001640858dbc4488900733a62ff7b2
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 59
skipped-passes: []
reviewed-digest:
  data-model.md: d1e04eeaa9f74921b26f7c3b5101c70832e7469dfc590f15e787d24cea604535
  scenarios/plan-records-the-design-as-it-stands.md: a50174ff23b21bc5475d52cb6f289f06085066fae8c7151fbf1be8eaa727221b
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 9f59c1ae42c42cc57e4f7a86379aefed593080ddbe40a0bbeea9e11c59bbb604
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
decisions:
  - key: "efficiency: exec analyze's step 16 reads HEAD on a done spec for a reopen-required it ignores, and halts when HEAD cannot be read — `runtime/src/interpreter/mod.rs:1078`"
    outcome: discarded
    reason: the cost was accepted in the 2026-10-01 decision recorded in prune-reduces-the-spec-directory's Resolved Questions
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs:689`"
    outcome: discarded
    reason: flat registries pinned by set-equality tests, a design AGENTS.md records
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs:2864`"
    outcome: discarded
    reason: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710`"
    outcome: discarded
    reason: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "simplicity: check_stuck converts the tasks path to a git path twice — `runtime/src/primitives/check_stuck.rs:91`"
    outcome: discarded
    reason: the helper takes a project-relative path by design; the cost is one string build
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: three sha256-to-hex spellings across analyze_subjects, prune_plan and fetch_archive — `runtime/src/primitives/analyze_subjects.rs:202`"
    outcome: discarded
    reason: consolidating them touches fetch-archive (048) for no change in behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing re-implements the HEAD blob read, and prune_plan imports shared helpers from prune_tasks — `runtime/src/primitives/inbox_standing.rs:191`"
    outcome: discarded
    reason: inbox_standing is the documented raw-repository exception, and where an import comes from changes no behavior
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: split_frontmatter takes a later bare-LF --- as the fence when the frontmatter closes with CRLF — `runtime/src/primitives/mod.rs:857`"
    outcome: discarded
    reason: no such mixed-ending file exists here, and the fix reaches every frontmatter reader for a case not seen
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: SkipScanner recognizes only backtick fences, so a ## inside a ~~~ fence is a section — `runtime/src/primitives/mod.rs:1742`"
    outcome: discarded
    reason: SkipScanner is 022's shared scanner, no ~~~ fence exists in this corpus, and widening it changes every parser built on it
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the prune-tasks and prune-plan walk skeletons are near-copies — `runtime/src/primitives/prune_plan.rs:210`"
    outcome: discarded
    reason: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "contract: an applying prune-tasks walk that errors on a later spec returns no record of the reductions it already wrote — `runtime/src/primitives/prune_tasks.rs:176`"
    outcome: discarded
    reason: each write is atomic and idempotent, so re-running the walk resumes; the data model and plan now say so
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "edge: a reset also drops anything above the H1, and its H1 search scans fenced and commented lines — `runtime/src/primitives/prune_tasks.rs:477`"
    outcome: discarded
    reason: a tasks.md opens with its H1 by construction from the template, and no file here has content above it or a heading only inside a fence
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "test: no prune-plan test that a preview on a done spec errors when HEAD cannot be read, or on a malformed decisions list — `runtime/src/primitives/prune_plan.rs:299`"
    outcome: discarded
    reason: the preview and apply share one reopen computation, whose unreadable-HEAD error is tested through an apply, and the decisions reader's refusal is tested where it lives
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: AGENTS.md's 022 routing entry names primitives/mod.rs as if it held every shared parser — `AGENTS.md:69`"
    outcome: discarded
    reason: the entry's catch-all, anything whose contract other specs rest on, already covers rule_sections.rs and parser/mod.rs
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "quality: a remote-tracking ref origin/ci/041-windows-check may survive locally — `AGENTS.md:87`"
    outcome: discarded
    reason: a local remote-tracking ref, not repository content and not 041's work
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: AGENTS.md entries restate 061's rationale and the recommendations rule, and carry dated measurements of 022's record and the file's size — `AGENTS.md:46`"
    outcome: discarded
    reason: each records what a rule cost here at a dated moment, which is what the file is for; classification to the constitution is 050's backlog
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8`"
    outcome: discarded
    reason: readability of correct procedure text; the content is accurate after this run's fixes
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: AC11 names only the --reset divergence while the design-record heading set has one too — `specs/041-task-pruning/spec.md:56`"
    outcome: discarded
    reason: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "claim: plan.md narrates the change it makes (gains, is sharpened, is corrected) inside the design record — `specs/041-task-pruning/plan.md:178`"
    outcome: discarded
    reason: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the template — `specs/041-task-pruning/data-model.md:231`"
    outcome: discarded
    reason: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 058's spent tasks cite analyze step 16, and 058's plan keeps a now step 19 history sentence and line citations — `specs/058-findings-route-at-discovery/plan.md:351`"
    outcome: discarded
    reason: spent tasks are not rewritten, the history sentence was discarded in 048's review (f7c14890), and path:line citations are provenance AGENTS.md says not to sweep
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 060's plan cites analyze.md line numbers that moved, and a 058 scenario's Context describes the pre-fix behavior — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md:87`"
    outcome: discarded
    reason: path:line provenance, and a Context section records what motivated its scenario
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
  - key: "stale: 041's spent task 2 says segmentation uses iter_phase_ranges — `specs/041-task-pruning/tasks.md:18`"
    outcome: discarded
    reason: a spent task's record is not rewritten; the plan and data model state the design
    decided-at: 2026-10-01T18:24:24Z
    decided-by: andy@stone.dev
---

# Review — 041-task-pruning

## Summary

Not blocking. A deliberate partial review of 041-task-pruning's reopen (natural base 38112a1d, the commit before the reopen 4f6b50fd), in the shape the operator approved on 2026-10-01: the change this reopen carries, not a full pass. The five passes read the complete diff since 38112a1d — every changed hunk of every changed file, with context, including 3e323778's exec-gate fix and the review chores in dc703836, 3e323778 and c9ccf64c — and these 4 of 59 scope entries in full: `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`; `specs/041-task-pruning/scenarios/plan-records-the-design-as-it-stands.md`; `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`; `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`. No rule finding. The exec /prune QUAL-STUB-001 MUST that the review run beside 0.56.0's found is fixed in 3e323778 (022 scenario a-confirmed-gate-authorizes-the-writes-after-it); its walker test fails with the binding removed. 1 observation(s), each fixed in c9ccf64c. Not read in full, named individually: `.claude/commands/ductus/*.md`; `.claude/commands/ductus/analyze.md`; `.claude/commands/ductus/consolidate.md`; `.claude/commands/ductus/fold.md`; `.claude/commands/ductus/implement.md`; `.claude/commands/ductus/prune.md`; `AGENTS.md`; `README.md`; `docs/analyze.md`; `docs/slash-commands.md`; `framework/bootstrap/configure/*.md`; `framework/commands/analyze.md`; `framework/commands/consolidate.md`; `framework/commands/fold.md`; `framework/commands/help.md`; `framework/commands/implement.md`; `framework/commands/prune.md`; `framework/constitution.md`; `framework/runtime-tools.txt`; `framework/templates/spec/plan.md`; `framework/templates/spec/tasks.md`; `runtime/CHANGELOG.md`; `runtime/src/interpreter/analyze_tally.rs`; `runtime/src/interpreter/mod.rs`; `runtime/src/main.rs`; `runtime/src/mcp/server.rs`; `runtime/src/primitives/analyze_subjects.rs`; `runtime/src/primitives/check_stuck.rs`; `runtime/src/primitives/fetch_archive.rs`; `runtime/src/primitives/mod.rs`; `runtime/src/primitives/prune_plan.rs`; `runtime/src/primitives/prune_tasks.rs`; `runtime/src/primitives/write_analysis.rs`; `runtime/src/schema/primitives.rs`; `runtime/src/schema/registry.rs`; `runtime/src/schema/status.rs`; `runtime/tests/crlf_preservation.rs`; `runtime/tests/golden/analyze-basic.jsonl`; `runtime/tests/golden/implement-basic.jsonl`; `runtime/tests/mcp.rs`; `runtime/tests/walker.rs`; `scripts/gen-help-tables.sh`; `specs/022-deterministic-runtime/data-model.md`; `specs/022-deterministic-runtime/spec.md`; `specs/022-deterministic-runtime/tasks.md`; `specs/041-task-pruning/data-model.md`; `specs/041-task-pruning/plan.md`; `specs/041-task-pruning/spec.md`; `specs/041-task-pruning/tasks.md`; `specs/047-analyze-findings-durability/spec.md`; `specs/047-analyze-findings-durability/tasks.md`; `specs/058-findings-route-at-discovery/review.md`; `specs/058-findings-route-at-discovery/scenarios/*.md`; `specs/062-concurrent-session-targets/spec.md`; `specs/062-concurrent-session-targets/tasks.md` — changed files among them were read at their changed hunks only, directory entries were not walked, and unchanged files were not re-read. This is not a full five-pass review of every path in scope.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- claim: a corrected plan.md sentence began in lowercase — `specs/041-task-pruning/plan.md:92` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
