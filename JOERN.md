# JOERN.md

Prompt snippets I may paste into Codex sessions.

This is a paste shelf, not project source truth, not a task queue, and not a
replacement for `AGENTS.md`. I usually paste only the code block. If an agent
reads this whole file, no snippet is active unless I pasted or named it in the
current chat.

Each entry has a visible `Use when` condition, a pasteable prompt, and a short
`Replaces` note explaining the failure pattern.

## Anti-Habits

Current working list, not exhaustive. These are recurring Jörn/agent interaction
patterns to notice. Some are targeted by snippets below; some may need different
handling.

- I keep adding explanations inside a chat whose main state is already repair.
  The useful project facts may be real, but they get mixed with recovery noise.
- I give examples or possible methods as evidence and idea generators. Agents
  often treat the concrete examples as scope, sequence, or permission.
- I answer repo-state questions that were locally inspectable. This spends
  Jörn attention and can hide the agent's missing local context.
- I give a correction, narrowing, or contradiction, and the next answer keeps
  serving the older plan.
- A surprising result appears, and the agent changes route without first making
  the changed state explicit.
- An answer leaves me inferring whether work is complete, blocked, waiting for
  me, or continuing.
- I am asked to review a broad pile that still contains things the agent could
  have filtered.
- A proxy check, benchmark, review pass, or artifact starts being discussed as
  if it settled the actual target.
- The agent dumps rough options, self-analysis, or abandoned ideas into chat,
  and I do the filtering work.
- One request, file, plan, command block, or session carries several purposes
  before the coupling has proven useful.
- I want calibration or problem modeling, but the agent turns that into
  implementation planning or approval-seeking.

Project-wide chat and autonomy rules live in `AGENTS.md`. The snippets below are
explicit Jörn steering when pasted; they are not active project instructions by
themselves. Removed GPT-5.5 planning and recovery controls remain in the
pre-GPT-5.6 harness commit named by `$gpt-56-harness`.

## Start Fresh

Use when: this chat has spent more effort on repair than project work, and I
want a clean restart with only reusable project state.

```text
This session is derailed. I am starting a fresh session from your next answer.

Extract only reusable project facts, decisions, answers I gave, source paths
mentioned, and agent-produced findings needed to make those facts usable.

Exclude process repair, apologies, self-analysis, attempted recovery, and plans
for continuing this session. Do not continue the task. Output a compact handoff
for a fresh agent.
```

Replaces: continuing to explain project facts inside a session that will not
continue the work.

## Check Locally First

Use when: I am being asked for file locations, repo state, source-backed facts,
or broad feedback that likely has local evidence.

```text
The question/request above looks locally answerable or under-specified.

Check the smallest relevant local/repo source first. Ask me only for judgment,
access, or facts not reasonably available locally.

If you still ask me, state: sources checked; remaining uncertainty; what
decision my answer changes; answer shape that helps.
```

Replaces: asking me to compare files, remember repo state, or answer broad
feedback questions before local inspection.

## Stay in Calibration

Use when: I want problem modeling, value/scope/epistemics calibration, or target
clarification, not edits or an implementation plan.

```text
Calibration only. Do not edit files or propose a committed implementation plan.

Give one compact problem model: target; main uncertainty; candidate decisions;
evidence that would change the next step. End with at most one focused question,
or say what agent-side inspection should happen next.
```

Replaces: treating discussion as "write a plan and wait for approval."

## Examples Are Not Scope

Use when: I gave examples, possible methods, possible prompts, or possible
failure modes, and those examples may be treated as scope or as a plan.

```text
Treat my examples as evidence and idea generators, not as a whitelist, final
scope, or plan.

State the broader question the examples illuminate. Separate what I explicitly
asked for from candidate actions you infer. If an example seems like the best
next action, say why it wins over nearby alternatives; do not pick it merely
because it is concrete or first-listed.
```

Replaces: implementing the first concrete example while losing the broader
question or value model.

## Follow Latest Message

Use when: I corrected, narrowed, or contradicted an earlier request, or the
current/last answer is still on the old task.

```text
Re-anchor on my latest substantive instruction before this prompt. Treat that
instruction as controlling over the earlier plan unless it explicitly says
otherwise.

State what changed, what prior work remains valid, what you will stop doing,
and the next action under the updated request. If there is a real conflict that
blocks action, ask one focused question.
```

Replaces: finishing the previous plan after I already changed the task.

## Reset After Surprise

Use when: a failed checkout, suspect branch, invalidated assumption, surprising
result, or repeated strategy switch changed the expected path.

```text
Stop before changing path. Reconstruct current state in 3-6 lines: objective;
surprise or blocker; normal path; evidence needed to change path; next bounded
action.

Then continue only if that action follows from the reconstruction. Otherwise
ask one focused question.
```

Replaces: switching branch, execution surface, or recovery strategy just because
the first path became awkward.

## Command Handoff

Use when: I need to operate an elevated resource such as LICCA, the devcontainer
CLI, mail, or another Codex session.

```text
Turn the command request above into a bounded handoff.

First state why this is not accessible local/repo work you can do yourself. If
this is LICCA, use the LICCA rules/skill before writing commands.

Then give: execution context; purpose; preconditions; one command block for one
purpose; expected output; stop condition; what I should paste back.
```

Replaces: broad command piles where I must infer what is safe, what output
matters, and when to stop.

## Done or Checkpoint

Use when: I received a final/status answer, or I asked for a wrap-up, and it is
unclear whether the work is complete, blocked, waiting for review, or merely at
a checkpoint.

```text
Clarify whether this is completion or checkpoint.

State: completed work; validation/review performed; remaining out-of-scope
work; whether you need my review, my answer, or no action from me. If this is a
checkpoint, state the next action instead of calling it done.
```

Replaces: ending with an artifact or status line that makes me infer the next
state.

## Narrow Review

Use when: I see a review request for a broad artifact, many questions, an
unfiltered draft, a known-bad pile, or an unclear target.

```text
Narrow the review request above.

State what is already checked, what remains uncertain, and the smallest
decision, passage, or premise where my feedback changes the next action. If the
artifact is broad or unfiltered, continue agent-side filtering instead of
asking me to review it.
```

Replaces: asking me to sort a pile the agent can reduce.

## Target vs Proxy

Use when: an experiment, heuristic, proof attempt, benchmark, or review is being
used as thesis evidence.

```text
Separate target, proxy, gap, and decision.

State the target claim we care about; the proxy/check currently available; why
the proxy is informative; where it could fail; and what decision changes if the
proxy succeeds or fails. Do not report proxy success as target success.
```

Replaces: saying a method is "better", "validated", or "trustworthy" without
naming the target claim and remaining gap.

## Draft First

Use when: the next answer needs drafting/filtering, or the last answer was a
rough idea dump that I had to read, filter, and repair.

```text
For this answer, use /tmp to draft and revise before replying.

In chat, send only the polished result, or a short pointer to the artifact plus
the exact deltas/questions I need to inspect. Do not dump brainstorming,
abandoned options, or process repair unless I asked for them.
```

Replaces: treating rough output as "high bandwidth" because it contains many
ideas.

## Split Concerns

Use when: a proposed file, command block, plan, experiment, or question serves
several purposes and the coupling is not clearly useful.

```text
Separate the concerns before continuing.

Name the distinct purposes in the current proposal. Pick the one purpose that
should be handled now, or explain why keeping them bundled is actually useful.
If they should split, say what goes into each artifact, command block, session,
or question.
```

Replaces: one file, command block, plan, or question that mixes project facts,
process repair, implementation, evaluation, and review request.

## Related Files

- `AGENTS.md`: project-wide agent rules.
