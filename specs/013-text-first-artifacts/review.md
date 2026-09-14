---
spec: 013-text-first-artifacts
reviewed-at: 2026-09-14T17:38:50Z
reviewed-against: 7592ef99105fe1590bf69f18663256a810aaedb2
diff-base: 2501ca9aa6cf358c400014d4b0a88aef5e3d8148
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 4
scope: 40
skipped-passes: []
---

# Review — 013-text-first-artifacts

## Summary

Clean across all five passes — 0 MUST, 0 SHOULD, 0 low-confidence — over a scope of 40, of which **4 were read in full**. The prior record predates `ductus-v0.49.0`: no `examined`, no `scope`, no `reviewed-digest`, so a run whose five passes never fired would have been byte-identical to this one. This pass found the campaign's largest defect set so far, and **`check-artifacts` reported `clean: true` with an empty `skipped` array throughout it** — because only **7 of the 39** backticked spans in these criteria are visible to `criterion-path-existence`, and all 7 resolve. An empty `skipped` array is a fact about backticks and slashes, not about claims; all 39 were walked by hand.

**Both diff bases measured after the step-4 commit, and the natural base taken.** Natural base `2501ca9a` (the reopen commit's parent): 7 modified-since / 40 in scope, plan-affected 35. `--since HEAD` gave 0 modified-since / 35 in scope — declined, because it excludes by construction the files this pass edited. The window widened across the pass rather than shrinking: it opened at 2 / 36, took the bootstrap fix and its `govern.md` mirror to 4 / 37, and reached 7 / 40 once the reference repairs and this spec's **own `review.md`** landed inside the diff window. A spec's `review.md` entering its own scope is the documented case — it is the record this run writes, not a subject its passes read, so it is named here and **not** counted in `examined`. The campaign item records 013's pre-reopen window as **163 in scope at 73,162 bytes**, measured 2026-09-14; that figure is cited rather than re-measured, since the reopen moved the base before this pass could take it. Both post-reopen legs returned inline.

**Nine of eighteen criteria asserted behaviour the tree contradicts, and one spec caused all of them.** [017 — Derive, don't ask](../017-derive-dont-ask/spec.md) reversed three of 013's decisions in the weeks after it closed, and **left 013 no signpost** — `grep` for `017` in the spec body returned nothing before this pass. [§cross-spec-impact](../../framework/constitution.md#cross-spec-impact) requires the affected spec to carry the change; nothing detects when it does not, which is stated there rather than implied. The signpost is now in place, as a blockquote, and `derive-dependencies` confirms it induced no edge.

**The `tags` programme — six criteria.** 013 made `tags` first-class through three reinforcement points: a `tags: []` placeholder in the spec template, a `/ductus:specify` prompt at creation with sibling-spec autocomplete, and a `/ductus:clarify` advisory at `draft → clarified`. 017 removed all three — `fc737946` dropped the starter vocabulary from the constitution, `d355db91` stripped the template placeholder as discipline-required frontmatter, `1c50e2cb` removed the prompt and the advisory. The reason is the one worth recording: **a field an author must remember to fill is exactly the diligence dependency [§design-principles](../../framework/constitution.md#design-principles) rejects**, which is 017's whole thesis. The constitution now names `tags` among the **stale fields that produce no findings**. AC2, AC4, AC5, AC7, AC8 and AC11 each asserted a piece of it; `framework/commands/specify.md` and `clarify.md` carry no tag handling at all today.

**The scenario key — two criteria.** `d355db91` renamed a scenario's required field from `spec-ref` to `section` and narrowed its meaning to the parent section alone. AC2 and AC6 still name `spec-ref` as required; the template ships `section: "{Section name}"`. `spec-ref` survives as the legacy fallback the constitution keeps for pre-017 scenarios, and the hard fail fires only when **both** are missing — so nothing broke, which is why it went unseen.

**The Quartz recommendation — two criteria.** `f1ada44d` dropped `npx quartz specs/` as **broken**, not reworded. AC16 promises the README documents it as the recommended viewer; the section now lists four options under "no viewer required". AC17 promises the bootstrap tip names that invocation; it reads tool-neutrally. Both scoping decisions held and are preserved — the recommendation lives in this repo only, and the project-readme template is still unmodified.

**One live defect in a shipped file, fixed in the pass.** `framework/bootstrap/ductus.md` §Frontmatter Migration constructs **both** frontmatter blocks — spec files and scenario files — with `tags: []`. A pre-frontmatter adopter running `/ductus` today therefore has every migrated artifact seeded with the field 017 retired on principle and which no check will ever read. Nothing errors, because the open-schema rule makes the key legal: the migration's output is valid and merely wrong, which is why it survived. Removed from both blocks; `framework/` only, so it reopens nothing, and `govern.md` was re-copied in the same commit for audit Family 21. **The scenario block's `spec-ref` is not the same defect and was deliberately left**, with a note added saying why: a project still on bold-prefix metadata is pre-017 by construction, and its `**spec-ref:**` value names the parent feature *and* the section — which is what `spec-ref` means and is not what `section` means, so writing it under `section` would rename the field while silently changing what it asserts.

**`data-model.md` is rewritten as a pointer rather than re-synced, and that is the reuse finding.** [§drift-prevention](../../framework/constitution.md#drift-prevention)'s canonical-sources map names the constitution's §text-first-artifacts as authoritative for this schema, so the file held a **second copy of a canonical fact** — the failure that section names in as many words. It had rotted exactly as predicted: a `tags` field the constitution deleted, `spec-ref` as a scenario's required key, a nine-row starter tag vocabulary no longer published anywhere, and an advisory severity for a check that now produces no findings. Nothing detected it, because no link check, anchor resolver or audit family compares two prose tables for agreement. Re-syncing would recreate the copy; the section's standing instruction is to point, so the tables are replaced by a pointer and what 013 genuinely owns is kept — the frontmatter block, its parse contract, the open-schema rule, and status-less scenarios. This also **discharges the five durable-contract lines two corpus-wide sweeps deferred into this pass** on 2026-09-13 (three from the common-noun sweep, and `:13`/`:27` from the `spec-and-plan.md` sweep), since the tables carrying them no longer exist.

**Two candidates were run down and are NOT defects — recorded so the next reader does not re-derive them.** `scenarios/criterion-identifiers.md` §Edge Cases names `max(existing) + 1`, which [§spec-requirements](../../framework/constitution.md#spec-requirements) forbids in as many words — but read in full, that scenario's own Resolved Questions walks from that rule to the adopted `max(highest label in body, next-criterion)` and states the exposure that forced the change (deleting the highest criterion drops the body maximum and reissues its label). `label_criteria.rs:88` implements the adopted rule. It is a decision record doing its job, not a contradiction. And `scenarios/past-tense-motivation-convention.md` says its guidance lands in the spec template's `## Motivation` section; the template has no such heading, but the guidance did ship — in the authoring comment block at `framework/templates/spec/spec.md:64`, conditional and with its full rationale. Substance delivered; the location phrasing is loose rather than false, so it was left rather than edited into a manufactured finding.

**Nine criteria verified as holding, several by direct measurement.** AC9 — all **54** specs carry frontmatter and none carries a bold-prefix `**Status:**`/`**Dependencies:**` line, checked by script across the corpus rather than sampled. AC10 — no bold-prefix metadata parsing survives anywhere under `framework/commands/`. AC1 and AC3 against the constitution, read in full. AC12–AC15 against `framework/bootstrap/ductus.md` §Frontmatter Migration, read in full, including the dirty-tree precheck (`git status --porcelain -- specs/`, refuse before any modification) and the `pinned.files` skip. AC18 — `markdownlint-cli2` clean over 509 files.

**Coverage boundary — what was read, and what was not.** Read in full and counted: `specs/013-text-first-artifacts/spec.md`, `specs/013-text-first-artifacts/data-model.md`, `framework/constitution.md` (761 lines), and `framework/templates/spec/scenario.md` (25 lines). **Read in full but not counted, because they are not in this run's scope**: both of 013's scenarios, `criterion-identifiers.md` (117 lines) and `past-tense-motivation-convention.md` (36 lines) — they were not modified since the base and the plan does not list them, so they are outside the denominator even though the pass read every line of each. **Not read in full, and none folded into the numerator**: `framework/bootstrap/ductus.md` (141KB) — §Frontmatter Migration was read entire, plus the spec-root and post-run-output sections, but not the file. `framework/templates/spec/spec.md` — its frontmatter, heading list and authoring comment block only. `README.md` — §Viewing artifacts and §Configuration. The twelve `framework/commands/*.md` entries — each searched for tag handling and bold-prefix parsing, with `specify.md`, `clarify.md` and `analyze.md` read at the relevant sites; none read whole. The thirteen sibling `specs/0NN/spec.md` entries and `specs/README.md` — **not read**; they are in scope because 013's plan lists the self-migration, and what that scope exists to check is AC9, which was verified across all 54 specs by script rather than by reading thirteen bodies.

**Five in-scope paths do not resolve, each with a known cause, and each stays in scope because the plan lists it.** `.claude/commands/ductus/*.md` is a glob rather than a path. `framework/commands/capture.md` and `framework/commands/elaborate.md` were folded into `/log` and `/amend`. `framework/templates/spec/spec-and-plan.md` was retired by `023-govern-refinement`, which this spec's body already annotates. `specs/005-workflows/spec.md` was consolidated into 043 on 2026-09-13 under the retired-feature rule.

Checks run against the committed tree: `lint-markdown` on the feature directory and on `framework/bootstrap/ductus.md` (clean), `check-corpus-links` (0 findings), `derive-dependencies` (drift false, 54 examined, 013's `dependencies:` unchanged by the signpost link), `resolve-anchor` on both edited artifacts (`unresolved: []`, with every `qualified` reference confirmed by hand against its target heading), and `cmp` holding `ductus.md` and `govern.md` byte-identical for Family 21.

**013's share of the 62 unresolved `§` references is discharged here**, which is what its own pass was the cheap moment for. Three references named a target document nowhere on their own line, so each read as a claim about the constitution, where no such marker exists: `scenarios/criterion-identifiers.md:113` and `tasks.md:194` both pointing at this spec's own §Frontmatter Schema, and one in an earlier draft of this Summary pointing at the bootstrap's §Frontmatter Migration. All three now name their document on the reference's line and `resolve-anchor` reports `unresolved: []` across all seven of this feature's artifacts. Fixing the scenario staled `reviewed-digest` inside this pass, so the review was re-run against the commit carrying the fix rather than recorded ahead of it.

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
