---
section: "The primitive library"
---

# Open-questions-are-any-list-item

## Context

`read-spec` reports a spec's `open-questions`, and each scenario's, by parsing the `## Open Questions` section (`runtime/src/primitives/read_spec.rs`, `parse_open_questions`). The parser started an entry only at a `-` bullet. A question written as a numbered list item, or with a `*` or `+` marker, was not an entry at all, so a section holding three unresolved questions read as zero. `/{project}:clarify` branches on that count, the pre-`done` gate reads the scenario half of it, and `append-question`'s duplicate check shares the parser, so every one of them saw an empty section. Nothing in the spec template or in `/{project}:specify` said a question had to be a `-` bullet, so a numbered list was a natural way to write one.

Measured 2026-10-01 over this repository: of 269 spec and scenario files, 247 carry `## Open Questions`, and none holds a numbered, `*`, `+` or nested list item there, so nothing here read wrong. The defect is reachable by any adopter who numbers their questions.

The same parser also split a question at a nested `-` line, counting a sub-bullet as a question of its own.

## Behavior

**An open question is a list item of the `## Open Questions` section, whatever its marker.** An entry starts at a line whose list marker — `-`, `*` or `+`, or a number of one to nine digits followed by `.` or `)` — is followed by whitespace and indented no deeper than the section's first entry. That makes a numbered list, a `*` list and a `+` list read exactly as a `-` list does. A thematic break — a line of three or more of the same `-`, `*` or `_`, optionally spaced, and nothing else — opens nothing. A marker with nothing after it opens an entry whose text comes from its continuation lines, and an entry left empty is dropped.

**A deeper list item belongs to the question above it.** A marker indented further than the section's first entry is a sub-item, and its text folds into the entry it sits under, as any other indented continuation line does. A question with nested options is one question.

**Everything else is unchanged.** A blank line still ends an entry; the placeholder lines are still skipped; a list item inside a fenced block or an HTML comment is still not an entry; and the parser is still the one `read-spec`, scenario collection, `check-review-gate` and `append-question` share. `dashboard`, which kept its own count of `-` lines, blind to HTML comments and fenced blocks, now counts the spec body's and the targeted scenario's questions with it too, so all of them agree on what the section holds. `append-question` strips a leading list marker of any kind the parser recognizes before its duplicate check, and before it writes the question as a `-` item.

**The documentation says what an entry is.** The spec template's Open Questions guidance and `/{project}:specify` say each question is one list item, and `/{project}:clarify`'s markdown-only count names every list marker rather than `-` alone.

## Edge Cases

- **A question written as a plain paragraph.** Not an entry. Prose under the section is how "none" is written in practice — 35 lines across this repository's specs say so in many spellings beside the template's placeholder, and one more is the template's guidance sentence kept as a one-line comment — so counting a paragraph as a question would block every one of those specs. The template and `/{project}:specify` say a question is a list item instead.
- **A section mixing markers** — numbered entries, then a `-` appended by `append-question`. Each counts as an entry, since each sits at the first entry's indentation.
- **A list indented as a whole.** The first entry's indentation is the list's level, so a section whose entries all sit two spaces in reads as it does unindented.
- **A nested item after a blank line.** The blank line ended the entry above it, and a deeper marker starts nothing, so it adds no entry.
- **Emphasis at the start of a line** — `**Bold**` or the `*None — all resolved.*` placeholder. A `*` not followed by whitespace is not a marker.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
