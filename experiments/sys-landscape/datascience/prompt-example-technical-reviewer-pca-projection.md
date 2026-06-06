# Prompt Example: PCA Projection Technical Reviewer

This is an example for reviewing committed method-row work's internal
consistency, reproducibility, data-science hygiene, and code/artifact quality.
It is not a template. For another method row, adapt the source files and the
method-specific leakage risks.

```text
Review the PCA projection work commit in:

`/workspaces/msc-math/.worktrees/ds-pca-projection`

Do not edit files. Do not rely on executor context. Treat the committed diff,
files, and commands you run as the evidence. If the worktree is dirty, review
the committed diff unless the dirty state affects reproducibility or makes the
committed work ambiguous.

Goal:

Report whether the PCA work is technically reliable and maintainable enough to
support an integration-branch decision. The orchestrator will use your findings
to decide whether to merge, request repair, split follow-up, defer, abandon, or
escalate, and whether to update approved status in `methods/STATUS.md`.

Context:

- The method table is thesis evidence. Technically bad work can falsely
  close a row, waste future agent time, or make thesis writing overtrust a
  broken result.
- Technical reliability includes reproducibility, data-science hygiene,
  leakage control, artifact consistency, code auditability, and alignment
  between the report and method navigation.
- For each material issue, make clear whether the work should merge, be
  repaired, be split into follow-up work, be deferred, be abandoned, or be
  escalated. Salvage value is not enough for integration.
- The reviewer owns the judgment of what technical risks matter. If a concern
  not named here is important, include it.

Source files likely needed:

- `experiments/sys-landscape/datascience/README.md`
- `experiments/sys-landscape/datascience/dataset/README.md`
- `experiments/sys-landscape/datascience/tables/README.md`
- `experiments/sys-landscape/datascience/methods/README.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/report.md`
- `experiments/sys-landscape/datascience/methods/pca-projection/analyze.py`
- `experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json`

The review is good if it establishes whether the work is trustworthy on the
technical merits, not merely whether it satisfies named checks. Evidence that
often matters for this work includes:

- dataset identity and fingerprint;
- reproduction command, runtime, and dependency declaration;
- tracked generated artifacts and whether the report names their purpose;
- included and excluded columns relative to the validity guard;
- leakage risks from `sys`, capacity columns, endpoint labels, dataset
  identity, optimizer provenance, or post-hoc `sys` inspection;
- separation of observation, inference, thesis use, proposed evidence
  classification, and proposed next action;
- consistency between `methods/README.md`, the report, script, and summary;
- code simplicity and auditability;
- whether old `pca-cluster-anomaly/` material contaminates the current work.

Your review may recommend a status or decision, but it does not approve
method-row status. Approved status is orchestrator-owned and recorded in
`methods/STATUS.md`.

If rerunning the work or a smaller check materially improves confidence, do
so. If not, state the residual risk.

Output findings first, ordered by severity, with file paths. Then give:

- whether any issue blocks integration;
- what approved-status update, if any, your findings support and what you
  actually checked;
- what would need to change before merge, if anything;
- what could be nonblocking follow-up after merge;
- which commit you reviewed;
- which commands you ran.
```
