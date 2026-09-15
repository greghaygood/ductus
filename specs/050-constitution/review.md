---
spec: 050-constitution
reviewed-at: 2026-09-15T16:06:25Z
reviewed-against: f352f6b8ba78759b69d506ed0cb0649ffcf6275a
diff-base: 047534d693d25f1e08158613680d1ad4900d778f
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 11
scope: 11
skipped-passes: []
---

# Review — 050-constitution

## Summary

Five passes over the resolved scope of 11 files, all 11 read in full (`examined: 11` of `scope: 11`). Rule files loaded via `discover-rule-files`: all 11 under `framework/rules/`.

**What was read.** `framework/constitution.md` (765 lines, in seven byte-bounded ranges, plus the edited §drift-prevention subsection re-read after the change); `AGENTS.md` (178 lines, fifteen ranges); `CLAUDE.md`; `framework/templates/project/agents.md`; `framework/templates/project/claude-md.md`; `specs/050-constitution/spec.md`, `plan.md`, `tasks.md`, and `scenarios/knowledge-routes-by-population-not-by-kind.md`; `specs/045-decision-state-drift-detection/spec.md` (in scope via the plan's Affected Files, unmodified by this pass); `specs/inbox.md`. Nothing in scope went unread and nothing in scope is absent.

**What the code-pattern rules had as a subject: nothing.** The scope is entirely markdown prose — no source file, no executable path. `quality-cross.md` scopes itself to code patterns in as many words, and the backend/frontend rule sets (authn/authz, input, XSS, CSRF, CSP, storage, queries, caching, pooling, concurrency, reliability, observability, accessibility, vitals) verify source in scope. So the security, reuse, efficiency and simplicity passes ran against prose artifacts and the rule IDs were available but unfirable, rather than examined and found clean. Stating that rather than reporting a clean sweep is `QUAL-CLAIM-001` applied to this record.

**Two defects were found and fixed in this pass rather than recorded as findings** (`f352f6b8`), both in the scenario this task authored, both the enumeration-drift shape: it claimed this spec "promoted 26 rules" when `plan.md` §Classification records 23 promoted and 3 already promoted out of 54 classified, and it called the two templates "create/skip"-strategy when `framework/bootstrap/ductus.md` gives both `AGENTS.md` and `CLAUDE.md` as `strategy: skip`. Each was caught by reading the claim against its cited source rather than forward from the draft.

**Reuse pass, the dimension most at risk here.** The change states one normative rule in `framework/constitution.md` §drift-prevention and points at it from four agent-facing surfaces, which is §Promotion mechanism's shape rather than four copies. Checked deliberately: the `AGENTS.md` header and `CLAUDE.md` §Auto-Memory Routing state the routing *question* in the register of a reader who is mid-write and adds no rule the constitution lacks — the constitution itself designates those files as where the principle is applied. `CLAUDE.md`'s destination list omits the organization tier; that is correct and not drift, because this repository registers no `[constitutions.*]` entry (`resolve-constitutions` reports `examined: 0`), the block links §drift-prevention for the full rule, and the shipped `claude-md.md` template — the copy that reaches a project that may have an organization — carries all three tiers.

**Verified by probe, not by reading.** `resolve-anchor` over the new scenario: 12 of 12 references resolved (one, `§Classification` on the Edge Cases line, was unresolved on the first run and repaired by naming `spec.md` on the reference's own line, then re-run). Family 25 over both per-line targets: 2 of 2 examined, clean, with the rewritten `AGENTS.md` header still one balanced line. `npx markdownlint-cli2`: 0 issues across 511 files. `scripts/audit/run-all.sh`: exit 0 on the committed tree, and proven capable of failing first by writing a wrong value into the repo-root `version` file (Family 20 went red, exit 1, then restored).

**Scope window.** `diff-base` is `047534d6`, the parent of this pass's first commit — the post-reopen leg, which returned inline at 13,946 bytes. `plan-affected` (5) understates what this task touched, because 050's Affected Files table is the original pass's design record and six prior back-edges did not amend it; that cost this review nothing, since every file this pass edited appears in `modified-since` and the scope is their union. Left as-is deliberately rather than amended here: changing that pattern belongs to whoever runs the bulk promotion pass.

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
