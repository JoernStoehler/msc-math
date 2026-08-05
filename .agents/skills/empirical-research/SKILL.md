---
name: empirical-research
description: "Use for empirical, computational, or experiment-supported research: questions, ideas, and frontier assessment; experiment planning, execution, review, and interpretation; experiment packets/files, data, provenance, and evidence promotion; research-agent or integration-branch coordination; and research-workflow improvement. Skip proof work without an empirical surface, publication-facing thesis writing or design, ordinary Rust maintenance, and LICCA commands."
---

# Empirical Research

This skill separates binding conventions, accepted evidence facts, and optional
suggestions so advice does not silently become authority.

- Sections labeled **convention** are binding shared interfaces. An owner may
  challenge one, but must coordinate before acting incompatibly.
- **Accepted evidence facts** are current source truth. Surface contradictory
  evidence; do not silently weaken them.
- **Suggestions** promote considerations to attention. They do not grant
  authority, require an architecture, or justify rejecting a sound plan.

Read `references/experiment-packets.md` when creating, moving, splitting,
joining, or documenting an experiment packet, or when deciding whether
experiment code should be copied or shared. The reference preserves
repo-specific situation, search, provenance, and dependency reasoning; it does
not decide physical placement without the local files and purpose.

## Coordination Conventions

These are binding shared interfaces. A worktree does not waive ownership or
review boundaries.

### Roles

The **portfolio owner** compares lines and allocates shared attention/resources.
It does not manage line agents, relay routine communication, or merge line
branches.

The **research-line lead** owns questions, hypotheses, idea search, cheap direct
investigations, proposal assessment, interpretation, and continue/pivot/stop
recommendations. It owns the line integration branch/worktree. It does not
manage executors/reviewers; it sends selected experiments to the line
orchestrator. It may use bounded scientific scouts when independent thinking or
source investigation repays the handoff, under the same fire-and-await rule.

The **line orchestrator** turns selected proposals into assignments, manages
executors/reviewers, records cost and state, routes repairs, and merges accepted
child branches during handoff. It does not choose research questions, invent
successor experiments, interpret results, or compare lines.

An **executor** owns a selected implementation/run and technical self-review.
It may return early when a premise, simpler route, or interpretation defect
makes the proposal poor. It does not silently change the question or promote a
scientific conclusion.

An **independent reviewer** checks a named transition. It does not expand scope
or repair work unless separately assigned that ownership.

### Fire And Await

Delegated work is fire-and-await. Do not inspect an agent's active commands or
poll internal state. Interact only for an agent question/blocker, a missed
declared return, an external resource symptom, or changed authority/scope.
Parallel work must have nonconflicting ownership.

### Branch And Integration Ownership

Each persistent line uses an integration worktree/branch from current Main and
records active ownership. Editing executors/reviewers use child branches; they
do not edit Main or another owner's branch.

The orchestrator merges only reviewed, in-scope commits during an explicit
lead-quiescent handoff. Only one agent writes the integration worktree at a
time. Scientific conflicts return to the lead. The lead owns the final account
and clean branch. Main still requires its normal review/Jörn gates; the
portfolio owner is not a merge relay.

The lead may execute and self-check a tiny standard investigation directly.
That does not combine the lead and orchestrator roles.

### Decision And Communication Returns

The research ledger is the shared interface. The lead records scientific
state; the orchestrator records assignments, measured cost, artifact/branch
pointers, technical verdicts, and unresolved gates. Decision-bearing facts
return there.

Ask Jörn from the owning line when mathematical expertise, thesis value,
private context, or elevated access changes the decision. Ask for the unresolved
quantity, not a composite `should` judgment. Cross-line cruxes go through the
portfolio owner.

## Research Ledger Convention

Give every item admitted to shared/current research state a stable ID: question,
observation, hypothesis, experiment idea/proposal, assessment, result, or
decision. Ephemeral thoughts and disposable checks need no ID unless their
result changes shared state.

Do not collapse different kinds into one ontology. Retain only what another
agent needs to understand, challenge, or use the item:

- ID, kind, topic, provenance, disposition, and related IDs;
- sources and reasoning;
- for assessed proposals: question served, outcome-conditioned updates, and
  cost;
- for results: direct observation, technical/review status, and proposal link.

A materially changed item gets a new ID linked by `supersedes` or
`derived_from`; it does not inherit the old item's premises, value, review, or
authority. Clarification or disposition changes may edit the same item.

Keep current records under Git near the research line or experiment material
whose question makes them interpretable. Topic, method, implementation,
consumer, and lifecycle relations do not each need another copy of the record.
Prune items that no longer help current agents; Git retains history. Do not
load an append-only archive into every session.

Keep a compact frontier view with:

- priority questions, active alternatives, and topic gaps;
- value/cost assessments and observed prediction errors;
- unresolved gates;
- stopped/rejected work and reasons.

Views do not replace the source evidence they point to. Evidence remains in the
producer artifacts, code, data, proofs, and reviewed interpretation that
establish it.
The orchestrator records assignments, actual cost, pointers, and technical
disposition. The lead records interpretation, belief/value changes, new ideas,
and successor proposals. Separate useful and defective parts.

Before handoff/closure, ensure loss of active sessions or scratch would not
erase material state. Session UUIDs can recover early work but do not replace
durable promoted evidence.

## Plan And Review Convention

This is the binding quality gate from an idea to consequential, irreversible,
or nonstandard execution. It does not prescribe code/report form.

Before material implementation, record enough for independent review:

- question and downstream decision;
- measured object, population/source, selection/target timing, controls, and
  comparison;
- material outcomes and their belief/decision updates;
- serious alternatives, staged cost, and the smallest useful observation;
- stop/return conditions and deferred scope;
- review contract: important failure modes, controls, claim boundaries, and
  what would resolve the proposal.

Value needs an explicit update path, not “this is about X.”

Obtain independent scrutiny before consequential, irreversible, or nonstandard
implementation. Review question fit, outcome interpretability, cheaper routes,
target/selection leakage, cost, controls, and claim support. Return accept,
bounded repair, or reject.

Before honoring a saved launch, stop, phase, or readiness decision, check
current instructions and Jörn/Kai/higher-owner facts. Surface contradictions;
packet readiness does not close a broader phase.

For a claim-bearing transition, also check that required sources are recoverable,
target-derived/post-target fields and prohibited claims are explicit, known
controls are not presented as independent discoveries, and whether a relevant
negative control can catch the likely failure.

Cheap standard/disposable checks may use owner self-review. If scope crosses
into retained infrastructure, target exposure, multi-agent execution, or a
claim-bearing result, stop and review the new transition.

Design the failure detector before building. Run an expected-red baseline only
when it tests the detector or provides a useful control. Review does not itself
authorize target exposure, resources, or successors.

Give the executor the proposal, review contract, sources, resource/ownership
boundaries, and return condition. Transfer purpose; leave form open unless a
real constraint fixes it.

The executor self-reviews and returns recoverable work. Independent result
review checks failures that would invalidate intended use. Findings give
evidence and consequence; repair is separate ownership. The lead interprets
after technical plausibility and records successors as new ledger items.

Small standard low-risk work may combine executor/reviewer roles when handoff
cost exceeds expected error reduction. Do not create review as ritual.

## Accepted Evidence Facts

- Artifact identity comes from code/input/artifact relationships, hashes, or
  reviewed verification—not timestamps, labels, maps, or prose alone.
- A smoke/plumbing check supports only the path it tested. A known positive
  control is not a discovery.
- Association/ranking, post-target diagnostics, frozen validation, independent
  candidate proposal, mechanism evidence, proof, and population generalization
  are different evidence roles.
- Technical/provenance validity and scientific interpretation can pass or fail
  independently.
- Review supports its named transition. Readiness is for a named consumer/use,
  not abstract completeness or stakeholder acceptance.

## Evidence-Handling Conventions

- Detailed metrics remain in generated artifacts and their producers.
  Regenerate rather than hand-edit or duplicate metric rows. Identify
  unexpected tracked changes before evidence use.
- Preserve the command, inputs, parameters, seeds/selection rules,
  non-obvious dependencies, and comparison contract needed for the intended
  claim or rerun. Required sources must be recoverable, not only untracked or
  absolute local paths.
- A producer run must not fall through from a failed build to a stale binary.
  Concurrent producers use separate outputs; merge only validated results.
- Before irreversible target evaluation, freeze the actual evaluator, source,
  dependencies, and inputs in a recoverable state. Material changes require a
  narrow recheck before exposure.
- Keep durable evidence and interpretation near the experiment question and
  comparison contract that give them meaning. Record other consumers and
  affected implementations as relations; do not relocate evidence merely to
  classify it by method, subject, or status. Publication-facing assets route
  through `$thesis`.

## Suggestions

The following are attention cues, not requirements or a checklist.

### Brainstorming

- State the live question and who would use its answer.
- Generate alternatives across objects/regimes, data sources, and methods.
- Include descriptive, predictive, falsification, mechanism, counterexample,
  proof-discovery, and confusing-observation routes where relevant.
- Run cheap source/data checks that improve the ideas.
- Look for omitted families and overconcentration in one family.
- Consider a fresh independent view.
- Estimate the marginal value of 5–30 more minutes of idea search versus
  investigating the current best ideas.
- Keep cheap redundancy when it improves success or interpretation.

### Assessing Research Questions And Experiments

- Separate present evidence from the value of resolving the question.
- List material positive, negative, confusing, and inconclusive outcomes.
- For each outcome, state the belief/decision update and its thesis or option
  value.
- Compare serious alternatives, including a standard/KISS route and a smaller
  first observation.
- Estimate staged costs, including likely repair and Jörn attention.
- Mark uncertainties/cruxes that affect cost, outcome probability, or value.
- Use probabilities, intervals, or distributions when they change priority;
  avoid false precision.
- Choose the smallest observation that distinguishes valuable outcomes.
- Defer polish/infrastructure without deferring validity or needed provenance.

### Investigation And Delegation

- Try the cheapest source/query/script/timing check that could settle or reshape
  the uncertainty.
- Keep seconds/minutes standard work local when handoff costs more.
- Do not durably record a disposable check unless it changes shared state.
- Delegate when iteration, context volume, independence, parallelism, or long
  output repays handoff and review.
- Prefer a standard method and thin end-to-end observation when adequate.
- Use custom infrastructure only when the question or measured repeated cost
  justifies it.
- In prompts, separate hard boundaries, downstream needs, current suggestions,
  and choices left to the executor.
- Judge model/effort/decomposition by usable output, defects, repair, and cost.

### Interpretation

- State the direct observation, source, and conditions.
- Separate hypotheses, assumptions, inferences, and competing explanations.
- State what changed in beliefs and what did not.
- Bound claims by population, selection, denominator, uncertainty, and
  generalization where relevant.
- Distinguish strongest, representative, clean, and diagnostic examples.
- Preserve useful parts of mixed or tainted work without laundering bad claims.
- Update failed routes, open questions, assessments, and successor ideas.

### Investigation Displays

- Name the question/decision and likely misleading reading.
- Use prose for short facts, tables for exact comparisons, and plots for shape.
- Expose population, denominator, selection, transforms, and missingness.
- Distinguish observations, fits, controls, thresholds, and selected examples.
- Say why displayed rows/examples were chosen.
- Keep scratch displays cheap; promote a producer only when later work needs it.

## Improving This Workflow

Read this section only for deliberate process diagnosis, testing, or skill
edits. Do not instrument routine experiments merely to populate process
records. The role/process design is a tested default, not a claim of optimality.
Change it through an explicit bounded trial and owner coordination, not silent
drift.

Treat an episode as evidence about the configured system:

```text
(task, role split, context, prompt, model/effort, tools, authority, oversight)
  -> (behavior, artifact, defects, repair, cost, downstream usability)
```

Distinguish missing discovery/routing/context from bad advice, failed
application, configuration limits, portfolio opportunity cost, and process
burden. Separate prevention, recovery, scientific salvage, and final object
value. Agent self-estimates are hypotheses; check measured time, product
equivalence, elementary upper/lower bounds, and competing explanations before
making a causal or savings claim.

Prefer real research transitions and downstream use. When testing unprimed
behavior, preserve exact prompt, raw output/artifact, reviewer verdict, and
parent interpretation separately. Use fresh agents and negative controls when
independence matters. Review semantic equivalence of an intervention contrast
before scaling matched probes; otherwise organization, wording, authority, and
content changes are confounded.

Use the weakest intervention likely to address the observed mechanism. Test
both a representative task and a case where the new advice should stay dormant.
One pass supports only that configured case.

Prune or replace obsolete process knowledge instead of appending episode
history. Keep model-specific results, raw telemetry, and detailed evaluations
in their empirical/session owners. Use `$harness-engineering` for durable instruction
design and behavior-evaluation integrity. Keep a skill diff separate and obtain
Jörn's exact review before Main merge.

## Adjacent Skills

- Use `$paper-conventions` when work consults source papers under `papers/` or
  checks published numbering.
- Use `$subagent-prompting` when a fresh bounded assignment needs non-obvious
  context, ownership, or a return contract after its outcome and delegation
  reason are selected.
- Use `$rust` when experiment code engages project-specific mathematical,
  numerical, observability, performance, or reusable-crate contracts.
- Use `$licca` before writing any command Jörn should run on, to, or from
  LICCA.
- Use `$thesis` when the main consumer is publication-facing prose, proof,
  figure/table design, or integrated thesis review. Investigation evidence
  becoming a thesis asset is a new consumer transition.
- Use `$harness-engineering` for durable skill/AGENTS/custom-agent design and
  behavior evaluation. This skill owns empirical outcome measures, not a
  duplicate general harness protocol.
