# 031 — Agent MCP Wiring Data Model

This feature modifies the **Agent Registry** schema established in
[012-multi-agent-govern](../012-multi-agent-govern/spec.md) and
[028-antigravity-agent](../028-antigravity-agent/spec.md). It removes MCP wiring from
the layout-derived value set and introduces a **per-agent MCP registration descriptor**.

## Per-agent MCP registration descriptor

MCP discovery is no longer derived from `layout`. Each registry agent gains a descriptor
with three fields:

| Field | Type | Meaning |
| --- | --- | --- |
| `target` | string (path) | Where the agent reads MCP server definitions. May be a repo-relative path or a `~`-rooted home path. |
| `scope` | enum `project-committed` \| `user-global` \| `home-level` | Whether `target` lives in (and travels with) the repo, or in the user's home and is shared across all their projects. |
| `mechanism` | enum `write-file` \| `surface-instruction` | How ductus's State-B auto-wire registers the server: write the file directly, or surface a copy-pasteable instruction the user runs. |

`scope` and `mechanism` are correlated but distinct: `project-committed` ⇒ `write-file`
(ductus owns a repo file); `user-global` / `home-level` ⇒ `surface-instruction` (ductus
must not mutate the user's home, per the spec's posture decision).

## Per-agent values

| key | `target` | `scope` | `mechanism` | Surfaced instruction (when `surface-instruction`) |
| --- | --- | --- | --- | --- |
| `claude` | `.mcp.json` (repo root) | `project-committed` | `write-file` | — |
| `auggie` | `~/.augment/settings.json` | `user-global` | `surface-instruction` | `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"` |
| `antigravity` | `~/.gemini/config/mcp_config.json` | `home-level` | `surface-instruction` | edit `~/.gemini/config/mcp_config.json` (add the `ductus` block running `~/.ductus/bin/ductus`), then `/mcp` reload |

The surfaced commands name the **absolute store path**, not the bare command: `048-govern-acquired-runtime`
made the runtime ductus-owned and `PATH` is never consulted. Absolute rather than the
repo-relative pointer because a `user-global` / `home-level` config is per-machine and
serves every project, so no project-relative path could be correct in it. Adopters still
carrying the bare `ductus` form are repointed by the `runtime-store-path` entry in
`framework/migrations.toml`. Both rows read `--command ductus` as delivered in June; the
values above are what ships.

### Antigravity: resolved by live-`agy` verification

The descriptor above was settled by testing the live `agy` CLI (see
[scenarios/antigravity-mcp-verification.md](scenarios/antigravity-mcp-verification.md)):
project-local `.agents/mcp_config.json` is **not loaded** (0 server spawns, 0 MCP log
references across two runs), while a home-level control at
`~/.gemini/config/mcp_config.json` **loads** (sentinel spawned, 19 MCP log references).
Issue #60 confirmed. Antigravity is therefore `home-level` / `surface-instruction`. ductus
stops writing `.agents/mcp_config.json` going forward; any already-written copy is **inert
cruft left in place** — agy ignores it, so no destructive cleanup migration is warranted
(symmetric with how ductus leaves Auggie's stale `.mcp.json`).

## Server entry shape

Every target in the table above — repo file or home file — uses the same `mcpServers` map
keyed by server name; only the file location and the command path differ.

```json
{ "mcpServers": { "ductus": { "command": ".ductus/bin/ductus", "args": ["mcp"] } } }
```

The `command` is the **repo-relative pointer** for a `project-committed` target (a shared
committed file must resolve for every contributor and every CI checkout, so a
machine-specific absolute path would break them) and the **absolute store path**
`~/.ductus/bin/ductus` for a `user-global` / `home-level` one. Both replaced the bare
`ductus` this spec delivered, per `048-govern-acquired-runtime`.

**This shape is not universal across the registry.** `032-opencode-agent` added a fourth
agent whose entry lives under an `mcp` key as a typed local-server object rather than in
`mcpServers`:

```json
{ "mcp": { "ductus": { "type": "local", "command": [".ductus/bin/ductus", "mcp"], "enabled": true } } }
```

So a `write-file` agent's server-entry shape is a per-agent property like its target, not a
constant. `framework/bootstrap/ductus.md` §MCP wiring is the canonical statement of both
shapes and of the additive merge that writes them.

## Relationship to the existing registry

- The `MCP-wiring file` row is **removed** from `framework/bootstrap/ductus.md` §Derived
  values (the *layout* table) — it was the source of the conflation.
- The descriptor above is added as a **per-agent** table (keyed by registry `key`, like
  `config_dir`), not a layout-derived value.
- Adding a new `claude-style` agent is therefore no longer a pure one-row append: it also
  needs an MCP registration descriptor entry, because MCP no longer rides `layout`.

## Notes

- The permission grant (`mcp__ductus__*` / `mcp:ductus:*` / `mcp(ductus/*)`) is **independent**
  of this descriptor and unchanged — it lives in the project-level settings file every
  agent reads, regardless of where the server itself is registered.
- The `ductus mcp` server is project-agnostic (operates on the working directory), so a
  single `user-global` / `home-level` registration serves every project — the user runs
  the surfaced instruction once per machine.
