# Sys-Landscape Datascience Methods

This directory owns consumer-side datascience scripts for `experiments/sys-landscape/datascience/`.

Current rule:
- the top-level `../README.md` owns method-executor, reviewer, and promotion
  rules;
- method waves use `../dataset/`;
- methods do not create private `/tmp` datasets as source truth;
- method-local filtering, scaling, and model matrices stay local to the method;
- new method spikes should get their own folder under this directory;
- required output is a report; JSON sidecars are optional and need a concrete
  consumer.

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
experiments/sys-landscape/datascience/dataset/
```

Historical committed reports may mention `/tmp/...` dataset snapshots. Treat
those as historical provenance only. New method-wave reports should cite the
shared dataset path and any dataset checks actually used.
