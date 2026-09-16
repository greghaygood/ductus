---
spec: 019-config-decisions
diff-base: a116118e2c85c1cd712c03991c6947d741edc19f
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T20:36:35Z
reviewed-against: 67f2c20b640c56ad0fcd902d88eba4fe01231145
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 12
reviewed-digest:
  data-model.md: 5dfef4053828f509aeb168d0be6948dfbe6c2cad1d6ac865a3b58822497d23a9
blocking: false
---

# Review — 019-config-decisions

## Summary

Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence. Rule files loaded via `discover-rule-files` (all 11); `quality-cross.md` read in full.

**Diff base.** Both measured before choosing. Natural base `a116118e` — scope 12, modified-since 10. `--since HEAD` (`44ab1892`) — scope 4, modified-since 0. Took the **natural base** even though it is the wider of the two, because it is the one that covers the edits this pass made: 019's own `spec.md` and `data-model.md` are inside it and outside the `HEAD` window. The campaign rule asks for the honest denominator, not the small one.

**examined: 4 against a scope of 12.** The eight not read split three ways, and they are not the same kind of thing.

*Five no longer exist.* `specs/005-workflows/{spec,plan,tasks,data-model,review}.md` — this pass consolidated 005 into 043 and removed the directory, and 019's `plan.md` lists 005's `spec.md` in its Affected Files, so it stays in scope and is named as absent.

*Two are other specs' bodies, carried in by the same commit.* `specs/043-workflows-sunset/spec.md` and `specs/054-remove-supersession/spec.md` were edited in `44ab1892` as inbound referrers to the removed 005, not as 019 subjects. Named rather than counted.

*One is the backlog file.* `specs/inbox.md`, edited in the same commit to move 005 off the campaign list.

*Read in full and counted:* `README.md`, `specs/019-config-decisions/spec.md`, `specs/019-config-decisions/data-model.md`, and `specs/019-config-decisions/tasks.md`. `framework/bootstrap/ductus.md` is in scope via the plan and was **not** read in full — 141KB — but its §Project Configuration, §Agent Registry, §Pre-run Migrations and the `[services]` validation prose were read directly, which is where every claim checked below lives. Nothing here is a claim about the rest of that file.

**One finding, fixed before this record rather than filed.** §Schema validation asserted the post-scaffolding summary is *the only* enforcement layer for `.ductus/config.toml`. Two have been added since 019 shipped: `/ductus:analyze` reads the `[services]` registry and reports a broken cross-service reference as Advisory (030), and `/ductus:audit` Family 17 reads `[host] project` to check namespace parity. The per-key design the section describes is intact; the word "only" was not. Corrected in `67f2c20b` — a durable contract, which is why this review runs against a commit that already contains the fix.

**Security.** 019's surface is a committed, user-editable TOML file that `/ductus` writes to. The relevant posture is fail-loud on malformed input and additive-only writes, and both hold: a TOML parse error aborts rather than degrading, writes add a section or key without reordering or overwriting sibling tables, and repeated declines do not duplicate entries. Nothing in scope handles credentials, network input, or untrusted data. No finding.

**Quality.** QUAL-CLAIM-001 is the rule with teeth here, and it is what the correction above was: a result asserting a global property ("the only enforcement layer") that its own subject had outgrown. The section now states its scope rather than a superlative. QUAL-GROUND-001: the unrecognized-entry path is the compliant shape — an entry that matches no registry category is surfaced in the summary rather than silently ignored or fatally rejected, which is a documented assumption paired with a visible, non-blocking signal. QUAL-STUB-001: no pass-through path; every branch either writes, reports, or aborts.

**Reuse / efficiency / simplicity.** The section-per-domain schema is 019's durable contribution and it is the reason this spec survives its own feature's removal: `[pinned]`, `[rules]`, `[paths]`, `[runtime]`, `[services]`, `[constitutions]`, `[host]`, `[project]` and `[migrations]` all follow the shape it resolved, each choosing keys that fit its decision rather than being forced into a generic `declined_*` form. The Resolved Question rejecting a `[decisions.*]` umbrella has been vindicated by eight later sections, none of which would have fit under it. No finding.

**On the ten criteria.** AC1–AC10 all describe the `[workflows] declined_categories` surface that 043 removed. They are **not** individually annotated and deliberately so: the spec carries one post-completion Note below its H1 stating that the section was removed, that the mechanism survives, and that body references to `[workflows]` below are historical. One well-placed note governing ten criteria is the pointer §drift-prevention asks for; ten copies of it would be the thing that rots. Verified against the tree: AC10's README rename holds (§Configuration exists), and the `declined_categories` key appears in no live artifact except `framework/migrations/workflows-sunset.md`, which is the migration that removes it.

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
