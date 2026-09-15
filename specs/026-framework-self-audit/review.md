---
spec: 026-framework-self-audit
reviewed-at: 2026-09-15T00:04:22Z
reviewed-against: 9c25716daa5a26f42ec034edb7e514fdd55a51e3
diff-base: 4b657f5181aedb31046da7e5a371ba96ba551434
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 19
scope: 22
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean across all five passes: 0 MUST, 0 SHOULD, 0 low-confidence, nothing blocking — and that is **after** the two findings the passes produced were fixed, not instead of them. Both are described below with the commit that closed each.

**Scope, and both bases measured.** Diff base `4b657f51`, the parent of this pass's first commit, resolving **10 modified-since in a 22-file scope at 2,307 bytes**. The pre-reopen natural base `d79a701c` was measured before step 4 made it unrecoverable: **354 modified-since / 368 in scope at 164,150 bytes**, far over the MCP output cap, so that leg ran through the release binary redirected to a file and was read with `jq` — an error there is not an empty scope. `--since HEAD` gave **0 / 15** and was declined for excluding by construction the ten files this pass edited, which is the same call the last eight passes made. The post-reopen window **widened twice across the pass**, 8 to 9 to 10, as the two script fixes landed; measuring after the step-4 commit is necessary and is not measure-once.

**examined: 19 of 22.** Three were not read and are named rather than folded into the numerator. `.claude/commands/ductus/audit.md` is the generated mirror of `framework/commands/audit.md`, which was read in full; `scripts/gen-claude-commands.sh --check` ran in this pass and reported all sixteen commands in sync, which is grounds to believe the mirror correct and is not a read. `.github/workflows/markdown-only-pipeline.yml` **does not exist** — spec 048 removed it and the step now runs as step (h) of `framework-checks.yml`; it stays in scope because the plan's Affected Files lists it. `.github/workflows/runtime-release.yml` was read only in the region that is 026's subject — the `audit` job and its `fetch-depth: 0` comment — plus a `continue-on-error` grep across all five workflows; the remaining 480-odd lines are the build and publish matrix. 026's own `review.md` joins this window as soon as this record is committed; it is the output of the run, not a subject of it, and is not counted either.

**Quality — two findings, both fixed in this pass, neither outstanding.**

`QUAL-CLAIM-001` against `scripts/audit/manifest-parity.sh`, fixed in `c33b3085`. Family 2 extracted the MCP tool set from each configure file with a hardcoded prefix grep and went straight to `comm`. Two empty sets compare equal, so a change to the generated entry shape that emptied both greps at once made the family report agreement it never established and exit 0 — as a hard release gate. `check-zero` does not reach it: it runs `gen-configure-mcp.sh --dry-run`, which compares the generator against the files it wrote, and a format change moves both together, while the two spellings this script greps for are a third, independent copy. Every family from 17 onward already fails closed on exactly this, `scripts/audit/README.md` states it as the contract, and 026's own AC16 and AC21 state it for Families 23 and 28. Proven rather than reasoned: breaking one prefix reports one finding, breaking both reports two and exits 1, where before the fix the both-broken case exited 0 in silence. The compared counts now go to stderr.

`QUAL-GROUND-001` against `scripts/audit/sibling-coupling.sh`, fixed in `9c25716d`. `extract_sibling_links` carried its own feature-directory grammar at three-or-more digits, so a body link to a branch-scoped sibling was invisible and the pair went unreported while the family exited 0 — a third copy of the defect `specs/inbox.md` already measures in the two frontmatter derivations, and the only one of the three that is shell rather than a shared parser under `runtime/`. The fix removes the grammar instead of widening it: nothing there has to recognise a feature directory, because the caller matches each candidate exactly against a slug the runtime enumerated, so `..`, `framework` and `rules` can never match. Probed on a fixture carrying all four link shapes — the old pattern drops `1234.1-retry-budget` outright, the new one keeps it. Not reachable in this corpus, which holds no branch-scoped directory; the two runtime copies stay under their existing item, because they route to spec 022 as a scenario and carry a release.

**A third quality question was examined and closed by the criteria walk rather than by a finding.** `scripts/audit/ssot-invariants.sh` is a stub — it sources the library, names itself, and exits 0, with no grep, no comparison and no reachable emitter. What made that a defect was AC8, which asserted detection the family has never performed; that criterion now says so, with the evidence that it was false at delivery rather than superseded. With the claim corrected, the surrounding contract no longer implies work: the script's header, the criterion, and the plan's known-limitation entry all say the detection is deferred, and neither `audit.md` nor the suite's README attaches a behavioural claim to it. Whether Family 6 is retired the way Family 3 was, or operationalised, is a design decision about the audit's coverage and is an operator's to make, not a backfill pass's — recorded here and in AC8 rather than taken. Worth stating because it is not free: within the shipped aggregator a family cannot say anything on a clean run, since `run_check` prints only on a non-zero exit, so the only loud signal available to a family is a finding on every run — which is the permanently-red gate this suite's own scoping arguments reject.

**Security** — nothing. The families read files under the repository and shell out with argument vectors; none takes external input. Family 22 is the only one that writes, and it writes only inside a `mktemp -d` fixture it removes, stubs the runtime so the family stays hermetic, and pins `PATH` to `/usr/bin:/bin` when it runs the shipped hook. The two fixes this pass landed narrow rather than widen what is read.

**Reuse** — the direction is right and the two remaining copies are named above. `lib.sh` holds the repo-root resolution, the drift accumulator, the emitter, the runtime-resolution tiers and the maintainer-only list once each; Families 26, 30, 31, 34 and 35a are entry points over runtime primitives rather than second implementations; and the Family 7 fix removed the last hand-rolled feature-directory grammar outside `runtime/`.

**Efficiency** — nothing. Every family in scope is a bounded walk over a corpus of a few hundred files, and the whole suite returns in seconds. Family 22 builds five fixtures, which is the cost of testing the shipped hook against every directory form rather than the one this repo happens to use.

**Simplicity** — nothing to argue. The v1 families predate the primitives-first standard and the suite's README says so in as many words, naming them the legacy shape rather than the pattern to copy; converting them is a separate decision with its own measurement, and this pass did not make it.

Verified before recording: `markdownlint-cli2` clean over 509 files; the six lint scripts and both `scripts/tests` suites green; `shellcheck -S warning` clean over all 55 shell files; the three generators and both frontmatter derivations reporting no drift; `check-orphaned-references`, `check-corpus-links` and `check-step-references` each at 0 findings; `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked` green with 20 of 20 `test result:` lines and no failures; and `scripts/audit/run-all.sh` exiting 0 **after** each commit. The audit was proven to fail as well as to pass — a wrong value in the repo-root version file reddened Family 20 with exit 1 and two findings, and the tree was restored and re-verified.

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
