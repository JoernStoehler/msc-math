# Recurring Agent Feedback 2026-06-01

Status: dated reference for future agents. Not source truth. Not a task queue.

Purpose: record repeated Jörn feedback that has occurred across many agent
sessions, so Jörn can refer future agents here instead of retyping the same
corrections.

Use: when Jörn says to read recurring agent feedback, or when work involves
planning, roadmap writing, assumption audits, user-review requests, delegation,
or multi-step thesis-success work.

This file does not replace `AGENTS.md`, task files, skills, source truth, or
Jörn's current message. Current instructions in chat overrule this dated
reference.

## Short Chat Preamble

Jörn can paste this into a future chat:

```text
Before you answer, read
`tasks/references/recurring-agent-feedback-2026-06-01.md`.
Apply it to this task. In particular: do not make me infer what you want
reviewed, do not stop after an intermediate artifact unless you state the next
action, do not ask me questions you can answer from the repo, and do ask a
focused question when high-confidence human/context input is actually needed.
```

Even shorter version:

```text
Read `tasks/references/recurring-agent-feedback-2026-06-01.md` first and obey
the Short Chat Preamble there.
```

## Repeated Failure Pattern

Agents repeatedly produce plausible-looking progress while shifting hidden work
to Jörn. The visible symptom is that Jörn has to reread the agent's message
several times to infer:

- what is ready for review;
- whether the agent expects a reply;
- what the next action is;
- which premise is a source-backed fact and which is a guess;
- whether weak old agent reasoning was silently promoted into an assumption;
- whether a missing high-confidence fact should be asked about or researched.

This is not a tone issue. It consumes the bottleneck resource: Jörn attention.

## Required Behavior

1. State the review target explicitly.
   - Good: "Please review only item 3."
   - Good: "No review needed; I am continuing with X."
   - Bad: ending with an artifact and making Jörn infer whether it needs
     review.

2. Do not stop after an intermediate artifact unless the stop condition is
   explicit.
   - State one of:
     - "This is complete; no review needed."
     - "Please review item X."
     - "I will continue with next step Y."
     - "I need answer Z before continuing."

3. Ask focused questions only when needed.
   - First inspect repo and official sources.
   - Ask Jörn only when the answer cannot be obtained there and changes the
     next action or claim strength.
   - Say the current default if unanswered.
   - Say why the answer changes the next action.

4. Do ask when high-confidence human/context input is actually needed.
   - Do not guess or hide uncertainty because asking feels costly.
   - If a premise needs high confidence and source evidence is weak, mark it as
     weak and ask a focused question or name the blocker.

5. Separate evidence strength.
   - Distinguish source-backed facts, inferences, guesses, stale-check-needed
     claims, Jörn judgments, Kai/advisor context, and official-source facts.
   - Do not treat task files, maps, old planning notes, old GPT reasoning, or
     legacy thesis prose as proof. Use them to find source truth.

6. Avoid local closure.
   - Producing a file, prompt, list, or draft is not automatically task
     completion.
   - Check whether the artifact advances the session goal and whether the next
     action is still yours.

7. Use review before presenting high-level planning as usable.
   - For high-level planning, run sanity, reasoning/completeness, and
     clarity/epistemics review.
   - Use subagents where available and proportionate, especially for
     clarity/style/epistemics.
   - Report review passes and findings.

8. Keep output easy to respond to.
   - Number items.
   - Use plain language.
   - Avoid bundling multiple assumptions into one item.
   - Avoid explanations that make Jörn infer the actionable request.

9. Do not repeatedly ask Jörn to reclassify already-settled thesis scope.
   - On 2026-06-01 Jörn stated that the current 11 listed thesis content areas
     are all must-have and that he does not want to be bothered again in the
     next weeks to re-answer that classification.
   - Future agents should read `tasks/current-state.md` before asking scope
     questions.
   - Ask only if Jörn's current message or new advisor/source evidence directly
     contradicts that recorded scope.

## Bad Patterns To Avoid

- Asking Jörn to review guessed assumptions before inspecting the repo.
- Explaining what an assumption is instead of producing the requested audit.
- Saying "you are right" and then giving another shallow patch.
- Presenting weak agent-written planning as settled.
- Ending a turn without saying whether Jörn should review, answer, or wait.
- Asking abstract questions whose answer should be derived from source files.
- Not asking when missing Jörn/Kai context is actually required.
- Treating a prompt, checklist, or plan as complete without testing it against
  the failure it was meant to prevent.

## Current Best Default For Planning Tasks

For thesis-success planning, the default first step is not a roadmap. It is an
assumption audit:

1. read the relevant source surfaces;
2. write the decision test;
3. classify premises by evidence type and confidence;
4. identify weak or stale premises;
5. ask only necessary focused questions;
6. run independent review;
7. state whether planning can continue and what the next action is.

## How Future Agents Should Use This

If this reference is relevant, say so briefly:

```text
I read the recurring feedback reference. I will state review targets and next
actions explicitly, separate evidence strength, and ask focused questions only
where repo/source inspection cannot settle a needed premise.
```

Then do the task. Do not turn this reference into a long apology or a second
process artifact.
