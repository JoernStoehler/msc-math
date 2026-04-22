# Sys-Landscape Datasets

This directory is the datascience-ready table layer for `experiments/sys-landscape/`.

Current rule:
- `raw/` owns the expensive geometry/witness corpus and generator traces;
- `datasets/` owns datascience-ready tables and durable feature blocks;
- consumer/method scripts should read dataset tables instead of rebuilding geometry;
- shared path helpers live in `experiments/sys-landscape/src/datasets.rs`.

The intended shape here is:
- one core-table producer for the join surface;
- one feature producer for cheap derived datascience tables;
- feature logic lives in library modules rather than one executable per feature block.

## Flat Dataset Producers

| Flat binary | Backing code today | Role |
| --- | --- | --- |
| `sys-dataset-core-tables` | `datasets/core-tables.rs` | join raw corpus into stable tables |
| `sys-dataset-features` | `datasets/features.rs` + `src/datascience/**/*.rs` | write datascience-facing feature tables |

## Planned Consumer Split

The intended long-run split is:
- `raw/`: cache-worthy geometry and witness corpus
- `datasets/`: datascience-ready tables
- `methods/`: consumer-side modeling, exploratory analysis, and comparison scripts

Today the consumer side still lives partly in legacy folders such as `feature-pattern-search/`. That is acceptable while the dataset-table surface settles, but new smoke and method work should target the combined dataset producers above.

## Smoke Path

[smoke-pipeline.sh](../smoke-pipeline.sh) runs the current end-to-end dataset
surface against temp directories:
- `raw/` producers emit ad hoc corpus files;
- `sys-dataset-core-tables --raw-dir <tmp/raw>` is a smoke convenience alias for
  the current canonical raw stems in that directory;
- `sys-dataset-features` reads the written core tables and writes the feature
  outputs under the chosen output directory.
