---
spec: 048-govern-acquired-runtime
last-run: 2026-09-28T13:26:30Z
reviewed-against: 929aa1207890b3b2e309ce4774456c35f5d15930
diff-base: 97058e9d530d7500759701787e827a93d913da7b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 21
skipped-passes: []
reviewed-digest:
  data-model.md: aaabca49a7dd3b0e8c5a3fe311f77ba06c98b5f3d106cbc19c3c846d9fcd9a33
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 0c5984d011f5d35b3685f05bc083e01d9aaa7bac4a31c0949e1d5bcd1f00627b
  scenarios/state-a-version-checks-the-pin.md: 40a4cbaf5dab4b5ee1127c4e808f70bbd08433415e952011ceb7c419a5e24e23
  scenarios/state-b-continues-in-session.md: b4d9b7da7ad0d9486268cff26ec375aa64d35a9639a5828a03cdde92da756ddd
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 048-govern-acquired-runtime

## Summary

Reopen for 061 (the pin's source, the store's recurring cost, the release asset set). Default diff base: the parent of this reopen's done -> in-progress commit, so the window is the reopen itself; compute-review-scope reports scope 21 from the plan's Affected Files, of which spec.md, data-model.md and scenarios/pin-is-readable-when-acquisition-needs-it.md were modified since the base. Five passes (security, reuse, quality, efficiency, simplicity) against all 11 rule files discover-rule-files selected, each read in full, plus AGENTS.md read in full. 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations. examined: 3 of 21 -- spec.md, data-model.md and the pin scenario, each read in full before its edit and its committed diff read in full after. The edits were checked against framework/bootstrap/ductus.md (Source resolution step 5, Runtime acquisition Branch 2 step 1, §Derived paths, §MCP registration's per-agent scope) and .github/workflows/runtime-release.yml's complete-set assertion (the SBOM and install.sh), both read in full this session. Two durable contracts changed (data-model.md and the pin scenario); the other four scenarios are unchanged in the window, including release-halves-publish-together, whose gating chain still holds because post-release jobs gate nothing. NOT read against this reopen, named rather than counted, all unchanged in this window: .ductus/config.toml, .github/workflows/{framework-checks,runtime-acquisition,runtime-release}.yml, .gitignore, .mcp.json, AGENTS.md, README.md, framework/bootstrap/ductus.md, the framework/commands/*.md glob, framework/constitution.md, framework/migrations.toml, framework/migrations/runtime-store-path.md, scripts/audit/run-all.sh, scripts/audit/version-agreement.sh, specs/021-runtime-boundary/spec.md, specs/029-bootstrap-runtime-autowire/spec.md, version (several of these were read or edited this session by 061's own tasks, before this window).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
