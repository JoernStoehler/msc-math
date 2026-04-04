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
  bash .claude/skills/create-workflow/scripts/download-system-prompt.sh <folder>
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

## 5. Self-review against quality criteria

Before presenting to Jörn, check the draft against these criteria:

- **Actionable, concrete.** Every instruction tells the agent what to do, not what to be. "Run `cargo clippy -- -D warnings` before committing" not "follow best practices."
- **Observable, measurable, verifiable.** Conditions in "if X then Y" instructions are observable by the agent. "If the file has more than 3 functions" is observable; "if the code is complex" is not. Expected outcomes are checkable during planning (does the plan satisfy the criteria?), implementation (is the agent doing it?), and review (did it work?).
- **Clear, unambiguous, low-overhead.** Each sentence has one reading. Agent doesn't need to spend attention resolving ambiguity or recalling novel terminology.
- **Correct, precise.** Claims about agent behavior, tool capabilities, or file formats are verified against the source (system prompt, llms.txt, observed behavior). Wrong instructions cause silent failures.
- **Overall adherence is testable.** There exists a realistic scenario where you could spawn a subagent and check whether it follows the instructions. If you can't imagine such a test, the instructions may be too vague to influence behavior.
- **Feedback is collected.** The instruction set includes or references a mechanism for future agents to report whether it worked (post-mortem, feedback/ files, subagent observations).
- **Vague-word scan.** Grep for "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", "robust" — replace each with what specifically makes it so.
- **Redundancy check.** Does each instruction add information beyond what agents already do from training? "Follow best practices" adds nothing. Remove instructions that don't change behavior.
- **Script-or-language decision.** For anything where getting it wrong has high cost, check whether a script/hook could enforce it instead of relying on the agent to remember.

## 6. Jörn reviews

Present the draft with a prioritized list of spots Jörn should check (uncertain areas, high-impact phrasing). Get explicit approval — don't guess at it. Accept pivots back to earlier steps.

## 7. Set up verification

Before shipping, decide how to verify the new infrastructure works:
- Think about testability: what observable behavior should change in future sessions?
- Plan how to gather feedback during live sessions — e.g. add to post-mortem radar, tell subagents to write observations to `feedback/<name>.md`
- Identify what a post-mortem should look for to evaluate whether this infrastructure helped or hurt

Do NOT write feedback into SKILL.md files. Raw observations only — analysis happens in dedicated sessions with Jörn.

## Reference sources

**Claude Code specs:** `curl -sL https://code.claude.com/docs/llms.txt`
**System prompt:** `references/system-prompt/`
**Skills guide:** `references/skills-guide.md`
**Expert model background:** `references/agent-expert-model.md`
