---
status: draft
dependencies: [003-bootstrap-automation, 015-tarball-fetch, 048-govern-acquired-runtime]
cross-spec-impact: [003-bootstrap-automation, 015-tarball-fetch, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 050-constitution]
next-criterion: 8
---

# 061 — Updates track the latest release tag

`/ductus` and the installer fetch from the latest release tag by default, so an adopter's update lands on a released state rather than on whatever `main` holds when it runs. Both accept an override to `main` or to a named release tag.

## Motivation

Every fetch `/ductus` made back into this repository named `main`. The framework archive came from `codeload.github.com/stonean/ductus/tar.gz/refs/heads/main` ([015](../015-tarball-fetch/spec.md), §Source). The runtime version pin came from `raw.githubusercontent.com/stonean/ductus/main/version` ([048](../048-govern-acquired-runtime/spec.md)). The bootstrap's own self-update came from `main` too (`framework/bootstrap/ductus.md`, §Small fetch). The installer fetched the bootstrap from `main`, and its header stated there was no release-pinning knob. [003](../003-bootstrap-automation/spec.md)'s `curl-sh-installer` scenario recorded the same.

An update therefore landed on whatever `main` held at the moment it ran. A commit that was correct only as part of an unfinished series reached every adopter who updated before the series finished. One example is a framework change that relied on a runtime change not yet released.

The runtime binary was already release-pinned: `/ductus` installed it from a `ductus-v{version}` release. Only the file that chose that release was read from `main`.

015 deferred a configurable ref to a later spec because no adopter had asked for one (015, Resolved Questions). This is that spec.

## Source selection

A run fetches from exactly one of three sources:

- **The latest release**: the default, used when no source is given.
- **`main`**: the override that restores the previous behavior.
- **A named release tag**: for staying on, or returning to, a specific release.

**One run, one source.** Every fetch a run makes back into this repository resolves against the same ref: the bootstrap self-update, the runtime version pin, and the framework archive. A run never mixes them. 048's pin and 015's archive agreed only because both named `main`. This spec keeps that agreement for every source rather than for `main` alone.

The runtime binary still comes from the release the pin names. On the latest release or a named tag, that is the release the source resolved to. On `main`, it is whatever version `main`'s `version` file names.

A run reports which source it used and the ref it resolved to. An adopter reading the output can tell which release they are on without re-deriving it.

## Failure behavior

- **The latest release cannot be resolved.** The run halts, naming what it tried. It does not fall back to `main`. A silent fallback would put the adopter on exactly the unreleased state this spec exists to keep them off, and would read identically to a successful resolution (§design-principles, `QUAL-CLAIM-001`).
- **A named tag does not exist.** The run halts, naming the tag, before anything is written.

## Installer

The installer defaults to the latest release and accepts the same two overrides. It fetches the bootstrap from the ref it resolved, so a fresh install and the first `/ductus` run start from the same release.

## Acceptance Criteria

- [ ] AC1: With no source argument, `/ductus` fetches its bootstrap self-update, the runtime version pin, and the framework archive from the latest release tag
- [ ] AC2: With the `main` override, `/ductus` fetches all three from `main`
- [ ] AC3: With a named tag, `/ductus` fetches all three from that tag, and a tag that does not exist halts the run naming the tag before anything is written
- [ ] AC4: When the latest release tag cannot be resolved, `/ductus` halts naming what it tried and does not fall back to `main`
- [ ] AC5: A `/ductus` run reports the source it used and the ref that source resolved to
- [ ] AC6: The installer defaults to the latest release tag and accepts the same `main` and named-tag overrides as `/ductus`
- [ ] AC7: No live artifact still states that the installer or `/ductus` tracks `main` by default (the prose-claim sweep, §drift-prevention)

## Applicable Rules

- `QUAL-CLAIM-001`: an unresolvable latest release must not read as a successful resolution. The run halts rather than falling back to `main`.

## Open Questions

- **How do framework-only changes reach adopters?** Constitution, rule, command and template changes reached adopters on their next `/ductus` because the archive tracked `main`. 050's Resolved Question *"Does a constitution-only change need a version bump?"* answers *no* on that basis alone. With the latest release as the default, such a change reaches adopters only at a tag. Does every framework change then need a release? And does a framework-only release bump `version`? `/ductus:audit` Family 20 requires `version` to equal `runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading. 048 has the `ductus-v{version}` tag carry that same number. So a bump there declares a runtime release whose assets must be published.
- **What resolves "the latest release"?** GitHub's *latest release*, or the highest `ductus-v*` tag by SemVer? Whichever it is must never select a historical `gvrn-v*` tag, a prerelease, or a draft. The resolution must use hosts `/ductus` already fetches from without a permission prompt, which 015 and 029 both designed around. A tag exists before its release assets do: the tag push triggers `.github/workflows/runtime-release.yml`, which creates the GitHub release in its last job. Does the latest release skip such a tag, or halt on it?
- **What is `/ductus`'s argument syntax for the source?** The inbox item asked for a first positional parameter. The first positional word is already the project name: the argument hint is `[project] [--agents=key1,key2,...] [--add-agent]`, and §Inputs reads the project name as the single non-flag word (`framework/bootstrap/ductus.md`). A bare `main` would be ambiguous with a project named `main`. Is the source a flag? If so, how does it compose with `--agents` and `--add-agent`?
- **What is the installer's syntax, and where does the installer itself come from?** Its first positional argument is the agent key (`sh -s -- claude`). The documented one-liner fetches the installer from `main`. Is that acceptable, given the installer only resolves a ref and fetches the bootstrap? Or does the one-liner move to a release URL?
- **Does the chosen source persist?** If the choice is per-run only, a project that chose `main` or a named tag moves to the latest release on its next plain `/ductus`. If it persists, where is it recorded: committed project config (the `.ductus/config.toml` `[source] ref` 015 anticipated) or per-contributor session state? Either way, what does moving to an older release do? While `main` was the only source, an update never moved a project backward, so neither the bootstrap self-update nor the manifest's overwrite strategies has had to handle that case.
- **Which tags are valid named targets?** Tags that predate 048's `version` file carry no pin to read. Tags that predate 049's rename carry the pre-rename names. Is there a floor below which a named tag is refused, and how is it stated?
- **What does the prose-claim sweep cover, and which hits reopen a spec?** This repository's own workflow justifies trunk-based development by *live-on-main*: `CLAUDE.md`'s Non-negotiables and the `AGENTS.md` Workflow entry *Commit directly to `main`*. Does trunk-based development stay, with its reason restated? `cross-spec-impact:` declares the done specs whose stated behavior changes. Three more done specs carry the claim without being declared: 023's Resolved Question on read-fallback lifetime (*"there is no version a project can pin to"*), and the update stories of 028 and 032 (*"re-scaffold (live-on-main)"*). Is each one a signpost that reopens it, or a correction?
