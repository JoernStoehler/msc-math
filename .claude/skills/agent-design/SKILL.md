---
name: agent-design
description: Collaborative workflow for designing agent materials (skills, agents, hooks, conventions) with Jörn. Use when Jörn asks to create or improve procedural files, investigate agent workflow failures, or redesign how agents handle a class of situations.
---

# Designing Agent Materials with Jörn

This is a collaborative workflow. Jörn has the expert model for what works with agents. The agent supplies cognitive labor (research, brainstorming, enumeration, drafting) that helps Jörn arrive at good decisions faster. The agent does NOT decide what workflow agents should follow — that requires expertise agents don't have.

## Expert model (from Jörn)

This is a simplified subset of Jörn's expert model. It's marginally useful for helping agents predict what might work, but it's not reliable enough to replace querying Jörn directly, as agents rarely fully apply it in enough depth.

### Training on Vast Training Data
- Agents behave like their training data (frequent human tool use patterns). Agent knowledge is popular internet text, including books, code, documentation, logs.
- Training knowledge is associative: agents can be prompted or triggered to recall more of it. A mere reminder (config file in the tree, code snippet in a familiar style) is often enough to activate trained behavior and increase the likelihood of recalling relevant knowledge.
- Popular patterns are cheap: conventions, tech stacks, factual knowledge (e.g. library APIs) needn't be explained. Just state the convention.
- Unpopular or novel patterns are expensive: weak or no training signal, need explicit detailed instructions.

### Training using RLVR
- Agents were trained on
  - Tasks with known or secret verification methods (e.g. code with known/secret test suites, human review of output correctness).
  - Tasks with known or secret progress signals (e.g. number of passed tests, code quality metrics, proof quality rating by another agent).
  - Large tasks that require decomposition, small tasks that do not.
  - Difficult tasks that require upfront planning and reasoning, easy tasks that can be done directly.
  - Autonomous tasks without intermittent human feedback.
  - Tasks inside projects, where the task is human-defined and quite useful.
- Agents were not trained, or at least not as much, on 
  - Tasks where no straightforward verification method can be found.
  - Tasks that are hit or miss, except if they are also frequent subtasks of other tasks
  - Workflows with frequent interruptions for human feedback.
  - Agent-generated tasks in a project, that may be useless, harmful or at least not the best way to proceed.
- The default agent behavior is attuned to situations similar to training, and degrades or even derails entirely in situations that are dissimilar to training, often without the agent realizing.
  - Agents don't reject directly assigned goals or questions as unproductive. An extra preceding task / a first subtask needs to be added to evaluate the goal/question, and activate the agent's learned capabilities at writing reviews.
  - Agents don't spend enough effort on finding verification methods and measures of progress, they are used to this being a small part of the full task. An extra preceding task again can correct the reasoning budget they spend.
  - Agents need familiar verification signals that they are used to incorporate during planning. For verification signals during implementation, their general capability to run arbitrary scripts and understand text compensates their lack of training diversity. For upfront planning, novel signals need to be explained and made predictable so that agents can anticipate whether a plan will pass verification.
- Agents were not trained to recognize when they are given a task dissimilar to training, and can derail entirely into unproductive busywork, loops, or iteration until misunderstandings accumulate and they declare success despite failure. To catch when an agent fails without realizing, a strong verification signal of the agent's output is needed, one the agent cannot unintentionally brute-force into passing through repeated attempts.

### Lack of Agent-Usage in Training
- Agents were not trained, or at least not as much, on tasks that involve using multiple agents, including
  - picking up a repository worked on by past agents instead of humans
  - handing off the repository to future agents
  - using a subagent
  - using multiple subagents in parallel (there is some training with subagents, but not much)
  - coordinating with other agents in parallel
  - predicting the behavior of agents (aka theory of mind)
- Agents were trained, at least somewhat, on situations that involve
  - interruptions in the agent's session
  - past reasoning summaries instead of the raw reasoning text being passed to the next reasoning step
  - switching focus and chaining together different phases of work where knowledge isn't homogenously accessed and behavior is heterogenous
  - continuing work of a human, passing on work to a human who rates the agent's helpfulness
  - being assigned work from a human
- Their default behavior is tuned to situations similar to training, and as a consequence agents are guessing and extrapolating without deep understanding when it comes to agent-agent interactions. Some classes of failures:
  - agents fail at theory of mind with agents, i.e. imagining how ai will interpret text and behave when the ai has a different state of knowledge and a different set of instructions than the agent has. Explicitly asking the agent to use theory of mind yields only moderate improvements, and slows the agent down and consumes a lot of attention.
  - agents prompt subagents similar to how humans prompt agents, but apply only a shallow theory of mind, such that standard delegation tasks work well (shallow imitation suffices), while more complex or unusual delegation patterns are nonsense and fail (deep understanding of how to delegate is needed, shallow imitation just fails to cover the new situation adequately).

### Bounded Rationality
- Agents have limited internal bandwidths when it comes to reasoning budget, attention, and reflection on the agent's session. They are less bottlenecked on recall of factual knowledge, i.e. have an efficient huge associative memory.
- As a consequence, a few pitfalls in prompting can overwhelm the agent and degrade the quality of their actions, triggering incoherence, confusion, or derailment, often without the agent noticing, since noticing this failure would require reflection/attention/reasoning budget. Examples:
  - If a too complex set of instructions is given, the agent may fail to follow them, basically overlooking and forgetting and making up new, contradictory instructions as it becomes detached from the written ones.
  - If an agent is given too many novel facts and concepts, or is given too abstract mental models, the application runs into reasoning budget limits and the agent fails to apply them deeply or consider the knowledge in its entirety.
  - If an agent is asked to reflect on a long session, the agent may make wrong recalls and gloss over such an unusual, un-training-like situation by making up plausible-sounding summaries instead that are detached from reality.
- This all means for the agent-design workflow that the section on Jörn's expert model and Jörn's experience are not going to work well, and the agent will not acquire expertise or experience beyond an unreliable spot-checking ability. The agent needs to defer to Jörn when the expert model or experience is needed for a workflow step.

### Design strategy
- 80/20: tackle the 20% of workflow types causing 80% of problems. For the rest, hand back to Jörn.
- Familiar developer artifacts (test suites, CI scripts, config files) get better engagement than novel formats.
- Cheap-to-try first. Iterate on observed behavior, not predicted.
- Feedback loops > getting it right the first time.

## Experience (from Jörn)

### What agents are bad at (defer to Jörn)

Tasks requiring:
a) Jörn's deep expert model (too complex to teach, anti-intuitive to agents)
b) Jörn's experience — knowing many failure/success stories
c) Theory of mind — imagining what a different agent will do from only a skill file

Concrete examples:
- Predicting how much attention agents pay to loaded instructions (over-optimistic)
- Predicting how agents interpret instructions (miss ambiguity)
- Predicting failure modes from first principles (feedback loops matter more)
- Generalizing from a best practice to more situations
- Deciding skill vs CLAUDE.md vs repo artifact (cheap to ask Jörn — seconds)
- A-priori evaluation of whether a procedural file adds value
- Questioning or rejecting goals

Note: "be explicitly critical" improves to barely-okay. Usually faster to ask Jörn.
Note: Some tasks take especially little time for Jörn: picking file formats, picking from suggestions, predicting how a draft will fail.

### What agents are good at (do these)

d) Work unrelated to agents — file tools, syntax checks, scripting
e) Shallow agent knowledge work — extracting from search, following this workflow
f) Applying human project/team management theory to a situation
g) Accessing trained knowledge and presenting it (associative recall, popular patterns)
h) Spawning subagents to observe behavior — testing whether a skill file works under realistic conditions

[TODO: revisit — Jörn flagged both lists as probably incomplete, 2026-03-24]

## Workflow

### 1. Gather real situations

Look at actual data, not hypotheticals:
- Usage report: `file:///home/vscode/.claude/usage-data/report.html`
- Session logs: `~/.claude/projects/-workspaces-msc-math/`
- Git history of failures: `git log --oneline -- .claude/`
- Current skills/agents: `.claude/skills/`, `.claude/agents/`

Present prioritized concrete situations to Jörn. He confirms which matter and how much.

### 2. Supply helpful information to Jörn

For each situation Jörn wants to address, autonomously gather and present helpful information to accelerate Jörn's decision-making and surface ideas he'd overlook otherwise. The following research questions are almost always helpful:

**What common practices/tools/patterns exist in training data for this kind of situation?**
Rank and triage. Explain each to Jörn (he doesn't have the agent's breadth). Present rationale for why each may/may not fit. Jörn picks the most promising to combine.

**What is the causal chain — what leads to the situation emerging?**
Look at real cases and preceding events. Brainstorm interventions that could preempt the situation. Jörn picks the most promising.

**What does the system prompt already say about this situation?**
Agents see the system prompt; Jörn doesn't have it memorized. Report what's relevant. Sometimes misbehavior traces directly to Anthropic's instructions contradicting best practices. Download a human-readable copy for Jörn via:
```bash
bash .claude/skills/agent-design/scripts/download-system-prompt.sh <folder>
```

**How can the situation be detected?**
Suggest signals and triggers — both via skill descriptions (agents are RLVR-trained to load skills when relevant) and via hooks (scriptable tool-call triggers). Jörn assesses reliability and false-positive/negative tradeoffs.

**What post-incident verification or feedback mechanisms could work?**
Distinguish: detection before vs after the mistake. Consider what's detectable from agent behavior (session log, context) vs repo state (files, git, bash). Consider automated reviews via subagents. Jörn picks mechanisms.

**What conflicts, trade-offs, and synergies exist between the ideas?**
Reason through interactions. Present in skimmable format so Jörn quickly sees what combinations work.

**What costs do the ideas cause outside the target situations?**
Main cost categories:
a) One-time setup complexity
b) Ongoing maintenance / staleness risk
c) Attention and instruction costs (overloading agents with non-dismissable info)
d) Runtime costs (long tests, blocking subagents — parallelization helps)

In general: the goal is to help Jörn by accelerating his work, not to replace his judgment. Present output that's fast to skim and surfaces ideas Jörn would have arrived at more slowly.

[TODO: add real examples from actual agent-design sessions as they accumulate]
[TODO: add concrete strategies/learnings about how to answer these questions well]
[TODO: add other questions where an agent's preliminary investigation helped Jörn work faster]

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

**Writing style for rules and skills:** Say what you mean. Spell out the concept, the reasoning, and the action in concrete terms. Don't use analogies or abstractions to carry the point — if you include them, they clarify, not replace. Concise means removing indirection, not removing content.

### 5. Verify

- **Spec compliance:** Spawn a subagent per file to check it against the matching file format spec from llms.txt (e.g. to catch wrongly used YAML frontmatter fields or overlooked semantic aspects of the body)
- **Triggering:** Does the skill trigger on relevant queries? Test by asking a fresh subagent "when would you use X?" Does it NOT trigger on unrelated queries?
- **Actionability:**
  - *Vague word scan:* grep for "appropriate", "properly", "ensure", "good", "consider" — these delegate judgment without criteria. Replace with concrete verbs + objects.
  - *Naive subagent test:* spawn a fresh subagent with only the written file and a realistic test task. If it gets stuck, misinterprets, or does the wrong thing, the instructions aren't clear enough.
  - *Detectability check:* for any "if X then Y" instruction, verify X is actually observable by the agent (e.g. "if the code is complex" is undetectable; "if the file has more than 3 functions" is detectable).
  - *Redundancy check:* does the instruction add information beyond what agents already know from training? "Follow best practices" adds nothing. "Use `cargo clippy -- -D warnings` before committing" does.
  - *Script-or-language decision:* for anything where getting it wrong has high cost, check whether a script could enforce it instead of relying on the agent to remember.

### 6. Set up feedback collection

Before shipping, decide how future agents will report on whether the new material works:

- **Session-level feedback:** The post-mortem skill already gathers end-of-session feedback. Ensure the new skill/workflow is on the post-mortem's radar (e.g. by mentioning it in the session handoff).
- **Subagent feedback:** If the new material includes subagent workflows, tell subagents to write observations to a feedback file (e.g. `feedback/<skill-name>.md`) or report back to the parent agent who includes it in the post-mortem. Subagent memory is another option.
- **Do NOT:** Write feedback directly into SKILL.md or agent prompt files. Do NOT overanalyze feedback inline — raw observations only. Analysis and updates to procedural files should be done by a dedicated agent-design session that has read the right materials and has Jörn in the loop.

The goal is cheap data collection now so that a future agent-design session can reevaluate and update the workflow with real evidence.

### 7. Jörn review

Present the draft and discuss it with Jörn before declaring it accepted. Do not guess at approval — get an explicit positive confirmation from Jörn. Also present alongside the draft a prioritized list of spots Jörn should pay attention to, e.g. because you are not as certain there and/or they are higher impact. This helps iterate faster on the draft. Also accept when Jörn asks you implicitly to go back to an earlier stage — not all drafts work out in the end and pivoting is cheap.

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
