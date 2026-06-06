---
method: pca-projection
description: >
  Fits a standardized PCA projection to scalar intrinsic retained polytope
  columns, excluding sys, capacity, ids, raw arrays, orbit/sigma witness
  columns, endpoint labels, dataset identity, and optimizer provenance. Fixed
  PCA-score regions are then audited against sys only after fitting.
result: >
  Current retained-dataset PCA run completed locally in 2.707 seconds wall
  time, with a PC2-high audit in 2.803 seconds wall time.
  PC2-high is statistically meaningful descriptive enrichment among already
  evaluated retained rows, mainly inside `gradient_ascent_products`. Within
  `gradient_ascent_products`, PC2-high still enriches source-local top-1% sys
  rows. This is partial/status evidence, not finished current method-table
  evidence: the report does not yet explain what PCA applied to this project
  tells us at method level. It records no current candidate-proposer and no
  validated new row.
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

## Partial Inference

This report does not yet give a thesis-usable answer to the method-level
question "what does PCA applied to this project tell us?" The committed
artifacts support narrower statements about one PCA run and one audited region.

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

The current component interpretation is incomplete. The report identifies the
largest PC1 coefficients as size and scale columns, and the largest PC2
coefficients as columns for near-zero ridge symplectic areas, near-zero
ridge-pair `omega` values, and transition bidirectionality. Since PCA
components usually have nonzero coefficients on many columns, this is not by
itself a geometric interpretation of the components. The report does not yet
establish whether PC2 mainly measures producer family, a product-specific
geometric condition, a near-degeneracy pattern, a combination of these, or
something else.

This is not a current candidate-proposer under the method-table definition.
The committed run audits fixed PCA-score regions inside an already evaluated
retained dataset. It does not provide an unevaluated candidate pool, a generator
for new polytopes, or a pre-registered interface that scores unevaluated rows
before their `sys` values are evaluated.

PC2-high is therefore at most a candidate-proposer hypothesis and future reopen
trigger, not a current positive method-table row. Future positive work would
need to specify, before `sys` audit, how unevaluated polytopes or rows are
generated, which allowed columns are computed for them, how the PCA transform
or PC2 band rule is fixed, and how proposed rows are evaluated.

The audit does not support a claim that PCA regions are uninformative. It does
support the narrower claim that this current PCA projection work found no
validated `sys > 1` row and no current candidate-generation interface.

The method-level interpretation remains open. Converting this partial evidence
into current method-table evidence would require a report-level answer to what
PCA reveals about the retained search landscape, including whether the
principal components mostly recover producer/source structure, geometric
structure, both, or an unresolved mixture. If that interpretation is not worth
the cost, the PCA row should be explicitly deferred or abandoned rather than
presented as finished.

## Report-Local Result

This section is the report's proposed evidence classification and next action.
Approved method-row status lives in `../STATUS.md`.

Report-local classification:

```text
partial/status evidence; method-level PCA interpretation incomplete
```

Supported local facts:

- no current candidate-proposer;
- no validated new row;
- supporting evidence only;
- future reopen trigger.

## Thesis Use

This work is not yet finished current method-table evidence.

Thesis writers may use it only as partial/status evidence:

- Reproducible observation: PC2-high concentrates high retained `sys` rows
  among already evaluated rows, mainly inside `gradient_ascent_products`.
- Reproducible negative fact: this PCA work commits no candidate-proposer and
  no validated `sys > 1` row.
- Open interpretation: the report does not yet explain what PCA applied to
  this project tells us at method level.
- Caveat: do not phrase this as a finished PCA verdict, a clean negative PCA
  result, an impossibility result, a density statement, a source-independent
  pattern, a monotone score rule, or an exhaustive PCA-region search.

Status wording for the method table or queue:

```text
PCA projection has reproducible partial evidence: PC2-high enriches already
evaluated high-sys rows, mainly inside gradient_ascent_products, and the run
found no candidate-proposer or validated sys > 1 row. The PCA method-level
interpretation remains open, so this is not yet a finished current method-table
evidence row.
```

## Reopen Condition

Reopen or continue this row if the retained dataset changes materially, if a
method-level PCA interpretation is needed for thesis use, if a pre-registered
PCA-score proposal rule is specified before `sys` audit, or if a new candidate
generation interface can turn PCA-score directions into unevaluated polytopes
without using endpoint labels, optimizer provenance, dataset identity, capacity
columns, or post-hoc `sys` inspection.

The concrete split follow-up, if reopened, is a product-family PCA-band
candidate-proposer. It must define the unevaluated row pool, allowed feature
computation, fixed PCA transform, PC2 band or threshold, and evaluation
protocol before inspecting new `sys` values. Defer this follow-up unless the
remaining method-table work still leaves a product-family candidate-proposer
gap.
