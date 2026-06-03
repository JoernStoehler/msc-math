# 2026-06-03 Current Datascience Batch

Purpose: shared input dataset for sys-landscape datascience method waves.

This batch was generated from the committed producer caches under
`experiments/sys-landscape/datascience/produce/`.

Build command:

```bash
experiments/sys-landscape/datascience/build-current-dataset.sh
```

Observed runtime in the devcontainer on 2026-06-03:

- warm release build: about `5m16s` wall time;
- first release run in a fresh worktree: about `38s` compile time plus about
  `5m48s` total command time.

Contents:

- `dataset/polytope-table.jsonl`
- `dataset/observation-table.jsonl`
- `FINGERPRINT.md`

Use this dataset as read-only input for method executors. Method outputs belong
under `../../methods/<slug>/`.

Do not replace this dataset with private `/tmp` output. If a refreshed batch is
needed, create a new dated batch directory or intentionally refresh this one and
review the fingerprint diff.
