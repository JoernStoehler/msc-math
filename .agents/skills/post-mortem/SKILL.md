---
name: post-mortem
description: End-of-session reflection workflow. Run at Jörn's request or after a session with significant friction, mistakes, or wasted time. Produces actionable findings (feedback/ entries, convention changes, decision records) — not just observations.
user-invocable: true
---

# Post-Mortem

Runs in main context (needs conversation history).

## Core questions — answer for every session

1. **Friction** — What slowed you down? Name the specific file, tool, or missing information.
2. **Unclear instructions** — What was confusing in `AGENTS.md`, skills, or agent prompts?
3. **Missing context** — What information wasn't provided but was needed?
4. **Jörn's time** — Where did Jörn spend time? Could agents have done it instead?
5. **What worked well** — What should be preserved or expanded?
6. **Suggested changes** — Specific, actionable improvements.

## Process checks — report only items that apply

1. Agent splitting needed? Multi-responsibility agent failed to cover all checks?
2. Fabrications slipped through to Jörn?
3. Iterated in front of user instead of delegating to subagents?
4. False attribution of mathematical results?
5. Assumed Jörn read something he may not have?
6. Regression test candidate? Concrete input→output pair worth preserving?

## Output

Persist to matching `feedback/` file (rules.md, skills.md, agents.md, output-style.md). Don't fix procedural files directly — a future `/update-workflow` session acts on feedback with Jörn. A postmortem that produces zero repo changes is fine if nothing actionable emerged.
