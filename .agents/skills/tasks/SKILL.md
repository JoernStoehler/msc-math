---
name: tasks
description: Use when Codex writes, updates, reviews, or derives a fresh-session prompt from repo task files under `tasks/`. Covers durable task context, task status, source-truth links, Jörn review gates, useful-progress/done criteria, constraints that keep Main usable for parallel agents, and preserving task context in prompts. Do not use for merely reading a task file unless you will change, review, or turn its context into a new task or session prompt.
---

# Tasks

Task files are durable context and routing material for future main sessions.
They should make the next agent understand what matters, what is trusted, what
is uncertain, and what is reserved for Jörn without needing to reconstruct the
chat.

This skill is about content, not formatting. Do not impose a template. Use any
structure that makes the task clear, checkable, and easy for future agents to
act on.

Use `tasks/README.md` for task-bundle conventions. This skill adds checks for
the context future agents need; it does not replace the task-bundle
conventions.

## When Writing Or Updating A Task

Make sure a future agent can recover the relevant parts of this list. Omit
items that are genuinely irrelevant, but do not omit an item just because it is
awkward to articulate.

- why this matters for thesis success
- what decision or deliverable the task supports
- what is source truth
- what is provisional, stale, scratch, or only a guess
- what must not be edited or decided by the agent
- what Jörn must review or decide
- what counts as useful progress
- what counts as done
- what would make the task harmful or not worth doing
- what checks or reviews are meaningful
- what constraints keep Main usable for other agents working in parallel
- what context a fresh-session prompt must preserve

## Updating Existing Task Files

Preserve the file's local organization unless it is actively harmful. Prefer
small, targeted edits over global reshaping.

When recording new information, separate:

- observed facts, such as commands run, files changed, commits merged, or
  review findings
- Jörn decisions and their scope
- agent interpretation or prediction
- remaining uncertainty and what would resolve it

Do not silently upgrade a guess into task steering. If a claim affects what a
future agent may edit, skip, trust, or report as done, attach the source or say
why the source is missing.

## Status And Done Claims

Task status should help future agents choose work without false confidence.

- Mark work done only when the relevant check or review gate has passed.
- If only a first slice is done, say what slice and what remains.
- If Jörn approval is required, do not replace it with agent judgment.
- If a task is no longer worth doing, record why and what made it obsolete.
- If the task is blocked, name the blocker and the cheapest useful next move.

## Fresh-Session Prompts From Task Files

When writing a first user message for Jörn to paste into a fresh main Codex
session, do not compress away the task context.

The prompt should preserve enough information for the new agent to know:

- why the task matters
- which task file or section is source context
- which inputs are trusted and which are scratch or preliminary
- what must not be edited or decided
- what "safe" means for this task, especially for Main, parallel agents, or
  harness material
- what output is expected
- what review criteria apply
- when the new agent should ask Jörn before continuing

Prefer pointing to a task file as the source of context over recreating the
context in chat. If the task file does not contain enough context for a safe
first-session prompt, update the task file first or tell Jörn what context is
missing.

## Reviews

When reviewing a task file or a prompt derived from one, prioritize blockers
that would make a future agent waste Jörn's time, edit the wrong thing, trust
scratch material, miss a Jörn gate, or leave Main harder for parallel agents to
work from.

Do not judge task files by visual symmetry. Judge whether a future agent can
act correctly from the available information.
