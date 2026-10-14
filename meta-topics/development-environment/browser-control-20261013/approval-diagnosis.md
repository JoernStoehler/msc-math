# MCP browser approval path (read-only diagnosis)

Codex binary: `codex-cli 0.162.1`. Cached release source at `/home/joern/.cache/codex-source`, Cargo version `0.162.1`, git `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`. Generated schema is in this task directory's `app-server-schema/`.

## Result

The exact approval is an MCP-tool user approval, not command execution approval. In release source `codex-rs/core/src/mcp_tool_call.rs`, `request_mcp_tool_user_approval` returns the observed error when `AskForApproval::Never`; under another policy it checks feature `ToolCallMcpElicitation`. This feature is stable and default-enabled in `codex-rs/features/src/lib.rs` (around line 1871), so the MCP approval is normally sent to the client as `mcpServer/elicitation/request` (`app-server-protocol/src/protocol/common.rs` around line 1803). If that feature is disabled, it instead uses `item/tool/requestUserInput`. Neither is `execCommandApproval`, which is for shell/command execution and has a distinct response schema.

The interactive TUI has handlers for both server request types (`tui/src/chatwidget/protocol_requests.rs`); it can present and collect the approval. MCP approval options in `core/src/mcp_tool_call.rs` include one-time `Allow` (“Run the tool and continue.”), session `Allow for this session`, optional persistent `Allow and don't ask me again`, and `Cancel`. The one-time choice is the narrow choice; no persisted approval variant should be selected.

The existing Python adapter at `create_browser_owner.py` imports the project's `codex-session.py` `Client`. Its `Client.call` treats every incoming server request generically, logs it as unhandled, and replies with JSON-RPC `-32601 Use an interactive Codex client for this request`. It cannot route an approval to Jörn and should not synthesize an approval response.

The existing thread can be resumed with `approvalPolicy: "on-request"`: release source `ThreadResumeParams` has `approval_policy: Option<AskForApproval>`; `AskForApproval::OnRequest` serializes as `on-request`. `TurnStartParams` also accepts `approval_policy`. Caveat: source comments explicitly say the turn/start override is for “this turn and subsequent turns”; do not describe it as strictly single-turn. If setting it on the existing thread, restore `never` after the one authorized approval/action, or use an interactive route that leaves thread policy `never` and can handle a pending one-time request (not available through the current adapter).

## Safe next action / boundary

Do not send RPCs or retry from this adapter. To obtain a genuine user decision, the supported route is a user-operated interactive Codex client/TUI attached to the existing thread, where the exact MCP request can be displayed and Jörn can choose the one-time `Allow` or `Cancel`. A new turn is needed to trigger the tool approval again because the previous call was blocked before navigation and is not pending. Any user confirmation must name the one browser action (Playwright MCP `browser_navigate` to public `https://openai.com/`, with the resulting title and first heading), and no approval should be presumed. Keep global policy/config untouched. After a choice, verify the thread policy is restored to `never` if it was overridden.

No RPC, browser call, process action, retry, or configuration change was made during this diagnosis.
