---
name: agent-writing
description: Use when writing content other agents will consume (issue bodies, specs, CLAUDE.md entries), editing CLAUDE.md, or testing comprehension of produced content with subagents.
---

# Writing for other agents

Content that agents will consume (issue bodies, specs, CLAUDE.md entries) benefits from:
- **Grounded over speculative** — state what happened or exists, not what might be useful
- **Knowledge over instructions** — inform, don't command. Agents have their own task instructions
- **Skimmable over comprehensive** — clear headers for different reader tasks

## Check clarity with subagents

When you produce content other agents will consume (CLAUDE.md files, mathematical definitions, proofs, algorithm descriptions): test comprehension by having a fresh Sonnet subagent attempt to USE the content (e.g. "implement from this description" or "answer these specific questions about the algorithm") as a thought experiment (e.g. "answer with your plan but make no edits"). Check whether their output matches your intent. Do not ask "is this clear?" — an agent that misunderstands may confidently say yes.

## Editing CLAUDE.md

Agents edit CLAUDE.md directly on their branch. Jörn reviews via git diff in VS Code and merges.

The file follows structural rules to prevent information-destroying edits. See `CLAUDE.adr.md` for the full style guide and the reasoning behind each rule. Key principles:

- **One claim per bullet.** Dense prose packs multiple claims that get lost when a sentence is rewritten.
- **Qualifier preservation.** Every adjective narrows meaning. "Clear, detailed, explicit, structured" is not a synonym list — each word names a different quality bar. When rewriting, check: does this rewrite preserve all constraints the original imposed?
- **Clarity & unambiguousness > correctness > maintainability >>> tokens.** Redundancy is welcome. Using 50 extra words to prevent a misunderstanding is always worth it.

Content conventions:
- Invariants and behaviors are documented only after empirically confirmed as useful
- Label invariants as `[aspirational]` if not yet satisfied
- Prefer simple, common, expected rules that don't claim excessive agent attention beyond their assigned work
