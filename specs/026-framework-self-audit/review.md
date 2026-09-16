---
spec: 026-framework-self-audit
diff-base: 6014d53caae11bcba77bc868829234207e49b459
captured-issues: 0
skipped-passes: []
last-run: 2026-09-15T18:36:19Z
reviewed-against: 1110d6fc9634a77b5fa09b53f3341beef902cfa7
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 18
scope: 20
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 035bb7ef52c236135d791f35c3ec407d7908c8d51a169483bce40bc26c7a46c8
  scenarios/family-10-migration-coverage.md: 8684e643e4ea938cbeeed6d1342aef27efc84a7bf7559b111b6a12cacacfa02b
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: e2f5e57dc51efec37f63fdc419d1c78ae4afb8319bcd41b604a2c22507af6296
  scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
  scenarios/family-19-review-freshness.md: 07adf4f5f7f0590e90c043d9254534f10f77fc717fb951bd7dfe2ff42a74d787
  scenarios/family-19-says-what-it-examined.md: 5bcffd3a32d015c740ee9ca232d86f4cb7d71f251705eb3d9f22cce1198ed1ed
  scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
  scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
  scenarios/family-31-review-block-agreement.md: 4f5f35a88a5ceb3668de37b4cf39d01427e813f170398901089b2a52073351b2
  scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
  scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
  scenarios/family-36-self-url-resolution.md: f5f69dd3e566a0e07ef2825d5843ec76cf18203355c7c7ddff8737673dfadb09
  scenarios/host-namespace-parity.md: 05714fe6b728391f699ed7328e1aea252a48259ae489fa9c06e0d0609dc1d376
  scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
  scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
blocking: false
---

# Review — 026-framework-self-audit

## Summary

Five passes over the resolved scope. **`examined: 18` of `scope: 20`**, and the two not counted are named individually rather than folded into the numerator.

**`.claude/commands/ductus/audit.md`** — the generated mirror of `framework/commands/audit.md`, which was read in full. `scripts/gen-claude-commands.sh` was re-run in this pass and reported all 18 command copies regenerated, so there are good grounds to believe it correct; that is confidence, not a read, and `examined` is only the second. **`.github/workflows/markdown-only-pipeline.yml`** — **absent**. It stays in scope because `plan.md` lists it, and its absence is grounded rather than assumed: 026's own §Resolved Questions records that `048-govern-acquired-runtime` removed that workflow and the `/audit` step now runs as `(h) Framework self-audit` in `.github/workflows/framework-checks.yml`.

The other 18 were read in full this session, including all eleven rule files `discover-rule-files` reports under `selected`, loaded before the passes ran.

**Security.** No source in scope but shell. The change adds six alternatives to a literal-built regex in `rename-sweep-residue.sh`; no input crosses a boundary, nothing is eval'd on user data, and `shellcheck -S warning` is clean over all 55 tracked scripts. The surface-specific rule sets (security-backend/frontend, api-backend, concurrency, observability, reliability, performance, accessibility, configuration-cross) verify design-time commitments this change makes none of — read and found to have no subject here, which is a different statement from checked-and-passed.

**Quality — `QUAL-CLAIM-001`, the rule this change is most exposed to, since the family *is* a claim-checker.** Three checks. The widened family still reports its examined-file count on stderr and still treats a degenerate scan as a finding rather than a pass, so the property that made it honest is unchanged. The header states its own limit outright: a ditransitive sentence (*"give ductus its due"*) would be a false positive, measured absent across the 584 tracked markdown files as of today. And the correction it carries is of exactly this shape — the family previously exited 0 over 514 files with live residue present, which is a check that could not see what it exists to find, and the record now says so instead of retaining *"exactly the 8 real sites, no others"*.

**Reuse.** The construction count is now stated in five places — script header, scenario, AC17, `framework/commands/audit.md`, `scripts/audit/README.md`. That is the framework's existing shape for all 37 families and Family 28 mechanically holds only the family-number registration, not the descriptions, so the prose copies are held by discipline. The disposition taken was the sweep `AGENTS.md` prescribes for a canonical set: all five were located and updated in one pass, and the task records that as a subtask rather than leaving it to be rediscovered.

**Simplicity.** `POSSESSIVES` is a separate variable rather than six more alternatives inside `FOLLOWERS`. Functionally identical; kept separate because the scenario and both registries now describe *three* constructions and a reader matching prose to code should find three lists.

**Two things the probe found that reading did not, recorded because they are the evidence the widening is calibrated rather than guessed.** Probing the **narrowed** direction — not the widened one — is what caught both: `my` had to be dropped from the possessive class because `README.md` documents adoption as `/ductus my-project`, so the front page puts the name before `my` in correct prose; and the inbox item recording the ninth site had quoted the defect in `*italics*` without the double quotes the family's stripper protects, so the widened run reported the item describing the bug. Neither was predictable by reading the pattern.

**Calibration, re-derived rather than quoted.** `govern its` occurs exactly once at `9da4a7ae^` — the `api-backend.md` site — and `their` / `our` / `your` / `his` / `her` zero times each. The whole closed class ships anyway because the existing lists are grammar-calibrated, not frequency-calibrated: at that same commit three of the seven followers (`whether`, `these`, `those`) and seven of the eight modals had zero instances and are listed regardless. That measurement is what makes adding the plural consistent with precedent rather than speculative.

**Pre-reopen window, measured before the flip because it is not recoverable after it:** base `c1b00ea8`, **81 modified-since / 93 in scope at 52,446 bytes**. Post-reopen: **8 / 20 at 4,234 bytes** on `6014d53c`, a 12x byte reduction, both legs inline.

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
