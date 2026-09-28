---
spec: 061-updates-track-the-latest-release-tag
last-run: 2026-09-28T14:02:34Z
reviewed-against: 3e47e14f395979eae41b8e2e7bd3a57a7e42e3ae
diff-base: a9c2db91cb804347c269acf0037f5a844b6ef1ef
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 15
scope: 58
skipped-passes: []
reviewed-digest:
  data-model.md: c0392200385f4900b25aceeca1a4e38c0d9d19526e4cd7ee66030731512a7622
blocking: false
dispositions:
  fixed: 2
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "other: 056's consumer table and its cross-spec-impact Resolved Question named the `archive/` URL as Family 36's subject; since 061 task 6 the family derives the slug from the codeload tar.gz URL — `specs/056-bootstrap-archive-boundary-split/spec.md`"
    outcome: routed
    target: specs/056-bootstrap-archive-boundary-split/spec.md
    decided-at: 2026-09-28T14:02:34Z
    decided-by: andy@stone.dev
---

# Review — 061-updates-track-the-latest-release-tag

## Summary

First review of 061. Default diff base a9c2db91, the parent of 061's planned -> in-progress commit (6d0a7fc5), so the window is all of 061's work. compute-review-scope reports scope 58: the plan's Affected Files, one of which is the pattern entry specs/{003,...,056}-*/... (a glob, not a path, so it has nothing to read), plus the 56 files modified since the base. Five passes (security, reuse, quality, efficiency, simplicity) ran against all 11 rule files discover-rule-files selected, each read in full this session, plus AGENTS.md. Result: 0 MUST, 0 SHOULD, 0 low-confidence. examined: 15 of 58, each read in full this session: framework/bootstrap/ductus.md, framework/bootstrap/ductus-procedure.md, install.sh, scripts/tests/test-install.sh, .github/workflows/framework-checks.yml, scripts/audit/self-url-resolution.sh, scripts/audit/installer-registry-parity.sh, AGENTS.md, CLAUDE.md, 061's spec.md, tasks.md and data-model.md, and 056's spec.md, plan.md and analysis.md. Read partially: .github/workflows/runtime-release.yml, lines 1-30 and 440-619 (release-assets, verify-published, verify-installer) plus every job's needs line; the build, acquire, sbom and publish jobs in between are unchanged by 061. Read as their diff against the base only: README.md, docs/slash-commands.md, framework/commands/audit.md, scripts/audit/README.md, runtime/src/primitives/fetch_archive.rs (a doc comment), runtime/Cargo.lock (the chacha20 bump, task 21), and the spec-side edits to 003, 007, 015, 023, 026, 028, 029, 032, 048 and 050. Each of those was reviewed in full in its own reopen's review this run. NOT read, named rather than counted: framework/bootstrap/govern.md (cmp-identical to ductus.md; Family 21 green); .claude/commands/ductus/audit.md (generated from audit.md; gen-claude-commands --check reports 16 commands in sync); scripts/lint-release-ordering.sh (unchanged in the window; the lint and its test passed in task 20's gate); 056's review.md (read in part); and the other specs' review.md and analysis.md records, each written and checked by its own reopen this run. Security: install.sh passes a --ref value into a URL only after validating it against an allowlist (latest, main, or ductus-v digits.digits.digits). It halts before any write on a bad value, a repeated --ref, an unresolvable latest release, a tag below the floor, or a 404 tag. Every fetch carries --proto =https --tlsv1.2, and there is no fallback to main. The bootstrap validates --ref with the same grammar before it records anything. release-assets stages install.sh from a checkout at the tag, and verify-installer byte-compares the published copy with it. Quality: test-install.sh's 18 cases cover the defaults, both argument positions, the floor (at, below, numeric not lexical), each halt with nothing written, and the next-command message. Three observations, all dispositioned before this record: (1) AGENTS.md's job-order enumeration, fixed at 5de17af2; (2) 061's data-model {source-label} example, fixed at 5de17af2; (3) 056's Family 36 subject, routed as a body edit to 056 (reopened at e55a855d, corrected at 3e47e14f). 056's analysis and re-close are outstanding. Task 9's pre-release exercise observed the main, invalid-value, repeated-ref, nonexistent-tag and latest-rewrite paths. The latest-release and named-tag success paths cannot be observed before the release that carries 061 (plan, Trade-offs).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: AGENTS.md's release entry gave runtime-release.yml as seven jobs in order with release-assets second from last; 061's verify-installer made it nine, and cargo-audit was already missing from the list — `AGENTS.md` — **fixed**
- other: the {source-label} example for the latest release omitted the origin suffix the bootstrap's Derived paths renders, (default) — `specs/061-updates-track-the-latest-release-tag/data-model.md` — **fixed**
- other: 056's consumer table and its cross-spec-impact Resolved Question named the `archive/` URL as Family 36's subject; since 061 task 6 the family derives the slug from the codeload tar.gz URL — `specs/056-bootstrap-archive-boundary-split/spec.md` — **routed** to `specs/056-bootstrap-archive-boundary-split/spec.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
