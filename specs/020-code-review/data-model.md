---
status: draft
---

# 020 — `/ductus:review` Data Model

Data structures introduced by [020 — `/ductus:review`](spec.md). Authoritative shapes; the spec body and `framework/commands/review.md` reference these. (The spec's embedded copy of that command source was replaced by a pointer on 2026-09-13 — the live source is the authority for command behavior, per §drift-prevention's canonical-sources rule.)

## Spec frontmatter `review:` block

Added to every spec's YAML frontmatter. Lazy-populated — the block is shipped in templates with safe defaults; existing adopter specs gain it on first `/ductus:review` run.

```yaml
review:
  last-run: 2026-05-10T14:32:00Z       # ISO 8601; null until first review
  reviewed-against: <sha>              # HEAD SHA at review time; null until first review
  must-violations: 0                   # post-waiver count
  should-violations: 3
  low-confidence: 2
  blocking: false                      # true iff must-violations > 0
  waivers: []                          # see "Waiver record" below; omitted entirely when empty
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `last-run` | ISO 8601 timestamp or null | yes | Set by `/ductus:review`. Null in templates and on un-reviewed specs. |
| `reviewed-against` | string (Git SHA) or null | yes | HEAD SHA at review time. Null in templates. |
| `must-violations` | integer ≥ 0 | yes | Count after waivers applied. |
| `should-violations` | integer ≥ 0 | yes | Advisory severity count. |
| `low-confidence` | integer ≥ 0 | yes | Quality-pass findings below 80 confidence. Excluded from `must-violations`. |
| `blocking` | boolean | yes | MUST equal `must-violations > 0`. Read by `/ductus:implement`, `/ductus:analyze`, CI template. |
| `examined` | integer ≥ 0 | no | How many in-scope files the five passes actually **read**. Recorded as absent rather than zero when unstated — an unstated claim and a stated zero are different. Added by 022; see AC15. |
| `scope` | integer ≥ 0 | no | The denominator `examined` is asserted against, derived by `write-review` rather than supplied. Added by 022. |
| `reviewed-digest` | map of path → digest | no | Per-path digest of the spec's **durable contracts** (`scenarios/*.md` and `data-model.md`) as the run read them. What freshness is computed from; absent on records predating it, which report freshness as undeterminable rather than current. Added by 022. |
| `waivers` | list of waiver records | no | Omitted entirely when empty. Schema below is open per §text-first-artifacts. |

### Validation severity

- **Hard fail** — block missing on a `done` spec; `blocking` not equal to `must-violations > 0`.
- **Blocking** (per §text-first-artifacts validation severity): `done` spec with `blocking: true` or missing `last-run`.
- **Informational** — unknown fields under `review:` (open-schema rule).

## Waiver record

One entry per waived MUST violation. Lives under `review.waivers` in spec frontmatter. The list itself is open-schema — adopters MAY add fields like `co-waived-by`, `approved-by-team`, `ticket`.

```yaml
- rule: SEC-BE-014
  file: src/api/internal.ts
  reason: "Endpoint is internal-only behind mTLS; rule applies to public APIs"
  waived-at: 2026-05-10T14:40:00Z
  waived-by: dev@example.com
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `rule` | string (rule ID) | yes | E.g. `SEC-BE-014`. Must reference a known rule at write time. |
| `file` | string (relative path) | yes | The path the waiver is anchored to. Waiver expires when the file moves or is deleted. |
| `reason` | string | yes | Free-text justification. Empty string is invalid. |
| `waived-at` | ISO 8601 timestamp | yes | Set by `/ductus:review --waive`. |
| `waived-by` | string (email) | yes | Sourced from `git config user.email`. |
| (additional fields) | any | no | Open-schema; ignored by `/ductus:review` and `/ductus:analyze`. |

### Expiry rule

A waiver expires (is dropped from frontmatter on the next `/ductus:review` run) when **either** of the following holds:

- The `file` path no longer exists in the repository (renamed or deleted).
- The named `rule` no longer fires at `file` (rule removed, or the violating code was fixed).

When the underlying finding still exists elsewhere in scope after expiry, it re-counts toward `must-violations` and the spec's `blocking` flag flips back to `true`. The detailed edge-case behavior is in [`scenarios/waiver-expiry.md`](scenarios/waiver-expiry.md).

## `review.md` artifact

Written to `specs/NNN-feature/review.md` — one review artifact per spec, regardless of whether the run targeted a feature or a scenario. A scenario-targeted run records which scenario it covered via the `scenario:` frontmatter field. Regenerated wholesale on each run; the most recent `/ductus:review` invocation supersedes any prior report.

### Frontmatter

```yaml
---
spec: 020-code-review
reviewed-at: 2026-05-10T14:32:00Z
reviewed-against: <sha-of-HEAD>
diff-base: <sha of the parent of the in-progress transition commit>
must-violations: 0
should-violations: 3
low-confidence: 2
captured-issues: 0
examined: 8
scope: 11
skipped-passes: []
---
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `spec` | string | yes | Feature slug — matches the parent directory. |
| `reviewed-at` | ISO 8601 timestamp | yes | Set on each run. |
| `reviewed-against` | string (Git SHA) | yes | HEAD SHA at review time. |
| `diff-base` | string (Git SHA) | yes | The SHA where the spec advanced to `in-progress`, or the value passed to `--since=<ref>`. |
| `must-violations` | integer ≥ 0 | yes | Post-waiver count, matches spec frontmatter. |
| `should-violations` | integer ≥ 0 | yes | Matches spec frontmatter. |
| `low-confidence` | integer ≥ 0 | yes | Matches spec frontmatter. |
| `captured-issues` | integer ≥ 0 | yes | Findings this run appended to the inbox. |
| `examined` | integer ≥ 0 | no | Mirrors the `review:` block's field — how many in-scope files the passes read. |
| `scope` | integer ≥ 0 | no | Mirrors the `review:` block's denominator. |
| `skipped-passes` | list of strings | yes | Empty when no flag restricts dimensions. Permitted values: `security`, `reuse`, `quality`, `efficiency`, `simplicity`. |

### Body sections (in order)

| Section | When emitted |
| --- | --- |
| `## Summary` | Always |
| `## MUST violations (blocking)` | Always (empty when none) |
| `## SHOULD violations (advisory)` | Always (empty when none) |
| `## Low-confidence findings` | Always (empty when none) |
| `## Waived findings` | Always (empty when none) |
| `## Skipped passes` | Always (empty when none) |

### Finding record

Each finding under MUST/SHOULD/Low-confidence sections:

```markdown
### MUST: <rule-id> — <one-line summary>

- **File**: `path/to/file.ts:42-55`
- **Rule**: <verbatim rule text from framework/rules/...>
- **Finding**: <one to three sentences>
- **Auto-fixable**: yes | no
- **Suggested fix**: <code block or prose>
```

Findings under **Waived findings** include an additional `**Waived**: <reason from spec frontmatter>` field.

### Idempotency invariant

For a given `(code-in-scope, loaded-rules, spec-acceptance-criteria, scenarios, waivers)` input set, the body of `review.md` is byte-identical across runs. Only `reviewed-at` and `reviewed-against` in the frontmatter are permitted to differ. This is the basis of acceptance criterion 6.

## `.ductus/config.toml [review]` section

New TOML section in the project's `.ductus/config.toml`. Added by `/ductus:review` (with operator confirmation) on the first successful tech-stack alignment check. `.ductus/config.toml` is shared adopter-side state per AGENTS.md (Workflow); this spec documents the section it adds rather than touching spec 019.

```toml
[review]
tech-stack-verified = true
```

| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| `tech-stack-verified` | boolean | `false` (when key absent) | When `true`, `/ductus:review` skips the tech-stack alignment pre-flight on every run until the operator removes the line. There is no auto-reset. |

The section is open-schema; future review-related persisted decisions can land under `[review]` without schema migration.

## Cross-references

- Spec lifecycle and §spec-requirements: [`framework/constitution.md`](../../framework/constitution.md).
- Open-schema rule for frontmatter: [`framework/constitution.md`](../../framework/constitution.md) §text-first-artifacts.
- Security rule ID conventions cited under `waivers[].rule`: [`framework/rules/security-backend.md`](../../framework/rules/security-backend.md), [`framework/rules/security-frontend.md`](../../framework/rules/security-frontend.md).
