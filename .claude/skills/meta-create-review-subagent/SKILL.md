---
name: meta-create-review-subagent
description: Workflow for creating a new review subagent — a specialized workflow that verifies a set of conventions. Load when you need to add a new review concern, create a new review checklist, or modify how reviews are structured. For general workflow creation, see meta-create-workflow. For the review methodology itself, see the review skill.
---

# Creating Review Subagents

A specialized form of `meta-create-workflow` for creating review subagents — workflows that verify whether a set of conventions is met.

## Related skills

- `meta-create-workflow` — general workflow creation (review subagents are a specialization)
- `meta-foundations` — conceptual foundation
- `review` — the review methodology that review subagents follow

## Architecture

Reviews separate four concerns (see `meta-foundations/references/decision-records.md` for why):

1. **What's correct** → convention skills (one canonical source per topic)
2. **How to detect violations** → checklist reference docs (one per review concern, in `review/references/`)
3. **How to do a review** → the `review` skill (sequential checklist methodology, output format, phase ordering)
4. **What tools/model a reviewer gets** → the `review` agent definition (minimal — just capabilities)

A "review subagent" is a generic agent that gets: the review agent definition (capabilities), the review skill (methodology), and a specific checklist (concern).

## Workflow: adding a new review concern

### 1. Identify the convention set

What conventions should this review check? They should already exist in a convention skill. If they don't, create the conventions first (see `meta-create-conventions`).

### 2. Write the checklist

Create a new file in `review/references/checklist-<concern>.md`. The checklist should:

- **Reference the convention skill** — don't restate the conventions. "Check that doc comments follow `rust-conventions` §Doc comments" not "Check that doc comments start with a verb."
- **List specific detection rules** — what to look for, what patterns indicate violations. Be concrete: "grep for `TODO` without a ticket reference" not "check for incomplete items."
- **Order items by severity** — most important violations first.
- **Include examples** of violations and correct versions where the rule is non-obvious.

### 3. Register the checklist

Add the new checklist to the `review` skill's list of available checklists, so the parent agent knows it exists when spawning reviews.

### 4. Test by use

Spawn a review subagent with the new checklist on a file you know has violations. Check that it finds them. Also check that it doesn't flag correct code as violations.

## Key principles

- **Checklists reference conventions, not restate them.** If a convention changes, only the convention skill needs updating. The checklist says "check X" where X is defined elsewhere.
- **One concern per checklist.** A checklist for "Rust style" should not also check "Rust correctness." The parent agent composes reviews by spawning multiple focused subagents.
- **Sequential methodology.** The review skill teaches the methodology: work through items one at a time, record findings immediately. This is what makes reviews reliable — not agent specialization.
