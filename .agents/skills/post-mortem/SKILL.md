---
name: post-mortem
description: Top-level, Jörn-invoked post-session reflection and blameless positive or negative incident analysis. Use only when Jörn explicitly invokes `$post-mortem` or asks the top-level session for a post-session reflection; do not use from subagents, routine reviews, pre-merge checks, or normal task completion.
---

# Post-Mortem

Use this skill only in the top-level session. It needs the conversation history and Jörn's framing.

The output is advisory. Do not edit files, create archival notes, update skills, or create commits unless Jörn separately asks for edits in the same turn.

## Operating model

Prefer concrete operational failures over abstract workflow stories.

- Name what actually went wrong first: a prompt ambiguity, a tool action, a bad edit, a missed question, a stale instruction, a wasted rerun, or a preventable user interruption.
- Treat process theories as secondary. Use them only when they explain an observed failure.
- Do not invent a new "problem" unless it caused concrete harm in the session.
- If a proposed harness change would not have prevented or shortened a real failure, treat it as optional polish or omit it.
- Separate:
  - actual problem
  - contributing factor
  - optional wording cleanup

Good reflection:
- "The worker edited root `main` instead of its assigned worktree; strengthen the worker cwd contract."
- "The agent preserved obvious leaked tracked junk too long; ask for immediate revert once confidence is high."

Bad reflection:
- "The session lost alignment with its phase energy."
- "A generalized mode-transition framework may be needed."
  Use wording like this only if you can point to a concrete failure it would have prevented.

## Questions

Answer only the questions that apply:

1. What happened? Name the concrete prompt, tool action, file, or instruction surface.
2. What slowed the session down?
3. Which instruction, skill trigger, subagent prompt, or repo convention contributed?
4. Where did Jörn spend time that an agent could have spent?
5. What future agent behavior should change?
6. What positive behavior should future agents preserve or repeat?
7. What wording or structural changes should a future guide-editing session try?
8. What conflicts, stale assumptions, or risks should that future session check?
9. What alternatives were considered and why were they not preferred?

## Output

Use this structure:

1. **Incident, friction, or positive pattern:** one short paragraph.
2. **Likely cause:** concrete instruction, prompt, skill wording, tool action, or process mismatch.
3. **Suggested changes:** bullets that Jörn can approve, reject, or copy into a focused editing session. Each bullet must say what concrete failure it addresses.
4. **Checks for the editing session:** conflicts, files to inspect, and validation commands.

Keep it concise. Do not turn reflection into implementation.
Do not pad the report with governance language, abstract phase models, or speculative architecture unless they are clearly tied to an observed failure.
