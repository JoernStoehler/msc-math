# Reading view: chatgpt-work-docs-mcp-server

> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.

- Source ID: `chatgpt-work-docs-mcp-server`
- Source URL: https://learn.chatgpt.com/docs/mcp-server.md
- Resolved URL: https://learn.chatgpt.com/docs/mcp-server.md
- Correction links: 0

## Curated corrections relevant to this source

No direct source-URL match in the current correction register.

## Source mirror

<!-- source-id:chatgpt-work-docs-mcp-server -->

# Codex MCP server removal

> For the complete documentation index, see [llms.txt](https://learn.chatgpt.com/llms.txt). Markdown versions of documentation pages are available by appending `.md` to the page URL.

The `codex mcp-server` command and the standalone `codex-mcp-server` binary have
been removed. Integrations that launch either command must migrate before
upgrading Codex. The previous MCP tool reference and Agents SDK examples on this
page are no longer supported.

## Use the Codex app server

Use the [Codex app server](https://learn.chatgpt.com/docs/app-server) for integrations that need
authentication, conversation history, approvals, and streamed agent events.

The app server uses its own [JSON-RPC protocol](https://learn.chatgpt.com/docs/app-server#protocol). It
isn't an MCP server or a drop-in replacement for an MCP client: update your
integration to use the app-server protocol instead of MCP tool calls.
The app-server command is experimental and isn't supported for production
workloads.

## Connect Codex to MCP tools

Codex continues to support [external MCP servers](https://learn.chatgpt.com/docs/extend/mcp).
Use `codex mcp` to manage those connections. The removal affects hosting Codex
as an MCP server.
