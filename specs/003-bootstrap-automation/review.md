---
spec: 003-bootstrap-automation
diff-base: 108714e07006a59d5306d33905dd7504efea877b
captured-issues: 0
skipped-passes: []
last-run: 2026-09-15T14:30:50Z
reviewed-against: dd03f65aed41d190c7f2cf3bcd76f7876123ef62
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 10
scope: 21
reviewed-digest:
  scenarios/curl-sh-installer.md: 9151c485280487366a82991246b3147164b5c52020d50cc25f309ffb642379da
blocking: false
---

# Review — 003-bootstrap-automation

## Summary

Five passes (security, reuse, quality, efficiency, simplicity) run against all 11 rule files `discover-rule-files` selected, each read in full, plus AGENTS.md's four rule-bearing sections and the constitution. 0 MUST, 0 SHOULD, 0 low-confidence. examined: 10 of 21 in scope. Read in full: this spec's `spec.md` and `scenarios/curl-sh-installer.md`, and the eight sibling artifacts this pass edited under 027, 028, 032 and 050. NOT read, named individually rather than folded into the numerator: the seven generated command mirrors `.claude/commands/ductus/{analyze,clarify,implement,plan,specify,status,target}.md`. Those are generator output; `scripts/gen-claude-commands.sh` was re-run in this pass and reported all 16 commands in sync, but that is a parity claim about the generator and I did not open the mirrors or their `framework/commands/` sources, so none is counted. ABSENT but in scope because the plan lists them: `.claude/commands/ductus/{about,init,next,setup}.md` — `init.md` was deleted on 2026-09-15 with the `/ductus:init` retirement this spec records, and the other three name the pre-rename command set. One defect was found and FIXED in this pass rather than recorded as outstanding: the Behavior bullet in `scenarios/curl-sh-installer.md` enumerated install destinations for three agents while `install.sh` has carried an `opencode` arm since spec 032 (commit dd03f65a).

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
