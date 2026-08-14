---
name: codex-gui
description: Coordinate durable work with Jörn through the codex-gui Tasks surface. Use when a Codex root must read a task, publish a question or decision-changing update, report a result, revise a task brief or comment, or close or reopen a task.
---

# Coordinate through Tasks

Treat Tasks as the only productive coordination surface. Publish every question, decision-changing update, and result there so Jörn can work from Tasks without supervising execution transcripts. Use Threads only to inspect execution detail or send steering into an execution root.

Before publishing text, draft it in an ordinary file and decide that it is ready to send. Then pass that snapshot with `--file`; the CLI reads it once and sends its contents to the Tasks API. Do not rely on the final-channel response at the end of a Codex turn being read as task communication; it remains part of the execution transcript.

Run the bundled CLI from the project root. The installed repository is the default project. Put `--project PROJECT` before the verb only for an intentional cross-project operation. `TASK_REF` and `COMMENT_REF` accept a UUID or copied Tasks link.

## Read

```sh
node .agents/skills/codex-gui/scripts/task.mjs list
node .agents/skills/codex-gui/scripts/task.mjs --project PROJECT list
node .agents/skills/codex-gui/scripts/task.mjs show TASK_REF
node .agents/skills/codex-gui/scripts/task.mjs comments TASK_REF
node .agents/skills/codex-gui/scripts/task.mjs comments TASK_REF --after COMMENT_OR_REVISION_REF
node .agents/skills/codex-gui/scripts/task.mjs comments TASK_REF --since 2026-08-14T10:00:00Z
```

`list` emits one task summary per JSONL line, suitable for `jq`, `grep`, and shell pipelines. Representative fields, with other fields omitted:

```json
{"id":"<task-uuid>","project":"<slug>","state":"active","currentRevision":{"id":"<revision-uuid>","number":1,"content":"# Title\n\nBrief"},"revisionCount":1,"attention":"new","latestActivityAt":"<RFC3339>"}
```

`show` emits one task document: the same summary fields plus `revisions`, `comments`, and `contextThreadIds`. `comments` emits one comment per JSONL line:

```json
{"id":"<comment-uuid>","currentRevision":{"id":"<revision-uuid>","number":1,"content":"Result","createdAt":"<RFC3339>","actor":{"kind":"agent","sourceThreadId":"<thread-uuid>"}},"delivery":null}
```

Filter locally rather than adding CLI filters:

```sh
node .agents/skills/codex-gui/scripts/task.mjs list | jq -c 'select(.state == "active")'
```

## Publish

Draft every body in `FILE` first, then run exactly one mutation:

```sh
node .agents/skills/codex-gui/scripts/task.mjs create --file FILE
node .agents/skills/codex-gui/scripts/task.mjs comment TASK_REF --file FILE
node .agents/skills/codex-gui/scripts/task.mjs revise TASK_REF --file FILE --base-revision REVISION_ID
node .agents/skills/codex-gui/scripts/task.mjs revise-comment COMMENT_REF --file FILE --base-revision REVISION_ID
node .agents/skills/codex-gui/scripts/task.mjs close TASK_REF
node .agents/skills/codex-gui/scripts/task.mjs reopen TASK_REF
```

Start task briefs with one level-one Markdown heading. Use `comment` for questions, decisions, decision-changing progress, and completed results. Use `revise` only when the task definition changes. Use `revise-comment` only to replace a comment authored by this execution root; take `REVISION_ID` from the current revision in the latest read.

Every successful mutation emits the resulting task document as one JSON object. Mutations obtain exact execution provenance from `CODEX_THREAD_ID` and retain revision history. Publish task communication before ending the Codex turn; keep the ordinary final response as an execution-record summary, not a substitute for the task comment.
