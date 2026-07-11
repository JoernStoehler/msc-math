# Session Log Paths And Queries

These paths and event shapes were current on 2026-07-11. Inspect the target
rollout before assuming that a schema detail remains current.

## Locate a rollout

```bash
printf '%s\n' "$CODEX_THREAD_ID"
thread=${CODEX_THREAD_ID:?}
find /home/vscode/.codex/sessions /home/vscode/.codex/archived_sessions \
  -type f -name "rollout-*-${thread}.jsonl" -print
tail -n 20 /home/vscode/.codex/session_index.jsonl | jq -c '.'
```

Rollouts normally live below
`/home/vscode/.codex/sessions/YYYY/MM/DD/` or
`/home/vscode/.codex/archived_sessions/`. Index rows contain `.id`,
`.thread_name`, and `.updated_at`, not rollout paths.

Set `ROLL=/path/to/rollout.jsonl`, then inspect its shape:

```bash
jq -r '[.type, .payload.type // "", .payload.role // "", .payload.phase // ""] | @tsv' "$ROLL" | sort | uniq -c
jq -r 'paths(scalars) | map(tostring) | join(".")' "$ROLL" | sort -u
```

Core fields include top-level `.timestamp`, `.type`, and `.payload`;
`session_meta` fields `.payload.id`, `.payload.forked_from_id`,
`.payload.thread_source`, `.payload.cwd`, `.payload.git.*`, and
`.payload.source.subagent.thread_spawn.*`; and tool-event fields
`.payload.name`, `.payload.call_id`, `.payload.arguments`, and
`.payload.output`.

## Focused extraction

```bash
# User messages
jq -r 'select(.payload.type=="user_message") | [.timestamp, .payload.message] | @tsv' "$ROLL"

# Role messages
jq -r 'select(.payload.type=="message") | [.timestamp, .payload.role, (.payload.phase // ""), ((.payload.content // []) | map(.text? // empty) | join("\n"))] | @tsv' "$ROLL"

# User-visible agent messages
jq -r 'select(.payload.type=="agent_message") | [.timestamp, (.payload.phase // ""), (.payload.message // "")] | @tsv' "$ROLL"

# Tool calls and outputs
jq -c 'select(.payload.type=="function_call") | {timestamp, name:.payload.name, call_id:.payload.call_id, arguments:.payload.arguments}' "$ROLL"
jq -c 'select(.payload.type=="function_call_output") | {timestamp, call_id:.payload.call_id, output:.payload.output}' "$ROLL"

# Compaction
jq -c 'select(.type=="compacted" or .payload.type=="context_compacted") | {timestamp,type,payload_type:.payload.type,payload_keys:(.payload|keys? // [])}' "$ROLL"

# Fork/subagent lineage (`?` is required because source can be a string)
jq -r 'select(.type=="session_meta") | [.payload.id, .payload.forked_from_id, .payload.thread_source, (.payload.source.subagent?.thread_spawn?.parent_thread_id // ""), (.payload.source.subagent?.thread_spawn?.agent_nickname // ""), (.payload.source.subagent?.thread_spawn?.agent_role // "")] | @tsv' "$ROLL"
```

Use `?`, `//`, and type checks because fields vary by event type. Function-call
`.payload.arguments` is usually a JSON-encoded string rather than an object;
sensitive nested values may be encrypted even when the surrounding event is
readable. `encrypted_content` is not readable source text.
