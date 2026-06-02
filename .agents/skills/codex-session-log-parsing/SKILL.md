---
name: codex-session-log-parsing
description: Use when Codex needs to inspect, summarize, audit, cite, or cross-check local Codex session logs under ~/.codex, especially session_index.jsonl, rollout JSONL files, message payloads, tool calls, compaction markers, fork/resume lineage, subagent metadata, or jq extraction commands.
---

# Codex Session Log Parsing

Use raw Codex session logs when a durable artifact depends on chat history,
session ids, user answers, tool calls, subagent results, compaction summaries,
or transcript provenance. Treat the raw rollout JSONL as source truth; derived
summaries, task notes, and `FACTSHEET.md` are downstream artifacts.

Do not use this skill for experiment JSONL, ordinary repo searches, or git
history.

## Locate The Current Log

Current thread id is often available:

```bash
printf '%s\n' "$CODEX_THREAD_ID"
```

Find the rollout JSONL from a thread id:

```bash
thread=${CODEX_THREAD_ID:?}
find /home/vscode/.codex/sessions /home/vscode/.codex/archived_sessions \
  -type f -name "rollout-*-${thread}.jsonl" -print
```

Use the session index to identify recent threads. It has ids, names, and update
times, not rollout paths:

```bash
tail -n 20 /home/vscode/.codex/session_index.jsonl | jq -c '.'
```

Rollout paths usually look like:

```text
/home/vscode/.codex/sessions/YYYY/MM/DD/rollout-<timestamp>-<thread-id>.jsonl
/home/vscode/.codex/archived_sessions/rollout-<timestamp>-<thread-id>.jsonl
```

When a durable file depends on a session, record both the thread id and rollout
path.

## Inspect Shape First

Set the log path once:

```bash
ROLL=/path/to/rollout.jsonl
```

Count event shapes:

```bash
jq -r '[.type, .payload.type // "", .payload.role // "", .payload.phase // ""] | @tsv' "$ROLL" |
  sort | uniq -c
```

Show scalar jq paths:

```bash
jq -r 'paths(scalars) | map(tostring) | join(".")' "$ROLL" | sort -u
```

Core paths:

- Top level: `.timestamp`, `.type`, `.payload`.
- Session index rows: `.id`, `.thread_name`, `.updated_at`.
- Session metadata: `.type=="session_meta"`, `.payload.id`,
  `.payload.forked_from_id`, `.payload.thread_source`, `.payload.cwd`,
  `.payload.git.*`, `.payload.source.subagent.thread_spawn.*`.
- Turn metadata: `.type=="turn_context"`, `.payload.turn_id`,
  `.payload.cwd`, `.payload.current_date`, `.payload.approval_policy`,
  `.payload.sandbox_policy`.
- Tool calls: `.payload.type=="function_call"`, `.payload.name`,
  `.payload.call_id`, `.payload.arguments`.
- Tool outputs: `.payload.type=="function_call_output"`,
  `.payload.call_id`, `.payload.output`.

Other event types observed include `custom_tool_call`,
`custom_tool_call_output`, `tool_search_call`, `tool_search_output`,
`web_search_call`, `web_search_end`, `patch_apply_end`, `task_started`,
`task_complete`, `token_count`, `thread_goal_updated`, and `turn_aborted`.

## Extract Common Content

User messages:

```bash
jq -r 'select(.payload.type=="user_message") |
  [.timestamp, .payload.message] | @tsv' "$ROLL"
```

Assistant/developer/user response messages:

```bash
jq -r 'select(.payload.type=="message") |
  [.timestamp, .payload.role, (.payload.phase // ""),
   ((.payload.content // []) | map(.text? // empty) | join("\n"))] | @tsv' "$ROLL"
```

User-visible agent messages:

```bash
jq -r 'select(.payload.type=="agent_message") |
  [.timestamp, (.payload.phase // ""), (.payload.message // "")] | @tsv' "$ROLL"
```

Tool calls and outputs:

```bash
jq -c 'select(.payload.type=="function_call") |
  {timestamp, name:.payload.name, call_id:.payload.call_id, arguments:.payload.arguments}' "$ROLL"

jq -c 'select(.payload.type=="function_call_output") |
  {timestamp, call_id:.payload.call_id, output:.payload.output}' "$ROLL"
```

Compaction markers:

```bash
jq -c 'select(.type=="compacted" or .payload.type=="context_compacted") |
  {timestamp,type,payload_type:.payload.type,payload_keys:(.payload|keys? // [])}' "$ROLL"
```

Subagent/fork lineage. Use `?` because `.payload.source` may be a string:

```bash
jq -r 'select(.type=="session_meta") |
  [.payload.id, .payload.forked_from_id, .payload.thread_source,
   (.payload.source.subagent?.thread_spawn?.parent_thread_id // ""),
   (.payload.source.subagent?.thread_spawn?.agent_nickname // ""),
   (.payload.source.subagent?.thread_spawn?.agent_role // "")] | @tsv' "$ROLL"
```

## Pitfalls

- Logs contain private and sensitive material: system/developer/user messages,
  tool outputs, command output, repo paths, git remotes, and base instructions.
  Prefer structural summaries over dumping large transcript excerpts.
- Do not broad-`rg` all rollout logs unless necessary. It is noisy and can
  expose unrelated sessions.
- Use `?`, `//`, or type checks in jq. Some fields vary by event type.
- `.payload.arguments` for function calls is often a JSON-encoded string, not
  an object.
- `encrypted_content` is not useful as readable source text.
- Resume/fork state may be inferred from `session_meta`, `forked_from_id`,
  repeated session metadata, and thread ids. Do not claim an explicit resume
  marker unless the log contains one.
- Keep copied quotes short. Prefer recording the session id, transcript path,
  jq command, and a focused extracted result.
