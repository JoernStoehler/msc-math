# Sys-Landscape Datasets

This directory is the flat datascience-ready table layer for `experiments/sys-landscape/`.

Current rule:
- `raw/` owns the expensive geometry/witness corpus and generator traces;
- `datasets/` owns datascience-ready tables and durable feature blocks;
- consumer/method scripts should read dataset tables instead of rebuilding geometry;
- shared path helpers live in `experiments/sys-landscape/src/datasets.rs`.

The current dataset binaries still mostly reuse legacy packet code under `normalized-dataset/` and `feature-*/`. The flat naming layer exists now so later cleanup can remove the legacy packets without changing the public surface again.

## Flat Dataset Producers

| Flat binary | Backing code today | Role |
| --- | --- | --- |
| `sys-dataset-normalized` | `datasets/normalized.rs` | join raw corpus into stable tables |
| `sys-dataset-feature-skeleton` | `feature-skeleton/main.rs` | combinatorial feature block |
| `sys-dataset-feature-dual-vertices` | `datasets/feature-dual-vertices.rs` | floating-point dual-vertex block |
| `sys-dataset-feature-capacity` | `datasets/feature-capacity.rs` | scalar capacity block |
| `sys-dataset-feature-volume` | `datasets/feature-volume.rs` | scalar volume block |
| `sys-dataset-feature-sys` | `datasets/feature-sys.rs` | scalar systolic-ratio block |
| `sys-dataset-feature-face-geometry` | `feature-face-geometry/main.rs` | Euclidean face-geometry block |
| `sys-dataset-feature-face-symplectic` | `feature-face-symplectic/main.rs` | symplectic face block |
| `sys-dataset-feature-omega` | `feature-omega/main.rs` | omega / sign pattern block |
| `sys-dataset-feature-orbit` | `feature-orbit/main.rs` | orbit / sigma-derived block |
| `sys-dataset-feature-trajectory` | `feature-trajectory/main.rs` | ascent trace summary block |

## Planned Consumer Split

The intended long-run split is:
- `raw/`: cache-worthy geometry and witness corpus
- `datasets/`: datascience-ready tables
- `methods/`: consumer-side modeling, exploratory analysis, and comparison scripts

Today the consumer side still lives partly in legacy folders such as `feature-pattern-search/`. That is acceptable while the dataset-table surface settles.

## Smoke Path

[smoke-pipeline.sh](../smoke-pipeline.sh) runs the current end-to-end dataset
surface against temp directories:
- `raw/` producers emit ad hoc corpus files;
- `sys-dataset-normalized --raw-dir <tmp/raw>` is a smoke convenience alias for
  the current canonical raw stems in that directory;
- the flat `sys-dataset-feature-*` binaries write temp feature JSONLs.
