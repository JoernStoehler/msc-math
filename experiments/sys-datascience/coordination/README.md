# Sys-Datascience Coordination

Use: active coordination surface for finishing the sys-datascience thesis
slice through parallel agent work. This folder is for agents, not for generated
metrics and not for thesis-facing prose. It helps sessions recover the current
research map, write packet prompts, review packet scope, and decide what
sessions to spawn, stop, split, merge, or rescope.

The broad thesis-slice question is:

> Can data-science methods help find a `sys > 1` example, rule out plausible
> search routes, or produce thesis-useful geometric/statistical explanations of
> why `sys` behaves as observed?

Most concrete evidence currently comes from retained/generated random-product
work. Treat that as the current evidence base, not as a scope limit on the full
datascience thesis slice.

Workflow status: launch-candidate parent-loop process with 2026-07-08 probe
coverage for the main known silent-failure bypasses. It is not a validated
guarantee. When launch behavior conflicts with `process-learnings.md`, the
incident analysis wins and this folder should be updated.

Current working milestone, 2026-07-08: build a broader sys-datascience
expansion plan and first execution/review wave around method coverage,
candidate generation, producer/distribution coverage, mechanism
interpretation, and tail/rare-event evidence. P1/P3 read-only design packets,
P2 execution/review/synthesis, P5 mechanism/tail thesis-use audit, bounded
retained-table source-map/writeup, P4 generated-candidate closure, and the
high-complexity producer compute packet are complete as local artifacts. The
bounded retained method-table claim remains the minimum fallback story, not
automatic full-slice closure. The only currently prepared execution candidate
that needs external resources is the smoke-first LICCA high-complexity producer
extension. Source pointers:
`../agent-memory-and-expansion-plan.md`,
`autonomous-parent-loop.md`,
`first-wave-p1-p3-results-2026-07-08.md`,
`p2-synthesis-2026-07-08.md`,
`p5-mechanism-tail-thesis-use-audit-2026-07-08.md`,
`bounded-retained-table-source-map-writeup-2026-07-08.md`,
`p4-generated-candidate-closure-2026-07-08.md`,
`high-complexity-producer-compute-packet-2026-07-08.md`,
`../../../thesis/central-claim-control.md`, and
`../methods/trusted-random-product-closure-summary.md`.

## Layer Boundaries

Keep these surfaces separate:

- Codebase infrastructure: durable repo entry points, command docs, data
  contracts, and method routing. These live mainly in `../README.md`,
  `../produce/README.md`, `../prepare/README.md`, and `../methods/README.md`.
- Workflow orchestration and coordination state: operational conventions for
  parallel sys-datascience sessions and subagents, plus planning beliefs,
  launch state, and topic maps used to coordinate that workflow. This layer
  lives in `autonomous-parent-loop.md`, `workflow-orchestration.md`,
  `prompt-templates.md`, `research-ledger.md`, `next-session-candidates.md`,
  `parked-and-rejected.md`, `active-work.md`, and `topics/*.md`. These files
  record planning beliefs by reference; experiment packets remain the source
  truth for generated metrics and packet-local interpretation.
- Workflow meta: rationale, process evidence, rejected alternatives, and
  incident analysis that explain why the orchestration layer looks this way.
  This layer lives in `workflow-design-rationale.md` and
  `process-learnings.md`.
- Experiment packets: code, generated artifacts, plots, provenance,
  packet-local README interpretation, and reviewable outputs. These live in
  `../methods/`, `../produce/`, `../prepare/`, or another experiment owner.
  This folder links to them and records belief updates; it is not a second
  source of generated numbers.

## Files

- `autonomous-parent-loop.md`: active launch-control surface for a parent agent
  trying to finish, materially reduce, or loudly fail the full slice.
- `first-wave-design-2026-07-08.md`: output of the first parent-loop control
  pass, with claim ladder, longlist, packet cards, and default continuation for
  the next wave.
- `first-wave-p1-p3-results-2026-07-08.md`: synthesis of the first two
  read-only design packets; promotes P2 as current default and parks blind
  producer expansion.
- `p2-synthesis-2026-07-08.md`: parent synthesis after the P2 execution packet;
  makes P5 mechanism/tail thesis-use audit the current default.
- `p5-mechanism-tail-thesis-use-audit-2026-07-08.md`: source-map audit for
  thesis-safe mechanism and tail wording; makes bounded retained-table
  source-map/writeup the current default.
- `bounded-retained-table-source-map-writeup-2026-07-08.md`: source map and
  draft wording for the fallback retained random/product method-table story.
- `p4-generated-candidate-closure-2026-07-08.md`: generated-candidate proposer
  closure note; parks scalar-filter rescue unless a future owner freezes an
  independent two-feature validation plan.
- `high-complexity-producer-compute-packet-2026-07-08.md`: smoke-first LICCA
  compute packet for the P3 high-complexity generic/product bucket extension.
- `workflow-orchestration.md`: roles, read paths, packet lifecycle, and update
  fanout for parallel sessions.
- `research-ledger.md`: recent global belief state with evidence/update traces,
  open discriminators, and prioritization-relevant uncertainty.
- `next-session-candidates.md`: compact decision board for possible
  spawn/rescope/stop actions.
- `parked-and-rejected.md`: rediscovery surface for ideas not on the current
  decision board.
- `active-work.md`: short-lived registry for active sys-datascience sessions,
  branches, blocking cruxes, and review/merge state.
- `workflow-design-rationale.md`: why this orchestration pattern is predicted
  to work, what it is optimizing, and which failure modes it addresses.
- `process-learnings.md`: reusable incidents, Jörn critiques, and
  prompt/workflow updates.
- `workflow-evaluations/`: workflow-test prompts, raw-response pointers,
  parent interpretations, and material-change verdicts.
- `prompt-templates.md`: reusable prompt shapes. These are templates, not
  complete prompts.
- `topics/`: topic-level research ledgers and packet seeds.

## Start Here

All sys-datascience research sessions should read this file first, then
`autonomous-parent-loop.md` for parent-loop or launch-control work, and
`workflow-orchestration.md` for ordinary packet/topic coordination. The
role-specific next reads are listed there.

Use `next-session-candidates.md` for current session-decision status,
`parked-and-rejected.md` for rediscovery-only ideas, `research-ledger.md` for
current cross-topic beliefs, and `topics/*.md` for local hypotheses, evidence
traces, and packet seeds.

Navigation is not a separate layer. It is a quality requirement for each
surface: future agents should be able to find the relevant file, understand
what use it is optimized for, and follow links to source truth without
reconstructing this chat.
