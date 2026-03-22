---
name: meta-create-review-subagent
description: Workflow for creating a new review subagent — a specialized workflow that verifies conventions or detects error patterns. Load when you need to add a new review concern or modify how reviews are structured. For general workflow creation, see meta-create-workflow. For the review methodology itself, see the review skill.
---

# Creating Review Subagents

A specialized form of `meta-create-workflow` for creating review subagents.

## Related skills

- `meta-create-workflow` — general workflow creation (review subagents are a specialization)
- `meta-foundations` — conceptual foundation
- `review` — the review orchestration and methodology

## Two kinds of review

**Convention review** — verifies target state properties. The review agent loads ONE convention skill and checks each convention. The conventions ARE the review specification — no separate checklist needed.

**Error detection / proofreading** — scans for known error patterns. A dedicated agent with detection patterns inline. Used when the thing to check isn't a convention (e.g., "look for unargued claims in proofs"). Example: the `math-review` agent.

## When to create a new review concern

For convention review: if the conventions already exist in a skill, you just need to add a row to the review skill's spawn mapping table. No new agent or file needed — the generic `review` agent loads the convention skill.

For error detection: if the concern requires a different workflow, different model, or detection patterns that aren't conventions, create a dedicated agent definition with the patterns inline.

## Workflow: adding a convention review concern

1. Ensure the conventions exist in a skill (see `meta-create-conventions`)
2. Add the concern to the review skill's spawn mapping table
3. Test by spawning the review agent with that skill on a file you know has violations

## Workflow: adding an error detection agent

1. Identify the error patterns — what to look for, what indicates a problem
2. Write an agent definition with the patterns inline (see `meta-create-workflow/references/agent-format.md`)
3. Choose the right model (Opus for reasoning-heavy detection, Sonnet for mechanical checks)
4. Add the agent to the review skill's spawn mapping table
5. Test on a file with known errors — check it finds them and doesn't produce false positives

## Key principles

- **One concern per spawn.** Never bundle multiple concerns into one subagent — this produces 10% quality on 10 tasks instead of 100% on 1.
- **Convention skills are the review specification.** Don't create separate checklists that restate conventions. Detection patterns only exist inline in dedicated agents for non-convention concerns.
- **Sequential methodology.** Work through items one at a time, record findings immediately. Don't hold all items in memory.
