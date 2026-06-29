# Sys Datascience Research Ledger

Purpose: this ledger is a working research-control surface for the
`experiments/sys-datascience` random-polytopes slice. It is used to preserve the
current research object, downstream use, observations, candidate patterns,
interpretation limits, and next-batch decision criteria across chat turns and
agent resumes.

It is not a polished report and not a place to launder exploratory observations
into conclusions. Entries should distinguish:
- what was being explored;
- why it matters for thesis success, `produce/` design, method interpretation,
  or later evidence;
- what was actually observed;
- what interpretations are allowed or not allowed;
- what uncertainty controls the next batch.

Schema note: entries before the invariant-only table migration may mention
raw Euclidean, omega-matrix, transition, `capacity`, or `volume` method
features. Those names are historical observations under older prepared-table
schemas, not active method-facing covariates. The active table contract is in
`prepare/README.md` and `prepare/invariant_features.rs`.

## 2026-06-25 - Post-milestone-0 next-batch brainstorm

Worktree: `/workspaces/msc-math/.worktrees/random-datascience-slice`

Context: milestone 0 cleaned `experiments/sys-datascience` to a random/product-only active surface. The next work should learn something about the random-polytopes thesis slice, not polish infrastructure.

### Candidate batches

1. Randomness-sensitivity baseline

Question: when varying the random distribution, do the high-`sys` tail, feature associations, and interesting-looking examples change?

Assessment: highest value. This directly tests whether the methods care about the randomness model, and it creates a comparison harness for later batches.

2. Top-tail anatomy

Question: what are the current highest-`sys` polytopes actually like?

Assessment: high value as a small companion to the sensitivity baseline. It may suggest features or math hypotheses that aggregate correlations hide.

3. Feature audit and feature invention

Question: are the current features the right vocabulary for this random-polytopes slice?

Assessment: useful, but less valuable before we know which distribution comparisons matter. Best after a first sensitivity/top-tail pass unless Jörn has a strong feature intuition.

4. Negative-result robustness for `sys > 1`

Question: how informative is the current "0 found" result by parameter bucket?

Assessment: valuable, especially for thesis claims and compute allocation. Should be near-term, but probably after the first distribution comparison unless the immediate goal is hostile search.

5. Method usefulness ranking

Question: which methods actually guide research decisions?

Assessment: useful anti-sprawl work, but less discovery-oriented. Best done after one more real data batch.

### Priority

Recommended next batch:

1. Randomness-sensitivity baseline.
2. Include a small top-tail anatomy component.
3. Then use the results to choose between negative-result robustness, feature invention, or hostile LICCA sampling.

Reasoning: this matches the declared slice most directly. The current research question is not just whether sampled `sys(a)` exceeds 1, but whether the observed behavior is stable under different random generators. A sensitivity baseline turns random distribution choice from an implementation detail into an interpretable experimental axis.

### Jörn feedback

Jörn agrees that looking at distribution sensitivity early is high-leverage.

Reason: if the observed `sys(a)` landscape is sensitive to the random generator,
then `produce/` design and later method interpretation must treat the generator
as an experimental variable. Data gathering should cover distributions
deliberately, method summaries should say which random model produced the data,
and high-`sys` observations may be conditional on the generator.

If the observed behavior is insensitive across the compared generators, later
experiment design is more robust. We can stop spending much attention on whether
the random distribution itself caused the observed result, except through shared
properties of all compared distributions.

Next batch decision: run a distribution-sensitivity baseline first, with a small
top-tail anatomy component so the result is not only aggregate tables.

## 2026-06-25 - Distribution-sensitivity baseline, local first pass

Worktree: `/workspaces/msc-math/.worktrees/random-datascience-slice`

Scratch output root:
`/tmp/sys-ds-distribution-sensitivity-20260625`

Method artifact:
`/tmp/sys-ds-distribution-sensitivity-20260625/artifacts/summary.json`

Code changes made for the batch:
- exposed `--h-min` and `--h-max` on both active producers,
  `sys-dataset-random` and `sys-dataset-random-product`;
- added `seed` and `attempt` to the remaining producer JSONL row schemas;
- fixed the compatibility prepare loader to carry `sample_seed`,
  `sample_attempt`, `seed_index`, and `lineage_id` into provenance rows;
- added `methods/distribution-sensitivity/analyze.py`, which compares prepared
  table directories by `sys` tail summaries, pairwise distribution tests,
  top rows, top geometry-only scalar associations, and matched-source deltas
  when provenance permits them.

Local run design:
- four variants;
- each variant has 64 generic rows (`F=5..8`, 16 accepted rows per `F`) and 48
  product rows (`3x3`, `3x4`, `4x4`, 16 accepted rows per bucket);
- variants:
  - `default_seed42`: height interval `[0.8,1.2]`;
  - `default_seed314159`: independent default-interval sample,
    height interval `[0.8,1.2]`;
  - `narrow_seed42`: height interval `[0.95,1.05]`;
  - `wide_seed42`: height interval `[0.5,2.0]`.
  The seed labels are reproducibility metadata, not research parameters.

Results:
- no `sys > 1` rows in any variant;
- overall Kruskal test across the four `sys` samples: `p = 0.672028`;
- pairwise KS/Mann-Whitney tests did not show a clear aggregate distribution
  shift at this sample size;
- max `sys` by variant:
  - `default_seed314159`: `0.725316077763`;
  - `default_seed42`: `0.592718700008`;
  - `narrow_seed42`: `0.606588297243`;
  - `wide_seed42`: `0.544718434534`;
- these maxima are descriptive order statistics only; do not use them as primary
  evidence for or against height-interval sensitivity;
- top rows are mostly product rows, especially `4x4` and `3x4`; the top generic
  row in this run had `sys = 0.6011427102454201`;
- the strongest geometry-only feature associations were stable across all four
  variants: prepared ridge symplectic-area summaries dominated
  (`ridge_symp_area_sum/max/mean/q90/q95/std`), all negatively
  associated with `sys`.

Matched-source diagnostic:
- where provenance permits matching source names across height intervals,
  aggregate distributions are fairly stable under height interval changes;
- individual matched-source examples are not uniformly stable:
  - `default_seed42 -> narrow_seed42`:
    - generic median absolute paired delta `0.000986`, max absolute delta
      `0.029187`;
    - product median absolute paired delta `0.003905`, max absolute delta
      `0.550621`, driven by `4x4`;
  - `default_seed42 -> wide_seed42`:
    - generic median absolute paired delta `0.003805`, max absolute delta
      `0.312423`, driven by `F8`;
    - product median absolute paired delta `0.040266`, max absolute delta
      `0.552542`, driven by `3x4`/`4x4`;
- `F5` generic rows and `3x3` product rows were invariant up to numerical noise
  in the matched-source comparisons. This may reflect low-complexity/simplex
  rigidity or a normalization effect; it needs mathematical interpretation
  before being treated as a fact beyond this run.

Interpretation status:
- exploratory/pilot only;
- valid uses: pipeline feasibility, provenance/schema needs, rough runtime,
  candidate metrics, and candidate patterns for follow-up;
- invalid uses: proving height-interval sensitivity, proving height-interval
  insensitivity, or treating non-significant p-values as equivalence evidence;
- the stable feature-association result is a candidate pattern, not yet a
  settled claim.

Question for Jörn:
- The invariance of generic `F5` and product `3x3` under paired height changes
  looks mathematically meaningful. Should I treat that as expected/trivial
  low-complexity behavior, or is it worth a short targeted explanation check?

## 2026-06-25 - Revised next milestone after interpretation reset

Status: current control point for the next batch.

The previous height-interval pilot is not a decision-grade sensitivity result.
It should be treated as exploratory work that mainly established pipeline
feasibility, method feasibility, provenance requirements, rough runtime, and a
few candidate patterns.

Jörn does not know whether height-interval variation is mathematically
meaningful. Therefore the next batch should not ask Jörn to classify it in
advance and should not assume that height interval is already the right
distribution axis.

### Milestone

Explore whether height-interval variation is a useful random-distribution axis
at all.

### Questions

1. Empirical materiality: does changing `[h_min,h_max]` change `sys`, engineered
   features, or high-tail behavior enough to matter for `produce/` design?
2. Interpretability: if it changes something, can we describe what geometric
   property changed, or is it only an arbitrary generator knob?
3. Axis comparison: how does height interval rank against other candidate
   random-distribution axes, such as direction distribution, facet count/product
   side counts, generic versus product generation, heavier-tailed or more
   concentrated heights, and rejection/admissibility effects?

### Process for the next batch

- Keep the batch small and diagnostic, not LICCA-scale evidence.
- Analyze what the existing pilot changed in feature space, not only in `sys`.
- Compare height interval conceptually against other randomness axes before
  spending larger compute.
- Run only cheap additional smokes if they directly distinguish whether height
  interval is worth more attention.
- End with a ranked list of random-distribution axes worth testing next and a
  clear recommendation for whether height interval should be demoted, retained
  as a sanity-check axis, or promoted to a serious `produce/` design variable.

### Reporting rule

Separate observations, candidate patterns, interpretation, and proposed next
tests. Do not present post hoc exploratory patterns as confirmed sensitivity
claims.

## 2026-06-25 - Milestone result: height interval as a random axis

Status: milestone completed for discussion.

Question: is height-interval variation useful enough to guide near-term
random-polytopes `produce/` design?

Input artifacts:
- previous pilot tables under `/tmp/sys-ds-distribution-sensitivity-20260625`;
- distribution-sensitivity summary at
  `/tmp/sys-ds-distribution-sensitivity-20260625/artifacts/summary.json`;
- random-axis diagnostic summary at
  `/tmp/sys-ds-axis-diagnostic-20260625/artifacts/summary.json`.

New method:
- `methods/random-axis-diagnostic/analyze.py`

### Observations

1. Height interval is an interpretable generator knob: for generic random rows,
   sampled dual vertices have `a_k = n_k / h_k`, so widening
   `[h_min,h_max]` increases the spread of facet-dual norms. For product rows,
   the same height interval controls the two factor-polygon support heights
   before taking the Lagrangian product.

2. Seed is not a random-distribution axis. It is only reproducibility metadata
   for independent rows. Uncertainty should come from statistics on the sampled
   rows, such as effect sizes, distribution summaries, and bootstrap intervals;
   not from reporting "seed effects".

3. In the pilot rows, height interval has very small effect on `sys` compared
   with existing bucket structure:
   - height interval on `sys`: `eta^2 = 0.0010478705465261776`,
     Kruskal `p = 0.7934708069312687`;
   - dataset family on `sys`: `eta^2 = 0.021619150220716317`,
     Kruskal `p = 0.007066722716097604`;
   - bucket on `sys`: `eta^2 = 0.18755593164545548`,
     Kruskal `p = 2.428085423081998e-15`.

4. Bucket-level height-interval effects on `sys` are also small in this pilot:
   - generic `F5`, product `3x3`: essentially zero;
   - generic `F6/F7/F8`: `eta^2 <= 0.002721`;
   - product `3x4`: `eta^2 = 0.004977`;
   - product `4x4`: `eta^2 = 0.054323`, the largest bucket-level effect but
     still not strong at this sample size (`p = 0.22744`).

5. Height interval does move feature space, but mainly through geometric
   dispersion features rather than through the features most associated with
   `sys`.
   - Across 121 geometry features, median interval `eta^2` is
     `0.0010107798485778205`; q90 is `0.015674433963631034`; max is
     `0.46778361798100765`.
   - The top interval-sensitive feature is `geom_norm_std`; mean values
     in the pilot rows:
     - narrow `[0.95,1.05]`: `0.098825`;
     - default `[0.8,1.2]`: `0.400342`;
     - wide `[0.5,2.0]`: `1.480616`.
   - Other interval-sensitive features include `geom_norm_max`,
     `ridge_abs_omega_top3_share`,
     `allpair_abs_omega_top3_share`, and
     `geom_pairwise_dist_std`.

6. The dominant `sys`-associated feature family from the earlier method,
   `ridge_symp_area_*`, barely moves under height interval in this
   pilot. For example, mean `ridge_symp_area_sum` in the matched pilot
   rows:
   - narrow: `35.101117`;
   - default: `35.107272`;
   - wide: `35.480379`.

7. Data size in this diagnostic is small by bucket/interval: 16 rows per
   non-default cell and 32 rows in default cells because the default interval
   has two independent batches. Bootstrap intervals for q90 are consequently
   wide. More rows would help estimate tail behavior and high-bucket effects;
   they are less needed for the already-obvious fact that height interval moves
   norm-dispersion features.

8. Matched-source high-tail identity can still change in higher-complexity
   buckets. This is a diagnostic based on provenance matching, not a seed-effect
   analysis:
   - top-5 overlap is perfect for generic `F5/F6/F7` and product `3x3`;
   - wide vs default/narrow has low top-5 overlap for generic `F8` and product
     `3x4`/`4x4`.

### Interpretation

Height interval is interpretable as a facet-distance-spread axis, but in this
pilot it is not a strong `sys` axis. It changes some shape/dispersion features
substantially, yet those moved features are not the stable strongest
`sys`-association features. Existing axes such as bucket/facet-count/product
structure explain much more `sys` variation.

For near-term `produce/` design, height interval should be demoted: keep it as a
sanity-check or occasional robustness axis, not as a primary data-budget axis.
If revisiting height interval later, use larger independent samples per
`(height interval, bucket)` and bootstrap/effect-size summaries; do not design
the analysis around PRNG seed batches.

### Axis ranking for near-term work

1. Bucket structure: generic `F` and product side-count buckets. Highest current
   value because it explains much more `sys` variation and controls where
   high-tail examples appear.
2. Product versus generic generator. Useful because product rows dominate many
   high-tail examples and family has visible `sys` effect.
3. Features tied to ridge symplectic-area summaries. Not a random generator
   axis, but currently the strongest target for interpretation and feature work.
4. Height interval. Useful for robustness checks and shape-dispersion probes,
   but not currently a primary `produce/` axis.
5. Untested distribution axes: direction distribution, heavy-tailed/concentrated
   heights, and rejection/admissibility-conditioned effects. These may be more
   meaningful than simple interval widening, but require a separate design.

### Predictions / checks for future work

- If height interval remains secondary, larger independent samples should show
  interval effects on `sys` smaller than bucket and family effects, even when
  norm dispersion features move strongly.
- If height interval matters for hostile search, the signal should appear first
  in higher-complexity buckets (`F8+`, product `3x4`/`4x4+`) through top-k
  instability rather than through global mean/median shifts.
- If ridge symplectic-area features are genuinely robust, they should remain
  among the top `sys` association features across new random axes, not only
  across height intervals.

### Recommended next batch

Do not run a larger height-interval sensitivity experiment yet. Next, work on
either:
- top-tail anatomy and bucket-focused hostile sampling, especially product
  buckets and larger `F`; or
- feature interpretation around `ridge_symp_area_*`, because this is
  the most stable signal seen so far.

## 2026-06-25 - Marginal `sys` broad distribution scan

Current exploratory distribution-screen method:

- `methods/sys-distribution-broad-scan/analyze.py`

Reason: the earlier small hand-picked scans were deleted because they were not
a useful active method surface. The active scan fits many SciPy continuous
families to `logit(sys)` and transforms them back to `(0,1)`, recording fit
failures instead of silently shrinking the family set.

Data source for the current local large random-only table:

- `/workspaces/msc-math/experiments/sys-datascience/prepare`
- `random_sample`: 4096 rows;
- `random_product_sample`: 10240 rows;
- total random-only rows: 14336.

Command:

```bash
uv run --script experiments/sys-datascience/methods/sys-distribution-broad-scan/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-shape-20260625/broad-scan \
  --max-fit-seconds 2
```

Screening result:

- 80 logit-transformed SciPy candidate families;
- 18 fixed buckets;
- 75-78 successful fits per bucket after timeouts/failures;
- screening tables:
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-screening-summary.md`;
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-screening-summary.tsv`;
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-bucket-screening-matrix.tsv`;
- ECDF overview plots:
  - `/tmp/sys-ds-shape-20260625/broad-scan/overview/generic-broad-scan-ecdf.png`;
  - `/tmp/sys-ds-shape-20260625/broad-scan/overview/product-broad-scan-ecdf.png`.

Top held-out family counts:

- `logit_jf_skew_t`: 5 buckets;
- `logit_powerlognorm`: 3 buckets;
- `logit_skewnorm`: 3 buckets;
- `logit_pearson3`: 2 buckets;
- `logit_burr12`, `logit_exponweib`, `logit_johnsonsu`,
  `logit_norminvgauss`, `logit_weibull_max`: 1 bucket each.

Important interpretation:

The broad scan is not a confirmatory goodness-of-fit or family-rule-out
procedure. It is a proposal/screening surface that shows many smooth
logit-transformed SciPy families can approximate the fixed-bucket ECDFs. The
useful claim from this artifact is not "the marginal law is family X"; it is
that fixed-bucket `sys` marginals can be summarized by smooth bounded CDF
approximations whose location/shape shift with bucket.

Examples of tiny held-out gaps:

- `product:3x3`: `logit_weibull_max` beats `logit_genextreme` by
  `2.013e-06` per row;
- `product:3x5`: `logit_pearson3` beats `logit_beta` by `3.807e-05`;
- `product:6x6`: `logit_jf_skew_t` beats `logit_burr12` by `2.842e-05`.

Next methodological step:

- stop treating family identity as the main object unless a later cross-run
  stability check shows it is stable;
- compare CDF/quantile trajectories across buckets directly;
- use broad-scan fitted CDFs as smooth empirical summaries, not as a literal
  distribution-family discovery;
- only add LICCA 10x data if the next question is tail precision or whether
  these CDF/quantile trajectories remain stable.

Screening table convention:

- `technical_fail`: SciPy fitting/evaluation failed or timed out;
- `cdf_flag`: max absolute ECDF/CDF error exceeds the bucket's 95% DKW band;
- `weak`: within the DKW band but above 75% of that band;
- `ok`: at most 75% of the DKW band.

Current broad result: 34 of 80 families are flagged by the CDF screen in all 18
buckets. Five families have zero CDF flags and zero technical failures:
`logit_gumbel_l`, `logit_loggamma`, `logit_jf_skew_t`, `logit_johnsonsu`,
`logit_norminvgauss`. They still have 2-4 weak cells each, as recorded in the
screening table. Use the table, not memory, for exact bucket-level status.

## 2026-06-25 - Marginal `sys` MLE likelihood table

Current likelihood-comparison method:

- `methods/sys-distribution-mle-likelihood-table/analyze.py`

Question: for each fixed bucket and candidate family, what are the
maximum-likelihood parameters and same-data likelihoods?

Interpretation: within one bucket, every family is fit and scored on the same
rows. Therefore log-likelihood differences are directly comparable. The
reported likelihood ratio is an upper-bound update between two named family
hypotheses under the artificial best case where the parameter prior mass is
concentrated at each family's MLE. This is not an integrated Bayes factor and
does not include a parameter-complexity penalty. For the current exploratory
question, that omission is deliberate: the target is whether likelihood ratios
are large enough that modest parameter-code penalties would not drive the
decision.

Command:

```bash
uv run --script experiments/sys-datascience/methods/sys-distribution-mle-likelihood-table/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-shape-20260625/mle-likelihood-table \
  --max-fit-seconds 3
```

Outputs:

- `/tmp/sys-ds-shape-20260625/mle-likelihood-table/mle-likelihood-table.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-likelihood-table/bucket-best-summary.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-likelihood-table/family-summary.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-likelihood-table/family-bucket-log2-gap-matrix.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-likelihood-table/summary.json`.

Run result:

- 14,336 rows, 18 fixed buckets, 80 candidate families;
- 1,404 successful bucket/family fits and 36 non-ok fits;
- non-ok statuses are optimizer/timeouts/technical failures, not statistical
  rejections;
- best family by bucket:
  - `logit_norminvgauss`: 7 buckets;
  - `logit_jf_skew_t`: 4 buckets;
  - `logit_johnsonsb`, `logit_gompertz`, `logit_skewnorm`: 2 buckets each;
  - `logit_johnsonsu`: 1 bucket.

Family-level likelihood-gap observations:

- No single candidate family wins all buckets.
- `logit_norminvgauss` is the strongest all-bucket compromise in this run:
  it wins 7 buckets, succeeds on all 18 buckets, has median gap
  `-0.2394` log2 units versus the bucket winner, and worst gap `-9.9409`.
- The only families that succeed on all 18 buckets and are never worse than
  20 log2 units from the bucket winner are:
  - `logit_norminvgauss`: worst `-9.9409`, median `-0.2394`, 7 wins;
  - `logit_johnsonsu`: worst `-12.8105`, median `-0.3789`, 1 win;
  - `logit_loggamma`: worst `-16.1580`, median `-1.4802`, 0 wins.
- `logit_beta` is not a stable explanation under this MLE scan: it succeeds on
  14 of 18 buckets, wins none, and is down by as much as `-39.1980` log2 units
  where it succeeds.
- Simple symmetric location-scale shapes are strongly dominated in product
  buckets. For example, `logit_norm` succeeds everywhere but has median gap
  `-148.3690` log2 units and worst gap `-241.3321`; `logit_logistic` succeeds
  everywhere but has median gap `-114.0690` and worst gap `-158.1131`.

Current interpretation:

- The all-data MLE evidence does not support one universal named family as the
  literal fixed-bucket law.
- It does support a small set of flexible logit-domain families as compact
  parametric summaries, with `logit_norminvgauss` currently the best
  cross-bucket compromise by MLE likelihood.
- Likelihood gaps are large enough to make some simple candidates unattractive
  even before a formal parameter-code penalty; this is strongest for
  `logit_norm` and `logit_logistic`.
- This result is still all-data exploratory evidence. It is the correct table
  for Jörn's upper-bound likelihood-ratio question, not a posterior model
  probability table.

## 2026-06-25 - Marginal `sys` transformed MLE likelihood table

Reason for rerun: the logit-only MLE table does not test whether a simple
family appears after another monotone transform of `sys`, such as `sys^2` or
`log(1-sys)`.

Method: for each transform `T` and SciPy continuous distribution family, fit
the family to `T(sys)` and score the fitted model as a density on the original
`sys` scale:

```text
log p_sys(s) = log p_Y(T(s)) + log |T'(s)|
```

This makes all transform/family pairs directly likelihood-comparable within a
bucket.

Transforms tested:

- `identity`;
- `logit`;
- `log`;
- `log1m`;
- `neglog1m`;
- `sqrt`;
- `square`;
- `cloglog`.

Command:

```bash
uv run --script experiments/sys-datascience/methods/sys-distribution-mle-likelihood-table/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-shape-20260625/mle-transform-table \
  --max-fit-seconds 3
```

Outputs:

- `/tmp/sys-ds-shape-20260625/mle-transform-table/mle-likelihood-table.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/bucket-best-summary.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/model-summary.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/model-bucket-log2-gap-matrix.tsv`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/bucket-best-readable-summary.md`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/model-readable-summary.md`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/model-bucket-readable-gap-matrix.md`;
- `/tmp/sys-ds-shape-20260625/mle-transform-table/summary.json`.

Run result:

- 14,336 rows, 18 buckets, 8 transforms, 80 distributions, 640 candidate
  transform/family models;
- bucket winners include `square+mielke`, `square+gamma`,
  `square+exponpow`, `square+beta`, `square+gengamma`, `log1m+beta`,
  `square+chi`, `square+exponweib`, `square+burr`,
  `cloglog+norminvgauss`, `neglog1m+gompertz`, and `log+jf_skew_t`.

Main observation:

- Allowing transforms matters. The logit-only winner is often beaten by
  transformed models; for example `logit+beta` is down by more than 20 bits in
  many buckets after the transform axis is allowed.
- No single transform/family model is plausible as a universal fixed-bucket law
  under this scan. In `model-summary.tsv`, no model succeeds on all 18 buckets
  while staying within 20 log2 units of the bucket winner everywhere.
- The models with the most bucket wins are specialized, not robust:
  `square+mielke` wins 4 buckets but has 13 `no`/non-ok buckets in the readable
  summary; `square+chi` wins 3 buckets but has 9 `no`/non-ok buckets.

Current interpretation:

- The earlier logit-only table was incomplete for the question "does a simple
  family appear after a different transform?"
- The transformed scan does not reveal one simple global parametric law either.
  It instead shows bucket-dependent transform/family preferences.
- The most useful next visual object is the CDF/quantile trajectory by bucket,
  possibly with bucket-local fitted summaries, rather than a single global
  family name.

## 2026-06-25 - High-`sys` tail diagnostic

Question: does the high-`sys` tail show a simpler or more thesis-relevant
pattern than the full marginal body?

Method:

- fixed bucket groups from
  `/workspaces/msc-math/experiments/sys-datascience/prepare`;
- empirical thresholds q80, q90, q95 in each bucket;
- excess variable `x = sys - u | sys > u`;
- fit GPD with `loc = 0`;
- compare against exponential excess tail;
- for negative GPD shape, record finite endpoint estimate
  `u - scale / shape`;
- bootstrap GPD endpoint estimates with 200 resamples per bucket/threshold.

Command:

```bash
uv run --script experiments/sys-datascience/methods/high-sys-tail-diagnostic/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-tail-20260625/high-sys-tail \
  --bootstrap-count 200
```

Outputs:

- `/tmp/sys-ds-tail-20260625/high-sys-tail/tail-pattern-summary.md`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/tail-fit-readable.md`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/tail-fit-table.tsv`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/bucket-tail-summary.tsv`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/gpd-endpoint-bootstrap.tsv`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/figures/generic-survival.png`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/figures/product-survival.png`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/figures/endpoint-by-bucket.png`;
- `/tmp/sys-ds-tail-20260625/high-sys-tail/summary.json`.

Observations:

- GPD beats exponential in every bucket/threshold cell.
- Exponential log2 likelihood gaps versus the best/GPD tail model:
  - q80: median `-11.35`, worst `-27.53`, closest `-3.47`;
  - q90: median `-3.61`, worst `-13.01`, closest `-0.41`;
  - q95: median `-1.22`, worst `-5.76`, closest `-0.007`.
- Most fitted GPD shapes are negative, giving finite endpoint estimates.
- Product buckets show the clearest bounded-tail pattern. At q90, every product
  bucket has negative-shape bootstrap fraction at least `0.995`, and endpoint
  bootstrap medians are below `0.90`.
- Product q90 endpoint bootstrap medians:
  - `3x3`: `0.666`;
  - `3x4`: `0.802`;
  - `3x5`: `0.811`;
  - `3x6`: `0.871`;
  - `4x4`: `0.859`;
  - `4x5`: `0.859`;
  - `4x6`: `0.840`;
  - `5x5`: `0.858`;
  - `5x6`: `0.894`;
  - `6x6`: `0.867`.
- Generic buckets are noisier. `generic:F10` is the main q90 exception, with
  endpoint estimate `1.179`, bootstrap median `1.006`, and wide upper bootstrap
  tail. Other generic q90 endpoint medians are below `0.87`.

Interpretation:

- The high-`sys` tail is easier to summarize than the full body in this run.
  The strongest pattern is not a single global full-density family; it is a
  bounded-tail excess pattern by bucket.
- This is evidence against a simple exponential/unbounded-tail extrapolation to
  `sys > 1` for the current sampled random/product distributions. Under the GPD
  tail diagnostic, the observed tails usually bend down before 1.
- This is not final evidence that `sys > 1` is impossible under the generators.
  Tail sample sizes are small: q95 has 26 generic rows or 52 product rows per
  bucket. The endpoint estimates are especially fragile in generic buckets.

Follow-up value:

- If we need decision-grade hostile-search allocation, generate more rows in
  high-tail buckets and rerun this tail diagnostic, especially product `5x6`,
  product `6x6`, and generic `F10/F12`.
- If we need thesis-facing exposition, the survival plots plus q90 endpoint
  table are more legible than the full marginal distribution scans.
