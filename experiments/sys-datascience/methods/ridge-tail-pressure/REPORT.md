# Stratified ridge/sys tail empirics

## Question and inputs

This packet asks whether low
`ridge_symp_area_sum_over_volume_sqrt` remains associated with high `sys` when
the ten Lagrangian-product `(k,m)` strata are respected, and whether the two
available pre-`sys` ridge selections give evidence of a persistence, saturation,
or reversal as selection becomes more extreme.

The retained diagnostic is the tracked current prepared table
`../../prepare/polytope-table.jsonl` (14,336 rows; 10,240
`random_product_sample` rows).  Its product rows are joined one-to-one to the
tracked `produce/random-product.jsonl` using `sys`: both have 10,240 unique
serialized `sys` values, and every row matches.  This is a checked identity
recovery for stratification, not a claim that `sys` is generally a safe key.

The 1M run has 1,000,000 frozen feature rows, but only 466 evaluated target
rows.  The per-bucket low-sum selection and its matched baseline use 100 each
(ten per `(k,m)`).  The retained minimal snapshot checks every evaluated ID
against its selected-before-target row; `(k,m)` and ridge-sum value agree
exactly.  The original full feature table remains available only as an optional
identity audit, not a required reproduction input.
The independent 100k concentration-validation packet contributes its frozen
stage-1 low-ridge `q=.01` selected rows (100 per bucket).

Frozen selection-plan provenance establishes the pre-target semantics: the 1M
plan (sha256 `4bf777f56fcb07fe18163863594ca90e047f0e402ae7cc4b0e5ea179d6d6e68d`)
uses seed 271828, 100,000 candidates per bucket, per-bucket low-sum top-10
(`q=10^-4`), and one matched baseline replicate.  The 100k stage-1 plan
(sha256 `3f87a89b8d0e54a4580c20a1df04a5f2ddc9a786041b671bce57cbb417bb7c8a`)
uses seed 1618033, 10,000 candidates per bucket, a low-sum `.01` fraction with
`ceil_min_one` rounding, and one matched baseline replicate.  `metadata.json`
records their paths, hashes, and the 1M selected-before-target cache hash.

Run from this directory (without requiring a worktree-specific path):

```bash
python3 analyze.py --out-dir .
```

Current input/output hashes and the full command context are in `metadata.json`.
Important inputs include retained table sha256
`607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`,
1M feature table sha256
`9af141c74510ea5309b77ebc4abd0190b4c6a6f6efd9c5d2d67c3059c25af36d`
(optional full-scan identity audit), and 1M evaluated cache sha256
`c07825434c9b12e2774619dadaf5ba8876f02406e891c9c3713a15de6c2c4914`.

## Retained, post-target diagnostic

`retained_by_km_q.tsv` is the primary per-bucket result.  At each `q`, it takes
the bottom `ceil(q*1024)` ridge sums and top `ceil(q*1024)` `sys` values within
each `(k,m)` bucket.  `same_q_overlap_n` is their literal intersection;
`same_q_enrichment` divides it by the stratified independence expectation.  It
is distinct from the conditional mean/median/p90 columns, which compare the
`sys` distribution of the low-ridge subset with the other rows.  Wilson and
stratified nonparametric re-ranking bootstrap intervals quantify sampling
uncertainty, not a causal/proposer claim.

The stratified pooled same-q result strengthens rather than fades over the
available retained resolutions:

| q | overlap / low-ridge rows | enrichment | low-ridge mean sys | other mean sys |
|---:|---:|---:|---:|---:|
| .10 | 637 / 1,030 | 6.15x | .611 | .304 |
| .05 | 220 / 520 | 8.33x | .627 | .319 |
| .02 | 50 / 210 | 11.61x | .637 | .329 |
| .01 | 15 / 110 | 12.69x | .637 | .332 |

But the q=.01 resolution is plainly heterogeneous: overlap is 4, 6, and 4 in
`3x4`, `3x5`, and `3x6`, zero in `3x3`, `4x4`, `4x6`, `5x5`, `5x6`, and `6x6`,
and one in `4x5`.  The apparent pooled strengthening is therefore not evidence
for a common bucket-level mechanism; the exact bucket table must accompany any
use of the pooled line.

## Pre-target selection evidence and tail-pressure comparison

`one_m_selection_by_km.tsv` is not a same-q overlap analysis.  Its rows compare
the 1M run's pre-`sys`, per-bucket top-10 low-sum selection (`q=10^-4`) to its
matched random baseline.  All ten bucket mean differences are positive.  The
equal-bucket selected-minus-baseline mean is `.2477` (bucket-resampling 95% CI
`.1717, .3235`); selected mean `sys=.6084`, baseline `.3607`.  This supports
selection enrichment at that frozen rule, but it does not estimate the high-sys
tail prevalence in the 1M population.  This interval resamples the ten observed
bucket mean differences, so it describes the equal-bucket aggregate and its
between-bucket spread in this run; it is not a target-row sampling interval.

`independent_100k_vs_1m_by_km.tsv` compares independent pre-target runs: the
100k concentration packet's stage-1 q=.01 selected mean with the 1M q=.0001
top-10 selected mean.  Four small-side buckets rise at the more extreme run
(`3x3`, `3x4`, `3x5`, `3x6`); all six buckets with `k>=4` fall.  Five hundred
within-bucket resamples of the evaluated sys values place the sign clearly
above zero for `3x3`, `3x4`, `3x6`, and clearly below zero for every `k>=4`
bucket; `3x5` remains compatible with zero.  The equal-bucket contrast is
`-.0277`, with bucket-resampling 95% interval `[-.0694, .0204]`.

This is a run-specific, hypothesis-generating sign pattern compatible with
bucket heterogeneity; it is not evidence against a universal monotone
selection-pressure curve, and it establishes neither reversal nor saturation.
The q values differ by 100-fold, and the runs use different seeds, populations,
and selected order statistics.  The within-bucket intervals resample target
values only within the already selected, different runs; the equal-bucket
interval resamples the ten observed bucket contrasts and reflects their
between-bucket spread.  A common-q, multi-seed design (or repeated runs at both
q values) is needed to discriminate a selection-pressure curve.

## Boundaries

- Retained same-q rows are post-`sys` descriptive associations, not validation
  of a candidate proposer.
- The evaluated 1M cache is selection-plus-baseline data, not a random sys
  sample.  Do not derive population tail probabilities from it.
- No artifact here observes `sys > 1`; this packet makes no extrapolation or
  probability estimate for `sys > 1`.
- The retained same-q bootstrap re-ranks nonparametric within-stratum resamples
  and is a descriptive uncertainty calculation for this retained table.  The
  1M selected-minus-baseline and pooled cross-run intervals instead resample
  observed bucket effects/contrasts; they are not target-row sampling intervals.
  Only the cross-run per-bucket intervals resample target values, and then only
  conditional on the already selected rows of their different runs.  None cover
  generator, feature, target-computation, or changed-q/run uncertainty.
