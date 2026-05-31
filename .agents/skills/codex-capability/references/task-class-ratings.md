# GPT-5.5 Task-Class Ratings

This file records one elicitation of Jorn's rough success-frequency estimates
for GPT-5.5/Codex task classes in this thesis project.

Source: extracted from
`.agents/skills/codex-capability/references/codex-capability-register.md`,
section `2026-05-18 Concrete Capability Point Ratings`.

The estimates are aggregate experience, not benchmark results. They are
provisional and should update when project experience changes them.

## Elicitation Context

Jorn was asked to estimate what fraction of tasks in each class would work
under different amounts of Jorn effort:

- `auto`: Codex discovers the need for this task during its own work, scopes it
  well enough, and either does it or delegates it without extra Jorn
  intervention.
- `low`: Jorn gives the natural prompt he would write without extra
  failure-mode engineering.
- `high`: Jorn deliberately compensates for predicted Codex failure modes.

High prompting is not a syntax category. It means Jorn deliberately tries to
compensate for predicted failure modes.

The `auto` column was added after the original elicitation and is intentionally
blank until Jorn fills it. As an interim heuristic, an autonomous agent should
usually be treated as at most as reliable as `low`, because it does not have
the expertise Jorn uses when creating `high` prompts.

The numbers are rough percentages.

## Success Meaning

Success means the agent produces a useful packet of work, not merely a literal
interpretation of the prompt.

Examples:

- For "add tests for the volume algorithm", adding a test that checks the
  volume of the unit cube does not advance verifiability of the codebase,
  which is part of thesis success, so it counts as `0.0` successes. Even
  though a literal interpretation of the prompt has been satisfied.
- For "add tests", adding 4 tests that check axioms of the volume function
  but do not cover edge cases, especially numerical edge cases, counts as
  `0.7` successes because it reduces the need for future work but not completely.
- For "add tests", adding 4 axiom tests plus 3 error and rejection paths plus
  a marker / justification for one untested rejection path with no known examples,
  counts as `1.0` successes because it completes the need for test-based
  verification of the volume algorithm.

## Ratings Table

### Code And Math

| Task | Auto | Low | High |
| --- | ---: | ---: | ---: |
| Fix a Rust compile error from compiler output after a rename. |  | 99 | 99 |
| Add tests for an already implemented helper when expected behavior is given. |  | 80 | 90 |
| Implement a Rust helper for a formula from a specific LaTeX lemma, plus tests. |  | 50 | 85 |
| Implement a full paper algorithm in Rust, plus documentation and verification path. |  | 15 | 35 |
| Audit Rust code against a formal note; report mismatches only, do not edit code. |  | 75 | 80 |
| Prove a 10-20 line lemma with statement and definitions already fixed. Line count assumes non-well-known definitions are inlined. |  | 80 | 85 |
| Prove a new lemma needed for HKO exact Packet 3. |  | 50 | 80 |
| Review an agent-written proof. List steps that are not obvious to the reviewer. |  | 75 | 85 |
| Repair a proof after a reviewer says one named step does not follow. |  | 85 | 90 |
| Decide whether a theorem-strength thesis claim is justified. |  | 65 | 85 |

### Writing

Writing ratings are high-uncertainty. Jorn suggested re-interviewing about
them after more agent successes/failures. Original target date: 2026-05-25.

| Task | Auto | Low | High |
| --- | ---: | ---: | ---: |
| Produce rough prose that helps Jorn think. |  | 60 | 80 |
| Produce structured prose draft from scaffold comments and linked notes. |  | 30 | 60 |
| Produce thesis-ready local prose for a bounded paragraph. |  | 15 | 30 |
| Produce publication-ready section prose. |  | 05 | 10 |
| Rewrite existing prose for clarity while preserving mathematical meaning. |  | 30 | 80 |
| Review prose for unsupported claims against listed source files. |  | 75 | 90 |
| Review prose for readability, flow, ambiguity, and missing context. |  | 60 | 80 |
| Fix prose after a reviewer lists concrete issues. |  | 75 | 90 |
| Decide final theorem wording or claim strength. Success means Jorn reviews and finds nothing to disagree with. |  | 75 | 80 |
| Decide whether a side result belongs in the thesis. |  | 05 | 30 |

### Experiments, Planning, And Agent Orchestration

| Task | Auto | Low | High |
| --- | ---: | ---: | ---: |
| Run an existing experiment command from a README and summarize output. |  | 95 | 95 |
| Debug a Python analysis script with crash command, stack trace, and script. |  | 95 | 98 |
| Design a small experiment for two concrete hypotheses supplied in the prompt. |  | 60 | 90 |
| Interpret an experiment table and decide what thesis claim it supports. |  | 90 | 90 |
| Decide whether another cluster-scale run is worth waiting for. |  | 50 | 75 |
| Read task files and produce a current-blocker roadmap. |  | 90 | 95 |
| Choose the next task under deadline pressure. |  | 80 | 90 |
| Prepare a decision packet for Jorn. |  | 70 | 95 |
| Write a worker subagent prompt for a bounded task. Success means one attempt, no follow-up or retry needed. |  | 66 | 80 |
| Write a worker subagent prompt for a bounded task. Success allows retries or follow-ups. |  | 80 | 90 |
| Write an independent review-agent prompt. |  | 75 | 95 |
| Rescue a derailed agent after it already made bad edits. |  | 05 | 30 |
| Extract lessons from a failed session and write a better restart prompt. |  | 10 | 15 |
| Append a raw Jorn observation with minimal cleanup. |  | 90 | 95 |
| Convert raw observations into agent rules. |  | 05 | 10 |
| Integrate deep-research findings without rewriting Jorn notes. |  | 90 | 95 |

### Harness And Failure Control

| Task | Auto | Low | High |
| --- | ---: | ---: | ---: |
| Add a pointer from `AGENTS.md` to the capability material. |  | 99 | 99 |
| Edit a skill or `AGENTS.md` passage to suppress a known Codex failure mode. Success means Jorn only accepts/rejects final proposal once. |  | 10 | 30 |
| Edit a skill or `AGENTS.md` passage to suppress a known Codex failure mode. Success allows Jorn to answer two questionnaire messages and say `dunno`. |  | 20 | 60 |
| Review whether a proposed harness instruction will help or cause side effects. |  | 03 | 03 |
| Decide when this capability file is mature enough to become mandatory. |  | 10 | 10 |
| Given a proposed task, identify likely Codex failure modes before starting. Success means all future errors appear in a list at most 4x too long. |  | 15 | 25 |
| Turn likely failure modes into prompt constraints when Jorn literally asks for constraints. |  | 01 | 01 |
| Turn likely failure modes into prompt edits when Jorn asks only for prompt edits. |  | 02 | 10 |
| Decide whether a task should be abandoned or restarted instead of rescued. |  | 30 | 60 |
| Decide whether a result is useful enough to merge despite known cleanup cost. |  | 30 | 60 |
