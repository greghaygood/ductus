# 058 — Findings route at discovery Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tasks 1–11 are runtime work, verified through the built binary rather than the MCP tools, which answer with the binary the session started on. Task 23 is the restart point. No pipeline command that consumes a changed primitive runs before it. Tasks 32–38 were routed back into 058 by its own first review (`3b22bc77`) and are done. Tasks 39–42 were routed in by its second review (`78329b7b`), whose four SHOULD violations they fix; 39, 41, and 42 changed the runtime, so a session's MCP server must run a binary built after them before any review or analyze consumes it. Task 42 reopened 022, 047, and 054 (`done → in-progress`) for contract annotations; each has been reviewed (examined: the changed contract), analyzed, and returned to `done`, and 022's review discharged task 40's `append-inbox` sync. Task 43 routed exec analyze's list seeds to 022's scenario `exec-analyze-derives-its-list-seeds` (its task 123), reopening 022, which stays `in-progress` until that task lands. 058's third review (diff-base `5a1d410b`) expired the second review's decisions, matched its discards, and found two SHOULD violations (QUAL-CLAIM-001: the inbox age in a mixed-depth shallow clone, and exec analyze recording steps it never ran as examined); it routed them with its observations to tasks 44 (runtime) and 45 (prose). Task 45 and eight of task 44's twelve items landed in `113820e5`; the other four and 022's task 123 in `61123419`, every new test shown to fail with its behavior removed. Task 46 was routed to a new draft spec, `059-project-in-a-repository-subdirectory`, and task 47 discarded (`a8692cde`). 051's task 26 landed in `5cf68693` (spec 051 was reopened for a fold owed to another tree), and every task in 051 and 022 is checked. 058's fourth review (diff-base `a523457c`) expired the third review's 21 routed decisions, matched its 9 discards, and found no rule violation; it routed one bug to task 48 (exec analyze counting a verdict on an empty rule) and discarded one doc omission that predates 058. Task 48 has landed, every new or changed test shown to fail with its behavior removed. 058's fifth review (diff-base `e52df370`, recorded in `a207b0a8`) expired task 48's routed decision and matched the ten stored discards. It found that exec analyze asks the host about one rule per assessment step, the same rule under both tiers. By operator decision the QUAL-CLAIM-001 half was waived on `analyze_tally.rs` and the double count routed to a new draft spec, `060-exec-analyze-assesses-each-loaded-rule`, which owns both fixes. AC26 was verified and ticked (`1cccad2d`). 058, 022, and 051 reached `done` on 2026-09-26 (`185b6d56`, `3bd4841a`, `0917515c`), each through its review, analysis, and completion gate. The `0.53.0` release in [plan.md](plan.md) §Release carries all three.

## 1. Record shapes: `dispositions` and `decisions`

- [x] Add `Dispositions { fixed, routed, discarded, undispositioned }` and `dispositions: Option<Dispositions>` to `ReviewBlock` and `AnalyzeBlock` in `runtime/src/schema/primitives.rs`
- [x] Add the stored-decision shapes (full entry with a flattened `extra` map, and a `DecisionRef`) per [data-model.md](data-model.md)
- [x] Generalize `read_recorded_waivers` into `read_recorded_list(file, key)` in `runtime/src/primitives/mod.rs`, keeping `read_recorded_waivers` as a wrapper over it
- [x] Tests: absent map versus an all-zero map; a record still carrying `captured-issues` parses; a `decisions:` list round-trips with extras preserved; an unparseable list is an error, not an empty list

- **Done when**: the new types round-trip, a pre-058 `review.md` and `analysis.md` load with `dispositions: None`, and `cargo test --release --locked` passes.

## 2. `process-decisions` primitive

- [x] Create `runtime/src/primitives/process_decisions.rs`, mirroring `process_waivers.rs`: matched, expired, and retained; malformed entries and duplicate keys produce notices and are never pruned
- [x] Register it in `runtime/src/schema/registry.rs`, `runtime/src/mcp/server.rs`, `runtime/src/main.rs`, and `runtime/src/interpreter/mod.rs`; add it to `framework/runtime-tools.txt` and to the permission lists in `framework/bootstrap/configure/claude.md` and `auggie.md`
- [x] Tests: each lifecycle state, restricted-run retention, missing `target` or `reason` as malformed, first duplicate wins, and a record with no `decisions:` yields empty lists

- **Done when**: the primitive answers through `./runtime/target/release/ductus process-decisions`, `runtime/tests/mcp.rs`'s tool-list parity passes, and the new tests pass.

## 3. `write-review`: stop capturing, record dispositions

- [x] Remove `capture_observations`, `inbox_bullet_text`, `observations_captured`, and `inbox_standing` from `write_review.rs` and its result
- [x] Remove `WriteReviewArgs.captured_issues`, the `captured-issues:` frontmatter line, and the `## Captured issues` section with `render_captured`
- [x] Add `disposition` and `decision-key` to `ReviewObservation`, validated per outcome before any write; render each observation's outcome; compute and write `dispositions:`
- [x] Take `expired-decisions` and `new-decisions`; re-render surviving, matched, and new decisions with extras preserved
- [x] Drop `captured-issues` from `invalidate_review.rs`'s nulled scalars
- [x] Replace the observation-capture tests (`observation_is_rendered_and_written_through_to_the_inbox` through `observation_capture_honors_the_configured_specs_root`) with tests that the inbox is never written, that the disposition renders and counts, that an empty-scope run still records dispositions, and that decisions persist and prune

- **Done when**: no path in `write_review.rs` reaches `append_inbox`, a run with observations leaves `specs/inbox.md` byte-identical, `review.md` carries `dispositions:` and `decisions:`, and the file's tests pass.

## 4. `write-analysis`: take findings, new skeleton

- [x] Replace `WriteAnalysisArgs.captured_issues` with `findings`, per the data model
- [x] Compute `dispositions:`, with `undispositioned` equal to the live tier total minus routed and discarded live findings
- [x] Reject a `discarded` outcome on a `hard-fail` or `blocking` finding before any write
- [x] Render live findings with their dispositions under each tier section, a `{n} finding(s) not itemized` line when a tier count exceeds its itemized findings, and `## Fixed in this run` in place of `## Captured issues`; drop the inbox sentences from the Summary and `tier_line`; keep checkbox stripping
- [x] Take `expired-decisions` and `new-decisions`, as `write-review` does
- [x] Rewrite `the_skeleton_is_fixed_and_complete`, `records_captured_issues_beside_advisory`, and `a_divergence_between_advisory_and_captured_is_recorded_not_smoothed` for the new shape; add tests for the unitemized-count rule and the discard rejection

- **Done when**: a call with tier counts and no findings records every live finding as undispositioned, a discard on a blocking finding is refused with nothing written, and the file's tests pass.

## 5. Remove the inbox windows from `compute-review-scope` and `diff-cross-spec`

- [x] Remove `captured_issues` from `ComputeReviewScopeResult` along with `diff_since`, `inbox_in_worktree`, and `inbox_at`; delete the three `captured_issues_*` tests
- [x] Remove `inbox_additions` and `inbox_standing` from `DiffCrossSpecResult` and `diff_cross_spec.rs`, keeping `inbox.md` out of `cross_spec_paths`; update `diff_cross_spec_round_trip` and the file's tests
- [x] Update `runtime/tests/mcp.rs`: `compute_review_scope_returns_structured_scope_via_mcp` and `diff_cross_spec_reports_the_standing_inbox_via_mcp`

- **Done when**: neither result carries an inbox field, an edit to `specs/inbox.md` still appears in no `cross-spec-paths`, and the affected tests pass.

## 6. `dashboard` renders the inbox row

- [x] Add `inbox-standing` to `DashboardResult`, computed by `inbox_standing::standing`
- [x] Render one `Inbox:` line in `render_callouts` in the four states from the data model, on every run
- [x] Correct `InboxStanding.state`'s "three states" doc
- [x] Tests: each of the four states renders, and a clean inbox renders a line rather than nothing

- **Done when**: `./runtime/target/release/ductus dashboard` prints the `Inbox:` line against this repository, and the tests pass.

## 7. `check-review-gate`: two new blocks

- [x] Add `record-predates-dispositions` and `undispositioned-findings` to `ReviewGateBlock`
- [x] Add `disposition_block` after the analyze checks, in the order: predates (review, then analysis), then undispositioned (review, then analysis)
- [x] Correct `run`'s "all six block reasons" doc and the module doc's check list; update the gate-order text in the `check-review-gate` MCP description
- [x] Tests: each block fires; each is outranked by every existing check (a stale analysis outranks a missing map); a map-less record on a `done` spec short-circuits at `already-done`; an all-zero map passes

- **Done when**: the gate's ordering tests cover both new variants and pass.

## 8. `check-artifacts`: `disposition-drift` family

- [x] Add `check_disposition_drift` at `done`, reading both records, `Blocking` per record with `undispositioned > 0`, and nothing for a map-less record
- [x] Update the `ArtifactFinding` family list and correct the family-order doc on `CheckArtifactsResult`
- [x] Tests: fires for either record, silent below `done`, silent for a map-less record, ordered after the analyze-state drift family

- **Done when**: `./runtime/target/release/ductus check-artifacts` reports the family against a fixture `done` spec with an undispositioned record, and the tests pass.

## 9. `append-task`: `dedup-title`

- [x] Add `dedup-title: bool` (default `false`) to `AppendTaskArgs`; when set, a pending section with an identical title returns its number with `appended: false`
- [x] Tests: dedup against a pending section, no dedup against a spent one, default behavior unchanged (`a_slugless_body_still_appends_every_time` still passes)

- **Done when**: two identical disposition-task appends with `dedup-title` produce one task, and the tests pass.

## 10. `validate-frontmatter` reports an unparseable `decisions:` list

- [x] In `validate_record_artifacts`, read each record's `decisions:` through `read_recorded_list` and report a parse failure as a hard failure naming the file and the key
- [x] Tests: a malformed list in `review.md` and in `analysis.md` each produce one finding; an absent list produces none

- **Done when**: the tests pass and a malformed list can no longer read as empty anywhere in the runtime.

## 11. Remaining runtime text and wire tests

- [x] Rewrite the `InboxRoute::Chore` and `InboxRoute::Discard` docs in `runtime/src/schema/extensions.rs`
- [x] Update inbox-naming comments in `runtime/src/interpreter/mod.rs` and `runtime/src/main.rs`, and the tool descriptions for `write-review`, `write-analysis`, `compute-review-scope`, `diff-cross-spec`, and `dashboard` in `runtime/src/mcp/server.rs`
- [x] Update `runtime/tests/walker.rs`'s `assert_observations_threaded` to assert the disposition renders and the inbox is untouched
- [x] Run `cargo fmt`, `cargo clippy --release --locked -- -D warnings`, and `cargo test --release --locked` (goldens are re-blessed in task 22, after the command prose)

- **Done when**: `grep -rn "inbox" runtime/src` returns only `append-inbox`, `remove-inbox-item`, `inbox_standing`, the dashboard row, the `routeInboxItem` extension, and test fixtures, and every non-golden test passes.

## 12. Constitution

- [x] §design-principles: drop "an inbox item" from the second disposition
- [x] §grounding: route an implementation-time assumption to a task or open question
- [x] §bug-handling: a chore found by a run is fixed in that run; a hand-logged chore lives in the inbox until done
- [x] §brownfield-inbox: rewrite as manual capture; replace `#### Automatic issue capture` with `#### Finding dispositions` (three dispositions, scope tiers, disposition tasks, persisted decisions, the gate)
- [x] Frontmatter Schema: replace both `captured-issues` rows with `dispositions` and `decisions`
- [x] §pipeline-boundaries: name `/{project}:log` as the inbox's only producer
- [x] Re-resolve every anchor the rewrite renames, with `check-corpus-links` and `resolve-anchor`
- [x] Keep §brownfield-inbox stating why the standing inbox count is a notice and never a gate (gating it would push the honest choice between a growing list and a silent one toward silence). Four sources cite "the reason §brownfield-inbox gives for capture": `runtime/src/primitives/check_promotion_coverage.rs` (module doc), `scripts/audit/promotion-coverage.sh`, `scripts/audit/README.md` (Family 38), and `framework/commands/audit.md` (Family 38). Keep that reason stated, or update all four in the same change
- [x] Inbound `#automatic-issue-capture` links: the constitution's own §grounding bullet, re-pointed here, and two in `specs/047-analyze-findings-durability/spec.md`, re-pointed in task 24 when 047 is reopened. `check-corpus-links` strips fragments before checking, so a renamed anchor does not block the pre-commit hook in between

- **Done when**: `grep -n "Automatic issue capture\|captured-issues\|captured to the inbox" framework/constitution.md` returns nothing, and every inbound `#automatic-issue-capture` link in the corpus is re-pointed.

## 13. `analyze.md`

- [x] Replace step 16 with host-responsibility steps: process decisions, fix and route (gated per write, no discard for hard-fail or blocking, `--all` grouped by spec with a leave-the-rest choice), and re-run detection when anything was written
- [x] Step 17 passes `findings` (each with tier, family, message, path, `live`, and disposition), `expired-decisions` from process-decisions, and `decided-by` (`git config user.email`); the writer derives new decisions from the routed and discarded live findings itself
- [x] Add `disposition-drift` to the `--fix` triggers in the frontmatter description, Purpose, Scope Boundaries, and step 18; add a markdown-only "Disposition drift" section
- [x] Rewrite §Finding capture (durability) as §Finding dispositions, and the Purpose and Scope Boundaries write lists

- **Done when**: `analyze.md` names no inbox write, and its walk states the detect → decide → re-check → record → render order.

## 14. `review.md`

- [x] Add a process-decisions step and a fix-and-route step before `write-review`; step 9 passes dispositioned observations (each with `decision-key` when process-decisions matched it), `expired-decisions`, and `decided-by`; the writer derives new decisions from the routed and discarded observations itself
- [x] Remove the inbox read from Scope Boundaries and Inputs, the `captured-issues` fields from both frontmatter examples, the `## Captured issues` skeleton entry, §Captured issues, the write-through half of §Observations, and §The inbox row with the `captured` output line
- [x] Replace review.md's "do not invent frontmatter fields to track dispositions" warning with a pointer to `dispositions:`

- **Done when**: `review.md` names no inbox write, and its skeleton matches what `write-review` renders.

## 15. `implement.md`

- [x] Walk step 5: an issue outside the spec becomes a disposition task through `append-task` with `dedup-title`; working a disposition task fixes, routes, or discards it, with a discard's reason written on the task
- [x] Steps 7, 13, and 15, and markdown-only gate step 6: drop `inbox-additions` and the `inbox` row; list the pending disposition tasks
- [x] Remove the inbox write-boundary carve-out from Scope Boundaries and `§brownfield-inbox (Automatic issue capture)` from the Reference line; add the two new gate blocks to the gate-order text

- **Done when**: `implement.md` names no inbox write, and the completion summary lists disposition tasks.

## 16. The other commands

- [x] `amend.md`: the chore guard, its routing row, and its output message fix the chore rather than redirecting to `/{project}:log`
- [x] `groom.md`: correct the step-2 "leave it in the inbox" chore line
- [x] `status.md`: the output includes the `Inbox:` line `dashboard` renders
- [x] `help.md` and `log.md`: describe the inbox as the place for manually captured todos

- **Done when**: none of the five describes automatic capture or a chore parked in the inbox by a run.

## 17. Inbox template, this repository's inbox, and the migration

- [x] `framework/templates/project/inbox.md`: drop the incidental-capture role, the chores-left-in-place line, and forms 2 and 3
- [x] `specs/inbox.md`: the same header change
- [x] Add the `inbox-guidance-refresh` entry to `framework/migrations.toml` (`introduced_in = "0.53.0"`) and `framework/migrations/inbox-guidance-refresh.md`: resolve the spec root, skip an absent inbox, check idempotency first, replace only the leading `<!-- Rules:` comment, and preserve line endings and every item

- **Done when**: running the procedure twice over a fixture inbox with items changes only the header on the first run and nothing on the second, and Family 10 passes.

## 18. The adoption security audit

- [x] `framework/bootstrap/ductus-procedure.md`: remove §Writing findings to the inbox and §Deduplication; §Audit summary and §Security audit summary print each finding with its spec and point to `/{project}:analyze`

- **Done when**: the procedure writes no inbox item, and Family 21 still passes.

## 19. Docs and README

- [x] `docs/analyze.md`: the write list, the record table, the body skeleton, and the closing example
- [x] `docs/slash-commands.md`: `/groom`'s "a chore left alone"; `/status` mentions the inbox row
- [x] `README.md`: the incidental-capture sentence and the log and groom rows

- **Done when**: none of the three states that review, analyze, or implement writes to the inbox.

## 20. `AGENTS.md`

- [x] Rewrite or retire the capture-era entries: "`done` is not the same as *discharged*", the machinery-observation entry, "The inbox is a queue to drain", the two-disposals entry, the in-progress-spec entry, the "Record" spans five destinations entry, and the `captured-issues` data-model entry; keep historical Reason clauses as history
- [x] Record the restart-before-pipeline ordering for this spec's runtime work only if a new learning surfaced, rather than duplicating the existing entry

- **Done when**: no entry describes automatic inbox capture as current behavior.

## 21. Reconcile the spec, plan, and data model

- [x] Re-read [spec.md](spec.md) against the finished prose and runtime; where an implementation decision refined a detail (key shape, section names), correct the spec, plan, or data model so the three agree
- [x] Known deviations to reconcile. `write-review` and `write-analysis` take no `new-decisions` list: they derive new decisions from the dispositioned findings and require `decided-by` when any is new, and a re-matched decision keeps its original stamp. `write-analysis`'s `findings` entries carry no `decision-key`, because the key is always `{family} — {message}`. `ReviewObservation` gained `disposition` and `decision-key`, and `Disposition`/`DispositionOutcome`/`AnalysisFinding`/`AnalysisTier` are the schema names. plan.md (§process-decisions, §write-review, §write-analysis) and data-model.md (the Changed primitive inputs and process-decisions sections) still describe `new-decisions`

- **Done when**: the spec, plan, and data model describe one set of field names, section names, and keys.

## 22. Goldens, mirrors, and the audit

- [x] Re-bless `analyze-basic`, `implement-basic`, and `status-basic` with `BLESS=1 cargo test --release --locked --test parity <test_name>`, reading each diff to confirm it contains only the intended changes. `review-basic` was already re-blessed in `c4b21ff4`, where its only change was the removed `captured-issues` key. The pre-commit hook runs the full `cargo test`, parity included, so a commit that changes a golden's stream re-blesses it in that same commit
- [x] Commit through the pre-commit hook so the `.claude/commands/ductus/` mirrors and help tables regenerate
- [x] Run `cargo test --release --locked`, `scripts/audit/run-all.sh`, and full markdown lint

- **Done when**: all three pass on a clean tree.

## 23. Restart onto the new runtime

- [x] `cargo build --release`, then restart the session so the MCP server and the slash commands load the new binary and the regenerated mirrors
- [x] Confirm through `mcp__ductus__dashboard` that the result carries `inbox-standing`

- **Done when**: an MCP call returns a shape only the new runtime produces.

## 24. Discharge 047 and 008

- [x] Reopen each with `set-status` (`from: done`); correct every claim 058 falsifies (047's inbox-persistence premise and its capture criteria; 008's adoption-audit inbox write); add a blockquote signpost linking to [058](spec.md). Measured 2026-09-25, re-derive before editing: 008's is `spec.md`'s brownfield-audit section (the inbox-item format, its dedup, the `security audit items added` summary line, and the rejected-alternative paragraph that argues for the inbox) plus AC22 and AC26; 047's is `spec.md` (7 hits for the capture-era shapes, the two anchor links among them)
- [x] Re-point 047 `spec.md`'s two `#automatic-issue-capture` links to the constitution section task 12 renamed
- [x] Re-run `/ductus:review` and `/ductus:analyze` on each so both records carry `dispositions:`; return each to `done` through `check-review-gate`

- **Done when**: `check-review-gate` passes for both and returns them to `done`, and 058's gate reports 008 and 047 discharged.

## 25. Discharge 020 and 057

- [x] Reopen; correct `review.md`'s and `analysis.md`'s field and section tables in each data model and every `spec.md` claim naming `captured-issues` or `## Captured issues`; add blockquote signposts. Measured 2026-09-25: 020 has 5 hits in `data-model.md` and 2 in `spec.md`; 057 has 6 in `data-model.md` and 2 in `spec.md`
- [x] Re-run review and analyze on each; return each to `done`

- **Done when**: both are `done` through their gates, and 058's gate reports them discharged.

## 26. Discharge 050

- [x] 050 is already `in-progress`: it was reopened in `c421d0e1` for scenario `report-outcomes-not-edits` (its task 24, done). Skip the reopen, and let this return to `done` cover both
- [x] Annotate `scenarios/findings-route-by-scope.md`'s chore and no-spec-in-progress edge cases as superseded by 058, and add the signpost to `spec.md`. Measured 2026-09-25: 4 capture-era hits in that scenario and 2 in `spec.md`. Task 22 (`f0716c7a`) also re-keyed a Family 38 row in 050's `plan.md` to `AGENTS.md`'s reworded machinery-observation entry, so 050's analysis is stale on `plan.md` as well; the analyze re-run below covers it
- [x] Re-run review and analyze; return to `done`

- **Done when**: 050 is `done` through its gate, and 058's gate reports it discharged.

## 27. Discharge 022

- [x] Reopen; mark `review-observations-write-through`, `the-inbox-row`, and `the-analyze-record-states-what-it-captured` superseded by 058; correct every other 022 scenario and data-model entry naming the inbox window, the standing row, or `captured-issues`; add the `spec.md` signpost. Measured 2026-09-25, excluding the `captured during scenario authoring` boilerplate: `data-model.md` (20 hits), `spec.md` (AC31 and the `captured-issues` sentence near it), and scenarios `the-committed-tree-horizon`, `a-review-states-what-it-read`, `the-promotion-coverage-line` (its inbox-row Resolved Question), `coverage-expansion-primitives`, `review-scope-parse-fidelity`, and `review-runtime-acceleration`. The hits in `primitive-robustness-hardening`, `spec-side-parser-hardening`, and `resolve-references-cli-exec-wiring` record that an item was tracked in the inbox — history, not a falsified contract
- [x] Re-run review, recording the changed contracts as `examined` against the full `scope`, and state in the Summary that the remaining scenarios were not re-read; re-run analyze; return to `done`

- **Done when**: 022 is `done` through its gate with a truthful `examined`, and 058's gate reports every declared impact discharged.

## 28. Final sweep

- [x] Run AC23's repo-wide search for `captured during` bullets, `inbox-additions`, a `Captured issues` section, and an `append-inbox` call outside `/ductus:log`; resolve every hit outside the allowed set
- [x] Walk AC1–AC33 against the tree and mark each verified

- **Done when**: the search returns only allowed hits, and every criterion is checked.

## 29. Disposition out-of-spec finding: 022's groom-command-acceleration says a chore item is left in place

- [x] specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md — its edge case says "A chore item is left in place (no write, no route) exactly as the constitution's inbox rules require", but groom now does a chore in the pass and removes it (groom.md step 3 chore route and step 8), as §brownfield-inbox's "When an item routes to a chore, fix it" rule requires. Stale since that rule changed, not since 058. — fixed

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 30. Disposition out-of-spec finding: 010's implement-offers-the-next-step names captured issues in the per-task summary

- [x] specs/010-agent-autonomy/scenarios/implement-offers-the-next-step.md — its Context says a default-mode /implement per-task summary renders "the task processed, cross-spec impact, captured issues, a reminder to commit". Since 058 the summary lists disposition tasks, not captured issues. The line opens "Today,", narrating the state that motivated the scenario, and it is not one of AC23's shapes. — discarded: the Context narrates the pre-058 state that motivated the scenario rather than stating a live contract or pointing at a removed name, and reopening 010 for one historical sentence is not worth a review and analysis cycle (operator decision, 2026-09-25)

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 31. Remove append-inbox's dead dedup-prefix

- [x] Drop `dedup-prefix` from `AppendInboxArgs` and `deduped` from `AppendInboxResult`, the dedup branch and its helper in `append_inbox.rs`, the MCP tool description, and the tests that exercise them; reword the module and struct docs that named the removed callers
- [x] Record the removal in 058's `data-model.md` Removed fields table; the 0.53.0 `runtime/CHANGELOG.md` section records it at release, since a 0.53.0 heading ahead of the version bump fails audit Family 20
- [x] Reopen 022 and update its append-inbox contract: `data-model.md`'s append-inbox section, and the `scaffolding-primitives` and `append-primitive-marker-normalization` scenarios; re-run review and analyze; return 022 to done

- **Done when**: no code, schema, test, or live doc names append-inbox's dedup-prefix or deduped except as a recorded removal, `cargo test --release --locked` passes, and 022 is done through its gate

## 32. Fix the disposition runtime's correctness defects the 058 review found

- [x] SHOULD QUAL-GROUND-001: reject C0/C1 control characters and the YAML line breaks (U+0085, U+2028, U+2029) in single-line arguments (`validate_single_line`), flatten them in `write_analysis`'s `single_line`, and test that a decision record carrying them is refused or re-reads cleanly
- [x] `exec analyze`: `write-analysis` must not bind another primitive's `findings` from the walker context (`check-orphaned-references` returns one); exec itemizes nothing
- [x] exec `performReview`: a disposition or `decision-key` the host's response supplies is dropped, so an exec run records every observation undispositioned (AC26)
- [x] `decisions::merge`: collapse this run's fresh decisions by key, refusing conflicting outcomes for one key; match a stored decision on key and outcome, as the data model says, keeping its original stamp
- [x] Refuse a blank `decision-key` (fall back to the observation line) and a blank `reviewed-at`/`analyzed-at`, so no writer stores an entry its reader calls malformed
- [x] Make `AnalysisFinding.tier` required, so an untagged finding cannot slip a discard past the hard-fail/blocking refusal
- [x] Share one disposition-to-decision conversion (`Disposition` on the schema) so `write-review` and `write-analysis` apply one newline policy and a stored analyze key matches the host's fired key
- [x] `append-task` `dedup-title`: use `split_numbered_heading` and the shared checkbox grammar, and match a title ending in `#` against the heading the renderer wrote
- [x] `invalidate-review`: drop the stale `dispositions:` map with the other run scalars
- [x] Inbox age: blame the working-tree content (`blame_buffer`), not HEAD, so an uncommitted edit cannot shift bullet lines onto other commits; uncommitted lines carry no date
- [x] The no-file inbox row says the file is unreadable when it exists but cannot be read
- [x] Run `dashboard`'s blame through `dispatch_blocking` on the MCP server, as other history walks are

- **Done when**: each defect has a test that fails before its fix and passes after, `cargo test --release --locked` and `clippy -D warnings` pass, and a decision carrying ESC or U+2028 never leaves a record unreadable

## 33. Remove 058's dead code and stale runtime docs; add the missing tests

- [x] Remove `Dispositions::total`, the inert `--decided-by` CLI flags on `write-review` and `write-analysis`, and `process-decisions`' dead `let … else` branch (one `match entry.to_ref()`); make `process-decisions`' `fired` `#[arg(skip)]` as `process-waivers`' is, and drop `DecisionRecord`'s `#[default]` if nothing needs it
- [x] Drop the leftover `show_untracked_content(true)` in `diff_cross_spec.rs` if `include_untracked` alone surfaces untracked paths (test it)
- [x] Fix `main.rs`'s `append-inbox` help (no dedup), and `dashboard.rs`'s misattached doc comment and `render_callouts` doc (it renders the Inbox line)
- [x] Share the `dispositions:` frontmatter block and the disposition suffix rendering between `write-review` and `write-analysis`
- [x] Add an MCP call-and-assert test for `process-decisions`, and schema round-trip tests pinning the kebab-case wire names of `Dispositions`, `DecisionRef`, `ProcessDecisionsArgs`/`Result`, `AnalysisFinding`, and `ReviewObservation`'s new fields

- **Done when**: no 058 item is dead or documented as doing what it does not, the new tests pass, and `cargo test --release --locked` and `clippy -D warnings` pass

## 34. Correct the prose the 058 review found stale

- [x] 058 `data-model.md`: a caller still passing `dedup-prefix` is refused on MCP and the CLI (unknown arguments are rejected) and ignored only by the exec interpreter
- [x] Reopen 022 (`from: done`): its `data-model.md` gains a `process-decisions` entry and the `disposition-drift` check-artifacts family ("Nine families" is ten), and its `dedup-prefix` sentence says refused, not ignored; re-run review and analyze; return 022 to done
- [x] `docs/slash-commands.md` /analyze section and `README.md`'s Analyze bullet describe dispositions, the three `--fix` triggers, and the undispositioned block
- [x] `AGENTS.md`: the inbox-queue entry's corollary no longer says passes grow the inbox
- [x] `review.md` Blocking semantics: the gate order matches `check-review-gate`
- [x] `implement.md`: a route reopens the target only when it is `done`, and Scope Boundaries admits reading the spec a disposition routes to, as review.md and analyze.md do
- [x] `analyze.md`: an unparseable `decisions:` list asks about each finding until it is repaired (as the spec says); the markdown-only Frontmatter schema section names that hard fail; steps 9/16 say what to do when process-decisions errors
- [x] Constitution: the chore paragraph does not contradict disposition tasks, and the exec sentence says exec matches no stored decision
- [x] `groom.md`: for a finding, the tree's leave-in-inbox branch is not a disposition, and a finding against an unpinned managed rule file is discarded with that reason
- [x] `status.md`: the markdown-only path renders the inbox age as undeterminable without git

- **Done when**: each named passage agrees with the runtime and the constitution, 022 is done through its gate, and full markdownlint and `scripts/audit/run-all.sh` pass

## 35. Implement scenario: analysis-drift-judges-the-record-it-writes

- [x] Implement the behavior described in `scenarios/analysis-drift-judges-the-record-it-writes.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested: a done spec whose analyze run dispositions every live finding reports no disposition drift, and one that leaves a finding undecided reports and, with --fix, reverts from the record it wrote

## 36. Implement scenario: only-unreadable-targets-retain-decisions

- [x] Implement the behavior described in `scenarios/only-unreadable-targets-retain-decisions.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested: analyze.md step 16 and docs/analyze.md set restricted only for could-not-be-read skip reasons, and the class list is named once

## 37. Implement scenario: analyze-findings-match-decisions-by-host-judgment

- [x] Implement the behavior described in `scenarios/analyze-findings-match-decisions-by-host-judgment.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested: AnalysisFinding carries an optional decision-key that write-analysis keys by, and analyze.md step 16 matches findings to stored decisions by host judgment

## 38. Implement scenario: auto-records-disposition-tasks-without-pausing

- [x] Implement the behavior described in `scenarios/auto-records-disposition-tasks-without-pausing.md`

- **Done when**: the scenario's described behavior is correctly implemented: implement.md's --auto gate list excepts appending a disposition task and still pauses to work one

## 39. Fix the second review's record hazards, inbox age, and runtime residue

- [x] SHOULD QUAL-GROUND-001: add U+FFFE and U+FFFF to `is_line_hazard`, so `validate_single_line` refuses them, `flatten_line` flattens them, and `yaml_string` escapes them; test that a decision carrying each is refused or re-reads cleanly
- [x] SHOULD QUAL-GROUND-001: `yaml_string` quotes any value whose unquoted form does not re-parse to the identical string (tab-then-`#` included), replacing the `" #"` test and the type-only re-parse check; test a key and a reason carrying tab-then-`#`
- [x] SHOULD QUAL-GROUND-001: date each working-tree inbox bullet by its text against HEAD's blame (trailing `\r` trimmed), so a CRLF inbox under `core.autocrlf` and an insertion above a removal (libgit2's buffer-blame misplacement) are dated correctly and an uncommitted line carries no date; a shallow clone's boundary hunks carry no date; the blame path is workdir-relative, so a project in a subdirectory of its repo is dated; test each case, and replace `an_uncommitted_bullet_is_not_the_oldest` with a test the old algorithm fails
- [x] `invalidate-review` nulls `captured-issues` again, for the pre-058 records that still carry it; test it
- [x] `performReview`'s response takes an observation's `text` and `path` only, so exec neither validates nor strips a disposition; drop the strip in `interpreter/mod.rs`
- [x] Remove dead code: the unused `Default` on `AnalysisFinding`, `AnalysisTier`, `InboxStanding`, and `InboxState` (and the doc paragraph justifying it), `needs_quote`'s unreachable newline test, and the unreachable fallbacks around `to_ref` (return the defect from it)
- [x] One equality rule for a stored decision: `process-decisions`, `merge`, and the in-run conflict check compare the flattened key and the outcome, and a re-decision is not lost to an ignored duplicate entry; test a hand-edited key with surrounding spaces, and two observations matched to one stored discard with differing reasons
- [x] Correct the stale runtime docs: `ProcessDecisionsArgs.restricted` (could-not-be-read reasons and an unresolved constitution) and `fired` (analyze fires a matched key too), `AppendTaskResult.appended` (`dedup-title` also returns `false`), the append-inbox dedup mentions in `primitives/mod.rs`, `write-analysis`'s `# Errors`, `yaml_string`'s context, `render_extra_field`'s indent, `decisions::render`'s shared-rendering claim, and `dispatch_blocking`'s tool list
- [x] `write-analysis`: a prior `analysis.md` with no frontmatter carries no decisions and is overwritten; one whose frontmatter does not parse is refused, and 058's data model and the writer's doc say so
- [x] Tests: a `decision-key` naming no stored decision is stored under that key and requires `decided-by`; an all-empty decision entry; `dedup-title` against a title differing only in case or whitespace

- **Done when**: each SHOULD's reproduction fails before its fix and passes after (a decision carrying U+FFFF or tab-then-`#` re-reads intact; a committed CRLF inbox under `core.autocrlf` reports its oldest date), no item above is dead or documented as doing what it does not, and `cargo test --release --locked` and `clippy -D warnings` pass

## 40. Correct the prose the second review found stale

- [x] `analyze.md` step 17, the constitution's Three dispositions bullet, and 058's own `spec.md` §Three dispositions: a route to a spec that is `done`, the spec in hand included, reopens it
- [x] 058 `data-model.md` and `plan.md`: disposition-drift judges `review.md` only; `restricted` is set only by a could-not-be-read skip reason or an unresolved constitution; the malformed and duplicate entries' survival names the two pruning cases the review discarded (an all-empty entry, a duplicate whose key expires or is re-decided); a decision is new unless its key and outcome are stored; `inbox_standing.rs` is not doc-only; and `invalidate-review` nulls `captured-issues` for the records that still carry it (task 39)
- [x] 022 `data-model.md`'s `append-inbox` entry: bullet scanning counts and does not dedup; a mechanical sync, after which 022's review record is refreshed (no reopen)
- [x] `docs/analyze.md` §decisions points to the reason-class table instead of restating it, and names the unresolved-constitution case
- [x] `framework/bootstrap/ductus-procedure.md`: a gap returns as a finding that is fixed or routed, and a SHOULD-tier gap may also be discarded
- [x] `review.md` markdown-only step 1.3: observations supplied to an empty-scope run are dispositioned before the record is written

- **Done when**: each named passage agrees with the runtime, the scenarios, and the other commands, 022's review is current, and full markdownlint and `scripts/audit/run-all.sh` pass

## 41. Exec analyze records the tier counts it detected

- [x] SHOULD QUAL-CLAIM-001: as each analyze step dispatches, the exec walker tallies its results into the tier counts, by the tiering the step states, and its skipped targets into `unexamined-by-reason`, and binds them to `write-analysis`; result keys collide in the walker context, so the tally is taken at dispatch
- [x] The exec walker binds `validate-frontmatter` (and the other spec-reading steps, `resolve-anchor` and `check-rule-ids`) to the spec file when the session `path` is the spec directory, as `write-session` writes it, and `resolve-anchor` to the project constitution, so `ductus exec analyze` gets past them on a real session; the seeds no session carries are task 43
- [x] Correct the comment on `interpreter/mod.rs`'s `write-analysis` binding and `plan.md`'s claim that the existing binding supplies the counts
- [x] Tests: an exec run over a fixture with live blocking and advisory findings records their counts, `blocking: true`, and `undispositioned` equal to the live total; a fixture whose session `path` is the spec directory

- **Done when**: the reproduction (a `planned` spec with no `plan.md` or `tasks.md`, for which exec wrote 0/0/0) records what detection found — with `analyze-basic`'s two scripted assessments, 3 blocking, 1 advisory, `blocking: true`, and 4 undispositioned; exec analyze completes with a directory session `path` when the other seeds are present; any re-blessed golden diff is limited to what the tally changes; and `cargo test --release --locked` and `clippy -D warnings` pass

## 42. Implement scenario: analyze-state-drift-judges-the-record-it-writes

- [x] Implement the behavior described in `scenarios/analyze-state-drift-judges-the-record-it-writes.md`
- [x] 022's `data-model.md` `check-artifacts` registry records the change, through a 022 reopen as task 34 did

- **Done when**: the scenario's described behavior is correctly implemented and tested: `check-artifacts` does not judge `analysis.md` during detection, a `done` spec whose run fixes its blocking finding records `blocking: false` and reports no drift, and one left blocking reports drift from the record it wrote

## 43. Disposition out-of-spec finding: exec analyze on a bare session stops at the arguments no session carries

- [x] `runtime/src/main.rs` — `ductus exec analyze` seeds its context from the session file and string `key=value` arguments. A session written by `write-session` carries `feature`, `path`, and `set-at` alone, so once task 41 binds the spec file the walk stops at step 5 (`missing field rule-files`), and `lint-markdown`'s `paths` and `write-analysis`'s `analyzed-at` and `analyzed-against` would follow. A string seed cannot supply a list. The parity fixtures seed them all in their session file. Predates 058; exec's seed contract belongs to 022. **Routed** to 022's scenario `exec-analyze-derives-its-list-seeds` and its task 123, reopening 022 (`done → in-progress`).

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 44. Fix the third review's runtime findings

- [x] SHOULD QUAL-CLAIM-001: `inbox_standing`'s age is undeterminable when any surviving bullet maps to a line behind a shallow cut, rather than the oldest date after the cut; test a mixed-depth shallow clone (origin 2020 and 2022 bullets, `--depth 2`)
- [x] SHOULD QUAL-CLAIM-001: exec analyze records what it did not examine — the host-responsibility detection steps it skips (13–15), a `check-rule-ids` that read zero rule files while citations exist, and `check-orphaned-references`' skipped referrers — under `unexamined-by-reason`, each reason classified in `analyze.md`'s Unexamined targets; test each
- [x] The exec tally takes an assessment's tier case-insensitively, as validation accepts it, and does not drop a failed assessment that carries no finding; test the walker path that feeds `assessSpecQuality` responses into the tally
- [x] `remove-inbox-item` and `append-inbox` apply one single-line rule, so any bullet `append-inbox` writes can be removed; test a U+2028 round trip
- [x] `process-decisions` reads a frontmatter-less `analysis.md` as holding no decisions, as `write-analysis` does; an unclosed frontmatter is named as unclosed, not missing, and a test pins `write-analysis`' refusal of it
- [x] One key, one disposition: two findings sharing a key are refused whenever their outcomes differ, not only routed against discarded; test routed with undispositioned
- [x] `dedup-title` matches a pending task on its title and body, so two out-of-spec findings sharing a summary stay two tasks; test it
- [x] Inbox age: repeated bullet texts pair so that an uncommitted removal of one duplicate leaves the survivor its own date; test it
- [x] `compute-review-scope` resolves the spec's history path against the git work tree, so a project in a subdirectory of its repo gets its diff base (predates 058); test it
- [x] One constitution resolver (`.ductus/` then `framework/`) serves the exec analyze binding and `writeCode`'s excerpts
- [x] Cleanups: drop `PassObservation`'s unused `Default` and `Eq`; project the validated `PerformReviewResponse` once instead of re-parsing each observation; use `UNBLOCKING_STATUSES` instead of the hand-copied status list in `analyze_tally.rs`; one route/discard companion check shared by `write-review` and `write-analysis`
- [x] Docs: the `# Errors` sections of `write-review`, `write-analysis`, and `process-decisions`; the malformed-entry claims in `decisions.rs` and `read_recorded_list`; `pass_observations`' doc placement above `criterion_verified_met`; `tests/mcp.rs`' "three fields" doc; `already_done_block`'s split doc; `check_artifacts.rs`' `analysis.md` paragraph moved after the family list; `analyze_subjects.rs`' list of `analysis.md` readers; `AppendTaskResult.task_number` on a dedup return

- **Done when**: each SHOULD's reproduction fails before its fix and passes after (a depth-2 clone holding a pre-cut bullet reads age undeterminable; exec analyze with `rule-files = []` and a citation, and any exec run, records what it did not examine), each bug above has a test that fails before its fix, no item is dead or documented as doing what it does not, and `cargo test --release --locked` and `clippy -D warnings` pass

## 45. Correct the prose the third review found stale

- [x] `review.md`'s Decisions persist and Malformed and duplicate waivers passages state what the runtime does: an all-empty entry is dropped on re-render, and a duplicate goes when its key expires or is re-decided, for decisions and waivers alike
- [x] 058 `plan.md`: the `invalidate_review.rs` Affected Files row, the reopened-spec count (seven, with 054), the disposition-drift decision's account of `check_analyze_drift` and its citations, and `write-review`'s step number (11)
- [x] 058 `data-model.md`'s stale line citations (`AnalyzeBlock`, `InboxStanding`) and `spec.md`'s "analyze's dedup key" edge case
- [x] The exec bullets of `analysis-drift-judges-the-record-it-writes` and `analyze-state-drift-judges-the-record-it-writes`: exec records the drift in the new record and does not report it, since step 20 is host responsibility
- [x] Family 37's set is a subset of what the CI template's analyze-record check exempts, not its exact population: `audit.md`, `analyze-record-backlog.sh`, `scripts/audit/README.md`, `docs/analyze.md`
- [x] `analyze.md`'s markdown-only list of checks names analyze state drift; `docs/analyze.md` points to `analyze.md` for the reason classes instead of carrying its own table
- [x] `status.md`'s shallow-clone wording follows task 44's inbox-age fix
- [x] `docs/slash-commands.md` and `README.md`'s `/review` entries describe observation dispositions and the `done` hold on an undispositioned one (AC19)

- **Done when**: each named passage agrees with the runtime, the scenarios, and the other commands, AC19 holds as written, and full markdownlint and `scripts/audit/run-all.sh` pass

## 46. Disposition out-of-spec finding: four primitives look up spec history relative to the project, not the git work tree

- [x] `runtime/src/primitives/{check_stuck,check_artifacts,diff_cross_spec,derive_boundary}.rs` — each joins spec-root-relative paths into git tree lookups after `Repository::discover`, so in a project that lives in a subdirectory of its repository it finds no history for the spec (predates 058). Task 44 fixed the same defect in `compute-review-scope` with the shared `workdir_prefix`. **Routed** to a new spec, `059-project-in-a-repository-subdirectory`, created `draft` through `/{project}:specify`'s procedure (operator decision, 2026-09-26)

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 47. Disposition out-of-spec finding: 020's waiver-expiry scenario says duplicate waivers are not auto-pruned

- [x] `specs/020-code-review/scenarios/waiver-expiry.md` — a duplicate waiver goes when its `(rule, file)` pair expires, as it did before 058 (`is_expired` compares rule and file only); nothing prunes a duplicate for being one, so the sentence can be read as true. 020 is `done`, so a sync reopens it — discarded: the sentence still holds, since nothing prunes a duplicate waiver for being a duplicate and it leaves only when its `(rule, file)` key expires, as every waiver does; reopening 020 would change no claim it makes (operator decision, 2026-09-26)

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 48. Fix the fourth review's runtime finding: exec analyze counts an assessment of no rule

- [x] Bug (from the fourth review): with no rule file read, the exec walker sends steps 11–12 an `assessSpecQuality` request whose rule has an empty `id` and `verification`, and `analyze_tally` counts the host's verdict on it. On `analyze-basic` with `framework/rules` removed, the record reads `blocking-findings: 1`, `advisory: 1`, `blocking: true` beside `rule-citations-not-checked: 1`
- [x] An `assessSpecQuality` step whose request resolves no rule is recorded unexamined, under a reason `analyze.md`'s Unexamined targets and 022's data model classify, and never counted in a tier
- [x] `analyze_with_no_rule_directory_records_its_citations_unexamined` asserts the corrected counts, and each new test is shown to fail with its behavior removed

- **Done when**: exec analyze with no rule directory records no blocking or advisory count from steps 11–12 and names them unexamined; every new or changed test fails with its behavior removed; `cargo test --release --locked` and `clippy -D warnings` pass

## 49. The analyze record reads the re-check: a finding gone from it is fixed, whatever confirmed write removed it (groomed from the inbox)

- [x] Spec body, §Order within a run: state that `analysis.md` records the re-check's outcome rather than the kind of write — a finding the re-check no longer produces is `fixed` (`live: false`) whether a chore or a confirmed route removed it, and `routed` marks a finding still firing until its routed work lands, the only state a stored decision is for
- [x] `framework/commands/analyze.md`: step 18 and the Fixed and Routed bullets under Finding dispositions say the same, naming `write-analysis`' refusal of any other outcome for a finding gone from the re-check
- [x] `framework/constitution.md` §brownfield-inbox, Finding dispositions: the *Detect, decide, re-check, record* bullet says the same for the analyze record
- [x] Sweep for claims that a body edit resolving a finding in the run is recorded as routed; lint the changed markdown, run the generators, `scripts/audit/run-all.sh`, and `cargo test` from `runtime/` (the parity tests read `framework/commands/analyze.md`)

- **Done when**: 058's body, analyze.md and the constitution agree with `write_analysis.rs:294`, and the local gate passes
