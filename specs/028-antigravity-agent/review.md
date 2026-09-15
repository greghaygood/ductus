---
spec: 028-antigravity-agent
reviewed-at: 2026-09-15T14:31:04Z
reviewed-against: dd03f65aed41d190c7f2cf3bcd76f7876123ef62
diff-base: 108714e07006a59d5306d33905dd7504efea877b
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 17
scope: 17
skipped-passes: []
---

# Review — 028-antigravity-agent

## Summary

Five passes run against all 11 rule files `discover-rule-files` selected, each read in full, plus AGENTS.md's four rule-bearing sections and the constitution. 0 MUST, 0 SHOULD, 0 low-confidence. examined: 17 of 17 — every in-scope file read in full, nothing absent and nothing unread. That includes `framework/bootstrap/ductus.md` (110,417 B, nine byte-bounded ranges covering the whole file), `README.md` (three ranges), `specs/012-multi-agent-govern/spec.md` (two ranges), `framework/bootstrap/configure/antigravity.md`, `scripts/gen-configure-mcp.sh` and its test, `framework/templates/project/gitignore`, and the eight sibling artifacts this pass edited. The pass's own change here is one line of `data-model.md`: the `§Agent Selection` reference now names `framework/bootstrap/ductus.md` on its own line. The target was verified present (`framework/bootstrap/ductus.md:128`) and its claim re-checked against the section — auto-detect rule 2 does key on `config_dir` existing, as the data-model bullet states. Security pass note: the permission surfaces in scope (`configure/antigravity.md`'s action-grammar allow/deny set and the registry `settings_template`) carry destructive-operation denies and no wildcard-before-subcommand shape; Family 29's subject set excludes the Antigravity grammar by construction because it has no glob grammar to get wrong, which the audit source states rather than leaves implied.

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
