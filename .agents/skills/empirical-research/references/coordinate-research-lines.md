# Coordinate Research Lines

Read this for multi-packet research lines and cross-line experiment portfolios.
It is a provisional, reversible operating model, not a mandatory lifecycle or
a fixed model-routing table. Adapt it when the line is small, source-obvious,
or coupled in a different way.

Contents: [ownership](#ownership), [line loop](#line-loop),
[review and resource control](#review-and-resource-control),
[model and effort](#model-and-effort-choice),
[working surfaces](#working-surfaces), and [Jörn cruxes](#jörn-cruxes).

## Ownership

The portfolio owner compares whole-project value and cost across lines. It
chooses which lines exist, their initial resource envelopes, shared-interface
ownership, and merge order. It does not micromanage packet execution or retain
every line's scientific model.

A persistent research-line lead owns the local question, hypotheses, cheap
investigations, packet sequence, delegation, scientific interpretation, and
continue/pivot/stop recommendation. Use an independent session for this role
when the line is likely to span several packets or discussions and retained
context is valuable. Use a forked subagent when parent context is essential and
the work is bounded; use a fresh subagent when independence is the point.

Executors own concrete implementation or evidence production. Reviewers own a
named transition gate. The lead integrates their products and remains
responsible for what the line learns; do not turn executor or reviewer
summaries into scientific conclusions without inspecting the evidence they
cite.

Prefer flatter work when one agent can cheaply hold the question, execute, and
check it. Delegation is useful when labor is separable, parallelism matters, or
freshness targets a plausible correlated error. It is not evidence of progress
by itself.

## Line Loop

1. Reconstruct the current source state and downstream thesis decision. Run
   cheap queries or small scripts when they settle premises needed for planning.
2. Maintain competing explanations and serious alternative lines. Compare the
   next material unit of work against those alternatives before local momentum
   becomes an implicit priority rule.
3. State the smallest discriminating packet, predicted outcomes, interpretation
   consequences, cost envelope, and stopping condition. Numbers are useful when
   they transmit a real estimate; do not manufacture precision.
4. Decide which labor the lead should retain and which can be delegated. Give
   each delegate exact owner files, downstream decision, resource envelope, and
   return condition. Let a lead manage its own bounded team when repeated local
   handoffs would otherwise return to the portfolio owner.
5. Use smoke-scale evidence before an expensive retained run when it can check
   input identity, build/binary identity, output schema, provenance fields, and
   the reviewer entry path. Disposable probes need only the checks relevant to
   their immediate inference.
6. Review at the transition where an undetected error would be costly. Repair
   specific findings, check those repairs cheaply, and repeat the whole review
   only when the repair can invalidate unrelated parts.
7. Have the line lead interpret technically plausible evidence, update local
   beliefs, and decide whether another packet beats returning the line to the
   portfolio comparison.
8. Promote durable scientific results to their experiment, code, proof, or
   thesis-support owner. Return cross-line implications to the portfolio owner.

Return a line to portfolio comparison when evidence changes its possible
thesis value, another line becomes a live competitor, the resource envelope
must expand, the question drifts, a shared interface is needed, a Jörn crux
remains, or a claim is ready for promotion. Feasible follow-up work and local
momentum are not continuation reasons by themselves.

## Review And Resource Control

Keep technical/provenance validity and mathematical/domain interpretation
separate when either can pass while the other fails. Combine them when the
same evidence makes that cheaper without crowding one out. A fresh reviewer is
especially useful for transition errors that executor self-review is likely to
share; it is not automatically worth its coordination cost for a small packet.

For a bounded review, name the likely failure dimensions and give a wall or
effort envelope. Add domain-specific contradictions, invariants, and negative
controls to generic readiness criteria. Review success means the requested
transition is supported, not that the reviewer exhausted every possible
defect.

Gate expensive producer commands on a successful current build or identified
binary in the same command path. Declare full-run and repair-run budgets before
execution. A failed build must not fall through to a stale binary. Do not live-
monitor an owned child merely because it is running; inspect after its declared
deadline, an explicit blocker, or an externally observed resource symptom.

## Model And Effort Choice

Treat routing as evidence about the configured system—task, decomposition,
context, prompt, model/effort, tools, and oversight—not as a model leaderboard.
Read `allocate-model-effort.md` when this choice is material. Choose the
cheapest plausible configuration for the actual labor, then use naturally
occurring failures, repair cost, and downstream usability to update the choice.
Do not benchmark adjacent levels without a live routing decision.

## Working Surfaces

Place knowledge by its consumer and lifetime:

- reusable experiment-process guidance belongs in this skill;
- durable observations, code, data, proofs, provenance, and interpretation
  belong in their scientific owner in the repository;
- temporary hypotheses, experiment ideas, value estimates, line status, and
  delegation state may live in a unique `/tmp/` line directory while active;
- material prepared for Jörn belongs under `/tmp/joern/` unless it is already a
  durable scientific artifact;
- raw agent behavior remains in Codex session JSONL; preserve exact prompts,
  raw outputs, and parent interpretation separately during a deliberate
  workflow evaluation.

Do not use `/tmp/joern/` as the line's only working memory or make Jörn-facing
material double as an agent coordination protocol. Promote any temporary fact
whose loss would impair later scientific use before closing the line.

Peers may read another line's status but should not edit it. Exchange exact
source pointers and decision-relevant updates rather than forwarding raw
packet output. The portfolio owner resolves shared-resource conflicts and
whole-thesis value questions; line leads coordinate direct scientific
dependencies when no portfolio choice is involved.

## Jörn Cruxes

Complete locally accessible feasibility, evidence, outcome, and cost reasoning
first. Ask Jörn for a mathematical fact, naturalness judgment, thesis-value
quantity, private context, or elevated resource only when plausible answers
change the line decision. State that dependency explicitly. Do not replace a
set of unresolved quantities with a composite yes/no `should` question.
A launch plan may identify the kind of stakeholder dependency likely to arise;
do not formulate it as a current request while accessible source inspection or
cheap evidence is still expected to change the alternatives or needed context.
