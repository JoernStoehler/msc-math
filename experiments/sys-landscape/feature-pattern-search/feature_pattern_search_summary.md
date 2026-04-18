# Feature Pattern Search Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset`
- joined rows: `282`
- random rows: `170`
- local rows: `112`
- dataset counts:
  - `gradient_ascent_general`: `10`
  - `gradient_ascent_products`: `12`
  - `random_product_sample`: `100`
  - `random_sample`: `70`
  - `variable_f_ascent`: `90`

## Feature Blocks

- `null`: train-mean predictor with no features
- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend
- `geometry`: cheap dual-vertex summaries from `polytopes.jsonl`
- `combined`: metadata and geometry together

## Metrics

Reported metrics are test-set `R^2` and RMSE. Within-regime results use grouped CV keyed by shared source ancestry in the local regime and by `state_id` in the random regime. Transfer results train on one regime and test on the other.

### Ridge

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1906 | 0.1847 |
| Within random | `geometry` | 0.4260 | 0.1556 |
| Within random | `combined` | 0.4214 | 0.1562 |
| Within local | `null` | -0.0227 | 0.1227 |
| Within local | `metadata` | 0.4367 | 0.0911 |
| Within local | `geometry` | -0.1145 | 0.1281 |
| Within local | `combined` | 0.3014 | 0.1014 |
| Random -> local | `null` | -17.8854 | 0.5272 |
| Random -> local | `metadata` | -12.4344 | 0.4446 |
| Random -> local | `geometry` | -9.9589 | 0.4016 |
| Random -> local | `combined` | -9.3303 | 0.3899 |
| Local -> random | `null` | -6.2430 | 0.5526 |
| Local -> random | `metadata` | -8.0876 | 0.6190 |
| Local -> random | `geometry` | -5.5594 | 0.5259 |
| Local -> random | `combined` | -6.8803 | 0.5764 |

### Random forest

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1345 | 0.1910 |
| Within random | `geometry` | 0.4174 | 0.1567 |
| Within random | `combined` | 0.4143 | 0.1571 |
| Within local | `null` | -0.0227 | 0.1227 |
| Within local | `metadata` | 0.4377 | 0.0910 |
| Within local | `geometry` | -0.3199 | 0.1394 |
| Within local | `combined` | 0.2889 | 0.1023 |
| Random -> local | `null` | -17.8854 | 0.5272 |
| Random -> local | `metadata` | -13.1447 | 0.4562 |
| Random -> local | `geometry` | -9.9360 | 0.4012 |
| Random -> local | `combined` | -9.7571 | 0.3979 |
| Local -> random | `null` | -6.2430 | 0.5526 |
| Local -> random | `metadata` | -6.8134 | 0.5739 |
| Local -> random | `geometry` | -3.8119 | 0.4504 |
| Local -> random | `combined` | -5.8791 | 0.5385 |

## Top States

| State | Dataset | Regime | sys |
|-------|---------|--------|-----|
| `variable_f::rq1_general_9_p0` | `variable_f_ascent` | `local` | 0.906316 |
| `variable_f::rq1_general_9_p4` | `variable_f_ascent` | `local` | 0.904680 |
| `variable_f::rq1_general_9_p1` | `variable_f_ascent` | `local` | 0.903770 |
| `variable_f::rq2_seed0_pathC_f11rand` | `variable_f_ascent` | `local` | 0.903176 |
| `ga_general::general_9` | `gradient_ascent_general` | `local` | 0.902965 |

## Interpretation

- within-random ridge: metadata `R^2=0.1906`, geometry `R^2=0.4260`
- within-local ridge: metadata `R^2=0.4367`, geometry `R^2=-0.1145`
- random-to-local transfer with combined ridge: `R^2=-9.3303`
- local-to-random transfer with geometry ridge: `R^2=-5.5594`
- cheap geometry summaries help inside the random regime, but they do not explain the local-endpoint regime and do not transfer across regimes.
- treat this as a bounded first pass: cheap geometry summaries only, no skeleton or orbit enrichments yet.
