---
spec: 056-bootstrap-archive-boundary-split
reviewed-at: 2026-09-15T13:51:08Z
reviewed-against: 3d84712f0b5e75afb690c7f81c455be87b076bc6
diff-base: a345df4dcf16403c81adc189b43e6caa8069455b
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 6
scope: 7
skipped-passes: []
---

# Review — 056-bootstrap-archive-boundary-split

## Summary

Five passes over 6 of the 7 in-scope files, against the 11 rule files `discover-rule-files` selected. 0 MUST, 0 SHOULD, 0 low-confidence.

**Not read, and why.** `framework/bootstrap/govern.md` is a byte-identical copy of `framework/bootstrap/ductus.md`, held so by audit Family 21 and confirmed here with `cmp` and by running that family. Believing it correct and having read it are different claims, so it is named here and excluded from `examined` rather than folded into the numerator.

**What the passes actually did.** The subject is a cut-and-paste of nine level-2 sections between two files, so the passes were aimed at what that can break rather than at code patterns. The nine moved sections were verified byte-identical to their pre-split source by sha256, before and again after the one repair below. Every excision seam in `ductus.md` and every block join in `ductus-procedure.md` was read directly and is `content / blank / heading`. Directional wording was scanned for targets that left the file — none broke; `Post-Write Integrity Check ... below` still points below.

**Security.** The rule set is about application code and fires nowhere here, but the one security-relevant question the split raises was checked directly: §Runtime acquisition's digest verification and its *a missing sidecar is a failure here* clause both stay in the **installed** half (1 occurrence in `ductus.md`, 0 in the archive half), so the integrity step an adopter depends on is still in the file they install and byte-compare. The trust surface is otherwise unchanged: the framework archive already supplied executable migration procedure bodies and is already fetched without a sidecar digest, so moving prose into it introduces no path an attacker did not already have — the self-update byte-compare is a staleness check against the same origin, not an integrity check.

**One defect found and fixed in-pass.** The split gave the installed half a *where a named section lives* convention and not its mirror, leaving 13 references across 8 sections pointing from `ductus-procedure.md` back into `ductus.md` with nothing saying where they resolve. Measured, not estimated. It fell inside the spec that was open, so it was fixed rather than captured (constitution §brownfield-inbox), in `3d84712f`, and the nine sections were re-hashed afterwards to confirm the repair touched no moved content. It maps to no loaded rule and is recorded here rather than as an observation, because capturing work already done would put a closed item in the inbox.

**Reuse / efficiency / simplicity.** No duplication: the move was verified byte-identical, so no content exists twice, and the `govern.md` copy is required by Family 21 rather than incidental. The two convention notes state complementary rules in opposite directions and cannot be shared, since each file is read without the other. The change is itself the efficiency result — 36,536 bytes off every curl, install, context load and byte-compare. The single pointer was preferred to five in-place stubs, which would have added back bytes and created five places to drift.

**Known limitation, already recorded in the plan's Trade-offs rather than left here.** Nothing detects a future section landing on the wrong side of the boundary, and the pointer names its nine sections by name, so a rename would stale it silently. That is prose discipline, stated rather than implied.

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
