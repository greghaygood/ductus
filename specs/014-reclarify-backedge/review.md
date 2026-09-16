---
spec: 014-reclarify-backedge
diff-base: 91bacaeead3c831785cf16f66173e9838ad001fb
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T19:01:43Z
reviewed-against: 5d8b492c1e2c6a0f3a99f5d1408829efb48c4639
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 11
reviewed-digest:
  scenarios/reopen-after-informal-edits.md: 969e20fe262affefdceaaa1e3b5ac900f0877d743f456035bc4de5f37cfe1a23
blocking: false
---

# Review — 014-reclarify-backedge

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes at this HEAD. Not blocking. One ticked criterion and six smaller claims were found stale during the run and all were corrected at source before this record was written (`5d8b492c`), so the zero counts describe the reviewed HEAD rather than a clean first pass.

**Examined: 8 of 11 in-scope files read in full.** Read completely: `framework/commands/amend.md` (235), `framework/commands/clarify.md` (195), `framework/commands/plan.md` (145), `framework/constitution.md` (761, in four ranges), `specs/000-slash-commands/spec.md` (151), `specs/014-reclarify-backedge/spec.md` (214), `plan.md` (121), and `scenarios/reopen-after-informal-edits.md` (59). **Not read: the three `.claude/commands/ductus/{amend,clarify,plan}.md` mirrors.** They are generated from the three `framework/commands/` sources that *were* read in full, by `scripts/gen-claude-commands.sh`, which was run during this pass and reported `No changes (16 command(s) in sync)` — so their content is the read content with `{project}` → `ductus` substituted. That is a reason to believe them correct, not a claim to have read them, and `examined` is 8 rather than 11 on that basis.

**Scope, measured on three bases before choosing.** Pre-reopen natural base (`36461bdd`, the 2026-08-03 review's own base): **799** files changed since. `--since HEAD` (`5d8b492c`): `modified-since` empty by construction, scope 8. Post-reopen natural base (`91bacaee`) — **chosen** — `modified-since` 3, scope 11. Same reasoning as the rest of this campaign: tighter than 799, and more honest than `HEAD`, which would exclude this pass's own edits.

**Criteria: 25 of 26 hold; AC3 did not.** AC3 claimed `/amend` on a `done` spec "refuses and reports: *Spec is `done`. Run `/{project}:amend` to capture this as a scenario instead*" — an instruction to run the command the user is already inside. It was not authored that way: at `a5e9ec52` it named the then-separate `/{project}:elaborate` command, and `023-govern-refinement` folded that command into `/amend`, after which the rename sweep rewrote the quoted message. That is the failure AGENTS.md §Gotchas names — a sweep rewriting the record of the thing being swept — and the resulting instruction was never emitted by any release. The behaviour behind it is superseded as well: a `done` spec on the question route is now **unreachable** rather than refused, because the classifier's status tiebreaker routes `done` to the scenario path (`amend.md` §Classify the input) and `flip` toward the question route is rejected there with its own distinct message (§Approval gate; the `done`/question row of §Status mutation summary reads "Not reachable"). Annotated in the house style of 009's AC18 and 046's AC22 rather than rewritten, with the surviving guarantee — no question recorded, no status mutation — stated explicitly.

**The same sentence was deliberately left alone in four places.** AC11, the spec's `/clarify` gate table, `tasks.md` task 25, and `framework/commands/clarify.md:34` all carry the identical string, but issued by `/{project}:clarify` it names a *different* command and is correct. One string, two meanings; a blanket fix would have broken the correct half. The diff touches only the two `/amend`-side lines, which was verified against the diff rather than assumed.

**Five further stale claims, all inside 014, all corrected.** (1) The spec's `/amend` status table carried the same superseded `done` row. (2) Two pointers to "`framework/constitution.md` lines 96–99" — §spec-lifecycle now begins at line 156, and 96–99 is §recommendations, so both resolved to unrelated text; the line numbers are dropped and the durable `§` anchor kept. (3) The plan's `done` bullet carried the same swept redirect, restored to `/elaborate` with its supersession note. (4) The plan asserted "the existing lightweight-track branch … is unchanged" — the `spec-and-plan.md` track has since been retired outright (`framework/migrations/spec-and-plan-sunset.md`), and no command source mentions it; verified by reading all three command sources in full. (5) The plan attributed the generated-command rule to `CLAUDE.md`, which does not carry it — it is `AGENTS.md` §Boundaries. (6) `## References` listed three of the four declared dependencies, omitting `023-govern-refinement`; adding it changed no frontmatter, since 023 was already a dependency via body links, and `derive-dependencies` reports no drift and no cycle after the edit.

**The scenario read as work owed and no longer does.** `reopen-after-informal-edits.md` framed its two re-open fixes as an open implementation choice ("this scenario is the place to pick one … during implementation"), though its own Resolved Questions picked both and both have shipped: Option B as `amend.md` §Re-open precondition and reconcile pass — whose delta definition (`git status --porcelain` over `scenarios/`, `spec.md`, `tasks.md`) and prompt match what the scenario specified — and Option A as the `AGENTS.md` §Workflow rule, promoted into `framework/constitution.md` §spec-lifecycle. A note records that; the options prose is left as the record of the choice.

**Behaviour verified against the sources, not inferred.** AC1/AC2/AC4–AC7 against `amend.md` (§Status mutation summary rows, §Impact display, the "Do not prompt again for the back-edge" line, the two post-question hints). AC8–AC12 and AC16 against `clarify.md`'s Gate table and Markdown-only reference — AC12's guarantee is stated in as many words at `clarify.md:151`, and AC16 twice, at lines 116 and 154. AC13's four display items and prompt match the Recovery path at `clarify.md:138-152`; the criterion quotes the prompt in abbreviated form while the spec body at line 93 carries the exact string the command uses, so it is an abbreviation rather than a contradiction. AC14/AC15 against step 3's guarded revert and its decline arm. AC17–AC22 against `plan.md` §Detect existing artifacts, including that **keep** is the default and that the protection runs before every generation rather than only post-back-edge. AC23 against the constitution's current back-edge bullet, which names `/amend` and `draft` exactly as §Constitution Updates prescribes. AC24 against `specs/000-slash-commands/spec.md:32`, which carries the signpost covering all three of its claims — correctly as a blockquote, so `derive-dependencies` induces no edge from 000 to 014. AC25's three targets exist and the generator reports them in sync; AC26 re-run clean over 513 files.

**All eight Resolved Questions still hold**, checked as live claims rather than history: `/amend` still owns both back-edges (and has since gained a third, the criterion route, which adds no edge); no flag exists on either path; checkboxes are still not cleared on the back-edge; `draft` is still the destination; `/clarify` is still the resolver with a recovery branch, stated in as many words at `clarify.md:14`; and the consent model is unchanged. The plan's five trade-offs hold likewise, with the lightweight-track sentence in the fourth being the one exception, now corrected.

**Observed, out of scope, and already queued.** `specs/000-slash-commands/spec.md` is in scope only because 014 added its signpost, and it carries substantial drift of its own — AC14 and AC15 plus a whole §Lightweight Track Detection section describing the retired combined-document track, and a `## Why this spec is still in-progress (2026-08-16)` section while its frontmatter reads `done`. None of it is 014's, and 000 sits on the examined/scope backfill's own Remaining list, so it is left for that pass rather than swept piecemeal here. The retired-track residue is wider than 000: measured this run at **187 hits across 48 files** — 49 in `spec.md` bodies, 53 in plans, 54 in the ephemeral `tasks.md`, 7 in scenarios, 4 each in data-models and regenerated reviews, against 13 load-bearing hits in three framework files where the name is the subject (`spec-and-plan-sunset.md`, `migrations.toml`, the shipped pre-commit hook) and must survive. Several carriers are specs this campaign has already marked backfilled, so the per-spec passes are demonstrably not catching it; it is filed as one measured inbox item rather than swept.

**Pass-by-pass.** Security: the scope carries no application code; the only executable surface named is `amend.md`'s `git status --porcelain` call, which is path-scoped to the feature directory and reads no diff bodies, and each command's Scope Boundaries constrain what may be read and written. Nothing found. Reuse: all three commands point at the constitution rather than restating it ("constitution loaded by `/{project}:target` — do not re-read", and the single §runtime-host-integration pointer), and the recovery path is documented once and referenced from the numbered step rather than duplicated. Nothing found. Quality: the AC3 defect and the six stale claims above. Efficiency: not applicable to prose procedure; nothing found. Simplicity: the gate is data-driven on `(status, open-question count)` rather than the flag the Resolved Questions rejected, which is the simpler of the two designs considered; nothing found.

Deterministic corroboration at this HEAD: `npx markdownlint-cli2` clean over 513 files; six lints and the audit (`scripts/audit/run-all.sh`, exit 0 — proved red earlier this session with a Family 21 probe, so its silence is evidence) all green; `gen-claude-commands.sh` and both derivations left the tree clean, with `derive-dependencies` reporting no drift and no cycles across 55 specs; `cargo test --release --locked` exit 0 with 20 of 20 binaries reporting ok.

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
