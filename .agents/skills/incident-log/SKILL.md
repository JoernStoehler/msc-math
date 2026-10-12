---
name: incident-log
description: Capture a user-identified consequential conversational or workflow incident as low-friction evidence intake. Preserve bounded evidence and uncertainty for later triage; do not diagnose, generalize, change prompts, or run a postmortem as part of intake.
---

# Incident log

Capture an incident when the user asks to log one or explicitly identifies one. Keep intake cheap: record the evidence, then continue the active task.

## Default destination

For cross-project agent/model/harness incidents, write one Markdown file to the `agent-skills` repository:

`incidents/intake/<UTC-timestamp>-<slug>.md`

Use a filesystem-safe UTC timestamp such as `YYYY-MM-DDTHHMMSSZ` and a short lower-kebab-case slug. Do **not** allocate counters or list the directory to choose a filename. If a create collides, retry with finer timestamp precision or a short suffix.

If the current project already has an explicit incident owner, follow that instead.

If the default repository is unavailable, retain the intake in the current project’s existing durable evidence location and state the actual path plus the unavailable canonical owner. Do not provision a repository or interrupt the parent task merely to obtain the default path. If no durable writable location is available, return the compact intake in chat and explicitly state that it has not been saved.

## Intake content

Markdown is intentionally only lightly structured. Record what is cheap and evidentially useful:

- capture time in UTC;
- surface/harness and exact model/version when known;
- brief context/objective;
- bounded evidence: exact short quotes, transcript anchors, tool/action results, or pointers;
- observed action/outcome and material consequence;
- the user's explanation when provided;
- uncertainty or missing information.

Keep **observation**, **user explanation**, and **agent hypothesis** distinct. Do not invent a causal explanation for the agent's behavior. Unknown is an acceptable value.

Do not require tags, incident IDs, severity, taxonomy, root cause, remediation, or a normalized schema at intake time. Those belong to downstream triage if useful.

## Preserve the intake boundary

Treat intake as source evidence. Later agents may split incidents, connect related cases, re-investigate, generalize, build evals, or propose interventions. Do not silently rewrite the original evidence to fit a later theory.

When a future replay/eval boundary is obvious and cheap to preserve, include the pre-failure context/configuration pointer and desired observable behavior. Do not build the eval during intake unless asked.

When maintaining this skill, read [design notes](references/design-notes.md) for its intake boundary, ownership history and bounded evaluation evidence.
