# Sys-Landscape Datascience Methods

This directory owns consumer-side datascience scripts for `experiments/sys-landscape/datascience/`.

Current rule:
- methods read tables written by `tables/main.rs`
- method waves should use the shared dataset path under
  `../batches/<batch>/dataset/`; methods should not create private `/tmp`
  datasets as source truth
- methods do not rebuild producer caches
- method-local filtering, scaling, and model matrices stay local to the method
- new method spikes should get their own folder under this directory
- avoid refactoring a shared helper layer during a spike wave; copy a small
  method-local loader/check template when that keeps workers independent and
  reviews simpler
- promote shared helpers only after repeated completed reports show the copied
  code is real maintenance cost

Current consumers:
- `eda.py`
- `feature-pattern-search/`
- `exact-f64-spot-check/`
- `pca-cluster-spike/`
- `supervised-alternatives-spike/`

The current dataset inputs are:
- `polytope-table.jsonl`
- `observation-table.jsonl`

Current shared dataset:

```text
experiments/sys-landscape/datascience/batches/2026-06-03-current/dataset/
```

Historical committed reports may mention `/tmp/...` dataset snapshots. Treat
those as historical provenance only. New method-wave reports should cite a
shared batch dataset path and its `FINGERPRINT.md`.
