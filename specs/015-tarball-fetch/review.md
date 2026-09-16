---
spec: 015-tarball-fetch
diff-base: 74ae1723556897c5ea696d612a7b6c2c064793f3
captured-issues: 0
skipped-passes: []
last-run: 2026-09-13T13:12:55Z
reviewed-against: 74ae1723556897c5ea696d612a7b6c2c064793f3
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
reviewed-digest: {}
blocking: false
---

# Review — 015-tarball-fetch

## Summary

Re-review of 015 to record `examined` against a derived `scope`; the prior record (2026-06-12) predated both fields, so its `0/0/0` could not be distinguished from a run whose five passes never fired. No MUST or SHOULD violation. **All thirteen criteria verified against the tree and every one holds** — the first spec in this campaign to need no correction to its own claims.

**What this review read: the whole scope.** `framework/bootstrap/ductus.md` — 1181 lines, 141 KB — read in full. It is the entirety of this spec's subject: the plan's Affected Files lists one file and records "No other files change."

**On the diff base, which is the interesting decision here.** Unlike the earlier specs in this campaign, 015 *does* have a recorded `in-progress` transition, so `compute-review-scope` derives a real base — `f4985f45` (2026-06-11). That resolves an **813-file** window: every change in the repository since June, because the spec has not been reopened since and the base does not move on its own. Basing on the last review instead (`7e19b69`) gives 799 — no better. Either would describe a review nobody performed, which is the failure AGENTS.md records from the 800-file window. The override earns its place precisely as that entry prescribes: the resolved window is genuinely wider than the change being re-reviewed. `HEAD` is passed, so the scope resolves to the plan's Affected Files and numerator and denominator describe the same one file. Worth recording that the subject *has* moved since the last review — 25 commits touched `ductus.md` in that span — so this is not a re-read of unchanged text.

**How each criterion was checked, against the file rather than against the spec's own prose.** AC1: §File Fetching is the archive-fetch + extract + per-file-resolution flow, with the per-file `curl` loop gone. AC2: "Issue exactly one `curl` against GitHub's archive host", and the per-language `.gitignore` fetches are explicitly excluded from the archive. AC3: §Per-file resolution step 3 names all five strategies — `update`, `create`, `skip`, `merge`, `pinned` — and keeps the content-equality semantics. AC4: a failed fetch or extract aborts with the quoted error and does not continue scaffolding, on the stated grounds that partial scaffolding from a missing archive is impossible. AC5: a missing entry warns `Source not found in archive: {source-path}; skipping.` and continues — the per-entry granularity the archive flow was required to preserve. AC6: §Cleanup logs the path in the post-scaffolding summary and on abort, and does not delete it. AC7: `tar` and `mktemp` are present in **all four** registry rows — Claude's `Bash(...)` allows, Auggie's `^tar`/`^mktemp` regexes, Antigravity's `command(...)` forms, OpenCode's bash map — and no `rm` allow exists in any of them, which is the half of the criterion easiest to let slide. AC8: Claude's template carries exactly the six `Read(...)` globs, both leading-slash forms across `/private/var/folders`, `/var/folders` and `/tmp`. AC9: §Post-Write Integrity Check re-reads `{tempdir}/ductus.md.upstream` or `{tempdir}/ductus-main/framework/bootstrap/ductus.md` — local sources, no second network call. AC10: §Self-update check compares the installed copy against the fetched upstream and still fires. AC11: the gitignore fetches remain separate `curl` calls. AC12: `update` reports `unchanged` on identical content. AC13: one `mktemp -d -t ductus-XXXXXX` per run in the Pre-flight Phase, reused by the archive step rather than duplicated, and never reused across runs.

**The one skipped target, checked by hand.** `check-artifacts` records AC11's `github.com/github/gitignore` as `root-absent` — correctly, since it is an external host rather than a repo path, so nothing about it is provable from this tree. That criterion was therefore verified by reading §File Fetching, which states the gitignore patterns "remain separate `curl` calls". Per AGENTS.md, a skipped entry is the manual worklist, not noise.

**One correction, and it did not reopen the spec.** The §References preamble credited `scripts/gen-spec-deps.sh` — deleted by 022's adopter-generator-promotion — with deriving `dependencies:`. The identical sentence stood in ten specs, so it was swept in one uniform substitution (`74ae172`), which is a mechanical edit under §spec-lifecycle case (a); the diff carries two distinct lines across ten files and every spec stayed `done`.

**Checked and clean elsewhere.** `README.md`, `AGENTS.md`, `CLAUDE.md` and `docs/` carry no mirror of this spec's claims; the only `.tar.gz` mention in docs is `runtime.md`'s release-asset sentence, which is the runtime's own distribution and not this archive.

015 has no scenarios and no data model, so `reviewed-digest` is `{}` — taken and empty, which reads as current.

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
