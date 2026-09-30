---
section: "Follow-on scenarios"
---

# Exec-analyze-derives-its-list-seeds

## Context

`ductus exec analyze` seeds its walker context from the session file, overlaid with `key=value` arguments bound as strings (`run_exec` in `runtime/src/main.rs`). A session written by `write-session` carries the target alone: `feature`, `path`, `set-at`, and a scenario pair when one is targeted. Since [058](../../058-findings-route-at-discovery/spec.md)'s task 41 bound the spec-reading steps to the spec file when `path` names the spec directory, the walk got past them and stopped at step 5 with `missing field rule-files`. Seeded by hand, it stopped next at step 7 with `missing field paths`. Both are lists, and a string argument cannot supply one, so no command line could complete the walk. `analyzed-at` and `analyzed-against` are strings a caller can pass as arguments; without them the walk stops at step 20, naming the field.

The parity fixtures did not show this, because `analyze-basic`'s session file seeds both lists itself, which no session `write-session` writes does. Reproduced at 39856db6 on a copy of that fixture with its session replaced by a `write-session`-shaped one: exit 1 at step 5; with `rule-files` seeded, exit 1 at step 7; with both seeded, the walk completes and writes `analysis.md`. Found while implementing 058's task 41 and routed here by 058's task 43, since exec's seed contract is this spec's.

## Behavior

- **`rule-files`.** When the context carries none, exec binds every rule file in the project's rule-file directory. `/{project}:analyze` loads all of them whatever the project's surfaces, because citation resolution spans surfaces, so the surface filter `discover-rule-files` applies for review does not apply here.
- **`paths`.** When the context carries none, exec binds the feature directory's markdown files, the subject step 7 names.
- **A seeded value wins.** A session or argument that supplies either list is used as given, so the fixtures and their goldens do not move.
- **The timestamps stay the caller's.** `analyzed-at` and `analyzed-against` are host-provided by contract, as `write-review`'s `reviewed-at` is. A missing one still fails by name at the step that needs it, never defaulted.

## Edge Cases

- **No rule files.** An absent or empty rule-file directory binds an empty list, and `check-rule-ids` reports `examined: 0`, which step 5 already tells the host to read as nothing checked rather than every citation missing.
- **A scenario-targeted session.** The feature directory stays the lint subject; the scenario pair in the session does not narrow it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
