---
spec: 035-groom-session-target
diff-base: 80cdfe2398c8c36ba65b65c34a3722c8e09f0143
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T17:41:03Z
reviewed-against: 3141463ab739f72882171628484f41bf797f8aee
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
reviewed-digest:
  scenarios/confirmation-names-reopen.md: 62f7f04097775cd8b7e33dc38cb05b47dcbef834e6b4eb54d60bf1bf1ead7829
  scenarios/reopen-done-spec-on-scenario.md: 5f1b1f7cc9a4fb3d5042d48ef2aae191697ff3821a0170ead1d019460cd6d6b2
blocking: false
---

# Review — 035-groom-session-target

## Summary

Five passes over 4 of 4 in-scope files — nothing unread. Backfill re-review under the pre-0.49.0 campaign: the 2026-08-03 record carried no `examined`, no `scope` and no `reviewed-digest`, so it could not be told apart from a review whose passes never fired.

**Scope choice, all three bases measured before choosing (§Gotchas).** The pre-reopen natural base `c97c5b9d` (2026-06-28) resolved a **792**-file raw window against a plan affecting two files. `--since HEAD` resolved **2** (plan-affected only, excluding this pass's own edits). The **post-reopen natural base `80cdfe23`** — the parent of this pass's first fix commit — resolves **4**, the union of the plan's two affected files and the two spec artifacts this pass corrected. Chose it: tighter than the natural base and more honest than `HEAD`, because it covers the edits this pass made rather than excluding them. Every in-scope path exists; none is absent.

**Rule set.** 11 files loaded. `concurrency-backend`, `configuration-cross`, `reliability-backend` and `observability-backend` verify against "any spec or plan", so `spec.md` and `plan.md` are genuinely in-subject rather than N/A by file type, and each was evaluated: the `set-status` `from: done` guard is an optimistic-concurrency choice that is named and justified (BE-COORD-002 satisfied, not violated); the literal `specs/{feature}` in the target write is the documented default under §spec-phase, which groom's Scope Boundaries cites, not a hardcoded-path defect; no outbound call, retry, fan-out, metric, pool or env var is introduced, so the timeout, retry, RED and CFG-ENV rules have no surface. `quality-cross`'s three rules verify against code paths and source, of which there are none in scope. `security-backend`, `security-frontend`, `api-backend`, `performance-backend`, `performance-frontend` and `accessibility-frontend` have no surface: no authn/authz, no user input, no network, no API, no UI, no perf-sensitive path. The single write is to `.ductus/session.toml`, confirmed gitignored at `.gitignore:10` so per-contributor state cannot be committed.

**Quality pass — one finding, fixed at source rather than carried.** groom's step-2 route list summarised `chore` as "left in place per step 8's exception", stating the exception as the rule; step 8 and the Step 4 decision tree both say a chore is done in the pass and then removed, and the constitution's §brownfield-inbox is explicit ("fix it — do not park it"). An agent picks its route from that list. Fixed in `3141463a` so the bullet states the rule and names the exception as one, matching its `discard` sibling. Counted zero here because the record is post-fix.

**Reuse pass — considered and dismissed on evidence.** groom's **Setting the session target** and **Reopening a `done` spec** sections restate a write shape that `specify.md`, `amend.md` and `target.md` also carry. That is not duplication to consolidate: §runtime-host-integration requires two paths that "share one contract; neither wraps the other", and groom.md states that its Markdown-only reference carries the full detail both paths follow. Filing it would have proposed collapsing a guarantee the constitution mandates.

**Efficiency and simplicity.** The per-item walk is explicitly serial by design (routing is per-item judgment, not batchable); the loop is over operator-authored inbox bullets, not user-controlled input, and is bounded by the file. No dead branches: all five routes have live paths, and the only-from-`done` reopen guard is reachable.

**Criteria and decisions verified against the tree, enumerations included.** All seven criteria hold. AC7's regeneration claim was verified by running `gen-claude-commands.sh` and confirming a clean `git status`. `check-artifacts` reports clean with an **empty** `skipped` array, and every path a criterion names is inside backticks, so that emptiness is a fact about claims rather than about backticks — the failure mode §Gotchas records for `criterion-path-existence`. AC3 enumerates three no-target classes where groom now lists four; it matched the source exactly when written and `discard` was added later, and since AC3 claims no exclusivity it remains true as a subset. Both scenarios still describe shipped behavior: the confirmation names the reopen (`groom.md:39`), and `amend.md:100` still carries the explicit prompt the scenario contrasts it against, while `check_review_gate.rs:511` still yields `AlreadyDone`, so the gate-fail the reopen exists to prevent is real.

**Two stale claims found by reading the decisions rather than the criteria, both corrected in `89449954`.** The §Trade-offs entry rejected moving groom onto the runtime primitives as "a larger, separate refactor ... out of scope here", on the premise that groom "uses no runtime primitives" — and `0abc237e` (2026-07-11, spec 022) performed exactly that refactor thirteen days after this spec closed, so groom now opens with the runtime preamble and invokes all three primitives the trade-off named as the road not taken. This is the adopted-rejected-alternative class that no check can see. Corrected in place with a pointer to what reversed it, at all three sites sharing the premise, with the decision record preserved in past tense. Separately, `spec.md` and `plan.md` both described the chore branch as "left in the inbox", superseded by the same do-not-park rule.

**Verification at this HEAD.** Full CI surface, not the markdown subset, because `framework/commands/` is runtime-adjacent: 20 test binaries invoked and 20 reported (`cargo test --release --locked`, 1328 lib tests, 0 failures), clippy `-D warnings` clean, `cargo fmt --check` clean, shellcheck `-S warning` over 55 files, all seven `lint-*.sh`, both `scripts/tests/*.sh`, markdownlint over 512 files, and the 37-family self-audit green — the audit first **proven to fail** on a perturbed generated command copy (exit 1, 1762 bytes) before its silent zero-byte pass was trusted, per §design-principles.

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
