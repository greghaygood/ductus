---
spec: 056-bootstrap-archive-boundary-split
last-run: 2026-09-28T13:40:52Z
reviewed-against: 7e5d9d3f16d90924adec16f39cc37dfab990f20f
diff-base: 5e49a1e3ed5fcacc91953101a46da6bf86b22672
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 7
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 056-bootstrap-archive-boundary-split

## Summary

Reopen for 061 (the archive half is addressed through the derived framework root). Default diff base: 5e49a1e3, the parent of this reopen's done -> in-progress commit (b0a3e44d), so the window is the reopen itself. compute-review-scope reports scope 7: the plan's five Affected Files, plus spec.md and 061's tasks.md, which joined the window when that task's subtasks were ticked (b2bca500). Five passes (security, reuse, quality, efficiency, simplicity) ran against all 11 rule files discover-rule-files selected, each read in full, plus AGENTS.md read in full. Result: 0 MUST, 0 SHOULD, 0 low-confidence. examined: 6 of 7. spec.md, plan.md, framework/bootstrap/ductus.md (914 lines), framework/bootstrap/ductus-procedure.md (443 lines), runtime/legacy-prose-commands.txt and 061's tasks.md were each read in full this session. NOT read, named rather than counted: framework/bootstrap/govern.md, confirmed byte-identical to ductus.md with cmp before and after the fix below; audit Family 21 passed on the fix commit. Claims checked against the tree: the corrected Resolved Question and the signpost match ductus.md §Archive fetch and extract step 2 and §The archive half, and ductus-procedure.md's header. The signpost's claim that GitHub names a release archive's directory ductus-<tag>/ was probed against the published ductus-v0.54.2 tag: its codeload tarball's single top-level directory is ductus-ductus-v0.54.2. Security: the --ref value reaches a URL only after the allowlist grammar in Source resolution step 2 validates it. {framework-root} is read from extract-archive, which blocks path traversal per entry. The runtime digest check stays in the installed half. Two observations, both fixed in the run and committed at 7e5d9d3f before this record. (1) plan.md:35 still placed the pointer at the first point where {tempdir}/ductus-main/ exists; it now carries the {framework-root} annotation line 37 already had. (2) ductus.md §Instructions step 8 called Pre-run Migrations the section 'above'; it now says it is in the archive half, and govern.md was re-copied. The affected passes were re-run over both lines after the fix. scripts/audit/run-all.sh exited 0 on 7e5d9d3f. 056 has no scenarios and no data model, so the durable-contract digest is empty.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: the plan's rationale for placing the pointer still says §File Fetching is the first point at which `{tempdir}/ductus-main/` exists; since 061 the root is `{framework-root}`, derived from the extraction — `specs/056-bootstrap-archive-boundary-split/plan.md:35` — **fixed**
- other: §Instructions step 8 calls **Pre-run Migrations** the section "above"; it lives in the archive half, and sat below §Instructions even before the split — `framework/bootstrap/ductus.md:42` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
