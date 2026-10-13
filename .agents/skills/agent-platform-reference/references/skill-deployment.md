# Agent Skills: one format, distinct deployment surfaces

**Reviewed:** 2026-10-13 against the local Agent Skills specification and product docs. A shared authoring format does not make skill discovery, permissions, or runtime access identical.

## Format and file organization

A Skill is a directory with a `SKILL.md` containing YAML `name` and `description` frontmatter followed by Markdown instructions. The spec describes optional `scripts/`, `references/`, and `assets/` directories and recommends progressive disclosure: keep the entry instructions focused and link to supporting references when needed. It does not define a bulk documentation corpus format or manifest schema. Source: [Agent Skills specification](https://agentskills.io/specification); local mirror: `corpus/articles/standard/agent-skills/agent-skills-spec.md`.

## Deployment by surface

- **Ordinary ChatGPT:** Global Custom Instructions are edited in Settings and are not a Skill or file retrieval mechanism. ChatGPT Skills are available on eligible surfaces/accounts; their availability and upload flow depend on the account/workspace. Select a Skill in ChatGPT using `@` where supported, or use the enabled Skills interface. Project files can provide project context but do not guarantee shell or `rg` access. Sources: [Custom Instructions](https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions), [Skills in ChatGPT](https://help.openai.com/en/articles/20001066-skills), [Projects](https://help.openai.com/en/articles/10169521-projects-in-chatgpt). Do not tell a user to install a Codex/Claude Code local directory for ordinary ChatGPT.
- **ChatGPT Work and Codex distribution:** OpenAI describes standalone Skills in ChatGPT desktop, Codex CLI, and IDE extension; Skills included in plugins can be surfaced across supported Chat and Work surfaces. Skill presence does not itself authorize tools or connections. Source: [Codex Skills](https://developers.openai.com/codex/skills); local mirror: `corpus/articles/openai/codex/codex-skills.md`.
- **Codex CLI/IDE local:** Put a repo-specific Skill in `.agents/skills/<skill-name>/SKILL.md`, or a personal Skill in `$HOME/.agents/skills/<skill-name>/SKILL.md`. Codex also documents admin/system locations. Invoke with `$skill-name` or discover using `/skills`; Codex detects local changes and can be restarted if necessary. Source: same Codex Skills doc.
- **Claude Code:** For a personal local Skill use `~/.claude/skills/<skill-name>/SKILL.md`; for a repository-shared Skill use `.claude/skills/<skill-name>/SKILL.md` and commit it; nested and managed paths are also documented. These local skills do not by themselves load into Cowork/cloud sessions. For Claude.ai-synced skills, follow that product's Skills UI/account setup. Source: [Claude Code Skills](https://code.claude.com/docs/en/skills); local mirror: `corpus/articles/anthropic/claude-code/claude-code-skills.md`.
- **OpenAI Responses API:** Upload a Skill as a directory/multipart set or ZIP and attach its version via `tools[].environment.skills` when configuring the hosted shell tool. The mirrored docs specify 50 MB maximum ZIP size, 500 files per version, and 25 MB maximum uncompressed file size. These limits apply to the documented API upload surface, not all local Skills. Agents API sessions instead discover Skills from directories in their sandbox. Source: [Skills API guide](https://developers.openai.com/api/docs/guides/tools-skills); local mirror: `corpus/articles/openai/api/openai-skills-api.md`.

For API-hosted agent and SDK distinctions, read [api-agents.md](api-agents.md); it separates application-owned loops, provider-managed harnesses, tool execution, and environment ownership.

## Packaging this reference

For local coding agents, retain curated reference pages and a `SKILL.md` together; keep the 638-source mirrored corpus separately addressable and query it only where that environment grants file access. For hosted API upload, package only the necessary curated pages and verify the documented limits. For ordinary ChatGPT, add only the intended material through an eligible Skill or Project source; do not assume that uploading a Skill enables shell access or mounts the rest of the corpus.

Do not conflate ChatGPT global Custom Instructions, ChatGPT Project instructions, ChatGPT Skills/plugins, Codex `AGENTS.md`, Claude Code `CLAUDE.md`/Skills, or API system/developer instructions. Check the target product docs and live account/session before describing installation as successful.

## Exact local install commands

From the directory containing the extracted `agent-platform-reference/` folder:

```bash
# Codex personal skill (documented user scope)
mkdir -p "$HOME/.agents/skills"
cp -R agent-platform-reference "$HOME/.agents/skills/agent-platform-reference"
# Or repository scope: .agents/skills/agent-platform-reference

# Claude Code personal skill
mkdir -p "$HOME/.claude/skills"
cp -R agent-platform-reference "$HOME/.claude/skills/agent-platform-reference"
# Or repository scope: .claude/skills/agent-platform-reference
```

Invoke `$agent-platform-reference` in Codex or `/agent-platform-reference` in Claude Code. Check the client's discovered skill list and filesystem access before assuming installation worked. Run maintenance commands from the installed skill root with Python dependencies from `requirements.txt`.

The full package exceeds the documented 500-file OpenAI Skills API limit. `dist/agent-platform-reference-thin.zip` contains the entry point and curated references only. For eligible ChatGPT Skills, open sidebar Plugins → Plugin Directory → Skills (or the desktop Skills sidebar), select Create → Upload from your computer, upload the thin ZIP and finish the scan/review. Work workspace policy can restrict uploads or tool access. For Claude chat/Cowork, upload the thin skill through the supported Skills interface (Customize → Skills where available); availability is documented rather than account-verified here. For ordinary Project knowledge, use selected curated Markdown files with Project instructions pointing to them; knowledge retrieval is not a filesystem mount.

For Responses API, upload the thin ZIP using the documented Skills API and attach the returned skill ID/version to the hosted shell tool's `environment.skills`. For Agents API, unpack the full skill into a sandbox skill directory documented for the selected environment and grant read access. For OpenAI Agents SDK/Claude Agent SDK or other application-owned harnesses, mount the full directory and expose a file-search tool, or install under the client skill-discovery path and enable that discovery in SDK configuration; a mounted directory alone does not enable tool access. None of these routes has been applied to a live account by this deliverable.
