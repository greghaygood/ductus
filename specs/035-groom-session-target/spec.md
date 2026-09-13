---
status: done
dependencies: [006-bug-workflow, 017-derive-dont-ask, 023-govern-refinement]
review:
  last-run: 2026-09-13T17:41:03Z
  reviewed-against: 3141463ab739f72882171628484f41bf797f8aee
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  examined: 4
  scope: 4
  reviewed-digest:
    scenarios/confirmation-names-reopen.md: 62f7f04097775cd8b7e33dc38cb05b47dcbef834e6b4eb54d60bf1bf1ead7829
    scenarios/reopen-done-spec-on-scenario.md: 5f1b1f7cc9a4fb3d5042d48ef2aae191697ff3821a0170ead1d019460cd6d6b2
  blocking: false
next-criterion: 8
analyze:
  last-run: 2026-09-13T17:41:51Z
  analyzed-against: edfa4d594b5dab80ab244ff3a1eef6edacbd8933
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 0
  analyzed-digest:
    plan.md: 34952826f813b3239d9607cb08d8f65bf791e8e4e4575ecd03491e36dfcce998
    review.md: 1e4561bfa231bb97f7374a43cfd0c0920018655d3d481dd6f8b8a89c8fba8368
    scenarios/confirmation-names-reopen.md: 62f7f04097775cd8b7e33dc38cb05b47dcbef834e6b4eb54d60bf1bf1ead7829
    scenarios/reopen-done-spec-on-scenario.md: 5f1b1f7cc9a4fb3d5042d48ef2aae191697ff3821a0170ead1d019460cd6d6b2
    spec.md: 1bf5c6cb5ceed000c6728acc75b8886844b408e9adb2eb1a3872b3d671a84207
    tasks.md: 562abf7c9095b0adb3e6ae7f0d25390782167d9f4fcbe5926c1171a4661be9a1
  blocking: false
---

# 035 — Groom sets the session target from the routed item

`/ductus:groom` sets `.ductus/session.toml` to the spec it routes an inbox item to, so a follow-on `/ductus:amend` or `/ductus:implement` operates on the right target without a manual `/ductus:target`.

## Motivation

`/ductus:groom` walks the inbox and, for each item, finds the matching feature by searching `specs/` (the [006-bug-workflow](../006-bug-workflow/spec.md) decision tree, §bug-handling) and routes it — a spec edit (Step 3) or a scenario under the matching spec (Step 4). But groom never writes the session target: its Context says a target "is not required" because it operates across all specs. The consequence is friction at exactly the moment the spec is known — groom has *just identified* the right feature, yet the operator must remember it and run `/ductus:target NNN` by hand before the follow-on `/ductus:amend` or `/ductus:implement`.

This is the same "don't make the operator remember session state" gap that self-contained inbox items only half-close: the item names its target, but nothing carries that target into the session. This spec carries it the rest of the way — groom sets the target as part of the routing it already performs.

## Behavior

- When groom routes an item to an **existing spec** — a spec edit (Step 3) or a scenario created under the matching spec (Step 4, durable-requirement branch) — it sets `.ductus/session.toml` to that feature as part of the routing action. The target is the feature the decision tree matched in Step 2 (reinforced by, but not dependent on, any `specs/NNN-*/` link in the item text).
- The per-item routing confirmation groom already requires before acting now **names the target it will set** — e.g., *"Create a scenario under `033-rule-surface-setting` and set it as the session target? (Y/n)"*. That single confirmation is the consent for both the routing and the target write; no separate target prompt is added (consistent with the procedural-fidelity / don't-add-prompts stance). The operator sees and confirms the target without having to recall it.
- **New-spec items** (Step 2, no spec exists → `/ductus:specify`) are unchanged: `/ductus:specify` already targets the spec it creates.
- **Rule items** (Step 1, amend a rule file) and **chores** (Step 4 chore, done in the pass and then removed) set no target — neither has a single spec home.
- Across a multi-item run, the session target **follows the current item**: each spec-routed item sets it, so when the run ends the target points at the most recently groomed spec (the one the operator is most likely to act on next).
- The session write **preserves any existing `cli-config-dir`** (the per-contributor agent identity), using the same `write-session` target-write semantics from [023-govern-refinement](../023-govern-refinement/spec.md); it must not be dropped.
- The completion summary names the resulting session target (or states it is unchanged when no item set one).

The change is confined to `framework/commands/groom.md` (and its generated `.claude/commands/ductus/groom.md` copy).

## Acceptance Criteria

- [x] AC1: When groom routes an item to a spec edit (Step 3) or a scenario under the matching spec (Step 4 durable branch), it sets `.ductus/session.toml` to that feature.
- [x] AC2: The per-item routing confirmation names the target it will set; groom adds no separate "set the target?" prompt.
- [x] AC3: New-spec items, rule-file items (Step 1), and chores (Step 4 chore) do **not** set a session target via groom.
- [x] AC4: Across a multi-item run, the session target follows the current item (the last spec-routed item is the final target).
- [x] AC5: The session-target write preserves any existing `cli-config-dir` value.
- [x] AC6: The completion summary names the resulting session target (or states it is unchanged when no item set one).
- [x] AC7: `framework/commands/groom.md` documents the behavior, and its generated `.claude/commands/ductus/groom.md` copy regenerates cleanly.

## Resolved Questions

- **Auto-set vs. explicit prompt.** Resolved: **no separate prompt.** The per-item routing confirmation groom already requires now names the target it will set, so that one confirmation is the consent for both the routing and the target write. The operator is still shown and confirms the target (satisfying "prompt for the session target") without a redundant second prompt — consistent with [017-derive-dont-ask](../017-derive-dont-ask/spec.md) and the procedural-fidelity stance.
- **Multi-item groom run.** Resolved: the target **follows the current item** — each spec-routed item sets it, so the run ends with the target pointing at the most recently groomed spec (the most likely next action). Not set-once.
- **Step 3 spec-edit vs. Step 4 scenario.** Resolved: **both.** Any routing to an existing spec sets the target; the operator's next command applies equally to a spec edit or a newly-created scenario.
- **Completion-summary detail.** Resolved: the summary names the **final** session target (the most recently set), and states "session target unchanged" when no groomed item set one. A per-item trail is unnecessary noise.
