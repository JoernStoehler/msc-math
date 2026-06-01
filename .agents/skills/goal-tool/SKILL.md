---
name: goal-tool
description: Use before creating, updating, checkpointing, or completing `/goal`.
---

# /goal Tool

## Bad

- Replacing the real objective with a rewritten `/goal` objective can lead to failure of the real objective.
- Rewriting/rephrasing the objective introduces drift, which again can lead to failure.
- Omitting trade-off considerations and other forms of context makes the interpretation of the written objective unstable in long sessions, and so in long sessions the real objective can be failed.
- Underspecified or implicit objectives where the executing agent is allowed to pick the interpretation drift toward the laziest interpretation over time often fail the implicit real objective due to the pressure to finish the goal.
- Claims without epistemic status have ambiguous interpretation with regard to how much to question them, and so mere ideas can be misinterpreted as must-have constraints, or guesses can become assumed facts that propagate into future sessions via the repo.
- Using the `/goal` tool creates a strong pressure towards conciseness, and does not allow easily editing out any imperfections, both of which trade off against conveying the real objective for no gain.

## Good

- Put the real objective in a charter file, which does not have any pressure towards conciseness and which is trivial to edit/iterate on/review-in-advance etc. Make `/goal` point to that charter file.
- Emphasize strong conditions for success in the charter and the `/goal` snippet.
- The charter preserves the real objective and the context needed for interpretation and properly marks epistemic status to not confuse necessary constraints with strongly expected properties with mere guesses.
- The charter fleshes out stopping conditions that are robust to interpretation drift, which can come from the pressure to finish quickly or from boisterous overconfidence.
- The charter does not try to plan ahead-of-time or break down the goal into easier checks or constrain the process by which the goal is achieved, instead it focuses on making the real objective explicit and robust. During execution / separately from charter-writing the agent will explore and plan and implement on the fly. The `/goal` loop just has to ensure that the agent does not hand in a failing deliverable without realizing it.
- Part of robustness is to clarify that subagent or programmatic review steps need to actually pass, as otherwise agents often cut corners and merely amend the deliverable after a failing review step without re-running the review, and thereby they miss new or overlooked issues and fail the objective.

- Example of a `/goal` snippet:
  ```text
  Execute the objective charter at <path>. Mark complete only under the charter's stopping conditions.
  ```

- Jörn cannot respond to questions while the loop runs, you can use, e.g., the status `blocked` if you need to pause and ask questions.
- Ask upfront, before starting the loop, whether Jörn has any feedback / corrections / suggestions for additions for the charter. It's often easier to fix the objective in advance and catch misunderstandings than to re-run the loop from scratch after Jörn notices the deliverables fail the real objective hard.

- The only purpose of `/goal` is to later recall the objective without any loss.
