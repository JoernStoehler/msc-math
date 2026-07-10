# Sys-Datascience Agent Memory And Expansion Plan

Use: agent-facing orientation for the full sys-datascience thesis slice. This
file records the interpretation and planning layer that future agents would
otherwise have to recover from git history, Codex session JSONL logs, code,
data, and scattered reasoning breadcrumbs.

This is not thesis prose, not a generated-metric ledger, and not source truth
for artifact rows. Source truth for numbers remains in the producer/prepare
artifacts, method packet READMEs, generated summaries, and code. This file
should stay useful even if concrete implementation details are later deleted,
regenerated, or reimplemented.

First source reads when checking this file:

- `README.md` for the active random/product scope and retained data flow;
- `produce/README.md` for producer contracts and production parameters;
- `prepare/README.md` and `feature-space-coverage-ledger.md` for the invariant
  feature contract;
- `methods/README.md`,
  `methods/trusted-random-product-closure-summary.md`, and
  `methods/trusted-random-product-method-dispositions.md` for current method
  packets and gates;
- `coordination/README.md` and `coordination/topics/method-surface-expansion.md`
  for workflow state and current method-surface planning;
- `FACTSHEET.md` for Jörn-confirmed thesis-scope constraints.

## Current State

The retained random/product pipeline has produced a useful first wave, not
closure of the data-science thesis slice.

Source-backed retained-pipeline facts:

- `produce/` owns random/product row generation and expensive cached
  polytope/capacity payloads.
- `prepare/` owns invariant feature computation and retained tables.
- `methods/` owns method packets over the prepared tables.
- The current retained table contains random polytopes and random
  Lagrangian-product polytopes only. It does not cover arbitrary random
  polytope models, arbitrary height ranges, arbitrary facet/product ranges, or
  independent seeds by itself.
- The active method-facing feature contract is invariant-only: combinatorial
  invariants and symplectic two-face area summaries normalized by
  `sqrt(volume)`, with source/provenance metadata kept as controls.
- Current retained method packets record no trusted retained-table `sys > 1`
  row and no validated generated-candidate proposer.

Current interpretation:

- The retained wave is enough for a bounded fallback claim about the named
  retained random/product pipeline.
- It is not enough for a broad claim that the standard data-science toolbox was
  exhausted or that black-box data science as a whole failed to produce
  mathematical progress.
- Real in-table structure exists, especially around symplectic two-face area
  summaries. That structure is useful as evidence and as a source of future
  hypotheses, but it is not a validated proposer or a mechanism theorem.

The current bounded claim is therefore:

> The retained random/product method table records no new source of `sys > 1`
> examples and no validated candidate-proposer among the named current method
> packets.

Do not silently strengthen this to:

- no `sys > 1` examples exist in random polytopes;
- arbitrary random distributions were covered;
- standard data science was exhausted;
- high-tail probabilities are calibrated;
- ridge-area features explain the mechanism;
- a candidate-proposer exists.

## Why More Data Science Is Expected

The thesis scope treats the search/data-science result as a major content area.
`FACTSHEET.md` says not to weaken method coverage to merely representative
families when standard repertoire coverage is feasible. The current retained
wave covers several ordinary methods, but it does not exhaust the standard
data-science repertoire and should not be presented as if it did.

Future work should therefore treat the retained wave as a strong starting
point and fallback, not the intended end state. The bounded fallback may close
the full sys-datascience slice only after an explicit Jörn or thesis-control
decision downgrades the intended claim strength.

Current 2026-07-08 workflow state: the first parent-loop control pass, P1/P3
read-only design packets, P2 execution/review/synthesis, P5 mechanism/tail
thesis-use audit, bounded retained-table source-map/writeup, P4
generated-candidate closure, and high-complexity producer compute-packet
preparation have run. The next material branch is either thesis prose from the
bounded source map if that fallback story is accepted, or smoke-first LICCA
execution/review of the high-complexity producer packet if broader
producer-axis evidence is still worth buying. The compute packet is not
evidence until it has validated smoke, production, prepare, fingerprint, and
review outputs.

The main missing work is not one specific algorithm. It is a broader campaign
that decides, method family by method family and producer axis by producer
axis, whether to run, reject, park, or use as future work. That campaign should
optimize for thesis success, not for making the launch board smaller.

## What "Done" Should Mean

The sys-datascience slice is thesis-done only when all retained thesis wording
has matching evidence and caveats. Depending on the final wording, this can
take one of three forms.

### Bounded fallback done

This is the minimum coherent result.

- The thesis says the claim is about the retained random/product table and the
  named method packets.
- The main text or appendix gives the row families, feature contract, method
  families, and verdicts.
- It states that no retained `sys > 1` row and no validated candidate-proposer
  were found.
- It states that in-table structure was found but did not validate a proposer.
- It explicitly leaves broader random distributions, broad standard-method
  exhaustion, calibrated density, and mechanism claims outside the result.

### Standard-method search done

This is stronger and closer to the intended thesis ambition.

- A broad method-surface audit has been done against the standard
  data-science/statistics/optimization repertoire.
- Each relevant method family is either run, rejected with a concrete interface
  reason, parked with a thesis-scope reason, or delegated to future work.
- Missing ordinary baselines such as lasso/elastic-net, boosting,
  high-tail classification, and feature-family ablations are either executed or
  explicitly ruled out as not changing retained wording.
- The final method table supports wording such as "standard methods we tried"
  without implying impossible universal exhaustion.

### Strong search story done

This requires separate evidence beyond the current retained wave.

- Candidate generation is tested on selected-before-`sys` unevaluated rows,
  preferably with independent seeds or bucket-matched validation.
- Producer/distribution variants are designed and, if promoted, run: height
  intervals, facet ranges, product side ranges, independent seeds, and
  alternative random polytope models.
- Tail or rare-event statements are either omitted or supported by
  model-sensitive caveats and backtests strong enough for the wording.
- Any mechanism claim names the measured object, the association operation,
  and a falsifier. For ridge-area features, this means distinguishing
  bucket/combinatorial proxy, magnitude signal, concentration signal, and
  extreme-tail Goodharting before writing causal prose.

## Expansion Axes

Use these axes to plan work. Do not launch a packet merely because an axis
exists. Launch when the packet changes a named thesis sentence, a search
decision, or a future packet choice.

### Method Coverage

Question: have enough ordinary methods been tried or explicitly dispositioned
to support the intended wording?

Current retained methods cover target scans, EDA/tails, scalar associations,
projection/clustering/anomaly diagnostics, supervised ranking, tail-rule
mining, and scalar generated-candidate filtering.

Likely next methods if stronger coverage is needed:

- lasso/elastic-net and related shrinkage models;
- gradient boosting;
- high-tail classification;
- feature-family ablation;
- simple generalized additive or spline-style checks if interactions look
  important;
- explicit rejection notes for methods whose interface is absent or low-value:
  neural nets, Gaussian processes, density models, kernel methods, SVM/kNN,
  Bayesian posterior methods, reinforcement learning, time series, and
  multi-fidelity methods unless a concrete new interface appears.

### Candidate Generation

Question: can a method choose unevaluated candidates before their `sys` values
are computed?

Current boundary:

- In-table prediction and scalar association do not validate proposers.
- The 100k scalar generated-candidate packet is real boundary evidence:
  selection was before `sys`, enrichment was present, but no `sys > 1` row was
  found.

Likely next work if reopened:

- freeze one two-feature or concentration-rule design before evaluation;
- use bucket-matched validation;
- use independent seeds when the rule is frozen;
- stop if the rule only restates ridge magnitude, works in one bucket only, or
  does not improve on the scalar boundary.

### Producer And Distribution Coverage

Question: is the claim only about the retained producer contract, or about a
larger random-search surface?

Current retained producer contract is narrow and should be named. Stronger
random-distribution wording needs a design pass before new production:

- independent same-contract seeds;
- broader or shifted height intervals;
- larger generic facet counts;
- larger product side ranges;
- alternative random polytope models;
- space-filling or Latin-hypercube designs over generator parameters.

Current prepared producer execution: the P3-promoted high-complexity bucket
extension has a compute packet at
`coordination/high-complexity-producer-compute-packet-2026-07-08.md`. It covers
generic `F=10,11,12` and product `4x6,5x5,5x6,6x6` at the current height
interval. It must be treated as unrun until LICCA outputs are validated and
reviewed.

### Mechanism And Geometry

Question: do the observed features teach a mathematical idea, or are they only
useful predictors?

Current best mechanism seed is symplectic two-face area. The useful compressed
idea is not the Rust implementation; it is:

- compute symplectic areas of primal two-faces;
- normalize by `sqrt(volume)` to remove scale;
- summarize the distribution across two-faces;
- compare low magnitude and concentration summaries against `sys`, while
  controlling for product bucket and combinatorial structure.

Future agents should preserve formulas, invariance contracts, and measured
objects. They do not need to preserve every implementation detail if code is
reimplemented.

### Tail And Rare Events

Question: should the thesis say anything about expected hit rates or sample
sizes?

Current answer: probably not as calibrated claims. Zero positives is a table
fact. Tail extrapolations and posterior/survival estimates are model-sensitive.
Use them for scale-up decisions or caveated future-work discussion unless a
stronger tail packet is deliberately promoted.

## Source Compression Model

Different evidence layers have different durability. Future agents should know
what to keep and what can be rederived.

### Keep As Source Truth

- current code when it defines the actual producer, feature, or method
  contract;
- generated artifacts when they are the direct output of a retained packet;
- method packet READMEs when they state current purpose, command, artifact
  paths, interpretation, and caveats;
- git history when recovering deleted packet context or old designs;
- Codex session JSONL logs when reconstructing why a decision was made and no
  durable note captured it.

### Compress Into Notes

Compress source material when the durable value is the idea, not the current
  code:

- formulas and invariance contracts;
- sampling contracts and producer axes;
- feature-family definitions;
- leakage and validation guards;
- failed method interfaces and why they failed;
- conjectures, falsifiers, and reopen triggers;
- packet prompt patterns that prevented known agent failure modes.

### Recompute Instead Of Preserving

Prefer regeneration when the artifact is cheap enough or when exact freshness
matters:

- prepared tables from producer files;
- method summaries from retained tables;
- plots and compact reports;
- smoke outputs and development slices.

Do not preserve stale generated numbers in prose. Link to artifacts or generate
compact reports.

## Labor Types, Context Bundles, And Orchestration

Use labor types instead of role names when planning sys-datascience work. A
single GPT-5.5 context window can combine several labor types when they share
the same loaded files, concepts, tools, and quality criteria. Conversely, the
same named "role" can be useless if the context window lacks the object-level
state needed for the decision.

Useful object-level labor types include:

- implementing and running an experiment packet;
- polishing code, data, artifacts, and reasoning into a usable handoff;
- interpreting results and updating the current research model;
- planning and prioritizing experiments;
- reviewing code, artifacts, provenance, and claim boundaries;
- designing schemas, feature contracts, and producer interfaces;
- translating observations into mathematical language or proof targets;
- writing thesis-facing claim ladders once the evidence target is clear.

These are not disjoint. An experiment executor is often already prepared to
write the immediate provenance and interpretation note. A reviewer is often
already prepared to provision the trusted parts of a packet. A schema designer
usually overlaps with software engineering and experiment design. A figure or
inspection view is usually part of experiment execution or provisioning unless
the hard question is which projection or comparison should be inspected.

Orchestration is an overlay on top of object-level labor, not a standalone
research mode. It splits work, assigns worktrees or subagents, tracks active
state, sets review gates, and merges outputs back into the research state.
Ordinary coding or proof packets do not need awareness that subagents exist;
they need a clear objective, source files, expected artifacts, stopping
conditions, and review standard. The orchestration layer owns why that packet
exists, who owns it, when it should stop, and how the result is integrated.

When describing future work, prefer a labor mix such as
`{method-surface planning, orchestration overlay, provisioning}` or
`{experiment execution, local software engineering, inspection views,
immediate provenance}` over a person-like label such as "orchestrator" or
"packet builder".

## Pre-Mortem Priorities

When hardening an autonomous sys-datascience parent loop, pre-empt failures in
this order. The ordering is impact-weighted by Jörn attention cost and by how
far downstream the failure becomes visible. Session-log checks on 2026-07-08
support the top items: the July 1 datascience parent made a false time/progress
claim before checking logs, began with a too-narrow ridge-area priority after a
short surface pass, and needed repeated Jörn correction of proposer definitions
and experiment strength. The July 7/8 scoping session overclaimed the story as
"black-box data science did not produce..." before immediately narrowing it
under questioning. The July 6 resume-packet repair came from an agent using an
execution cursor as a substitute for the governing resumption model.

### 1. Unsupported Progress Or Answer Claims

First unrecovered error: the parent says a research question is answered, a
story is coherent, a packet is complete, a run tested the intended thing, or a
surface has been explored enough before it can state the actual answer and
evidence at the original question's level.

Observed shapes:

- claiming broad "black-box data science" failure from named retained-pipeline
  packets;
- claiming substantial surface-search time without checking the session log;
- describing an experiment as the intended proposer strength when it only
  tested a weaker acceptance tail;
- folding under the first "what is the answer?" follow-up and replacing the
  question with an easier one.

Guard:

- before any "answered", "complete", "enough", "validated", or "done" claim,
  write the original question, the literal answer, source evidence, and what
  remains unanswered;
- if the answer cannot be stated without changing the question, say it is not
  answered;
- log/time/progress claims must be checked from logs or stated as guesses;
- final/status messages for incomplete work must say "not complete" and give
  the autonomous continuation or blocker.

### 2. Premature Stop With Weak Scope Or Continuation

First unrecovered error: the agent stops with a status report or git-heavy
summary while the assigned scope is incomplete, or gives only a contingent next
action that does not recover the active scope and plan.

Observed shapes:

- reporting a useful precursor as if the live task were handled;
- saying only "next I will read file X" or another execution cursor when the
  session needs the planning horizon;
- leaving Jörn to ask for a resume packet, scope recall, plan, and then
  "continue".

Guard:

- if the assigned scope is incomplete and no Jörn-only crux/blocker exists,
  continue autonomously instead of finalizing;
- if stopping because the turn must end, state control state, active scope,
  current status, default continuation, and why the current next step belongs
  to that scope;
- do not include git state unless it changes review, merge, or safety.

### 3. Shallow Search And Premature Planning Convergence

First unrecovered error: the parent spends too little effort on idea generation
or approach assessment, then runs the best idea found in the first small sample
of thought as if marginal planning had gone negative.

Observed shapes:

- a short surface pass initially prioritized ridge-area follow-up too narrowly
  before generator breadth, method-surface expansion, tail data, and omitted
  options were forced into view;
- after saying the surface was explored, later prompts immediately produced
  better ideas or necessary distinctions;
- "unknown" buckets were left too unexamined to support prioritization.

Guard:

- for research/planning packets, maintain an explicit estimate of the value of
  another planning pass and update it from idea yield;
- require broad longlists before narrowing: method families, producer axes,
  proposer routes, mechanism hypotheses, proof/theory bridges, and
  omitted/residual unknowns;
- run at least one alternative framing or scout when the first recommendation
  is dominated by one recently visible signal;
- do not claim planning convergence unless recent marginal search produced no
  better options and the remaining unknowns are named.

### 4. Work Selection Without A Claim Or Learning Target

First unrecovered error: the parent drains `next-session-candidates.md`, picks
the easiest runnable packet, or asks subagents to explore a plausible topic
before identifying what thesis sentence, decision, or model uncertainty the
work can change.

Guard:

- every packet prompt must name the thesis sentence, decision, or claim-ladder
  cell it can affect;
- every packet prompt must include assumptions, allowed pivots, stop rules, and
  review standard;
- no broad model/producer run before the parent names one method or producer
  axis and why it beats alternatives.

### 5. Delegation Without Parent Synthesis

First unrecovered error: subagents produce locally valid outputs, but the
parent accepts summaries and launches follow-ups without integrating results
into the global research model.

Guard:

- after each execution/review wave, update or explicitly preserve the claim
  ladder, source map, open discriminators, and next packet ranking;
- do not launch a new wave merely because child outputs contain plausible next
  actions;
- the parent owns thesis-scope inference. Subagents own bounded artifacts.

### 6. Review Bypass Or Weak Review Target

First unrecovered error: executor artifacts become accepted evidence before a
fresh check of code, data, leakage, provenance, experiment strength, and claim
boundaries.

Guard:

- no packet result may strengthen thesis wording until reviewed;
- reviewer output must classify downstream use: thesis evidence,
  exploration-only, smoke/plumbing, future-work seed, revise, park, or discard;
- review the experiment actually run against the experiment intended, not only
  whether code executed;
- mixed packets must separate trusted artifacts from tainted interpretation.

### 7. Method-Surface Undersampling

First unrecovered error: the parent runs several familiar methods and then
treats that as standard-repertoire coverage.

Guard:

- before standard-method wording, run a method-surface longlist/disposition
  pass;
- each relevant family is run, rejected with an interface/value reason, parked
  with a thesis-scope reason, or made future work;
- do not let the current retained method packets stand in for all standard
  data science.

### 8. State And Provenance Loss

First unrecovered error: work is real but not reusable because commands,
artifact paths, data contracts, active ownership, or interpretation boundaries
are missing.

Guard:

- accepted packets need a minimal durable handoff: command, inputs, artifacts,
  source-truth path, interpretation boundary, and reopen trigger;
- update active-work and coordination state when ownership changes;
- prefer regenerable artifacts plus source maps over hand-maintained numeric
  prose.

### 9. Bad Jörn Interaction Timing

First unrecovered error: the parent either interrupts Jörn for low-value
permission questions, asks should-questions instead of crux questions, or asks
for estimates that accessible local evidence could have improved first.

Guard:

- inspect accessible repo/log evidence before asking Jörn for predictions about
  agent behavior or current repo state;
- ask Jörn only for cruxes that affect packet ranking, claim strength, or
  mathematical/thesis taste;
- phrase the question around assumptions and predicted consequences, not
  "should I continue?";
- if the parent can cheaply cover both branches with agent work, do that and
  record the assumption instead of asking.

## Parent-Loop Active Checklist

Use this checklist in the launch prompt or active control file for any
autonomous parent loop. It is the behavioral version of the pre-mortem above.

### Before Claiming Progress Or Closure

Before writing or implying "answered", "complete", "done", "validated",
"enough", "supports", "the story is coherent", "we can write this now", or
equivalent progress/closure wording in a final, status update, parent summary,
or packet review, fill this check:

```text
Original question:
Literal answer:
Source evidence:
Boundary / unanswered remainder:
Downstream use allowed:
```

If the literal answer cannot be stated without changing the question, the
question is not answered. Say what is known and continue or re-scope.
The fields must be specific enough that a reviewer can tell whether the
original question was answered without rereading the transcript. "Progress",
"useful work", "see notes", or a bare file path are not literal answers or
source evidence.

### Before Stopping Or Sending A Final Status

Classify the state:

```text
complete / milestone-complete / loud-failure / awaiting-Jörn-crux / blocked
```

If the assigned scope is incomplete and there is no Jörn-only crux or external
blocker, do not stop at a status report. Continue autonomously. If a checkpoint
is necessary, it is not a stop state unless it satisfies `milestone-complete`
or `loud-failure`: state the active scope, current status, default
continuation, and why the next step belongs to the scope. Git state is included
only when it changes review, merge, or safety.

### Before Ending A Planning Phase

For research or experiment-planning work, record why more planning is no longer
the best next use of agent time. A usable planning phase should include:

- a longlist across method families, producer axes, proposer routes,
  mechanism hypotheses, theory/proof bridges, inspection routes, and controls;
- omitted or residual unknowns;
- at least one alternative framing pass, such as "what would make the current
  best plan too local?";
- a planning-yield note: whether the last pass found a new candidate that
  changed the ranking or opened a new axis.

Do not claim planning convergence while the last pass is still producing
better ideas or important missing axes. A longlist alone is not convergence;
the planning-yield note must say whether the last pass changed top-ranked
options, exposed a new axis, or left residual unknowns that still matter.

### Before Launching A Packet Or Subagent

Write a packet card:

```text
Target claim / decision / model uncertainty:
Why this packet beats the best currently parked or conditional alternative:
Assumptions:
Outcome branches and allowed pivots:
Stop condition:
Review target and review standard:
```

Do not launch a packet merely because it is runnable or listed in
`next-session-candidates.md`. If no concrete alternative is named, the packet
is not yet justified.

### After A Packet Wave

Before launching follow-ups from child outputs, update or explicitly preserve:

- claim ladder;
- source map;
- open discriminators;
- next packet ranking;
- parked/rejected list.

Child outputs may contain plausible next actions. They are not launch
authorization until the parent integrates them into the global research state.
Follow-up packet cards must cite the updated synthesis; do not decide the
follow-up first and write synthesis afterward.

### Before Promoting Evidence To Thesis Wording

Require review that checks:

- code and command correctness;
- generated artifacts and provenance;
- leakage and evaluation protocol;
- intended experiment versus actual experiment;
- claim boundary and downstream use.

The review verdict should classify the result as thesis evidence,
exploration-only, smoke/plumbing, future-work seed, revise, park, or discard.
Reject or redo reviews that do not check intended experiment versus actual
experiment and do not give a downstream-use verdict.

### Before Treating This Material As Hardened

This note is background. A parent-agent launch prompt or active control file
must include the short checklist above directly; do not assume a future agent
will keep a long note active after reading it once.

Before treating this workflow material as hardened, run at least one fresh-agent
probe. Ask the probe what it would do first, what would count as done, which
checklist gates it would apply before launching work, and what it found
confusing, irrelevant, or too weak. Update the material from the probe's actual
misreads.

Current workflow-hardening evidence: the 2026-07-08 parent-loop probe wave in
`coordination/workflow-evaluations/2026-07-08-parent-loop-probes.md` found and
then re-checked the main known silent-failure bypasses. The focused re-probe
resolved those named bypasses. This is evidence for launch readiness on those
failure modes, not proof that the full autonomous workflow is validated.

## Workflow-Hardening Evaluation Packets

Some high-value packets have zero direct merge value for the thesis text,
experiment data, or mathematical result. They are still legitimate work when
their target is to test whether the agent-facing material causes future agents
to choose, execute, review, or stop correctly.

Useful evaluation-only packets include:

- mock fresh-agent runs on controlled workspaces and controlled source
  material;
- selected tasks whose only purpose is to expose whether the material supports
  the intended read, first action, done condition, and gates;
- A/B tests of launch prompts or active control files;
- replay tests where a known past failure point is rerun against the updated
  material;
- adversarial bypass reviews that ask how a compliant agent could still fail.

For these packets, success is not a mergeable thesis artifact. Success is a
diagnosis about material behavior: what the fresh agent read, inferred, missed,
misread, skipped, or treated as done. Preserve the task prompt, controlled
workspace state, raw response, parent interpretation, and resulting material
change or no-change verdict separately. Do not count these packets as evidence
for sys-datascience mathematical claims or method-table closure.

## Planning Rules For Future Agents

1. Name the thesis sentence before launching a broad packet.
2. Separate method coverage, candidate generation, distribution coverage,
   mechanism, and tail modeling. A packet may inform several axes, but it
   should say which axis owns the decision.
3. Treat research ideas as heavy-tailed. Short brainstorming is usually too
   weak. Use longlists, method-family recall checklists, and multiple scouts
   when choosing the next high-value direction.
4. Make assumptions explicit and attach update triggers. If a packet result
   falsifies an assumption, pivot or return to the parent scope; do not patch a
   plan whose premise failed.
5. Prefer planned contingencies over repeated Jörn interruptions. A packet
   prompt should say what to do if the primary assumption fails, what fallback
   route is allowed, and when to stop.
6. Ask Jörn for crux judgments, not permission. Useful cruxes are about thesis
   value, mathematical taste, or correlations between assumptions that agents
   are likely to misestimate.
7. Do not use `next-session-candidates.md` as a queue. Before launching work,
   state the milestone, why this beats alternatives, and what active ownership
   will be recorded.
8. Record current beliefs with source pointers. Beliefs may be uncertain or
   speculative if their uncertainty and allowed downstream use are visible.
9. Keep workflow-hardening evaluation packets separate from thesis-progress
   packets. They can have high value for the agent material while licensing no
   sys-datascience result.

## Immediate Next Useful Work

The next high-value work is a broader method-surface and expansion-plan pass,
not direct thesis prose and not another arbitrary model run.

Recommended first packet:

- labor mix: method-surface planning, research-model synthesis, and enough
  orchestration overlay to spawn follow-up packets cleanly;
- output: a source-linked expansion plan that separates bounded fallback,
  standard-method search, and strong search story;
- required content: longlist of method families and producer axes; run/park/
  reject status; estimated thesis value; expected cost; assumptions; cruxes;
  packet prompts for the highest-value next executors;
- review standard: the plan must make clear which wording becomes licensed if
  the listed packets succeed, and which current claims remain unsupported.

After that, spawn packet executors only for the highest-value cells in the plan.
The likely first execution candidates are the tiny retained-table
standard-method baseline, one generated-candidate rule-freezing packet, and one
distribution-design scout. Their relative order should be chosen by the exact
thesis sentence the slice is trying to support.
