---
name: codex-session-log-parsing
description: Use when inspecting, summarizing, auditing, citing, or cross-checking local Codex session logs under ~/.codex, including rollout lineage, messages, tool events, and compaction.
---

# Codex Session Log Parsing

Raw rollout JSONL is the source truth for chat history, session ids, tool calls,
subagent results, compaction, and transcript provenance. Derived summaries and
task notes are downstream artifacts. This skill does not apply to experiment
JSONL, ordinary repository search, or Git history.

Read `references/session-log-queries.md` for current paths, event shapes, and
tested `jq` recipes. Inspect the actual event shape before relying on the dated
schema map.

Logs contain private material, including instructions, messages, tool output,
paths, and remotes. Prefer focused structural extraction over transcript dumps
or broad searches across unrelated rollouts.

Function-call arguments remain JSON strings, but sensitive nested values can be
encrypted. Preserve the visible structure and report unreadable values as
encrypted; do not infer their contents.

Resume and fork lineage may be inferred from session metadata and thread ids.
Do not claim an explicit resume marker unless one exists. For durable claims,
record the thread id, rollout path, focused query, and extracted result; keep
quoted transcript text short.
