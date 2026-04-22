# Sys-Landscape Datasets

This directory is the datascience-ready table layer for `experiments/sys-landscape/`.

Current rule:
- `raw/` owns the expensive geometry/witness corpus and generator traces;
- `datasets/` owns the datascience-ready tables written from those raw inputs;
- consumer/method scripts should read dataset tables instead of rebuilding geometry;
- shared path helpers live in `experiments/sys-landscape/src/datasets.rs`.

The intended shape here is:
- one dataset producer that loads the raw corpus and writes the whole shared
  dataset surface;
- feature logic lives in library modules rather than one executable per feature block.

## Maintained Dataset Producer

- `sys-dataset`
  - backing code: `datasets/main.rs` plus `src/datascience/**/*.rs`
  - role: load raw producer outputs and write the shared dataset files used by methods

Current shared outputs include:
- `polytopes.jsonl`
- `states.jsonl`
- `capacity_results.jsonl`
- `orbit_records.jsonl`
- `step_events.jsonl`
- `polytope-features.jsonl`
- `trajectory-features.jsonl`

## Planned Consumer Split

The intended long-run split is:
- `raw/`: cache-worthy geometry and witness corpus
- `datasets/`: datascience-ready tables
- `methods/`: consumer-side modeling, exploratory analysis, and comparison scripts

Today the consumer side still lives partly in legacy folders such as `feature-pattern-search/`. That is acceptable while the method surface settles, but new smoke and method work should target the single dataset producer above.

## Smoke Path

[smoke-pipeline.sh](../smoke-pipeline.sh) runs the current end-to-end dataset
surface against temp directories:
- `raw/` producers emit ad hoc corpus files;
- `sys-dataset --raw-dir <tmp/raw>` consumes the current canonical raw stems in
  that directory and writes the whole dataset surface under the chosen output
  directory.
