---
section: "Follow-on scenarios"
---

# Report-outcomes-not-edits

## Context

Working a spec's tasks, an agent narrated its edits in its chat replies: it described each addition and modification, and each file write rendered the full new content into the session. The operator's objection was about the channel, not the accuracy — *"don't show me the additions/modifications you are making to the code as responses in the chat session. this only clutters the session history. i can see file additions/modifcations on disk. only show proposed changes if explicitly asked"* (2026-09-24).

The changes already have a home the operator reads directly: the working tree, the diff, and the commit. Restating them in the reply duplicates that home into a channel that grows with every task and is never re-read, and the length buries what the reply is actually for — what happened, what passed, what failed, and what the operator needs to decide next. A long session becomes hard to navigate precisely at the points where the operator most needs to find a decision.

The rule is universal rather than this repository's: every project running the pipeline has an operator reading the same session, so it belongs in the constitution rather than in a contributor guide ([§drift-prevention](../../../framework/constitution.md#drift-prevention), *Shared knowledge stays in git*).

## Behavior

[§pipeline-boundaries](../../../framework/constitution.md#pipeline-boundaries) carries a bullet stating that an agent **reports outcomes, not edits**. While working, its chat replies do not reproduce the code or artifact additions and modifications it makes. They report at the level of outcomes: which files changed, what a check or test returned, what was committed, and what the operator has to decide. A proposed change is shown only when the operator asks to see it.

The bullet states the reason with the rule — the operator reads the changes on disk, and a reply that restates them buries the outcome it exists to report — because a rule stated without its reason reads as a style preference and is the first thing dropped under pressure.

## Edge Cases

- **The operator asks to see a change.** Show it. The rule governs what is volunteered, not what is refused.
- **A command's own approval gate.** A gate that asks the operator to approve content before it is written — `/{project}:amend`'s preview, a proposed question resolution in `/{project}:clarify`, a plan summary before `planned` — is the operator asking. Show what approval requires, concisely, and offer the full text rather than pasting it by default.
- **A failure.** Quote the failing output that explains it. An error message is an outcome, not an edit.
- **Naming a location.** Citing a file, a function, or a `path:line` is not reproducing a change; it is how an outcome says where it happened.
- **Durable artifacts.** The rule covers chat replies only. Commit messages, `review.md`, `analysis.md`, and spec and plan prose describe changes fully, because they are the records a later reader actually consults.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
