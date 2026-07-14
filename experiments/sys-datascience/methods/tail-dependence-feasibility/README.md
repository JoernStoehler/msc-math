# Ridge/sys tail-dependence feasibility

Status: first-gate feasibility and conjecture packet. It uses existing artifacts
only and is ready to inform a new-data decision, not thesis prose or an
asymptotic claim.

## Question and decision

For the frozen within-product-bucket proxy

`R = ridge_symp_area_sum_over_volume_sqrt`,

which tail statements about high `sys` are identifiable now? Does harder low-`R`
conditioning favor persistent dependence, coarse enrichment followed by
asymptotic independence, saturation/reversal, or bucket mixture? The decision is
whether a fresh nested-band target panel is worth producing and, if so, what it
must distinguish.

This packet does not replace the frozen generated-candidate proposer. It reads
that packet and the retained random-product table without producing any new
geometry or target evaluations.

## Source and censoring inventory

The common generator boundary is random Lagrangian products from polygon
height representations on `[0.8,1.2]`, stratified by the ten buckets
`3x3,...,6x6`.

- The retained seed-42 table has all proxy and target values for 1,024 rows per
  bucket. It identifies empirical same-quantile overlap down to about 1%; below
  that, each bucket has too few tail rows for more than singleton diagnostics.
- The independently frozen seed-1618033 100k cascade has 10,000 generated rows
  per bucket and target values for every row in the lowest 1% by `R`. Therefore
  exploratory disjoint bands `0-.1%`, `.1-.2%`, `.2-.5%`, and `.5-1%` have
  respectively 10, 10, 30, and 50 target rows per bucket. Targets outside the
  lowest 1% are available only through matched baselines; full-population target
  ranks are censored.
- The seed-271828 100k promising-scalars packet gives 10 target rows per bucket
  at the lowest `.1%` by `R`.
- The seed-271828 1M packet gives 10 target rows per bucket at the lowest
  `.01%`. Its 100k population is a prefix of the 1M population, so those two
  panels are dependent rather than independent replications. The tracked 1M
  packet retains aggregate selected-target rows, not all row-level targets.
- Matched baselines estimate coarse selected-versus-random differences. They
  are far too small to estimate the `.1%` or `.01%` target quantiles, so they do
  not recover same-quantile tail dependence after target censoring.

The 100k cascade subdivision was chosen after seeing the target values, but its
entire lowest-1% set was frozen before target evaluation. It is honest
exploratory analysis of a pre-target-selected population, not a new frozen
validation.

## Existing-data observations

The exact repeated comparisons are generated in `artifacts/current/`.

1. In the full retained table, the empirical coefficient
   `P(sys in bucket top q | R in bucket bottom q)` falls from `0.762` near
   `q=20%`, through `0.618` at `10%` and `0.423` at `5%`, to `0.136` at an
   effective `q=1.074%`. The corresponding independence enrichment is still
   `12.7x` at the smallest panel, so coarse enrichment is real while the
   tail-dependence coefficient is decreasing.
2. The retained 1% overlap is heterogeneous: only `3x4`, `3x5`, `3x6`, and
   `4x5` have any same-quantile intersection; the first three contain 14 of the
   15 pooled intersections. Pooled asymptotics would erase this structure.
3. Within the independently frozen lowest 1%, the pooled within-bucket rank
   correlation between `R` and `sys` is `0.032`, rather than the negative sign
   expected from continued low-`R` improvement. The more extreme half has a
   higher mean `sys` in 4 of 10 buckets; the most extreme tenth improves over
   the remaining 90% in only 2 of 10.
4. Cumulative mean `sys` in that frozen panel is `0.636` at 1%, `0.633` at
   0.5%, `0.632` at 0.2%, and `0.625` at 0.1%. The independent seed-271828 0.1%
   panel has mean `0.626`. The 0.01% panel has mean `0.608`; under the dependent
   same-seed comparison, hardening from 0.1% to 0.01% improves only 3 of 10
   buckets.
5. The bucket pattern is structured rather than random-looking. The `3x4` and
   `3x6` means improve under same-seed hardening, `3x5` is flat, `3x3` is an
   almost constant-`sys` control, and all six `k>=4` buckets decline except for
   a nearly flat `4x5` comparison. This is evidence for a bucket interaction,
   not yet an independently validated law.
6. No selected row in any panel has `sys > 1`. Zero hits among 1,000 rows in
   the lowest 1% give only the one-sided binomial bound
   `p(sys>1 | R-tail) < 0.00299` at 95% under iid fixed-band assumptions. The
   corresponding bounds are `0.0149` for the two 0.1% panels combined and
   `0.0295` for the 0.01% panel. These are upper bounds, not hit-rate estimates.

These observations favor coarse enrichment followed by saturation together
with bucket mixture. They weaken a pooled persistent-dependence story over the
observed range, but sparse extreme panels and target censoring prevent an
asymptotic conclusion.

## Competing conjectures and discriminating observations

### Persistent within-bucket tail dependence

For each nondegenerate bucket, lowering the `R` quantile continues to raise
fixed high-`sys` exceedance probabilities and upper conditional quantiles.
Independent nested bands should show positive contrasts from `.1%` to `.01%`
in most buckets, not merely higher means than an unfiltered baseline.

Current status: weakened in pooled form by the flat lowest-1% trend and by only
2/10 positive most-extreme-tenth contrasts. A restricted `3x4`/`3x6` version
remains plausible.

### Asymptotic independence after useful coarse enrichment

Low `R` moves candidates into a broad high-`sys` region, but conditional
high-target exceedance and within-band rank information plateau as the proxy
quantile shrinks. Same-quantile overlap tends toward zero even while enrichment
over an unconditional draw remains large.

Predicted new observation: nested bands remain better than baseline, but the
`.01%` band is statistically and materially indistinguishable from adjacent
`.01-.1%` and `.1-.5%` bands after bucket stratification.

### Saturation or reversal

Below a bucket-specific ridge scale, harder proxy selection ceases helping or
reduces `sys`. Predicted new observation: the `.01%` band has lower fixed-tail
exceedance and mean/upper quantiles than `.01-.1%` in an independent seed, with
a predeclared material contrast rather than a post-hoc maximum comparison.

Current status: plausible, especially for `k>=4`, but the only 0.01% panel has
10 rows per bucket and is dependent on the same-seed 0.1% population.

### Bucket mixture

There is no useful pooled limit without conditioning on product type. A sharper
version suggested by current data is persistent or weakly improving behavior
for some `3xm` buckets and saturation/reversal for `k>=4`, with `3x3` as a
structural near-constant control.

Predicted new observation: an independent nested-band panel reproduces an
interaction in which the sign of hardening contrasts differs between the
predeclared `k=3,m>3` and `k>=4` groups. Failure of that sign split would reject
the sharpened mixture conjecture even if pooled enrichment persists.

## Compute-to-hit boundary

For proxy quantile `q` and assumed conditional threshold-hit probability `p`,
sample-and-filter needs

`ceil(log(0.5) / log(1-p))`

selected target evaluations for a 50% hit chance, and that number divided by
`q` raw candidates. The generated sensitivity table keeps `q` and `p`
independent because current zero-hit data do not estimate `p`. For example,
`q=10^-6, p=10^-4` requires about `6.93e9` raw candidates; `q=10^-6, p=10^-6`
requires about `6.93e11`. These are conditional scenarios, not predictions.

The 100k source run records 12-worker wall times of 9.026 seconds for geometry,
features, and selection, plus 65.818 seconds for 2,490 target evaluations. It
does not record hardware identity or CPU utilization, so conversion to
core-hours would be fabricated. Storage and I/O also need not scale linearly.

Direct proxy optimization is a different experiment. To compare it with
sample-and-filter optimization strength, a future optimizer must report the
attained within-bucket empirical `R` quantile on an independent ridge marginal
per step and its wall/resource cost. Current artifacts contain neither an
optimizer trajectory nor such a calibration.

## Cheapest discriminating new-data design

Do not launch this before the first Jörn gate.

If the bucket-sensitive saturation question is worth resolving, generate an
independent 500,000-row feature population per bucket (5M total), freeze `R`,
and evaluate 50 rows per bucket in each disjoint band

`0-.01%, .01-.1%, .1-.5%, .5-1%, 1-5%, 5-20%, 20-100%`.

This is 3,500 target evaluations. Fix retained-table bucket-specific `sys`
thresholds before the run, and use both threshold exceedance and the conditional
`sys` distribution; do not try to estimate a new `.01%` target quantile from 50
rows. Predeclare the `k=3,m>3` versus `k>=4` interaction, retain `3x3` as a
control, and escalate any `sys>1` row immediately.

At the observed 100k throughput, naive linear scaling is roughly 451 seconds
of 12-worker pre-target wall time and 93 seconds of target wall time, before
overhead. The 1M cache occupies about 6.4 GB, so the current cache architecture
would imply roughly 32 GB at 5M. These are feasibility anchors, not a resource
promise. A streaming selector that retains only selected geometry may reduce
storage but requires implementation and review.

Stop after 50 rows per band. Expand toward 100 only if uncertainty around the
predeclared interaction or a material `sys` contrast of about 0.04 could still
change the line decision. A pre-target manifest review must check exact
within-bucket ranks, disjoint bands, seed independence, target-field absence,
and that no same-seed prefix is mislabeled as replication.

## Command and artifacts

Hydrate the six LFS inputs named by the script, then run:

```bash
python3 experiments/sys-datascience/methods/tail-dependence-feasibility/analyze.py
```

Generated owner files:

- `summary.json`: source hashes, identifiability boundary, key counts, zero-hit
  bounds, and observed runtime calibration;
- `retained-tail-overlap.tsv`: same-quantile overlap by bucket and pooled;
- `nested-band-summary.tsv`: exact disjoint-band target summaries;
- `cross-scale-summary.tsv`: cumulative panels across sources and quantiles;
- `cross-scale-by-bucket.tsv`: independent 0.1% replication and dependent
  same-seed 0.1%-to-0.01% comparison;
- `frozen-100k-bucket-trends.tsv`: within-lowest-1% rank and contrast checks;
- `hit-rate-sensitivity.tsv`: assumption-only sample-and-filter arithmetic.

Allowed use: choose or design further research for this generator and state the
four conjectures above. Prohibited use: claim an asymptotic limit, calibrated
`sys>1` hit rate, arbitrary-generator transfer, geometric mechanism, or
core-hour forecast.
