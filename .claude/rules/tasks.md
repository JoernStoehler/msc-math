# TASKS.md conventions

Triggers: reading or writing `TASKS.md`.

## Format

- `##` sections group by theme (research questions, thesis, code quality, infrastructure)
- `###` items are individual work units
- Every `##` and `###` header has a status tag: `[done]`, `[active]`, `[blocked]`, `[open]`, `[Jörn]`, `[future]`
- `[done]` items include a date: `### [done] [2026-04-04] Item title`
- A `##` group is `[done]` when all children are done, `[open]` otherwise

## Status tags

- `[done]` — completed. Include date. One-line summary in header, minimal body.
- `[active]` — currently being worked on.
- `[blocked]` — waiting on something specific. Body says what.
- `[open]` — ready to start, no one has picked it up.
- `[Jörn]` — needs Jörn's input, verification, or decision.
- `[future]` — idea or direction, not in scope for current deadline.

## Writing style

- Headers carry the key info. Body only when the header isn't enough.
- No tables, no prose paragraphs. Bullets for details.
- Link to logbooks for findings — don't duplicate findings here.
- Working notes style, not a polished document.

## TOC script

Run `bash scripts/tasks-toc.sh` to get a section index with line ranges.
Use the line ranges to read specific sections: `Read(file, offset=start, limit=end-start+1)`.

## When editing

- When an item's status changes, update the tag. Add date for `[done]`.
- When an item becomes `[done]`, keep it in its thematic group (don't move to historical unless it's a cross-cutting task with no thematic home).
- Don't cache derivable state (test counts, build status). Run the command instead.
- Record decisions and reasons — these can't be derived later.
