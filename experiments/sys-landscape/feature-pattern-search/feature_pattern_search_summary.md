# Feature Pattern Search Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset -- --out-dir /tmp/feature-pattern-search-3i2mt8hc/normalized`
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
- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry
- `face_symplectic`: ridge-polygon symplectic-area summaries from ordered ridge vertex cycles
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: ridge-local `omega_0` summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars
- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries
- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit, and trajectory together

## Metrics

Reported metrics are test-set `R^2` and RMSE. Within-regime results use grouped CV keyed by persisted `root_group_id` whenever that field is present. Transfer results train on one regime and test on the other.

### Ridge

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1846 | 0.1854 |
| Within random | `geometry` | 0.4683 | 0.1497 |
| Within random | `face_geometry` | 0.1167 | 0.1930 |
| Within random | `face_symplectic` | 0.4883 | 0.1469 |
| Within random | `skeleton` | 0.1628 | 0.1879 |
| Within random | `omega` | 0.2768 | 0.1746 |
| Within random | `orbit` | 0.4495 | 0.1523 |
| Within random | `trajectory` | -0.0140 | 0.2068 |
| Within random | `all` | 0.6185 | 0.1268 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4364 | 0.0911 |
| Within endpoint | `geometry` | -0.1427 | 0.1297 |
| Within endpoint | `face_geometry` | 0.0835 | 0.1161 |
| Within endpoint | `face_symplectic` | 0.3824 | 0.0953 |
| Within endpoint | `skeleton` | -0.0588 | 0.1248 |
| Within endpoint | `omega` | 0.0176 | 0.1202 |
| Within endpoint | `orbit` | 0.1104 | 0.1144 |
| Within endpoint | `trajectory` | 0.0066 | 0.1209 |
| Within endpoint | `all` | 0.3560 | 0.0974 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.6634 | 0.4645 |
| Random -> endpoint | `geometry` | -11.3323 | 0.4260 |
| Random -> endpoint | `face_geometry` | -10.8728 | 0.4180 |
| Random -> endpoint | `face_symplectic` | -8.5693 | 0.3753 |
| Random -> endpoint | `skeleton` | -12.5768 | 0.4470 |
| Random -> endpoint | `omega` | -13.2314 | 0.4576 |
| Random -> endpoint | `orbit` | -32.4641 | 0.7018 |
| Random -> endpoint | `trajectory` | -17.8854 | 0.5272 |
| Random -> endpoint | `all` | -9.8885 | 0.4003 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.3370 | 0.5562 |
| Endpoint -> random | `geometry` | -6.2254 | 0.5519 |
| Endpoint -> random | `face_geometry` | -141.2712 | 2.4491 |
| Endpoint -> random | `face_symplectic` | -623.5699 | 5.1315 |
| Endpoint -> random | `skeleton` | -7.3161 | 0.5921 |
| Endpoint -> random | `omega` | -6.7102 | 0.5701 |
| Endpoint -> random | `orbit` | -6.6180 | 0.5667 |
| Endpoint -> random | `trajectory` | -6.1604 | 0.5494 |
| Endpoint -> random | `all` | -254.1216 | 3.2796 |

### Random forest

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1345 | 0.1910 |
| Within random | `geometry` | 0.4174 | 0.1567 |
| Within random | `face_geometry` | 0.6756 | 0.1170 |
| Within random | `face_symplectic` | 0.8166 | 0.0879 |
| Within random | `skeleton` | 0.1103 | 0.1937 |
| Within random | `omega` | 0.2514 | 0.1777 |
| Within random | `orbit` | 0.3970 | 0.1594 |
| Within random | `trajectory` | -0.0147 | 0.2068 |
| Within random | `all` | 0.8383 | 0.0826 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4377 | 0.0910 |
| Within endpoint | `geometry` | -0.3199 | 0.1394 |
| Within endpoint | `face_geometry` | 0.0988 | 0.1152 |
| Within endpoint | `face_symplectic` | 0.2330 | 0.1062 |
| Within endpoint | `skeleton` | -0.3751 | 0.1423 |
| Within endpoint | `omega` | -0.2243 | 0.1342 |
| Within endpoint | `orbit` | 0.0967 | 0.1153 |
| Within endpoint | `trajectory` | 0.0003 | 0.1213 |
| Within endpoint | `all` | 0.2473 | 0.1052 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.1447 | 0.4562 |
| Random -> endpoint | `geometry` | -9.9360 | 0.4012 |
| Random -> endpoint | `face_geometry` | -6.8727 | 0.3404 |
| Random -> endpoint | `face_symplectic` | -6.2050 | 0.3256 |
| Random -> endpoint | `skeleton` | -14.8860 | 0.4835 |
| Random -> endpoint | `omega` | -14.3213 | 0.4748 |
| Random -> endpoint | `orbit` | -20.3696 | 0.5608 |
| Random -> endpoint | `trajectory` | -17.7599 | 0.5254 |
| Random -> endpoint | `all` | -5.1868 | 0.3017 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.8134 | 0.5739 |
| Endpoint -> random | `geometry` | -3.8119 | 0.4504 |
| Endpoint -> random | `face_geometry` | -1.6001 | 0.3311 |
| Endpoint -> random | `face_symplectic` | -0.4971 | 0.2512 |
| Endpoint -> random | `skeleton` | -7.0737 | 0.5834 |
| Endpoint -> random | `omega` | -5.8803 | 0.5386 |
| Endpoint -> random | `orbit` | -6.5339 | 0.5636 |
| Endpoint -> random | `trajectory` | -6.1450 | 0.5488 |
| Endpoint -> random | `all` | -2.0848 | 0.3606 |

## Top States

| State | Dataset | Regime | sys |
|-------|---------|--------|-----|
| `variable_f::rq1_general_9_p0` | `variable_f_ascent` | `endpoint` | 0.906316 |
| `variable_f::rq1_general_9_p4` | `variable_f_ascent` | `endpoint` | 0.904680 |
| `variable_f::rq1_general_9_p1` | `variable_f_ascent` | `endpoint` | 0.903770 |
| `variable_f::rq2_seed0_pathC_f11rand` | `variable_f_ascent` | `endpoint` | 0.903176 |
| `ga_general::general_9` | `gradient_ascent_general` | `endpoint` | 0.902965 |

## Interpretation

- within-random ridge: metadata `R^2=0.1846`, geometry `R^2=0.4683`, face_geometry `R^2=0.1167`, face_symplectic `R^2=0.4883`, skeleton `R^2=0.1628`, omega `R^2=0.2768`, orbit `R^2=0.4495`, trajectory `R^2=-0.0140`
- within-endpoint ridge: metadata `R^2=0.4364`, geometry `R^2=-0.1427`, face_geometry `R^2=0.0835`, face_symplectic `R^2=0.3824`, skeleton `R^2=-0.0588`, omega `R^2=0.0176`, orbit `R^2=0.1104`, trajectory `R^2=0.0066`
- random-forest strengthens the face-level picture: `face_geometry` reaches `R^2=0.6756` within random, while `face_symplectic` reaches `R^2=0.8166` within random and `0.2330` within endpoints.
- random-to-endpoint transfer with full ridge block: `R^2=-9.8885`
- endpoint-to-random transfer with trajectory ridge: `R^2=-6.1604`
- the richer orbit block now includes bounded best-orbit KKT scalars, using cached search-level payloads where available and a one-best-sigma fallback solve on older cache rows.
