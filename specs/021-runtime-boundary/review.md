---
spec: 021-runtime-boundary
reviewed-at: 2026-09-13T13:15:24Z
reviewed-against: cab970412a37cef25d192915d56786d4f964f054
diff-base: cab970412a37cef25d192915d56786d4f964f054
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 3
scope: 7
skipped-passes: []
---

# Review — 021-runtime-boundary

## Summary

First review of 021 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. All eleven criteria verified against the tree; **five no longer describe current behaviour**, and the cause is a cross-spec obligation that was never discharged.

**What this review read: 3 of the 7 files in scope, and here is the other 4.** Read in full: `framework/constitution.md` (759 lines — the artifact this spec's criteria are almost entirely about), `framework/runtime-tools.txt`, and `scripts/lint-tool-coverage.sh`. **Read in part:** `scripts/lint-frontmatter.sh` — its header, its feature-directory rule and the start of its file loop (60 of 129 lines), enough to confirm AC7(e) and that it names `framework-checks.yml` as its consumer, not the retired workflow. **Not read:** this spec's own `plan.md` and `tasks.md`. **And one path that does not exist:** `.github/workflows/markdown-only-pipeline.yml`, which is itself part of the finding below.

**048 amended the section 021 created, and never signposted it.** §runtime-boundary — the subsection this entire spec exists to add — was substantially rewritten by 048: the runtime went from **opt-in** to **required and pipeline-acquired**. §cross-spec-impact requires the change to be recorded in the affected spec with a back-link, and it was not. 021 has therefore read as the live description of a boundary that stopped being current, with no pointer to what replaced it. This is exactly the failure AGENTS.md records under *done is not the same as discharged*: nothing gates on a signpost, `check-artifacts` does not look for one, `diff-cross-spec` detects the sibling you changed and never the one you owe, and both specs stayed green throughout. Discharged in `cab9704` as a blockquote signpost after the H1, following the precedent 018 set for 017 — the body is left as the record of what 021 shipped rather than rewritten into a description of what replaced it.

**The five superseded claims, each checked against the constitution as it stands.** **AC1** says the opening paragraph references an *optional* runtime; it now says the artifacts are standalone but "the *pipeline* that reads them **requires** the runtime". **AC2** enumerates "the opt-in invariant" as a component; the constitution now carries an **acquisition invariant** and states in so many words that it "replaces the **opt-in invariant**". **AC3** claims all five principles use MUST / MUST NOT; principle 3 was rewritten and now reads "pipeline commands **MAY** assume determinism" — still RFC 2119, but not the register the criterion names. **AC7** has gone stale twice over: its CI workflow `markdown-only-pipeline.yml` was retired by 048, and two of the three generators it names (`gen-spec-deps.sh`, `gen-readme-table.sh`) were deleted by 022's generator promotion — only `gen-help-tables.sh` survives. **AC8** describes that same retired workflow's triggers. The body's principle 3 and eligibility criterion 3 (*Degradation, not failure, when removed*, now *Specifiable as prose*) carry the same supersession and are named in the signpost.

**What still holds, verified rather than assumed.** **AC5**: the `<!-- §runtime-boundary -->` marker is at constitution line 530. **AC6**: the canonical-sources row reads "Runtime contract / boundary | `framework/constitution.md` §runtime-boundary" at line 601. **AC4**: the non-scope list still uses MUST NOT for all four exclusions. **AC9**: the five original §text-first-artifacts principle bullets are present and unedited — later specs added bullets alongside them, which does not falsify "unchanged". **AC7(c), (d), (e)**: `markdownlint-cli2` exits 0 over 512 files, and both lint scripts exist, run clean, and correctly name `framework-checks.yml` as their consumer. **AC10, AC11**: this analyze run and the markdown lint both pass.

**Read as code, not just as artifacts.** `lint-tool-coverage.sh` is a proximity scanner with a file-wide exemption for any command carrying the `runtime-host-integration` pointer, which is why it stays green as commands migrate to that contract; its `grep … || true` and `[ -z "$lineno" ] && continue` are inside a loop body rather than trailing a block, so `set -e` cannot kill the run on a no-match. Its empty-manifest path exits 0 — defensible here, since the manifest is asserted set-equal to `TOOL_NAMES` by `runtime/tests/mcp.rs`, so an empty one fails a different gate rather than passing silently.

**On the diff base.** No commit records 021 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 021 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
