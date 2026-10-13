# Claude Code operational reference

**Reviewed:** 2026-10-13 against Anthropic's locally mirrored Markdown docs. Claude Code CLI/Desktop/cloud and ordinary Claude chat/Cowork are distinct surfaces. Documentation should be checked against the installed version and effective session state.

## Instructions and skills

- Claude Code supports `CLAUDE.md` instructions at managed, user, project, and local scopes, with nested files and imports. Documentation explicitly distinguishes guidance from enforced permissions: use permissions/hooks for enforced boundaries. Source: [Memory and instructions](https://code.claude.com/docs/en/memory); local mirror: `corpus/articles/anthropic/claude-code/claude-code-memory.md`.
- Current documentation describes native `AGENTS.md` support from v2.1.277, with default project instruction selection `claude-md-or-agents-md`. A `CLAUDE.md` or `CLAUDE.local.md` in scope causes AGENTS.md to be skipped by default; `/config` can select `claude-md-and-agents-md`. Some sessions require v2.1.281 or later, and the built-in AGENTS.md plugin can be disabled. Verify `claude --version`, `/config`, and effective files with `/memory` or `/context`. Source: same memory doc. These thresholds are documentation claims, not a runtime test here.
- Claude Code Skills have `SKILL.md` frontmatter and a separately loaded body; skill discovery and invocation differ from ordinary Claude chat Skills. Local documentation gives personal `~/.claude/skills/<name>/SKILL.md`, project `.claude/skills/<name>/SKILL.md`, nested, managed, and added-directory deployment paths. Source: [Skills](https://code.claude.com/docs/en/skills); local mirror: `corpus/articles/anthropic/claude-code/claude-code-skills.md`.
- A Skill may link to focused reference files on demand; there is no requirement to put a large corpus inside the `SKILL.md`. The [Agent Skills specification](https://agentskills.io/specification) describes optional `references/`, `scripts/`, and `assets/` directories; local mirror: `corpus/articles/standard/agent-skills/agent-skills-spec.md`.

## Verification and surface boundaries

Use `claude --version`; `/context` or `/memory` to inspect loaded instructions; `/doctor` for configuration diagnostics; `/hooks` and `/mcp` for effective hooks and MCP. Sources: [Debug your configuration](https://code.claude.com/docs/en/debug-your-config), local mirror `corpus/articles/anthropic/claude-code/claude-code-debug.md`.

Hooks, MCP, plugins, subagents, and sessions have distinct behavior and permissions. Start from [Features overview](https://code.claude.com/docs/en/features-overview) and check the relevant feature doc rather than transferring semantics between them. `/goal` documents continuing until a completion condition, impossible condition, or blocking error; this is Claude Code behavior and does not establish equivalent behavior in ordinary Claude chat. Source: [Goal](https://code.claude.com/docs/en/goal).

## Corpus acquisition

Anthropic publishes a machine-readable index at https://code.claude.com/docs/llms.txt and direct Markdown pages. Local mirrors under `corpus/articles/anthropic/claude-code/` include the index, memory, skills, debug, features, goal, hooks, MCP, subagents, sessions, overview, changelog, context, and Agent SDK/headless material. Direct Markdown should be preserved as Markdown; do not convert it through HTML.
