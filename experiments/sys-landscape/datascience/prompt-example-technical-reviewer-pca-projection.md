# Prompt Example: PCA Projection Technical Reviewer

This is an example for reviewing a completed method packet's internal
consistency, reproducibility, data-science hygiene, and code/artifact quality.
It is not a template. For another method row, adapt the source files and the
method-specific leakage risks.

```text
Review the PCA projection packet commit in:

`/workspaces/msc-math/.worktrees/ds-pca-projection`

Do not edit files. Do not rely on executor context. Treat the committed packet
diff, files, and commands you run as the evidence. If the worktree is dirty,
review the committed packet diff unless the dirty state affects reproducibility
or makes the committed packet ambiguous.

Goal:

Report whether the PCA packet is technically reliable enough to count as
current evidence for the data-science method table. The orchestrator will use
your findings to decide acceptance.

Context:

- The method table is thesis evidence. A technically bad packet can falsely
  close a row, waste future agent time, or make thesis writing overtrust a
  broken result.
- Technical reliability includes reproducibility, data-science hygiene,
  leakage control, artifact consistency, code auditability, and alignment
  between the report and method navigation.
- For each material issue, make clear whether the packet can still be used as
  current technical evidence now, or whether it needs substantial repair,
  reinterpretation, rerun, or archaeology before a future agent can rely on it.
  Salvage value is not enough for current evidence.
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

The review is good if it establishes whether the packet is trustworthy on the
technical merits, not merely whether it satisfies named checks. Evidence that
often matters for this packet includes:

- dataset identity and fingerprint;
- reproduction command, runtime, and dependency declaration;
- tracked generated artifacts and whether the report names their purpose;
- included and excluded columns relative to the validity guard;
- leakage risks from `sys`, capacity columns, endpoint labels, dataset
  identity, optimizer provenance, or post-hoc `sys` inspection;
- separation of observation, inference, thesis use, and terminal state;
- consistency between `methods/README.md`, the report, script, and summary;
- code simplicity and auditability;
- whether old `pca-cluster-anomaly/` material contaminates the current packet.

If rerunning the packet or a smaller check materially improves confidence, do
so. If not, state the residual risk.

Output findings first, ordered by severity, with file paths. Then give:

- whether any issue blocks direct use as current technical evidence;
- what would need to change before direct use, if anything;
- what could be follow-up after direct use;
- which commit you reviewed;
- which commands you ran.
```
