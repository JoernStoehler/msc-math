---
name: meta-feedback-processing
description: Workflow for processing Jörn's feedback during interactive sessions. Covers the generalization loop — fixing the instance, abstracting the error class, scanning for all instances, and recording durably. Load when receiving corrections or review feedback from Jörn.
---

# Feedback Processing

## When to load

During interactive sessions when Jörn flags an error or gives a correction. Not needed for autonomous subagent work.

## The generalization loop

When Jörn flags a specific error (e.g. "line 15 doesn't check theorem X's preconditions"):

1. **Fix the specific instance.** This is the minimum.

2. **Abstract the reasoning error.** Not "theorem X on line 15" but "applying results without verifying preconditions." The error class is about the *type of reasoning mistake*, not the specific content. It won't be the same theorem elsewhere — it will be a different result applied without checking its own preconditions.

3. **Scan for the same error class.** Search the current file and other files from this session. Look for the same *type* of reasoning error, not the same surface pattern. Dispatch subagents for large scopes.

4. **Fix all instances found.**

5. **Record the reasoning error in MEMORY.md.** Describe the error class with enough context that a future agent in a different session recognizes when it applies. Include: what the error looks like, why it's wrong, and how to check for it.

6. **Re-review.** Dispatch a subagent to re-read the deliverable through the lens of the feedback. "Given that I made error X in two places, what else did I miss?"

## Why this matters

Jörn's time is the scarcest resource. Every round of feedback he gives should prevent the *entire error class*, not just the one instance he pointed at. If Jörn has to flag the same type of mistake twice, the feedback loop is broken.

## Common failure modes

- **Fix the instance, stop.** Line 15 gets fixed, line 56 has the same reasoning error but the agent doesn't look.
- **Fix superficially.** Agent writes "the requirements can be checked" instead of actually checking them. Jörn then has to flag that the fix is empty, wasting another round.
- **Record the specific instance, not the class.** MEMORY.md says "don't forget theorem X preconditions on line 15" instead of "always verify preconditions before applying any result."
- **No re-review.** Agent fixes the flagged items and presents the deliverable again without re-reading it. Jörn finds new instances of the same error class.

## Two-speed knowledge recording

- **Fast (this session):** Record the reasoning error in MEMORY.md immediately. It may be rough — that's fine.
- **Slow (across sessions):** Periodically, stable MEMORY.md entries get migrated into permanent artifacts (CLAUDE.md, skills, review checklists). This is a separate maintenance task, not part of the feedback loop.
