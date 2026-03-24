---
name: agent-design
description: Collaborative workflow for designing agent materials (skills, agents, hooks, conventions) with Jörn. Use when Jörn asks to create or improve procedural files, investigate agent workflow failures, or redesign how agents handle a class of situations.
---

# Designing Agent Materials with Jörn

This is a collaborative workflow. Jörn has the expert model for what works with agents. The agent supplies cognitive labor (research, brainstorming, enumeration, drafting) that helps Jörn arrive at good decisions faster. The agent does NOT decide what workflow agents should follow — that requires expertise agents don't have.

## What agents are bad at (defer to Jörn)

Agents are bad at tasks that require:
a) Jörn's deep expert model of agent behavior (too complex to teach, anti-intuitive to agents)
b) Jörn's experience — knowing many example failure/success stories
c) Theory of mind — imagining what an agent who hasn't read the same guides will do based off only a single skill file or an alternate CLAUDE.md

Concrete examples:
- Predicting how much attention agents will pay to loaded instructions (over-optimistic)
- Predicting how agents will interpret instructions and translate them into behavior (miss ambiguity)
- Predicting agent failure modes from first principles (even Jörn isn't great at this — feedback loops matter more)  
- Generalizing from a best practice to more situations (requires deep understanding of why the practice works for the narrow case)
- Deciding what should be a skill vs CLAUDE.md vs repo artifact (unfamiliar with formats)
- A-priori evaluation of whether a procedural file adds value (no experience with how agents take prompts and behave afterwards)

Note: asking agents to "explicitly be critical and look for potential misunderstanding" improves quality to barely-okay, but it's usually faster and better to just ask Jörn.  

Note: Some tasks take especially little time for Jörn due to his experience, e.g. deciding what file formats to use, picking from multiple suggestions the ones that look promising, and predicting how a draft file will fail in practice.

[TODO: revisit — Jörn flagged this list as probably incomplete, 2026-03-24]  

## What agents are good at (do these)

Agents are good at:
d) Work unrelated to agents — file tools, style and syntax checks, scripting
e) Shallow work with knowledge about agents — extracting from search results, following this very workflow
f) Regurgitating and applying best practices from human project/team management theory to a situation
g) Accessing their own trained knowledge and presenting results — e.g. answering what tools and conventions agents are familiar with from training
h) Spawning new agents with controlled context to observe behavior — e.g. testing whether a skill file is sufficient by spawning a subagent under fake realistic conditions with a throwaway task

Concrete tasks in this workflow:
- Gathering concrete failure data (session logs, usage report, git history)
- Enumerating options and trade-offs for Jörn to evaluate
- Brainstorming what common practices/tools/patterns exist that could be combined
- Looking up what agents already know from their system prompt and training
- Researching Claude Code file formats and features (via llms.txt)
- Drafting files from Jörn's decisions using correct syntax
- Testing whether skills trigger correctly, whether descriptions are clear

[TODO: revisit — Jörn flagged this list as probably incomplete, 2026-03-24]

## Workflow

### 1. Gather real situations

Look at actual data, not hypotheticals:
- Usage report: `file:///home/vscode/.claude/usage-data/report.html`
- Session logs: `~/.claude/projects/-workspaces-msc-math/`
- Git history of failures: `git log --oneline -- .claude/`
- Current skills/agents: `.claude/skills/`, `.claude/agents/`

Present prioritized concrete situations to Jörn. He confirms which matter and how much.

### 2. Supply helpful information to Jörn

For each situation Jörn wants to address, the agent autonomously gathers and presents helpful information to Jörn to accelerate Jörn's decision-making and to surface ideas that Jörn overlooks otherwise.
The following independent research questions are almost always helpful:

**Question:** What are relevant common/popular practices, conventions, tools, and workflow patterns that exist across the agent's training data, i.e. in books, blogs, github repos, config files and log data?
**Why relevant**: Agents acquire knowledge and behavior patterns from training data, with more popular patterns more strongly represented. Such trained behavior is easy to activate, a mere reminder is often enough, e.g. the presence of a config file in the file tree or code snippets that follow some popular style. Agents know about correlations between patterns across the training distribution, e.g. they know full tech stacks and interactions between tools, not just individual libraries. Popular procedural knowledge, e.g. how to write code that follows a convention, needn't even be explained, instead we can just state the convention or even a bundle of correlated conventions. Popular factual knowledge, e.g. what api functions a library version exposes, also is known to the agent without further explanation. 
Agents' training knowledge is associative, similar to human memory, and they can be prompted or triggered to recall more of it, or pay more attention to what they already know.
**How to answer**: Simply think through what the most popular / most common patterns, best practices, conventions, tools and workflows/processes are that are associated with the concrete scenario. Rank and triage. Present to Jörn a rationale for why each may/may not fit. Explain each item to Jörn, but not to agents. Jörn does not have as broad knowledge as agents do. Jörn will add his own ideas, and pick the most promising ideas to combine.

**Question:** What is the causal chain for this situation, i.e. what leads to the emergence of the situation?
**Why relevant**: Often we can/need to change agent behavior as to avoid scenarios from ever emerging. For example, telling an agent to use some set of convention can prevent a complex bug from reemerging, if the bug depends on earlier convention violations, and if the agents follow the conventions reliably. To plan where to intervene, we need to understand how the situation comes to happen.
**How to answer:** Look at real cases of the scenario, and the preceding events. This is a rather difficult task, since it borders on using theory of mind, but a preliminary analysis with brainstormed ideas for interventions that preempt the situation can be helpful for Jörn. Jörn will also add his own ideas to the pool, and he will pick the most promising interventions to try out.

**Question:** How is the system prompt / how is the situation already interacting with the agent? What instructions is the agent given in the moment?
**Why relevant**: Jörn does not memorized, read through and reason through the system prompt with the same amount of efforts as agents do, and Jörn does not have the same reading comprehension behavior as agents. So it's important to get an agent's perspective on what the system prompt says that's relevant to the situation. Sometimes misbehavior even is directly traced back to bad instructions that Anthropic added to claude code and which contradict what would be best practices in the situation. Fighting against the system prompt is especially annoying, and requires loud, strong prompting that overrides the system prompt and that sadly takes up attention and causes overhead. Disabling sections of the system prompt is sometimes possible, see the configuration options in llms.txt, but usually it is not editable.
**How to answer:** Agents already see the system prompt and tool instructions. If there's anything relevant, you can download a human-readable copy of the system prompt files via
```bash
bash .claude/skills/agent-design/scripts/download-system-prompt.sh <folder>
```
Then tell Jörn about where to read up on the relevant sections, so he is up to speed on what the agent is already being told by the harness itself, regardless of what other files we add.

**Question:** How can the situation be detected, i.e. what are reliable signals that an agent has entered a situation in which agents have made mistakes in the past and where our additional prompts are needed?
**Why relevant**: If we can detect the situation, we can trigger a tailored response workflow that helps out the agent to deal with the situation well. The most frequent way to encode triggers is via skill names+descriptions, since agents have been a trained on the claude code harness during their RLVR stage (reinforcement learning with verifiable reward) to load skills when skills become useful. There's some tradeoff to make between false positive and false negative rates, but Jörn has more experience and better assessment of the benefits/costs of both types of errors than agents do. So the focus is on suggesting potential signals and triggers, with preliminary guesstimates of their reliability. Jörn will then add his own ideas, and pick the most promising trigger(s) to use to inject additional prompts into the agent.
**How to answer:** Look at real cases of the scenario, and brainstorm what is related to the situation, e.g. by noticing first what is unusual / different about it. For concrete trigger pathways that SKILL.md descriptions enable, be ambitious and suggest natural language reasoning in the background even if it seems vague, but also more concrete natural language reasoning options. For concrete trigger pathways that hooks enable, consider what triggers are scriptable, e.g. via the symbolic tool calls from agents (tool hooks), including file reads, or lifecycle hooks. Jörn has more expertise in predicting whether a trigger moment is actually correctly detecting the situation, and whether a SKILL.md description and hook logic are reliable enough. So again Jörn profits from being given a lot of ideas to pick from, besides his own.

**Question:** What post-incident verification or feedback mechanisms could work?
**Why relevant**: Besides preempting mistakes and changing behavior during a situation that could lead to mistakes, we can also just correct mistakes. To do so we need to catch them reliably. The previous question of detecting a dangerous situation is related and similar, but the trigger there ideally precedes the mistake, while here we are after the fact. One important difference is whether the mistake is detectable from the agent's behavior, which is part of the session log / the agent's context window, and/or from the repo state, which consists of files, git log, and any bash tool that can be run. Like before, triggers can be implemented via skill descriptions and hooks. If mistakes are fine to be caught in batch and later, then we can also have review processes that review e.g. multiple commits, the whole repo, or one or more session logs. Reviews can be completely or partially automated, depending on whether finding a class of mistakes is within the capabilities of agents or requires Jörn's help. The more we can automate, the cheaper, faster and more scalable the feedback loop. For reviews, often instead of a SKILL.md an agent prompt is useful, since it isolates the review process from the working agent, and thus the subagents are more focused on the review alone and also work faster due to a smaller context window length. Good places to trigger the review-running behavior are the agent prompt descriptions, which ideally remind the agent that a review process is available and needs to be run, and a generic pre-merge workflow (already set up [TODO]) which reminds agents to spawn review subagents before merging into the main branch.
**How to answer:** Look at real cases of the scenario, and what complaints Jörn had about agent behavior. How can the mistakes be detected, i.e. how can Jörn's complaining be automated. Ask Jörn what tipped him off and what he paid attention to, since that often points directly at reliable and automatable signals. Brainstorm and filter potential detection mechanisms, e.g. by thinking about downstream effects (symptoms) of an error, about what distinguishes an error-free from an erroneous result, and whether there are tests/verification steps that can be done via scripts, via subagents, or via ideally efficient queries to Jörn. As usual, this is meant to help Jörn pick and design the best mechanism, not to replace his judgement or reduce his action space.

**Question:** What conflicts and trade-offs and synergies exist between the brainstormed ideas?
**Why relevant**: Straightforward. Jörn isn't as broadly knowledgable as agents, and does not know which combinations of patterns are popular, or what side effects / mechanisms the proposed scripts would have. This is simply to help out Jörn arrive faster at a good model of what ideas can be combined.
**How to answer:** Freeform reasoning through the interactions of the promising ideas so far. Common patterns to look out for: defense in depth, redundancy, parallelization, contradictions, fast paths, the most reliable option, the most easy-to-try option, etc etc. Present your reasoning to Jörn in a skimmable format, so that he quickly gets the idea of what interactions to be mindful of when designing the workflow.

**Question:** What costs do the brainstormed ideas cause outside the situations we want to address?
**Why relevant**: So far questions mainly revolved around what happens around the mistake, not what happens in entirely different situations. The main costs most solutions have are
a) one-time complexity cost to set up, especially if they require Jörn to e.g. write a lot of text
b) ongoing maintenance effort, e.g. if they can grow stale over time, including the cost of stale information causing agents to hold false beliefs
c) attention and instruction costs from the name+description of skills and subagents, from loaded skill bodies, from printed hook outputs, and from the read repo files / tool outputs. The attention cost is about overloading an agent with non-dismissable information that isn't quite obviously irrelevant to its situation. The instruction cost is about overloading an agent with too many instructions that need to be considered while it works, including instructions that are after consideration not applicable. E.g. rust conventions still eat into the instruction budget even while an agent works on python, though somewhat less severely.
d) runtime costs, e.g. long test suites or subagents that block the agent's workflow until they finish. Parallelization of e.g. multiple subagents or verification scripts can help reduce wall-time and is a useful trick to keep in mind.
**How to answer:** Go through the ideas one by one, assess effects and costs, and write up your guesses for Jörn. This requires a lot of experience with how agents act, and so the information is mostly there to help Jörn get started, and isn't suitable for making decisions. Jörn will contradict most cost and impact assessments, but it's good if basic ideas such as parallelization at least done once on side of the agent, so that it's in the list of recommendations and Jörn doesn't forget about it as an option.

In general, for all questions the goal is to help out Jörn by accelerating his work, not to replace his judgment. The agents' written output is meant to be skimmed quickly by Jörn, is meant to serve as a first exploration that Jörn can use to focus his attention where his expertise is actually impactful and needed, and is meant to surface ideas that Jörn would have arrived more slowly at than the agent.

[TODO: add real examples from actual agent-design sessions as they accumulate]
[TODO: add concrete strategies/learnings about how to answer these questions well]
[TODO: add other questions where an agent's preliminary investigation / draft answers helped Jörn work faster]

### 3. Jörn decides on the approach

Jörn applies his expert model to pick/design the workflow. The agent does not substitute its own judgment here. What the agent should do during this phase:

- Present step 2 findings in a format Jörn can skim in seconds (bullets, tables, short labels — not paragraphs)
- When Jörn picks/proposes an approach, ask clarifying questions until the proposal is unambiguous enough to implement. Specifically check:
  - What file type(s) should be written (skill, agent, hook, CLAUDE.md addition, repo artifact)?
  - What is the trigger / when should this activate?
  - What is the expected agent behavior after the material is loaded?
  - Are there known edge cases or exceptions Jörn wants handled?
- If something in Jörn's proposal seems like it might be misinterpreted by agents (ambiguous phrasing, implicit assumptions), flag it — Jörn can quickly confirm or rephrase
- Do NOT silently fill in gaps with your own judgment. If Jörn left something unspecified, ask rather than guess.

### 4. Agent writes it up

Draft the files Jörn specified, using the correct file formats. Before writing:
- Fetch the relevant spec: `curl -sL https://code.claude.com/docs/llms.txt` then the specific page
- For skill creation specifically, follow the guide at `references/skills-guide.md` (transcribed from Anthropic's "Complete Guide to Building Skills for Claude")
- Key from the guide: start with use cases, description = WHAT + WHEN + trigger phrases, progressive disclosure, test for triggering

### 5. Verify

- **Spec compliance:** Spawn a subagent per file to check it against the matching file format spec from llms.txt (e.g. to catch wrongly used YAML frontmatter fields or overlooked semantic aspects of the body)
- **Triggering:** Does the skill trigger on relevant queries? Test by asking a fresh subagent "when would you use X?" Does it NOT trigger on unrelated queries?
- **Actionability:**
  - *Vague word scan:* grep for "appropriate", "properly", "ensure", "good", "consider" — these delegate judgment without criteria. Replace with concrete verbs + objects.
  - *Naive subagent test:* spawn a fresh subagent with only the written file and a realistic test task. If it gets stuck, misinterprets, or does the wrong thing, the instructions aren't clear enough.
  - *Detectability check:* for any "if X then Y" instruction, verify X is actually observable by the agent (e.g. "if the code is complex" is undetectable; "if the file has more than 3 functions" is detectable).
  - *Redundancy check:* does the instruction add information beyond what agents already know from training? "Follow best practices" adds nothing. "Use `cargo clippy -- -D warnings` before committing" does.
  - *Script-or-language decision:* for anything where getting it wrong has high cost, check whether a script could enforce it instead of relying on the agent to remember.

### 6. Jörn review

Present the draft to Jörn before committing. Flag anything you're uncertain about.

## Reference sources

**Claude Code file formats and features:**
```bash
curl -sL https://code.claude.com/docs/llms.txt
# Then fetch specific page:
curl -sL https://code.claude.com/docs/en/<page>.md -o /tmp/<page>.md
```

**System prompt (what agents already know) — for Jörn to review:**
Committed at `references/system-prompt/`. To update and check for changes:
```bash
bash .claude/skills/agent-design/scripts/download-system-prompt.sh .claude/skills/agent-design/references/system-prompt
git diff .claude/skills/agent-design/references/system-prompt/
```

**Skills creation guide:**
`references/skills-guide.md` (transcribed from https://resources.anthropic.com/hubfs/The-Complete-Guide-to-Building-Skill-for-Claude.pdf)

## Key principles (from Jörn)

- Agents behave like their training data (frequent human tool use patterns). Agent knowledge is popular internet text. Design repo structures that activate trained patterns.
- Agents need tractable verifiable goals (RLVR training). They flail on tasks without clear verification. Build verification into structure, don't rely on agent self-reflection.
- Don't teach agents abstract models of agent cognition. Instead: for specific high-impact situations, apply expert knowledge and record the result as concrete artifacts agents can handle.
- 80/20: tackle the 20% of workflow types that cause 80% of agent-centric labor problems. For the rest, agents hand work back to Jörn.
