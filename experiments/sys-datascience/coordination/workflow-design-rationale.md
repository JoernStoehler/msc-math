# Sys-Datascience Workflow Rationale

Use: this file explains why the current multi-session research workflow is
predicted to work, what costs it is trying to manage, and which failure modes
it is designed to avoid. It is optimized for agents who need to reason about
whether to follow, adapt, or replace the workflow. Update it when process
evidence changes, not after every packet.

Status: this is a working prediction, not a validated process guarantee. Use
`process-learnings.md`, fresh-agent review results, and future workflow-test
packets to revise it when observed behavior differs from the prediction.

## How To Read Process Claims

Keep four statuses distinct:

- `observed benefit`: useful behavior occurred, without necessarily identifying
  which process choice caused it or whether net cost was favorable;
- `observed burden`: a failure, repair, or coordination cost occurred, without
  necessarily measuring its size or avoidability;
- `provisional default`: the current best reversible choice under uncertainty;
- `speculative alternative/risk`: an untested option or predicted side effect
  that a local lead may adopt, reject, or test from richer task context.

Evidence sufficient for a reversible trial is weaker than evidence sufficient
for a recurring rule or causal claim. Do not turn a successful packet, a salient
failure, or agreement between agents reading the same summary into a validated
workflow law.

## Core Prediction

The sys-datascience slice is research under uncertainty, not a linear software
project. The useful order of work is not known in advance because the behavior
of `sys` is only partially understood. Good work therefore alternates between
idea generation, experiment execution, interpretation, review, and writeup
preparation. These are aspects of a loop, not sequential phases.

The workflow separates surface scouting, topic ownership, packet execution,
packet review, and interpretation review because these roles have different
costs and failure modes:

- surface scouting improves the option set and helps Jörn decide which sessions
  should exist;
- topic ownership preserves local working memory and hypotheses across packets;
- packet execution benefits from fresh context and a bounded definition of done;
- packet review catches code/provenance/claim defects before results affect
  later planning;
- interpretation review asks what belief updates follow after the packet is
  technically plausible.

## Why Parallel Topic Ownership

Research idea value and effort are expected to be heavy-tailed. Many ideas are
cheap and low value; a few may open larger search spaces or explain many
observations. Shallowly sampling many topics can miss such ideas because useful
insight often requires sustained reasoning within one local ontology. At the
same time, committing one long-running session to every possible topic creates
context dilution and makes opportunity-cost comparison harder.

The current compromise is:

- keep a global surface scout for breadth, missing-topic detection, and
  session-decision advice;
- create topic-owner sessions only when a seed looks worth sustained attention;
- let topic owners reason deeply and maintain their own surfaces;
- let packet executors work fresh and bounded when the topic owner has a
  concrete experiment;
- propagate only planning-relevant belief updates into the global ledger.

For a topic with several dependent stages or specialized child roles, the
current provisional default is a persistent hybrid research lead. The lead
owns local modeling, sequencing, integration, and its resumption surface. The
parent retains cross-topic priority, resource expansion, Jörn communication,
final interpretation/claims, and merge authority. This split is a best guess
supported by the 2026-07-12 session, not a measured optimum; later successful
lines were also more mature. Plausible alternatives include flatter execution,
fewer concurrent lines, stronger producer contracts, or a consolidated review
gate. A lead should adapt locally when one of those has lower coordination or
drift risk.

`workflow-orchestration.md` owns the exact return conditions. Their purpose is
to amortize local context without granting a line indefinite continuation:
local momentum and feasible follow-ups are not enough to consume more
portfolio budget.

## Portfolio Reasoning

The next locally useful action may not be the best use of the next bounded unit
of work. At material boundaries, compare active and plausible research lines
using quantities that should remain separate:

- possible thesis value and the outcome distribution;
- information value, including option value for later work;
- implementation and review cost;
- probability that the work is decisive at the intended claim strength;
- dependencies and effects on other lines;
- reproducibility, provenance burden, and reviewability.

Do not collapse evidence strength, hypothesis plausibility, potential value,
and experiment value into one label. A weakly supported hypothesis may be worth
a cheap discriminating test; a strong observation may have little thesis
value. Use a compact portfolio comparison when lines genuinely compete, not as
a fixed-time administrative ritual.

Before asking Jörn, estimate agent-accessible quantities and identify the exact
mathematical, stakeholder, taste, private-context, or elevated-access quantity
for which Jörn is unusually informative. State how plausible answers change
the decision. This is a cognitive tactic, not a demand that every crux be
artificially reduced to one independent scalar.

Overlap between agents is acceptable when it costs mostly agent time and may
catch missed ideas. Coordination and Jörn attention are more expensive, so the
workflow should make handoffs and review targets explicit.

## Beliefs And Evidence

The belief surfaces are not registries of finalized results. A claim belongs
in a topic file or the global ledger when it is useful for future reasoning and
has a trace to why it is currently believed, suspected, doubted, or blocked.

Useful epistemic labels include:

- `source-backed fact`: directly checkable in code, data, or generated output;
- `reviewed inference`: interpretation that has survived enough review to guide
  planning, with caveats;
- `live hypothesis`: plausible and useful to distinguish, not settled;
- `speculative seed`: weak evidence but potentially high value if true;
- `mixed/tainted`: some parts are useful, but some claim/artifact has a known
  defect;
- `currently low-value`: considered and not worth pursuing unless new evidence
  changes the opportunity cost.

Mixed packets should not be discarded wholesale. Separate usable facts from
tainted claims. Example: an earlier tail-hardening packet had useful
zero-positive and model-sensitivity artifacts, but its HKO-distance/flank
interpretation was tainted by a scale mismatch. The current clean
`../methods/tail-survival-1m-posterior/` packet keeps the usable tail material
and excludes that HKO-distance/flank section.

## Packet Boundary Updates

Experiments produce many local observations. Most should remain in the packet
artifacts or topic file. Propagating every intermediate detail globally creates
maintenance cost and stale claims. Propagate to the global ledger when the
update changes at least one of:

- which sessions should be spawned, stopped, split, merged, or rescoped;
- which packet should be run next;
- what thesis wording is plausible;
- which hypotheses other topic owners should condition on;
- which work is tainted, blocked, or no longer worth repeating.

Topic owners should update their topic files more often than the global ledger,
because their files also serve their own session resumption after context
compaction or deep dives.

A useful topic resumption surface distinguishes direct observations and their
measurement conditions from hypotheses, rival explanations, inferences,
current beliefs, proposed discriminators, predicted outcome branches, and
decision consequences. These are linked research objects, not homogeneous
fields that every note must fill. Choose a local representation that helps the
lead and its children reason; avoid building an ontology-maintenance project.

## Workflow-Test Packets

Some exploratory work is selected for information about the workflow or prompt
material, not for direct thesis value or direct value-of-information about
`sys`. These packets are allowed to use coherent but non-priority research
questions if that makes prompt/material failure modes easier to see. Their
research outputs may be discarded and redone under better materials. Their
durable result is the process evidence: what the agent understood, what it had
to infer, where it under-scoped, and which instructions caused or prevented
useful work.

Mark workflow-test packets explicitly in prompts and reports. Do not propagate
their research conclusions into topic beliefs unless a normal review says the
evidence is good enough for research use.

Use this visible header in workflow-test prompts and reports:

```text
Workflow-test: yes/no
Research conclusions may update beliefs: no unless later normal review
Process evidence to report:
```

## Subagents And Context

Packet executors usually start fresh. They need a strong prompt with the local
question, why it matters, source files, expected artifacts, review standard,
and stopping condition. They should report externally relevant conclusions,
changed files, commands, and risks. They should not report every internal
development trace unless it changes interpretation or future work.

Interpretation reviewers may benefit from forked or context-rich prompts when
the parent context contains important discussion. They should err toward
preserving relevant caveats and evidence traces rather than over-compressing
epistemics.

If a packet executor genuinely needs Jörn input, it should pause and ask its
parent. The parent asks Jörn, then resumes the executor. Most execution and
interpretation questions should be answerable from repo context and the packet
prompt.

## What Makes A Prompt Good

A packet prompt should include enough global context that local tradeoffs are
decidable. Examples:

- whether a method is meant for local-maxima discovery, generic `sys`
  maximization, or generated-candidate proposal;
- whether performance matters because the method will generate many candidates;
- whether evidence is in-table explanatory evidence or independent proposer
  evidence;
- what packet-local polish is useful versus scope expansion;
- what should be written for future agents to reinterpret the result.

Weak prompts cause agents to deliver internally consistent but unusable
packets, partial work with no clear definition of done, or code that optimizes
the wrong tradeoff.

## Review Standard

Review should ask:

- Does the packet answer the motivating question?
- Are generated artifacts reproducible and source-linked?
- Are claims separated by epistemic status?
- Are mixed/tainted results preserved without letting bad claims propagate?
- Is the packet worth merging, parking, rewriting, or discarding?
- Did the work update topic beliefs, global prioritization, or neither?
- In hindsight, was this packet worth launching relative to the best alternative
  available at launch time?
- Should similar packets be repeated, stopped, split, or rescoped?
- Which higher-level thesis milestone did this packet advance or fail to
  advance?

For small documentation-only changes, self-review can be enough. For code,
data, or thesis-relevant interpretation packets, use a reviewer when the cost
is justified by likely error reduction.

The 2026-07-12 cycle gave selected independent reviews realized diagnostic
value, but did not show that more review is always better or that its total
allocation was cost-optimal. Keep the risk-based promotion policy in
`workflow-orchestration.md`. Convert a caught defect into a producer assertion,
schema check, or reusable review question only when the check is cheap and the
defect class plausibly recurs. Do not create review telemetry unless review
cost itself becomes a live decision problem.

The retainability preflight in `workflow-orchestration.md` is a bounded trial,
not a universal gate. Its predicted benefit is asymmetric: a small smoke-scale
check may prevent an expensive rerun or unusable evidence. Its predicted harms
are premature schema commitment and friction during disposable exploration.
Leads should adapt the checks to anticipated retained-run and repair cost.

For parked packets, first review the artifact in its parked commit, then decide
whether current code needs to be compared or rerun. Useful read-only entry
commands are:

```bash
git ls-tree -r <commit> -- <packet-path>
git show <commit>:<packet-path>/README.md
git show <commit>:<packet-path>/<artifact-or-note>
```

Parked-packet review should separate source-backed facts, model-sensitive
inferences, tainted claims, missing provenance, and integration blockers. It
should say whether to merge, park, rewrite, cherry-pick, or discard, and which
topic/global surfaces should or should not be updated.
