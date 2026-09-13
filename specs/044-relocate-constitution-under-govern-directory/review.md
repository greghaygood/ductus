---
spec: 044-relocate-constitution-under-govern-directory
reviewed-at: 2026-09-13T13:26:22Z
reviewed-against: b421c51e6f629b5abedd4ac249cc2f0bd48963c0
diff-base: b421c51e6f629b5abedd4ac249cc2f0bd48963c0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 9
scope: 15
skipped-passes: []
---

# Review — 044-relocate-constitution-under-govern-directory

## Summary

First review of 044 to record `examined` against a derived `scope`; the prior record predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. **All ten criteria verified against the tree and every one holds** — no correction, so 044 never left `done`.

**What this review read: 9 of the 15 files in scope, and here is the other 6.** Read in full: `framework/bootstrap/ductus.md`, `framework/commands/analyze.md`, `groom.md`, `specify.md`, `target.md`, `framework/migrations.toml`, `framework/migrations/constitution-relocate.md`, `README.md`, and `AGENTS.md`. **Not examined:** `framework/commands/clarify.md` (read only at the lines naming a constitution path), the three `framework/templates/project/` seeds — `agents.md`, `claude-md.md`, `project-readme.md` — each read only at its constitution reference, which is the whole of what AC2 asserts about them, and `runtime/CHANGELOG.md` and `runtime/Cargo.toml`, which the plan lists for the version bump rather than for behaviour.

**One thing that looked like a defect and is not, worth recording so the next reader does not re-derive it.** `migrations.toml`'s `constitution-relocate` summary and its whole procedure name the destination as **`.govern/constitution.md`**, while AC4 and the live manifest both say `.ductus/constitution.md`. That is correct, not drift: `constitution-relocate` (`introduced_in = "0.24.0"`) writes `.govern/`, and `ductus-rename` — later in the registry's `introduced_in` ordering — carries `.govern/` in its `target_paths` and converges it to `.ductus/`. Rewriting the earlier procedure to say `.ductus/` would claim it writes somewhere it does not and would strand an adopter part-way through the chain. This is §drift-prevention's load-bearing exception verbatim: a migration procedure whose subject *is* a move must name both sides to stay auditable. AC4's claim is about what a `/ductus` **run** produces, and a run applies the whole chain, so it holds.

**How each criterion was checked.** AC1: the manifest's `update`-strategy destinations were scanned for root-anchored paths — exactly one survives, `.markdownlint-cli2.jsonc`, which is the criterion's own stated exception, and the constitution lands at `.ductus/constitution.md`. AC2: `claude-md.md` carries `@import .ductus/constitution.md`; `agents.md` and `project-readme.md` link `.ductus/constitution.md`, the latter in both its Documentation list and its pipeline deep-link; no seed names a root `constitution.md`. AC3: the command bodies read `.ductus/constitution.md`, and `analyze.md`'s anchor-resolution step states both sides explicitly — `framework/constitution.md` in ductus's own repo, `.ductus/constitution.md` at an adopter root — which is the half of the criterion that is easy to leave half-done. AC5, AC6, AC7: the procedure's steps 3–5 carry the pin re-pointing, the per-seed rewrite with a named warning for a hand-altered reference and silent skip for an absent file, the pinned-command warning, and the convergence rule that deletes a stale root copy in both the identical and divergent cases rather than leaving one behind. AC8: `introduced_in = "0.24.0"`, no `sunset_after`, with the reason recorded inline, and `procedure_file` resolves. AC10: grepping the live artifacts for an adopter-root `constitution.md` returns only spec-055's shared-constitution usage — where `constitution.md` is the filename *inside a registered checkout*, a different subject entirely — plus one unit-test string in `parse_affected_files` that exists to prove a qualified table cell parses. Neither is a stale adopter path.

**AC9 is satisfied vacuously, and that is stated rather than left to the count.** It claims the runtime's parity and golden fixtures "that encode the shipped constitution destination" reflect `.ductus/constitution.md`. No fixture or golden encodes that destination at all — the `ductus-basic` fixture is shaped like *this* repo (`framework/constitution.md`), and no golden asserts a manifest destination for the constitution. So nothing contradicts the criterion and nothing confirms it either; what the suite does prove is the second half, that parity passes, verified this session across 20 test binaries. A criterion true because its subject is empty is not the same as one checked, and the distinction is the campaign's own subject.

**On the diff base.** No commit records 044 entering `in-progress`, so the natural derivation is empty and the denominator would collapse to `scope: 0`. `HEAD` is passed instead. 044 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
