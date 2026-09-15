---
spec: 032-opencode-agent
reviewed-at: 2026-09-15T14:31:10Z
reviewed-against: dd03f65aed41d190c7f2cf3bcd76f7876123ef62
diff-base: 108714e07006a59d5306d33905dd7504efea877b
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 16
scope: 17
skipped-passes: []
---

# Review — 032-opencode-agent

## Summary

Five passes run against all 11 rule files `discover-rule-files` selected, each read in full, plus AGENTS.md's four rule-bearing sections and the constitution. 0 MUST, 0 SHOULD, 0 low-confidence. examined: 16 of 17. Read in full: `install.sh`, `framework/bootstrap/ductus.md` (nine byte-bounded ranges covering the whole file), `README.md` (three ranges), `framework/bootstrap/configure/opencode.md`, `scripts/gen-configure-mcp.sh` and its test, and the eight sibling artifacts this pass edited. ABSENT but in scope because the plan lists it, named rather than dropped: `framework/templates/project/gitignore.md` — the shipped template has no `.md` extension and lives at `framework/templates/project/gitignore`, which IS present and was read in full under 028's scope; the plan row carries the wrong filename, not a missing file. The pass's own change here is `data-model.md`'s `§Permission Setup` reference, which was split across a line wrap (`§Permission` ended one line, `Setup` began the next) and is now rewrapped so the whole anchor and `framework/bootstrap/ductus.md` sit on one line; the target was verified present at `framework/bootstrap/ductus.md:146`. Security pass note: `install.sh` enforces `--proto '=https' --tlsv1.2` with no `--insecure`, validates the payload's frontmatter-delimiter count before writing so a captive-portal 200 cannot be reported as a successful install, uses `mktemp` with an EXIT trap, and writes each agent's permission seed only when the file is absent. Its `[ ! -f … ]` guards are `if` blocks rather than trailing `&&`, so the `set -eu` last-command hazard AGENTS.md records does not apply. OpenCode's `permission` block orders the broad `"*": "ask"` first and the denies last, which is load-bearing under last-match-wins and is stated as contract in `data-model.md` rather than left to presentation.

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
