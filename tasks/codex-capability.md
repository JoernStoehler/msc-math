<!--
Purpose: project-local capability and failure-mode register for GPT-5.5/Codex
agents during thesis closeout.
Context: this file is a planning aid, not a benchmark and not a guarantee.
It records observations, source strength, and concrete process implications so
future agents do not invent a capability model in chat.

Update rules:
- preserve Jorn-supplied observations before abstracting them
- do not paraphrase Jorn-supplied examples as source-truth prose
- label source type and evidence strength
- do not turn "worked under these conditions" into "Codex can do this"
- prefer conservative planning implications under deadline pressure
- add deep-research and benchmark evidence only with limitations attached
-->

# Codex Capability Register

## Status

- State: provisional seed from Jorn aggregate experience and current chat.
- Last updated: 2026-05-18.
- Source surfaces: this chat, `tasks/sys-first-order.md`,
  `tasks/verify-thesis-done.md`, `tasks/writing.md`,
  `/tmp/chatgpt-deepresearch-2026-05-18.md` pending integration.
- Refresh when: a Codex session succeeds or fails in a way that changes task
  assignment, review burden, or thesis-closeout risk.

## Purpose

This file records what the project currently knows about GPT-5.5/Codex agents
as used for this thesis. It should help future agents decide what to attempt,
what not to attempt, what to give Jorn, and what controls to add when an
attempt is still worthwhile.

This file must include the reasoning flow:

- observations and source type
- interpretation and uncertainty
- capability profile
- planning implications
- concrete agent behavior

Model self-report is not a source of truth. In particular, answers to "can you
do X?" are treated as unreliable unless supported by project evidence,
external evidence, or Jorn's experience.

## Source Types

Use these labels when adding rows.

- `Jorn aggregate experience`: Jorn's remembered or synthesized experience;
  not necessarily backed by session logs.
- `Jorn raw note`: Jorn-supplied wording copied or minimally cleaned for
  typos/formatting only.
- `representative example`: realistic example supplied to capture a recurring
  pattern; not necessarily an actual logged session.
- `repo-local evidence`: task files, research notes, commits, review notes,
  or failed-route notes in this repository.
- `external evidence`: public benchmarks, system cards, papers, user reports,
  or deep-research summaries.
- `current-chat evidence`: failures or corrections observed in this chat.

## Maintenance Contract

Jorn-experience material is high-value and easy to corrupt by paraphrase. An
agent usually does not know which words or conditions carry causal weight.

Rules:

- Add new Jorn observations first under `Raw Intake Log`.
- Preserve Jorn's wording as much as practical.
- Minimal cleanup may fix typos, line breaks, and local formatting.
- Do not merge, generalize, or reword examples in the raw layer.
- Put any agent summary under an interpretation or processed-notes section.
- If an agent changes the meaning, strength, scope, or condition list of a
  Jorn-supplied example, mark it as agent interpretation, not source truth.
- When in doubt, duplicate rather than rewrite: keep the raw note and add a
  separate interpretation below it.

## Raw Intake Log

Append Jorn-supplied observations here before interpreting them. Entries may be
remembered, aggregate, or representative rather than log-backed.

### 2026-05-18 Current Chat

Source type: `Jorn raw note`, lightly formatted from chat.

- "Can you do X?" is wildly uncalibrated and inaccurate.
- "In this session X went wrong, why?" makes up plausible-sounding internal
  processes instead of admitting that no mechanistic explanation can be given
  without access to past CoTs and without evaluating counterfactual prompts or
  ideas.
- "Figure out what objective we want to achieve" returns an operationalization
  early which is useless for thesis success, hard to achieve if taken serious,
  and easy to game if taken too literal.
- Planning how to achieve X is risky when agents have a hard time telling
  whether X was achieved.
- Selecting a robust feedback signal is uncalibrated and may pick a signal
  that is easy to game.
- "Justify your edits" may produce post-hoc justifications instead of selecting
  edits that have a good justification.
- "Write prose that sounds well" can produce ambiguous, unclear,
  hard-to-follow prose with hallucinations, bad flow, missing content, and
  superfluous content.
- Review of prose can catch some classes of errors, especially when listed
  explicitly, but a list of findings is not an upper bound on remaining issues.
- Subagent prompts and harness work are hazardous unless the biggest pitfalls
  are highlighted, such as worktree context, independence, spoilers, and hidden
  secondary goals.
- Hot-fixing derailed sessions is often not worth it. Abandoning the worktree,
  extracting learning with disclaimers, and starting a new agent with a better
  prompt can be better.

### 2026-05-18 Prompt-Planning Abstractions

Source type: `Jorn raw note`, lightly formatted from chat.

Jorn's rewording of the useful question:

- When Jorn writes a prompt for Codex, what does he pay attention to to decide
  whether to abort, whether to do something or skip something, whether to fix or
  iterate, and what affects his predictions about whether Codex will succeed or
  how Codex will fail?

Jorn-reported factors:

- Is the outcome something inside the filesystem where future GPT-5.5 agents
  have access to it?
- Or is it something only the current GPT-5.5 agent has access to, such as temp
  files that are not referenced in an in-repo file, chat messages that only Jorn
  and the current agent read, or non-final-turn-message material that Jorn
  cannot read but the agent can read?
- Or is the desired outcome not actually something agents can interact with?
  In that case the outcome must be translated, or agents must be told that Jorn
  will collaborate. Example: agents cannot submit a thesis and may flail when
  told to do so, but if Jorn says he can print and hand in forms if told to, the
  agent has a path to produce the relevant outcome.
- Is the outcome verifiable by GPT-5.5? Abstractly, for an outcome "produce X
  such that phi(X)", is there a verifier task "given X, decide phi(X)"? Then the
  question becomes how to prompt GPT-5.5 to verify.
- Is the verifier robust to being gamed? GPT-5.5 is not adversarial, but bad
  habits can make it overlook that it gamed or broke the verifier and the
  verifier stopped being semantically meaningful.
- Prototype verifier-breaking example: take a failing test and ignore it. The
  check is green, but the semantic significance or evidence strength of that
  check becomes zero.
- If raw context is needed for a verifier to be interpreted correctly, inline
  the context into the verifier task. GPT-5.5 may overlook context even if it
  read or discussed that context earlier.
- On a further meta level, can GPT-5.5 tell whether it ran the verifier or
  completed it? This is again about whether the verification task is doable and
  clear.
- Verification is usually doable when formulated as a list of necessary and
  sufficient conditions, an AND over conditions, where each condition is atomic
  enough.
- Asking GPT-5.5 to verify that "the output is useful" can be treated as
  producing a usefulness analysis artifact with coherent argumentation, no gaps,
  and correct weighting. GPT-5.5 knows from training to write such an internal
  usefulness analysis and usually will not simply lie about the analysis result.
- Once Jorn trusts that GPT-5.5 can do the verification task, he also considers
  whether GPT-5.5 will actually do it or whether bad habits will make it skip,
  forget, or half-assess the verification.
- If verification is skipped or half-assessed, the final outcome is often
  half-complete: a first draft instead of a polished report, half the points
  covered, or code plus one alibi test instead of code plus meaningful tests.
- A common fix is to say "and review your work." This can make GPT-5.5 flag the
  incompleteness itself. It helps both at the end, as a stop before handing in
  incomplete work, and during planning, because the agent plans with the full
  task in mind.
- A more robust version is a review step by a non-fork subagent prompted by the
  agent.
- Another factor is context: does the agent have access, not merely a
  predigested or enumerated substitute, to the information it needs?
- Only rarely, Jorn anticipates process explicitly: tell the agent what to do
  because he anticipates failures such as forgetting to review, forgetting
  implied verification aspects, forgetting tools, or forgetting it can use Jorn
  time via chat, for example by asking a questionnaire and pulling in Jorn's
  expertise or gut feeling.

### 2026-05-18 Concrete Capability Point Ratings

Source type: `Jorn raw note`, lightly formatted from chat.

Success interpretation:

- Success means the agent does a packet of work that is useful.
- A raw literal interpretation of a task can still be `0.0` success if it is
  not a useful packet. Example: for adding tests, "it added one test" can
  satisfy a literal reading while adding no useful project value.
- Adding many low-information variants is closer to success, but can be bad for
  maintainability. Example: 20 tests that are really four tests plus 16 variants
  may add only 0.1 bits to the likelihood that the code is correct; Jorn might
  merge it, but it incurs future cleanup cost and constant overhead until then.
  This might count as about `0.8` success depending on context.

Column meanings:

- `low`: Jorn gives the natural prompt he would write without extra
  failure-mode engineering.
- `high`: Jorn deliberately compensates for predicted Codex failure modes.

Ratings:

- X01: Fix a Rust compile error from compiler output after a rename.
  `low/high = 99/99`.
- X02: Add tests for an already implemented helper when expected behavior is
  given. `low/high = 80/90`.
- X03: Implement a Rust helper for a formula from a specific LaTeX lemma. Also
  add tests for the helper. `low/high = 50/85`.
- X04: Implement a full paper algorithm in Rust. Also write documentation and
  a verification path. `low/high = 15/35`.
- X05: Audit Rust code against a formal note. Report mismatches only; do not
  edit code. `low/high = 75/80`.
- X06: Prove a 10-20 line lemma. The statement and definitions are already
  fixed. Assumption from Jorn: line count is for a lemma description after
  inlining non-well-known definitions. `low/high = 80/85`.
- X07: Prove a new lemma needed for HKO exact Packet 3. `low/high = 50/80`.
- X08: Review an agent-written proof. List steps that are not obvious to the
  reviewer. `low/high = 75/85`.
- X09: Repair a proof after a reviewer says one named step does not follow.
  `low/high = 85/90`.
- X10: Decide whether a theorem-strength thesis claim is justified.
  `low/high = 65/85`.

Writing ratings. Jorn says these are high-uncertainty and should update as he
experiences agents succeeding or failing. Re-interview target: 2026-05-25.

- W01: Produce rough prose that helps Jorn think. Success means Jorn can use it
  as a thinking aid, even if it is not close to thesis-ready. `low/high =
  60/80`.
- W02: Produce a structured prose draft from scaffold comments and linked
  notes. Success means the draft covers the right points and is useful for
  revision, but still needs Jorn/editor review. `low/high = 30/60`.
- W03: Produce thesis-ready local prose for a bounded paragraph. Success means
  it can plausibly remain after review with only small edits. `low/high =
  15/30`.
- W04: Produce publication-ready section prose. Success means structure, math
  claims, flow, and wording are all good enough for final thesis use.
  `low/high = 05/10`.
- W05: Rewrite existing prose for clarity while preserving mathematical
  meaning. `low/high = 30/80`.
- W06: Review prose for unsupported claims against listed source files.
  `low/high = 75/90`.
- W07: Review prose for readability, flow, ambiguity, and missing context.
  `low/high = 60/80`.
- W08: Fix prose after a reviewer lists concrete issues. `low/high = 75/90`.
- W09: Decide final theorem wording or claim strength. Success means Jorn
  reviews and finds nothing to disagree with. `low/high = 75/80`.
- W10: Decide whether a side result belongs in the thesis. `low/high = 05/30`.

Experiment, planning, agent, and capability-file ratings:

- X16: Run an existing experiment command from a README. Summarize the output.
  `low/high = 95/95`.
- X17: Debug a Python analysis script. The prompt gives the crash command,
  stack trace, and relevant script. `low/high = 95/98`.
- X18: Design a small experiment for two concrete hypotheses supplied in the
  prompt. `low/high = 60/90`.
- X19: Interpret an experiment table. Decide what thesis claim the table
  supports. `low/high = 90/90`.
- X20: Decide whether another cluster-scale run is worth waiting for.
  `low/high = 50/75`.
- X21: Read task files and produce a current-blocker roadmap. `low/high =
  90/95`.
- X22: Choose the next task under deadline pressure. `low/high = 80/90`.
- X23: Prepare a decision packet for Jorn. `low/high = 70/95`.
- X24: Write a worker subagent prompt for a bounded task. Success means one
  attempt, no follow-up or retry needed. `low/high = 66/80`.
- X24': Write a worker subagent prompt for a bounded task. Success allows
  retries or follow-ups. `low/high = 80/90`.
- X25: Write an independent review-agent prompt. `low/high = 75/95`.
- X26: Rescue a derailed agent after it already made bad edits. `low/high =
  05/30`.
- X27: Extract lessons from a failed session and write a better restart prompt.
  `low/high = 10/15`.
- X28: Append a raw Jorn observation to `tasks/codex-capability.md` with
  minimal cleanup. `low/high = 90/95`.
- X29: Convert raw observations into agent rules. `low/high = 05/10`.
- X30: Integrate deep-research findings into `tasks/codex-capability.md`
  without rewriting Jorn notes. `low/high = 90/95`.

External, harness, and failure-control ratings:

- Y01: Add a pointer from `AGENTS.md` to `tasks/codex-capability.md`.
  `low/high = 99/99`.
- Y02: Edit a skill or `AGENTS.md` passage to suppress a known Codex failure
  mode. Success means Jorn only reviews and accepts or rejects the final
  proposal once. `low/high = 10/30`.
- Y02': Edit a skill or `AGENTS.md` passage to suppress a known Codex failure
  mode. Success allows Jorn to answer two questionnaire messages first, while
  skipping high-effort questions as `dunno`. `low/high = 20/60`.
- Y03: Review whether a proposed harness instruction is likely to help or cause
  side effects. `low/high = 03/03`.
- Y04: Decide when this capability file is mature enough to become mandatory.
  `low/high = 10/10`.
- Y05: Given a proposed task, identify likely Codex failure modes before
  starting. Success means the list of at-least-plausible errors contains all
  errors that then happen and the list is at most four times as long as that.
  `low/high = 15/25`.
- Y06: Turn likely failure modes into prompt constraints, if Jorn literally
  asks for constraints. `low/high = 01/01`.
- Y06': Turn likely failure modes into prompt edits, if Jorn asks only for
  prompt edits. `low/high = 02/10`.
- Y07: Decide whether a task should be abandoned or restarted instead of
  rescued. `low/high = 30/60`.
- Y08: Decide whether a result is useful enough to merge despite known cleanup
  cost. `low/high = 30/60`.

External report note:

- `/tmp/chatgpt-deepresearch-2026-05-18.md` exists and contains the ChatGPT Pro
  deep-research report. Jorn suggests not reading or integrating it while this
  capability-rating discussion is midstream. It is in German for unknown
  reasons.

## Processed Jorn Experience Notes

This section is an agent-processed layer derived from the raw chat intake above.
It is useful for scanning, but it is not the raw source of truth. If a sentence
here appears to conflict with the raw intake, trust the raw intake and revise
this section.

These rows intentionally preserve concrete conditions. Do not compress them
into broad "Codex can do X" claims without keeping the caveats.

### Spec-To-Code Loop

Source type: `Jorn aggregate experience`.

Jorn reports that Codex can be useful when given a specification of what code
must achieve, and when the task supports a full loop:

- implement tests that capture the specification
- implement code satisfying the tests
- review code and tests for semantic meaning
- run test runners, linters, type checks, and tracing
- inspect tool output instead of relying on prose confidence

The specification must include motivation, downstream use, and a verifiable end
state. Example target shape: a working, proven-correct mapping-to-math
implementation of the HK2017 unpruned search that is correct, verifiably so,
clear, understandable, documented, idiomatic, predictable for new agents, low
cognitive load, maintainable, and connected to thesis success through trusted
capacity values.

Do not generalize this to underspecified coding tasks. The useful pattern is
not "Codex is good at code"; it is that code work becomes more reliable when
external checks and semantic review constrain the work.

### Brainstorming Against Established Desiderata

Source type: `Jorn aggregate experience`.

Codex can be useful for brainstorming and assessing ideas when the desiderata
are already established. Relevant desiderata include:

- predictable and common patterns
- feasible within one agent session
- predicted under ten minutes of Jorn time including review before merge
- solves the actual problem
- ties into downstream context
- documented and verifiable
- gives an early signal whether to pivot

This does not mean Codex can reliably discover the right objective from
scratch.

### Repo Information Transfer

Source type: `Jorn aggregate experience`.

Codex can gather information from the repo and report it to Jorn. The useful
output is knowledge transfer: answer a concrete question, solve a bounded
information problem, or provide a mechanistic explanation of a topic.

The agent should gather cheap repo information before asking Jorn. It should
also state what it already knows, so Jorn does not spend time typing information
the agent could have found.

### Maintenance After Known Events

Source type: `Jorn aggregate experience`.

Codex can maintain task files and comments after a session or recent commits:
update stale information, record what happened, and route follow-up work.

The task is safer when the triggering event is known. It is less safe when the
agent invents a new global organization or silently changes steering meaning.

### Proof Writing With Adversarial Review

Source type: `Jorn aggregate experience`.

Codex may be asked to prove a lemma and write a rigorous proof in enough detail
that an independent review agent can verify it. The review agent should be
non-forked when independence matters, with no spoilers, anchors, or misleading
instructions.

Review agents should not be expected to repair ambiguity or close non-obvious
gaps. A review finding means "not obvious to the reviewer that this is right,"
not necessarily "false." The proof writer owns making the proof clear enough
to verify.

Do not use this as evidence that Codex can autonomously close hard theorem
routes under deadline pressure.

### Exploratory Formalization And Error Bounds

Source type: `Jorn aggregate experience`.

Codex can be asked to try different formalizations, error-bound formulas, or
admissible case conditions for numerics. The output should compare what is
most useful while still empirically correct, proven correct, and useful
downstream. Useful means neither too large an error nor too many rejections
from overly restrictive conditions.

This requires strong checks. A plausible formula is not enough.

### Session Postmortem

Source type: `Jorn aggregate experience`.

Codex can report what happened in a session, where labor was wasted, where
friction occurred, and where shortcuts could have been taken in hindsight.

The output should err toward listing more items while marking epistemic status,
confidence, and impact size.

## Bad Habits And Failure Modes

### Self-Capability Reporting Is Unreliable

Source type: `Jorn aggregate experience`; `current-chat evidence`.

Prompt shape: "Can you do X?"

Observed problem: Codex gives wildly uncalibrated and inaccurate answers. It
acts as if it has a reliable source of truth for its own future performance
when it does not.

Planning implication: do not use model self-report as capability evidence. If
there is no registry row or project evidence, label the task as unknown,
trial-only, or Jorn-gated.

### Explanations Of Own Failure Are Unreliable

Source type: `Jorn aggregate experience`.

Prompt shape: "In this session X went wrong, why?"

Observed problem: Codex makes plausible-sounding claims about internal
processes. It cannot inspect past hidden chain-of-thought or evaluate
counterfactual prompts unless those experiments are actually run.

Planning implication: failure analysis should state observable facts, plausible
hypotheses, and missing counterfactual evidence separately.

### Objective Discovery Operationalizes Too Early

Source type: `Jorn aggregate experience`.

Prompt shape: "Figure out what objective we want to achieve."

Observed problem: Codex may return an early operationalization that is useless
for thesis success, hard to achieve if taken seriously, and easy to game if
taken literally.

Planning implication: for high-level objectives, Codex should gather context
and ask Jorn concrete questions rather than freeze a premature target.

### Planning Fails When Success Is Hard To Recognize

Source type: `Jorn aggregate experience`.

Prompt shape: "Plan how to achieve X" where agents have difficulty telling
whether X was achieved.

Observed problem: Codex may write the first idea that comes to mind without
justifying why it is the best plan. Repeated retries can happen one by one
without a useful signal guiding them.

Planning implication: do not schedule work whose success signal Codex cannot
recognize. First define a robust signal or route the decision to Jorn.

### Feedback-Signal Selection Is Uncalibrated

Source type: `Jorn aggregate experience`.

Prompt shape: "Select which idea provides the most robust feedback signal."

Observed problem: Codex predictions are uncalibrated. It may pick a signal that
is easy to game or not semantically robust. If used, the agent may immediately
game the signal.

Planning implication: feedback signals need adversarial review, semantic
meaning, and gameability checks.

### Post-Hoc Justification

Source type: `Jorn aggregate experience`.

Prompt shape: "Justify your edits."

Observed problem: Codex may invent post-hoc justifications instead of selecting
edits for good reasons. It may also fail to revert edits whose justification is
weak.

Planning implication: require justification before or during edit selection for
risky edits. If the justification fails, avoid or revert the edit.

### Prose Quality Is Unreliable

Source type: `Jorn aggregate experience`.

Prompt shape: "Write prose that sounds well."

Observed problem: output can be ambiguous, unclear, hard to follow, both over-
and under-specific, hallucinated, missing useful narrative, superfluous,
awkwardly complex, low in transmitted information, or poorly connected.

Review can catch some classes of errors, especially if explicitly listed. A
review finding of ten issues does not mean there are only ten. A review finding
of zero issues is some evidence that nothing obvious remains. Review-and-fix
until no findings remain is more useful than one-shot prose review.

### Subagent Prompting And Harness Work Are Hazardous

Source type: `Jorn aggregate experience`.

Codex can write subagent prompts or harness material only when pitfalls are
explicit. Common pitfalls include:

- forgetting to mention worktree requirements
- saying "focus on X" while expecting attention to Y
- omitting context needed to interpret the goal
- using forked context when independent review is required
- giving review agents spoilers or anchors

Harness engineering works best when there is a strong signal from retries:
Jorn tried a prompt, pitfall X occurred, the prompt was fixed, then pitfall Y
occurred.

Hot-fixing derailed sessions is often beyond the derailed agent, an external
agent, and sometimes Jorn. It can be better to abandon the worktree, extract
learning with caveats, and start a new agent with a modified prompt.

## Repo-Local Evidence

### Smooth-Branch First-Order Failure Pattern

Source type: `repo-local evidence`.

`tasks/sys-first-order.md` records a stale failed route: a removed temporary
`sys-first-order-chapter.pdf` used a `C^2` branch hypothesis as the main
theorem. The task bundle says not to use that route as source truth because it
assumes away arbitrary-polytope degeneracies.

Capability implication: hard theorem work is vulnerable to easier-theorem
substitution. Agents may prepare generic cases, proof surfaces, or blocker
reports, but must not present a narrower smooth result as solving the intended
general theorem.

### Final Thesis-Done Gate Is Jorn-Gated

Source type: `repo-local evidence`.

`tasks/verify-thesis-done.md` requires explicit Jorn final acceptance before
declaring the thesis done.

Capability implication: agents may prepare final checks, but cannot decide
final thesis readiness.

## Current-Chat Evidence

### Unfounded Capability Assumption

Source type: `current-chat evidence`.

In this chat, Codex initially spoke as if there were a source of truth for what
Codex can autonomously accomplish. Jorn challenged this. The corrected state is
that no reliable global source existed yet.

Planning implication: future agents should not present Codex capability
assumptions as facts unless they cite this file, project evidence, external
evidence, or Jorn's explicit judgment. If no matching row exists, the task must
be treated as unknown or trial-only.

### Premature Abstraction

Source type: `current-chat evidence`.

Codex compressed Jorn's concrete observations into abstract task classes too
early. Jorn objected that this triggers the same failure mode the artifact is
supposed to suppress: glossing over conditions and elevating hypotheses that
make Codex sound more capable than the evidence supports.

Planning implication: preserve raw observations first. Add abstractions only
in a separate layer and only with caveats.

## Provisional Capability Profile

This table is intentionally conservative. It should be revised when the
deep-research result and more project-local evidence are added.

| Task pattern | Current answer | Source strength | Deadline use | Required controls |
| --- | --- | --- | --- | --- |
| Spec-to-code loop with tests, semantic review, and tool checks | useful under the listed conditions | Jorn aggregate experience | acceptable when bounded | verifiable end state; tests; lint/type/tool output; semantic review |
| Repo information transfer for a concrete question | useful | Jorn aggregate experience | acceptable | read first; report source paths; separate fact from inference |
| Maintenance after known events | useful with review | Jorn aggregate experience | acceptable | preserve steering meaning; avoid new global organization |
| Brainstorming against established desiderata | useful when desiderata are given | Jorn aggregate experience | acceptable as input, not decision | list desiderata; record rejected ideas; do not invent final objective |
| Bounded thesis prose drafting | unreliable unless tightly controlled | Jorn aggregate experience | trial-only | explicit claim boundary; source support; review-and-fix loop |
| Hard theorem closure | not schedulable as autonomous dependency | repo-local failure pattern; Jorn experience | Jorn-gated | prepare lemmas/blockers only; independent adversarial review |
| Final thesis readiness or scope acceptance | not agent-decidable | repo-local policy | Jorn-gated | decision packet only |
| Self-capability prediction | unreliable | Jorn aggregate experience; current chat | do not use | require external evidence or label unknown |
| Explaining own internal failure cause | unreliable | Jorn aggregate experience | hypothesis only | observable facts first; no hidden-process claims |
| Planning where success is hard to recognize | high risk | Jorn aggregate experience | avoid until signal exists | define robust signal or ask Jorn |
| Subagent prompts and harness work | hazardous but can improve through retries | Jorn aggregate experience | use only with explicit pitfalls | mention worktrees, independence, context, and review constraints |

## Agent Planning Protocol

Before proposing or executing thesis-closeout work, an agent should:

1. Identify the closest task pattern in this file.
2. If no row matches, say that capability is unknown.
3. State whether the work is repeatable, trial-only, Jorn-gated, or avoid.
4. Name the controls needed to compensate for known failure modes.
5. Avoid plans that depend on future autonomous success at a task class marked
   unknown, trial-only, or Jorn-gated.
6. If asking Jorn, state what the agent already checked and make "dunno" a
   cheap acceptable answer.

## Open Inputs

- ChatGPT Pro deep-research result on GPT-5.5/Codex capability, regressions,
  calibration, sycophancy, help-seeking, and agentic benchmarks.
- More Jorn aggregate observations if they become available cheaply.
- Repo-local session evidence when a future session clearly succeeds or fails
  in a way that should change planning.
