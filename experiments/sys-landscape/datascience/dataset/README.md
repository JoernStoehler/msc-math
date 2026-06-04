# Sys-Landscape Datascience Dataset

Purpose: active shared input dataset for sys-landscape datascience method waves.

This dataset was generated from the committed producer caches under
`experiments/sys-landscape/datascience/produce/`.

Build command:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Observed runtime in the devcontainer on 2026-06-03:

- warm release build: about `5m16s` wall time;
- first release run in a fresh worktree: about `38s` compile time plus about
  `5m48s` total command time.

Contents:

- `polytope-table.jsonl`
- `observation-table.jsonl`
- `FINGERPRINT.md`

Use this dataset as read-only input for method executors. Method outputs belong
under `../../methods/<slug>/`.

Do not replace this dataset with private `/tmp` output. If the producer caches
or table code change, refresh this directory intentionally and review the
fingerprint diff.
