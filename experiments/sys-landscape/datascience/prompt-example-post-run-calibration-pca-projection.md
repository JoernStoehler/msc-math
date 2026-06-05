# Prompt Example: PCA Projection Post-Run Calibration

This is a follow-up prompt for an executor or reviewer after it has already
finished its assigned PCA packet task. It is an example, not a template.

Do not include this in the initial task prompt. Sending it afterward helps the
orchestrator learn which parts of the prompt, README files, or repo structure
were unclear without priming the agent to optimize for this feedback question.

```text
Now that you have finished, give a short calibration note for the orchestrator.
Do not edit files unless you discover a concrete blocker that invalidates your
result.

The goal is to improve the method-packet orchestration flow for long-term
thesis success. The current README files may provide enough context, but this
is hard to know without asking agents after real runs.

State:

- what in the initial prompt, README files, or repo structure was unclear or
  under-specified;
- where you deviated from the initial prompt because the actual repo/task made
  a better path obvious;
- what goal or context should be changed in the prompt for the next method
  packet to better serve long-term thesis success;
- which missing context, if any, would have changed your work product;
- whether your final answer is still reliable after this reflection.
```
