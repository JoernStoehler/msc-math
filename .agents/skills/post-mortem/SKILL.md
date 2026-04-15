---
name: post-mortem
description: Top-level, Jörn-invoked post-session reflection and blameless incident analysis. Use only when Jörn explicitly invokes `$post-mortem` or asks the top-level session for a post-session reflection; do not use from subagents, routine reviews, pre-merge checks, or normal task completion.
---

# Post-Mortem

Use this skill only in the top-level session. It needs the conversation history and Jörn's framing.

The output is advisory. Do not edit files, create archival notes, update skills, or create commits unless Jörn separately asks for edits in the same turn.

## Questions

Answer only the questions that apply:

1. What happened? Name the concrete prompt, tool action, file, or instruction surface.
2. What slowed the session down?
3. Which instruction, skill trigger, subagent prompt, or repo convention contributed?
4. Where did Jörn spend time that an agent could have spent?
5. What future agent behavior should change?
6. What wording or structural changes should a future guide-editing session try?
7. What conflicts, stale assumptions, or risks should that future session check?
8. What alternatives were considered and why were they not preferred?

## Output

Use this structure:

1. **Incident or friction:** one short paragraph.
2. **Likely cause:** concrete instruction or process mismatch.
3. **Suggested changes:** bullets that Jörn can approve, reject, or copy into a focused editing session.
4. **Checks for the editing session:** conflicts, files to inspect, and validation commands.

Keep it concise. Do not turn reflection into implementation.
