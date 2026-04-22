# Sys-Landscape Datasets

This directory is the datascience-ready table layer for `experiments/sys-landscape/`.

Current rule:
- `raw/` owns the expensive geometry/witness corpus and generator traces;
- `datasets/` owns datascience-ready tables and durable feature blocks;
- consumer/method scripts should read dataset tables instead of rebuilding geometry;
- shared path helpers live in `experiments/sys-landscape/src/datasets.rs`.

The intended shape here is:
- one normalized-table producer for the join surface;
- one polytope-feature assembler for polytope-level enrichment;
- one trajectory-feature producer for step-event summaries;
- feature logic lives in library modules rather than one executable per feature block.

## Flat Dataset Producers

| Flat binary | Backing code today | Role |
| --- | --- | --- |
| `sys-dataset-normalized` | `datasets/normalized.rs` | join raw corpus into stable tables |
| `sys-dataset-polytope-features` | `datasets/polytope-features.rs` + `src/polytope_features.rs` | combined polytope-level enrichment |
| `sys-dataset-feature-trajectory` | `datasets/feature-trajectory.rs` | ascent trace summary block |

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
- `sys-dataset-normalized --raw-dir <tmp/raw>` is a smoke convenience alias for
  the current canonical raw stems in that directory;
- `sys-dataset-polytope-features` writes one combined polytope-level feature JSONL;
- `sys-dataset-feature-trajectory` writes the trajectory summary JSONL.
