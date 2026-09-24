# 058 — Findings route at discovery Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tasks 1–11 are runtime work, verified through the built binary rather than the MCP tools, which answer with the binary the session started on. Task 23 is the restart point. No pipeline command that consumes a changed primitive runs before it.

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

- [ ] §design-principles: drop "an inbox item" from the second disposition
- [ ] §grounding: route an implementation-time assumption to a task or open question
- [ ] §bug-handling: a chore found by a run is fixed in that run; a hand-logged chore lives in the inbox until done
- [ ] §brownfield-inbox: rewrite as manual capture; replace `#### Automatic issue capture` with `#### Finding dispositions` (three dispositions, scope tiers, disposition tasks, persisted decisions, the gate)
- [ ] Frontmatter Schema: replace both `captured-issues` rows with `dispositions` and `decisions`
- [ ] §pipeline-boundaries: name `/{project}:log` as the inbox's only producer
- [ ] Re-resolve every anchor the rewrite renames, with `check-corpus-links` and `resolve-anchor`
- [ ] Keep §brownfield-inbox stating why the standing inbox count is a notice and never a gate (gating it would push the honest choice between a growing list and a silent one toward silence). Four sources cite "the reason §brownfield-inbox gives for capture": `runtime/src/primitives/check_promotion_coverage.rs` (module doc), `scripts/audit/promotion-coverage.sh`, `scripts/audit/README.md` (Family 38), and `framework/commands/audit.md` (Family 38). Keep that reason stated, or update all four in the same change
- [ ] Inbound `#automatic-issue-capture` links: the constitution's own §grounding bullet, re-pointed here, and two in `specs/047-analyze-findings-durability/spec.md`, re-pointed in task 24 when 047 is reopened. `check-corpus-links` strips fragments before checking, so a renamed anchor does not block the pre-commit hook in between

- **Done when**: `grep -n "Automatic issue capture\|captured-issues\|captured to the inbox" framework/constitution.md` returns nothing, and every inbound `#automatic-issue-capture` link in the corpus is re-pointed.

## 13. `analyze.md`

- [ ] Replace step 16 with host-responsibility steps: process decisions, fix and route (gated per write, no discard for hard-fail or blocking, `--all` grouped by spec with a leave-the-rest choice), and re-run detection when anything was written
- [ ] Step 17 passes `findings` (each with tier, family, message, path, `live`, and disposition), `expired-decisions` from process-decisions, and `decided-by` (`git config user.email`); the writer derives new decisions from the routed and discarded live findings itself
- [ ] Add `disposition-drift` to the `--fix` triggers in the frontmatter description, Purpose, Scope Boundaries, and step 18; add a markdown-only "Disposition drift" section
- [ ] Rewrite §Finding capture (durability) as §Finding dispositions, and the Purpose and Scope Boundaries write lists

- **Done when**: `analyze.md` names no inbox write, and its walk states the detect → decide → re-check → record → render order.

## 14. `review.md`

- [ ] Add a process-decisions step and a fix-and-route step before `write-review`; step 9 passes dispositioned observations (each with `decision-key` when process-decisions matched it), `expired-decisions`, and `decided-by`; the writer derives new decisions from the routed and discarded observations itself
- [ ] Remove the inbox read from Scope Boundaries and Inputs, the `captured-issues` fields from both frontmatter examples, the `## Captured issues` skeleton entry, §Captured issues, the write-through half of §Observations, and §The inbox row with the `captured` output line
- [ ] Replace review.md's "do not invent frontmatter fields to track dispositions" warning with a pointer to `dispositions:`

- **Done when**: `review.md` names no inbox write, and its skeleton matches what `write-review` renders.

## 15. `implement.md`

- [ ] Walk step 5: an issue outside the spec becomes a disposition task through `append-task` with `dedup-title`; working a disposition task fixes, routes, or discards it, with a discard's reason written on the task
- [ ] Steps 7, 13, and 15, and markdown-only gate step 6: drop `inbox-additions` and the `inbox` row; list the pending disposition tasks
- [ ] Remove the inbox write-boundary carve-out from Scope Boundaries and `§brownfield-inbox (Automatic issue capture)` from the Reference line; add the two new gate blocks to the gate-order text

- **Done when**: `implement.md` names no inbox write, and the completion summary lists disposition tasks.

## 16. The other commands

- [ ] `amend.md`: the chore guard, its routing row, and its output message fix the chore rather than redirecting to `/{project}:log`
- [ ] `groom.md`: correct the step-2 "leave it in the inbox" chore line
- [ ] `status.md`: the output includes the `Inbox:` line `dashboard` renders
- [ ] `help.md` and `log.md`: describe the inbox as the place for manually captured todos

- **Done when**: none of the five describes automatic capture or a chore parked in the inbox by a run.

## 17. Inbox template, this repository's inbox, and the migration

- [ ] `framework/templates/project/inbox.md`: drop the incidental-capture role, the chores-left-in-place line, and forms 2 and 3
- [ ] `specs/inbox.md`: the same header change
- [ ] Add the `inbox-guidance-refresh` entry to `framework/migrations.toml` (`introduced_in = "0.53.0"`) and `framework/migrations/inbox-guidance-refresh.md`: resolve the spec root, skip an absent inbox, check idempotency first, replace only the leading `<!-- Rules:` comment, and preserve line endings and every item

- **Done when**: running the procedure twice over a fixture inbox with items changes only the header on the first run and nothing on the second, and Family 10 passes.

## 18. The adoption security audit

- [ ] `framework/bootstrap/ductus-procedure.md`: remove §Writing findings to the inbox and §Deduplication; §Audit summary and §Security audit summary print each finding with its spec and point to `/{project}:analyze`

- **Done when**: the procedure writes no inbox item, and Family 21 still passes.

## 19. Docs and README

- [ ] `docs/analyze.md`: the write list, the record table, the body skeleton, and the closing example
- [ ] `docs/slash-commands.md`: `/groom`'s "a chore left alone"; `/status` mentions the inbox row
- [ ] `README.md`: the incidental-capture sentence and the log and groom rows

- **Done when**: none of the three states that review, analyze, or implement writes to the inbox.

## 20. `AGENTS.md`

- [ ] Rewrite or retire the capture-era entries: "`done` is not the same as *discharged*", the machinery-observation entry, "The inbox is a queue to drain", the two-disposals entry, the in-progress-spec entry, the "Record" spans five destinations entry, and the `captured-issues` data-model entry; keep historical Reason clauses as history
- [ ] Record the restart-before-pipeline ordering for this spec's runtime work only if a new learning surfaced, rather than duplicating the existing entry

- **Done when**: no entry describes automatic inbox capture as current behavior.

## 21. Reconcile the spec, plan, and data model

- [ ] Re-read [spec.md](spec.md) against the finished prose and runtime; where an implementation decision refined a detail (key shape, section names), correct the spec, plan, or data model so the three agree
- [ ] Known deviations to reconcile. `write-review` and `write-analysis` take no `new-decisions` list: they derive new decisions from the dispositioned findings and require `decided-by` when any is new, and a re-matched decision keeps its original stamp. `write-analysis`'s `findings` entries carry no `decision-key`, because the key is always `{family} — {message}`. `ReviewObservation` gained `disposition` and `decision-key`, and `Disposition`/`DispositionOutcome`/`AnalysisFinding`/`AnalysisTier` are the schema names. plan.md (§process-decisions, §write-review, §write-analysis) and data-model.md (the Changed primitive inputs and process-decisions sections) still describe `new-decisions`

- **Done when**: the spec, plan, and data model describe one set of field names, section names, and keys.

## 22. Goldens, mirrors, and the audit

- [ ] Re-bless `analyze-basic`, `implement-basic`, and `status-basic` with `BLESS=1 cargo test --release --locked --test parity <test_name>`, reading each diff to confirm it contains only the intended changes. `review-basic` was already re-blessed in `c4b21ff4`, where its only change was the removed `captured-issues` key. The pre-commit hook runs the full `cargo test`, parity included, so a commit that changes a golden's stream re-blesses it in that same commit
- [ ] Commit through the pre-commit hook so the `.claude/commands/ductus/` mirrors and help tables regenerate
- [ ] Run `cargo test --release --locked`, `scripts/audit/run-all.sh`, and full markdown lint

- **Done when**: all three pass on a clean tree.

## 23. Restart onto the new runtime

- [ ] `cargo build --release`, then restart the session so the MCP server and the slash commands load the new binary and the regenerated mirrors
- [ ] Confirm through `mcp__ductus__dashboard` that the result carries `inbox-standing`

- **Done when**: an MCP call returns a shape only the new runtime produces.

## 24. Discharge 047 and 008

- [ ] Reopen each with `set-status` (`from: done`); correct every claim 058 falsifies (047's inbox-persistence premise and its capture criteria; 008's adoption-audit inbox write); add a blockquote signpost linking to [058](spec.md)
- [ ] Re-point 047 `spec.md`'s two `#automatic-issue-capture` links to the constitution section task 12 renamed
- [ ] Re-run `/ductus:review` and `/ductus:analyze` on each so both records carry `dispositions:`; return each to `done` through `check-review-gate`

- **Done when**: `check-review-gate` passes for both and returns them to `done`, and 058's gate reports 008 and 047 discharged.

## 25. Discharge 020 and 057

- [ ] Reopen; correct `review.md`'s and `analysis.md`'s field and section tables in each data model and every `spec.md` claim naming `captured-issues` or `## Captured issues`; add blockquote signposts
- [ ] Re-run review and analyze on each; return each to `done`

- **Done when**: both are `done` through their gates, and 058's gate reports them discharged.

## 26. Discharge 050

- [ ] 050 is already `in-progress`: it was reopened in `c421d0e1` for scenario `report-outcomes-not-edits` (its task 24, done). Skip the reopen, and let this return to `done` cover both
- [ ] Annotate `scenarios/findings-route-by-scope.md`'s chore and no-spec-in-progress edge cases as superseded by 058, and add the signpost to `spec.md`
- [ ] Re-run review and analyze; return to `done`

- **Done when**: 050 is `done` through its gate, and 058's gate reports it discharged.

## 27. Discharge 022

- [ ] Reopen; mark `review-observations-write-through`, `the-inbox-row`, and `the-analyze-record-states-what-it-captured` superseded by 058; correct every other 022 scenario and data-model entry naming the inbox window, the standing row, or `captured-issues`; add the `spec.md` signpost
- [ ] Re-run review, recording the changed contracts as `examined` against the full `scope`, and state in the Summary that the remaining scenarios were not re-read; re-run analyze; return to `done`

- **Done when**: 022 is `done` through its gate with a truthful `examined`, and 058's gate reports every declared impact discharged.

## 28. Final sweep

- [ ] Run AC23's repo-wide search for `captured during` bullets, `inbox-additions`, a `Captured issues` section, and an `append-inbox` call outside `/ductus:log`; resolve every hit outside the allowed set
- [ ] Walk AC1–AC33 against the tree and mark each verified

- **Done when**: the search returns only allowed hits, and every criterion is checked.
