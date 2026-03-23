---
name: anthropic-docs
description: Index of Anthropic's Claude Code documentation. Load before editing skills, agents, hooks, settings, or CLAUDE.md.
disable-model-invocation: true
---

# Anthropic Claude Code Documentation Index

Full index: `https://code.claude.com/docs/llms.txt` (68 pages, markdown).

To fetch a page: `curl -sL https://code.claude.com/docs/en/<page>.md -o /tmp/cc-<page>.md`
Then read with the Read tool. Do NOT use WebFetch — it inflates content through an intermediate model.

## Pages to consult by task

### Editing CLAUDE.md
- `memory.md` — how CLAUDE.md files load, effective instructions, size targets (<200 lines), imports, rules

### Editing skills (.claude/skills/)
- `skills.md` — skill format, frontmatter fields, supporting files, invocation control, context:fork, dynamic injection
- `features-overview.md` — when to use skills vs CLAUDE.md vs rules vs subagents vs hooks

### Editing agents (.claude/agents/)
- `sub-agents.md` — agent format, frontmatter fields (tools, model, permissionMode, hooks, memory, skills, isolation), built-in agents, patterns

### Editing hooks (.claude/hooks/)
- `hooks.md` — full reference: all events, JSON schemas, exit codes, async hooks, MCP tool hooks
- `hooks-guide.md` — practical examples: notifications, auto-format, file protection, context injection

### Editing settings (.claude/settings.json)
- `settings.md` — all settings fields, scopes (managed/user/project/local), permission settings

### Model configuration
- `model-config.md` — aliases (sonnet/opus/haiku/opusplan), effort levels, 1M context, env vars

### Output customization
- `output-styles.md` — custom output styles, frontmatter, comparison with CLAUDE.md and agents

### Other relevant pages
- `best-practices.md` — Anthropic's recommended patterns for working with Claude Code
- `permissions.md` — permission modes, tool-specific rules, sandbox
- `common-workflows.md` — plan mode, git worktrees, parallel sessions
- `agent-teams.md` — multi-session coordination with shared tasks and messaging
- `mcp.md` — connecting external services
- `plugins.md` — packaging skills + hooks + agents + MCP for distribution
