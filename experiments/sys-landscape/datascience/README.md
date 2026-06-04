# Sys-Landscape Datascience

This directory owns the maintained data flow for sys-landscape datascience
experiments. Read this file before launching, executing, or reviewing a
datascience method wave.

## Folder Map

```text
experiments/sys-landscape/datascience/
|-- README.md              # this data-flow and agent-role guide
|-- build-dataset.sh       # refreshes dataset/ from produce/ through tables/
|-- fingerprint-dataset.py # prints on-demand row counts, hashes, and guard facts
|-- dataset/               # active shared method input; read-only for method agents
|-- produce/               # accepted polytope/observation producers and caches
|-- tables/                # Rust table builder and feature columns
|-- methods/               # method experiments and reports
`-- smoke-pipeline.sh      # temp-output integration smoke path
```

## Current Dataset

Current shared dataset:

```text
experiments/sys-landscape/datascience/dataset/
```

Contents:

- `polytope-table.jsonl`
- `observation-table.jsonl`

Build or refresh these files from the current committed producer caches with:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Observed on 2026-06-03 in the devcontainer:

- release build already warm: about `5m11s` wall time;
- first release run in a fresh worktree also paid about `38s` compile time;
- output size: about `2.5M`;
- output rows: `282` polytope rows and `282` observation rows.

Because the final tables are small and `*.jsonl` is tracked through Git LFS in
this repo, committing the retained dataset is reasonable. The expensive artifact
is the producer cache stage, especially `produce/continuation-cache.jsonl`.

Use `git log -- experiments/sys-landscape/datascience/dataset/` and file mtimes
for timing/provenance questions. Use `fingerprint-dataset.py` only when a report
or review needs explicit row counts, hashes, max `sys`, or `sys > 1` count.

## Agent Roles

### Orchestrators

- Create an integration worktree for a method wave.
- Refresh `dataset/` only if `produce/`, `tables/`, or table-related source
  changed.
- Give executors the dataset path, write scope, research question, stop
  conditions, and required report path.
- Treat `/tmp` as scratch only. Do not use a private `/tmp` dataset as source
  truth for a method wave.

### Method Executors

- For method-only experiments, read `dataset/` and write only under
  `methods/<slug>/`.
- Do not rebuild producer caches or overwrite `dataset/` unless explicitly
  assigned that stage.
- Required output is `report.md` or an already established report filename in
  an existing method folder. Machine-readable sidecars are optional and need a
  real consumer.
- If a method computes local exploratory features, keep them in the method
  folder and label them in the report as local exploratory features, candidate
  table features, or rejected features.

### Feature/Table Executors

- Use this role for reusable columns derived from existing producer rows.
- Local feature probes should live in `methods/<slug>/` first unless the feature
  is already clearly reusable and mathematically natural.
- Accepted reusable features belong in `tables/`; refreshing `dataset/` is part
  of that stage change.
- The report must separate the table-feature claim from any method-result claim.

### Producer Executors

- Use this role for new polytope or observation sources.
- Speculative producer output should stay method-local, for example under
  `methods/<slug>/candidate-dataset/`, until reviewed.
- Accepted producer output belongs in `produce/`; refreshing `dataset/` is part
  of accepting the producer into the maintained data flow.
- The report must separate what was generated from what later methods inferred.

### Reviewers

- Review the report, command, changed files, and dataset path.
- Recompute dataset guard facts when needed with `fingerprint-dataset.py`; do
  not require a tracked metadata sidecar unless the method created one for a
  specific consumer.
- Check leakage, stale data, overclaiming, and whether the result supports the
  stated verdict.

## Source-Truth Rules

- `produce/` owns accepted producer rows and caches.
- `tables/` owns reusable table columns and table-building code.
- `dataset/` is the active shared input for methods.
- `methods/<slug>/` owns method-local code, reports, figures, and local
  exploratory features.
- Historical reports may cite `/tmp` dataset paths. Treat those paths as
  provenance for that old run, not as current source truth.
- Do not patch-edit generated JSONL tables. Regenerate them with the recorded
  command and review the diff.

## On-Demand Dataset Checks

Print row counts, hashes, source counts, max `sys`, and `sys > 1` count with:

```bash
uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
  experiments/sys-landscape/datascience/dataset
```

Add `--format json` if a reviewer or script needs machine-readable output.

## Stage Documentation

- Producer stage: `produce/README.md`
- Table stage: `tables/README.md`
- Method stage: `methods/README.md`
