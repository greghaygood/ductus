---
spec: 003-bootstrap-automation
reviewed-at: 2026-09-13T14:07:50Z
reviewed-against: 5a519cbb66d888ad05df3476925bc5c894711d6b
diff-base: 5a519cbb66d888ad05df3476925bc5c894711d6b
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 11
skipped-passes: []
---

# Review — 003-bootstrap-automation

## Summary

Clean: 0 MUST, 0 SHOULD, 0 low-confidence outstanding. Re-review for the examined/scope backfill; the prior record (2026-08-03) predated ductus-v0.49.0 and carried neither `examined` nor `reviewed-digest`.

Scope, and what was NOT read. `examined: 8` of `scope: 11`. Eight files were read end to end: `analyze.md` (391 lines), `clarify.md` (195), `implement.md` (187), `init.md` (199), `plan.md` (145), `specify.md` (202), `status.md` (86), `target.md` (72). **Three in-scope paths do not exist and could not be read**, each for a reason this spec's own §Note records: `.claude/commands/ductus/about.md` — renamed to `help.md` by 012; `.claude/commands/ductus/setup.md` — renamed to `configure.md` by 012; `.claude/commands/ductus/next.md` — retired, its next-step suggestions absorbed into `/ductus:status` and `/ductus:target`. They remain in scope because 003's plan lists them as Affected Files and `compute-review-scope` reads that list; the successor files are not in scope and were not substituted for them. Nothing was skipped for convenience.

Diff base. `HEAD` was passed. The natural base resolves a window spanning every change since 003 closed, against a plan affecting eleven files; both measured before choosing, per AGENTS.md §Gotchas.

`check-artifacts` skipped array, walked by hand. Three entries, none of which the family could substantiate, each checked against the tree here. `.ductus/session.toml` (`not-a-live-claim`, AC2) — the claim holds: the commands do reference it, and AC2's trailing annotation that `.govern.session.toml` survives as a legacy fallback is confirmed by `target.md` step 1, which resolves the newest of three tiers. `.claude/gov-session.json` (`not-a-live-claim`) — AC2's own parenthetical recording the pre-0.10.0 filename; a historical record, correctly exempt. `specs/templates/` (`ships-to-adopter`, AC11) — absent here because this repo uses `framework/templates/`, and `init.md` does create `{spec-root}/templates/` in the scaffolded project, so the criterion holds where it applies.

Rule selection. All 11 rule files loaded; none fires. The scope is eight command-definition documents in a project with no HTTP surface, no database and no browser.

Acceptance criteria. All seventeen verified against the tree, enumerations included. AC1 was **false on two counts and is fixed at source** in 5a519cbb — see below. AC2–AC4 hold (session path, the gates in `clarify.md` / `plan.md` / `implement.md`, and the dependency checks `traverse-deps` performs). AC5–AC17 hold against `init.md`: it exists at the named path, collects the four inputs, pre-flight-checks the target directory, scaffolds the file set, substitutes `{project}`, copies commands and spec templates, creates the system stubs, fetches per-language `.gitignore` patterns, runs markdownlint, displays next steps, and its §What This Command Does NOT Do carries the two negative criteria.

One defect was found and fixed at source in 5a519cbb, before this record was written; the counts state what is outstanding, which is nothing. 049 renamed `commands/gov/` to `commands/ductus/` and left the bare `gov` namespace token beside it, so AC1 read "copied to `.claude/commands/ductus/` with `{project}` replaced by `gov`" — self-contradictory, and false since the generator has read `PROJECT="ductus"` since that rename. AC1 also froze a count, "all ten command templates", against a set that is now sixteen sources plus `configure` and `init`. The same half-swept sentences were corrected in 003's §Behavior and plan, in `AGENTS.md` §Gotchas (which told contributors the generator substitutes `gov`), and in the `[host]` schema example shipped in `ductus.md`, where `project = "gov"` sat beside `project.name = "my-service"` while the prose says the first is written from the second. Records rather than claims were left alone: 003's plan §Trade-offs rejected alternative, 049's own plan, `framework/migrations/ductus-rename.md`'s rewrite rule, and the ticked task records in 003/006/007/012.

No observations. The one candidate — `init.md`'s accumulated drift — was measured and captured during 004's review, whose scope also covers that file; recording it again here would duplicate an inbox item rather than add one.

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
