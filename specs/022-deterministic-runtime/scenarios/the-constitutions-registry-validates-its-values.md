---
section: "Follow-on scenarios"
---

# The-constitutions-registry-validates-its-values

## Context

`framework/bootstrap/ductus.md` §Validating the registry states that each `[constitutions.<alias>]` entry is validated when the config is read, and that the two failure modes carry deliberately different severities. A **malformed entry halts** per `CFG-ENV-003`, naming the offending alias and field: an alias that is not a bare TOML key, a `repo` that is not URL-shaped, or an empty `path`. A `path` that does not **resolve** only warns — that is the `not-checked-out` state, correct for any contributor who has not cloned the governance repository yet. [055](../../055-shared-constitution/spec.md)'s `data-model.md` declares the same three constraints per field.

Nothing enforced the halt. `runtime/src/schema/constitutions.rs`'s `from_toml_str` did TOML parsing and serde required-field enforcement only — its own module doc said as much — and `resolve_constitutions::classify` reads the filesystem without ever inspecting a value. `resolve-constitutions` is the only deterministic surface that reads the registry, so `/{project}:target`, `write-review` and `write-analysis` all accepted entries the bootstrap says are rejected.

Probed 2026-09-13 against the `ductus-v0.49.2` release binary. `repo = "not-a-url-at-all"` over a resolvable checkout resolved `loaded` with no complaint at all. `path = ""` resolved to the repository root — which exists, and holds no `constitution.md` — and was reported `no-constitution-document`. The second answer is worse than silence, because it is confidently **wrong**: it tells the operator their checkout lacks the document when the real mistake is an empty path in their own config, so it misdirects the fix as well as missing it. That is the exact conflation 055's Edge Cases forbid — *"an operator who cloned the wrong repository must not read the same message as one who cloned nothing"* — and [§design-principles](../../../framework/constitution.md#design-principles)' bar that a check which cannot run must never be indistinguishable from one that passed.

055's AC6 is **not** falsified by any of this. It scopes rejection to configuration time, and the bootstrap prose does specify it; what was missing is the enforcement, not the requirement. This is unimplemented deterministic validation rather than a stale criterion, and the checks are pure value inspection — deterministic, already specified as prose, and therefore runtime-eligible by [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)'s three criteria, where eligibility is a default rather than a permission.

## Behavior

The three value checks the bootstrap declares are enforced, and they run **inside the registry's parser** rather than beside it: `Constitutions::from_toml_str` validates every entry before returning, so a registry built from a config document is validated by construction. A separate `validate()` a caller had to remember to invoke would be the diligence dependency §design-principles rejects, and the failure would be silent — an unvalidated registry resolves perfectly well and simply answers wrongly, which is the state this scenario exists to end.

An entry is malformed when its alias is not a bare TOML key (ASCII letters, digits, hyphens and underscores, and non-empty), when its `repo` is not URL-shaped, or when its `path` is empty. `repo` is shape-checked and never reached: a scheme matching RFC 3986's grammar, the `://` separator, and a non-empty host in the authority once any `userinfo@` prefix and `:port` suffix are removed. Shape is the whole requirement, because `repo` is identity and navigation only and is never fetched.

A rejection names the alias and the field, because nothing else in the run ever will. `resolve-constitutions` surfaces it as its own operational error, distinct from the TOML parse error that covers a document which will not parse at all: that one carries serde's location and needs no alias, while this one describes an entry that parsed cleanly and would otherwise be invisible. The halt is what `/{project}:target` and `/{project}` meet, and a halt must never render as the empty-registry case, which reports nothing.

Only the **first** violation is reported, in a stated order — entries in alias order, and `alias` then `repo` then `path` within an entry. One message rather than a list, because the entry halts: the operator fixes it and re-runs, and the next violation surfaces then. Stating the order is what keeps two machines reporting the same one.

Validation touches no filesystem, and that is the asymmetry rather than an implementation detail. A `path` that does not resolve is **not** checked here and stays the warning-level `not-checked-out` state, because it is a machine-local condition that is correct for a contributor who has not cloned the governance repository yet, and blocking on it would make the pipeline a hard dependency on someone else's repo state. A malformed entry is a mistake in the project's own committed config and is always wrong. Collapsing the two would tell an operator to clone a repository when the mistake is in their config — which is precisely the failure probed above.

The two primitives that read the registry as an *input* rather than as their subject keep their existing posture, deliberately and unchanged. `write-review` renders the unreadable registry as an `## Unexamined governance` bullet and `write-analysis` records it under the `constitution-registry-unreadable` reason; neither raises, because each one's contract is that it records a run rather than fails it, and a config typo must not cost the operator their findings. A malformed entry reaches those two as exactly that state, with the alias and field in the message.

## Edge Cases

- **A `path` that does not resolve.** Unchanged: `not-checked-out`, warned and reported as unexamined, exit 0. This is the case the asymmetry exists for and is asserted rather than assumed.
- **A checkout holding no `constitution.md`.** Unchanged: `no-constitution-document`. The reason is now reachable only when it is true, which is what makes it worth distinguishing from the one above.
- **A whitespace-only `path`.** Empty for this check. It is the same operator mistake, and the bootstrap says so in as many words rather than leaving it to be inferred from the word *empty*.
- **A quoted alias.** `[constitutions."my org"]` and `[constitutions."a.b"]` are legal TOML and reach the registry carrying whitespace or a dot. Rejected: the alias is what every report names the source by, and one carrying a space reads as two fields in a single-line report.
- **An scp-style git address.** `git@github.com:acme/gov.git` carries no scheme and is rejected. That is the schema as declared, not an oversight — `ssh://git@github.com/acme/gov.git` says the same thing, is navigable, and is what the bootstrap now tells the operator to write.
- **An authority with a port and no host.** `https://:8080/g` is rejected; a non-empty authority is not a host. A bracketed IPv6 literal (`https://[::1]:8080/g`) is accepted, because the port is stripped only when what follows the last colon is all digits, which leaves the literal intact.
- **Two malformed entries.** One message, for the first in the stated order. The second surfaces on the re-run after the first is fixed.
- **A config that will not parse.** Still the TOML parse error, never a value rejection. An omitted required field is serde's to report; the value checks own only what parsed.
- **`description` is not validated.** It is free text with no resolution behavior, already whitespace-collapsed by the primitive so a multi-line value cannot break a single-line report. Nothing about it can be malformed.
- **This repository registers nothing.** `.ductus/config.toml` here carries no `[constitutions]` table, so the change is inert for ductus itself; every existing fixture and test config in the tree uses a URL-shaped `repo` and a non-empty `path`, which was checked rather than assumed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
