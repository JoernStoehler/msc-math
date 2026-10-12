# What the random/product analyses establish

The data-science work asks three different questions: whether a stored sample
contains `sys > 1`, whether features organize its sub-threshold tail, and whether
a rule fixed before target evaluation improves newly generated candidates.
The first has a finite-table negative answer; the second has substantial ridge
symplectic-area signal; the third has bounded sub-threshold enrichment results.
None establishes a calibrated method for finding a counterexample to Viterbo's
conjecture. Jörn's recorded overall scope remains “moderately incomplete”; the
closed individual experiments do not decide which unfinished work belongs in
the thesis. [Scope and entry point](sys-datascience/README.md)
and [current question map](sys-datascience/coordination/current-question-map.md)
own those distinctions.

This account explains the methods and the retained interpretation. The
[physical packet inventory](sys-datascience/methods/README.md)
routes execution code; each packet owns its detailed protocol, generated
results, and caveats. Historical portfolio dispositions are not new execution
authority.

## Population, target, and feature contract

The retained sample has 4,096 generic random polytopes (512 accepted rows at
each `F=5..12`) and 10,240 Lagrangian products (1,024 accepted rows in each of
the ten `3 <= k <= m <= 6` buckets). Both producer contracts use seed 42 and
support heights `[0.8,1.2]`, with rejection to obtain valid accepted rows.
These are different conditioned generators, not interchangeable draws from a
single intrinsic population. Pooled statistics also reflect the chosen bucket
weights. [Producer contracts](polytope-datasets/README.md)
own the laws and retained source objects.

`sys` is the recorded numerical target. The dataset-label column
`capacity_source` identifies its source, not its capacity evaluator. Neither
that column nor the provenance `backend` proves which evaluator route actually
ran. Fresh evaluators and certified bounds must not be attributed retroactively
to the old rows. Read the
[retained numerical lineage](polytope-datasets/retained-lineage.md)
before translating a numerical observation into a mathematical statement.

The [trusted filter](sys-datascience/methods/_shared/random_only.py)
unions provenance dataset labels with `capacity_source`, accepts only
`random_sample`/`random_product_sample`, and excludes optimizer/ascent/continuation
labels. This is a source-membership filter, not a numerical validity
certificate: in particular, a row with an allowed `capacity_source` can pass
without provenance. The stricter
[tail-rule preflight](sys-datascience/methods/tail-rule-mining/analyze.py)
additionally demands unique IDs, finite targets, exactly one provenance row,
and its required control fields. The
[trusted-dataset audit](sys-datascience/methods/trusted-random-dataset/README.md)
retains counts and duplicate/label checks for the actual sample.

The invariant feature matrix contains combinatorial counts/incidences and
symplectic two-face areas normalized by `sqrt(volume)`, with distribution and
concentration summaries. Raw capacity, raw volume, and Euclidean coordinate
summaries are excluded from that matrix. Source/bucket/bounce labels are
separate metadata controls. The
[feature ledger](sys-datascience/feature-space-coverage-ledger.md)
and [table implementation](polytope-invariant-table/invariant_features.rs)
own the definitions and invariance obligations. “Cheap” here means cheaper
than target capacity evaluation in a proposed selection pipeline, not free:
vertices, incidence, volume and ordered ridges still require computation.

### Two schemas, not one simultaneous rerun

The registered prepared snapshot has 39 invariant numeric features. Its
association, projection and prediction artifacts retain that feature list.
Today's shared selector requires 45 features; the six additions are three normalized small-area fractions, ridge entropy,
effective face count, and normalized entropy. It fails on a missing required column or any reported incomplete
ridge ordering. A column must be finite numeric in at least 98% of rows;
`matrix_for` substitutes zero for the remaining missing/nonfinite cells.
This imputation is a consumer behavior, not evidence that missing values mean
geometric zero.

[P2](sys-datascience/methods/standard-baseline-p2/README.md),
the later feature quotient and the recorded tail-rule run used a separately
rebuilt 45-feature table. P2 records prepared-table SHA-256
`49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`
and provenance SHA-256
`6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.
Its producer-object hashes and rebuild command replace dependence on a
surviving `/tmp` directory. The registered table hash is different and lacks
six required ridge fields. A default run of today's strict feature consumers
therefore does not reproduce the older 39-feature summaries. Additional fields
or producer changes need a new identified run; preserve frozen outputs rather
than overwriting them during inspection. The historical full rebuild was
resource intensive and is not a migration check.

## Ordinary retained-table methods

| Question | Actual analysis and retained result | What it supports |
| --- | --- | --- |
| Are any stored targets above one? | [Predicate scan](sys-datascience/methods/scan-sys-gt-1/README.md), with counts/max also retained in the [input audit summary](sys-datascience/methods/trusted-random-dataset/artifacts/summary.json): 14,336 rows, zero `sys > 1`, maximum `0.86258589584944`. | Absence in these bytes; neither universal absence nor correctness of every target. The scan itself prints stdout rather than retaining a separate report. |
| Which sources and tails differ? | [Random-tail EDA](sys-datascience/methods/random-tail-eda/README.md) summarizes source/facet/product strata; [conditional-tail atlas](sys-datascience/methods/conditional-tail-atlas/README.md) retains figures and their numerical TSV summaries. Low ridge sum enriches the within-bucket high tail, with the same correlation sign in all 18 buckets. The most extreme conditioning does not monotonically improve the observed maximum. | Descriptive structure and figure candidates, not a directional construction theorem. |
| Which scalar associations matter? | [Association screening](sys-datascience/methods/statistical-associations/README.md) tests Pearson/Spearman, categorical factors, a bootstrap source mean contrast and a 200-permutation family-maximum rank check. The 39-feature run tested 29 nonconstant scalars; strongest absolute Spearman `0.9384`. | Schema-relative screening with multiplicity and source caveats, not exhaustive feature engineering. |
| Does unsupervised structure identify promising rows? | [Projection/structure](sys-datascience/methods/projection-structure/README.md): standardized PCA, k-means, isolation forest, plus metadata overlays; PC1/target correlation `0.4644`, zero overlap between top-25 anomalies and top-2% targets. | Exploratory geometry/source structure; the recorded anomaly ranking does not select the high target tail. Only its compact JSON is retained. |
| Can ordinary predictors rank already evaluated rows? | [Prediction/ranking](sys-datascience/methods/prediction-ranking/README.md): ridge and random forest, invariant and metadata-only inputs, one grouped holdout. In the 39-feature run forest `R²=0.8852`, versus `0.0086` metadata-only; only ten permutations were used (`p=1/11`). | Held-out retained-table signal under this split, with a coarse null check; no fresh-candidate validation. |
| Do missing standard baselines change that story? | [P2](sys-datascience/methods/standard-baseline-p2/README.md): lasso/elastic-net, histogram boosting, high-tail classifiers and feature-family ablation on the 45-feature rebuild. [Ablation table](sys-datascience/methods/standard-baseline-p2/artifacts/feature-family-ablation.tsv): boosting regression `R²=0.8784` all features, `0.8872` ridge-only, `0.0431` combinatorial-only. | Ridge summaries carry most of the retained holdout signal under the specified partition; this does not prove a mechanism. |
| Can interpretable filters survive controls? | [Tail rules](sys-datascience/methods/tail-rule-mining/README.md): frozen single-feature thresholds, shallow trees, grouped models, budget prefixes, residualized feature attribution and provenance controls. Detailed full-run outputs are described at scratch paths rather than retained in the packet. | A recorded current-schema diagnostic and a reproducible analysis interface; exact detailed metrics need the identified outputs or deliberate regeneration. |

The prediction and P2 split groups are `capacity_source:facet_count`, not
individual rows and not exact product pairs. Product buckets with the same total
facet count travel together. P2's artifact records 8,704 training and 5,632 test
rows; its top-tail label uses the training 90th-percentile cutoff. Conversely,
`prediction-ranking`'s field called `top_decile_enrichment` is the **fraction**
of selected test rows lying above the test target 90th percentile, not a ratio
against baseline prevalence. Its `0.6033` and P2's `3.8121` enrichment ratio
therefore cannot be compared as the same metric. See the actual
[prediction metric implementation](sys-datascience/methods/prediction-ranking/analyze.py)
and [P2 regression table](sys-datascience/methods/standard-baseline-p2/artifacts/regression-metrics.tsv).

Ridge families are not 18 independent discoveries.
[Feature-family quotient artifacts](sys-datascience/methods/statistical-associations/artifacts/feature-family-quotient.json)
check named simple-polytope identities and ridge rank redundancies. At fixed
product bucket, mean and sum are rank-equivalent because ridge count is fixed;
entropy, effective count and normalized entropy also have the same ordering.
Other highly correlated magnitude summaries select different extreme rows.
This is a declared first-pass quotient, not a complete algebraic reduction.
The audit also reports row-weighted pooled correlations after within-bucket
fractional ranking; those are neither equal-bucket averages nor causal estimates.

## From association to a fresh selection result

A fitted model evaluated on rows whose targets were already computed is not a
generated-candidate experiment. The evidential change occurs when geometry and
features are produced first, the IDs/rules/control arm are frozen without target
fields, and only the selected union is target-evaluated. The
[extreme scalar proposer](sys-datascience/methods/extreme-scalar-rejection-proposer/README.md)
retains both the original `100k-promising-scalars` selections and an independent
frozen ridge-concentration validation. The latter validates one sub-threshold
pool-screening rule under its generator, not threshold crossing. Compact
artifacts omit the full geometry/feature caches; their durable configs are
reproduction inputs, and regenerated current evaluators are not byte-identical
historical targets.

The [mechanism discriminator](sys-datascience/methods/ridge-mechanism-discriminator/README.md)
joined already exposed targets and features. Its concentration split was
post-target hypothesis formation, subsequently tested by the independent
packet; it is not itself that validation. Its compact discriminator/overlap
outputs are retained, while exact regeneration also needs the large local
feature cache and detailed tail-rule outputs.

The [generic/product source comparison](sys-datascience/methods/ridge-tail-source-comparison/README.md)
records a frozen generic `F=10` contract, censored product panels, and `5x5`
versus `4x6` sensitivity. Alternate selectors using the same target cache are
reused observations. [Generic stage 1](sys-datascience/methods/generic-ridge-tail-stage1/README.md)
froze 10,000 target-free candidates and
[its target packet](sys-datascience/methods/generic-ridge-tail-stage1-target/README.md)
evaluated exactly 200 selected/control rows. Low ridge transferred as a coarse
filter, but the hardest band did not improve target values; the frozen rule
stopped the experiment at 10,000. [Tail anatomy](sys-datascience/methods/ridge-tail-anatomy/README.md)
separates Euclidean area and Kähler-angle components descriptively without
finding a supported scalar ordering inside the favorable region.

[Canonical vertex covariance](sys-datascience/methods/canonical-vertex-covariance/README.md)
is a post-target retained association using distinct rational primal extreme
vertices and symplectic covariance eccentricity `rho`, not arbitrary point
clouds. The subsequently frozen rho-validation packet is routed from the
scalar proposer. [Alternative-source transfer](sys-datascience/methods/alternative-source-transfer/README.md)
then implements one fresh area-normalized `factorial-both` law in `4x6` and
`6x6`: 91 unique evaluated rows, five rho/ridge overlaps and shared controls.
Both selectors enrich relative to controls in that finite design, with zero
`sys > 1`. This extends the operational sub-threshold finding to that named
source; it establishes no rho superiority, causal support-height effect, broad
population transfer or threshold hit rate.

## Marginal distribution and rare-event reasoning

The [broad fit scan](sys-datascience/methods/sys-distribution-broad-scan/README.md),
[transformed MLE-score table](sys-datascience/methods/sys-distribution-mle-likelihood-table/README.md)
and [excess-tail diagnostic](sys-datascience/methods/high-sys-tail-diagnostic/README.md)
retain code and scratch-run accounts, without compact promoted results. The
broad scan fits 80 families after logit transformation; the MLE table tries
eight transforms; the excess-tail script compares GPD and exponential fits at
bucket q80/q90/q95. Their selected winners, DKW comparisons and negative-shape
endpoints are exploratory. The bounded-domain loader now rejects targets outside strict `(0,1)`,
including threshold hits and nonfinite values, rather than clipping them into
that interval.
That guard does not establish the conjectured bound.

The MLE score includes a Jacobian but fits unconstrained location/scale. It
does not normalize families onto each transform's image of `(0,1)`, and SciPy
fits are not guaranteed global maxima. Until this support contract is repaired,
its cross-transform scores do not compare normalized bounded marginal models.
Ratios of fitted likelihoods also do not bound Bayes factors. This is a live
interpretation limitation of the code, not a reason to reproduce its old
scratch rankings as thesis evidence.

Unlike those scratch packets,
[tail-survival/1M sensitivity](sys-datascience/methods/tail-survival-1m-posterior/README.md)
retains empirical survival, spacings, support-limited resampling, beta-binomial
zero-positive predictions, excess-tail fits, threshold scans and observed-tail
backtests. Its retained zero-positive prior choices imply widely different
1M hit predictions (`0.0367` to `0.986` pooled). GPD and exponential
extrapolations disagree; favorable backtests inside observed support do not
validate extrapolation above the record. The account gives reasons against
calling a blind 1M run a calibrated hit-search plan. It does not prove zero
positive probability or authorize another run.

## Later retained interventions and search pilots

The July closure summaries do not encompass every later result.
[Selection mechanism](sys-datascience/methods/selection-mechanism/README.md)
reranks ridge sum and targets inside selected strata. The pooled coefficient
falls from `−0.944` for all rows to `−0.239` in the high-target 5% and `−0.113`
in the low-ridge 5%. Those subsets reuse known targets; range restriction is
an interpretive caveat, not proof of a mechanism. A
[Gaussian-copula reference](sys-datascience/methods/selection-mechanism/gaussian-result.md)
calibrated to each group's bulk correlation attenuates much less, so that
particular dependence model does not reproduce the conditional pattern.

[Paired tangentialization](sys-datascience/methods/paired-tangentialization/README.md)
freezes 16 latent pairs in `4x4` and `4x6`, each with four support-height arms.
All 64 numerical evaluations succeeded; the both-factor mean effect is
`+0.01527`, with 11 positive and five negative pair effects. The jointly
admissible source and bucket heterogeneity prevent a universal law. Its
[later fixed-normal analysis](sys-datascience/methods/selection-mechanism/controlled-result.md)
adds no target calls and finds opposite ridge/target changes in 15/16 primary
pairs. [Covariance balancing](sys-datascience/methods/selection-mechanism/balance-result.md)
uses a target-blind 320-product selection and 640 baseline/transformed calls:
314 ratios improve and the within-group ridge/target coefficient weakens from
`−0.929` to `−0.249`. The specified determinant-one factor deformation changes
shape and relative position; it does not isolate covariance ratio as a unique
cause. Agreement of these 320 recomputed product baselines with old targets
corroborates that subset, not the whole historical sample.

[Orientation allocation](sys-datascience/methods/orientation-allocation/README.md)
compares 16 fixed-orientation requests on a shared start with 16 fresh IID
shapes in four frozen blocks. All 124 unique targets (128 charged memberships)
succeed, but IID has the higher block maximum in all four blocks and costs less
on this evaluator. Improving a particular starting body and winning a search
budget comparison are distinct outcomes.

[Diagonal CEM pilot](sys-datascience/methods/diagonal-cem-pilot/README.md)
retains a later, actually executed three-seed `5x5` comparison: 1,344 successful
unique targets, 1,536 charged memberships, best and top-eight-median improvements
in all three blocks, maximum `0.85565`, no threshold hits. It is separate from
the unrun July full IID/local/CEM programme. Supported-v2 records an earlier
partial target exposure and an admission repair; that history precludes
presenting it as pristine first-exposure validation. Its analyzer also checks
historic executable hashes and overwrites summaries, so the retained JSON
is the ordinary reading surface, not an automatically runnable no-argument
recipe.

The [current-body adapter](sys-datascience/methods/current-body-evaluator/README.md)
is reusable serial evaluation infrastructure with retained analytic/scaling/
unitary/dispatch/failure controls, not a population experiment. Its
[retrospective interval fork](sys-datascience/methods/retrospective-interval-evaluator/README.md)
preserves capacity bounds when scalar tolerance is refused; it does not
certify the volume or a ratio interval. These distinguish evaluator readiness
from the scientific outcomes above.

## Geometry-specific and parked method interfaces

These packets answer different questions from ordinary scalar prediction:

- [Bounce distribution](sys-datascience/methods/product-bounce-distribution/README.md),
  [bounce mechanism](sys-datascience/methods/product-bounce-mechanism/README.md)
  and [class degeneration](sys-datascience/methods/product-bounce-class-degeneration/README.md)
  describe generator-local two/three-bounce associations and decomposition;
  neither labels nor controlled associations alone establish causality.
- [Triangle classification](sys-datascience/methods/product-triangle-bounce-classification/README.md)
  and [two-bounce width shortcut](sys-datascience/methods/product-bounce-width-shortcut/README.md)
  are bounded support/check packets for separate formal arguments. Their
  packet READMEs route the authoritative mathematical proof sources; the
  empirical artifacts provide checks rather than proof premises.
- [HKO coverage](sys-datascience/methods/hko-reference-coverage/README.md)
  and [HKO ridge smoke](sys-datascience/methods/hko-ridge-source-smoke/README.md)
  use a declared known-positive reference, rather than discovering a new one.
- [Orientation smoke](sys-datascience/methods/generator-orientation-smoke/README.md)
  and [orientation target pilot](sys-datascience/methods/generator-orientation-target-pilot/README.md)
  supply a fixed-shape witness and unitary control, with a failed directional
  ridge-mediator gate. No population or proposer-transfer claim follows.
- [Ridge pressure](sys-datascience/methods/ridge-tail-pressure/README.md),
  [endpoint paths](sys-datascience/methods/ridge-endpoint-path/README.md)
  and [symmetry completion](sys-datascience/methods/ridge-symmetry-completion/README.md)
  combine explicit finite path observations and separately owned mathematics;
  they do not empirically certify an unrestricted endpoint inequality.
- [Residual exemplar seeds](sys-datascience/methods/residual-exemplar-seeds/README.md)
  are hypothesis-forming inspections. [Equal-budget product search](sys-datascience/methods/equal-budget-product-search/README.md)
  is a parked prototype without the intended real comparison. Reusable AMS
  code is not an empirical result.
- [Distribution sensitivity](sys-datascience/methods/distribution-sensitivity/README.md)
  and [random-axis diagnostic](sys-datascience/methods/random-axis-diagnostic/README.md)
  are variant-analysis interfaces. The latter describes a 1,152-row scratch
  two-height check, not thesis-scale generator coverage. Independent seeds
  address sampling variability; a particular PRNG seed is not a geometric axis.

[Tail-dependence feasibility](sys-datascience/methods/tail-dependence-feasibility/README.md)
records why censored generated panels could not answer all tail questions and
why the bounded generic experiment was chosen. The
[method dispositions](sys-datascience/methods/trusted-random-product-method-dispositions.md)
record covered interfaces, unexecuted methods and reopen conditions; “covered
by interface” or “parked” does not mean scientifically evaluated.

## Maintained duplication and the next consumer

The packet code and generated artifacts remain the source owners. This account
collects their relationships; the closure summary and question map are routing
views. They must not acquire independent metric tables that silently supersede
the packet. Known feature redundancy is audited explicitly rather than
interpreting column count as geometric breadth. Exact duplicate height-range
labeling in prediction, projection and tail-rule code now comes from the
shared helper; association's intentionally different missing-value behavior
remains separate.

For thesis composition, choose whether the intended sentence reports finite
absence, a retained association, a frozen generated enrichment, or a theorem.
Then follow the appropriate packet to its bytes and numerical/proof boundary.
Before stronger wording, resolve the outstanding schema/reproduction gaps and
have the proposed claim reviewed. No new experiment is selected by this
account.
