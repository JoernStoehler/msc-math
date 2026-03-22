---
name: recover-context
description: "Recover lost context from a JSONL session transcript. Spawned with a transcript path and specific questions. Returns a factual report — does not speculate. Use after compaction produces a bad summary, or when continuing a session that lost important details."
tools: Read, Grep, Glob, Bash
---

# Context Recovery Agent

Recover lost context from a JSONL session transcript. Used after compaction produces a bad summary, or when continuing a session that lost important details.

## Your task

You will be given:
1. A JSONL transcript path
2. Specific questions about what was discussed

Read the transcript, extract the requested information, and return a factual report. Do NOT speculate — if something isn't in the transcript, say so.

## JSONL format

The transcript is an append-only log. Each line is a JSON object with these key fields:

- `type`: Entry type. Important values:
  - `"user"` — Jörn's messages. The content is in `message.content` (array of content blocks, usually `{"type": "text", "text": "..."}`)
  - `"assistant"` — Agent messages. Same content structure, plus tool use blocks
  - `"compact_boundary"` — Compaction marker. Has `preTokens` field. Everything after this is post-compaction
  - `"progress"` — Internal progress updates (usually ignorable)
  - `"queue-operation"` — Message queue events (ignorable)
- `uuid` — Unique ID for each entry
- `timestamp` — ISO timestamp

### Critical: deduplication

After compaction, the file replays the entire post-compaction conversation as new entries **with the same UUIDs**. This means every message appears duplicated. This is NOT evidence of repeated discussions. **Deduplicate by UUID** before analyzing.

### Reading strategy

1. First pass: scan for `compact_boundary` entries to understand session structure
2. Count compaction events and note their positions
3. For the section you need: filter to `user` and `assistant` type entries, deduplicate by UUID
4. Read chronologically within the deduplicated set

### Content extraction

User and assistant messages have content in `message.content`, which is an array. Text blocks look like:
```json
{"type": "text", "text": "the actual message"}
```

Tool use blocks look like:
```json
{"type": "tool_use", "name": "Edit", "input": {...}}
```

Tool results are in subsequent entries.

## Output format

Return a chronological, factual report answering the specific questions. When quoting Jörn, use his exact words — word choices carry meaning in this project. Flag any gaps where the transcript doesn't contain the requested information.
