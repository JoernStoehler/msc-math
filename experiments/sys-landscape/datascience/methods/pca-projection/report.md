---
method: pca-projection
description: >
  Fits a standardized PCA projection to scalar intrinsic retained polytope
  columns, excluding sys, capacity, ids, raw arrays, orbit/sigma witness
  columns, endpoint labels, dataset identity, and optimizer provenance. Fixed
  PCA-score regions are then audited against sys only after fitting.
result: >
  Current retained-dataset run completed locally in 2.8 seconds. The audited
  PCA regions show weak descriptive structure but do not supply a reproducible
  candidate-proposer for new high-sys polytopes; terminal state: ran with no
  candidate-proposer and no new validated row.
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

Observed wall runtime: `2.835s`. The full retained-dataset run is within the
accepted local reproducibility bound.

Smoke command used during development:

```bash
time uv run --script experiments/sys-landscape/datascience/methods/pca-projection/analyze.py \
  --limit 200 \
  --output /tmp/pca-projection-smoke.json
```

Observed wall runtime: `0.261s`.

## Validity Guard

The candidate-proposer guard is that any proposed row set must be definable
before `sys` is inspected. This packet therefore fits PCA only on allowed
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

The stale `pca-cluster-anomaly/` packet was not rerun and no code or result was
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

The audited `sys` distribution has global mean `0.756982`, p90 `0.859590`, p99
`0.912872`, and max `0.975077`. Fixed pre-`sys` PCA candidate regions, each
selecting about `5.01%` of rows (`423` rows), gave:

| Region | Max sys | Mean sys | p90 sys | Top-1% rows captured |
| --- | ---: | ---: | ---: | ---: |
| PC1 high | `0.875133` | `0.499180` | `0.778924` | `0` |
| PC1 low | `0.944267` | `0.818344` | `0.882917` | `8` |
| PC2 high | `0.960570` | `0.666710` | `0.879253` | `20` |
| PC2 low | `0.896303` | `0.741667` | `0.835445` | `0` |
| PC1/PC2 radius high | `0.875133` | `0.503388` | `0.778924` | `0` |

The machine-readable run summary is
`experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json`.

## Inference

PCA finds interpretable low-dimensional axes, mainly size/scale geometry and
near-zero symplectic-area or transition features. These axes are descriptive
but not enough for a candidate-proposer. The best fixed region by top-1%
capture, PC2 high, captures only `20` of the retained top-1% `sys` rows and
misses the global maximum. PC1 low has an elevated mean `sys`, but it captures
only `8` top-1% rows and also misses the global maximum.

Selecting a narrower or different PCA-score rule because it looks better after
this audit would be post-hoc `sys` inspection. This packet therefore does not
claim a PCA-derived proposal rule.

## Terminal State

Primary terminal state:

```text
ran with no candidate-proposer and no new validated row
```

Secondary label:

```text
supporting evidence only
```

## Thesis Use

This row can support the closed method table as a negative PCA projection
packet: a standard low-dimensional linear projection of allowed retained
columns was run on the current retained dataset and did not yield a valid
candidate-proposer or any validated new `sys > 1` row.

It should not be phrased as an impossibility, density statement, or exhaustive
search claim.

## Reopen Condition

Reopen this row if the retained dataset changes materially, if a pre-registered
PCA-score proposal rule is specified before `sys` audit, or if a new candidate
generation interface can turn PCA-score directions into unevaluated polytopes
without using endpoint labels, optimizer provenance, dataset identity, capacity
columns, or post-hoc `sys` inspection.
