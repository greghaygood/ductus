---
spec: 041-task-pruning
last-run: 2026-10-01T18:24:24Z
reviewed-against: 0fbcaae60f485190a448bf3d43d74a6b52f736fb
diff-base: 4e745f1589165c4da87222069ae040227dc5a962
must-violations: 0
should-violations: 0
low-confidence: 2
examined: 68
scope: 100
skipped-passes: []
reviewed-digest:
  data-model.md: d1e04eeaa9f74921b26f7c3b5101c70832e7469dfc590f15e787d24cea604535
  scenarios/plan-records-the-design-as-it-stands.md: 617feb2d97e676600074e386b656b15f9d9c632500aaf418d554e2d91364a4eb
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 2f1a33caf5c5c72e94c6e13c6652e0e8465162ffe2d1e9c158a0ce18e60b8c30
blocking: false
dispositions:
  fixed: 32
  routed: 0
  discarded: 23
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

Not blocking. Review of 041's reopen window, natural base 4e745f15 (041's in-progress transition), in the shape the operator chose on 2026-10-01: 041's own work, read in full. 68 of the 100 scope entries were read in full this session by eleven read-only reviewers, all five passes, against the eight backend and cross rule files. The three frontend files apply to no file here. The reviewers worked in parallel from file groups and wrote per-file reports. 58 entries are the paths 041's commits and the release commit touched, plus runtime/tests/mcp.rs, read by ten reviewers. Ten more are the 022 reopen's files in this window, read in full by 022's reviewer: read_spec.rs, the 022 scenario, spec.md and tasks.md, clarify.md, specify.md, the spec template, their two mirrors, and specs/inbox.md. Read only in the hunks the review fixes changed (0fbcaae6): dashboard.rs, append_question.rs, apply_manifest.rs and merge_managed_block.rs. Not read: 24 paths touched in this window only by other specs' commits, each before that spec's own record. These are the 048 review fixes and task 18 (groom.md, target.md and their mirrors, docs/runtime.md, fetch_archive.rs, write_session.rs), 062's session.rs, 052's retire_feature.rs, 050's plan.md, and the review and analysis records 048, 052, 058 and 062 wrote with their own spec files. This record makes no claim they were read. Also not read: the four unmodified glob entries: .claude/commands/ductus/*.md, framework/bootstrap/configure/*.md, scripts/gen-help-tables.sh and 058's scenarios/*.md. Findings: one high-confidence SHOULD, QUAL-CLAIM-001 (the --all reopen gap), was fixed in the run with the --reset stale-reading defect, so the re-check records no SHOULD. Two low-confidence findings remain: CFG-CONST-001 on a bare done literal, and QUAL-CLAIM-001 on an unreported symlinked subject. Two other low-confidence findings, an unquoted git diff path and blob_text reading every lookup error as absent, were fixed. 55 observations: 32 fixed in 0fbcaae6, 23 discarded with their reasons. Every file was read before the fixes. The fixes were re-checked as a diff against each finding, with each new test proven red by mutation (18 mutations, dev profile), not by re-reading whole files. The full local gate is green at 0fbcaae6.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: CFG-CONST-001 — a bare "done" literal where status.rs holds the lifecycle set

- **File**: `runtime/src/primitives/prune_plan.rs:308`
- **Finding**: The reopen check compares status with a bare "done" literal, as prune_tasks.rs:261 and check_artifacts.rs do, while schema/status.rs is the central home of the lifecycle set. Low confidence: no DONE constant exists yet and the literal is the codebase's established convention.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-CLAIM-001 — a symlinked subject is neither digested nor reported unreadable

- **File**: `runtime/src/primitives/analyze_subjects.rs:148-197`
- **Finding**: The subject walk does not follow links and skips anything that is not a regular file, so a symlinked .md under a feature is neither digested nor listed as unreadable, and freshness can read Current with it unexamined. Pre-existing; whether a symlink is a subject is a design call, hence low confidence.
- **Auto-fixable**: no

## Waived findings

*None.*

## Observations

- contract: the --all plan walk dropped a done spec whose plan had nothing to prune, so a reopen from edits made before the run was neither named nor performed when only its tasks.md was reduced (QUAL-CLAIM-001 SHOULD, high confidence, fixed in the run) — `runtime/src/primitives/prune_plan.rs:221` — **fixed**
- contract: --reset reopened a done spec on a stale reading after removing the unchecked boxes the reopen came from; step 9 now decides on a fresh preview after the run's writes — `framework/commands/prune.md:58` — **fixed**
- contract: a prune-plan walk asked to apply with an empty remove list was told to list a section rather than refused for the walk — `runtime/src/primitives/prune_plan.rs:189` — **fixed**
- dead-code: content.is_some() in the apply test could never decide the result — `runtime/src/primitives/prune_plan.rs:280` — **fixed**
- test: the tasks trigger's comment and fence awareness, a section listed twice, prune-plan CRLF, and "reopen-required": false on the wire had no test — `runtime/src/primitives/prune_plan.rs:139` — **fixed**
- doc: prune-plan's and prune-tasks' error lists omitted InvalidPath, MissingFrontmatter, Yaml and the walk's status read — `runtime/src/primitives/prune_plan.rs:165` — **fixed**
- latent: keep-pending dropped a phase governing no task, so a phased file with nothing spent could be rewritten and a note under such a heading lost (AC9, constitution §tasks-phase, the tasks template) — `runtime/src/primitives/prune_tasks.rs:428` — **fixed**
- test: the walk's per-line status assertion used a corpus where every spec was in-progress — `runtime/src/primitives/prune_tasks.rs:743` — **fixed**
- security: unexamined-by-reason keys and the spec, last-run and analyzed-against scalars were written bare, so a reason with a newline or a colon could inject a key or leave a record the next run refuses; the injection test pinned the unreadable form — `runtime/src/primitives/write_analysis.rs:430` — **fixed**
- claim: write-analysis's replaced looked for an analyze: block in spec.md, which the primitive no longer writes — `runtime/src/primitives/write_analysis.rs:120` — **fixed**
- doc: finding_key_of overstated that prune's key and the written key always agree — `runtime/src/primitives/write_analysis.rs:373` — **fixed**
- dead-code: subject_digest's repo fallback for a spec path with no parent could never be taken — `runtime/src/primitives/write_analysis.rs:208` — **fixed**
- doc: stale schema docs: SizeSummary (tasks.md only), PlanSection::decided (exact key), Args feature (under specs/), RecordFreshness::Undeterminable (retired git-diff basis), analyzed_against (analyze record only), derive results' updated, unexamined_by_reason (a map) — `runtime/src/schema/primitives.rs` — **fixed**
- doc: status.rs's consumer list omitted check-artifacts and dashboard, and registry.rs claimed manifest order — `runtime/src/schema/status.rs:3` — **fixed**
- doc: the prune-tasks and prune-plan MCP descriptions omitted all, and the blocking-pool docs omitted the session-lock waits — `runtime/src/mcp/server.rs:694` — **fixed**
- doc: the step-16 tally comment described the interactive host's decision matching, and its test pinned only planned and clarified — `runtime/src/interpreter/analyze_tally.rs:147` — **fixed**
- test: interpreter fixtures labelled write-analysis step 19 after the renumbering made it 20 — `runtime/src/interpreter/mod.rs:1620` — **fixed**
- doc: main.rs said the certificate variables are read lazily on Linux where the gate is non-Apple Unix — `runtime/src/main.rs:682` — **fixed**
- doc: check_stuck's doc broke mid-sentence — `runtime/src/primitives/check_stuck.rs:76` — **fixed**
- doc: FeatureForm said three digits, list_feature_dirs said sorted by name, read_at_head's doc missed the directory case, and section_lines named readers that no longer use it — `runtime/src/primitives/mod.rs:2238` — **fixed**
- contract: UnknownManifestStrategy hard-coded apply-manifest's expected set, contradicting merge-managed-block's marker-style message — `runtime/src/primitives/mod.rs:276` — **fixed**
- latent: parse_affected_files did not skip HTML comments, so the plan template's example table would join a review scope — `runtime/src/primitives/mod.rs:2769` — **fixed**
- test: no test that 999-x sorts before 1000-x — `runtime/src/primitives/mod.rs:3509` — **fixed**
- doc: mcp.rs claimed to invoke every read-only primitive, and walker.rs's doc claimed a check the test does not make — `runtime/tests/mcp.rs:9` — **fixed**
- prose: prune.md omitted the yaml refusal, overstated the plan's size after under --reset, was ambiguous on --force with --all, said --all changes no target, normalized one seam where the runtime normalizes all, under-specified the markdown-only reopen comparison, and left the git diff path unquoted — `framework/commands/prune.md:38` — **fixed**
- prose: analyze.md step 16 restated the design-record definition, gave a false reason for its planned gate, and did not say what an interactive host does on an error; Plan record omitted that a # heading ends a section — `framework/commands/analyze.md:75` — **fixed**
- prose: the constitution omitted the advisory's planned gate, restated prune's mechanics, used design record for two scopes, overstated triage for MUST and SHOULD violations, and left don't backtrack silently empty — `framework/constitution.md:242` — **fixed**
- prose: consolidate.md's prune sentence misparsed, called prune not a state transition, and its destroyed lists omitted analysis.md — `framework/commands/consolidate.md:12` — **fixed**
- prose: docs/slash-commands.md said three commands delete rather than rewrite; README's /prune line omitted --reset and --all, its SHOULD line read as never gating done, and it listed three of nine extension points — `README.md:78` — **fixed**
- stale: AGENTS.md gave MSRV 1.88, 17/19/20 test counts, parity.rs:730, a garbled sentence, a site count pointing at the wrong entry, and session writes predating 062 — `AGENTS.md:143` — **fixed**
- claim: the 0.56.0 CHANGELOG overstated per-process targets and prune's reach, recorded a fix for a race no release shipped, and omitted write-session's refusals, exec's peek and the read-spec change's other consumers — `runtime/CHANGELOG.md:7` — **fixed**
- claim: 041's spec, plan, data model and scenarios carried false or stale claims (the Malformed edge case, the summary's status, circular wording, three comment-held headings in 058, errors writing nothing on a walk, a release step, undated counts, the walk and error tables, plan-records' analyze claim, Q1's superseded naming); task 17 was stale — `specs/041-task-pruning/plan.md:115` — **fixed**
- efficiency: exec analyze's step 16 reads HEAD on a done spec for a reopen-required it ignores, and halts when HEAD cannot be read — `runtime/src/interpreter/mod.rs:1078` — **discarded**: the cost was accepted in the 2026-10-01 decision recorded in prune-reduces-the-spec-directory's Resolved Questions
- reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs:689` — **discarded**: flat registries pinned by set-equality tests, a design AGENTS.md records
- contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs:2864` — **discarded**: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; AGENTS.md records the flattened-halves design
- efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710` — **discarded**: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
- simplicity: check_stuck converts the tasks path to a git path twice — `runtime/src/primitives/check_stuck.rs:91` — **discarded**: the helper takes a project-relative path by design; the cost is one string build
- reuse: three sha256-to-hex spellings across analyze_subjects, prune_plan and fetch_archive — `runtime/src/primitives/analyze_subjects.rs:202` — **discarded**: consolidating them touches fetch-archive (048) for no change in behavior
- reuse: inbox_standing re-implements the HEAD blob read, and prune_plan imports shared helpers from prune_tasks — `runtime/src/primitives/inbox_standing.rs:191` — **discarded**: inbox_standing is the documented raw-repository exception, and where an import comes from changes no behavior
- edge: split_frontmatter takes a later bare-LF --- as the fence when the frontmatter closes with CRLF — `runtime/src/primitives/mod.rs:857` — **discarded**: no such mixed-ending file exists here, and the fix reaches every frontmatter reader for a case not seen
- contract: SkipScanner recognizes only backtick fences, so a ## inside a ~~~ fence is a section — `runtime/src/primitives/mod.rs:1742` — **discarded**: SkipScanner is 022's shared scanner, no ~~~ fence exists in this corpus, and widening it changes every parser built on it
- reuse: the prune-tasks and prune-plan walk skeletons are near-copies — `runtime/src/primitives/prune_plan.rs:210` — **discarded**: the walks differ in what they list and how; a shared skeleton would be generic over both results for one loop
- contract: an applying prune-tasks walk that errors on a later spec returns no record of the reductions it already wrote — `runtime/src/primitives/prune_tasks.rs:176` — **discarded**: each write is atomic and idempotent, so re-running the walk resumes; the data model and plan now say so
- edge: a reset also drops anything above the H1, and its H1 search scans fenced and commented lines — `runtime/src/primitives/prune_tasks.rs:477` — **discarded**: a tasks.md opens with its H1 by construction from the template, and no file here has content above it or a heading only inside a fence
- test: no prune-plan test that a preview on a done spec errors when HEAD cannot be read, or on a malformed decisions list — `runtime/src/primitives/prune_plan.rs:299` — **discarded**: the preview and apply share one reopen computation, whose unreadable-HEAD error is tested through an apply, and the decisions reader's refusal is tested where it lives
- quality: AGENTS.md's 022 routing entry names primitives/mod.rs as if it held every shared parser — `AGENTS.md:69` — **discarded**: the entry's catch-all, anything whose contract other specs rest on, already covers rule_sections.rs and parser/mod.rs
- quality: a remote-tracking ref origin/ci/041-windows-check may survive locally — `AGENTS.md:87` — **discarded**: a local remote-tracking ref, not repository content and not 041's work
- reuse: AGENTS.md entries restate 061's rationale and the recommendations rule, and carry dated measurements of 022's record and the file's size — `AGENTS.md:46` — **discarded**: each records what a rule cost here at a dated moment, which is what the file is for; classification to the constitution is 050's backlog
- prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8` — **discarded**: readability of correct procedure text; the content is accurate after this run's fixes
- claim: AC11 names only the --reset divergence while the design-record heading set has one too — `specs/041-task-pruning/spec.md:56` — **discarded**: AC11 is the tasks.md criterion; the plan's bounded divergence is stated in prune.md, analyze.md and the plan-records scenario
- claim: plan.md narrates the change it makes (gains, is sharpened, is corrected) inside the design record — `specs/041-task-pruning/plan.md:178` — **discarded**: a plan describes the change it makes; these sentences state the design, not a history of revisions to it
- reuse: the design-record headings are listed in 041's plan, data model and plan-records scenario beside the template — `specs/041-task-pruning/data-model.md:231` — **discarded**: the data model is the contract the drift test holds the runtime constant to, and the scenario records the decision that chose the set
- stale: 058's spent tasks cite analyze step 16, and 058's plan keeps a now step 19 history sentence and line citations — `specs/058-findings-route-at-discovery/plan.md:351` — **discarded**: spent tasks are not rewritten, the history sentence was discarded in 048's review (f7c14890), and path:line citations are provenance AGENTS.md says not to sweep
- stale: 060's plan cites analyze.md line numbers that moved, and a 058 scenario's Context describes the pre-fix behavior — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md:87` — **discarded**: path:line provenance, and a Context section records what motivated its scenario
- stale: 041's spent task 2 says segmentation uses iter_phase_ranges — `specs/041-task-pruning/tasks.md:18` — **discarded**: a spent task's record is not rewritten; the plan and data model state the design

## Skipped passes

*None.*

## Unexamined governance

*None.*
