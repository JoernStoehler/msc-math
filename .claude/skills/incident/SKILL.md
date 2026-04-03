---
name: incident
description: Record an agent behavior incident to feedback/ for the next context engineering pass. Use when Jörn flags something the agent did wrong mid-session.
user-invocable: true
argument-hint: optional description of the incident
---

Jörn flagged an incident — something the agent just did that needs to be recorded for the next context engineering pass over the repo.

## Steps

1. **Identify the incident.** If `$ARGUMENTS` describes it, use that. If `$ARGUMENTS` is empty or you're unsure what Jörn is referencing, ask him. Don't guess — the cost of recording the wrong incident is higher than one question.

2. **Write the entry.** Append to the matching file in `feedback/` (one of: `rules.md`, `skills.md`, `agents.md`, `output-style.md` — pick by category). Use this format:

   ```
   ### YYYY-MM-DD — Short description

   What happened: the specific sequence of events.
   What should have happened: the correct behavior.

   **Pattern:** Abstract the error class. Name the pattern if it matches a prior entry.
   ```

   Check whether a prior entry in the same file describes the same error class. If so, reference it ("Same error class as YYYY-MM-DD 'title'").

3. **Check for memory entry.** If the incident reveals a behavioral rule that should persist across sessions, save or update a feedback memory in the auto-memory system. If a memory already covers this, note that the memory exists but the incident recurred — this means the memory alone isn't enough and the next context engineering pass should strengthen the rule.

4. **Done.** Don't narrate what you saved — Jörn can check the diff. Continue with whatever work was in progress before the incident.
