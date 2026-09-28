---
spec: 015-tarball-fetch
last-run: 2026-09-28T13:16:25Z
reviewed-against: 1191da602ff69367ebe805230f22800381fcc9e7
diff-base: 96b41f636dc4101c77aaab29495ce537e52ea45e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 3
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 015-tarball-fetch

## Summary

Reopen for 061 (the archive ref and the derived framework root). Default diff base: the parent of this reopen's done -> in-progress commit, so the window is the reopen itself; compute-review-scope reports scope 3: framework/bootstrap/ductus.md (the plan's one affected file), spec.md and plan.md (both modified since the base). Five passes (security, reuse, quality, efficiency, simplicity) against all 11 rule files discover-rule-files selected, each read in full, plus AGENTS.md read in full. 0 MUST, 0 SHOULD, 0 low-confidence. examined: 3 of 3. spec.md and plan.md were read in full after their edits. framework/bootstrap/ductus.md was read in full at the start of this session and lines 169-739 were re-read after 061's tasks 1-3 edited them -- every section 061 changed except the argument-hint and the §Inputs flag bullet, which are one line each; the passes found nothing in Source resolution, Recording the source, the Small fetch, or Archive fetch and extract. One observation, fixed in the run and committed at 1191da60 before this record: plan.md's resolved-question summary still read 'defer' after the spec's Resolved Question and the plan's trade-off were marked adopted by 061.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: plan.md's resolved-question summary still said ref pinning is deferred, contradicting the adoption this reopen records — `specs/015-tarball-fetch/plan.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
