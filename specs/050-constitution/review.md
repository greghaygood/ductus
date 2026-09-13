---
spec: 050-constitution
reviewed-at: 2026-09-13T17:56:30Z
reviewed-against: 4ef49b83fa005373827f63d6cda70d1a2b6fe9b7
diff-base: d0256337cb318486c91838b765cd3f9208e9dca5
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 8
scope: 8
skipped-passes: []
---

# Review — 050-constitution

## Summary

Five passes over 8 of 8 in-scope files — nothing unread. Scenario back-edge: `a-canonical-source-is-pointed-at-not-copied` states under §drift-prevention's *Canonical sources* that referencing means a pointer and that a reproduction is never one, with both reasons and the back-edge consequence.

**Why the rule was needed at all.** The existing MUST — "reference the canonical source rather than restate it" — is satisfied on its face by a verbatim copy, because a copy is not a restatement. The gap was not theoretical: 020's §Embedded artifacts is a frozen copy of `framework/commands/review.md` running lines 242–802 of an 802-line spec, against a live source now at 894 lines. What made it a rule rather than a fix is that **nothing could report it** — the snapshot sits in a code fence, unreachable by every link check, anchor resolver and audit family, and a scan for the next `##` heading stops 540 lines early on a heading belonging to the copied file.

**Scope, measured before choosing.** Post-reopen natural base `d0256337` resolves 8: the union of 050's five plan-affected files and the six this pass modified. No override was taken — the natural base is already the tight one after a reopen, and `--since HEAD` would have excluded this pass's own edits, which are the substance of the change. Every in-scope path exists; none is absent.

**Criteria re-verified against the tree, enumerations included.** All nineteen hold, and four were checked specifically because this change could have falsified them. **AC5** (promoted rules worded for an adopter): the new text cites only the constitution, git history and generic tooling categories — "link check, anchor resolver, structural audit" — and names no path that exists only here, which is what the reword test requires. **AC10** (anchor set undisplaced): the rule is a paragraph inside an existing subsection, so no `<!-- §anchor -->` moved; `resolve-anchor` returns all eight of the scenario's references resolved, `unresolved: []`. **AC12** (the version pin does not move): `version`, `runtime/Cargo.toml` and `runtime/CHANGELOG.md` are untouched and Family 20 is clean — a constitution-only change reaches adopters by the Shared Files manifest row, not by a tag. **AC3** (a mirror states nothing normative of its own): the first draft of the `AGENTS.md` entry duplicated the constitution's back-edge clause; that clause was removed and replaced with a pointer to the section, so grepping the rule's phrasing finds one statement and one mirror.

**050's own §Trade-offs read as live claims.** All five still hold, and this change conforms to two of them rather than contradicting them: "promoted rules lose their war stories" — the constitution gets the adopter-neutral rule and the `AGENTS.md` mirror keeps the 020 incident; "bullets are less citable than sections" — the rule landed inside §drift-prevention rather than as a new section, per the clarify walk's resolution.

**Rule set.** 11 files loaded. Every one verifies against code paths, specs or plans that introduce shared state, outbound calls, config values, metrics, API surface or UI — this change introduces none of those; it is governance prose in a document. No rule's Verification trigger fires. The passes are recorded as run and empty rather than skipped: the reuse pass is the one with a real surface here and it is what caught the duplicated back-edge clause, fixed before this record was written.

**One observation, captured rather than swept.** 045's spec names an adopter project twice, and the inbox item tracking that pattern does not list 045 or 046 — its enumeration is exactly the `anvil` spread and correct for that name, while the second name was never measured. Recorded as an observation so it reaches the inbox in this same call; not swept, because 045 and 046 each take their own back-edge, and not re-measured, because routing is `/groom`'s.

**Verification at this HEAD.** Full CI surface, because the constitution reaches two parity goldens (`implement-basic`, `target-basic`) and that was checked rather than assumed: 20 test binaries invoked and 20 reported, 0 failures, no golden re-bless needed; clippy `-D warnings` and `cargo fmt --check` clean; six `lint-*.sh` green; the 37-family self-audit green; markdownlint over 513 files. `derive-dependencies` reports no drift — the scenario's link to 020 induces no edge, confirmed by running it rather than by assuming scenarios are exempt.

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

- convention: the adopter-project-name item enumerates only one of the two names in the corpus, so two specs are outside its routing. Measured 2026-09-13 during 050's review: that item's spec-prose tier lists 000, 001, 002, 017, 022 and 036, which is exactly the `anvil` spread and is correct for it. The second name, `svc-zmc-api`, was never measured — it lives in `specs/045-decision-state-drift-detection/spec.md` (lines 42 and 172, the observed-case table and the Prior art section) and in `specs/046-scenario-open-question-visibility/`, neither of which the item names. 022 overlaps both. This is the enumeration-drift class AGENTS.md Gotchas records: the item's own count is a claim, and a sweep run against its list passes cleanly over 045 and 046. Not re-measured here beyond confirming which directories carry each name, and not swept — 045 and 046 each take their own back-edge, and the routing is /groom's call. The fix is to widen the existing item's measurement rather than open a second one — `specs/045-decision-state-drift-detection/spec.md:42`

## Skipped passes

*None.*

## Unexamined governance

*None.*
