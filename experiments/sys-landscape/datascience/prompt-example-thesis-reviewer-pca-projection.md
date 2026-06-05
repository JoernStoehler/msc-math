# Prompt Example: PCA Projection Thesis Reviewer

This is an example for reviewing whether a technically plausible method packet
helps the thesis result. It is intentionally broader and less checklist-shaped
than the technical review prompt. For another method row, adapt the method
question and nearby row interactions.

```text
Review the PCA projection packet commit in:

`/workspaces/msc-math/.worktrees/ds-pca-projection`

Do not edit files. Do not rely on executor context. Treat the committed packet
diff, files, and any commands you run as the evidence. If the worktree is
dirty, review the committed packet diff unless the dirty state affects
interpretation or makes the committed packet ambiguous.

Goal:

Report whether the PCA packet is directly usable as current method-table
evidence for the data-science thesis result. The orchestrator will use your
findings to decide acceptance.

Context:

- The data-science thesis result is a closed method table.
- A useful method packet should help decide whether the method contributes
  positive evidence, negative evidence, a candidate-proposer, a validated row,
  or a reason to abandon or defer the row.
- A committed method packet is successful when it can be used as current
  method-table evidence without rerunning the experiment, repairing the
  analysis, further interpreting the result, or inspecting stale files.
- The packet should reduce future Jörn/agent work. It should not preserve stale
  structure or make thesis writing easier to get wrong.
- A technically correct packet can still fail if it answers the wrong question,
  closes the wrong row, misses a cheap high-value analysis, weakens the
  narrative, or invites a future thesis-writing agent to overclaim.
- For each material issue, make clear whether the packet is still directly
  usable as current method-table evidence with ordinary thesis-writing work
  remaining, or whether it needs substantial repair, reinterpretation, rerun, or
  archaeology before the row can be trusted. Salvage value is not enough for
  current evidence.

Source files likely needed:

- `experiments/sys-landscape/datascience/README.md`
- `experiments/sys-landscape/datascience/methods/README.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/report.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/analyze.py`
- `experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json`

The review is good if it establishes whether this packet is the right
thesis-useful artifact, not merely whether it is internally coherent. Evidence
that often matters for this packet includes:

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
- whether the packet interacts badly with nearby clustering/anomaly rows;
- whether the row should instead be accepted, revised, expanded, split,
  renamed, abandoned, or replaced by a different PCA-style packet;
- whether `methods/README.md` helps a future thesis-writing agent use the row
  correctly;
- whether the packet reduces future Jörn/agent time or creates confusing
  maintenance burden.

Output findings first, ordered by importance. Then give:

- confidence percentages where useful;
- whether any issue blocks direct use as current method-table evidence;
- what would need to change before direct use, if anything;
- what could be follow-up after direct use;
- which commit you reviewed;
- commands you ran, if any.
```
