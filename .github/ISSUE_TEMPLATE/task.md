---
name: Task
about: A candidate task believed to constitute progress toward the thesis (#1)
---

<!--
LIFECYCLE — full description with examples: docs/references/issue-lifecycle.md

  created (label: draft)
    Issue captured from a spark — an idea that came up during triage, a session,
    or any conversation. Most sections may be empty or rough. That's fine.
    The issue exists so the idea isn't lost.

  → refined via edits (label: draft)
    Over one or more triage sessions, sections get filled in and sharpened.
    Open questions get resolved and their answers flow into other sections.
    Facts and claims get verified. Scope gets negotiated with Jörn.

  → approved (label: approved)
    Jörn reads the issue and labels it "approved". This means:
    the goal is worth pursuing, the scope is appropriate, the deliverable
    is clear, and the open questions are resolved (or non-blocking).
    From this point, the issue IS the prompt for an agent session.

  → agent session (label: in-progress)
    Agent + Jörn discuss scope, plan, implement, review, push.
    Agent works on a feature branch. See root CLAUDE.md for session workflow.

  → PR + merge
    Jörn creates PR, reviews, merges to main.

  → closed (label: done)
    Issue closed. Follow-up ideas captured as new issues during triage.

AUTHORING GUIDELINES

  These are the known failure modes when writing issues. Guard against them:

  - Unclear or ambiguous wording. Don't sacrifice clarity for brevity.
    Prefer an extra sentence over a vague word. If a term could mean
    two things, say which one you mean.

  - Misleading confidence signals. If something is unreviewed or uncertain,
    mark it explicitly — and do so for EVERY such item, not just some.
    Labeling one item "unreviewed" implies the others ARE reviewed.

  - False facts. Don't claim relationships between concepts unless verified.
    Don't state that X determines Y when actually X, Y, and Z all contribute.
    Don't call one thing a "specialization" of another unless it literally is.

  - Misrepresenting process. Don't omit stages the session agent will go
    through. Don't claim something is approved when it isn't. Represent
    the actual state of decisions accurately.

  - Over-constraining implementation. Don't prescribe file names, file counts,
    section structures, or other decisions the agent can trivially make
    during implementation. Constrain only what has external consequences
    (API surfaces, conventions from CLAUDE.md, mathematical correctness).
-->

## Goal

<!-- What this task achieves for the thesis. Short — a sentence or two.
     This is the "what" at the highest level. -->

## Background

<!-- Domain knowledge a reader needs for this issue to make sense.
     Concepts, definitions, theorems, prior work. Link to papers, files,
     issues for deeper reading — don't repeat their content here.
     This section answers: "what do I need to UNDERSTAND?" -->

## Context

<!-- Why this task constitutes progress toward the thesis (#1).
     How it connects to parent issues and the dependency graph.
     What completing this unblocks or improves.
     Desired benefits and anticipated risks to the project.
     This section answers: "why should we DO this?" -->

## Deliverable

<!-- What the agent produces. Describe the substance, not the form —
     the agent decides files, structure, and commits.
     What is the interaction surface with the rest of the project?
     What downstream agents or code will consume this deliverable? -->

## Scope

<!-- Agreed-upon boundaries. What's in, what's out, and why.
     Each exclusion should have a reason (out of scope because X,
     deferred to issue #Y, not needed because Z).
     This section prevents scope creep during implementation. -->

## Sources

<!-- Where to find information needed for implementation.
     Papers, code, specs, existing implementations.
     Flag trustworthiness: is this a reviewed paper, an untrusted
     archaeology file, a known-broken implementation?
     (Background = read to understand. Sources = read to implement.) -->

## Acceptance criteria

<!-- How to know the task is done. Each criterion should be:
     - Measurable: an agent or Jörn can unambiguously check it
     - Motivated: why this criterion matters (what goes wrong without it)

     Two kinds:
     External — the deliverable serves the project. Downstream agents
       and code can consume it. It integrates correctly.
     Internal — quality bar for long-term project health. Clarity,
       correctness, review requirements per crates/CLAUDE.md.
       E.g.: "proofs are drafts until Jörn reviews" is an internal
       criterion motivated by agents' inability to verify math reliably. -->

## Notes

<!-- Preliminary findings from scoping and triage sessions.
     Known risks inside scope. Suggested sub-issues worth considering.
     Anything discovered during refinement that the session agent
     should know about. Can be empty for fresh issues. -->

## Open questions

<!-- Uncertainties, decisions that need Jörn's input, dependencies
     on other issues. Resolve via edits as answers emerge — move
     answers into the appropriate section above.
     Once this section is empty and Jörn approves, the task is
     ready for assignment. -->
