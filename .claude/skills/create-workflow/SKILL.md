---
name: create-workflow
description: Collaborative workflow for creating new agent infrastructure (skills, hooks, rules, CLAUDE.md sections) with Jörn. Use when Jörn asks to build something new for how agents work, not when updating existing infrastructure.
---

# Create New Agent Infrastructure

Collaborative workflow. Jörn has the expert model for what works with agents — the agent supplies research and drafting labor. The agent does NOT decide what agents should do — that requires expertise agents don't have.

## 1. Gather real situations

Look at actual data, not hypotheticals:
- Session logs: `~/.claude/projects/-workspaces-msc-math/`
- Git history: `git log --oneline -- .claude/`
- Current infrastructure: `.claude/skills/`, `.claude/agents/`, `.claude/rules/`
- Feedback files: `feedback/`

Present prioritized concrete situations to Jörn. He confirms which matter.

## 2. Research and present information

For each situation Jörn wants to address, gather and present:

- **Existing patterns:** What common practices exist for this kind of situation? (Agents have broad training-data recall here — use it.) Rank, triage, explain each to Jörn.
- **Causal chain:** What leads to the situation? Look at real cases. Brainstorm interventions.
- **System prompt:** What do agents already see about this? Report relevant parts — Jörn doesn't have the system prompt memorized. Download via:
  ```bash
  bash .claude/skills/agent-design/scripts/download-system-prompt.sh <folder>
  ```
- **Detection:** How can the situation be detected? Skill descriptions (RLVR-trained triggering), hooks (scriptable tool-call triggers), subagent reviews.
- **Costs:** One-time setup, ongoing maintenance/staleness, attention budget consumed, runtime costs.

Goal: accelerate Jörn's decision-making, surface ideas he'd overlook. Not replace his judgment.

## 3. Jörn decides

Jörn picks the approach. The agent:
- Asks clarifying questions until the approach is unambiguous enough to implement:
  - What file type(s)? (skill, hook, rule, CLAUDE.md section, repo artifact)
  - What triggers activation?
  - What is the expected agent behavior?
  - Known edge cases or exceptions?
- Flags phrasing that agents might misinterpret.
- Does NOT silently fill gaps — ask rather than guess.

## 4. Draft

Write the files Jörn specified. Before writing:
- Fetch relevant spec: `curl -sL https://code.claude.com/docs/llms.txt` then the specific page
- For skills: follow `references/skills-guide.md`
- Writing style: follow CLAUDE.md "Text that agents read" section — correct, corrigible, verifiable, unambiguous, complete, actionable, simple. Run the vague-word scan.

## 5. Jörn reviews

Present the draft with a prioritized list of spots Jörn should check (uncertain areas, high-impact phrasing). Get explicit approval — don't guess at it. Accept pivots back to earlier steps.

## 6. Set up verification

Before shipping, decide how to verify the new infrastructure works:
- Define ≥1 test task for `/test-workflow` (concrete scenario, expected behavior, pass/fail criteria)
- Add to post-mortem radar if relevant
- For subagent workflows: tell subagents to write observations to `feedback/<name>.md`

Do NOT write feedback into SKILL.md files. Raw observations only — analysis happens in dedicated sessions with Jörn.

## Reference sources

**Claude Code specs:** `curl -sL https://code.claude.com/docs/llms.txt`
**System prompt:** `references/system-prompt/`
**Skills guide:** `references/skills-guide.md`
**Expert model background:** `references/agent-expert-model.md`
