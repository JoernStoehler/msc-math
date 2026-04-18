# Feature Pattern Search Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset`
- joined rows: `282`
- random rows: `170`
- endpoint rows: `112`
- rows with cached sigma payload: `267`
- rows with bounded best-orbit KKT payload: `267`
- rows with cached search-level orbit scalars: `192`
- rows with trace-derived trajectory payload: `22`
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
- bounded best-orbit KKT coverage by dataset:
  - `gradient_ascent_general`: `10` / `10`
  - `gradient_ascent_products`: `12` / `12`
  - `random_product_sample`: `100` / `100`
  - `random_sample`: `70` / `70`
  - `variable_f_ascent`: `75` / `90`
- cached search-level orbit-scalar coverage by dataset:
  - `gradient_ascent_general`: `10` / `10`
  - `gradient_ascent_products`: `12` / `12`
  - `random_product_sample`: `100` / `100`
  - `random_sample`: `70` / `70`
  - `variable_f_ascent`: `0` / `90`
- trajectory trace coverage by dataset:
  - `gradient_ascent_general`: `10` / `10`
  - `gradient_ascent_products`: `12` / `12`
  - `random_product_sample`: `0` / `100`
  - `random_sample`: `0` / `70`
  - `variable_f_ascent`: `0` / `90`

## Feature Blocks

- `null`: train-mean predictor with no features
- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend
- `geometry`: cheap dual-vertex summaries from `polytopes.jsonl`
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: ridge-local `omega_0` summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars
- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries
- `all`: metadata, geometry, skeleton, omega, orbit, and trajectory together

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
| Within random | `orbit` | 0.3222 | 0.1690 |
| Within random | `trajectory` | -0.0140 | 0.2068 |
| Within random | `all` | 0.4894 | 0.1467 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4367 | 0.0911 |
| Within endpoint | `geometry` | -0.1145 | 0.1281 |
| Within endpoint | `skeleton` | -0.1228 | 0.1285 |
| Within endpoint | `omega` | -0.0275 | 0.1230 |
| Within endpoint | `orbit` | 0.1083 | 0.1146 |
| Within endpoint | `trajectory` | 0.0026 | 0.1212 |
| Within endpoint | `all` | 0.1266 | 0.1134 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -12.4344 | 0.4446 |
| Random -> endpoint | `geometry` | -9.9589 | 0.4016 |
| Random -> endpoint | `skeleton` | -12.1083 | 0.4392 |
| Random -> endpoint | `omega` | -11.3543 | 0.4264 |
| Random -> endpoint | `orbit` | -95.4662 | 1.1915 |
| Random -> endpoint | `trajectory` | -17.8854 | 0.5272 |
| Random -> endpoint | `all` | -62.3199 | 0.9653 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -8.0876 | 0.6190 |
| Endpoint -> random | `geometry` | -5.5594 | 0.5259 |
| Endpoint -> random | `skeleton` | -7.6995 | 0.6056 |
| Endpoint -> random | `omega` | -6.8095 | 0.5738 |
| Endpoint -> random | `orbit` | -6.7717 | 0.5724 |
| Endpoint -> random | `trajectory` | -6.1597 | 0.5494 |
| Endpoint -> random | `all` | -5.7220 | 0.5324 |

### Random forest

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1345 | 0.1910 |
| Within random | `geometry` | 0.4174 | 0.1567 |
| Within random | `skeleton` | 0.1103 | 0.1937 |
| Within random | `omega` | 0.2514 | 0.1777 |
| Within random | `orbit` | 0.3970 | 0.1594 |
| Within random | `trajectory` | -0.0147 | 0.2068 |
| Within random | `all` | 0.5342 | 0.1401 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4377 | 0.0910 |
| Within endpoint | `geometry` | -0.3199 | 0.1394 |
| Within endpoint | `skeleton` | -0.3751 | 0.1423 |
| Within endpoint | `omega` | -0.2243 | 0.1342 |
| Within endpoint | `orbit` | 0.0967 | 0.1153 |
| Within endpoint | `trajectory` | 0.0003 | 0.1213 |
| Within endpoint | `all` | 0.2324 | 0.1063 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.1447 | 0.4562 |
| Random -> endpoint | `geometry` | -9.9360 | 0.4012 |
| Random -> endpoint | `skeleton` | -14.8860 | 0.4835 |
| Random -> endpoint | `omega` | -14.3213 | 0.4748 |
| Random -> endpoint | `orbit` | -20.3696 | 0.5608 |
| Random -> endpoint | `trajectory` | -17.7599 | 0.5254 |
| Random -> endpoint | `all` | -10.9486 | 0.4193 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.8134 | 0.5739 |
| Endpoint -> random | `geometry` | -3.8119 | 0.4504 |
| Endpoint -> random | `skeleton` | -7.0737 | 0.5834 |
| Endpoint -> random | `omega` | -5.8803 | 0.5386 |
| Endpoint -> random | `orbit` | -6.5339 | 0.5636 |
| Endpoint -> random | `trajectory` | -6.1450 | 0.5488 |
| Endpoint -> random | `all` | -5.8719 | 0.5383 |

## Top States

| State | Dataset | Regime | sys |
|-------|---------|--------|-----|
| `variable_f::rq1_general_9_p0` | `variable_f_ascent` | `endpoint` | 0.906316 |
| `variable_f::rq1_general_9_p4` | `variable_f_ascent` | `endpoint` | 0.904680 |
| `variable_f::rq1_general_9_p1` | `variable_f_ascent` | `endpoint` | 0.903770 |
| `variable_f::rq2_seed0_pathC_f11rand` | `variable_f_ascent` | `endpoint` | 0.903176 |
| `ga_general::general_9` | `gradient_ascent_general` | `endpoint` | 0.902965 |

## Interpretation

- within-random ridge: metadata `R^2=0.1906`, geometry `R^2=0.4260`, skeleton `R^2=0.1350`, omega `R^2=0.2652`, orbit `R^2=0.3222`, trajectory `R^2=-0.0140`
- within-endpoint ridge: metadata `R^2=0.4367`, geometry `R^2=-0.1145`, skeleton `R^2=-0.1228`, omega `R^2=-0.0275`, orbit `R^2=0.1083`, trajectory `R^2=0.0026`
- random-to-endpoint transfer with full ridge block: `R^2=-62.3199`
- endpoint-to-random transfer with trajectory ridge: `R^2=-6.1597`
- the richer orbit block now includes bounded best-orbit KKT scalars, using cached search-level payloads where available and a one-best-sigma fallback solve on older cache rows.
