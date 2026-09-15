# 050 — Constitution Plan

Implements [050 — Constitution](spec.md).

## Overview

Three pieces of work over two files. Classify every rule-bearing `AGENTS.md`
entry, move the universal ones into `framework/constitution.md` leaving pointers
behind, and add two rules that were learned here but belong to every adopter —
criterion verification and the mechanical-edit test.

The work is almost entirely prose, and the risk is not that it breaks but that
it drifts: two copies of a rule, a promoted rule that still names this
repository's machinery, or a moved anchor another artifact cites. The plan is
shaped around making each of those checkable rather than careful.

## Technical Decisions

### The classification is written down before anything moves

The survey behind this spec was a judgment call recorded as three counts. AC1
asks for something stronger: every entry carries a classification *and* the
reason. So the first task produces the full table — one row per rule-bearing
entry, with its verdict and a one-line reason — and nothing is promoted until
that table exists.

The table lives in this plan rather than in `AGENTS.md`, per the clarify walk's
fourth resolution: a per-entry marker in `AGENTS.md` would add authored state to
every future entry, which §design-principles rejects as a design that depends on
author diligence.

Scope is the four rule-bearing sections — `Workflow`, `Gotchas`, `Boundaries`,
`Design Principles`. `Project Structure` and `Tech Stack` are descriptive prose
about this repository, not rules, and are out of scope. As of this plan those
sections hold 34, 16, 2 and 2 entries respectively; the pass runs against the
file as it stands at implementation time, not against those numbers.

### The reword test is applied, and its output recorded

The borderline group is decided by the test the clarify walk chose: an entry is
promoted if and only if it can be restated without naming machinery that exists
only here, *and* without losing what makes it actionable. Both halves matter —
dropping the second turns the test into "can it be made vague enough to ship".

The recorded reason is what makes it auditable: for a promoted borderline entry,
the reason names the machinery removed and asserts the rule still bites; for a
rejected one, it names what was lost. `scripts/audit/`, `runtime/`, the agent
registry, the `ductus-v*` release loop, the retired project name and the cargo
and rustup gotchas are all repo-only surfaces — an entry resting on one of them
fails the first half unless the rule survives its removal.

### Promotions land as bullets, in the section that already owns the subject

Per the clarify walk's second resolution. Each promoted rule is a bullet under
the existing section whose subject it shares — a spec-lifecycle rule under
§spec-lifecycle, a frontmatter rule under §text-first-artifacts, an
artifact-integrity rule under §drift-prevention — and a new section only when
the rule genuinely stands alone, as §recommendations did.

This is also what keeps AC10 cheap. The constitution carries 40-odd anchors of
the form `<!-- §name -->` that other artifacts cite by name; appending a bullet
inside a section moves none of them, while promoting a dozen rules to sections
would roughly double the top-level surface and put one-paragraph rules at
section rank.

### The mirror points and does not restate

A promoted entry's `AGENTS.md` line is rewritten to name the constitution
section and say nothing normative of its own — the shape §recommendations
already uses ("The rule is §recommendations … this entry is the contributor-side
mirror, not a second copy"). Two copies of a rule is precisely the drift
§drift-prevention exists to prevent, and a mirror that restates is two copies.

The check for this is mechanical and worth stating as one: grepping a promoted
rule's distinctive phrasing must find exactly one normative statement and one
pointer (AC3).

### Two rules are authored rather than promoted

**Criterion verification** (AC6) is not in `AGENTS.md` at all — it was learned
after the survey. It is universal for a structural reason: `check-artifacts`'
`criterion-path-existence` family examines specs at `done` only, so a spec that
sits in progress indefinitely never has its criteria examined, and the family
proves only that a path *resolves* — a criterion whose paths all exist while its
claim about their contents is false is invisible to it. Both halves were
observed here on 2026-08-17. Adopters run the same pipeline and inherit the same
blind spot.

**The mechanical-edit test** (AC13) settles what §spec-lifecycle's three
enumerated cases leave open: whether an edit that changes no claim is mechanical.
Today the list reads as closed, so a pure typo repair in a `done` spec's body is
argued to reopen it — which is the disproportion the held `045` chore in
`specs/inbox.md` is waiting on. The section gains a stated test rather than a
fourth enumerated case, so the next new case is decidable without another
constitution change.

### The version pin does not move

AC12, and it is a constraint rather than a deliverable. The repo-root `version`
file is the runtime acquisition pin: `/ductus` reads it to choose which release
to fetch, and `/ductus:audit` Family 20 requires it to equal
`runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading
(`scripts/audit/version-agreement.sh`). Bumping it alone fails that family;
bumping all three declares a runtime release that must be tagged `ductus-v*` or
every adopter fetches assets that do not exist. The constitution reaches
adopters by the **Shared Files** manifest row copying
`framework/constitution.md` to `.ductus/constitution.md`
(`framework/bootstrap/ductus.md:714`), which tracks `main` — so no bump, and no
tag.

### Verification is the audit, not a reading

AC7 through AC10 are all checkable without judgment, and the plan leans on that
rather than on care: `/ductus:audit` Family 1 catches a promotion that
contradicts another document's claim, the manifest row proves adopters receive
the file, `markdownlint` covers AC9, and the anchor set is a grep. The
substantive risk the checks do *not* cover is AC5 — whether a promoted rule
truly holds for an adopter — which is why the reword test's recorded reason
carries that weight.

## Classification

Every rule-bearing entry, one verdict and one reason each (AC1). **23 to promote,
3 already promoted, 28 project-only — 54 total.**

**Entries are keyed by their `AGENTS.md` lead phrase, with the survey's positional
ID kept in parentheses as a historical marker.** The positional ID was the original
key and cannot stay one: it encodes where an entry sat on 2026-08-17, so inserting
a single entry anywhere above it renumbers every entry below, and `AGENTS.md` has
since grown from 54 rule-bearing entries to 119. Re-deriving the mapping therefore
needed `git show 9814d3ef^:AGENTS.md` rather than the file on disk. The lead phrase
is not a perfect key either, and the limit is stated rather than implied: **an entry
that is later reworded stops matching**, which is what five of these 54 had already
done (the entries now reading *never record anything durable in `review.md`*, *a
`.ductus/config.toml` change is cross-spec impact only when…*, *the generator rule
has no exceptions*, *the two frontmatter derivations are runtime primitives*, and
*never edit an installed command file directly*). Those five were re-matched by hand;
the other 49 matched mechanically. The key is chosen because it degrades from
*matches* to *needs one hand re-match*, where the positional ID degrades silently
to *matches the wrong entry*.

**Coverage, measured 2026-09-15 against `AGENTS.md` at 119 rule-bearing entries**
(Workflow 49, Gotchas 59, Boundaries 3, Design Principles 8): *unclassified =
rule-bearing − (table-keyed ∪ constitution-citing)* = 119 − 75 = **44** (Workflow 7,
Gotchas 34, Design Principles 3). All 54 table entries still resolve to a live entry,
and 47 in-scope entries cite a `framework/constitution.md#` anchor. Count the citing
side as **rule-bearing bullets in the four in-scope sections** — a bare
`grep -c` over the file answers 51, because it counts anchor-bearing *lines*
including two outside those sections, and 51 is right about lines while 44 needs 47.
This is a **notice, never a gate**: promotion has to stay free, or the honest choice
between a growing backlog and a silent one pushes toward silence.

`R` marks a verdict reached through the reword test rather than directly.

Two entries were reclassified after checking the constitution rather than the
survey: **W14 and W15 are already promoted.** They are cases (b) and (c) of
§spec-lifecycle's mechanical-edit rule, and both `AGENTS.md` entries already
cite the section by anchor — they were in the target shape before this spec
started. The survey counted them as pending because it read `AGENTS.md` alone.

That check also surfaced a defect this promotion fixes rather than causes.
§spec-lifecycle's back-edge paragraph cites "the **Design Principles** rule:
never depend on human diligence" — a rule that lives in `AGENTS.md`, which the
**Shared Files** manifest never ships. So the one artifact every adopter
receives cites a rule none of them can read. Promoting DP2 resolves the
dangling reference; DP1's substance appears at §recommendations as a supporting
clause rather than as a principle in its own right.

### Promoted — universal

| Entry | Reason |
| --- | --- |
| A spec does not reach `done` with an outstanding SHOULD — every review finding is resolved or waived first, not only the MUST ones (survey W4) | Every adopter runs the review command and inherits its blocking semantics; a spec sitting at `done` with an unaddressed finding is indistinguishable from unfinished work anywhere |
| Never record anything durable in `review.md` — it is regenerated wholesale (survey W6) | Specs, criteria and `review.md` are all adopter artifacts, and `write-review` regenerates the Summary wholesale for them too |
| Record a review's `reviewed-against` as a commit that already contains the artifacts it reviewed — so commit the work first, then review, then commit the review (survey W7) | **R** — drops the Family 19 citation; the ordering rule (commit, review, commit the review) stands on its own and adopters' freshness check reads the same field |
| A `.ductus/config.toml` change is cross-spec impact only when it modifies a key another spec already documented (survey W10) | Adopters add config sections per spec; the anti-pattern of reopening the config's originating spec is the same |
| No dead references in live artifacts (survey W11) | **R** — the artifact list is restated generically (specs, commands, rules, docs, README) rather than naming this repo's directories |
| A behavior change needs a prose-claim sweep, not just an identifier sweep (survey W12) | Identifier greps miss stale claims in any project; the failure mode is the wording, not the paths |
| Never hand-write an `AC{n}:` label — add the criterion unlabelled and run `label-criteria` (survey W16) | The counter is adopter frontmatter and the collision risk is identical |
| Use the `Write` tool, not Bash redirects, for `.ductus/session.toml` (survey W18) | **R** — stated as the session file rather than a permission-entry anecdote; adopters carry the same per-path grants |
| Re-open a `done` spec via `set-status` when the only intent is to reflect on-disk edits, not via `/{project}:amend` (survey W21) | The back-edge and the refinement loop are both adopter-facing |
| Add a rule to its surface's home spec via the back-edge — do not spawn a new spec for it (survey W22) | Adopters own rule files and hit the same spec-proliferation pressure |
| A canonical table living on a `done` spec is synced as a mechanical edit — no back-edge (survey W24) | Follows from the canonical-sources map, which adopters receive |
| A CI check that reads git history needs `fetch-depth: 0` (survey W27) | **R** — the shipped adopter CI template runs exactly such a check, so this is adopter-facing already |
| A new test that reads git history or shells out to a repo script is a change to the workflow that runs it, not just to the suite (survey W28) | **R** — same family as W27; stated as history/inputs rather than this repo's workflow filenames |
| When an item routes to a chore, fix it — do not park it (survey W29) | The groom command and its five routes ship unchanged |
| `git checkout -- specs/{feature}/` silently reverts uncommitted pipeline state (survey W30) | Status flips and ticked criteria are tracked-file writes in every adopter repo |
| Renaming this repository orphans contributor-local state that no migration can reach (survey W31) | **R** — generalised from this rename to any; no migration can reach state keyed to a path outside the repo |
| Before reporting that a declarative entry misbehaved, read the entry (survey W32) | Adopters run the migration registry and read its gating fields; this is §grounding applied to a data row |
| Never `git add -A` / `git add .` in this repo (survey G5) | The untracked-draft hazard is created by the pipeline itself, which adopters run |
| A markdown link in a spec body creates a `dependencies:` edge — cite in prose when you mean a citation (survey G11) | The dependency generator ships; citing versus depending is an adopter distinction |
| `create-scenario` appends its own Open/Resolved Questions scaffolding — do not author those headings in the body you pass it (survey G12) | An adopter-facing primitive with adopter-facing output |
| Never edit an installed command file directly — edit the source the generator copies from (survey B2) | **R** — stated as "the installer overwrites it; pin it instead" rather than naming this repo's generator, which is what makes it matter to an adopter |
| A check that cannot run must never look like one that passed (survey DP1) | Already `QUAL-CLAIM-001` in a shipped rule file; the design-time statement belongs beside it |
| Never design framework features that depend on human diligence or discipline (survey DP2) | The hardest constraint on any pipeline artifact an adopter authors |

### Already promoted

| Entry | Reason |
| --- | --- |
| Work a recommendation out to its result *before* presenting it (survey W5) | Promoted 2026-08-17 as §recommendations; the entry is already the pointer-shaped mirror this spec generalises |
| Criterion-label assignment is mechanical-class (non-reopening) (survey W15) | Adopters receive the labelling pass and the same back-edge question |
| Cross-service reference edits are mechanical-class (non-reopening) (survey W14) | Cross-service references ship to adopters; the non-reopening rule is part of that contract |

### Project-only

| Entry | Reason |
| --- | --- |
| Commit directly to `main` — this repo uses trunk-based development (survey W1) | This repo's trunk-based flow; adopters choose their own branching |
| Never create a repository named `govern` under this account again, and never reuse a retired project name (survey W2) | Concerns this project's distribution redirect, not an adopter's pipeline |
| A `runtime/` change ships via a `ductus-v<version>` tag — the commit alone reaches nobody (survey W3) | Adopters have no `runtime/` and cut no release |
| Never record another project's name in this repository — describe the shape instead (survey W8) | Arises from using outside projects to test this framework |
| Read `framework/commands/{name}.md` before recommending, describing, or disambiguating a s (survey W9) | Loses nothing an adopter needs that §grounding does not already state |
| A change to an agent-facing surface must cover every agent in the registry, not just Claude (survey W13) | The registry is the framework's own enumeration |
| Run `/ductus` per its spec — no ad-hoc prompts (survey W17) | Canonical statement already ships inside the installer itself; this is its mirror |
| Use repo-relative paths in tool calls (survey W19) | Agent tool hygiene rather than pipeline governance; genericised it stops biting |
| Never call specs "frozen archaeology" or use frozen-archaeology phrasing (survey W20) | The substance (specs are living documents) is already §spec-lifecycle; what remains is a phrasing ban local to this repo |
| Route runtime work to spec 022 via the back-edge — 022 is the durable home for runtime rules (survey W23) | Names this repository's own spec |
| Backtick a primitive in an `## Instructions` step only when the walker can actually supply its arguments (survey W25) | Concerns authoring framework command sources |
| Do not invoke `cargo build` twice in one shell command and read only the second output (survey W26) | A cargo-specific shell trap in this repo's toolchain |
| A real adopter run is the only test of composition this project has (survey W33) | About testing this framework against adopters |
| A run summary is evidence about what that agent wrote, not about what the framework does — verify every other claim against the source (survey W34) | Restates §grounding's prefer-the-source rule for a narrow case; the general rule already ships |
| Use `npx markdownlint-cli2` to run markdown linting — do not suggest installing it globall (survey G1) | Tooling preference, not a rule |
| The command generator substitutes `{project}` → `ductus` and `{cli-config-dir}` → `.claude (survey G2) | Framework build step |
| An edit to `framework/commands/*.md` is invisible to that slash command until the generator re-runs — including to the command you are editing, in the same session (survey G3) | Framework build step |
| The generator rule has no exceptions — every file under `.claude/commands/ductus/` is generated output, and one with no source is drift for the prune loop rather than a file to hand-maintain (survey G4) | Framework build step |
| The two frontmatter derivations are runtime primitives, not shipped scripts (survey G6) | Framework authoring |
| `write_atomic_bytes` (the create-then-rename helper backing every primitive write) lands t (survey G7) | Runtime internals |
| `runtime/rust-toolchain.toml` pins `components = ["clippy", "rustfmt"]`, and every `cargo` (survey G8) | Runtime build |
| A new `ductus` runtime primitive wires into more than `primitives/mod.rs` + `mcp/server.rs (survey G9) | Runtime authoring |
| `<!-- audit:ignore-promotion -->` silences the framework self-audit's Family-9 (primitive- (survey G10) | Framework command authoring |
| A version bump needs one build without `--locked` (survey G13) | Runtime release |
| Gitignored adopter state survives a `git` reset, so a "reset" subject is not a fresh one (survey G14) | About testing adopters, not being one |
| This repo's dogfooded copy of an adopter file is not the copy adopters get — test the shipped one (survey G15) | Structurally impossible for an adopter, who has only the shipped copy |
| `path` is a reserved array in zsh and clobbering it destroys `PATH` (survey G16) | A shell trap with no pipeline connection |
| No host-level enforcement of the deterministic (`ductus`) path — it is a deliberate non-goal (survey B1) | A framework design non-goal |

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Receives every promoted rule as a bullet under the section owning its subject, plus the criterion-verification rule, the mechanical-edit test in §spec-lifecycle, and a §drift-prevention canonical-sources row naming this spec |
| `AGENTS.md` | Modify | Each promoted entry rewritten to a pointer; project-only entries untouched |
| `specs/050-constitution/plan.md` | Modify | Carries the classification table — the audit trail AC1 requires |
| `specs/inbox.md` | Modify | The held `045` chore is resolved against the new mechanical-edit test and removed |
| `specs/045-decision-state-drift-detection/spec.md` | Modify | The one-word sweep-residue repair the chore describes, once the test licenses it |

## Trade-offs

**The classification table lives in a plan, which is a design record.** Plans are
not durable contracts, so nothing gates on this table staying accurate as
`AGENTS.md` grows. The alternative — a per-entry marker in `AGENTS.md` — was
rejected in the clarify walk because it depends on every future author
remembering to classify. Accepted limitation: the table is a snapshot of one
pass, and a later promotion round re-derives rather than amends it.

**Promoted rules lose their war stories.** `AGENTS.md` entries carry the failure
that produced them ("surfaced 2026-08-17 when…"), which is much of what makes
them stick. An adopter-neutral restatement cannot cite this repository's
incidents, so the constitution gets the rule and the mirror keeps the story.
Considered and rejected: shipping the incident text too, which would put
this project's history into every adopter's constitution.

**The reword test is a judgment, applied by one reader.** It is more auditable
than a vote — the reason is recorded per entry and can be disagreed with
specifically — but it is not mechanical, and two readers could classify a
borderline entry differently. Accepted: the alternative was deferring the whole
borderline group, which costs adopters ~10 substantively universal rules.

**Nothing verifies AC5 mechanically.** No check proves a promoted rule holds for
an adopter; a rule can pass every gate here and still read as advice about
somebody else's repository. The reword test's recorded reason is the only
defence, and it is a written argument rather than a check.

**Bullets are less citable than sections.** A rule promoted as a bullet has no
anchor of its own, so another artifact can cite only its containing section.
Accepted per the clarify walk: the alternative doubles the constitution's
section surface, and a rule that later proves worth citing directly can be
promoted to a section without moving any existing anchor.
