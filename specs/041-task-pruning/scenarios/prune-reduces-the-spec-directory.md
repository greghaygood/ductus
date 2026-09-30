---
section: "Behavior"
---

# Prune-reduces-the-spec-directory

## Context

This spec made `tasks.md` the whole of prune's scope and then promoted that to an invariant. Its *Scope confirmation* resolution calls single-artifact scope "a hard boundary" and names `plan.md` among the files prune never touches, reasoning that reconciling the plan "requires interpretive judgment a mechanical hygiene command must not do". That was right for what prune was: a deterministic reduction of an artifact holding nothing durable.

`plan-records-the-design-as-it-stands` changes the premise. A plan can now carry content that does not belong in it, and some of that content is durable — in the 3,622-line plan that scenario measures, the only true statement of two design decisions sat in sections outside the design record. So a plan needs the reclaiming `tasks.md` already gets. But a reduction that simply dropped those sections would destroy the corrections, and "recovery is git history" is no recovery for a fact the design record now contradicts.

## Behavior

**`/{project}:prune` reduces every prunable artifact in the targeted spec directory, each by the rule its durability class allows:**

| Artifact | Class | What prune does |
| --- | --- | --- |
| `tasks.md` | Ephemeral | Drops spent sections (keep-pending) or resets (`--reset`), exactly as today |
| `plan.md` | Design record | Removes each section outside the design record, after its durable content has been moved home |
| `spec.md`, `scenarios/*.md`, `data-model.md` | Durable | Never touched |
| `research.md` | Reference | Never touched |
| `review.md`, `analysis.md` | Regenerated | Never written — each run rewrites its own body and prunes its own stale decisions; prune reads only `analysis.md`'s stored decisions, below |

Anything else in the directory is the project's, not the pipeline's: prune neither reads nor reports a file outside this table.

**A plan section is removed only once it has been emptied of what lasts.** For each section outside the design record, the host reads it and proposes, piece by piece, where each durable piece goes: a changed decision into the Technical Decisions entry it amends, edited in place; contributor knowledge into `AGENTS.md`; owed work onto its pending task in `tasks.md`. It names what is dropped because git already holds it. The operator confirms a section's moves and its removal together; the host writes the moves, and the runtime removes the section. Knowledge that holds for every project is named for the operator to route to the constitution — prune writes no shipped artifact.

**A section analyze has already decided is not proposed again.** When `/{project}:analyze`'s plan advisory for a section has been discarded with its reason, prune skips that section and lists it in the preview as *kept — decided in `analysis.md`*. The advisory keys its finding `{family} — {message}`, as every analyze finding is keyed, and one runtime function builds that message from the section heading for both the advisory and prune, so the match is exact and needs no judgment. A stored decision whose key no longer matches — analyze re-matches a reworded finding by judgment and keeps its original key — is matched by the host the way `/{project}:analyze`'s stored-decision step matches it, before the section is proposed. Prune reads `analysis.md` and never writes it. Declining a section at prune's own prompt skips it for that run only, and the prompt says so: the lasting *keep* is an analyze discard, because that is where the decision already lives, and one decision should not need two records.

**The deterministic half stays in the runtime and the judgment stays with the host** ([§runtime-boundary](../../../framework/constitution.md#runtime-boundary)). The runtime segments `plan.md` into sections, classifies each against the design-record set, reports sizes in the preview, and performs every removal atomically, so no file body round-trips through context for a write — the guarantee AC11 already makes for `tasks.md`. Which pieces of a section last, and where each goes, is semantic judgment, so it is the host's, confirmed by the operator before anything is written.

**Preview first, then a confirmation per write.** The preview reports each artifact's classification and its size before and after, so the whole reduction is visible before any of it happens. The run reports *nothing to prune* only when no artifact has anything to reduce.

**On a `done` spec, prune reopens it exactly when the diff changes the design record or adds owed work** — a trigger read from the diff alone, as [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle)'s mechanical test requires. A diff that changes any design-record section of `plan.md`, or adds an unchecked task to `tasks.md`, takes the `done → in-progress` back-edge: the first corrects what the plan asserts, and the second leaves a `done` spec with work outstanding, the state `/{project}:amend`'s reconcile pass reopens for. Prune performs the reopen itself with the status primitive's guarded write, naming it at the confirmation before anything is written, the way `/{project}:groom` and `/{project}:amend` name theirs. Anything else is mechanical and the spec stays `done`. Removing a section whose heading is outside the design record is visible as such in the diff, and it changes no claim, because only the design record carries a plan's claims (`plan-records-the-design-as-it-stands`) and the move step has already carried anything durable home. A spent `tasks.md` section is visible as spent by its checkboxes. A move to `AGENTS.md` does not touch the spec at all. The reopen costs a fresh `/{project}:analyze` and the completion gate back to `done`: `plan.md` and `tasks.md` are not durable contracts, so the review record stays valid. A spec below `done` is never moved.

**This spec's body is corrected to match.** Implementing this rewrites the *Scope confirmation* resolution it supersedes, the Behavior section's "the command's scope is `tasks.md` only", AC1's `tasks.md`-only wording, and `framework/commands/prune.md`'s Scope Boundaries — including its promise that prune never changes pipeline status, which becomes *prune changes status only to reopen a `done` spec, and names the reopen first*. The title broadens from *Task Pruning*; the directory slug stays, because a rename is a corpus sweep that buys nothing here.

## Edge Cases

- **A plan with no section outside the design record.** Prune reduces `tasks.md` alone, as it does today.
- **A section whose content is all spent** — evidence, pass counts, scratch paths. Nothing moves, and the section is removed after confirmation.
- **A declined section.** It stays byte-for-byte, and the rest of the reduction proceeds. It is proposed again on the next run unless an analyze discard records it as part of the design.
- **A spec whose analysis predates the plan advisory, or that has none.** There is no stored decision to honor, so every section outside the record is proposed; declining stays run-only until an analyze records a discard.
- **A design section under another heading: keep it, or fold it.** Keeping it by an analyze discard writes nothing. Folding it into a record section — a *Known limitations* into Trade-offs, where `/{project}:plan` already puts known limitations — changes a record section, so on a `done` spec it reopens. Both are legitimate, and the section's confirmation names the reopen a fold would take before the choice is made.
- **A moved decision that contradicts the entry it lands in.** This is the case the rule exists for: the entry is rewritten to state what is true now, and on a `done` spec the rewrite reopens it.
- **A reduction that only removes sections outside the design record from a `done` spec's plan.** The diff touches no record section and adds no task, so the spec stays `done` — including when pieces of those sections moved to `AGENTS.md`.
- **The status changed between preview and write** — a concurrent edit reopened or closed the spec. The guarded status write refuses on the stale value and the run stops, naming what it found, rather than overwriting it.
- **A move is written and the section's removal then fails.** The moves are written first and the removal last, so a failure leaves the content in two places rather than in none — the safe direction. The next run finds the section again and proposes it afresh against the entry that already holds its piece.
- **A spec with no `tasks.md`.** When it has no plan either — the `draft` and `clarified` case, since `/{project}:plan` writes both — prune stops and directs to `/{project}:plan`, as AC10 does today. When a plan exists without a task list, prune reduces the plan and reports the missing task list instead of stopping.
- **A file in the spec directory that is not a pipeline artifact.** Left alone and unreported. Measured over the 120 spec directories in this repository and the local adopter corpora, exactly one such file exists — a migration spec's extraction inventory, kept deliberately.
- **`--reset`.** Unchanged: it resets `tasks.md` and never touches the plan.
- **An in-flight task's handoff notes found in a plan section.** They move onto that task in `tasks.md`, where keep-pending preserves them until the task is checked.

## What this does not do

It does not walk other specs — that is `prune-all-walks-every-spec`. It does not decide which sections form the design record; `plan-records-the-design-as-it-stands` does. The runtime half is a sibling primitive, `prune-plan`, owned here as `prune-tasks` is: its segmentation and schema go in this spec's `data-model.md` beside the `tasks.md` ones, and nothing is registered in `022-deterministic-runtime`.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What does a plan reduction cost on a `done` spec, and who pays it?** Prune reopens the spec itself when the diff changes a design-record section of `plan.md` or adds an unchecked task to `tasks.md`, naming the reopen at the confirmation, and treats anything else as mechanical — option (a) of three, the others being that prune never touches status and the operator reopens by hand (b), and that the whole reduction counts as mechanical (c). What decided (a) is that nothing else would ever notice: analysis freshness is checked only at the `in-progress → done` gate, which short-circuits for a spec already `done` (`runtime/src/primitives/check_review_gate.rs`, `already_done_block` and its doc comment), so under (b) a `done` spec whose design record had just changed would carry that change with no check able to see it. (c) contradicts [§spec-lifecycle](../../../framework/constitution.md#spec-lifecycle), whose mechanical test excludes a factual correction by name. **The trigger was corrected the same day it was first recorded.** Its first wording reopened when a move "changes what the spec claims or owes" and kept removing an "all-spent" section mechanical — both judgments, where §spec-lifecycle's test requires a call determinable from the diff without author judgment. Tested against that test, the diff-visible form above gives the same outcome on every section worked through in `prune-all-walks-every-spec`'s resolution, and a check can compute it. The reopen is cheap — `plan.md` and `tasks.md` are not durable contracts, so the review record stays valid and the cost is an analyze plus the completion gate. Resolved 2026-09-29.
- **Does prune report files in the spec directory that are not pipeline artifacts?** No — prune neither reads nor reports them; option (c) of three, the others being to list them in the preview (a) and to offer each for removal (b). The question was drafted on the premise that a scratch or handoff file left beside the artifacts is the same accretion one level up, and the measurement did not support it: across the 120 spec directories in this repository and the local adopter corpora, a scan for any file other than the seven artifacts and `scenarios/*.md` found exactly one, a migration spec's extraction inventory, which is a deliberate supporting artifact. Even the 3,622-line journal that motivated this work accreted inside `plan.md`, not beside it. (a) was the draft's recommendation and is withdrawn: prune stores no decisions, so a legitimate file would be reported on every run with no way to settle it. (b) puts a destructive prompt on files the pipeline does not own, with no observed case where removal was right. A pattern of stray files, should one appear, is a finding to route when it is observed. Resolved 2026-09-29.
