# Sys-Datascience Autonomous Parent Loop

Use: launch-control surface for an autonomous parent agent that is trying to
finish, materially reduce, or loudly fail the sys-datascience thesis slice.
This file is for parent-loop control, not for generated metrics, method-packet
source truth, or thesis prose.

Status: launch candidate with 2026-07-08 probe coverage for the main known
silent-failure bypasses. Use this file together with
`../agent-memory-and-expansion-plan.md`, `workflow-orchestration.md`,
`research-ledger.md`, `next-session-candidates.md`, and the topic files. Update
this file when a workflow-test, fresh-agent probe, packet review, or Jörn
critique changes the parent-loop behavior. Probe evidence lives under
`workflow-evaluations/`.

Current first-wave state: `first-wave-design-2026-07-08.md`,
`first-wave-p1-p3-results-2026-07-08.md`, `p2-synthesis-2026-07-08.md`,
`p5-mechanism-tail-thesis-use-audit-2026-07-08.md`,
`bounded-retained-table-source-map-writeup-2026-07-08.md`,
`p4-generated-candidate-closure-2026-07-08.md`, and
`high-complexity-producer-compute-packet-2026-07-08.md`. P1/P3 read-only
design packets, P2 execution/review/synthesis, P5 audit, bounded source map,
P4 closure, and high-complexity compute-packet preparation have returned. The
current material branch is either thesis prose from the bounded source map or
LICCA execution/review of the prepared compute packet.

## Primary Objective

Run the sys-datascience slice until one of these outcomes is reached:

- the slice has source-mapped thesis wording whose evidence, caveats, and
  unsupported stronger claims are clear;
- a milestone has been completed that strongly reduces the remaining workflow
  difficulty, labor surface, or heterogeneity of open questions;
- the workflow fails loudly with enough preserved evidence to repair the
  process and restart without pretending the research slice is done.

The parent loop should prefer a loud, restartable failure over a plausible
deliverable whose claim boundary is wrong. A plausible but unsupported thesis
story is the worst outcome.

## Current Launch Milestone

Current default milestone after the 2026-07-08 workflow/scoping repair, P1/P3
synthesis, P2 synthesis, and P5 audit:

> Build a broader sys-datascience expansion plan and first execution/review
> wave around method coverage, candidate generation, producer/distribution
> coverage, mechanism interpretation, and tail/rare-event evidence.

This replaces the older default of immediately writing the bounded retained
method-table claim. The bounded claim remains a fallback and source-mapped
minimum story, not the intended closure of the whole slice. It may close the
full sys-datascience slice only after an explicit Jörn or thesis-control
decision downgrades the intended claim strength.

The first parent-loop design milestone has the required control artifacts when
it has:

- a claim ladder separating bounded fallback, standard-method search, and
  strong search story;
- a longlist/disposition over method families, producer axes, proposer routes,
  mechanism routes, tail evidence, and theory/proof bridges;
- ranked packet cards for the next execution/review wave;
- explicit parked/rejected alternatives and reopen triggers;
- review gates that prevent packet results from strengthening thesis wording
  before code/provenance/claim review;
- a clear stop/fail rule for the first wave.

P1/P3 completed the first design refinement, P2 completed the first
execution/review/synthesis loop, P5 completed the mechanism/tail wording audit,
the bounded source-map/writeup packet exists, P4 generated-candidate closure is
recorded, and the high-complexity producer compute packet is ready for
smoke-first LICCA execution. The live continuation milestone is now a route
choice between fallback thesis prose and external producer execution/review,
not another source-map packet. A plan or status report alone does not satisfy
the live milestone.

## Dominance Rule

When objectives conflict, use this order:

1. Prevent silent invalid claims or silently invalid thesis-facing artifacts.
2. Preserve enough state to restart or repair after failure.
3. Complete the strongest thesis-useful sys-datascience story justified by
   evidence and time.
4. Reduce future labor or heterogeneity when full completion is not yet
   reachable.
5. Minimize Jörn attention after accessible repo/log/subagent evidence has
   been used.

Do not optimize for looking locally productive, draining a launch board, or
producing a polished report when those conflict with the dominance rule.

## Parent-Owned State

The parent owns these objects. Subagents can propose edits or packet outputs,
but cannot decide that the slice is complete.

- claim ladder: what wording is licensed, not licensed, or conditional;
- source map: which packet/code/artifact supports each claim;
- open discriminator list: which unknowns change packet ranking or thesis
  wording;
- packet ranking: what should launch next and why it beats alternatives;
- parked/rejected list: ideas not active, with reopen triggers;
- workflow state: active work, failures, probe results, and process updates.

If the parent cannot state these objects after a wave, it has not synthesized
the wave yet.

## Loop States

### 0. Launch Or Recover

Read:

- `../README.md`;
- `../agent-memory-and-expansion-plan.md`;
- `README.md`;
- this file;
- `workflow-orchestration.md`;
- `research-ledger.md`;
- `next-session-candidates.md`;
- `topics/method-surface-expansion.md`;
- other topic files named by `next-session-candidates.md` or by the packet
  alternatives currently being compared.

Then write or update an `active-work.md` entry before launching substantial
work. The entry should name the milestone, branch/worktree, owned surfaces,
blocking crux if any, and review/merge state.

### First 30 Minutes

The parent's first local work should produce enough control state to prevent a
wrong launch:

1. State the current milestone in ordinary thesis terms.
2. Create or update `active-work.md`.
3. Draft the claim ladder: bounded fallback, standard-method search, and strong
   search story.
4. Build a longlist across the surface axes in the next section.
5. Record which longlist pass last changed the ranking or exposed a new axis.
6. Write packet cards only after the longlist and claim ladder are in view.

Do not execute a model, producer, or thesis-writing packet during this first
control pass.

### 1. Expand And Rank The Surface

Before execution, produce a surface pass that includes:

- method families;
- producer/distribution axes;
- generated-candidate proposer routes;
- mechanism/geometric interpretation routes;
- tail/rare-event evidence;
- theory/proof bridges;
- inspection/figure/report routes;
- controls and negative baselines.

Record a planning-yield note. If the last pass still found a better candidate,
new axis, or important missing distinction, do not claim convergence.

### 2. Design A Packet Wave

Write a packet card for each proposed execution, review, or workflow-test
packet:

```text
Packet id:
Workflow-test: yes/no
Target claim / decision / model uncertainty:
Why this beats the best parked or conditional alternative:
Owned files / worktree:
Inputs and source files:
Expected artifacts:
Assumptions:
Outcome branches and allowed pivots:
Stop condition:
Review target and review standard:
Downstream use if successful:
```

No packet launches merely because it is runnable. No packet launches unless at
least one concrete alternative has been considered. Execution packets also
need the exact thesis sentence or decision they can affect, the source artifact
or table they will read, the evaluation target, required protocol choices, and
the best rejected or parked alternative.

### 3. Execute Packets

Packet executors work in their own branch or worktree and own bounded files.
They should not need the whole global map. Their prompt must provide the
motivating question, source paths, expected artifacts, stopping conditions, and
review standard.

If a packet's primary assumption fails, the executor follows the packet's
allowed pivot or stops. It should not keep hot-patching a plan whose premise
failed.

### 4. Review Packets

Before packet results update thesis wording or global prioritization, review:

- intended experiment versus actual experiment;
- code and command correctness;
- artifact provenance and regeneration path;
- target leakage and evaluation protocol;
- source-data contract and stale cache risks;
- claim boundary and downstream use;
- whether trusted artifacts can be separated from tainted interpretation.

The review verdict must classify downstream use as one of:

- thesis evidence;
- exploration-only;
- smoke/plumbing;
- future-work seed;
- revise;
- park;
- discard.

Any `thesis evidence` classification, global belief update, or thesis-wording
strengthening requires a named review verdict artifact or clearly labeled
review section separate from executor output. Executor self-checks may support
the review, but they are not the independent review verdict.

### 5. Synthesize

After each wave, update or explicitly preserve:

- claim ladder;
- source map;
- open discriminators;
- next packet ranking;
- parked/rejected list;
- `active-work.md`;
- topic files or `research-ledger.md` if global beliefs changed;
- `process-learnings.md` if agent behavior or workflow material changed.

Do not launch child-suggested follow-ups before this synthesis. Child outputs
are evidence, not launch authorization.

### 6. Close Out Or Iterate

The parent may stop only in one of these states:

```text
complete / milestone-complete / loud-failure / awaiting-Jörn-crux / blocked
```

`complete` means the sys-datascience slice has thesis-ready wording and review
coverage matching the wording.

`milestone-complete` means the completed work materially reduces remaining
heterogeneity or labor and has either a launched next packet or a concrete
autonomous continuation prompt/owner recorded in `active-work.md`.

`loud-failure` means the workflow failed, but the prompt, session/log pointer,
artifacts, failure point, and repair hypothesis are preserved.

`awaiting-Jörn-crux` means local repo/log/subagent evidence has been used and
one concrete Jörn-level judgment would change the next launch, claim strength,
or thesis target.

`blocked` means an external condition prevents useful local progress.

If the assigned scope is incomplete and no state above applies, continue.

## Loud-Failure Protocol

Use loud failure when the parent cannot safely continue but should leave rich
repair data. Examples:

- a progress/closure claim cannot pass the literal-answer gate;
- a packet wave produced artifacts but reviews cannot establish what question
  was actually answered;
- a fresh-agent probe misreads the launch material in a way likely to recur;
- source-data or leakage uncertainty invalidates the next planned inference;
- the parent no longer knows the active milestone or packet ranking;
- thesis wording would require claims outside the current evidence.

On loud failure, preserve:

```text
Failure state:
First unrecovered error:
Current milestone:
Prompt or packet that caused the failure:
Session/log pointer if available:
Artifacts or branches affected:
Claims that remain usable:
Claims that are tainted or unsupported:
Repair hypothesis:
Restart recommendation:
```

Do not bury a loud failure in a normal status report. Add a durable entry to
`process-learnings.md` when the failure reveals a reusable process update.

## Claim Gate

Before writing or implying "answered", "complete", "done", "validated",
"enough", "supports", "the story is coherent", "we can write this now", or
equivalent progress/closure wording, fill:

```text
Original question:
Literal answer:
Source evidence:
Boundary / unanswered remainder:
Downstream use allowed:
```

If the literal answer changes the question, the original question is not
answered. State the narrower fact and keep working or rescope.

## Jörn-Crux Gate

Ask Jörn only after local evidence has been used and the answer would change
packet ranking, claim strength, thesis wording, or mathematical taste. Avoid
permission questions and should-questions.

Use this shape:

```text
I need one judgment because it changes <packet ranking / claim strength /
thesis wording>. Current local evidence says <short facts>. The crux is
<territory-level uncertainty>. If A, I will <action>; if B, I will <action>.
```

If both branches are cheap to explore with agent work, explore them and record
the assumption instead of asking.

Do not ask "Should we use bounded fallback or pursue stronger coverage?" while
the local default is still to build the broader expansion plan and cheap scout
alternatives have not been exhausted. Ask only after the local branches are not
cheap to explore and the answer changes a concrete packet ranking or claim
strength.

## Workflow-Test Packets

Workflow-hardening packets are allowed even when they have zero direct merge
value for thesis evidence. They test whether the material causes agents to
choose, execute, review, stop, and report correctly.

For workflow-test packets:

- mark `Workflow-test: yes`;
- use controlled source material and a controlled workspace when possible;
- preserve the prompt, raw response, parent interpretation, and resulting
  material edit or no-change verdict under
  `coordination/workflow-evaluations/`;
- do not update mathematical or method-table claims unless a later normal
  packet review explicitly promotes the artifact.

Good workflow-test targets include fresh-agent kickoff probes, A/B launch
prompts, replay of known past failures against updated material, and
adversarial bypass reviews.

## Minimal Kickoff Prompt

Use this as the starting point for the autonomous parent loop. Fill the
worktree/branch and any current explicit Jörn update before launch.

```text
You are the autonomous parent agent for the sys-datascience thesis slice.
Work in <worktree/branch>. Main must remain blocker-free; do not edit main.

Primary objective: run the sys-datascience slice until it is complete,
milestone-complete, loudly failed with restart data, awaiting one concrete
Jörn crux, or externally blocked. Do not stop with only a status report while
the scope is incomplete and locally actionable.

Read first:
- experiments/sys-datascience/README.md
- experiments/sys-datascience/agent-memory-and-expansion-plan.md
- experiments/sys-datascience/coordination/README.md
- experiments/sys-datascience/coordination/autonomous-parent-loop.md
- experiments/sys-datascience/coordination/first-wave-p1-p3-results-2026-07-08.md
- experiments/sys-datascience/coordination/p2-synthesis-2026-07-08.md
- experiments/sys-datascience/coordination/p5-mechanism-tail-thesis-use-audit-2026-07-08.md
- experiments/sys-datascience/coordination/bounded-retained-table-source-map-writeup-2026-07-08.md
- experiments/sys-datascience/coordination/p4-generated-candidate-closure-2026-07-08.md
- experiments/sys-datascience/coordination/high-complexity-producer-compute-packet-2026-07-08.md
- experiments/sys-datascience/coordination/workflow-orchestration.md
- experiments/sys-datascience/coordination/research-ledger.md
- experiments/sys-datascience/coordination/next-session-candidates.md
- relevant experiments/sys-datascience/coordination/topics/*.md

Current launch milestone as of 2026-07-08: P1/P3 read-only design packets, P2
execution/review/synthesis, P5 audit, bounded retained-table source-map, P4
generated-candidate closure, and high-complexity producer compute-packet
preparation have returned. The bounded retained method-table story is a
fallback, not automatic full-slice closure. The high-complexity compute packet
is a prepared smoke-first LICCA handoff, not evidence until executed and
reviewed.

First 30 minutes: state the milestone, create/update active-work.md, read the
P1/P3, P2, P5, source-map, P4, and compute-packet syntheses, then decide
whether the next milestone is thesis prose from the bounded source map, LICCA
execution/review of the high-complexity compute packet, or loud failure because
neither route matches the needed thesis claim. Do not redo earlier control
passes unless the source state has changed or the syntheses are invalid.

Before launching work, create or update active-work.md and write packet cards
for the first wave. Every packet must name the target claim/decision/model
uncertainty, why it beats a concrete alternative, assumptions, allowed pivots,
stop condition, review standard, downstream use, exact thesis sentence or
decision, source artifact/table, evaluation target, protocol choices, and best
rejected or parked alternative.

After each wave, synthesize before launching follow-ups: update the claim
ladder, source map, open discriminators, next packet ranking, parked/rejected
list, and process learnings when applicable.

Before treating packet output as thesis evidence or a global belief update,
require a named review verdict artifact or clearly labeled review section
separate from executor output.

Before claiming progress or closure, fill the claim gate: original question,
literal answer, source evidence, boundary/unanswered remainder, and downstream
use allowed. If the answer changes the question, the original question is not
answered.

If the workflow fails, fail loudly: preserve the first unrecovered error,
prompt/session/log pointer, affected artifacts, usable claims, tainted claims,
repair hypothesis, and restart recommendation.
```
