# Sys-Landscape Datascience Dataset

Purpose: active shared input dataset for sys-landscape datascience method waves.

This dataset was generated from the committed producer caches under
`experiments/sys-landscape/datascience/produce/`.

Build command:

```bash
experiments/sys-landscape/datascience/build-dataset.sh
```

Observed runtime for the current retained dataset:

- LICCA table build on 2026-06-04 at repo `155d527b`: `2m06s` Slurm wall
  time, `32` allocated CPUs, `1:07:12` CPUTime, about `1.1G` MaxRSS.
- Stage timings from the Slurm log: `49.8s` loading producer caches, `72.4s`
  building the polytope table, `0.0s` building the observation table, and
  `0.2s` writing outputs.

Contents:

- `polytope-table.jsonl`
- `observation-table.jsonl`

Current fingerprint:

- polytope rows: `8445`
- observation rows: `8445`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`
- `polytope-table.jsonl` sha256:
  `bc96000d2c7a70c4aa777891a020bf3c8f7d11d8ee17a084519e2706ce2b4554`
- `observation-table.jsonl` sha256:
  `5382d131dadb4f220512015e876e65566fee51d7c2a25521f7c891c2db8450ce`

Use this dataset as read-only input for method executors. Method outputs belong
under `../methods/<slug>/`.

Do not replace this dataset with private `/tmp` output. If the producer caches
or table code change, refresh this directory intentionally and review the table
diff.

For row counts, hashes, source counts, max `sys`, and `sys > 1` count, run:

```bash
uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
  experiments/sys-landscape/datascience/dataset
```
