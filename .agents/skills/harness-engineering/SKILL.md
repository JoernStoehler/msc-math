---
name: harness-engineering
description: Use when Codex edits, reviews, or proposes changes to repo-local harness material such as `AGENTS.md`, `.agents/skills/*/SKILL.md`, `.codex/agents/*.toml`. Not needed for reading and using harness material. Not needed for editing other files.
---

# Harness Engineering

For projects that make use of massive numbers of AI agents, a lot of tradeoffs shift away from standard projects, towards ensuring that agents can work more effectively, with less human oversight or instruction. Harness Engineering is the new name for picking conventions, writing material, and refactoring projects to be more useable by agents.
This skill conveys a bunch of instrumental objectives that increase agent autonomy across long chains of agents, and breaks them into heuristics that can be applied to the repository without having to first observe agent behavior over a long time to assess the ground-truth agent success/effectiveness in the project.

The name "Harness Engineering" was coined by https://openai.com/index/harness-engineering/ "Harness engineering: leveraging Codex in an agent-first world", though the article is rather basic in providing actionable help.

## Use Cases, Instrumental Objectives, and Heuristics

With agents as the sole readers for most files, the best writing style / the tradeoffs between different quality aspects change.
Files are still read more often than written, so iterating in writing, and explicit style improvements are worth it.

We list the main interaction patterns that agents have with the repository while working on their tasks, and highlight instrumentally useful properties the file content and folder structure should have to make those interactions more effective.

### Files as Sources of Knowledge
- **Information Extraction**: Agent reads a whole file, and later combines content with other observations and with its own knowledge to come up with new ideas/inferences.
  - content is relevant, focused, complete, neither over- nor under-specific
  - style is plain, unambiguous, clear, with standard terminology, low cognitive load, single-concern sentences
  - content is correct, trustworthy, checkable, with explicit certainty/uncertainty
  - no mixing of observation, hypotheses, evidence, inference, utility, predictions, terminal and instrumental goals, domain knowledge and instructions, etc.
  - implications are spelled out
- **Finding Information**: Agent finds files or sections within large files, before reading them whole.
  - folders and files have descriptive names, even if that makes the filenames long
  - folders and files are in predictable locations
  - keywords and terminology is predictable and grep-able
  - related material is referenced with explicit links
  - knowledge often needed together is colocated in the same file, folder, file-type, etc. to be predictably found together
  - files are single-concern / not too long, so that agents don't waste most of their reading time on irrelevant content
- **Updating Information**: Agent propagates knowledge/decisions/assessments across the repository.
  - file structure is modular, with intertwined content close together, so that natural updates are contiguous diffs rather than spread across whole files.
  - style is plain, single-concern sentences, so that grammar does not get in the way of editing
  - content is checkable, so that the agent can discover newly introduced errors and fix them
  - after the update, "information extraction" must be effective still

### Code
- The section **Files as Sources of Knowledge** still applies to code files and documentation files
  - code is readable, clear, devoid of magic and cleverness, plain and stupid
  - symbols are descriptive, even if that makes them long
  - docstrings and comments are focused on information transfer, trustworthy, and maintainable, with checkable claims, and spell out implications
  - code is correct, trustworthy, checkable, with explicit gaps/trust/conjectures
  - code uses standard terminology, avoids custom aliases and wrappers that require fresh agents to read about them first
- **Running Code without Reading It**: Agent builds/runs/tests code, processes data, filters output, without having read the code or data beforehand.
  - the command behavior is predictable from the outside, matching patterns that gpt-5.5 was trained on, including command line arguments, resource usage, stdout/stderr syntax and verbosity, side effects, error behavior, caching/rerun behavior, etc.
  - `--help` output explains the behavior well enough to avoid trial-and-error and avoid reading the code.
  - if there's a standard use case, the standard command behavior optimizes for it
  - if there's no standard use case, explicit inputs are required without providing false defaults
  - names/text is optimized for agent readers (see "information extraction")
- **Orchestrating Multiple Commands**: Agent chains multiple commands in parallel/sequence, interspersed with linux tools and ephemeral bash scripts.
  - granularity of executables/commands matches the conceptual steps the agent thinks in, neither too fine-grained and certainly not too coarse-grained
  - commands are compatible with standard linux patterns, such as pipes, grep, jq, parallel execution, filesystem-based communication
- **Verifying Code**: Agent reassesses or upgrades his belief in the correctness/usefulness/other types of trust of code.
  - code is modular and self-contained with explicit interfaces, often states contract logic in the docstring
  - comments provide breadcrumbs for verification reasoning, e.g. to elevate something to attention or provide non-codified arguments
  - code is single-purpose, matches the conceptual domain layer that agents reason in
  - re-assessment is automated via the default test suite, situational test suites, type checkers, and runtime asserts
  - tests, asserts, type checks are in predictable locations where they work reliably
  - imperfect checks, where ideal concept and operationalized implementation diverge, explain and justify the difference
- **Maintainance**: Agent cleans up tech-debt and propagates learnings/exploration results across the codebase.
  - code is concrete/specialized for the occasion, instead of abstracting across multiple call contexts that don't also move together
  - related code stays together, ideally in one file, to minimize indirection/edit scope
  - the complexity:verbosity tradeoff for agents is way more towards avoiding complexity than for human developers
  - concretely: it's easier to maintain two copies of a function, edited slightly for their different use cases, than to maintain a more complex function that abstracts
  - concretely: it's easier to maintain inlined orchestration that can be read top-to-bottom, with useful comments/variable scope boundaries, than jump between called-once helpers

### Instructions
- We view "instructions" as "narrowing the agent's task from the project goal towards a subgoal", perhaps with connotations such as "it's not the agent's responsibility to deal with subgoals besides its own".
- For many agents, it would be wasteful to verify first that their task makes sense / fits the project goal, for some it is of utter importance to avoid harmful or wasted labor.
- The section **Files as Sources of Knowledge** still applies to text snippets that also contain instructions for agents, since usually instructions/scoping are accompanied by information transfer.
- **Contextualizing the Task**: Agent looks up material, reasons about the project goal and the task definitions, adjusts its understanding of the task and pulls in / defers aspects to make task completion more useful for the long-term project.
  - breadcrumbs connect instructions to the project goal, since all instructions are ever only as useful as they help the project succeed, i.e. they are purely instrumental objectives
  - accompanying information transfer, such as relevant domain knowledge or claims about past and future work, are correct, checkable, with explicit certainty/uncertainty
  - implications are spelled out if already known, and are mentioned as absent if the receiving agent is the first agent who has to figure out the implications
  - the task is scoped to be modular, i.e. low in interference with other tasks, and follow-ups such as merging/unification have an unambiguous owner/schedule
  - an escape hatch is provided for if the task turns out to be harmful or at least not helpful in its form, or if helpfulness cannot be assessed with sufficient confidence to justify large resource expenditure; standard hatches are to refuse and escalate back to a parent task, or to inform Jörn and ask for help/guidance
  - knowledge about tasks is documented in the repo, ideally as files in a dedicated folder, and is as usual epistemically clear, checkable with breadcrumbs, and maintainable
- **Task Execution**: Agent plans, executes, reorients, iterates, attempts-and-pivots, reviews, and so on until it completes the task.
  - task instructions do not prescribe prematurely any process, steps, constraints, content, actions. the task agent has more information and context, has more focused reasoning, and so is in a better position to pick the process for how to achieve the task objective than any planner has in advance
  - the task instructions define an objective to plan towards, that is observable to the agent at least in principle, and ideally even in practice
  - the task instruction does not prescribe prematurely how to measure task progress, since that depends too much on the process usually
  - useful suggestions are framed clearly as predictions, or are merely given as knowledge without spelling out/prescribing the implied actions prematurely
- **Review**: Agent reviews another agent's deliverable.
  - reviews are adversarial to counteract the casual blindspots of gpt-5.5 agents that are shared between worker and reviewer
  - review output is information transfer, and so it is correct, checkable, with explicit certainty/uncertainty, and implications are spelled out where known already
  - review tasks should be scoped wide, unrelated observations that are predicted to be valuable can be pulled into the output as well
  - failure of the reviewer to evaluate/review/verify something is evidence that a quality baseline wasn't met yet with regards to checkability/maintainability
- **Help from Jörn**: Agent asks Jörn to help with contextualization, planning, process choices, review, final done checks, and anything else really.
  - gpt-5.5 agents usually majorize Jörn in standard, narrow-scoped tasks, and his help will not be useful, so the interruption cost can be skipped
  - for task contextualization, Jörn has a better view of long-term project health (i.e. what future agents profit from) and of tasks that are not well-described by the repo. for tasks that are well-described or even defined/tracked, Jörn is unlikely to have more information.
  - as measured, the main draw on Jörn's limited time is from highly complex requests such as restructuring the task scope of multiple tasks, and from high-frequency short requests that collect information inefficiently bit-for-bit in a conversational format rather than an asynchronous work-and-report format.
  - questions and requests for Jörn are worded clearly, unambiguously, are neither over- nor under-specific
  - they use terminology that Jörn knows, rather than that all gpt-5.5 agents know from training, or that only the current agent knows
  - they use progressive disclosure, since Jörn, unlike agents, can skim and skip message text
  - they provide identified knowledge, reasoning breadcrumbs, predictions and so on that may accelerate Jörn's answer or improve its quality. This interacts with progressive disclosure.
- **Durable Instruction Material**: Agent reads a file with instructions that were written for multiple tasks, not just one.
  - `AGENTS.md`, `SKILL.md` files, and less durable `/tmp/` prompt snippets are the canonical locations for instructing future agents to pursue some instrumental (!) objectives besides their instrumental local task objective.
  - all instructions still just serve the project success, and if durable they are often about long-term invariants / quality attributes, or milestones and components of the full project goal.
  - the style is more careful than normal prompts, since contextualization is expensive for instructions that are instrumental via long-term effects instead of short-term effects
  - in particular, epistemic status and dependencies are made explicit, distinguishing ideas, suggestions, conventions, hard constraints, and tracking what is instrumental for what via breadcrumbs, all the way to the project goal.
  - just like always, the **Files as Sources of Knowledge** section applies, e.g. style should be clear, plain, focused on information transfer, correct, checkable, and so on.

## File Genres

### Suggestions for AGENTS.md
A few ideas and suggestions that in the past improved agent effectiveness:
- use AGENTS.md as a map to the repository, and nothing more. No long conventions, no detailed information, just files, commands, and the very basics to collapse uncertainty for fresh agents with regards to what kind of project they find themselves in.
- no nested AGENTS.md, that's outdated style for older models and not something gpt-5.5 was trained strongly on

### Suggestions for SKILL.md Files
- use SKILL.md names and descriptions to tell agents whether to read said files or whether to skip reading them. See `$skill-creator` for good guidance on this. Don't use AGENTS.md for listing skills.
- simply and plainly transfer information, both about what behavior/what instrumental subgoals serve the project goal, and about domain knowledge or other facts or reasoning artifacts such as predictions or inferences. Usually however domain knowledge has a better place in standard repo files rather than `SKILL.md` files, and skills merely point to the relevant domain knowledge.
- use references/*.md files for content that's not worth inlining into the SKILL.md, or that fits a nicer standard genre of file, such as a guide book, a report/overview of some topic, template or example files, etc. Keep SKILL.md focused on instructions and meta-discussions about the project goal, since that kind of content has no good genre of file that agents are used to reading or writing.
- optimize hard, this is especially difficult content, and it's worth iterating
- fresh subagents can provide a fresh perspective, mock tasks are more realistic than imaginary playthroughs, and Jörn can review more accurately how instruction material fits together with the project goal
- loose bullet list style and short prose have proven more maintainable, plain, and Jörn-reviewable than long prose
- if a complex structure is needed, that indicates usually that whatever instructions need to be conveyed are simply too complex to work out in the end
- since agents lack any experience with writing skills, and with predicing long-term impact on agent behavior, the task needs to be anchored on a source of truth rather than flailing predictions; it's basically impossible to write a SKILL.md file without having two constrastive examples: one where the agent behaved ineffectively, and one where the agent behaved effectively; so the first priority is always to ensure both examples are understood thoroughly, before even writing the first word. Same is true for skill updates. Jörn can be asked to provide a recap of two incidents, if the ongoing session isn't the source.
- Due to the risks, and how late feedback arrives / how subtle feedback is, edits to SKILL.md files need 1:1 review by Jörn, i.e. Jörn needs to read and approve the whole file before it gets committed.
- Note: only the main branch AGENTS.md, SKILL.md files are loaded by the codex harness, so edits in worktrees are not immediately active.

## This Repository

Here's an overview of what the harness looks like in `msc-math`. Note that there is heavy overlap between these concrete files and `$harness-engineering`, since
latter is too complex to be loaded for everyday use.

### AGENTS.md
- sections: Project, Files, Map Files, Commands
- Map Files are files that merely index / summarize other files, as a more detailed intermediate layer

### $harness-engineering
- This file was written by Jörn.
- Except for typo-corrections, it should never be touched by agents.
- It is only needed when editing the harness, not for using it

### $skill-creator
- Built-in, not all that great but reinforces good practices

### $chat-advanced
- the basic chat flows between agent and Jörn work okay
- this is for more ambitious flows, e.g. scoping the task, information transfer to Jörn, salvaging something from a failed attempt

### $formal-math
- mainly about how to make the mathematical content of msc-math more correct, checkable, Jörn-reviewable, and maintainable concretely
- fleshes out the generic instrumental objectives from the "Files as Sources of Knowledge" section

### $rust
- again, fleshes out the generic instrumental objectives for rust code, tests, comments
- the most complicated pattern here is that we want code-math correspondence, i.e. code follows conceptually elegant mathematical definitions, and mathematical definitions enable readable code
- basic guidance for the api boundary between `crates/` and `experiments/`

### $post-mortem
- reflection after a session to learn how to improve the harness over time
- Jörn accumulates examples and directional feedback, so the workflow only indirectly edits the harness

### $research-experiments-data
- again, fleshes out the generic instrumental objectives for research experiments
- has reference files with even more specific architecture and execution patterns for research experiments

### $thesis
- again, fleshes out the generic instrumental objectives for writing thesis content
- since the thesis is the one artifact in the repo that is actually read by humans, and that leaves the internal project repository, tradeoffs are again different
- basically, standard knowledge of how to write a good thesis becomes relevant, and writers optimize for the real+imaginary audience of thesis advisors+master students

### $tasks
- unique instrumental goals for project management, including maintaining the durable task tracking system, and writing good task descriptions for other agents
- main focus is on reinforcing prompting for gpt-5.5 , which differs from prompt engineering for older models, since gpt 5.5 mainly just needs the objective and no help

### `tasks/<group>.md`
- the task tracking system

### $plan-implement-review
- orchestration pattern that helps gpt-5.5 resolve contradictions and opposing pressures in the right direction
- counteracts reward-hacking (concrete example: tests must fail on bad code in order for test success to even be an instrumental objective)
- balances the benefits and costs of pivoting to better paths and propagating learnings
- highlights a few useful patterns, such as ephemeral feasibility spikes, babble-and-prune

### $subagent-prompts
- a specialized version of $harness-engineering for writing ephemeral instruction packets for subagents

### $git-worktrees-merge
- enables parallel work by multiple agents without interference
- suggests standard git practices that are good enough, e.g. full reviews only before merges into main

### $map-file-regenerate
- brief workflow/checklist for all the different map files (see `AGENTS.md`) to keep them from going stale
