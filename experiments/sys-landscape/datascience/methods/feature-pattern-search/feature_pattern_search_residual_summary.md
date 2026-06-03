# Feature Pattern Search Residual Summary

## Dataset

- endpoint regime only: `gradient_ascent_general`, `gradient_ascent_products`, and `variable_f_ascent`
- endpoint rows: `112`
- grouped endpoint folds: `5`
- dataset counts:
  - `gradient_ascent_general`: `10`
  - `gradient_ascent_products`: `12`
  - `variable_f_ascent`: `90`

## Method

- baseline: grouped CV model on metadata only
- residual packet: fit metadata on each train fold, subtract its train-fold predictions, then fit one block model on the residuals
- signal criterion: `combined R^2 > metadata R^2`; `residual R^2 > 0` is the direct residual check
- metrics: grouped-CV `R^2` and RMSE

## Blocks

- `geometry`: cheap dual-vertex summaries from the existing geometry packet
- `face_geometry`: edge-length and facet-volume summaries
- `face_symplectic`: ridge-polygon symplectic-area summaries
- `skeleton`: combinatorial face-lattice summaries
- `omega`: dual-side `omega_0` magnitudes, sign structure, and transition graph summaries
- `orbit`: cached-best-sigma support and bounded orbit/KKT summaries
- `trajectory`: endpoint trace aggregates
- `all_non_metadata`: concatenation of the seven non-metadata blocks above

## Metrics

Reported values are out-of-fold endpoint scores. `Delta R^2` is `combined R^2 - metadata R^2`.

### Ridge

| Block | Metadata R^2 | Combined R^2 | Delta R^2 | Residual R^2 | Metadata RMSE | Combined RMSE | Adds signal? |
|-------|-------------|--------------|-----------|--------------|---------------|---------------|-------------|
| `geometry` | -0.0159 | -0.0932 | -0.0773 | -0.0761 | 0.1223 | 0.1268 | no |
| `face_geometry` | -0.0159 | 0.0264 | 0.0423 | 0.0416 | 0.1223 | 0.1197 | yes |
| `face_symplectic` | -0.0159 | 0.4065 | 0.4224 | 0.4158 | 0.1223 | 0.0935 | yes |
| `skeleton` | -0.0159 | -0.0503 | -0.0344 | -0.0338 | 0.1223 | 0.1243 | no |
| `omega` | -0.0159 | -0.1113 | -0.0954 | -0.0939 | 0.1223 | 0.1279 | no |
| `orbit` | -0.0159 | 0.1224 | 0.1383 | 0.1362 | 0.1223 | 0.1136 | yes |
| `trajectory` | -0.0159 | 0.0052 | 0.0211 | 0.0208 | 0.1223 | 0.1210 | yes |
| `all_non_metadata` | -0.0159 | 0.3024 | 0.3183 | 0.3134 | 0.1223 | 0.1013 | yes |

### Random forest

| Block | Metadata R^2 | Combined R^2 | Delta R^2 | Residual R^2 | Metadata RMSE | Combined RMSE | Adds signal? |
|-------|-------------|--------------|-----------|--------------|---------------|---------------|-------------|
| `geometry` | -0.0079 | -0.2101 | -0.2022 | -0.2006 | 0.1218 | 0.1334 | no |
| `face_geometry` | -0.0079 | 0.0230 | 0.0309 | 0.0306 | 0.1218 | 0.1199 | yes |
| `face_symplectic` | -0.0079 | 0.2408 | 0.2487 | 0.2468 | 0.1218 | 0.1057 | yes |
| `skeleton` | -0.0079 | -0.6260 | -0.6181 | -0.6133 | 0.1218 | 0.1547 | no |
| `omega` | -0.0079 | -0.2130 | -0.2051 | -0.2035 | 0.1218 | 0.1336 | no |
| `orbit` | -0.0079 | 0.1280 | 0.1359 | 0.1349 | 0.1218 | 0.1133 | yes |
| `trajectory` | -0.0079 | 0.0063 | 0.0142 | 0.0141 | 0.1218 | 0.1209 | yes |
| `all_non_metadata` | -0.0079 | 0.3225 | 0.3304 | 0.3278 | 0.1218 | 0.0999 | yes |

## Verdict

This endpoint-only residual check records endpoint-side association beyond metadata, not a candidate-proposer.
It does not produce a validated new `sys > 1` row and does not give a rule for proposing fresh candidates before inspecting `sys`, endpoint labels, producer identity, optimizer provenance, or HKO2024-derived status.
Use it as supporting/caveat evidence only.

Packet verdict: `no-search-output`.
