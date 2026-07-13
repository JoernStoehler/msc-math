# Agent Capability Ledger

Use: compact, source-linked working beliefs about which agent configurations
are likely to produce usable sys-datascience work for recurring task shapes.
Its readers are portfolio parents, research leads, and prompt/workflow owners.
It is not a model leaderboard, transcript archive, or requirement to evaluate
every agent call.

Status: provisional workflow and initially empty evidence ledger. Raw Codex
rollout JSONL, repository artifacts, and reviews remain source truth. Add an
episode only when it changes a routing decision, informs a contemplated prompt
or workflow change, records a surprising/costly failure, or gives an
informative comparison likely to recur.

## Object Of Inference

Treat an episode as evidence about a configured system, not a model alone:

```text
(task shape, decomposition, context, prompt, model/effort, tools, oversight)
    -> (observable behavior, artifact quality, downstream cost and salvage)
```

Keep distinct labor requirements visible when they affect routing. Examples
include context reconstruction, scientific abstraction, decomposition,
long-horizon planning, implementation, obligation tracking, live monitoring
and replanning, review initiation, adversarial checking, interpretation,
provenance discipline, and synthesis for another agent. This is an open list,
not a homogeneous ontology every entry must fill.

Do not infer internal causes from behavior alone. A file read shows exposure,
not understanding; an omitted check may reflect context, elicitation, reasoning
budget, planning, execution memory, monitoring, calibration, or orchestration.
Encrypted reasoning is unavailable. Agent final summaries are navigation and
self-report, not authoritative evidence of correctness or cognitive cause.

## Evidence Order

Use the narrowest sources needed for the claim:

1. rollout JSONL for prompt/context boundary, lineage, visible messages, tool
   calls, timing, compaction, and what the agent inspected or changed;
2. commits, artifacts, tests, and generated outputs for the delivered product;
3. independent review and downstream repair for usability and hidden defects;
4. parent feedback for integration burden, missed expectations, salvage, and
   the portfolio decision the episode informed;
5. agent self-summary for claims to verify or intent not otherwise visible.

For durable log claims record the thread id, rollout path, focused query or
event description, and extracted result. Do not copy broad transcripts or
private material into this file. Follow `$codex-session-log-parsing` when doing
intake.

## Repeated Intake Workflow

Typical input is a list of parent/child task handles or thread ids plus the
parent's already-written feedback about usefulness, defects, repair, or other
surprises.

Prefer thread ids when already available. Task handles can be resolved through
the parent rollout, but that adds intake work and names may not be stable. An
episode can still be useful when the incoming prompt is encrypted or it has no
artifact/reviewer: mark those fields unavailable, rely only on visible events
and separately supplied parent evaluation, and weaken the supported claim.
Parents and research leads expecting later intake should preserve child and
subchild thread ids in their handoff or scratch state when the spawn result
exposes them; do not create a ledger entry merely to store an id.

### 1. Fix the decision target

State which future choice the intake may change: model/effort, fresh versus
forked context, planner/executor split, prompt reminder, review requirement,
resource envelope, or another concrete routing decision. If no recurring
decision is visible, leave the evidence in the raw logs and task owner.

### 2. Resolve lineage and products

For each selected episode, locate the rollout and its parent/child lineage.
Identify the exact task prompt, context mode, model/effort when available,
important reads/actions, product artifact or commit, reviewer, repairs, and
parent feedback with its message/event or durable-note pointer when available.
Mark recollected or otherwise untraceable feedback as such. Include subsubagents
as part of the configured system: a
Sol-planned/Luna-executed/reviewer-repaired result is not evidence that any one
component completed the end-to-end task alone.

### 3. Record behavior before diagnosis

Describe externally checkable behavior narrowly. Examples:

- implemented the enumerated analysis and passed its stated tests;
- ended without performing an implied review;
- planned a provenance check but omitted it during execution;
- noticed an invalid assumption and replanned without prompting;
- produced a plausible interpretation later rejected by review.

Keep the parent's evaluation distinct from the log-derived observation. Record
what repair was needed and which partial work remained useful.

### 4. Maintain competing explanations

List only diagnoses that would change a future treatment. Do not collapse
`could do when asked` into `spontaneously initiates`, or `stronger model
succeeded` into `weaker model is incapable`. Useful discriminators include:

- same agent plus a focused reminder;
- same agent with a supplied plan or better source context;
- same model at higher effort;
- stronger model under the same prompt;
- fresh review for a plausible hidden defect;
- a naturally recurring comparable task.

Run a discriminator only when its expected routing or workflow value exceeds
its cost. Do not benchmark every adjacent level or create work merely to fill
the ledger.

### 5. Update the narrowest routing belief

Prefer a conditional statement such as:

> Luna-low is a plausible executor for an explicitly enumerated,
> externally-checkable scalar analysis under a lead; this episode does not
> support autonomous experiment design or interpretation.

Avoid `model X is good at data science`. State confidence, counterexamples,
and what evidence would change the belief. Separate immediate low-regret use
from a durable prompt/harness proposal.

### 6. Fan out sparingly

- Keep episode rows and current routing beliefs in this file.
- Put task-specific scientific/process consequences in the owning topic or
  packet only when its future work needs them.
- Update `process-learnings.md` when a recurring workflow failure or benefit
  changes sys-datascience behavior.
- Propose shared-skill or harness changes only after the applicable workflow
  change gate and forward tests; this ledger does not authorize them.

## Episode Template

Use compact prose or structured fields; omit fields irrelevant to the decision.

```text
### <stable episode name>

Decision informed:
Configuration: task shape; decomposition; model/effort; fresh/forked;
  planner/executor/reviewer roles; resource envelope
Sources: thread id; rollout path; focused timestamp/event/query;
  artifact/commit/review or explicit `none`; encrypted/unreadable fields
Parent evaluation: already-observed usefulness, defect, repair, or surprise
Observed behavior: source-backed actions and omissions, without mental cause
Outcome: independently usable / cheap repair / useful substrate / obvious
  failure / plausible hidden failure / discarded
Downstream cost and salvage: only quantities or comparisons actually known
Live explanations: context / elicitation / budget / planning / execution /
  monitoring / calibration / orchestration / model-capability, or task-specific
  alternatives
Discriminator: cheapest worthwhile follow-up, or `none`
Routing update: narrow conditional belief
Confidence, counterexamples, and expiry/revisit condition:
```

## Current Routing Beliefs

None yet. Intake should establish source-linked episodes before promoting
session impressions into this section.

## Episode Index

None yet.
