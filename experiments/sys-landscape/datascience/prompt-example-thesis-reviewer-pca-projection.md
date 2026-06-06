# Prompt Example: PCA Projection Thesis Reviewer

This is an example for reviewing whether technically plausible method-row work
helps the thesis result. It is intentionally broader and less checklist-shaped
than the technical review prompt. For another method row, adapt the method
question and nearby row interactions.

```text
Review the PCA projection work commit in:

`/workspaces/msc-math/.worktrees/ds-pca-projection`

Do not edit files. Do not rely on executor context. Treat the committed diff,
files, and any commands you run as the evidence. If the worktree is dirty,
review the committed diff unless the dirty state affects interpretation or
makes the committed work ambiguous.

Goal:

Report whether the PCA work should merge into the data-science integration
branch for thesis success. The orchestrator will use your findings to decide
whether to merge, request repair, split follow-up, defer, abandon, escalate,
or update approved status in `methods/STATUS.md`.

Context:

- The data-science thesis result is a closed method table.
- Useful method-row work should help decide whether the method contributes
  positive evidence, negative evidence, a candidate-proposer, a validated row,
  or a reason to abandon or defer the row.
- The work should reduce future Jörn/agent work. It should not preserve stale
  structure, create false closure, or make thesis writing easier to get wrong.
- Technically correct work can still fail if it answers the wrong question,
  closes the wrong row, misses a cheap high-value analysis, weakens the
  narrative, or invites a future thesis-writing agent to overclaim.
- For each material issue, make clear whether the work should merge, be
  repaired, be split into follow-up work, be deferred, be abandoned, or be
  escalated. Salvage value is not enough for integration.

Source files likely needed:

- `experiments/sys-landscape/datascience/README.md`
- `experiments/sys-landscape/datascience/methods/README.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/report.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/analyze.py`
- `experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json`

The review is good if it establishes whether this work is the right
thesis-useful artifact, not merely whether it is internally coherent. Evidence
that often matters for this work includes:

- what thesis success requires from the PCA row;
- whether the report answers the PCA method question rather than merely reports
  numbers;
- whether the negative result is calibrated enough for the closed method table;
- whether any PCA region, subgroup, slice, model output, or rule output has
  high-`sys` tail concentration relative to a stated baseline before "no
  candidate-proposer" or "no validated row" is treated as the row's main
  takeaway;
- whether leakage, post-hoc selection, and overclaiming are controlled well
  enough for thesis interpretation;
- whether a cheap missing analysis would materially change the row's thesis
  value;
- whether the work interacts badly with nearby clustering/anomaly rows;
- whether the work should merge as current evidence or a status record, be
  revised, expanded, split, renamed, abandoned, or replaced by different
  PCA-style work;
- whether `methods/README.md` helps a future thesis-writing agent use the row
  correctly;
- whether the work reduces future Jörn/agent time or creates confusing
  maintenance burden.

Your review may recommend a status or decision, but it does not approve
method-row status. Approved status is orchestrator-owned and recorded in
`methods/STATUS.md`. A green review is evidence only for the checks recorded in
your findings and commands, not proof that all relevant questions are answered.

Output findings first, ordered by importance. Then give:

- confidence percentages where useful;
- whether any issue blocks integration;
- what status update, if any, your findings support and what you actually
  checked;
- what would need to change before merge, if anything;
- what could be nonblocking follow-up after merge;
- which commit you reviewed;
- commands you ran, if any.
```
