# Regime Classification Summary

## Dataset

- dataset source: `/tmp/sys-ds-reset-pilot-tables-VJ6D0P`
- joined rows: `282`
- groups used for leakage control: `212`
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
- `facet_count`: one non-provenance combinatorial scalar from the polytope table
- `provenance_metadata`: dataset/family/role/search-space/optimizer/backend, excluding facet count
- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend
- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention
- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling
- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`
- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice
- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries
- `orbit_combinatorics`: cached-`best_sigma` support-size and cycle-structure summaries
- `orbit_geometry`: sigma-local dual-norm and cycle `omega_0` magnitude summaries
- `orbit_search`: bounded best-orbit KKT and search-scalar availability summaries
- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars
- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries
- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit_combinatorics, orbit_geometry, orbit_search, orbit, and trajectory together

## Metrics

Reported metrics are out-of-fold accuracy, balanced accuracy, and ROC AUC. Splits are grouped by persisted `root_group_id` when present, with the same lineage/source fallback used in the pattern-search pass.

### Logistic regression

| Block | Accuracy | Balanced acc. | ROC AUC |
|-------|----------|---------------|---------|
| `null` | 0.6028 | 0.5000 | 0.4946 |
| `facet_count` | 0.7872 | 0.8235 | 0.8248 |
| `provenance_metadata` | 1.0000 | 1.0000 | 1.0000 |
| `metadata` | 1.0000 | 1.0000 | 1.0000 |
| `geometry` | 0.8511 | 0.8719 | 0.9549 |
| `face_geometry` | 0.8121 | 0.8411 | 0.8772 |
| `face_symplectic` | 0.8227 | 0.8331 | 0.9166 |
| `skeleton` | 0.8723 | 0.8728 | 0.9223 |
| `omega` | 0.8511 | 0.8628 | 0.9244 |
| `orbit_combinatorics` | 0.7943 | 0.8127 | 0.9028 |
| `orbit_geometry` | 0.6809 | 0.6987 | 0.8026 |
| `orbit_search` | 0.9220 | 0.9018 | 0.9432 |
| `orbit` | 0.9220 | 0.9018 | 0.9605 |
| `trajectory` | 0.6809 | 0.5982 | 0.5982 |
| `all` | 1.0000 | 1.0000 | 1.0000 |

- best block by balanced accuracy: `provenance_metadata` (`balanced_accuracy=1.0000`, `roc_auc=1.0000`)
- best non-metadata block: `orbit` (`balanced_accuracy=0.9018`, `roc_auc=0.9605`)
- best non-provenance geometry/orbit block: `skeleton` (`balanced_accuracy=0.8728`, `roc_auc=0.9223`)

### Random forest

| Block | Accuracy | Balanced acc. | ROC AUC |
|-------|----------|---------------|---------|
| `null` | 0.6028 | 0.5000 | 0.4946 |
| `facet_count` | 0.8404 | 0.8676 | 0.9078 |
| `provenance_metadata` | 1.0000 | 1.0000 | 1.0000 |
| `metadata` | 1.0000 | 1.0000 | 1.0000 |
| `geometry` | 0.9255 | 0.9200 | 0.9847 |
| `face_geometry` | 0.9291 | 0.9275 | 0.9831 |
| `face_symplectic` | 0.9220 | 0.9155 | 0.9838 |
| `skeleton` | 0.8723 | 0.8743 | 0.9581 |
| `omega` | 0.9220 | 0.9170 | 0.9704 |
| `orbit_combinatorics` | 0.8511 | 0.8521 | 0.9410 |
| `orbit_geometry` | 0.8830 | 0.8572 | 0.9127 |
| `orbit_search` | 0.9113 | 0.9006 | 0.9636 |
| `orbit` | 0.9574 | 0.9464 | 0.9869 |
| `trajectory` | 0.6809 | 0.5982 | 0.5982 |
| `all` | 1.0000 | 1.0000 | 1.0000 |

- best block by balanced accuracy: `provenance_metadata` (`balanced_accuracy=1.0000`, `roc_auc=1.0000`)
- best non-metadata block: `orbit` (`balanced_accuracy=0.9464`, `roc_auc=0.9869`)
- best non-provenance geometry/orbit block: `face_geometry` (`balanced_accuracy=0.9275`, `roc_auc=0.9831`)

## Interpretation

- metadata is the clearest separator, but that block includes regime-linked provenance fields (`dataset`, `family`, `role`, `search_space`, `optimizer`, `backend`), so it is not a pure geometry test.
- among non-provenance blocks, logistic regression and random forest both favor `orbit`.
- compare `provenance_metadata`, `facet_count`, and the geometry/orbit sub-blocks before treating a high score as geometric association.
- `all` is only a ceiling because it mixes the provenance block with every feature family.
