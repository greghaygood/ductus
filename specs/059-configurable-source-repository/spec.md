---
status: in-progress
dependencies: []
next-criterion: 9
---

# 059 — Configurable Source Repository

Make the repository the `ductus` bootstrap fetches from **configurable**, so the
framework can be adopted, developed, and tested from a fork — not only from the
canonical `stonean/ductus`. A `DUCTUS_REPO` environment variable (default
`stonean/ductus`) parameterizes the four exec-time fetch sites, so a fork test
and the canonical adoption are one code path; unset, every rendered fetch URL is
byte-identical to today.

## Motivation

`ductus` is live-on-main: `/ductus` and `install.sh` fetch the bootstrap, the
framework archive, the version pin, and the runtime release asset from the
canonical repository, spelled as a hardcoded `stonean/ductus` across four fetch
sites. That made the framework **unforkable**: a contributor checking out their
own fork (to test a change before proposing it upstream, or to adopt a private
variant) could not run `/ductus` from it, because every fetch resolved to the
canonical origin regardless of where the invoking files physically came from.
This blocked the two workflows that matter most for an open framework: *test the
change before proposing it* and *adopt an unreleased main from the fork you are
developing on*. The self-url audit family
(`scripts/audit/self-url-resolution.sh`) already anticipated this — its header
derives the canonical repository slug from the installer's archive URL rather
than hardcoding it, precisely so a fork's identity is not "wrong" but "resolved":
*"Hardcoding `stonean/ductus` here would make every finding wrong in a fork
rather than merely absent."* The fetch sites never caught up with the audit.

## Behavior

- **`$DUCTUS_REPO` names the source repository.** The four exec-time fetch sites
  — (1) the `install.sh` bootstrap fetch, (2) the version pin fetch, (3) the
  runtime release download, and (4) the framework archive fetch — resolve their
  repository component from the `DUCTUS_REPO` environment variable, defaulting to
  `stonean/ductus` when the variable is unset or empty. With the default, every
  fetch URL renders byte-identically to the pre-059 text; the audits stay green
  because they grep the committed text, whose defaults still spell the canonical
  repository.
- **All four sites use the same value, consistently.** A value that names one
  site but not another would half-fork the adoption — the archive from the fork
  paired with a version pin from the canonical repo (a divergence that can
  silently resolve to a mismatched pin) or a runtime download that 404s. The
  parameterization is one variable for all four.
- **`self-url-resolution.sh` derives, and keeps deriving.** The family's
  `<owner>/<repo>` slug comes from the archive URL in `framework/bootstrap/ductus.md`.
  The default text still contains `stonean/ductus/archive/…`, so the derivation
  is unchanged; a fork that changed the *default* would need to change the URL
  it derives from — which is correct behavior (the fork's identity is the fork's
  archive URL), not a broken audit.
- **`{placeholder}` recognition stays exact.** The runtime release download URLs
  carry `ductus-v{pin}` / `ductus-{triple}` placeholders, which the self-url
  family excludes by construction. Parameterizing the repository component must
  not disturb the placeholder shapes those exclusions rely on.

## Design Decisions

- **Environment variable over config file.** `DUCTUS_REPO` is read at fetch
  time, before any `.ductus/config.toml` exists (the bootstrap creates it), so
  the variable is the only mechanism that can guide the first fetch. Config-file
  keys (`[source] repo = …`) remain the obvious follow-on for persistent
  overrides; this spec deliberately does not add one — the variable alone covers
  the fork-test and adoption paths, and a config key would require the bootstrap
  to read its own not-yet-created config.
- **Four fetch sites, not every mention of the repository.** Documentation URLs
  (`github.com/stonean/ductus/blob/…` in prose, README links, changelog entries)
  stay as written: they are canonical citations, not fetches, and a fork
  adoption's *prose* citing the canonical location is correct and desirable.
  Only the exec-time fetches parameterize.
- **The `install.sh` fetch and the archive fetch both resolve the variable.**
  `install.sh` fetches the bootstrap from `$DUCTUS_REPO`'s raw URL, and `/ductus`
  fetches the archive from `$DUCTUS_REPO`'s codeload URL — the two entry points
  must agree on the same origin or the adoption silently mixes a fork bootstrap
  with a canonical archive.
- **No runtime change.** The parameterization lives in `install.sh` and the
  bootstrap's markdown fetch instructions (which the host executes), not in the
  Rust runtime, so this ships live-on-main without a release tag.

## Acceptance Criteria

- [ ] AC1: `install.sh` fetches the bootstrap from `raw.githubusercontent.com/$DUCTUS_REPO/main/…`, defaulting to `stonean/ductus` when `DUCTUS_REPO` is unset or empty, and the variable's semantics are documented in its header
- [ ] AC2: The version-pin fetch (`ductus.md` §Pre-flight / Runtime acquisition) resolves the repository from `$DUCTUS_REPO`, default `stonean/ductus`
- [ ] AC3: The runtime release download fetches `…/$DUCTUS_REPO/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz`, default `stonean/ductus`, keeping the `{pin}` / `{triple}` placeholders intact
- [ ] AC4: The framework archive fetch (`ductus.md` §File Fetching) uses `$DUCTUS_REPO`'s codeload URL with the same default
- [ ] AC5: All four sites use a single variable with a single default — no site hardcodes a different repository
- [ ] AC6: `self-url-resolution.sh` still derives the canonical slug from the archive URL (default text unchanged), passes on the default tree, and reports no footing loss
- [ ] AC7: The staleness/self-update fetch (which compares the installed bootstrap against upstream) resolves the same `$DUCTUS_REPO`, so a fork adoption sees its own upstream rather than the canonical one
- [ ] AC8: `ductus.md` documents `$DUCTUS_REPO` in its pre-flight / fetch sections and in §Project Configuration's environment-notes (as appropriate), and `govern.md` remains byte-identical to `ductus.md`

## Resolved Questions

- **Env var vs config key** — settled above: the variable must exist before the
  config file, so the variable is the mechanism; a `[source]` config key is a
  documented non-goal for this spec.
- **Prose URLs stay canonical** — a fork's documentation citations should still
  point at the canonical repository; only fetches parameterize.
- **Self-url family behavior in a fork** — the family already derives the slug
  from the archive URL, so a fork with a fork default just resolves its own
  identity. No family change required; the AC above pins that the default tree
  is unaffected.
