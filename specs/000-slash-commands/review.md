---
spec: 000-slash-commands
reviewed-at: 2026-09-14T22:40:43Z
reviewed-against: 2265d2ed59aacf5a5fc68e4e144ea17628f87434
diff-base: aadc1d30f0b214b31e2326e06ef21c66bf0cdc2d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 19
skipped-passes: []
---

# Review — 000-slash-commands

## Summary

All five passes ran over the nine live in-scope files, each read in full: `spec.md`, `plan.md`, six `scenarios/*.md`, and `specs/README.md`. **examined 9 of scope 19.** The other ten are the plan's Affected Files — `commands/{about,analyze,clarify,implement,next,plan,setup,specify,status,target}.md` — and **none of them exists**: 000's directory was reorganized to `framework/commands/`, `about` became `help`, `setup` became `configure`, and `next` was retired. They stay in scope because the plan lists them and are named here as absent rather than folded into the numerator. Read outside the scope, for the criterion walk: `tasks.md`, `review.md`, and the four unedited scenarios (`clarify-one-at-a-time`, `implement-skips-planned-prompt`, `scenario-without-task-visibility`, `target-argument-parsing`) — all four verified against the tree and correct as written.

Recorded 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped. **Two findings were raised and fixed inside the pass, and both were defects in this pass's own step-4 edits** — reported because they bear on whether this review's subject can be trusted. *Reuse*: the §Session State repair replaced the retired JSON block with a TOML one, which is a copy of a canonical source replaced by another copy, and already two keys short of it (`009-scenario-targeting` added `scenario` and `scenario-path`); it now points at 022's `write-session-primitive`. *Quality*: §Parameterization generalized `.claude/` to `{cli-config-dir}/` and left `commands/` — the `claude-style` branch of a three-way layout-derived value — asserted as a universal, the 028/032 defect; both it and the Invocation form now point at `framework/bootstrap/ductus.md` §Derived values, confirmed by hand at line 57 because a *qualified* anchor is reported resolved without being checked. Security, efficiency and simplicity produced nothing: the scope is documentation, with no code, credentials, queries or loops in it.

**`criterion-path-existence` has never checked a single path claim in this spec.** `check-artifacts` reports `clean: true` with exactly one skip (`specs/templates/`, reason `ships-to-adopter`) — an exclusion by construction, not a worklist — and that one entry is the *only* path-like span among the **22** backticked spans in the 15 criteria. All 22 were walked by hand; the rest fail `is_path_like` on braces, a missing interior slash, or whitespace. This is a third shape distinct from both 051's empty array and 023's 24 skips.

**Step (5), both bases measured and recorded.** Pre-reopen natural base `043a0345` resolved **157 modified-since / 167 in scope at 106,952 bytes** — over the MCP cap, so it ran through the CLI and `jq`; that is **+2.8KB** against the 104,124 the campaign item recorded earlier the same day, `043a0345` being a base that widens with every commit to `main`. Pre-reopen `--since HEAD` gave 0 / 10. The step-4 commit collapsed the natural base to **9 / 19 at 1,481 bytes** on `aadc1d30`, a 72x reduction, and post-reopen `--since HEAD` gave 0 / 10 and was declined for excluding the nine files this pass edited. Both post-reopen legs returned inline.

Fourteen corrections landed. The two largest are content the corpus lost with nothing noticing: `ccdd3ac6` — 000's own *reaches done* commit, the one the prior review was recorded against — truncated `criterion-route-after-draft.md` mid-sentence at an unterminated backtick, dropping four Edge Cases and swallowing the `## Open Questions` heading into a code span, so that file has had no Open Questions heading since 2026-08-17 and the pre-`done` gate was satisfied by the damage rather than despite it; and `validate-fix-mode.md` still requires a checkbox-correcting `--fix` that spec **017 removed entirely** in its own commit message, while today's `--fix` is a different flag whose only writes are guarded `done → in-progress` reverts. That is the fourth 017 reversal the campaign has found with no signpost.

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
