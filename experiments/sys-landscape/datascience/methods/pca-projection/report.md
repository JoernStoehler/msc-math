---
method: pca-projection
description: >
  Fits a standardized PCA projection to scalar intrinsic retained polytope
  columns, excluding sys, capacity, ids, raw arrays, orbit/sigma witness
  columns, endpoint labels, dataset identity, and optimizer provenance. Fixed
  PCA-score regions are then audited against sys only after fitting.
result: >
  Current retained-dataset run completed locally in 2.921 seconds wall time.
  PC2-high is statistically meaningful descriptive enrichment among already
  evaluated retained rows, but this work does not contain a current
  candidate-generation interface. Primary result/status label: ran with no
  candidate-proposer and no new validated row. Secondary labels: supporting
  evidence only; future reopen trigger. This is not a clean "no patterns found"
  result.
---

# PCA Projection

## Dataset And Command

Dataset path:

```text
experiments/sys-landscape/datascience/dataset/
```

Fingerprint used by the run:

- polytope rows: `8445`
- observation rows: `8445`
- max `sys`: `0.9750768559799221`
- `sys > 1` rows: `0`
- `polytope-table.jsonl` sha256: `bc96000d2c7a70c4aa777891a020bf3c8f7d11d8ee17a084519e2706ce2b4554`
- `observation-table.jsonl` sha256: `5382d131dadb4f220512015e876e65566fee51d7c2a25521f7c891c2db8450ce`
- source counts: `gradient_ascent_general=4096`,
  `gradient_ascent_products=4089`, `random_product_sample=100`,
  `random_sample=70`, `variable_f_ascent=90`

Reproduction command:

```bash
time uv run --script experiments/sys-landscape/datascience/methods/pca-projection/analyze.py \
  --dataset experiments/sys-landscape/datascience/dataset \
  --output experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json
```

Observed wall runtime on 2026-06-05: `2.921s`. The full retained-dataset run is
within the accepted local reproducibility bound. The JSON-internal analysis
runtime was `2.556s`.

Smoke command used during development:

```bash
time uv run --script experiments/sys-landscape/datascience/methods/pca-projection/analyze.py \
  --limit 200 \
  --output /tmp/pca-projection-smoke.json
```

Observed wall runtime on 2026-06-05: `0.330s`.

## Validity Guard

The candidate-proposer guard is that any proposed row set must be definable
before `sys` is inspected. This work therefore fits PCA only on allowed
scalar intrinsic polytope columns and uses `sys` only after fitting as audit and
interpretation.

Included input columns:

- `80` nonconstant scalar intrinsic columns;
- combinatorial columns such as `facet_count`, `vertex_count`, `edge_count`,
  `ridge_count`, `dual_vertex_count`, density, and facet/vertex/ridge summary
  counts;
- Euclidean and symplectic geometry summaries with prefixes `geom_`,
  `edge_`, `facet_`, `ridge_`, `allpair_`, and `transition_`;
- `volume`.

Excluded input columns:

- target/capacity/identity/raw columns: `sys`, `capacity`, `capacity_source`,
  `poly_id`, raw vertex arrays, `sigma_gap_cutoff`, `sigmas`,
  `raw_orbit_scalars`;
- capacity-search witness columns with prefix `orbit_`;
- `capacity_iterations`;
- constant columns, including `is_simple`, `simple_vertex_fraction`, and
  constant vertex-degree/incidence summaries;
- all observation-table provenance and endpoint-label fields, including
  `dataset`, `family`, `role`, `optimizer`, `backend`, `source_name`,
  `root_group_id`, `lineage_id`, and trajectory fields.

The stale `pca-cluster-anomaly/` bundle was not rerun and no code or result was
extracted from it.

## Observation

The first six PCA explained-variance ratios are:

```text
PC1 0.3284
PC2 0.1972
PC3 0.1145
PC4 0.0701
PC5 0.0349
PC6 0.0308
```

The strongest PC1 loadings are broad size/scale geometry summaries:
`geom_vol1_pairwise_dist_mean`, `facet_volume_vol1_sum`,
`geom_vol1_norm_mean`, `facet_volume_vol1_mean`,
`ridge_abs_omega_vol1_mean`, `allpair_abs_omega_vol1_mean`,
`geom_vol1_norm_max`, and `allpair_abs_omega_vol1_max`.

The strongest PC2 loadings are small symplectic-area or near-zero omega
fractions and transition bidirectionality:
`ridge_symp_area_volnorm_zero_fraction`,
`ridge_abs_omega_vol1_le_1em3_fraction`,
`ridge_abs_omega_vol1_le_1em2_fraction`,
`ridge_abs_omega_vol1_le_1em1_fraction`,
`ridge_symp_area_volnorm_le_1em3_fraction`,
`ridge_symp_area_volnorm_le_1em2_fraction`,
`transition_bidirectional_given_facet_intersection_fraction`, and
`ridge_zero_fraction`.

The audited `sys` distribution has global mean `0.756982`, p90 `0.859590`,
p99 `0.912872`, top-1% threshold `0.913102`, top-1% row count `85`, and max
`0.975077`.

The committed fixed-region audit now covers all six computed PCs and cumulative
PCA-score radii from PC1-PC2 through PC1-PC6. Each region selects `423` rows,
or `5.01%` of the retained dataset. A random region of this size is expected
to capture `4.26` of the `85` top-1% rows. The hypergeometric p-value is
`P(X >= observed)` under random selection of `423` rows from the retained
dataset.

| Region | Max sys | Mean sys | p90 sys | Top-1% captured | Enrichment | p-value |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| PC1 high | `0.875133` | `0.499180` | `0.778924` | `0` | `0.00x` | `1.00` |
| PC1 low | `0.944267` | `0.818344` | `0.882917` | `8` | `1.88x` | `6.20e-02` |
| PC2 high | `0.960570` | `0.666710` | `0.879253` | `20` | `4.70x` | `4.34e-09` |
| PC2 low | `0.896303` | `0.741667` | `0.835445` | `0` | `0.00x` | `1.00` |
| PC3 high | `0.943176` | `0.686071` | `0.853535` | `7` | `1.64x` | `1.33e-01` |
| PC3 low | `0.935247` | `0.728157` | `0.876177` | `3` | `0.70x` | `8.06e-01` |
| PC4 high | `0.916527` | `0.699551` | `0.861162` | `2` | `0.47x` | `9.31e-01` |
| PC4 low | `0.927901` | `0.625916` | `0.812196` | `1` | `0.23x` | `9.88e-01` |
| PC5 high | `0.949887` | `0.741730` | `0.870422` | `4` | `0.94x` | `6.22e-01` |
| PC5 low | `0.909015` | `0.603484` | `0.813283` | `0` | `0.00x` | `1.00` |
| PC6 high | `0.927901` | `0.669317` | `0.825179` | `2` | `0.47x` | `9.31e-01` |
| PC6 low | `0.932218` | `0.660553` | `0.843824` | `1` | `0.23x` | `9.88e-01` |
| PC1-PC2 radius high | `0.875133` | `0.503388` | `0.778924` | `0` | `0.00x` | `1.00` |
| PC1-PC3 radius high | `0.941291` | `0.503524` | `0.821400` | `4` | `0.94x` | `6.22e-01` |
| PC1-PC4 radius high | `0.935247` | `0.497761` | `0.793854` | `1` | `0.23x` | `9.88e-01` |
| PC1-PC5 radius high | `0.935247` | `0.481965` | `0.785453` | `1` | `0.23x` | `9.88e-01` |
| PC1-PC6 radius high | `0.935247` | `0.484354` | `0.782351` | `1` | `0.23x` | `9.88e-01` |

The machine-readable run summary is
`experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json`.

## Inference

PCA finds interpretable low-dimensional axes, mainly size/scale geometry and
near-zero symplectic-area or transition features.

PC2-high is statistically meaningful descriptive enrichment among already
evaluated retained rows. It captures `20/85` top-1% rows in a `423/8445` row
region, compared with `4.26` expected by random selection. This is `4.70x`
enrichment with hypergeometric p-value `4.34e-09`.

This is not a current candidate-proposer under the method-table definition.
The committed run audits fixed PCA-score regions inside an already evaluated
retained dataset. It does not provide an unevaluated candidate pool, a generator
for new polytopes, or a pre-registered interface that scores unevaluated rows
before their `sys` values are evaluated.

PC2-high is therefore a candidate-proposer hypothesis and future follow-up
trigger, not a current positive method-table row. Future positive work would
need to specify, before `sys` audit, how unevaluated polytopes or rows are
generated, which allowed columns are computed for them, how the PCA transform
or PC2-high rule is fixed, and how proposed rows are evaluated.

The audit does not support a claim that PCA regions are uninformative. It does
support the narrower claim that this current PCA projection work found no
validated `sys > 1` row and no current candidate-generation interface.

## Current Result/Status Label

Primary result/status label:

```text
ran with no candidate-proposer and no new validated row
```

Secondary labels:

```text
supporting evidence only
future reopen trigger
```

## Thesis Use

This work is directly usable as mixed current method-table evidence:

- Negative current method-table verdict: no candidate-proposer and no validated
  new row are committed here.
- Positive descriptive finding: PC2-high concentrates high retained `sys` rows
  among already evaluated rows.
- Caveat: do not phrase this as a clean negative PCA result, an impossibility
  result, a density statement, or an exhaustive PCA-region search.

Recommended row wording:

```text
PCA projection on allowed retained scalar columns found statistically
meaningful PC2-high enrichment among already evaluated high-sys rows, but the
work did not define a candidate-generation interface for unevaluated rows and
found no validated sys > 1 row.
```

## Reopen Condition

Reopen this row if the retained dataset changes materially, if a pre-registered
PCA-score proposal rule is specified before `sys` audit, or if a new candidate
generation interface can turn PCA-score directions into unevaluated polytopes
without using endpoint labels, optimizer provenance, dataset identity, capacity
columns, or post-hoc `sys` inspection.
