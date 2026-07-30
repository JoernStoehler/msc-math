---
name: post-mortem
description: Use only when Jörn asks for a postmortem, session reflection, lessons learned, avoidable-work analysis, or what the model, repository, harness, or Jörn should learn from a session. Review both imperfect and unexpectedly successful behavior. Do not use for ordinary completion summaries.
---

# Post-Mortem

## Purpose

Turn a completed or paused session into evidence for improving expected thesis
success. The report should help decide what future work should learn about the
repo, harness, prompts, task approach, Jörn's involvement, or model capability,
and whether that learning justifies a durable change. It is not a completion
summary, blame exercise, or exhaustive event log. Invocation authorizes
inspection, analysis, and proposals, not edits.

## Process

Reconstruct the goal, expected route, important decisions, interventions, and
outcome. Inspect the actual artifact and relevant current repo state before
making claims about them; distinguish problems that occurred from problems
still present at the end. Use `$codex-session-log-parsing` when exact chronology,
tool actions, compaction, or subagent lineage matters.
Use `$gpt-56-harness` when evaluating durable harness behavior or recommending
changes to AGENTS.md, skills, Codex configuration or agents, or reusable prompts.
This route does not expand the postmortem's no-edit authority.

Look for process learning. Useful questions can include:

- What was imperfect, omitted, wasted, risky, or unnecessarily costly? When was
  a better route knowable, and what would have selected it?
- What worked better or more reliably than expected? Which expected failures
  were absent, and what does that update?
- What does the session show about the model under these task conditions,
  without generalizing one case into a universal capability claim?
- How did explicit instructions and the repo's implicit structure—ownership,
  APIs, defaults, file placement, discovery, validation, automation—shape the
  trajectory?
- How did the initial prompt, task and reviewer prompts, subagents, tools, and
  review gates affect what was noticed or missed?
- Which Jörn interventions supplied genuinely private context or good judgment,
  and which attention costs could the agent or repo have avoided? Judge this by
  what was knowable then, not by hindsight.
- What interactions between these factors explain the outcome, and what
  downstream work or risk did the session create?

These are prompts for investigation, not a finding checklist. Follow the
evidence and add other useful questions. Separate observation from causal
interpretation, consider plausible alternatives, and calibrate confidence.

## Report

Lead with the outcome and the most important update for future thesis work.
Report the smallest set that preserves the material learning, grouping events
that support the same conclusion. Include concrete changes or discriminating
tests when useful; rank them by expected project value, recurrence, cost,
confidence, and side effects when priorities differ. Do not turn every one-off
error into a permanent rule.

Translate model and process learning into the surfaces that can actually
change: Jörn's behavior, ordinary repo files or structure, and harness files.
When recommending a durable change, name its owner: Jörn's future behavior, an
ordinary repo file or structural boundary, or a harness file. Do not list
session-local prompts or task approaches as change surfaces: they disappear
with the session unless assigned to a reusable owner. Desired agent behavior
describes an outcome rather than the control that would produce it, and a
model-capability update is evidence for choosing an intervention rather than an
intervention itself. Map a recurring lesson to a concrete owner above, or say
that no durable change is justified.

Do not say there was nothing to learn merely because the outcome was good.
Silence is justified only when outcome and process were effectively perfect and
that perfection was itself expected. Do not invent a fault to fill a category,
repeat one lesson in several sections, or implement proposals without a separate
request.
