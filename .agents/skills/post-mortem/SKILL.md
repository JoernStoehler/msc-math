---
name: post-mortem
description: Use when Jörn asks for a postmortem, session reflection, lessons learned, avoidable work analysis, or follow-up suggestions after a Codex session.
---

# Post-Mortem Conventions

## Instrumental Objective

Postmortems are for Jörn after a session. They save him from rereading the chat
and reconstructing lessons from scratch. Future agents benefit indirectly:
Jörn may turn a lesson into `AGENTS.md`, a skill, a task file, or his own notes
after reading and discussing the chat report.

A good postmortem lets Jörn quickly do two things:

1. Check whether the agent missed something, got something wrong, or understood
   the session differently than he did.
2. Assess suggestions well enough that his own ideas, objections, and follow-up
   questions surface faster.

A bad postmortem makes Jörn reconstruct the conversation, hides uncertainty,
rounds assessments into vague words, invents terminology or structure, or gives
generic advice without enough reason to assess it.

## Suggestions

- Use concrete bullets. Jörn should be able to accept, reject, or correct each
  bullet without first asking what the agent means.
- Summarize concrete results before lessons.
- Separate what went well, what went badly, wasted or avoidable labor, and
  suggestions.
- Successful work can contain wasted labor when hindsight shows a faster or more
  autonomous path.
- Mark causal uncertainty. Say what appears to have helped; do not pretend the
  causal story is known.
- For strength or priority claims, name the axis. Useful axes include severity,
  avoidability, future risk, confidence, urgency, and expected value.
- Use ad hoc quantitative scores only when they help compare items. Say what the
  scores measure.
- Avoid unqualified "should", "better", "strong", or weights. State the reason
  or comparison.
- Do not write a chronology dump.
- Mention git state and low-level actions only as current state, not process
  noise.
- Don't invent shit. Use what actually happened in the session, not hypothetical
  imaginary counterfactuals.
- Keep observations, interpretations, uncertainty, and suggestions
  distinguishable. Do not turn uncertainty into a neat story.
- after sessions, reflect on what was necessary for success, and what was wasted effort
- report a blameless post-mortem in chat, don't follow-up with high-risk actions
- brainstorm, triage and present potential repo changes that affect future agents positively

Here is a default shape:

1. **Session Summary**.
2. **What Made It Productive**.
3. **What Went Badly**.
4. **Suggestions**.
5. **Current Take**.
