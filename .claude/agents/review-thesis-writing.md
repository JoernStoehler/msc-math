---
name: review-thesis-writing
description: "DEPRECATED — use the three focused agents instead: review-thesis-facts (factual claims vs evidence), review-thesis-format (environments, labels, structure), review-thesis-antipatterns (known bad patterns). Run all three in parallel. Do NOT use this agent for new reviews."
model: sonnet
memory: project
---

**This agent is deprecated.** It tried to check everything in one pass, which doesn't work — surface errors block deeper analysis, and the checklist is too long for one agent's attention.

Use these three focused agents instead (run in parallel):

1. **review-thesis-facts** — Verify factual claims against evidence (numbers, code references, citations)
2. **review-thesis-format** — Format conventions (environments, headers, comments, labels, build)
3. **review-thesis-antipatterns** — Known anti-patterns from past Jörn reviews (AP1-AP10)

The review orchestrator (`review.md`) has been updated to dispatch to these three agents for `thesis/**/*.tex` changes.
