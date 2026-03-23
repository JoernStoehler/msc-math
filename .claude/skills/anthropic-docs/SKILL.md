---
name: anthropic-docs
description: Index of Anthropic's official Claude Code documentation. Load when creating or editing skills, agents, hooks, settings, CLAUDE.md, output styles, or plugins — or when unsure which Claude Code feature to use.
user-invocable: false
---

# Anthropic Docs Index

Official documentation lives at `https://code.claude.com/docs/en/<page>.md`.
To read a page: `curl -sL <url> -o /tmp/cc-<name>.md` then use the Read tool.
Do NOT use WebFetch (it inflates content through an intermediate model).

## Which doc to read for what

| Task | Page | Key content |
|------|------|-------------|
| Choosing which feature to use | [features-overview](https://code.claude.com/docs/en/features-overview.md) | Decision table: CLAUDE.md vs skills vs subagents vs hooks vs MCP vs plugins. Context cost comparison. How features layer and combine. |
| Writing or editing CLAUDE.md | [memory](https://code.claude.com/docs/en/memory.md) | File locations (managed/project/user), `@path` imports, `.claude/rules/` with path-scoped frontmatter, auto memory config, size target (<200 lines). |
| Writing or editing skills | [skills](https://code.claude.com/docs/en/skills.md) | SKILL.md format, all frontmatter fields (`name`, `description`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `model`, `effort`, `context`, `agent`, `hooks`), `$ARGUMENTS` substitution, `!`backtick`` dynamic injection, `context: fork` for subagent execution, supporting files, skill discovery and precedence. |
| Writing or editing subagents | [sub-agents](https://code.claude.com/docs/en/sub-agents.md) | Agent .md format, all frontmatter fields (`name`, `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `maxTurns`, `skills`, `mcpServers`, `hooks`, `memory`, `background`, `effort`, `isolation`), built-in agents (Explore/Plan/general-purpose), scope and precedence, `--agent` flag, persistent memory, hooks in agents. |
| Writing or editing hooks | [hooks-guide](https://code.claude.com/docs/en/hooks-guide.md) | Common patterns (notification, auto-format, file protection, post-compaction re-inject, audit, auto-approve). Hook types: command, http, prompt, agent. |
| Hook reference (schemas, exit codes) | [hooks](https://code.claude.com/docs/en/hooks.md) | All event types and their JSON input schemas, exit code semantics, matcher syntax, async hooks, MCP tool hooks, environment variables (`$CLAUDE_PROJECT_DIR` etc). |
| Configuring settings.json | [settings](https://code.claude.com/docs/en/settings.md) | All settings files (user/project/local/managed), precedence order, permission settings, hook config location, env vars. |
| Configuring permissions | [permissions](https://code.claude.com/docs/en/permissions.md) | Permission modes (`default`/`acceptEdits`/`plan`/`dontAsk`/`bypassPermissions`), rule syntax (`Tool(specifier)`), wildcards, Read/Edit/Bash/WebFetch/MCP/Agent rules, managed settings. |
| Writing output styles | [output-styles](https://code.claude.com/docs/en/output-styles.md) | Frontmatter (`name`, `description`, `keep-coding-instructions`), file locations (`~/.claude/output-styles/` or `.claude/output-styles/`), built-in styles (Default/Explanatory/Learning). |
| Model configuration | [model-config](https://code.claude.com/docs/en/model-config.md) | Aliases (`sonnet`/`opus`/`haiku`/`opusplan`), `[1m]` suffix, effort levels, env var overrides (`ANTHROPIC_DEFAULT_*_MODEL`). |
| Creating or distributing plugins | [plugins](https://code.claude.com/docs/en/plugins.md) | Plugin structure (`.claude-plugin/plugin.json` + `skills/` + `agents/` + `hooks/` + `.mcp.json`), namespacing, `--plugin-dir` testing, migration from standalone. |
| Plugin technical reference | [plugins-reference](https://code.claude.com/docs/en/plugins-reference.md) | Full manifest schema, directory structure spec, version management, debugging tools. |
| Best practices | [best-practices](https://code.claude.com/docs/en/best-practices.md) | Verification, explore-plan-code workflow, context management, CLAUDE.md writing tips, subagent patterns, common failure patterns. |
| Agent teams | [agent-teams](https://code.claude.com/docs/en/agent-teams.md) | Multi-session coordination with shared tasks and messaging (vs subagents which are single-session). |
| Headless / Agent SDK | [headless](https://code.claude.com/docs/en/headless.md) | Running Claude Code programmatically from CLI, Python, or TypeScript. |
| Environment variables | [env-vars](https://code.claude.com/docs/en/env-vars.md) | Complete env var reference. |

## Full index

All 68 pages are listed at: `https://code.claude.com/docs/llms.txt`

Read that file to discover pages not in the table above (e.g. Bedrock, Vertex, GitHub Actions, Slack, Chrome, sandboxing, troubleshooting).
