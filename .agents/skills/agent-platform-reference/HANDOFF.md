# Work-mode handoff — complete the agent-platform reference

The attached ZIP is a **research prototype**, not a completed offline corpus. It contains one Agent Skill (`SKILL.md`), 11 curated references, a 43-source manifest, refresh/discovery scripts, a Codex acquisition benchmark, and 9 passing unit/structural tests. All 43 source mirrors are `pending`: the previous execution container could not access external hosts. Treat this as a starting point, not authoritative documentation.

## Objective

Produce a **verified, locally greppable, refreshable reference package** that subsequent agents can use without repeatedly searching the web for established behavior of OpenAI and Anthropic agent products. Cover ordinary ChatGPT chats (including settings, Custom Instructions, Projects, memory, Work and tool/connector behavior), Codex CLI/app/cloud, OpenAI API/agent SDKs, Claude Chat/Cowork, Claude Code, Claude Agent SDK, and Agent Skills. Preserve platform/version/surface distinctions. Identify vendor-doc errors, misleading statements, unsupported assumptions, and missing context; link corrections to source evidence and, where possible, executable observations.

## First action: check whether execution is viable

Unpack the ZIP in your workspace. Immediately test (a) a writable persistent workspace, (b) HTTPS fetching of official Markdown/HTML sources into actual files, (c) ability to run shell/Python, and (d) access to GitHub source archives or `git` transport. Record actual commands, status, byte counts and elapsed times. If bulk fetching is blocked, try reasonable alternatives, but **escalate the blocking constraint promptly, before substantial implementation**. Don't substitute a design-only deliverable without explicit authorization.

## Work

1. Inspect the existing files and run `python scripts/validate.py && python -m pytest -q tests`. Do not assume the previous prototype is correct.
2. Inventory the primary sources *before* finalizing storage structure. Include counts, fetch formats, total downloaded bytes, conversion fidelity, duplicate sources, versioning and update methods. Discover beyond the seed manifest via official indexes, sitemaps, public repositories and release notes.
3. For `openai/codex`, measure at least a tag archive and a release-tag shallow clone; optionally a filtered sparse checkout. Record actual walltime, transfer bytes when measurable, checked-out bytes, Git object size, source tag/commit, and limits of comparability. Don't invent GB or minute figures. Re-check the latest applicable release tag.
4. Compare at least: (A) all material under `references/`, (B) a separate mirrored corpus plus curated references, and (C) immutable mirrors plus generated correction-enriched reading views. Compare against actual measured corpus size, agent retrieval fidelity, correction visibility, refresh safety, and compatibility with ChatGPT/Claude/Codex skill packaging. Select on evidence.
5. Ingest and normalize official documentation as human-readable, `rg`-searchable Markdown without destroying important code blocks, tables, links or version notes. Track URL, resolved URL, retrieval time, content hash, HTTP cache metadata, version/scope, and conversion errors. Do not silently replace valid articles with login/error pages. Review the final inventory and gaps.
6. Ground curated references and correction claims in retrieved primary sources. Separate docs statements, source-code observations, and runtime-tested behavior. Never automatically erase custom corrections after a vendor refresh. Add targeted verification tests for critical precedence/tool/context claims wherever practical.
7. Pay special attention to **how ChatGPT chats work** and propose a concise, paste-ready Custom Instructions text optimized for end-to-end task success, evidence gathering, autonomous recovery, early escalation of truly blocking constraints, low user-interaction overhead, and appropriate effort allocation. Confirm where and whether it can be edited via current settings/tools; don't claim it has been applied unless verified. Clearly separate global Custom Instructions, Project instructions, Skills, Work, Codex AGENTS.md and Claude CLAUDE.md.
8. Run full validation, include representative retrieval tests and changed-source/correction tests. Produce a usable package plus a terse report with source coverage, downloaded size, measured timings, verified behavior, unresolved risks, and exact deployment instructions for each supported agent environment. Keep the source snapshot separate from hand-curated edits.

## Completion gate

Do **not** mark complete until the source corpus is actually present, coverage and retrieval work, corrections are discoverable, refresh was exercised against real upstream data, tests pass, and acquisition metrics are reported. If any gate remains blocked, state that at the top, with the exact blocker and smallest action required of the user. Do not repeatedly hand in architectural sketches or partial progress as final deliverables. Prefer doing the work to describing how it could be done. When a task is small, avoid unnecessary ceremony.
