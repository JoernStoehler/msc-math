# Feature Pattern Search Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset -- --out-dir /tmp/feature-pattern-search-a8v3jetr/normalized`
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
- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention
- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling
- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars
- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries
- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit, and trajectory together

## Symmetry Status

| Block | `vol(K)=1` convention | Translation invariant | `Sp(4)`-invariant | Notes |
|-------|------------------------|-----------------------|-------------------|-------|
| `metadata` | no | no | no | Search provenance and family labels, not geometry invariants. |
| `geometry` | yes | no | no | Uses dual-coordinate norms, centroids, and singular values after `vol(K)=1` rescaling. |
| `face_geometry` | yes | yes | no | Euclidean edge/facet sizes on the rescaled polytope. |
| `face_symplectic` | yes | yes | yes | Ridge-polygon symplectic areas divided by `vol(K)^(1/2)`. |
| `skeleton` | yes | yes | yes | Pure combinatorics; unaffected by translation, `Sp(4)`, or scaling. |
| `omega` | yes | no | mixed | `omega_0` magnitudes are volume-normalized, but the dual-coordinate packet still depends on translation gauge; transition graph and zero-sign structure do not. |
| `orbit` | mixed | mixed | mixed | Mixes sigma-local geometry, transition summaries, and search/KKT scalars. |
| `trajectory` | no | no | no | Search-procedure diagnostics, not geometry invariants. |

## Metrics

Reported metrics are test-set `R^2` and RMSE. Within-regime results use grouped CV keyed by persisted `root_group_id` whenever that field is present. Transfer results train on one regime and test on the other.

### Ridge

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1846 | 0.1854 |
| Within random | `geometry` | 0.4427 | 0.1533 |
| Within random | `face_geometry` | 0.3847 | 0.1611 |
| Within random | `face_symplectic` | 0.5483 | 0.1380 |
| Within random | `skeleton` | 0.1628 | 0.1879 |
| Within random | `omega` | 0.4251 | 0.1557 |
| Within random | `orbit` | 0.4495 | 0.1523 |
| Within random | `trajectory` | -0.0140 | 0.2068 |
| Within random | `all` | 0.5985 | 0.1301 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4364 | 0.0911 |
| Within endpoint | `geometry` | -0.0965 | 0.1270 |
| Within endpoint | `face_geometry` | 0.1030 | 0.1149 |
| Within endpoint | `face_symplectic` | 0.4000 | 0.0940 |
| Within endpoint | `skeleton` | -0.0588 | 0.1248 |
| Within endpoint | `omega` | 0.0901 | 0.1157 |
| Within endpoint | `orbit` | 0.1104 | 0.1144 |
| Within endpoint | `trajectory` | 0.0066 | 0.1209 |
| Within endpoint | `all` | 0.3239 | 0.0997 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.6634 | 0.4645 |
| Random -> endpoint | `geometry` | -10.3664 | 0.4090 |
| Random -> endpoint | `face_geometry` | -10.3816 | 0.4093 |
| Random -> endpoint | `face_symplectic` | -9.1481 | 0.3864 |
| Random -> endpoint | `skeleton` | -12.5768 | 0.4470 |
| Random -> endpoint | `omega` | -11.9827 | 0.4371 |
| Random -> endpoint | `orbit` | -32.4641 | 0.7018 |
| Random -> endpoint | `trajectory` | -17.8854 | 0.5272 |
| Random -> endpoint | `all` | -11.0146 | 0.4205 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.3370 | 0.5562 |
| Endpoint -> random | `geometry` | -3.3729 | 0.4294 |
| Endpoint -> random | `face_geometry` | -27.1522 | 1.0894 |
| Endpoint -> random | `face_symplectic` | -20.1806 | 0.9450 |
| Endpoint -> random | `skeleton` | -7.3161 | 0.5921 |
| Endpoint -> random | `omega` | -3.4124 | 0.4313 |
| Endpoint -> random | `orbit` | -6.6180 | 0.5667 |
| Endpoint -> random | `trajectory` | -6.1604 | 0.5494 |
| Endpoint -> random | `all` | -22.9752 | 1.0054 |

### Random forest

| Surface | Block | R^2 | RMSE |
|---------|-------|-----|------|
| Within random | `null` | -0.0140 | 0.2068 |
| Within random | `metadata` | 0.1345 | 0.1910 |
| Within random | `geometry` | 0.7211 | 0.1084 |
| Within random | `face_geometry` | 0.7009 | 0.1123 |
| Within random | `face_symplectic` | 0.8779 | 0.0717 |
| Within random | `skeleton` | 0.1103 | 0.1937 |
| Within random | `omega` | 0.7948 | 0.0930 |
| Within random | `orbit` | 0.3970 | 0.1594 |
| Within random | `trajectory` | -0.0147 | 0.2068 |
| Within random | `all` | 0.9003 | 0.0648 |
| Within endpoint | `null` | -0.0227 | 0.1227 |
| Within endpoint | `metadata` | 0.4377 | 0.0910 |
| Within endpoint | `geometry` | -0.1747 | 0.1315 |
| Within endpoint | `face_geometry` | 0.1218 | 0.1137 |
| Within endpoint | `face_symplectic` | 0.2934 | 0.1020 |
| Within endpoint | `skeleton` | -0.3751 | 0.1423 |
| Within endpoint | `omega` | -0.2536 | 0.1358 |
| Within endpoint | `orbit` | 0.0967 | 0.1153 |
| Within endpoint | `trajectory` | 0.0003 | 0.1213 |
| Within endpoint | `all` | 0.3438 | 0.0983 |
| Random -> endpoint | `null` | -17.8854 | 0.5272 |
| Random -> endpoint | `metadata` | -13.1447 | 0.4562 |
| Random -> endpoint | `geometry` | -5.2355 | 0.3029 |
| Random -> endpoint | `face_geometry` | -6.8685 | 0.3403 |
| Random -> endpoint | `face_symplectic` | -4.6835 | 0.2892 |
| Random -> endpoint | `skeleton` | -14.8860 | 0.4835 |
| Random -> endpoint | `omega` | -4.6180 | 0.2875 |
| Random -> endpoint | `orbit` | -20.3696 | 0.5608 |
| Random -> endpoint | `trajectory` | -17.7599 | 0.5254 |
| Random -> endpoint | `all` | -2.8902 | 0.2393 |
| Endpoint -> random | `null` | -6.2430 | 0.5526 |
| Endpoint -> random | `metadata` | -6.8134 | 0.5739 |
| Endpoint -> random | `geometry` | -3.1679 | 0.4192 |
| Endpoint -> random | `face_geometry` | -1.6891 | 0.3367 |
| Endpoint -> random | `face_symplectic` | -0.0082 | 0.2062 |
| Endpoint -> random | `skeleton` | -7.0737 | 0.5834 |
| Endpoint -> random | `omega` | -4.4154 | 0.4778 |
| Endpoint -> random | `orbit` | -6.5339 | 0.5636 |
| Endpoint -> random | `trajectory` | -6.1450 | 0.5488 |
| Endpoint -> random | `all` | -0.4932 | 0.2509 |

## Top States

| State | Dataset | Regime | sys |
|-------|---------|--------|-----|
| `variable_f::rq1_general_9_p0` | `variable_f_ascent` | `endpoint` | 0.906316 |
| `variable_f::rq1_general_9_p4` | `variable_f_ascent` | `endpoint` | 0.904680 |
| `variable_f::rq1_general_9_p1` | `variable_f_ascent` | `endpoint` | 0.903770 |
| `variable_f::rq2_seed0_pathC_f11rand` | `variable_f_ascent` | `endpoint` | 0.903176 |
| `ga_general::general_9` | `gradient_ascent_general` | `endpoint` | 0.902965 |

## Interpretation

- within-random ridge: metadata `R^2=0.1846`, geometry `R^2=0.4427`, face_geometry `R^2=0.3847`, face_symplectic `R^2=0.5483`, skeleton `R^2=0.1628`, omega `R^2=0.4251`, orbit `R^2=0.4495`, trajectory `R^2=-0.0140`
- within-endpoint ridge: metadata `R^2=0.4364`, geometry `R^2=-0.0965`, face_geometry `R^2=0.1030`, face_symplectic `R^2=0.4000`, skeleton `R^2=-0.0588`, omega `R^2=0.0901`, orbit `R^2=0.1104`, trajectory `R^2=0.0066`
- random-forest strengthens the face-level picture: `face_geometry` remains strong within random, while volume-normalized `face_symplectic` stays the strongest non-metadata endpoint-side face block.
- random-to-endpoint transfer with full ridge block: `R^2=-11.0146`
- endpoint-to-random transfer with trajectory ridge: `R^2=-6.1604`
- All geometric magnitude blocks in this packet now use the `vol(K)=1` convention; other symmetry-aware normalizations remain possible and are not ruled out by this packet.
- the richer orbit block now includes bounded best-orbit KKT scalars, using cached search-level payloads where available and a one-best-sigma fallback solve on older cache rows.
