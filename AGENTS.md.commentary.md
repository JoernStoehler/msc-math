# Collaboration context and rationale

Companion to [AGENTS.md](AGENTS.md). The filename is not automatically loaded by
normal AGENTS.md discovery; the main file supplies explicit reading triggers.
This file keeps useful rationale and evidence near active guidance. It is not
a transcript, assignment registry or global policy.

Source: Jörn's 2026-09-30 discussion in thread
`01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a`, including his handoff from another session.
User preferences, historical observations and proposed mechanisms are separated
below. No behavioral or post-compaction validation of this guidance has been
performed. Global promotion is deferred until the project works well and Jörn
explicitly selects it.

## Agent PM and Jörn's spending gate

**Context:** Jörn reports that agents can perform substantial PM work, including
delegation, independent sessions and planning. He also reports reasoning errors
and unreliable checklist following, with better attention to explicit permission
gates. The cause and generality of these reports are not established.
**Reason:** His gate is intended to improve selection of experiments, results
and approaches before expensive execution. Efficient execution of a poor plan
still spends effort on the wrong work. Preliminary exploration/planning deserves
real resources, rather than being treated as overhead to minimize automatically.
**Boundary:** This does not require Jörn to dispatch every worker or approve
every small probe. It does not authorize new thesis work merely because the
backlog exists. Role/lifecycle separation does not separate the work from its
original scope and spending boundary.

### Proposed packet lifecycle

This procedure is a local implementation proposal. Its numeric limits and
enforcement mechanism remain open.

1. An owner undertakes bounded discovery and planning within selected work.
   It can use reading/triage subagents and an independent planning session.
2. It prepares the objective, existing evidence, important alternatives,
   proposed experiments/results, expected outcomes and costs, and useful
   partial/failure outcomes. The plan can recommend dropping or decomposing the
   proposed package.
3. It asks Jörn for the actual selection/spending decision. The question or
   linked packet supplies enough context to assess scientific value and likely
   oversights; it does not just ask permission to launch a command.
4. When substantial expenditure or another established requirement needs
   Jörn's approval, the dependent package waits. An optional clarification does
   not create a mandatory gate; preserve its stated fallback.
   Its owner may continue useful independent authorized work
   or finish its planning assignment with an explicit receiving owner. Ownership
   transfer must preserve the pending gate; do not invent work to stay active.
5. Approval can cover a substantial coherent execution package. Record the
   approved scope and resources; discuss a material change before consuming
   resources beyond that approval.

**Open choices:** what counts as large expenditure; per-packet exploration and
execution allowances versus a project-wide ceiling; budget units; and whether
any mechanical enforcement is needed. Async-question UI durability is also
unresolved. An unanswered required approval remains pending regardless of UI
visibility; optional clarifications retain their stated fallback.

### Relevant observed failures

- [September process audit](docs/history/process-audit/REPORT.md), findings 1
  and 5: the incumbent computational approach received execution effort before
  a meaningful alternative comparison; a later correction produced a stronger
  route. The same audit found an invented approval gate for small already
  authorized experiments. Preserve both lessons: invest in strategy and avoid
  indiscriminate gating. The later proof does not establish that earlier
  discovery was guaranteed, and the audit is observational.
- [Planning context](docs/history/memories/planning-context.md): deferring a
  summary design was treated as closing the authoring-workflow branch. Deferring
  one proposed solution does not dispose of its parent objective.
- [Interaction trial](docs/history/reviewer-trial/interaction-handoff.md):
  concrete questions with sufficient context elicited useful judgments;
  shortness alone did not. The historical relayed rules in that file are not
  automatically current instructions.
- Process-audit findings 3–4: local reviews and resolved findings did not close
  assembled-artifact interfaces or establish whole-document readiness.
  Completion must name the deliverable and evidence actually checked.
- [Workstation OTel setup](docs/coordination/otel-design.md), 2026-09-30:
  the handoff reports that Jörn found the verification/documentation effort
  excessive for this low-maintenance setup. This is scoped feedback, not
  permission to skip necessary checks. Verification should resolve remaining
  consequential uncertainty. Completion required the actual daemon cutover,
  which Jörn performed; live export was subsequently API-verified.

These observations motivate candidate interventions. They do not establish a
general model defect, a psychological cause or that new wording will work.

## Cost and approval mechanics: limits of current evidence

**Reasoning cost:** Jörn proposed isolating planning in subagents and returning
compact breadcrumbs to control parent context growth. This is a useful design
hypothesis, not measured savings. Reasoning tokens still occupy context and are
accounted as output tokens; whether earlier reasoning is retained depends on
model/runtime behaviour. Five minutes of thinking is not inherently cheap.
Measure worker usage, parent input/context growth and tool round trips together.
[OpenAI reasoning documentation](https://developers.openai.com/api/docs/guides/reasoning#how-reasoning-works).

**Automatic approval:** Codex's Auto permission preset and its automatic reviewer
are different controls. Auto-review evaluates eligible actions that already
require approval, including sandbox escalation and some MCP/app calls; it does
not inspect every allowed action. It adds model calls. Native subagent spawning
and complete experiment-budget gating are not established by that coverage.
An action-safety reviewer does not replace Jörn's decision about scientific
priority and expenditure. Prefer instructions and convenient tools initially;
do not change permission settings to test an assumption silently.
[Auto-review documentation](https://learn.chatgpt.com/docs/sandboxing/auto-review).

**Async questions:** the 0.157.0 changelog lists a change to clear pending async
question notifications at turn end. Generic app-server user-input requests can
also be cleared on turn start, completion or interruption, emitting the same
resolution notification as an answer. On installed 0.159.2, generated schemas
show async questions on agent-message items but no per-question status there;
the generic resolution notification has no reason field. These facts do not
establish which path this tool uses or whether steering caused a turn-end/UI
cleanup. On a natural occurrence, inspect bounded event metadata and retained
question-item presence before choosing a patch. No fix timetable is known.
[Changelog](https://learn.chatgpt.com/docs/changelog),
[app-server request lifecycle](https://learn.chatgpt.com/docs/app-server#toolrequestuserinput).

**Current disposition, 30 September:** Jörn reports receiving none of the async
questions, despite accepted tool submissions. The project now prohibits using
that route until delivery is verified. This establishes a local delivery failure,
not its cause or a global product defect. Direct chat supplies the fallback;
do not make Jörn discover a hidden unanswered decision. The restriction changes
agent usage instructions and does not remove the runtime tool.

## Other settled interaction preferences and unresolved defaults

**Context:** Jörn multitasks. Omit narration without decision value, including
routine acknowledgements, apologies and self-commentary. Correct errors directly;
report consequences and uncertainty that affect his work. Limited attention
does not justify skipped review, reduced scope, premature delivery or hidden
uncertainty.

**Continuation:** In the 3 October 2026 Codex configuration walkthrough, a brief
"thx" received a courtesy-only end-of-turn while the broader discussion remained
unfinished. Jörn identified this as a recurring mistake and clarified that the
concern is the Codex+user workflow, not merely his personal communication style.
The rule in AGENTS.md preserves authorized work across acknowledgments so Jörn
does not have to reactivate it. It does not authorize new scope or bypass a
required decision. The wording's effect on recurrence has not been tested.

**Structure:** Distinguish context, rule, boundary and useful reason/example
where this aids interpretation; these are labels, not a mandatory four-paragraph
schema. Keep interpretation-critical boundaries in active guidance. Longer
rationale and alternatives can live here. HTML comments are model-visible and
do not disable guidance.

**Unresolved defaults:** whether exploratory “maybe we could build X” discussion
permits reversible implementation experiments; and whether substantial
deliverables always require independent review or independence should depend on
usefulness/consequence. Neither agent recommendation is a settled user choice.
The retired global rules about commits, secrets, question preparation, uncertainty,
workspace hygiene, plugin ownership and host operations need local assessment;
do not automatically reinstate them.

## Retrieval and evaluation

Use [the knowledge index](docs/knowledge/README.md) to find the process evidence
relevant to the current decision. Curate the lesson and its boundary here or
with its domain owner; leave matched inputs and exact reviews with their evidence
owners. Do not routinely crawl Git history to reconstruct current guidance.

A representative forward evaluation should check whether an agent compares
approaches before execution, asks a concrete spending question, preserves an
unanswered gate through steering/handoff, and distinguishes a bounded review
from final acceptance. Include an already-authorized small task to catch
unnecessary gates. Such an evaluation has not been run; syntax/link checks do
not establish behavioural improvement.
