---
name: subagent-prompting
description: Use when a fresh subagent with zero inherited conversation turns will own a bounded assignment that needs non-obvious context, ownership boundaries, completion evidence, or a return contract; this includes consequential independent review and repair after failed delegation. The outcome and reason for delegation must already be decided. Skip assignments expressible clearly in one or two sentences and open-ended session ownership.
---

# Subagent Prompting

Subagents are useful when another agent can own a bounded result while the parent
keeps noisy intermediate work out of its context, runs independent work in
parallel, or gets an unprimed judgment. Delegation also adds coordination cost:
the recipient lacks private context from the parent session, and the parent
must be able to judge and use the result without reconstructing the work.

This skill helps make that trade worthwhile. Turn an already-chosen outcome and
reason for delegation into a prompt that carries the information the recipient
cannot recover, while leaving generic reasoning and ordinary implementation
choices to the capable agent doing the work.

## Confirm The Assignment Exists

Begin only when all are already established:

- a desired result, state change, or decision;
- a reason the work belongs with a subagent, such as quarantining noisy
  intermediate context, parallel execution, independent interpretation, or
  bounded ownership of implementation and validation;
- a fresh recipient with zero inherited conversation turns.

This verifies the assignment precondition; it does not reopen the parent's
delegation choice. Do not start from available subagent capacity and search for
a purpose. If no established outcome exists, the desired result is still a
tentative strategy, several incompatible outcomes are being conflated, or the
parent cannot say what it will do with the result, report the missing task
definition instead of disguising one proposal as an assignment.

## Build The Task Model

Determine the following before polishing prose. Include only details that can
change the subagent's work or the parent's ability to use it.

This is a decision inventory, not a required prompt template. Combine fields,
omit answers already obvious from the recipient's actual context, and do not
reproduce these headings or emit empty sections.

- **Problem and downstream use:** Explain why the result is wanted and what
  questions or problems it must address. Name the decision, artifact, or later
  action it must support.
- **Deliverable and integration boundary:** Define the required hand-in and the
  surface downstream work will consume or merge. Bound required changes so the
  parent can evaluate the result and combine parallel work without surprise
  scope expansion or broad refactors. A worktree isolates execution; it does
  not define the acceptable hand-in. State whether adjacent repairs should only
  be reported or may be implemented as optional hand-ins. When implemented,
  keep them separable as optional commits or a second worktree so the parent
  can decline them without losing the required result.
- **Task context:** Supply session-private decisions, rationale, observations,
  constraints, or current status that changes how the recipient should
  interpret the task and cannot be recovered from repo sources. Distinguish
  fixed assignment boundaries and accepted stakeholder decisions from factual
  premises, provisional diagnoses, and suggested strategies. Say what is fixed
  for the assignment and what reasoning or evidence the subagent may
  re-evaluate.
- **Source routes and recommended reading:** Name authoritative files,
  artifacts, or commands the recipient should inspect, and distinguish required
  sources of truth from optional starting points and from preliminary reasoning.
  When a source's role is not obvious, say plainly why you recommend it, to
  avoid overeager guesses about private motives or treating source material as
  hard constraints or vetted for correctness beyond what it was actually
  checked for.
- **Working authority:** State the editing or read-only surface, allowed side
  effects, workspace ownership, and decisions reserved for the parent because
  it owns integration or for Jörn because they depend on stakeholder
  preference. Name relevant concurrent work and derive concrete scope
  boundaries from it; for example, identify shared interfaces other agents
  depend on instead of merely saying the parent has wider context. Tell editing
  workers when they are not alone and must preserve others' changes. For an
  editing assignment, let the subagent create its own worktree unless the
  parent supplies an appropriate isolated workspace. Use an existing shared
  integration worktree when the task deliberately requires coordinated edits
  there, and state each agent's ownership boundary. For read-only work, name the
  workspace or artifacts to inspect; do not create a worktree merely for
  isolation.
- **Completion evidence:** State both the observable evidence and any abstract
  quality desiderata that downstream use depends on. Let the capable subagent
  infer their task-local implications instead of trying to enumerate every
  quality dimension. Include observed failure modes or known ways to detect
  them when they materially improve the work, clearly as non-exhaustive
  attention guidance rather than a mandatory checklist. Consider whether an
  easier but useless substitute could satisfy the stated evidence.
- **Return contract:** Request only what the parent needs to evaluate or
  continue. Leave implementation-local paths, commands, type signatures, or
  architecture choices to the subagent unless the downstream contract fixes
  them. Say what test evidence, README-level behavior, documentation,
  limitations, or blockers the parent needs for review. The parent usually need
  not receive a narration of the process.
- **Stop and escalation conditions:** Identify assumptions behind the
  assignment whose falsification would make continuing invalid or wasteful.
  State what missing evidence, blocked action, scope expansion, or failed
  validation should cause the subagent to stop and report rather than guess or
  silently change the objective. Require the same when authoritative source
  evidence conflicts with a fixed boundary, an accepted decision, or a factual
  premise material to the hand-in. The subagent should return the conflict and,
  when useful, conditional paths rather than choose silently or override the
  assignment.

For review prompts, also name the target, source material, downstream decision,
and priority lenses so the subagent can interpret open or abstract review
targets. Make clear whether the lenses are ranked or weighted priorities, a
closed whitelist, or reminders of past oversights. When findings will drive
repairs, ask for findings first with evidence or location, downstream
consequence, and a concrete correction, plus an explicit statement when no
material finding remains.

When a decision depends materially on the parent's wider task context, ask for
the evidence, alternatives, tradeoffs, and conditional reasoning that make the
decision legible. A bare verdict such as `yes` is rarely a useful hand-in. A
conclusion may accompany the analysis, but the parent should be able to
evaluate it in context rather than inherit it as an unexamined premise.

State the relative costs of false positives and false negatives when they shape
the review. For example, bias a code review toward finding nearly every defect
only when downstream evaluation can afford to filter the additional false
alarms.

## Use A Fresh Recipient

Spawn the subagent with zero inherited conversation turns
(`fork_turns="none"`). Supply all task-relevant private context explicitly in
the prompt and route the recipient to repo sources it can inspect directly.
Fresh context keeps intermediate discussion out of the recipient, prevents
inherited clues from masking prompt defects, and avoids the unpredictable
effects of partial context sharing. Point to source truth rather than only the
parent's reasoning so the subagent can check and improve that reasoning.

Choose model and reasoning effort from task difficulty and the evidence needed,
not a permanent Sol/Luna routing slogan. Smaller or cheaper recipients may need
more explicit task-local structure, but missing private context is a prompt
defect for every model. Treat model differences as empirical until
representative results support a stable rule.

## Write And Self-Review The Prompt

For a nontrivial prompt, draft in `/tmp` and review it before exposing it as
ready. Lead with the problem and outcome, then the context and constraints that
shape the work. Use direct prose and the lightest structure that preserves the
task model.

Do not prescribe step-by-step reasoning or implementation unless the sequence
is a real dependency, safety boundary, required workflow, or part of the
artifact contract. Explain non-obvious constraints through the outcome or risk
they protect. Remove repeated rules and context that the recipient can recover
cheaply.

Before using the prompt, ask yourself the following. Do not copy this
self-review into the recipient prompt:

- Can the recipient tell why this work matters, what it must hand in, which
  integration boundaries apply, and what done means?
- Which important facts am I assuming only because I saw the parent session?
- Did I turn an observation, diagnosis, or proposed strategy into an unjustified
  imperative?
- Could the recipient return something formally compliant but worthless for
  the downstream use?
- Does the return contract keep noisy intermediate work out of the parent while
  preserving enough evidence to trust the result?

## Evaluate Proportionally

For ordinary bounded work, inspect the returned artifact and validation rather
than adding a ritual prompt review. For a costly, reusable, repeatedly failing,
or hard-to-reverse prompt, use representative evidence before scaling it.

For representative evaluation, judge recipient comprehension and assignment
value separately. Test comprehension with the surfaces a real recipient would
receive: ask what it understood, inferred, found ambiguous, and would do first.
Test value against the named downstream use: can the parent attain the desired
state or make the decision from the result without reconstructing the
subagent's work, and did the result produce the otherwise-needed artifact or
reduce the relevant uncertainty? A clear, executable prompt fails if its result
does not create the usable change it was assigned to create.

Keep the exact prompt, raw result, evaluation verdict, and parent interpretation
separate when the evidence will guide a durable workflow or harness change.
Adjudicate premise-threatening reviewer findings explicitly; do not convert
every warning into a local wording repair of the same prompt shape.
