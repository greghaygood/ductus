---
spec: 048-govern-acquired-runtime
reviewed-at: 2026-09-14T23:15:23Z
reviewed-against: d0bfe956d9db520556693a780e7eb8dba652d606
diff-base: 22dfdc45e1495dc66e6c97865f7acc92667f331d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 24
scope: 28
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Backfill pass: full five-pass review, examined 24 of 28. This is the second run of the passes — the first found a defect in a durable contract, and repairing a durable contract stales `reviewed-digest`, so the review was re-run against the commit carrying the fix rather than recorded over it. The 11 rule files were loaded and `check-rule-ids` reports `examined: 11` over zero citations (048 cites no rule IDs). `process-waivers` ran unrestricted: 0 applied, 0 expired, 0 retained. Step (5) measured both bases: the pre-reopen natural base `3db3d0e9` resolved **319 modified-since / 330 in scope at 147,052 bytes**, over the MCP cap, so it ran through the CLI and was read with `jq`; the step-4 commit collapsed it to **8 / 25 at 1,955 bytes** on `22dfdc45`, and the pass's own later commits widened it to **13 / 28** — the window grew across this pass rather than collapsing further, because `review.md` and the repaired scenario both joined it. `--since HEAD` gave 0 / 18 and was declined for excluding the thirteen files this pass edited, as the last six passes did. Both post-reopen legs returned inline.

Three files in scope were not read in full and are named rather than counted. `framework/bootstrap/govern.md` is a byte-identical mirror of `framework/bootstrap/ductus.md`, which was read in full (1182 lines); audit Family 21 holds the two identical and `cmp` confirmed it in this pass after each edit — believing it correct and having read it are different claims, and only the second is `examined`. `framework/commands/*.md` is a glob rather than a path, carried in scope from the plan's Affected Files; it cannot resolve and never could. `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md` joined the window when this pass repaired one line in it; only that line and its surrounding context were read, so it is named here rather than folded into the numerator — a file read in the region you changed is not a file you examined.

**The pass found ten defects in this spec's own artifacts, all fixed before this record was written, which is why the 0/0/0 is after them rather than instead of them.** Two of 048's own tasks reversed decisions 048 had already recorded, and neither reversal reached the artifacts outside the sections it edited. Task 16 (`pin-is-readable-when-acquisition-needs-it`) moved the version pin off the fetched archive to a direct fetch of `raw.githubusercontent.com`, leaving four sites — the **Acquisition** bullet, the "Which version does `/ductus` resolve?" resolved question and its whole rationale, an Edge Case framed as a missing archive file, and `data-model.md`'s "Read by" row, which still named the retired `{staging-dir}` placeholder. Task 13 (`state-b-continues-in-session`) moved State B's restart from a pre-flight abort to the Closing restart, leaving **Detection states** and `data-model.md`'s State B row asserting an abort that AC10 contradicts in the same file. Two more were intra-spec contradictions present at delivery rather than superseded later: **Acquisition** said a checksum mismatch "degrades to the markdown path" while the **Runtime requirement** section refuting it landed in the same clarify commit `da7c45c8`, and the Asset naming bullet promised a Windows `.zip` pair against AC23, the data-model's asset table, and task 3's own workflow change. `plan.md` additionally claimed former State C's tip was "deleted, not repurposed", the reverse of what shipped. **The tenth was not 048's doing and is the one that cost the re-run**: 050's retired-filename sweep (`510eb25c`) rewrote `retired-namespace-tools-are-off-limits.md` to read *a pre-`.ductus/` binary resolves `.ductus/` and then the legacy root* — self-contradictory, and it destroys the resolution-ladder claim the scenario's justification rests on. The shipped `framework/bootstrap/ductus.md` still read `.govern/` and was correct throughout. The `X` to `X` grep AGENTS.md prescribes for sweep damage cannot see this shape, because the two sides genuinely differ and the sentence still parses. **It is a class, not an instance, and the first measurement of it was too narrow** — scoped to a `pre-<name>` qualifier followed by that same name, which found only this one. Widening to an aligned per-file diff over all 319 of that sweep's substitutions, filtered to pairs that removed the *last* retired token from a sentence carrying a legacy marker, yields 11 candidates across 8 files. A second was confirmed and fixed in the same session — `026`'s `family-23-sweep-target-manifest-parity.md` inverted the incident it exists to record — and it was found because repairing this one costs `mechanical_sweep`'s repo-wide uniformity test, so Family 19 immediately reported 026 stale, a true signal the exemption had been masking. The remaining nine are unread, sit in specs the campaign has closed, and are captured as one measured inbox item priced by destination.

**Fourteen more were in the shipped bootstrap, found by reading it in full rather than by grepping.** The same task-13 reversal had left `framework/bootstrap/ductus.md` describing the old mechanism in nine places, and the worst was not a stale label: **State B**'s notice block told the adopter "`/ductus` could not use the deterministic path this run", which the step directly above it and the **Closing restart**'s own notice both contradict — two different messages specified for one event. The phase also accumulated one restart set while the **Closing restart** reads a `deferred-restart set` nothing filled, so a host following **Pre-flight abort** would emit no restart notice at all and the newly wired server would never load. **Instructions**' host tool-prefix mapping named two agents where **ductus runtime detection** and **State A** in the same file name all four, and **State A** points readers at **Instructions** for it — the outlier is the drift. A behaviour-claim sweep by meaning across `*.sh`, `*.yml`, `*.md` and `*.toml` found the pin claim in exactly two further live places, `scripts/audit/version-agreement.sh` and `.github/workflows/runtime-acquisition.yml`; every other "from the fetched archive" is `framework/migrations.toml`, which genuinely is. All of it is `framework/` and `scripts/`, so it reopened nothing and staled no other spec's review. `govern.md` was re-mirrored in both commits.

Read and found sound, not defects. The acquisition sequence verifies the sidecar digest before anything is written, treats a missing sidecar as a failure rather than a skip, installs by tempfile+rename, re-probes the binary afterwards, and never consults `PATH`; all four permission grammars grant the full sequence including verification. The release graph is `audit`+`build` to `acquire` to `sbom` to `publish` (crates.io, irreversible) to `release-assets` to `verify-published`, with `acquire` as the five-target completeness gate and an explicit `REQUIRED_TARGETS` assertion behind it — AC20 and the `release-halves-publish-together` scenario both hold. `runtime-acquisition.yml`'s `[ "$target" = ... ] && binary=` sits mid-block, not last, so the `set -e` hazard does not apply. AC2, AC11, AC12, AC15, AC16, AC17, AC19, AC21, AC22, AC23 and AC24 were each checked against the tree; AC17's six remaining "With no ductus runtime registered" lines all open a `## Markdown-only reference` section, which is what that criterion preserves.

Two things deliberately not filed. 029's AC20 describes the bootstrap's shell blocks as "the State-B/C fallback spec"; this pass reworded that phrase in `ductus.md` because it named a state 048 deleted, but 029 is closed and its signpost already carries the blanket annotation that its criteria "reason inside the three-state model of the time" — the contract AC20 asserts is intact, so there is nothing to reopen it for. And `framework/bootstrap/govern.md`'s 141 KB of byte-identical duplication is already a measured inbox item carrying its operator call; re-filing it would be the treadmill the inbox rules forbid.

Ten of twelve unresolved section-anchor references were repaired by naming `framework/bootstrap/ductus.md` on each reference's own line; the other two resolved through the same edits. `resolve-anchor` now reports `unresolved: []` across all ten artifacts, and every heading the new qualified references name was confirmed by hand, since a qualified reference is reported resolved without being checked. The two in `tasks.md` were read before being touched, per the standing caution: neither names a section its own task deleted, so making them resolve destroyed no record. The scenarios' present-tense Context sections were **not** swept — 013's `past-tense-motivation-convention` scopes itself to `## Motivation` and records the operator decision that existing `done` specs are not retroactively swept.

This spec's own `review.md` is now in scope, having entered the window when the first review commit landed; it is named here and not counted in `examined`, since it is this run's output rather than a subject its passes read. Nothing reached `runtime/`, so no version bump and no tag are owed.

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
