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

## Layer Boundaries

Keep these surfaces separate:

- Codebase infrastructure: durable repo entry points, command docs, data
  contracts, and method routing. These live mainly in `../README.md`,
  `../produce/README.md`, `../prepare/README.md`, and `../methods/README.md`.
- Workflow orchestration and coordination state: operational conventions for
  parallel sys-datascience sessions and subagents, plus planning beliefs,
  launch state, and topic maps used to coordinate that workflow. This layer
  lives in `workflow-orchestration.md`, `prompt-templates.md`,
  `research-ledger.md`, `next-session-candidates.md`, and `topics/*.md`. These
  files record planning beliefs by reference; experiment packets remain the
  source truth for generated metrics and packet-local interpretation.
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

- `workflow-orchestration.md`: roles, read paths, packet lifecycle, and update
  fanout for parallel sessions.
- `research-ledger.md`: recent global belief state with evidence/update traces,
  open discriminators, and prioritization-relevant uncertainty.
- `next-session-candidates.md`: compact spawn/rescope/stop board.
- `workflow-design-rationale.md`: why this orchestration pattern is predicted
  to work, what it is optimizing, and which failure modes it addresses.
- `process-learnings.md`: reusable incidents, Jörn critiques, and
  prompt/workflow updates.
- `prompt-templates.md`: reusable prompt shapes. These are templates, not
  complete prompts.
- `topics/`: topic-level research ledgers and packet seeds.

## Start Here

All sys-datascience research sessions should read this file first, then
`workflow-orchestration.md`. The role-specific next reads are listed there.

Use `next-session-candidates.md` for current launch status, `research-ledger.md`
for current cross-topic beliefs, and `topics/*.md` for local hypotheses,
evidence traces, and packet seeds.

Navigation is not a separate layer. It is a quality requirement for each
surface: future agents should be able to find the relevant file, understand
what use it is optimized for, and follow links to source truth without
reconstructing this chat.
