---
status: draft
dependencies: []
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
analyze:
  last-run: null
  analyzed-against: null
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 0
  blocking: false
next-criterion: 12
---

# 056 — Bootstrap archive-boundary split

Split `framework/bootstrap/ductus.md` so the file an adopter curls, installs into every agent, loads into context at every invocation, and byte-compares on every run carries only what the run needs **before the framework archive exists**. The remainder ships in the archive, which the run fetches anyway, and is read from the extracted tree.

## Motivation

`framework/bootstrap/ductus.md` was a single file carrying the whole adoption procedure. Four costs compounded, and all four were paid per invocation rather than once:

- The file was **curled** by `install.sh` on every install and refresh.
- It was **installed into every selected agent** — verbatim for `claude-style` and `opencode`, body-wrapped as a skill for `antigravity` — so an adopter on three agents held three copies.
- It was **loaded into the agent's context at every `/ductus` invocation**, whole, before the run knew which of its sections it would need.
- It was **byte-compared against upstream on every run** by the self-update check, and rewritten whenever it differed.

The file's own §Pre-flight Phase existed to keep the restart-requiring checks cheap: it states that neither check "pays the cost of the multi-hundred-KB archive; both run on a small fetch or no fetch". That property held for the *fetches* the phase performs and had eroded for the *file* that performs them.

The **cause was structural rather than editorial**. Composition was measured at 90.2% prose, 7.0% tables, 2.8% fenced blocks, so the reductions a cleanup pass reaches — dead shell fallback, inline permission blobs, retired-release tokens — recover low single digits. Size was not what a dead-code pass would have bought.

## The boundary

### It is execution order, not reading order

The boundary is **derived, not chosen**: the installed half is exactly what the run needs before an archive exists, and the archive half is exactly what cannot run until one does.

The authority for that order is §Instructions — the numbered walker — and **not** the order sections appear in the file. §Instructions **step 1** assembles the walker context, and that context already carries both the project inputs and `manifest-entries`, "the per-strategy list described in **Shared Files** and **Per-Agent Scaffolding**". Step 2 fetches the archive and step 3 extracts it. So three sections that *read* as though they came after the boundary are in fact step-1 inputs:

- §Collect Project Inputs
- §Shared Files
- §Per-Agent Scaffolding

Reading the file top-to-bottom puts these after §Pre-flight abort and suggests they move. They do not. This is the single most important fact about the split, and it is the one a section-order reading gets wrong.

### What stays, and why

| Region | Why it stays |
| --- | --- |
| Frontmatter through §Pre-flight Phase | Runs before any archive exists — the whole point of the phase |
| §Collect Project Inputs | §Instructions step 1 resolves the inputs before the step-2 fetch |
| §Shared Files, §Per-Agent Scaffolding | §Instructions step 1 builds `manifest-entries` from these tables |
| §Post-Write Integrity Check | Executable forward-dependency from §Stale → defer to pre-flight abort, step 2 |

The keep-every-placeholder-literal rule is the other executable forward-dependency (cited from the same stale-write path, step 1). It already lives inside §`ductus` self-installation, which is inside §Per-Agent Scaffolding, so it stays without being moved.

### Measured

Measured 2026-09-15 over `framework/bootstrap/ductus.md` at 146,953 bytes / 1186 lines, read in full:

| Half | Bytes | Share |
| --- | --- | --- |
| Installed (stays) | 87,785 | 59.7% |
| Archive (moves) | 59,168 | 40.3% |

So what an adopter curls, installs, loads and byte-compares drops by **~40%**.

Exactly **two** executable forward-dependencies cross the boundary, both on the pre-flight stale-write path, and both resolve inside the installed half under the assignment above. Every other forward reference is **narrative** — an ordering note, a deferred-output pointer, a "described in §X" — measured at 38 reference occurrences across 13 distinct targets, of which §Closing restart alone accounts for 11. None is a step the installed half executes.

### The recorded figures reproduce

`specs/inbox.md` recorded this split at 82,705 B installed / 58,280 B archive against a 140,985-byte file, with a headline cut of ~40%. Re-derived at that item's own measurement commit `5a519cbb`, the assignment above gives 85,693 / 55,292 and a ~39% cut — within ~3KB per side and one percentage point of the headline.

The residual is **where exactly the line falls among a handful of sections**, which is what §Open Questions settles. The item's structural figures reproduce too: 479 B for the placeholder rule against 479, and 1,097 B for §Post-Write Integrity Check against 1,096.

This is recorded because a first derivation here cut at §Collect Project Inputs on section order alone, produced 61,288 / 85,665, and concluded the recorded figures had been transposed. They had not: the recorded figures encoded the step-1 constraint the section-order reading had not yet found. **A measurement that disagrees with a recorded one is a reason to look for the constraint the record encodes, not yet a reason to correct the record.**

### What the split does not reach

Because the manifest tables stay, the derivation that parses them — `adopter_destinations` in `check_artifacts.rs`, which reads the **Shared Files** tables out of `framework/bootstrap/ductus.md` by path — keeps working untouched. That derivation fails toward an empty set rather than an error, so moving the tables out from under it would have started emitting suppressible findings silently. Keeping them is what makes this a `framework/`-and-`scripts/` change rather than a `runtime/` change carrying a version bump and a release tag.

## Behavior

### What the adopter installs

`install.sh` and the `ductus` self-install step place the **installed half** only. The archive half is never installed into an agent, never curled by `install.sh`, and never byte-compared — it is read from the extracted archive the run fetches at §Instructions step 2.

### What the run reads, and when

The installed half runs through §Pre-flight abort exactly as before. Past the archive fetch the run resolves the archive half from the extracted tree and continues through it. A run that aborts in pre-flight never resolves it — the same set of sections §Pre-flight abort already skips.

### Staleness

The self-update check keeps comparing the installed file against upstream, now over the smaller file. The archive half needs no staleness check and gets none: it is re-fetched on every run, so it cannot be stale. This **narrows** what a stale installed copy can misdescribe — today an installed copy predating a procedure change carries the old procedure, and after this change it cannot.

### The retired bootstrap alias

`framework/bootstrap/govern.md` stays a byte-identical copy of the installed half, which is what audit Family 21 already requires and why it requires it: an unmigrated adopter's self-update byte-compares against that path and **writes whatever it finds** over their installed command. It needs no archive-half counterpart of its own — the archive half is resolved from the archive, which is the same tree for every adopter, so such a run reaches it by the path the installed half names.

### Rationale disposition

Rationale is 17–25% of the file and does not move as one class:

- **Rationale that constrains execution stays** with the step it constrains, in whichever half that step lands. It is what stops an agent doing the tempting wrong thing, and a pointer to it elsewhere would be read after the mistake.
- **Dated incident narrative moves** to the spec that decided the fix, with a pointer back from the procedure.

## Acceptance Criteria

- [ ] AC1: `framework/bootstrap/ductus.md` contains no step that requires the framework archive, and every step it does contain resolves within it — including both executable forward-dependencies named in §The boundary.
- [ ] AC2: The archive half ships under `framework/bootstrap/`, is reachable from the extracted tree by the path the installed half names, and appears in no manifest that writes into an adopter's agent directory.
- [ ] AC3: `install.sh` places only the installed half, for every one of its four agent arms, and its frontmatter-delimiter payload check still passes against it.
- [ ] AC4: The `ductus` self-install step writes only the installed half, at each layout's install path, and the Post-Write Integrity Check's body assertion holds against it for every layout.
- [ ] AC5: The self-update check byte-compares the installed half alone; no staleness check is defined for the archive half, and its freshness is stated to come from the archive fetch.
- [ ] AC6: `framework/bootstrap/govern.md` is byte-identical to the post-split `framework/bootstrap/ductus.md`, so audit Family 21 passes unchanged; no `govern`-named archive half is created.
- [ ] AC7: `adopter_destinations` in `check_artifacts.rs` still derives a non-empty destination set from `framework/bootstrap/ductus.md` after the split, verified by probe rather than by reading, and no `runtime/` source change is required by this spec.
- [ ] AC8: Every audit family whose subject moved is re-pointed at the file that now holds it — at minimum Family 36, which derives this repository's canonical slug from the `archive/` URL that lives in §File Fetching — and each such family reports a finding when its subject is absent rather than treating an empty extraction as agreement.
- [ ] AC9: `scripts/audit/run-all.sh` reports no findings, and the three generators plus `derive-dependencies` and `derive-references` report no drift.
- [ ] AC10: The whole local gate passes: `npx markdownlint-cli2`, the six `lint-*.sh` scripts, `scripts/tests/*.sh`, `shellcheck -S warning` over the tracked shell set, and under `runtime/` `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked`.
- [ ] AC11: The reduction to the installed half is stated in the plan from a measurement taken **after** the split, against the 146,953-byte pre-split file, rather than from this spec's estimate.

## Open Questions

- What is the archive half named, and how does the installed half address it — by a path relative to the extracted framework root, or by one the walker resolves from the staging directory it already holds?
- Exactly which sections land in the archive half? §File Fetching, §Pre-run Migrations, §Frontmatter Migration, §Security Audit and §Post-Scaffolding Output read or follow the extracted tree; §Project Configuration, §Hook Installation, §Placeholder Substitution, §Re-Run Behavior, §What This Command Does NOT Do, §Edge Cases, §Idempotency and §Directory Creation do not obviously belong to either half. §Edge Cases describes both halves in one list and may have to be split or duplicated.
- Does the archive half carry frontmatter and an `## Instructions` section of its own? If it does, Family 34's `BOOTSTRAP_FILES` in `check_step_references.rs` must gain it — which would make this a `runtime/` change after all, with the version bump and tag that implies.
- Does `runtime/tests/parity.rs` stage this file, and does any golden carry bytes from it? Its `read_parity_spec` falls back to a fixture-local `framework/bootstrap/<cmd>.md`, so the question is whether a fixture names this file.
- Does this declare `cross-spec-impact` on the spec that owns the audit families, or is re-pointing a family's subject path a script change that leaves that spec's criteria true?
