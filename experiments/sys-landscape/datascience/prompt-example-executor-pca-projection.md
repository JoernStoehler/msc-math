# Prompt Example: PCA Projection Executor

This is the executor prompt used as the source pattern for the current
`pca-projection` method packet. It is an example, not a template. For another
method row, adapt the goal, source-truth files, method folder, validity guard,
and expected evidence instead of mechanically replacing names.

```text
Work in `/workspaces/msc-math/.worktrees/ds-pca-projection`, branched from
`ds-method-integration`.

Create a fresh PCA projection packet for the sys-landscape data-science method
table, or give a clear reason why PCA should be abandoned or deferred for this
dataset/search interface.

Method question:

> Does a low-dimensional linear projection of allowed retained dataset columns
> suggest a reproducible rule for proposing high-`sys` candidate polytopes?

The useful output is a committed method packet that satisfies the method packet
success standard in `experiments/sys-landscape/datascience/README.md`. A PCA
run is successful if the packet is usable as current method-table evidence,
even when the row does not end negatively.

Read these first:

- `experiments/sys-landscape/datascience/README.md`
- `experiments/sys-landscape/datascience/dataset/README.md`
- `experiments/sys-landscape/datascience/tables/README.md`
- `experiments/sys-landscape/datascience/methods/README.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/report.md`

Use the retained dataset at:

`experiments/sys-landscape/datascience/dataset/`

Do not edit `dataset/`, `produce/`, or `tables/`.

Own this method folder:

`experiments/sys-landscape/datascience/methods/pca-projection/`

Add a small script and any small method-owned artifacts needed for review or
reproduction. Replace `report.md` with the final report.

The report must keep this YAML header shape:

---
method: pca-projection
description: >
  Few-sentence method description.
result: >
  Few-sentence outcome/status.
---

The report body structure is your decision. A reviewer must be able to check:

- dataset path and fingerprint facts;
- command and runtime;
- included and excluded input columns;
- validity guard;
- observation;
- inference;
- terminal state;
- thesis use;
- reopen condition.

A candidate-proposer must propose candidate polytopes or rows before their
`sys` values are evaluated.

Do not use these as candidate-proposer inputs:

- `sys`;
- capacity columns;
- endpoint labels;
- dataset identity;
- optimizer provenance;
- post-hoc inspection of `sys`.

You may use `sys` after fitting only for audit and interpretation. Label that
use as observation or audit, not as part of the proposing rule.

When the analysis discusses a PCA region, subgroup, slice, model output, or
rule output, choose and state an explicit baseline for high-`sys` tail
concentration when that comparison is meaningful. For same-dataset subgroup
tail capture, the row-fraction baseline is the cheap default. Do this before
treating "no candidate-proposer" or "no validated row" as the row's main
takeaway.

Be explicit about geometric columns versus metadata/provenance columns. Do not
make impossibility, density, or exhaustive-search claims.

`experiments/sys-landscape/datascience/methods/pca-cluster-anomaly/` is stale
source material, not current evidence. Do not rerun it by default. Inspect it
only if it has concrete value for this fresh PCA packet. If you extract an idea
or code, say what you extracted and why in the report or final summary.
Otherwise leave it alone.

Design the PCA analysis yourself. Use standard data-science libraries where
they fit, but choose the library based on the analysis rather than a rule in
this prompt.

Use a smoke run before the full retained-dataset run. Experiments over 30
minutes are not accepted as local reproducible method evidence. If the best PCA
route appears to require more than 30 minutes, record that as a compute/data
bound instead of claiming current evidence.

Keep the analysis narrow enough that the reviewer can decide whether the PCA
row is done.

Before finishing, run the report reproduction command, or explain why no
accepted current command exists. Run a cheap syntax/check command for new
Python code. Check `git diff --check`.

Commit the method packet on the method branch before handing it back. The
reviewers should be able to review a committed packet diff, not reconstruct
which loose working-tree files belong to the method result.

Final response:

- commit hash;
- changed files;
- commands run and runtime if measured;
- recommended terminal state for the PCA row;
- whether old `pca-cluster-anomaly/` material was used, and why;
- what the reviewer should focus on.
```
