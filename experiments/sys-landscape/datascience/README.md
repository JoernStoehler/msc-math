# Sys-Landscape Datascience Pipeline

This directory owns the maintained data pipeline for sys-landscape datascience
method experiments.

## Pipeline

The pipeline has three stages:

1. `produce/` owns expensive producer caches and summary JSONL files.
2. `tables/` builds the final method tables:
   - `polytope-table.jsonl`
   - `observation-table.jsonl`
3. `methods/` owns consumer-side method scripts and reports.

Method agents should normally start from an existing batch dataset under
`batches/`, not rebuild producer caches and not create private source-truth
datasets under `/tmp`.

## Current Batch Dataset

Current shared batch dataset:

```text
experiments/sys-landscape/datascience/batches/2026-06-03-current/dataset/
```

Fingerprint:

```text
experiments/sys-landscape/datascience/batches/2026-06-03-current/FINGERPRINT.md
```

Build or refresh it from the current committed producer caches with:

```bash
experiments/sys-landscape/datascience/build-current-dataset.sh
```

Observed on 2026-06-03 in the devcontainer:

- release build already warm: about `5m11s` wall time;
- first release run in a fresh worktree also paid about `38s` compile time;
- output size: about `2.5M`;
- output rows: `282` polytope rows and `282` observation rows.

Because the final tables are small and `*.jsonl` is tracked through Git LFS in
this repo, committing a retained batch dataset is reasonable. The expensive
artifact is the producer cache stage, especially `produce/continuation-cache.jsonl`.

## Worktree / Agent Flow

For a method wave:

1. Create an integration worktree.
2. Build or refresh one batch dataset in that worktree.
3. Pass that `dataset/` path to every method executor.
4. Spawn method executors in their own worktrees.
5. Executors write only their method folder under `methods/<slug>/`.
6. Reviewers read the report, code, and dataset, then write `review.md` in the
   method folder.

Do not ask every method executor to run `sys-dataset` into `/tmp`. That repeats
minutes of table work, creates incomparable inputs, and makes later review
harder.

Use `/tmp` only for disposable scratch. If scratch output becomes source truth,
reproduce it into the owned worktree path before citing it in a report.

## Dataset Fingerprint

Print a fingerprint for a dataset with:

```bash
uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
  experiments/sys-landscape/datascience/batches/2026-06-03-current/dataset
```

A method report should cite the dataset path and either include this fingerprint
or link to the batch `FINGERPRINT.md`.

## Stage Documentation

- Producer stage: `produce/README.md`
- Table stage: `tables/README.md`
- Method stage: `methods/README.md`
