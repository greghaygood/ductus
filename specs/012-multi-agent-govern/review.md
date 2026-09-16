---
spec: 012-multi-agent-govern
diff-base: a86b5cb2f5891639e42d3d6886de6a794abca884
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T20:25:31Z
reviewed-against: a7e7af9221c5fef89849d0b6e847e38518eb013a
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 24
reviewed-digest:
  data-model.md: 8ae49e75034c7ccd56250528e2dddb06f0fd2e3f3cb14857a2572531137e74e6
  scenarios/settings-template-bash-allowlist.md: 86737c4f1200bcc492dbc8c48f01f2fbe5d76ea7db30b504c7d2fc0199e7de36
blocking: false
---

# Review — 012-multi-agent-govern

## Summary

Five passes over the resolved scope; 0 MUST, 0 SHOULD, 0 low-confidence. Rule files loaded via `discover-rule-files` (all 11); `quality-cross.md` read in full, the security and configuration files consulted against the surfaces 012 actually touches.

**Diff base.** Both measured before choosing, per the campaign rule. Natural base `a86b5cb2` — scope 15, modified-since 4. `--since HEAD` (`ae6b4705`) — scope 13, modified-since 0. Took the natural base: it is the tighter *and* more honest window, because the step-4 reopen moved it to this pass's own parent, so the four files this pass edited fall inside the denominator rather than outside it.

**examined: 7 against a scope of 24, and the 17 break into two groups that are not the same.**

*Eight paths do not exist.* `ductus/ductus.md`, `ductus/ductus-auggie.md`, `commands/setup.md`, `commands/setup-auggie.md`, `commands/setup/claude.md`, `commands/setup/auggie.md`, `.claude/commands/ductus/setup.md`, `specs/spec.md`. They stay in scope because `plan.md` lists them; the repository was reorganised after 012 shipped and `/setup` was renamed to `/configure`.

*Nine are other specs' `spec.md`, pulled in by a bookkeeping commit.* 000, 008, 013, 016, 017, 018, 019, 020, 023 entered this window only because `b5936481` corrected a timestamp in each one's `analyze:` block — a field I wrote and verified in that commit, not a 012 subject. They are **named rather than counted**: being confident about a one-line change I authored is not the same as having reviewed the file, and `examined` is only the second. That is why the denominator moved from 15 to 24 between the base measurement and this record — two later commits widened `a86b5cb2..HEAD`.

*Read in full, and counted:* `CLAUDE.md`, `README.md`, `specs/007-govern-workflow/spec.md`, and 012's `spec.md`, `data-model.md`, `tasks.md`, `scenarios/settings-template-bash-allowlist.md`.

**Read but not counted, because it is not in the recorded scope.** 012's behaviour now lives in `framework/bootstrap/ductus.md`, the successor of `ductus/ductus.md`. I read its §Agent Registry, §Derived values, §Agent Selection, §Permission Setup, §Shared Files manifest, §Pre-run Migrations filter and the ten numbered Instructions steps — **not** the whole 141KB file. Every behavioural criterion below was verified against those sections; the rest of that file was not examined and nothing here is a claim about it.

**Security.** The one real surface is the per-agent `settings_template` allowlists, which decide what runs unprompted in an adopter's agent. The layering is sound: the registry seeds only what `/ductus` itself invokes, with `"deny": []`, and `/ductus:configure` installs the canonical set — whose deny list covers destructive file operations (`rm -rf` and five variants) and dangerous git operations (`push --force`, `reset --hard`, `git rm`, `clean -fd`). The git allow patterns are narrow (`git status *`, `git config *`, `git rev-parse *`, `git diff *`, `git ls-files *`) with no bare `Bash(git *)`, which is exactly what this spec's own scenario §Edge Cases asks for. `Bash(curl *)` is intrinsic to a curl-based installer. No finding.

**Quality.** QUAL-STUB-001: the zero-agent and unknown-key paths both reject loudly and refuse to partially scaffold — compliant, this is the fail-loud form. QUAL-CLAIM-001: the post-scaffolding output reports created / updated / unchanged / skipped / pinned / merged as distinct states rather than a bare success, which is the distinguishable result the rule asks for. QUAL-GROUND-001: `install.sh`'s per-agent destination paths encode each CLI's layout — a contract ductus does not own — but Family 14 holds them against the registry-derived path in three directions and fails loudly, so the guard exists.

**Examined and judged not to be a finding, stated rather than filed.** Three of four `settings_template` values encode an agent's native settings schema as literal JSON with no typed binding and no test against the real shape; only `opencode` carries a `$schema` reference. That is the QUAL-GROUND-001 shape. Not filed because `AGENTS.md` §Workflow already records the absence of per-layout behaviour parity checking as known contributor discipline, and the three agents concerned arrived with 028 and later rather than with 012 — capturing it would re-file a recorded gap.

**Reuse / efficiency / simplicity.** 012 exists to remove a duplication — two 95%-identical bootstrap files held in sync by a parity rule — and replaces it with one registry-driven file whose per-agent values are derived by convention rather than branched on. That is the win, and it held: the registry has since absorbed two more agents and a `layout` dimension without reintroducing a second bootstrap source. No finding.

Seven corrections to 012's own artifacts and one to `README.md` were made before this review and are not counted as findings here; they are recorded in `ae6b4705` and `a7e7af92`.

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
