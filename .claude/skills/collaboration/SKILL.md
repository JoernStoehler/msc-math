---
name: collaboration
description: "How to coordinate work across multiple agents and sessions. Load when deciding between subagents vs teams vs deferred sessions, writing handoff files, or splitting work across session boundaries."
---

# Collaboration Patterns

## Subagent (Agent tool, same session)

Spawns a child agent. Child runs foreground or background, returns results to parent, shares worktree (unless `isolation: worktree`).

**Use when:** Self-contained task, results needed this session. Typical: review, research, mechanical edits.

**Key rules:**
- N independent files → N parallel background agents, one per file
- Opus for math/correctness, Sonnet for formatting/style, Haiku for search
- Include conventions explicitly in prompts — subagents don't load skills unless told
- State ground truth hierarchy when sources conflict: data > code > .tex > prose
- Don't treat "review found nothing" as "verified correct"

## Agent team (TeamCreate, same session)

Team lead spawns teammates as separate instances. Async parallel work with messaging.

**Use when:** 2-4 agents need to coordinate on related work (coupled components, communicating reviewers).

## Deferred session (handoff file)

Write a handoff file, commit it, Jörn opens a new session pointed at it.

**Use when:** Work is independent of current session or will continue later.

## Parallel sessions (Jörn orchestrates)

Multiple terminals, each on its own worktree/branch. Jörn coordinates manually.

---

# Handoff File Format

Put in `handoffs/<name>.md`, committed to the branch.

```markdown
# Task: <imperative verb phrase>

## Context
<Why this task exists. 2-4 sentences.>

## Scope
<What to do — numbered steps.>

## Out of scope
<What NOT to do. Name specific temptations.>

## Key files
<Absolute paths. Not summaries — pointers.>

## Prior findings
<Facts not in the repo that the next agent would re-derive.>

## Success criteria
<Concrete and verifiable. E.g. "cargo test --lib passes".>

## Dependencies
<Blocked on / blocking other sessions.>
```

**Principles:** Pointers over summaries. Scope boundaries prevent drift. One task per file. Success criteria must be agent-verifiable.
