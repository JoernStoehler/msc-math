# Feature Pattern Search Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset`
- joined rows: `282`
- random rows: `170`
- endpoint rows: `112`
- rows with cached sigma payload: `267`
- dataset counts:
  - `gradient_ascent_general`: `10`
  - `gradient_ascent_products`: `12`
  - `random_product_sample`: `100`
  - `random_sample`: `70`
  - `variable_f_ascent`: `90`
- cached sigma coverage by dataset:
  - `gradient_ascent_general`: `10` / `10`
  - `gradient_ascent_products`: `12` / `12`
  - `random_product_sample`: `100` / `100`
  - `random_sample`: `70` / `70`
  - `variable_f_ascent`: `75` / `90`

## Feature Blocks

- `null`: train-mean predictor with no features
- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend
- `geometry`: cheap dual-vertex summaries from `polytopes.jsonl`
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: ridge-local `omega_0` summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, and transition summaries
- `all`: metadata, geometry, skeleton, omega, and orbit together

## Metrics

Reported metrics are test-set `R^2` and RMSE. Within-regime results use grouped CV keyed by persisted `root_group_id` whenever that field is present. Transfer results train on one regime and test on the other.

### Ridge

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1906 | 0.1847 |
| Within random | `geometry` | 0.4260 | 0.1556 |
| Within random | `skeleton` | 0.1350 | 0.1910 |
| Within random | `omega` | 0.2652 | 0.1760 |
| Within random | `orbit` | 0.2615 | 0.1765 |
| Within random | `all` | 0.4579 | 0.1512 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4367 | 0.0911 |
| Within endpoint | `geometry` | -0.1145 | 0.1281 |
| Within endpoint | `skeleton` | -0.1228 | 0.1285 |
| Within endpoint | `omega` | -0.0275 | 0.1230 |
| Within endpoint | `orbit` | 0.1083 | 0.1146 |
| Within endpoint | `all` | 0.1152 | 0.1141 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -12.4344 | 0.4446 |
| Random -> endpoint | `geometry` | -9.9589 | 0.4016 |
| Random -> endpoint | `skeleton` | -12.1083 | 0.4392 |
| Random -> endpoint | `omega` | -11.3543 | 0.4264 |
| Random -> endpoint | `orbit` | -13.3072 | 0.4589 |
| Random -> endpoint | `all` | -9.3037 | 0.3894 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -8.0876 | 0.6190 |
| Endpoint -> random | `geometry` | -5.5594 | 0.5259 |
| Endpoint -> random | `skeleton` | -7.6995 | 0.6056 |
| Endpoint -> random | `omega` | -6.8095 | 0.5738 |
| Endpoint -> random | `orbit` | -5.8141 | 0.5360 |
| Endpoint -> random | `all` | -6.5148 | 0.5629 |

### Random forest

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1345 | 0.1910 |
| Within random | `geometry` | 0.4174 | 0.1567 |
| Within random | `skeleton` | 0.1103 | 0.1937 |
| Within random | `omega` | 0.2514 | 0.1777 |
| Within random | `orbit` | 0.2577 | 0.1769 |
| Within random | `all` | 0.4760 | 0.1486 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4377 | 0.0910 |
| Within endpoint | `geometry` | -0.3199 | 0.1394 |
| Within endpoint | `skeleton` | -0.3751 | 0.1423 |
| Within endpoint | `omega` | -0.2243 | 0.1342 |
| Within endpoint | `orbit` | 0.0917 | 0.1156 |
| Within endpoint | `all` | 0.2338 | 0.1062 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.1447 | 0.4562 |
| Random -> endpoint | `geometry` | -9.9360 | 0.4012 |
| Random -> endpoint | `skeleton` | -14.8860 | 0.4835 |
| Random -> endpoint | `omega` | -14.3213 | 0.4748 |
| Random -> endpoint | `orbit` | -13.5354 | 0.4625 |
| Random -> endpoint | `all` | -9.9519 | 0.4015 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.8134 | 0.5739 |
| Endpoint -> random | `geometry` | -3.8119 | 0.4504 |
| Endpoint -> random | `skeleton` | -7.0737 | 0.5834 |
| Endpoint -> random | `omega` | -5.8803 | 0.5386 |
| Endpoint -> random | `orbit` | -6.5350 | 0.5636 |
| Endpoint -> random | `all` | -5.8260 | 0.5365 |

## Top States

| State | Dataset | Regime | sys |
|-------|---------|--------|-----|
| `variable_f::rq1_general_9_p0` | `variable_f_ascent` | `endpoint` | 0.906316 |
| `variable_f::rq1_general_9_p4` | `variable_f_ascent` | `endpoint` | 0.904680 |
| `variable_f::rq1_general_9_p1` | `variable_f_ascent` | `endpoint` | 0.903770 |
| `variable_f::rq2_seed0_pathC_f11rand` | `variable_f_ascent` | `endpoint` | 0.903176 |
| `ga_general::general_9` | `gradient_ascent_general` | `endpoint` | 0.902965 |

## Interpretation

- within-random ridge: metadata `R^2=0.1906`, geometry `R^2=0.4260`, skeleton `R^2=0.1350`, omega `R^2=0.2652`, orbit `R^2=0.2615`
- within-endpoint ridge: metadata `R^2=0.4367`, geometry `R^2=-0.1145`, skeleton `R^2=-0.1228`, omega `R^2=-0.0275`, orbit `R^2=0.1083`
- random-to-endpoint transfer with full ridge block: `R^2=-9.3037`
- endpoint-to-random transfer with orbit ridge: `R^2=-5.8141`
- this packet still stops before orbit recomputation; the new signal surface adds sigma-local orbit features derived only from cached `best_sigma` payloads.
