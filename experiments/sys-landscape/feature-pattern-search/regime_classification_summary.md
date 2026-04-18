# Regime Classification Summary

## Dataset

- normalized input source: temporary refresh via `cargo run -p exp-sys-landscape --release --bin sys-normalized-dataset -- --out-dir /tmp/regime-classification-ra_v7ln5/normalized`
- joined rows: `282`
- groups used for leakage control: `202`
- random rows: `170`
- endpoint rows: `112`
- dataset counts:
  - `gradient_ascent_general`: `10`
  - `gradient_ascent_products`: `12`
  - `random_product_sample`: `100`
  - `random_sample`: `70`
  - `variable_f_ascent`: `90`

## Feature Blocks

- `null`: majority-class baseline with no features
- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend
- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention
- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling
- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars
- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries
- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit, and trajectory together

## Metrics

Reported metrics are out-of-fold accuracy, balanced accuracy, and ROC AUC. Splits are grouped by persisted `root_group_id` when present, with the same lineage/source fallback used in the pattern-search pass.

### Logistic regression

| Block | Accuracy | Balanced acc. | ROC AUC |
|-------|----------|---------------|---------|
| `null` | 0.6028 | 0.5000 | 0.4946 |
| `metadata` | 1.0000 | 1.0000 | 1.0000 |
| `geometry` | 0.8511 | 0.8704 | 0.9619 |
| `face_geometry` | 0.8156 | 0.8425 | 0.8801 |
| `face_symplectic` | 0.8121 | 0.8228 | 0.9195 |
| `skeleton` | 0.8723 | 0.8743 | 0.9211 |
| `omega` | 0.8511 | 0.8628 | 0.9289 |
| `orbit` | 0.9220 | 0.9018 | 0.9611 |
| `trajectory` | 0.6809 | 0.5982 | 0.5982 |
| `all` | 1.0000 | 1.0000 | 1.0000 |

- best block by balanced accuracy: `metadata` (`balanced_accuracy=1.0000`, `roc_auc=1.0000`)
- best non-provenance block: `orbit` (`balanced_accuracy=0.9018`, `roc_auc=0.9611`)

### Random forest

| Block | Accuracy | Balanced acc. | ROC AUC |
|-------|----------|---------------|---------|
| `null` | 0.6028 | 0.5000 | 0.4946 |
| `metadata` | 1.0000 | 1.0000 | 1.0000 |
| `geometry` | 0.9184 | 0.9080 | 0.9798 |
| `face_geometry` | 0.9255 | 0.9261 | 0.9835 |
| `face_symplectic` | 0.9255 | 0.9200 | 0.9852 |
| `skeleton` | 0.9078 | 0.9053 | 0.9689 |
| `omega` | 0.9255 | 0.9200 | 0.9691 |
| `orbit` | 0.9574 | 0.9464 | 0.9912 |
| `trajectory` | 0.6809 | 0.5982 | 0.5982 |
| `all` | 1.0000 | 1.0000 | 1.0000 |

- best block by balanced accuracy: `metadata` (`balanced_accuracy=1.0000`, `roc_auc=1.0000`)
- best non-provenance block: `orbit` (`balanced_accuracy=0.9464`, `roc_auc=0.9912`)

## Interpretation

- metadata is the clearest separator, but that block includes regime-linked provenance fields (`dataset`, `family`, `role`, `search_space`, `optimizer`, `backend`), so it is not a pure geometry test.
- among non-provenance blocks, logistic regression and random forest both favor `orbit`.
- `orbit` is the strongest non-provenance separator; `face_symplectic` and `omega` are the cleanest pure geometric blocks behind it, while `skeleton` and `trajectory` are weak separators on this task.
- `all` is only a ceiling because it mixes the provenance block with every feature family.
