# 036 — Cross-cutting code-quality rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Author `framework/rules/quality-cross.md`

- [x] Write the file header: title (`# Code Quality Rules`), an intro stating the discipline is cross-cutting (applies to every stack), the RFC 2119 note, the `QUAL-{CATEGORY}-{NNN}` ID-format / category-declaration line (category `STUB`, with the `See specs/036-quality-cross-rules/data-model.md` + `specs/008-security-rules/data-model.md` pointers), and the pin note adapted for a cross file (always applies; pin in `.ductus/config.toml` `[pinned]` if customized).
- [x] Write the `## QUAL-STUB — Silent stubs` section with `### QUAL-STUB-001`: Statement (MUST), Rationale (silent-stub hazard + the adopter rate-limiter incident), and Verification (review-time three-part discriminator — reachable + contract-implies-work + no-loud-signal — plus the exemption list).
- [x] Cross-reference rather than restate: cite `api-backend.md` `BE-SCHEMA-002` for the build-time schema fail-loud case.
- Done when: the file exists with one well-formed `QUAL-STUB-001` rule (MUST), the `-cross.md` schema, and the `BE-SCHEMA-002` citation.

## 2. Register the `QUAL` surface

- [x] Write `specs/036-quality-cross-rules/data-model.md` registering the `QUAL` surface and `STUB` category, referencing 008's schema (already drafted at plan time — confirm it matches the shipped rule file).
- [x] In `scripts/lint-rule-ids.sh`, extend the allowlist regex `^(BE|FE|CFG)-…` to include `QUAL`, update the error-message string to `{BE|FE|CFG|QUAL}`, and add `specs/036-quality-cross-rules/data-model.md` to the "Source of truth" comment block.
- Done when: `scripts/lint-rule-ids.sh` accepts `QUAL-STUB-001` and still rejects malformed IDs; the data-model and the rule file agree.

## 3. Register the file in the `/ductus` manifest

- [x] Add `framework/rules/quality-cross.md → specs/rules/quality-cross.md` to the `### ductus-owned shared files` table in `framework/bootstrap/ductus.md`, slotted between `performance-frontend.md` and `security-backend.md` (strategy: update).
- [x] Make the §Shared Files "Rule-file surface filter" note count-free (dropped the hard-coded number, which had silently drifted — it read "six" while seven rule files were already listed — and which nothing machine-checks; removing it eliminates the drift class rather than just correcting the value).
- Done when: the manifest row is present, the note no longer hard-codes a rule-file count, and the `-cross.md` suffix makes 024's loader select it for every stack and 033's filter keep it unconditionally.

## 4. Validate

- [x] `scripts/lint-rule-ids.sh` passes (`QUAL-STUB-001` well-formed and accepted; categories disjoint from `BE`/`FE`/`CFG`; no duplicates).
- [x] `scripts/lint-rule-filenames.sh` passes (the `-cross.md` suffix).
- [x] `npx markdownlint-cli2`, the other `scripts/lint-*.sh`, and `scripts/audit/*` pass (frontmatter, tool-coverage, procedure-parseability, manifest-parity, ssot-invariants, cross-doc-consistency — all green).
- [x] `STUB` confirmed disjoint from existing categories; `QUAL` confirmed disjoint from `BE`/`FE`/`CFG` surfaces.
- Done when: all lints/audits green.

## 5. Review and complete

- [x] Run `/ductus:review` over the change set; resolve any MUST findings.
- Done when: `/ductus:review` reports no blocking violations and the spec can advance to `done`.

## 6. Add QUAL-GROUND-001 (verify external contracts)

- [x] Declare the `GROUND` category in the `quality-cross.md` file header and register it in `data-model.md` (category table + `QUAL-GROUND` namespace).
- [x] Add `QUAL-GROUND-001` (SHOULD) to `quality-cross.md` following the canonical schema — the code-side counterpart to `/ductus:analyze`'s grounding check, enforcing constitution §grounding.
- Done when: `scripts/lint-rule-ids.sh` accepts `QUAL-GROUND-001`, markdownlint and `scripts/audit/*` pass, and the rule is registered in both the file header and the data-model. Re-run `/ductus:review` before advancing 036 back to `done`.

## 7. Add QUAL-CLAIM-001 (unsubstantiated clean results)

- [x] Declare the `CLAIM` category in the `quality-cross.md` file header and register it in `data-model.md` (category table + `QUAL-CLAIM` namespace).
- [x] Add `QUAL-CLAIM-001` (SHOULD) to `quality-cross.md` following the canonical schema, with the promotion criterion to MUST documented alongside the Verification paragraph.
- [x] Cite the four originating instances in the rule's Source paragraph so the rule is traceable to observed defects rather than speculation.

- **Done when**: `scripts/lint-rule-ids.sh` accepts `QUAL-CLAIM-001`; the category appears in both the `quality-cross.md` header and `data-model.md` (table + namespace section); markdownlint and `bash scripts/audit/run-all.sh` pass; `/ductus:review` is re-run before 036 advances back to `done`.

## 8. Add QUAL-DELEG-001 (delegation to shared code)

- [x] Declare the `DELEG` category in the `quality-cross.md` file header, per the per-file category-declaration policy
- [x] Register the category in `data-model.md` §Category abbreviations and add the `QUAL-DELEG` namespace entry under §Rule set
- [x] Author the rule as SHOULD, with a Verification clause naming the enumeration a delegation owes and treating a test exercising both the widened and the narrowed direction as compliant
- [x] State the discriminator against `QUAL-CLAIM-001` and `QUAL-GROUND-001` in the Rationale, as `QUAL-CLAIM-001` does against its siblings
- [x] Carry the `050-constitution` signpost in the spec body so the declared cross-spec impact is discharged by a reciprocal link rather than by deleting the key
- [x] Verify: `scripts/lint-rule-ids.sh` passes, `check-rule-ids` reports the ID known against `examined: 11`, and `npx markdownlint-cli2` is clean

- **Done when**: `framework/rules/quality-cross.md` carries `QUAL-DELEG-001` with Statement / Rationale / Verification, the `DELEG` category is declared in the file header and registered in `data-model.md`, `scripts/lint-rule-ids.sh` and `check-rule-ids` accept the ID, and `050-constitution`'s `cross-spec-impact:` declaration is discharged by the signpost this spec carries.
