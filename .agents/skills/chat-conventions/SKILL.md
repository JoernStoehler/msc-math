---
name: chat-conventions
description: Use when Codex communicates with Jörn in this repo for planning, investigation reports, status updates, postmortems, feedback responses, process proposals, documentation proposals, or decisions about agent behavior. Applies to chat output, not durable repo prose.
---

# Chat Conventions

This skill applies to chat output for Jörn. It does not govern durable repo
prose.

## Instrumental Objectives

A Codex message to Jörn should satisfy this set of objectives:

1. Jörn can tell what role the message plays: answer, status update,
   investigation report, proposal, draft for review, correction, or decision
   request.
2. Jörn can tell what the agent knows, what the agent inferred, and what remains
   uncertain.
3. Jörn can tell whether the agent needs anything from him, and if so which
   decision or review judgment is needed.
4. Jörn can check or correct the message without reconstructing hidden reasoning
   from earlier chat.
5. Jörn can refer to numbered items in follow-up messages when the topic has
   multiple failures, options, findings, or decisions.
6. Jörn's attention is spent on decisions he owns, not on lookup, inventory,
   comparison, or initial drafting that an agent can do.

## Answer Shape

Start with the requested answer, current status, artifact, or decision request.
Put reasoning after that only when it names a source, command, artifact,
decision criterion, or uncertainty that Jörn needs for the next decision.

Do not present early reasoning as a result. If you cannot yet state the answer,
status, artifact, or decision request, keep iterating in `/tmp` or say what is
still unfinished.

## Investigation Reports

Start with one status when reporting an investigation:

1. `Found directly`
2. `Reconstructed indirectly`
3. `Not found`
4. `Still uncertain`

Then separate:

1. source facts;
2. inferences;
3. uncertainty;
4. recommendation, if any.

Do not make Jörn infer whether you found the target, reconstructed it, or are
still uncertain.

## Questions

Before asking Jörn, do the repo-checkable work yourself.

Ask Jörn for decisions he owns:

1. thesis scope;
2. mathematical judgment;
3. advisor-facing framing;
4. taste or presentation preference;
5. external-world actions;
6. design pivots.

Do not ask Jörn to do grep, inventory, comparison, source lookup, or initial
drafting that an agent can do.

Define key terms before asking a question that depends on them.

## Chat Format

Avoid Markdown tables in chat because the TUI does not render them readably.

Use numbered lists for observed failures, options, review items, or any list
Jörn may refer to later. Use bullet lists when item numbers will not be used in
follow-up messages.

Use chat for math that fits in at most three displayed equations. For longer
math or planning with more than three candidates, create a PDF or `/tmp`
artifact before asking for review.

## Corrections

After Jörn corrects an answer, give the corrected answer first.

Keep mistake explanations to one or two sentences. Include them only when they
change the next action.

Do not send an initial repair when the draft is missing status, owner, evidence,
or decision request, or mainly explains your reasoning process.

## Process Changes

Before proposing `AGENTS.md`, skill, process, or documentation changes, separate:

1. observed source facts;
2. inferences;
3. uncertainty;
4. diagnosed failure modes;
5. candidate fixes;
6. recommendation.

Compare at least two fixes that would address the diagnosed failure before
recommending one. Recommend a durable repo change only when the same failure has
already happened more than once or is likely to happen again under the current
instructions.

When using a scratch artifact, report the current best result and remaining
decision, not just that the file was updated.

Do not present early reasoning as a result. If you cannot yet state the answer,
status, artifact, or decision request, keep iterating in `/tmp` or say what is
still unfinished.

## Durable Files

Use chat and `/tmp` artifacts to get Jörn's feedback before changing durable
repo files.

Durable repo files should contain source-traceable decisions, current project
state, and guidance for future agents. Chat and `/tmp` should contain
exploratory drafts and feedback exchanges.
