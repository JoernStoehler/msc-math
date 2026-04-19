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
| `geometry` | 0.4364 | 0.3257 | -0.1107 | -0.1965 | 0.0911 | 0.0996 | no |
| `face_geometry` | 0.4364 | 0.4472 | 0.0108 | 0.0192 | 0.0911 | 0.0902 | yes |
| `face_symplectic` | 0.4364 | 0.5549 | 0.1185 | 0.2103 | 0.0911 | 0.0809 | yes |
| `skeleton` | 0.4364 | 0.3760 | -0.0605 | -0.1073 | 0.0911 | 0.0958 | no |
| `omega` | 0.4364 | 0.4136 | -0.0228 | -0.0405 | 0.0911 | 0.0929 | no |
| `orbit` | 0.4364 | 0.3779 | -0.0586 | -0.1039 | 0.0911 | 0.0957 | no |
| `trajectory` | 0.4364 | 0.4626 | 0.0261 | 0.0464 | 0.0911 | 0.0889 | yes |
| `all_non_metadata` | 0.4364 | 0.3496 | -0.0868 | -0.1540 | 0.0911 | 0.0978 | no |

### Random forest

| Block | Metadata R^2 | Combined R^2 | Delta R^2 | Residual R^2 | Metadata RMSE | Combined RMSE | Adds signal? |
|-------|-------------|--------------|-----------|--------------|---------------|---------------|-------------|
| `geometry` | 0.4377 | 0.2340 | -0.2037 | -0.3623 | 0.0910 | 0.1062 | no |
| `face_geometry` | 0.4377 | 0.3623 | -0.0754 | -0.1341 | 0.0910 | 0.0969 | no |
| `face_symplectic` | 0.4377 | 0.4388 | 0.0011 | 0.0020 | 0.0910 | 0.0909 | yes |
| `skeleton` | 0.4377 | 0.2972 | -0.1405 | -0.2499 | 0.0910 | 0.1017 | no |
| `omega` | 0.4377 | 0.3359 | -0.1018 | -0.1810 | 0.0910 | 0.0989 | no |
| `orbit` | 0.4377 | 0.3521 | -0.0856 | -0.1522 | 0.0910 | 0.0976 | no |
| `trajectory` | 0.4377 | 0.4568 | 0.0191 | 0.0339 | 0.0910 | 0.0894 | yes |
| `all_non_metadata` | 0.4377 | 0.3985 | -0.0392 | -0.0697 | 0.0910 | 0.0941 | no |

