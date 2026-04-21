# Sys-Landscape Datasets

This directory is the flat dataset-layer surface for `experiments/sys-landscape/`.

Current rule:
- producer executables own tracked data;
- consumer/method scripts should read produced datasets instead of rebuilding them;
- shared domain logic lives in `experiments/sys-landscape/src/lib.rs`;
- canonical output names in this directory follow `name.jsonl`, `name-trace.jsonl`, `name-cache.jsonl`, and transient smoke names `smoke-name.jsonl`.

The current legacy producer folders stay in place until a later cleanup pass removes or archives them.

## Canonical Producers

| Canonical file stem | Producer entrypoint | Legacy source | Notes |
| --- | --- | --- | --- |
| `random` | `datasets/random.rs` | `random-sample/main.rs` | generic random baseline |
| `random-product` | `datasets/random-product.rs` | `random-product-sample/main.rs` | product baseline |
| `ascent` | `datasets/ascent.rs` | `gradient-ascent-general/main.rs` | fixed-`F` general ascent |
| `ascent-product` | `datasets/ascent-product.rs` | `gradient-ascent-products/main.rs` | fixed-`F` product ascent |
| `continuation` | `datasets/continuation.rs` | `variable-f-ascent/main.rs` | variable-`F` continuation surface |

The following producers remain legacy-only for now:
- `rejection-calibration/`
- `rotated-regular-products/`
- `normalized-dataset/`
- `feature-*/`

## Planned Consumer Split

The intended long-run split is:
- `datasets/`: producer-side surfaces and derived join tables
- `methods/`: consumer-side modeling, exploratory analysis, and comparison scripts

Today the consumer side still lives partly in legacy folders such as `feature-pattern-search/`. That is acceptable while the new producer surface and canonical dataset names are still settling.
