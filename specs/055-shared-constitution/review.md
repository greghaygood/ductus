---
spec: 055-shared-constitution
diff-base: 3c37c3ba5f288f44e296d79679b3f191bb16c395
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T21:48:06Z
reviewed-against: 3c37c3ba5f288f44e296d79679b3f191bb16c395
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 22
reviewed-digest:
  data-model.md: 7d54e3ee75399840c7e55932462ed7580b0e3c1f7c29c0ade4c0d1f00641e158
  scenarios/a-registered-source-is-named-with-its-description.md: 0150662e0bebc24665b91b0894dce653054754490b00fca6fb5628ff5c730205
blocking: false
---

# Review — 055-shared-constitution

## Summary

Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence, and **one observation** that maps to no loaded rule and is recorded rather than counted.

**Diff base.** Both measured before choosing. Natural base `1f6c6b7` (2026-09-12) — scope **261**, modified-since **255**. `--since HEAD` — scope **22**, modified-since 0. Took `HEAD`. This is the override case AGENTS.md §Gotchas describes: 055 needed no correction this pass, so there are no pass edits to keep inside the window, and the natural base's 255 files are the whole repo's churn since 022's last reopen rather than anything 055 owns. The 22 resolve to exactly the plan's Affected Files.

**examined: 5 of 22, counted strictly.** Read in full and counted: `runtime/src/primitives/resolve_constitutions.rs` (implementation through `classify`; its 260-line test module was not read), `runtime/src/schema/constitutions.rs` (same bound), `specs/055-shared-constitution/data-model.md`, `framework/constitution.md`, `framework/commands/target.md`.

Named rather than counted, with what I relied on instead: `framework/bootstrap/ductus.md` — read by section (§Project Configuration's `constitutions.<alias>` entry, §Validating the registry, §Shared constitution imports), not across 141KB, and every claim below cites one of those sections. `framework/templates/project/claude-md.md` and `agents.md` — their `ductus:constitutions` managed blocks were read and checked against AC9, the rest of each file was not. `framework/bootstrap/govern.md` — the transitional copy audit Family 21 holds byte-identical to `ductus.md`, so reading it would show the same bytes; that is a reason to believe it correct, not a read. `.claude/commands/ductus/*.md` — a glob of generated mirrors whose sources are in scope. The seven remaining runtime files (`main.rs`, `mcp/server.rs`, `interpreter/mod.rs`, `primitives/mod.rs`, `schema/{mod,primitives,registry}.rs`) are the registration sites; I confirmed the primitive is wired by calling it on both the MCP and CLI paths rather than by reading them, which is evidence about the wiring and not about those files.

**All twelve criteria verified against the tree**, not against the review that preceded them. AC1 by construction — an absent config and an absent table both yield an empty registry, so nothing downstream can tell them apart; confirmed live, `resolve-constitutions` returns all-empty in this repo. AC7 by `BTreeMap` alias order plus a `const DOCUMENT_NAME`, both carrying comments naming AC7 as the reason. AC6's halting behaviour is specified in §Validating the registry. AC9's managed block is `merge-managed-block` with marker `ductus:constitutions` into the layout-derived native rules file, which is strategy `skip` — so the file is preserved and only the region rewritten, and `.ductus/constitution.md` is never the target. AC10's precedence is stated once in §governance-precedence, including that nothing enforces it, and README and `docs/shared-constitution.md` point at it rather than restating it.

**One criterion I initially read as false and was wrong about.** AC8 requires a report produced without a registered constitution to say so, and `## Unexamined governance` renders `*None.*` when none are registered — which does not distinguish "none registered" from "all loaded cleanly". The tree settles it the other way: `/{project}:target` step 4 specifies reporting *nothing* when `loaded` and `skipped` are both empty, "so a project without the feature reads exactly as it did before", and AC1 requires that same silence. The two criteria constrain each other and the implementation satisfies both; the reading that would have made AC8 a finding would have made AC1 one instead.

**Security.** No network: `repo` is recorded verbatim and never fetched, which the module doc, the entry doc and the data-model all state, and which removes the transport entirely — "unreachable" collapses to "not checked out" rather than being an error class. Path handling accepts `..` and absolute paths deliberately, with the reasoning cited to `validate_no_traversal`'s config-sourced-path boundary rather than restated, and adjudicated in 022's `config-sourced-paths-and-the-traversal-boundary` scenario; the value originates in committed config, not from an LLM or a host. Registration is not transitive — a registered checkout's own config is never read — so there is no recursion to bound and no cycle to detect. No finding.

**Quality.** `resolve-constitutions` is the strongest QUAL-CLAIM-001 implementation in the corpus I have read: it names the rule in its own module doc, splits `loaded`/`skipped` so a caller "has to drop a field it was handed" to report a clean result over an unreadable source, and carries `examined` as the denominator. The bootstrap's import block extends the same discipline — when `loaded` is empty the block is written **empty rather than omitted**, because "an empty managed region and an absent one differ: the first says `/ductus` looked and found nothing registered, the second says nothing ran." QUAL-GROUND-001 does not fire: the registry is bound through a typed serde structure, which is the compliant form. QUAL-STUB-001 does not fire: every path resolves, reports, or errors.

**Reuse / efficiency / simplicity.** The registry deliberately mirrors `[services]` — `load_services`/`classify` shapes reused, and the missing-checkout-is-a-state posture inherited rather than re-decided — with the module doc saying so and why: "reusing that path keeps the two registries legible to each other." One document per checkout, one filename, no `ref`/`version` field, no fetching. No finding.

**The observation, stated here because a reader of this record should meet it.** The validation §Validating the registry promises is enforced nowhere deterministic. Probed against the release binary: a non-URL `repo` loads silently, and an empty `path` is reported `no-constitution-document` — a confident wrong reason for what is actually a config mistake. It does not falsify AC6, which scopes rejection to configuration time; it is deterministic validation that was specified and never built, so the fix is 022's by the runtime-home rule and carries a release.

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

- convention: the `[constitutions.*]` validation `framework/bootstrap/ductus.md` promises is enforced nowhere deterministic, so `/{project}:target` silently accepts entries the bootstrap would reject. §Validating the registry states that a malformed entry halts the run naming the alias and field — an alias that is not a bare TOML key, a `repo` that is not URL-shaped (a scheme and a host), or an empty `path`. `runtime/src/schema/constitutions.rs`'s `from_toml_str` does TOML parsing and serde required-field enforcement only, and its own doc says so ("the pure shape plus parser"); `resolve_constitutions::classify` reads the filesystem and never inspects the values. Probed 2026-09-13 against the release binary with a two-entry config: `repo = "not-a-url-at-all"` resolved `loaded` with no complaint, and `path = ""` resolved to the repo root and was reported `no-constitution-document` — a confident WRONG reason, telling the operator their checkout lacks the document when the actual mistake is an empty path in their own config. That is the exact conflation 055's Edge Cases forbid between the two failure states ("an operator who cloned the wrong repository must not read the same message as one who cloned nothing"), and §design-principles' bar that a check which cannot run must not be indistinguishable from one that passed. AC6 is NOT falsified — it scopes the rejection to configuration time, and the bootstrap prose does specify it — so this is unimplemented deterministic validation rather than a stale criterion. The checks are pure value inspection, hence runtime-eligible by §runtime-boundary's three criteria, where eligibility is a default rather than a permission. Fix belongs to 022 as a scenario per AGENTS.md §Workflow's runtime-home rule, and lands a `runtime/` change, so it carries the version-bump-and-tag obligation in the same sitting. — `runtime/src/schema/constitutions.rs:70`

## Skipped passes

*None.*

## Unexamined governance

*None.*
