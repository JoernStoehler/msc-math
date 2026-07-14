# Improve The Empirical-Research Workflow

Read this only during an explicitly authorized workflow-review or skill-editing
task. The current skill is provisional process knowledge, not a validated
method. Ordinary research should use it with judgment and report a material
observed burden or failure when one occurs; do not instrument every packet or
create process work merely to fill an evidence ledger.

Contents: [find the failure](#find-the-failure),
[use informative evidence](#use-informative-evidence),
[record a candidate](#record-a-candidate),
[choose an intervention](#choose-an-intervention),
[review architecture](#review-architecture), and [change gate](#change-gate).

## Find The Failure

Before editing instructions, distinguish:

1. **Discovery:** the skill did not trigger or was unavailable.
2. **Routing:** the skill loaded the wrong reference or omitted a needed one.
3. **Context:** the reference lacked repo-specific information the agent could
   not cheaply reconstruct from its owners.
4. **Advice:** the agent followed the process, but the process itself produced
   a worse scientific, portfolio, or cost outcome.
5. **Application:** the advice was suitable, but the agent ignored,
   misunderstood, or ritualized it.
6. **Configuration:** the prompt, decomposition, context mode, model/effort,
   tools, or oversight made the outcome unlikely.
7. **Portfolio:** the local packet succeeded while suppressing a more valuable
   line, delaying a return, or consuming disproportionate attention.
8. **Burden:** the process reduced errors but cost more than the avoided error,
   or created review, status, provenance, or coordination work with no consumer.

These diagnoses can coexist. They imply different treatments; adding another
instruction is appropriate for only some of them.

## Use Informative Evidence

Prefer evidence from real research transitions and downstream use over
instruction recitation or agent confidence. Useful problem-finding situations
include:

- a small or source-obvious packet that should not instantiate the full line
  workflow, exposing ritual overhead;
- a fresh executor or interpreter consuming a packet without hidden context;
- a cold resume from the line's maintained state;
- an independent source-based option search that can expose skill-induced
  narrowing or missed experiment families;
- an early smoke-stage review compared with later review when a live packet
  makes the timing decision material;
- a negative-control task on which a proposed new rule should stay dormant;
- a boundary task crossing investigation displays and `$thesis`, empirical
  code and `$rust`, or local feasibility and `$licca`;
- naturally comparable model/effort or flat/delegated configurations when the
  real routing decision could plausibly choose either.

Measure final decisions, artifacts, surviving defects, repair, reruns, parent
attention, CPU/wall time, and downstream usability where available. A passing
packet does not establish that its process caused success or had positive net
value. A caught defect shows recovery value conditional on the defective input;
also consider cheaper prevention.

When unprimed behavior matters, do not tell the agent the suspected failure or
intended fix. Preserve the exact prompt, raw response or artifact, reviewer
verdict, and parent interpretation separately. Raw Codex session JSONL is the
behavior source of truth; read events show exposure, not understanding.

## Record A Candidate

A reusable candidate should name:

- the observed failure or benefit and downstream transition;
- what the agent, self-review, reviewer, or consumer missed or caught;
- actual known burden, salvage, and remaining gate;
- live competing explanations that would change treatment;
- likely recurrence and consequence if no change is made;
- the smallest reminder, default, review question, invariant, or hard gate that
  could address it;
- false positives, option suppression, or extra work the change could cause;
- a realistic future packet and negative control on which to test it.

Do not promote a stylistic preference, one-off scientific fact, self-estimated
counterfactual, or one successful packet into the skill. Keep exact evaluation
material in an isolated `/tmp/` directory while active. Promote the smallest
transferable current instruction; Git history is sufficient for superseded
instructions and the skill should not become an episode archive.

Treat a capability episode as evidence about the configured system:

```text
(task, decomposition, context, prompt, model/effort, tools, oversight)
    -> (observable behavior, product quality, repair, downstream usability)
```

An omission may arise from elicitation, effort, planning, execution memory,
monitoring, calibration, orchestration, or model capability. Run a deliberate
discriminator only when the resulting routing or process decision is live.

## Choose An Intervention

Use the weakest mechanism likely to prevent the observed failure. Increasing
or decreasing intervention strength should be evidence-driven. Name the
observed burden or false-positive pattern when removing a constraint. Never
weaken a scientific or readiness gate merely to make the current packet pass.

After changing the skill, check both a representative task and a task where the
new guidance should not create work. Prune or replace superseded guidance
instead of accumulating exceptions. Keep `provisional reversible choice`
distinct from `established process knowledge`.

## Review Architecture

Choose review structure from failure independence and context needs:

- combine checks when they inspect the same source and neither is likely to be
  crowded out;
- split technical/provenance review from interpretation when each can pass
  while the other fails;
- use fresh context when independent downstream use is part of readiness;
- use a context-rich lead when the scientific model or stakeholder boundary is
  not durable yet;
- repeat narrow reviews only when expected error reduction exceeds
  coordination and wall-time cost.

Record which checks disappeared under overload and which combinations reduced
duplication. More reviewers are not automatically safer.

## Change Gate

Keep skill changes as a separate reviewable diff. Check trigger precision,
overlap with nearby skills, duplicated owner-local truth, accidental work
creation, and preservation of Jörn/Kai gates. Validate syntax, links, discovery,
and representative behavior in proportion to risk. Jörn reviews accumulated
process knowledge before it becomes main-branch behavior.
