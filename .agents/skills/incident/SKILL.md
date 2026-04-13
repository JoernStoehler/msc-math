---
name: incident
description: Record an agent behavior incident to feedback/ for the next context engineering pass. Use when Jörn flags something the agent did wrong mid-session.
user-invocable: true
argument-hint: optional description of the incident
---

# Incident

1. **Identify.** Use `$ARGUMENTS` if provided. If unclear, ask Jörn — don't guess.
2. **Write entry** in matching `feedback/` file (one of: `rules.md`, `skills.md`, `agents.md`, `output-style.md`):
   ```
   ### YYYY-MM-DD — Short description
   What happened. What should have happened.
   **Pattern:** Abstract error class. Reference prior entry if same class.
   ```
3. **Check memory.** If this reveals a persistent behavioral rule, save/update a feedback memory. If a memory already covers this but the incident recurred, note that — the memory alone isn't enough.
4. Continue with prior work.
