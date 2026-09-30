---
spec: 048-govern-acquired-runtime
last-run: 2026-09-30T16:32:54Z
reviewed-against: 015ecacdede49ad179ed22a65472fdb45767212a
diff-base: ce1cfc1e58f4e43cbd199c1e8a14d0d2c28210c5
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 73
scope: 97
skipped-passes: []
reviewed-digest:
  data-model.md: aaabca49a7dd3b0e8c5a3fe311f77ba06c98b5f3d106cbc19c3c846d9fcd9a33
  scenarios/fetch-archive-reads-its-proxy-once.md: 1e1d9506af9ed4186032a1e205ff4eabf4045dacb0b5defab5023abbc95b0133
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 0c5984d011f5d35b3685f05bc083e01d9aaa7bac4a31c0949e1d5bcd1f00627b
  scenarios/state-a-version-checks-the-pin.md: 40a4cbaf5dab4b5ee1127c4e808f70bbd08433415e952011ceb7c419a5e24e23
  scenarios/state-b-continues-in-session.md: b4d9b7da7ad0d9486268cff26ec375aa64d35a9639a5828a03cdde92da756ddd
blocking: false
dispositions:
  fixed: 20
  routed: 6
  discarded: 16
  undispositioned: 0
decisions:
  - key: "contract: prune-plan's apply writes plan.md before its reopen check can fail, against the plan's 'each error writes nothing' — `runtime/src/primitives/prune_plan.rs:264`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: prune-tasks' all + apply walk rewrites tasks.md before its status read can fail — `runtime/src/primitives/prune_tasks.rs:177`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: prune-plan rebuilds the stored-decision key inline beside write_analysis::finding_key — `runtime/src/primitives/prune_plan.rs:305`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: 041's spec, plan, data-model, scenarios, prune.md, analyze.md and the constitution's new text disagree on the reopen's timing, AC19's trigger, what prune touches, contributor knowledge's destinations and the design-record list's home — `specs/041-task-pruning/spec.md`"
    outcome: routed
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 052's review.md calls its 7 unread files unchanged since the full review; all 7 changed — `specs/052-spec-supersession-and-consolidation/review.md:32`"
    outcome: routed
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "claim: 052's spec says prune destroys tasks.md; since 041 it also removes plan sections — `specs/052-spec-supersession-and-consolidation/spec.md:65`"
    outcome: routed
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: common/mod.rs's stated reason for concurrent_sessions.rs's own spawn helper is inexact — `runtime/tests/common/mod.rs:22`"
    outcome: discarded
    reason: that file removes the same variables and then sets each child's identity; converging it on ductus_command is a refactor with no behavior at stake
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: a redundant `||` in a crlf assertion — `runtime/tests/crlf_preservation.rs:210`"
    outcome: discarded
    reason: cosmetic, and predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the crlf prune-tasks cases do not assert that the rewrite happened — `runtime/tests/crlf_preservation.rs:196`"
    outcome: discarded
    reason: their inputs do trigger the rewrite, so a CRLF regression fails them; predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the walker prune test's doc says the preview reports a missing plan without asserting it — `runtime/tests/walker.rs:829`"
    outcome: discarded
    reason: prune_plan's a_missing_plan_is_reported_not_an_error pins that; the walker test's subject is the walk continuing
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18`"
    outcome: discarded
    reason: predates the window; consolidating test helpers is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "dead code: dispatch_primitive's gate-confirm arm is unreachable in production — `runtime/src/interpreter/mod.rs:1083`"
    outcome: discarded
    reason: the registry-pin test requires every registry primitive to dispatch; removing the arm means exempting gate-confirm from that pin, a design change outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: the exec session report joins feature and scenario by hand beside SessionTarget::display — `runtime/src/interpreter/mod.rs:875`"
    outcome: discarded
    reason: a SessionPeer is not a SessionTarget, and a shared helper for two three-line joins would be premature
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "robustness: blob_text reads every tree lookup error as an absent file — `runtime/src/primitives/mod.rs:1488`"
    outcome: discarded
    reason: only a corrupt object store reaches it, and the repository's other reads would fail first
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: fetch-archive's sha256_hex hand-rolls the encoding analyze_subjects::hex shares, and hex lives in the analyze-record module — `runtime/src/primitives/fetch_archive.rs:655`"
    outcome: discarded
    reason: moving hex into mod.rs first is the prerequisite, a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "naming: check_stuck mixes project-root and work-tree paths, and its _rel names mean work-tree paths — `runtime/src/primitives/check_stuck.rs:48`"
    outcome: discarded
    reason: both are correct as written and pinned by the subdirectory test; a read_at_commit sibling is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: InvalidSlug and InvalidPath print a rejected value raw, a newline included — `runtime/src/primitives/mod.rs:332`"
    outcome: discarded
    reason: the value reaches callers inside a JSON error envelope, which escapes it
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: a consolidation given only path or scenario is refused naming feature — `runtime/src/primitives/retarget_sessions.rs:146`"
    outcome: discarded
    reason: the refusal is correct and names an argument the call needs; which one it names first is cosmetic
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's review.md cites plan.md line 274 for 275 and says no 022 file changed — `specs/058-findings-route-at-discovery/review.md:83`"
    outcome: discarded
    reason: a review record is pinned to its reviewed-against sha and is regenerated by 058's next review
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's plan calls its write-analysis step 'now step 19' — `specs/058-findings-route-at-discovery/plan.md:347`"
    outcome: discarded
    reason: it records 058's own renumbering at the time, correct as history
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "inbox: an item's fix list predates 041's prune — `specs/inbox.md:31`"
    outcome: discarded
    reason: /groom re-reads each item when it routes it; the inbox is not a live artifact
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "readability: three nested em-dashes in §concurrent-features' adoption sentence — `framework/constitution.md:782`"
    outcome: discarded
    reason: readability only; the sentence's meaning is intact
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
---

# Review — 048-govern-acquired-runtime

## Summary

Not blocking. A deliberate partial review over 048's reopen window (diff base `ce1cfc1e`, the commit before 062's review reopened 048 at `cb54dc8f`), in the shape the operator chose: every file modified since that base, not only 048's own. The five passes read 73 of the 82 modified files in full, split across reviewers. 048's own change is task 18: `fetch-archive` captures the proxy variables at startup and applies them as explicit proxies, loads the Linux certificate roots once per process, and states the operator's proxy as the trust boundary. It found no rule violation. `runtime` run for `c4ae7c5c` passed on ubuntu, macOS and Windows, with the new proxy test running through the Linux root path. Across the window, two SHOULD findings (`QUAL-TEST-001`) and eighteen other observations were fixed in the run (`628a89db`); six were routed (041 task 19, 052 task 1, `015ecacd`); sixteen were discarded with reasons; one `QUAL-CLAIM-001` finding is recorded at low confidence. The nine modified files not read are the eight generated mirrors under `.claude/commands/ductus/` — `analyze.md`, `configure.md`, `consolidate.md`, `fold.md`, `groom.md`, `help.md`, `prune.md`, `target.md` — which `scripts/gen-claude-commands.sh` reports in sync with their sources, all read, and `runtime/Cargo.lock`, of which only its diff was read. `scope: 97` counts scope entries; the other fifteen are plan-only entries unchanged in the window, none read this pass: `.ductus/config.toml`, `.github/workflows/framework-checks.yml`, `.github/workflows/runtime-acquisition.yml`, `.github/workflows/runtime-release.yml`, `.gitignore`, `.mcp.json`, `framework/bootstrap/ductus.md`, `framework/commands/*.md` (whose files changed in the window were read through their own entries), `framework/migrations.toml`, `framework/migrations/runtime-store-path.md`, `scripts/audit/run-all.sh`, `scripts/audit/version-agreement.sh`, `specs/021-runtime-boundary/spec.md`, `specs/029-bootstrap-runtime-autowire/spec.md` and `version`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-CLAIM-001 — every ProjectRepository::discover error reads as 'no repository'

- **File**: `runtime/src/primitives/mod.rs:2435-2548`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both.
- **Finding**: tracked_project_paths, list_untracked_specs and list_staged_specs map any discover failure to the no-repository answer, so a repository that exists but cannot be opened (permission denied, a corrupt .git, libgit2's owner validation) falls back to the worktree walk or returns empty. Low confidence: the trigger depends on the environment, and predates this window.
- **Auto-fixable**: no
- **Suggested fix**: Treat only a not-found discover as no repository, and return every other error as PrimitiveError::Git.

## Waived findings

*None.*

## Observations

- QUAL-TEST-001: a_subject_the_walk_cannot_reach_is_recorded carried cfg(unix) inside its body, so on Windows it asserted nothing — `runtime/src/primitives/analyze_subjects.rs:807` — **fixed**
- QUAL-TEST-001: dropping_named_tempfile_leaves_existing_session_unchanged called no ductus code; replaced by an inode test proven red against an in-place write — `runtime/src/primitives/write_session.rs:694` — **fixed**
- dead code: ProxyEnv's unused derives, whose Debug would print proxy credentials — `runtime/src/primitives/fetch_archive.rs:349` — **fixed**
- dead reference: 'prune.md step 4's shape' names the gate-confirm step, now step 6, in two places — `runtime/src/interpreter/mod.rs:26` — **fixed**
- doc: PruneWalk::features called every entry the single-feature summary; prune-tasks' entry is a compact line — `runtime/src/schema/primitives.rs:2934` — **fixed**
- doc: StatusOnly and read_status named only the --reset gate among their callers — `runtime/src/primitives/prune_tasks.rs:42` — **fixed**
- doc: the exec session report said peers share the target; they share the feature — `runtime/src/interpreter/mod.rs:853` — **fixed**
- doc: feature_dir_cmp said sequential prefixes are exactly three digits — `runtime/src/primitives/mod.rs:2607` — **fixed**
- doc: COMPATIBLE_STATUSES did not name the plan-record tally as its second consumer — `runtime/src/schema/status.rs:15` — **fixed**
- doc: write-session's MCP description omitted the expired and unreadable result fields — `runtime/src/mcp/server.rs:735` — **fixed**
- doc: eight schema doc comments described behavior the code no longer has (analysis record file, dry-run, a retired type, null status, inbox bullet form, commit-based freshness, invalidate-review's file, own-path) — `runtime/src/schema/primitives.rs` — **fixed**
- claim: the constitution said validate for /analyze and hard-coded /ductus:analyze in adopter-shipped text — `framework/constitution.md:737` — **fixed**
- claim: analyze.md and the constitution said every advisory check carries a published promotion criterion — `framework/commands/analyze.md:95` — **fixed**
- claim: analyze.md's full set of checks omitted grounding, criterion labels and cross-service references, and named a report-shape section that does not exist — `framework/commands/analyze.md:104` — **fixed**
- claim: analyze.md's Scope Boundaries counted three writes and left out resolve-session's session state — `framework/commands/analyze.md:33` — **fixed**
- claim: help.md gave /groom three routes, the session id as every agent's, and a partial artifact list — `framework/commands/help.md:120` — **fixed**
- claim: README gave quality-cross three failure modes; it has five — `README.md:111` — **fixed**
- claim: docs/slash-commands.md said /prune removes only tasks.md sections — `docs/slash-commands.md:90` — **fixed**
- claim: target.md said only /target changes the target — `framework/commands/target.md:15` — **fixed**
- claim: groom.md and target.md said cli-config-dir is always preserved; a malformed default is replaced without it — `framework/commands/groom.md:96` — **fixed**
- contract: prune-plan's apply writes plan.md before its reopen check can fail, against the plan's 'each error writes nothing' — `runtime/src/primitives/prune_plan.rs:264` — **routed** to `specs/041-task-pruning/tasks.md`
- contract: prune-tasks' all + apply walk rewrites tasks.md before its status read can fail — `runtime/src/primitives/prune_tasks.rs:177` — **routed** to `specs/041-task-pruning/tasks.md`
- reuse: prune-plan rebuilds the stored-decision key inline beside write_analysis::finding_key — `runtime/src/primitives/prune_plan.rs:305` — **routed** to `specs/041-task-pruning/tasks.md`
- contract: 041's spec, plan, data-model, scenarios, prune.md, analyze.md and the constitution's new text disagree on the reopen's timing, AC19's trigger, what prune touches, contributor knowledge's destinations and the design-record list's home — `specs/041-task-pruning/spec.md` — **routed** to `specs/041-task-pruning/tasks.md`
- record: 052's review.md calls its 7 unread files unchanged since the full review; all 7 changed — `specs/052-spec-supersession-and-consolidation/review.md:32` — **routed** to `specs/052-spec-supersession-and-consolidation/tasks.md`
- claim: 052's spec says prune destroys tasks.md; since 041 it also removes plan sections — `specs/052-spec-supersession-and-consolidation/spec.md:65` — **routed** to `specs/052-spec-supersession-and-consolidation/tasks.md`
- test: common/mod.rs's stated reason for concurrent_sessions.rs's own spawn helper is inexact — `runtime/tests/common/mod.rs:22` — **discarded**: that file removes the same variables and then sets each child's identity; converging it on ductus_command is a refactor with no behavior at stake
- test: a redundant `||` in a crlf assertion — `runtime/tests/crlf_preservation.rs:210` — **discarded**: cosmetic, and predates the window
- test: the crlf prune-tasks cases do not assert that the rewrite happened — `runtime/tests/crlf_preservation.rs:196` — **discarded**: their inputs do trigger the rewrite, so a CRLF regression fails them; predates the window
- test: the walker prune test's doc says the preview reports a missing plan without asserting it — `runtime/tests/walker.rs:829` — **discarded**: prune_plan's a_missing_plan_is_reported_not_an_error pins that; the walker test's subject is the walk continuing
- reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18` — **discarded**: predates the window; consolidating test helpers is a refactor outside 048
- dead code: dispatch_primitive's gate-confirm arm is unreachable in production — `runtime/src/interpreter/mod.rs:1083` — **discarded**: the registry-pin test requires every registry primitive to dispatch; removing the arm means exempting gate-confirm from that pin, a design change outside 048
- reuse: the exec session report joins feature and scenario by hand beside SessionTarget::display — `runtime/src/interpreter/mod.rs:875` — **discarded**: a SessionPeer is not a SessionTarget, and a shared helper for two three-line joins would be premature
- robustness: blob_text reads every tree lookup error as an absent file — `runtime/src/primitives/mod.rs:1488` — **discarded**: only a corrupt object store reaches it, and the repository's other reads would fail first
- reuse: fetch-archive's sha256_hex hand-rolls the encoding analyze_subjects::hex shares, and hex lives in the analyze-record module — `runtime/src/primitives/fetch_archive.rs:655` — **discarded**: moving hex into mod.rs first is the prerequisite, a refactor outside 048
- naming: check_stuck mixes project-root and work-tree paths, and its_rel names mean work-tree paths — `runtime/src/primitives/check_stuck.rs:48` — **discarded**: both are correct as written and pinned by the subdirectory test; a read_at_commit sibling is a refactor outside 048
- errors: InvalidSlug and InvalidPath print a rejected value raw, a newline included — `runtime/src/primitives/mod.rs:332` — **discarded**: the value reaches callers inside a JSON error envelope, which escapes it
- errors: a consolidation given only path or scenario is refused naming feature — `runtime/src/primitives/retarget_sessions.rs:146` — **discarded**: the refusal is correct and names an argument the call needs; which one it names first is cosmetic
- record: 058's review.md cites plan.md line 274 for 275 and says no 022 file changed — `specs/058-findings-route-at-discovery/review.md:83` — **discarded**: a review record is pinned to its reviewed-against sha and is regenerated by 058's next review
- record: 058's plan calls its write-analysis step 'now step 19' — `specs/058-findings-route-at-discovery/plan.md:347` — **discarded**: it records 058's own renumbering at the time, correct as history
- inbox: an item's fix list predates 041's prune — `specs/inbox.md:31` — **discarded**: /groom re-reads each item when it routes it; the inbox is not a live artifact
- readability: three nested em-dashes in §concurrent-features' adoption sentence — `framework/constitution.md:782` — **discarded**: readability only; the sentence's meaning is intact

## Skipped passes

*None.*

## Unexamined governance

*None.*
