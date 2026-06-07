---
method: pca-projection
description: >
  Fits a standardized PCA projection to scalar intrinsic retained polytope
  columns, excluding sys, capacity, ids, raw arrays, orbit/sigma witness
  columns, endpoint labels, dataset identity, and optimizer provenance. Fixed
  PCA-score regions are then audited against sys only after fitting.
result: >
  Current retained-dataset PCA run completed locally in 2.707 seconds wall
  time, with a PC2-high audit in 2.803 seconds wall time and a component
  interpretation run in 2.938 seconds wall time. Reading the loading vectors,
  PC1 is dominated by volume-normalized Euclidean geometry summaries, not by a
  meaningful scale direction for `sys`. PC2 is dominated by near-zero ridge
  symplectic-area columns and transition-bidirectionality columns. The PC2-high
  region enriches already evaluated high-`sys` rows, including inside
  `gradient_ascent_products`, but post-fit source-label audits show that this
  is not a source-independent PCA rule. The run records no current
  candidate-proposer and no validated new row.
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

Observed wall runtime on 2026-06-06: `2.707s`. The full retained-dataset run is
within the accepted local reproducibility bound.

PC2-high audit command:

```bash
time uv run --script experiments/sys-landscape/datascience/methods/pca-projection/interpret_pc2_high.py \
  --dataset experiments/sys-landscape/datascience/dataset \
  --summary experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json \
  --output experiments/sys-landscape/datascience/methods/pca-projection/pc2-high-audit.json
```

Observed wall runtime on 2026-06-06: `2.803s`.

This command consumes `pca-summary.json` for dataset fingerprint checks,
included/excluded feature policy checks, PC2 loading columns, and the
candidate-region p-value. It recomputes the same PCA scores from the retained
dataset and writes the compact audit artifact `pc2-high-audit.json`.

Component interpretation command:

```bash
time uv run --script experiments/sys-landscape/datascience/methods/pca-projection/interpret_components.py \
  --dataset experiments/sys-landscape/datascience/dataset \
  --summary experiments/sys-landscape/datascience/methods/pca-projection/pca-summary.json \
  --output experiments/sys-landscape/datascience/methods/pca-projection/component-interpretation.json
```

Observed wall runtime on 2026-06-06: `2.938s`.

This command consumes `pca-summary.json`, checks the retained-dataset
fingerprint and feature policy, recomputes PCA scores, and writes
feature-family loading, source-label audit, source-local region, and PC-`sys`
association audits to `component-interpretation.json`.

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

The strongest PC1 loadings are broad volume-normalized Euclidean geometry summaries:
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

The PC2-high audit is
`experiments/sys-landscape/datascience/methods/pca-projection/pc2-high-audit.json`.
It records these additional observations:

| Row set | `gradient_ascent_general` | `gradient_ascent_products` | `random_product_sample` | `random_sample` | `variable_f_ascent` |
| --- | ---: | ---: | ---: | ---: | ---: |
| All retained rows | `4096` | `4089` | `100` | `70` | `90` |
| Global top-1% `sys` rows | `5` | `80` | `0` | `0` | `0` |
| PC2-high rows | `0` | `337` | `62` | `24` | `0` |
| PC2-high global top-1% rows | `0` | `20` | `0` | `0` | `0` |

Within `gradient_ascent_products`, PC2-high selects `337/4089` rows. It
captures `20/80` global top-1% rows from that source, versus `6.59` expected
from a same-source random subset of size `337` (`3.03x`, hypergeometric
`p=4.26e-06`). It also captures `14/41` source-local top-1% rows, versus
`3.38` expected (`4.14x`, `p=2.31e-06`).

The PC2-high p-value remains small after a narrow Bonferroni correction over
the `17` fixed candidate regions in `pca-summary.json`: raw `4.34e-09`,
adjusted `7.38e-08`. This correction does not cover all exploratory choices
made while interpreting the signal.

The signal is not monotone in PC2. Among all retained rows, the top `0.5%` and
top `1%` by PC2 capture `0` global top-1% `sys` rows. The top `2%`, `5%`,
`10%`, and `20%` by PC2 capture `6`, `20`, `35`, and `54` global top-1% rows,
respectively. The most extreme PC2 rows include many `random_product_sample`
rows, whose highest-PC2 deciles have low `sys`.

The top PC2 loading columns are near-zero symplectic-area fractions and
transition bidirectionality. For example,
`ridge_symp_area_volnorm_zero_fraction` has mean `0.146` globally, `0.308` in
PC2-high, and `0.323` in PC2-high global top-1% rows. The matching
`gradient_ascent_products` means are `0.322` inside PC2-high and `0.291`
outside PC2-high. The full per-column audit is in `pc2-high-audit.json`.

The component interpretation artifact is
`experiments/sys-landscape/datascience/methods/pca-projection/component-interpretation.json`.
It records:

| Component | Variance | Source eta-squared | Main loading families |
| --- | ---: | ---: | --- |
| PC1 | `0.328` | `0.337` | `geom 0.33`, `ridge 0.24`, `facet 0.17`, `edge 0.11` |
| PC2 | `0.197` | `0.889` | `ridge 0.53`, `transition 0.11`, `facet 0.10`, `geom 0.08` |
| PC3 | `0.115` | `0.036` | `facet 0.56`, `ridge 0.22`, `geom 0.09`, `transition 0.07` |
| PC4 | `0.070` | `0.112` | `ridge 0.26`, `geom 0.24`, `facet 0.18`, `edge 0.12` |
| PC5 | `0.035` | `0.036` | `transition 0.43`, `ridge 0.27`, `facet 0.11`, `allpair 0.08` |
| PC6 | `0.031` | `0.036` | `ridge 0.28`, `geom 0.23`, `edge 0.17`, `facet 0.16` |

The loading-family columns are the primary component interpretation in this
report. Source eta-squared is a post-fit audit: it is the fraction of PC-score
variance explained by the observation-table `dataset` label after fitting PCA
without that label as an input. It is used to detect whether a pooled PCA
coordinate is specific to retained data subsets, not to define what the
component means.

PC2 source-score means are separated by source:

| Source | Rows | PC2 mean | PC2 p05 | PC2 p95 |
| --- | ---: | ---: | ---: | ---: |
| `gradient_ascent_general` | `4096` | `-3.714` | `-4.857` | `-2.602` |
| `gradient_ascent_products` | `4089` | `3.585` | `1.476` | `5.168` |
| `random_product_sample` | `100` | `8.163` | `-1.517` | `17.301` |
| `random_sample` | `70` | `2.637` | `-7.784` | `14.415` |
| `variable_f_ascent` | `90` | `-4.993` | `-6.438` | `-3.687` |

Within `gradient_ascent_products`, the source-local top-5% PC2-high region
selects `205/4089` rows and captures `12/41` source-local top-1% `sys` rows.
A same-source random region of this size would capture `2.06` such rows in
expectation; the enrichment is `5.84x`, with hypergeometric `p=3.99e-07`.
No other source-local top-5% or bottom-5% region among PC1-PC6 has comparable
top-tail capture.

PC-`sys` correlations are descriptive audits on already evaluated rows. The
largest product-family correlations are:

- PC5 inside `gradient_ascent_products`: Pearson `0.524`, Spearman `0.509`;
- PC1 inside `gradient_ascent_products`: Pearson `-0.622`, Spearman `-0.355`;
- PC2 inside `gradient_ascent_products`: Pearson `0.274`, Spearman `0.312`.

Despite the PC5 correlation, the source-local top-5% PC5-high region inside
`gradient_ascent_products` captures only `2/41` source-local top-1% `sys`
rows, close to the `2.06` random expectation. In this retained dataset, PC5 is
a broad within-product association, not a top-tail proposal rule.

## Inference

Pooled PCA applied to the retained dataset gives this direct component reading:
PC1 is dominated by volume-normalized Euclidean geometry summaries, while PC2
is dominated by near-zero ridge symplectic-area columns and
transition-bidirectionality columns. The PC2-high region enriches already
evaluated high-`sys` rows. Source labels were not PCA inputs; they were used
only after fitting to audit whether this pooled PCA signal is shared across
retained data subsets.

PC2-high has statistically meaningful descriptive enrichment among already
evaluated retained rows. It captures `20/85` top-1% rows in a `423/8445` row
region, compared with `4.26` expected by random selection. This is `4.70x`
enrichment with hypergeometric p-value `4.34e-09`.

The PC2-high audit adds these supported facts:

- PC2-high contains `337` `gradient_ascent_products` rows, `62`
  `random_product_sample` rows, `24` `random_sample` rows, and no
  `gradient_ascent_general` or `variable_f_ascent` rows.
- All `20` PC2-high rows that are also global top-1% in `sys` are
  `gradient_ascent_products` rows.
- PC2-high is not explained only by selecting `gradient_ascent_products`.
  Within `gradient_ascent_products`, PC2-high captures `14/41` source-local
  top-1% `sys` rows, compared with `3.38` expected from a same-source random
  subset.
- PC2-high is not a clean monotone score rule. The most extreme PC2 rows do
  not contain high-`sys` rows, so a candidate rule would need a band or
  source-specific interface that has not been specified before `sys` audit.

The loading vectors support this component reading:

- PC1 is mostly a volume-normalized Euclidean geometry-summary direction. It
  explains `32.8%` of allowed-feature variance, has largest loading-family
  mass in `geom`, `ridge`, `facet`, and `edge` columns, and is negatively
  correlated with `sys` globally.
- PC2 is mainly a near-zero-ridge symplectic direction. Post-fit
  source-label audits show that this pooled-PCA coordinate is not
  source-independent. It explains `19.7%` of allowed-feature variance, has
  `53%` of squared loading mass in `ridge` columns, and its largest loadings
  are near-zero ridge symplectic-area fractions and transition
  bidirectionality.
- The source-label audit is a caveat, not the component interpretation.
  PC2 has source eta-squared `0.889`: its high side contains product rows and
  random product-like rows, while its low side contains general and variable-f
  ascent rows. Within `gradient_ascent_products`, the source-local PC2-high
  top-5% region still captures `12/41` source-local top-1% `sys` rows, versus
  `2.06` expected from a same-source random subset.
- PC5 has the strongest monotone product-family PC-`sys` correlation among
  the first six PCs, but its source-local top-5% region does not capture a
  top-tail excess. This makes it descriptive association evidence, not a
  candidate-proposer.

This is not a current candidate-proposer under the method-table definition.
The committed run audits fixed PCA-score regions inside an already evaluated
retained dataset. It does not provide an unevaluated candidate pool, a generator
for new polytopes, or a pre-registered interface that scores unevaluated rows
before their `sys` values are evaluated.

PC2-high is therefore at most a candidate-proposer hypothesis and future reopen
trigger, not a current positive search-method row. Future positive work would
need to specify, before `sys` audit, how unevaluated polytopes or rows are
generated, which allowed columns are computed for them, how the PCA transform
or PC2 band rule is fixed, and how proposed rows are evaluated.

The audit does not support a claim that PCA is uninformative. It supports the
narrower claim that this retained-dataset PCA work found a real descriptive
near-zero-ridge symplectic-area pattern but no current candidate-proposer, no
validated `sys > 1` row, and no source-independent high-`sys` PCA rule.

## Report-Local Result

This section is the report's proposed evidence classification and next action.
Approved method-row status lives in `../STATUS.md`.

Report-local classification:

```text
current retained-dataset descriptive evidence; no current candidate-proposer;
no validated new row
```

Supported local facts:

- no current candidate-proposer;
- no validated new row;
- descriptive near-zero-ridge PC2 signal among already evaluated rows;
- future reopen trigger for a product-family PCA-band candidate-proposer.

## Thesis Use

This work is usable as current retained-dataset descriptive evidence for the
PCA row, subject to the caveats below.

- Reproducible observation: PC2-high concentrates high retained `sys` rows
  among already evaluated rows, mainly inside `gradient_ascent_products`.
- Interpretation: PC2 is mostly a near-zero-ridge symplectic-area direction.
  Post-fit source-label audits show that the pooled-PC2 signal is not
  source-independent, but it remains enriched inside `gradient_ascent_products`.
- Positive-but-limited fact: PCA found a real descriptive pattern in the
  retained data, not an empty or purely null result.
- Reproducible negative fact: this PCA work commits no candidate-proposer and
  no validated `sys > 1` row.
- Caveat: do not phrase this as a finished PCA verdict, a clean negative PCA
  result, an impossibility result, a density statement, a source-independent
  pattern, a monotone score rule, or an exhaustive PCA-region search.
- Caveat: this pooled PCA run does not answer the separate question whether
  PCAs trained on individual retained sources, or transferred across source
  subsets, reveal shared geometry useful for candidate proposal.

Status wording for the method table or queue:

```text
Pooled PCA projection on retained scalar table columns found a descriptive
near-zero-ridge symplectic-area signal in PC2. The PC2-high region enriches
already evaluated top-sys rows, including inside gradient_ascent_products, but
post-fit source-label audits show that this is not a source-independent PCA
rule. The run found no current
candidate-proposer and no validated sys > 1 row.
```

## Reopen Condition

Reopen or continue this row if the retained dataset changes materially, if a
pre-registered PCA-score proposal rule is specified before `sys` audit, if a
new candidate-generation interface can turn PCA-score directions into
unevaluated polytopes without using endpoint labels, optimizer provenance,
dataset identity, capacity columns, or post-hoc `sys` inspection, or if
per-source or cross-source-transfer PCA has enough value of information after
higher-priority method rows are handled.

The concrete split follow-up, if reopened, is a product-family PCA-band
candidate-proposer. It must define the unevaluated row pool, allowed feature
computation, fixed PCA transform, PC2 band or threshold, and evaluation
protocol before inspecting new `sys` values. Defer this follow-up unless the
remaining method-table work still leaves a product-family candidate-proposer
gap.
