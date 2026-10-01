---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-10-01T20:33:09Z
reviewed-against: 1e06ab84c9be89b84e4b36806dafbbefea1d50a8
diff-base: c963ab4506211448bb18045bcd275b4615c5dafe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 28
scope: 46
skipped-passes: []
reviewed-digest:
  data-model.md: 416e488447e1fb195e85babd92882b3d5eefa1cb5e3e2876809a59f487420818
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: ca44ec6cd85ccd0d3a52d9124c047fff81958807c8ffdb55f2858d6fd60a54aa
  scenarios/a-pending-fold-never-holds-done.md: 29f85dfd8b78f2030ffaf7a517a4e0236ec580bfe8e05d8f1cbaa772c6ff0c08
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 051-branch-scoped-spec-numbering

## Summary

Task 27 (scenario a-pending-fold-never-holds-done) over 051's window since it re-entered in-progress (diff-base c963ab45): 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. Examined 28 of 46 in scope: every hunk of 2ccf71a5, ae8195c6 and 1e06ab84 outside the four .claude/commands/ductus mirrors. The mirrors are not counted: gen-claude-commands regenerates them byte-for-byte from the framework sources read here and reported them in sync. The 14 plan-affected files this window did not touch (framework/bootstrap/*, analyze.md, specify.md, runtime-tools.txt, payload.rs, parser/mod.rs, create_feature.rs, mod.rs, resolve_feature.rs, rewrite_spec_links.rs, validate_frontmatter.rs, extensions.rs, mixed_corpus.rs, gen-help-tables.sh) were not read; a prose-claim grep over them for fold-gate statements found none. QUAL-TEST-001 holds for the new tests: four mutations, restored byte-identical, each failed at least one of them — a fold blocking on any folds-into, the exact old in-tree block, a fold passing early, and the fold owning the Next Action cell at every status. One observation, the gate module's doc reflow, was fixed as a chore in 1e06ab84; the stored discard about 050's scenario expired because this change rewrote that sentence.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: check-review-gate's module doc listed its checks on one overlong line after the fold check's removal — `runtime/src/primitives/check_review_gate.rs` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
