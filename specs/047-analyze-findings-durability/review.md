---
spec: 047-analyze-findings-durability
last-run: 2026-09-16T15:51:43Z
reviewed-against: 8e238dabc40e0d620c1c2e525832be254296430a
diff-base: ddfd95edc715fa17c24bb0a8273bf96c4814bb51
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 7
scope: 13
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 4873de36588f18d253409758c41ab5d0b200c1a1655271c64aa5e8d98998a39f
  scenarios/analyze-run-durability.md: 7719db590c1b78447332523a3725c1d05faa64abb0950d43aedeff50ed3648a2
blocking: false
---

# Review — 047-analyze-findings-durability

## Summary

**Second reopen of this spec for 057, and the reason it was needed is the finding.** The first (task 18) corrected the record-location claims in `spec.md` and in `scenarios/analyze-record-freshness.md`, and closed. Task 19's Family-31 grep then found a live claim in the *other* scenario, `analyze-run-durability.md`, and reading that file for these passes found more. Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, not blocking. No waivers.

**What the first pass missed, and why the instrument could not see it.** Task 17's classifier defines the pre-relocation shape as a record token within ~140 characters of `spec.md`, "spec frontmatter", or "the spec". `analyze-run-durability.md`'s entire Behavior section specifies the record as *"an `analyze:` frontmatter block"* and **names no file** — so the classifier reported nothing, correctly by its own definition, over a durable contract specifying a shape that no longer ships. The same blindness hid its §Resolved Questions entry rejecting an `analysis.md` artifact, which 057 reversed; task 19's grep caught that one only because it happens to mention Family 31. **A classifier tuned for a location cannot find a claim that omits the location**, and that is now recorded on 057's task 17 rather than left as a fact about this session.

**The disposition.** The Behavior section is annotated once at its head rather than re-pointed sentence by sentence, because only the home changed — the fields, the write-on-every-run rule, the two deliberate asymmetries with the review record (`advisory` recorded and never gating; `unexamined` having no counterpart at all), the gate's block reasons and the drift family are all unchanged. The annotation records the two things the move added: the **file's** absence is the never-analyzed state, which is a sharper signal than a missing key in a file that exists for other reasons, and a present-but-unparseable `analysis.md` is a third state the gate must not collapse into never-run. The YAML example keeps its block form with a comment saying that is what 047 delivered. The reversed resolved question is annotated with the observation that *one fact per home* — the rule that bullet argued from — is precisely what produced the reversal: the review record already had two homes, and Family 31 was a third copy binding them rather than a fix.

**Left alone deliberately.** The table at line 15 pairs `/{project}:review` with `write-review` → `review:` block and names Family 31 among what held it fresh. Its surrounding prose is explicitly past tense about the pre-047 state — *"Both gates **were** required; only one **was** recorded … There **was** no field to compare"* — and Family 31 existed then, so it is a correct historical account and rewriting it would record a fiction. Classified by tense, which is the test the whole sweep uses.

**Scope.** `diff-base` ddfd95ed, 13 in scope. Examined **7 of 13**: this spec's `spec.md`, `tasks.md` and both scenarios, 026's `spec.md` and `family-19-says-what-it-examined.md`, and 057's `tasks.md`. The six unread are named. `framework/constitution.md` was read in §spec-lifecycle, §implement-phase, Frontmatter Schema and Validation Severity — the four regions this spec's criteria and this pass's corrections assert on — and not end to end; the same for `framework/commands/analyze.md` (its capture step, record-writing step and §Review state drift) and `framework/commands/audit.md` (the Family 19 entry). `scripts/audit/README.md` was read at its Family 19 line only. The two `.claude/commands/ductus/*.md` files are generated mirrors of sources that were not fully read either, so they are doubly uncounted. `specs/026-framework-self-audit/scenarios/family-31-review-block-agreement.md` is in scope as a **deletion**; it was read in full immediately before being removed, and is counted under 026's review rather than twice.

**Passes.** Security, reuse and efficiency had no subject — prose corrections and one deletion. Quality, against `quality-cross.md`, is where this pass earned its keep, and the finding it produced is about the *sweep's* instrument rather than about code: an enumeration is a claim (§drift-prevention), and a classifier that cannot fire on a whole class of hit is `QUAL-CLAIM-001` wearing a regex — a clean result from it is evidence about its own pattern and nothing more. That is why this spec needed a second reopen and why the boundary is now written down. Simplicity: the section-level annotation is the smaller edit than re-pointing eleven sentences, and it keeps the record of what 047 delivered intact.

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
