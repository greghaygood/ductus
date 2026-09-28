# 061 — Updates track the latest release tag Data Model

The structures this spec adds: one configuration table, one argument grammar, the resolved source a run carries, and one release asset. None is a runtime type. The bootstrap and the installer carry all of them in prose and in POSIX `sh`.

## `.ductus/config.toml` `[source]`

A new table, documented here under the configuration-file rule in §cross-spec-impact. It is also listed in the bootstrap's §Project Configuration example. It is written as a `# ductus (source)` line-prefix managed block by `merge-managed-block`, into the active config file (`framework/bootstrap/ductus.md`, §Project Configuration write policy).

```toml
# ductus (source)
[source]
ref = "main"
```

| Key | Type | Allowed values | Absent means |
| --- | --- | --- | --- |
| `ref` | string | `main`, or `ductus-v<MAJOR>.<MINOR>.<PATCH>` | the latest release |

- **Written by** a `/ductus` run given `--ref=main` or `--ref=<tag>`, after the ref has passed every check and the runtime has been acquired, and before the self-update check.
- **Rewritten without `ref`** by `--ref=latest` when a block exists. A bare `[source]` table is the default. With no block present, `--ref=latest` writes nothing.
- **Never written by the installer**, which reads and writes no project configuration.
- **A value outside the grammar halts the run**, naming the value, `.ductus/config.toml` as its origin, and the accepted forms. `latest` is a flag value: `/ductus` never records it, because the record's absence already means it. A hand-written `latest` is inside the spec's grammar (Failure behavior), so it does not halt and reads as the default.
- **Consulted only when no `--ref` is given.** A `--ref` replaces the record, so a record outside the grammar is repaired by re-running with `--ref` rather than halting that run.

## `--ref` grammar

Shared by `/ductus` and the installer.

```text
--ref=latest | --ref=main | --ref=ductus-v<MAJOR>.<MINOR>.<PATCH>
```

- Accepted in any position beside the other arguments: the project name, `--agents` and `--add-agent` for `/ductus`, and the agent key for the installer.
- Given more than once, the run halts, naming every value.
- An empty value (`--ref=`) or any other form halts, naming it.
- The `<MAJOR>.<MINOR>.<PATCH>` part is SemVer without a pre-release or build suffix, which is the form every `ductus-v*` tag carries.

## Resolved source

What Source resolution settles once per run and every later step reads.

| Field | Values | Notes |
| --- | --- | --- |
| origin | `flag`, `recorded`, `default` | `--ref`, then `[source] ref`, then neither |
| kind | `main`, `tag` | `latest` resolves to a `tag` |
| tag | `ductus-v<SemVer>` | Present when kind is `tag` |
| `{raw-ref}` | `main`, or the tag | Path segment for `raw.githubusercontent.com` |
| `{archive-ref}` | `refs/heads/main`, or `refs/tags/<tag>` | Path segment for `codeload.github.com` |
| `{source-label}` | e.g. `latest release ductus-v0.55.0`, `main (recorded)`, `ductus-v0.55.0 (--ref)` | The reported source line (AC5) and the self-update notice (AC14) |

## Floors

Both apply to any resolved tag, whether it came from `latest` or was named. Neither applies to `main`.

| Floor | Value | Source | Check |
| --- | --- | --- | --- |
| Release floor | `{ref-floor}`: the first release that carries this spec | A constant in `framework/bootstrap/ductus.md` and `install.sh`, whose agreement Family 14 asserts | SemVer less-than on the tag |
| Migration floor | The `introduced_in` of the project's `[migrations] last_applied` | The tag's `framework/migrations.toml`, then `main`'s | Halt when the id is absent at the tag and present at `main`; pass when present at the tag, absent at both, or `last_applied` is null |

The installer checks the release floor only: the migration floor reads project configuration.

## Release asset

| Asset | Added by | Checked by |
| --- | --- | --- |
| `install.sh` | `release-assets`, from a checkout at the tag, uploaded with the complete set | The complete-set assertion, and a post-release job comparing `releases/download/<tag>/install.sh` byte-for-byte with the tag's copy |

The installer carries no `.sha256` sidecar. It is fetched over HTTPS and piped to `sh` like the raw-file installer it replaces, and the spec requires no digest for it.
