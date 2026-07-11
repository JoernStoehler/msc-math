# Residual Exemplar Seeds

Date: 2026-07-11.

Status: current retained-table inspection-candidate packet. The generated
artifacts are post-target exploratory evidence (`G` only). They do not
demonstrate a feature-map failure or omitted variable, validate a candidate
proposer, identify a mechanism, transfer to another generator, or supply
theorem evidence.

## Research Question

Which trusted retained random/product rows are useful post-target inspection
candidates where the current invariant summaries or two ordinary models leave
target variation unexplained?

The packet supplies concrete inputs for bounded incidence-aware and HK-branch
inspection. Such inspection may generate a feature or mechanism hypothesis;
the packet does not establish in advance that one exists. It is not another
search for the best in-table predictor.

## Inputs And Identity

The frozen input is the reviewed current-schema P2 table reconstructed from the
canonical random/product producers:

- `polytope-table.jsonl`: 14,336 rows, sha256
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`: 14,336 rows, sha256
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.

The analyzer fails on another input identity. Rebuild the table through the
`standard-baseline-p2/` input contract; do not use the stale prepared LFS table
described by that owner.

The 45 method-facing covariates comprise 27 current combinatorial summaries and
18 symplectic two-face-area summaries. They are invariant scalar summaries,
not a complete description of the face lattice, area field, or symplectic
geometry.

## F3: Target-Free Pair Construction

`sys` is excluded from pair formation, distance fitting, PCA, nearest-neighbor
selection, and the three closeness calipers. It is read only after pairs are
fixed, to rank which matches are most discordant.

The two source arms remain separate:

- `product_exact_summary`: exact product `(k,m)` bucket and exact equality of
  all 27 current combinatorial summaries, followed by agreement of mutual
  nearest neighbors under a compact ridge magnitude/shape representation and
  a full-ridge-family PCA representation;
- `generic_caliper`: exact generic facet bucket, followed by agreement of
  mutual nearest neighbors under two combined representations: full
  combinatorial summaries plus compact ridge shape, and PCA-reduced
  combinatorial plus PCA-reduced ridge summaries.

All representations are standardized inside the exact source bucket. The
generic arm uses approximate summary proximity and must not be pooled with the
product exact-summary arm.

For each arm, `q10`, `q25`, and `q50` independently require both neighbor-map
distances to be at or below that quantile among robust candidate pairs. These
rules were chosen after the initial scout and are fixed only for this bounded
inspection. They are analysis-chosen sensitivity variants, not predeclared
confirmatory evidence. `q25` is the convenience rule used for the inspection
panel; `summary.json` reports selected counts, counts with `sys` gap at least
`0.1`, and maximum gap for every arm and quantile so a weak or negative `q10`
result and caliper dependence remain visible. The panel retains the two largest
post-formation `sys` gaps per arm, with at most one pair per bucket.

## F4: Model-Consistent Residual Exemplars

Two ordinary models predict `sys` from all 45 active invariant features:

- standardized ridge regression with `alpha=10`;
- histogram gradient boosting with fixed complexity and regularization.

Each model is cross-fitted with two fixed seeds and five folds stratified by
the 18 exact source buckets. A row is called OOF-sign-consistent in this packet
only when all four model/seed residuals have the same sign. This is
model-specific consistency, not independent evidence for an omitted feature.
The compact candidate artifact retains sign-consistent rows with absolute
median residual at least `0.1`.

Each source bucket is then left out once and scored by both model families. This
is a weak extrapolation stress: an exemplar receives
`leave_bucket_out_sign_agrees` only when both leave-bucket-out residuals
preserve its out-of-fold sign. It is neither independent validation nor
omitted-feature evidence. Holding out a facet or product bucket is
extrapolation under the same generators, not evidence for transfer to a
different generator.

The inspection panel first retains the largest OOF-sign-consistent residual
whose sign survives this weak leave-bucket-out check in each source-arm/sign
cell, then fills to six residual exemplars by magnitude without repeating
buckets.

## Command

```bash
TABLES_DIR=/tmp/sys-ds-p2-normalization-full
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 \
  uv run --script experiments/sys-datascience/methods/residual-exemplar-seeds/analyze.py \
  --tables-dir "$TABLES_DIR"
```

The analyzer writes only under `--out-dir`, which defaults to `artifacts/`.

## Artifacts

- `artifacts/summary.json`: input identity, fixed bounded-analysis contracts,
  denominators, per-quantile arm results, sensitivity cutoffs, and evidence
  boundary;
- `artifacts/discordant-pairs.tsv`: every robust target-free pair, distances,
  post-formation target gap, and `q10`/`q25`/`q50` membership;
- `artifacts/residual-candidates.tsv`: model-specific OOF-sign-consistent
  residual candidates, exact per-model/seed residuals, and the weak
  leave-bucket-out check;
- `artifacts/inspection-panel.tsv`: four pair records and six exemplar records,
  preserving the `poly_id`s needed to join owning geometry and HK diagnostics;
- `artifacts/command.txt`: promoted-run command.

## Current Observation And Interpretation

The current generated summary records 324 robust product candidates and 187
robust generic candidates before the analysis-chosen distance variants. The
selected counts and post-formation target gaps change with the caliper. Use the
per-quantile results in `summary.json`, including the stricter `q10` result,
rather than treating the convenience `q25` panel as uniquely supported or
copying metric rows here.

The two specified residual models yield sign-consistent positive and negative
inspection candidates in both arms, and many preserve their sign under the
weak leave-bucket-out stress. This is enough to supply a small geometry/branch
inspection list. It does not demonstrate that the current scalar summaries
fail or that omitted geometry explains the residuals.

The packet does **not** show that paired polytopes are geometrically close, that
an omitted feature causes their target difference, or that residuals identify
multiple mechanisms. A large residual can reflect model bias, sparse support,
extrapolation, heteroskedasticity, or omitted geometry. Selecting large target
gaps and residuals is deliberately post-target and subject to winner's curse.

## Downstream Use And Stop Rule

The only promoted downstream use is a bounded inspection of the ten panel rows:

1. join their `poly_id`s to the owning exact geometry;
2. compare incidence-aware two-face-area organization and HK action/near-active
   branch diagnostics under a predeclared small panel contract;
3. state a discriminating feature or mechanism hypothesis, or record that the
   examples did not yield one.

Stop this line if that inspection produces no discriminating F1/M0/M1
hypothesis. Do not enlarge the panel, add a model zoo, or treat these retained
rows as generated candidates merely because more post-target anomalies can be
ranked.

## Geometry And Branch Inspection

Status: executed on the frozen panel. The ten panel records contain fourteen
distinct polytopes because each of the four pair records has two members. All
fourteen exact owner geometries reconstructed, all two-faces ordered, and the
HK diagnostic recomputed every stored `sys` value with zero reported delta.
See `artifacts/geometry-branch-inspection/summary.json`, the branch diagnostic
summary, and the generated `panel-comparison.tsv` for detailed rows.

The incidence contract treats each two-face as a node identified by its
unordered containing-facet pair, weighted by unsigned symplectic area divided
by `sqrt(volume)`. Two nodes are adjacent when the reconstructed faces share a
polytope edge. The compact inspection records adjacent-area variation and
correlation plus connectivity of the top area quartile. These quantities have
the same `Sp(4)`, translation, positive-scale, and relabeling boundary as the
owning area features, subject to f64 reconstruction. Their formulas and input
hashes are in the generated summary.

No inspected incidence statistic had one strict direction across all four
discordant pairs. The product `4x4` pair has a visually strong top-quartile
connectivity difference, but it is not repeated by the product `4x6` or generic
pairs. Therefore this compact F1 incidence family stops here: do not enlarge or
validate it from this packet alone. It can reopen if an independently motivated
mathematical definition predicts which product bucket should exhibit the
connectivity effect.

The M0 action anatomy does yield a discriminating hypothesis. The branch
diagnostic preserves raw solver-returned orbit words and counts separately from
cyclic classes. For every nonempty raw sigma word, its canonical representative
is the lexicographically smallest of all cyclic rotations; distinct canonical
representatives are the distinct cyclic classes. In both matched product pairs
the lower-`sys` member has four distinct cyclic classes tied at the minimum to
relative tolerance `1e-12`, while the higher-`sys` member has one near-active
cyclic class even at the `1e-2` window. The low product residual exemplar
likewise has four exact cyclic classes and the high product residual exemplar
has one, although those exemplars are not a matched pair. The four canonical
representatives differ by swaps of two facet pairs; they are not
cyclic-rotation duplicates. This supports a bounded product-family hypothesis:
fourfold order-choice ties mark a different low-`sys` action regime from the
unique-minimizer regime. It does not establish why the tie occurs or that
breaking it raises `sys`.

The generic evidence is weaker and window-dependent: one matched pair changes
from a single cyclic class to a second only at the `1e-3` window, while the
other is tied at one throughout; a high positive residual reaches five
near-active cyclic classes only at `1e-2`. Thus a generator-agnostic monotone
“more branches means lower `sys`” story is already prohibited. M0 should
continue only as a small untouched product test that distinguishes
algebraic/product symmetry ties from genuinely competing HK branches. M1 may
retain this product tie regime as one arm competing against ridge
magnitude/concentration and generator structure; the current packet is not
mechanism evidence.

The entire inspection is post-target `G`: `sys` and model residuals selected
the panel, and HK action spectra are target-cost diagnostics. None of these
features is a validated proposer, intervention, or causal explanation. The
action-window sweep is descriptive sensitivity analysis; absence of a second
returned word means none lies within the largest requested one-percent action
gap, not that no other admissible orbit exists.

Reproduce from a reviewed P2 reconstruction at
`/tmp/sys-ds-p2-normalization-full` using
`artifacts/geometry-branch-inspection/command.txt`. The generated branch-input
rows are a compact adapter to the existing real-data diagnostic; the canonical
producer files remain the geometry owners.

## Reopen Triggers

- the active invariant feature schema or trusted retained table changes;
- a new generator is available and a transfer question is explicitly selected;
- the geometry/branch inspection identifies a frozen discriminator worth
  testing;
- an input hash mismatch requires a deliberate packet refresh.
