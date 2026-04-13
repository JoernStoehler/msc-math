# Agent Expert Model (Background Reference)

This is a simplified subset of Jörn's expert model about how agents behave. It provides background context for understanding agent infrastructure design decisions. It is NOT reliable enough to substitute for querying Jörn directly — agents rarely apply this model deeply enough to make good design decisions from it alone.

## Training on Vast Training Data

- Agents behave like their training data (frequent human tool use patterns). Agent knowledge is popular internet text, including books, code, documentation, logs.
- Training knowledge is associative: agents can be prompted or triggered to recall more of it. A mere reminder (config file in the tree, code snippet in a familiar style) is often enough to activate trained behavior.
- Popular patterns are cheap: conventions, tech stacks, factual knowledge needn't be explained. Just state the convention.
- Unpopular or novel patterns are expensive: weak or no training signal, need explicit detailed instructions.

## Training using RLVR

Agents were trained on:
- Tasks with known or secret verification methods (e.g. code with test suites, human review)
- Tasks with progress signals (e.g. number of passed tests, quality metrics)
- Large tasks requiring decomposition, small tasks that do not
- Difficult tasks requiring upfront planning, easy tasks done directly
- Autonomous tasks without intermittent human feedback
- Tasks inside projects, where the task is human-defined and useful

Agents were NOT trained as much on:
- Tasks where no straightforward verification method exists
- Tasks that are hit or miss
- Workflows with frequent interruptions for human feedback
- Agent-generated tasks that may be useless or harmful

The default agent behavior is attuned to training-like situations and degrades in dissimilar situations, often without the agent realizing.

## Lack of Agent-Usage in Training

Agents were NOT trained much on tasks involving:
- Picking up a repository worked on by past agents
- Handing off to future agents
- Using subagents, especially multiple in parallel
- Coordinating with other agents in parallel
- Predicting agent behavior (theory of mind)

Consequences:
- Agents fail at theory of mind with other agents — imagining how a different agent will interpret text given different knowledge and instructions
- Agents prompt subagents using shallow imitation of how humans prompt agents. Standard delegation works; complex or unusual delegation fails.

## Bounded Rationality

- Agents have limited reasoning budget, attention, and reflection capacity. They are less bottlenecked on factual recall (efficient associative memory).
- Too-complex instructions → agent overlooks/forgets/fabricates instructions
- Too many novel concepts → reasoning budget exhausted, shallow application
- Reflection on long sessions → wrong recalls, plausible-sounding but detached summaries
- Agents don't recognize when they're in a training-dissimilar situation and can derail into unproductive busywork or loops.

## Design Strategy

- 80/20: tackle the 20% of workflow types causing 80% of problems
- Familiar developer artifacts (test suites, CI scripts, config files) get better engagement than novel formats
- Cheap-to-try first. Iterate on observed behavior, not predicted.
- Feedback loops > getting it right the first time

## Default Agent Behaviors (what they do if not corrected)

These are observed defaults — what agents actually do when instructions are absent or vague. Understanding these helps write instructions that correct the right thing.

- **"Push back" means "contradict the repo":** Agents interpret "push back if Jörn is wrong" narrowly — they flag factual contradictions with repo state, but don't push back on suboptimal approaches, imprecise formulations, or drift from the project goal. CLAUDE.md now expands this explicitly.
- **Serve the literal subtask, not the project goal:** Agents execute the immediate request without checking whether it still serves the thesis. They don't flag when a subtask has drifted or become counterproductive. CLAUDE.md now has a "keep the project goal in view" bullet.
- **Strong bias toward action:** Agents default to acting rather than asking. Instructions like "default to action" reinforce an already-strong bias and can cause agents to act on wrong understanding. Omitting such instructions is often better than including them.
- **Ownership language and permission-seeking:** Agents say "my analysis suggests" and end with "Should I proceed?" despite instructions not to. The "What to avoid" section corrects for this but compliance is inconsistent.
- **Confidence markers ignored:** Instructions to use explicit confidence levels ("~70% confident", "speculative:") were not reliably followed. Removed from CLAUDE.md as a result (2026-03-29).
- **Terse reporting after tool calls:** Agents read files/code via tools, then report findings as incomprehensible terse bullets without quoting or explaining. CLAUDE.md "Complete" quality corrects for this.

## What Agents Are Bad At (Defer to Jörn)

- Predicting how much attention agents pay to loaded instructions (over-optimistic)
- Predicting how agents interpret instructions (miss ambiguity)
- Predicting failure modes from first principles
- Generalizing from a best practice to more situations
- Deciding skill vs CLAUDE.md vs repo artifact
- A-priori evaluation of whether a procedural file adds value
- Questioning or rejecting goals

## What Agents Are Good At

- Work unrelated to agents — file tools, syntax checks, scripting
- Shallow agent knowledge work — extracting from search, following workflows
- Applying human project/team management theory
- Accessing trained knowledge (associative recall, popular patterns)
- Spawning subagents to observe behavior — testing whether infrastructure works
