# Canonical contact invariant fails broad pentagon-product tails

The frozen canonical product score passes numerical coordinate and relabelling controls. Its two unchanged local predictors fail on the complete **1,024-row retained arbitrary $5\times5$ bucket**: upper-decile rank correlation is **−0.00763**, falling to **−0.5857** in the top 2%. Thus the canonical extension does not supply the requested broad upper-tail predictor. No target fit, clipping, recalibration or choice between the two formulas was made after scoring.

## Frozen definition and test

The independently owned [canonical API](../contact-stability/canonical.py) fixes a nine-parameter product gauge: map three labelled primal vertices of the first pentagon to the reference HKO triangle, transform the second factor contragrediently, center it, and fix a support scale. The score minimizes the fixed 20-form contact defect over two factor roles, two common cyclic orientations and five starts per factor. There are at most 100 candidate gauges; exactly 50 have the required positive scale in every retained row. The definition is for products with their splitting, not arbitrary four-dimensional bodies. Its exact invariance argument and local-theorem obligations belong to the [contact packet](../contact-stability/CANONICAL-PLAN.md).

[PLAN.md](PLAN.md) freezes the two parameter-free formulas
$$
 \widehat{\mathrm{sys}}_{\exp}=S_H e^{-I},\qquad
 \widehat{\mathrm{sys}}_{\rm recip}=\frac{S_H}{1+I},
 \qquad S_H=(3+\sqrt5)/5.
$$
Both are evaluated throughout. The comparison with a raw chart, a target-selected gauge and this explicit finite geometric gauge was settled before computation. A raw chart would leave coordinate dependence; a gauge selected by capacity would use the target. The chosen finite geometric normalization makes the proposed broad transfer directly testable. Bulk fit alone would reward degeneration; tail ordering alone would omit calibration, so errors, rank and recall are all reported.

The complete source-order pentagon bucket is used without ratio selection. Historical target values were visible in earlier work, so this is a retrospective transfer experiment. The new geometry stage does not access the source's `sys` or `capacity` fields. It saved all predictions before a separate scoring stage read these historical target scalars. [frozen-predictions.json](frozen-predictions.json), SHA256 `4d787bfe27fada28d0dce6a247c68d0551f46f3ce5eac3fcfebe66f363a9b853`, records the prediction, source, plan, API and gradient-bank identities.

## Broad and upper-tail results

Relative errors are absolute prediction/target errors. Each tail contains the highest true ratios in this one bucket; recall compares that set with the same number of highest predictions from all 1,024 rows. Ranking is Spearman within the true tail.

| Formula | Bulk median / p90 error | Top 10%, 103 rows | Top 5%, 52 rows | Top 2%, 21 rows |
|---|---:|---:|---:|---:|
| $S_H e^{-I}$ | 27.80% / 279.85% | 25.76% / 40.57% | 28.71% / 44.32% | 29.53% / 44.64% |
| $S_H/(1+I)$ | 43.16% / 376.64% | 12.09% / 24.46% | 16.16% / 26.94% | 18.57% / 29.24% |

| Same ordering for both formulas | Bulk | Top 10% | Top 5% | Top 2% |
|---|---:|---:|---:|---:|
| Spearman | 0.36769 | −0.00763 | −0.01818 | −0.58571 |
| Tail recall | — | 0.28155 | 0.19231 | 0.14286 |

The ratio range is **0.0014871974–0.8437955079**, while the invariant lies only in **0.4067054270–1.1911353157**. Consequently the exponential predictions occupy **0.3182231549–0.6972770146**, and the reciprocal predictions occupy **0.4779319598–0.7444441284**. There are no negative invariants, invalid predictions or overflow repairs in this run. The compressed prediction range misses both severe degeneration and the largest retained ratios.

Matched parameter-free baselines use the same source rows and targets: the geometric two-bounce ratio ceiling $W$, and the centered-polar calibration $c=5/\alpha$, equivalently $25/(2A^2)$ for $A=\alpha\sqrt V$. These are predeclared comparisons, not selected replacement formulas.

| Baseline and population | Median / p90 error | Tail Spearman | Recall |
|---|---:|---:|---:|
| Width, top 10% | 8.66% / 35.18% | 0.51177 | 0.56311 |
| Polar, top 10% | 19.41% / 35.21% | 0.34010 | 0.65049 |
| Width, top 2% | 15.66% / 21.24% | 0.19091 | 0.09524 |
| Polar, top 2% | 20.37% / 27.40% | 0.53506 | 0.33333 |

The reciprocal contact formula has a smaller top-decile p90 error than these two baselines, yet its within-tail ordering is essentially absent and reverses in the smaller top tail. This does not satisfy a predictor required to resolve the upper tail. The [complete results](results.json) include all formulas, the constant-HKO control, each retained row, and explicit counterexamples.

## Counterexamples and scope

| Body | Historical ratio | $I$-exponential prediction | $I$-reciprocal prediction |
|---|---:|---:|---:|
| `random_5x5_s2_104`, largest true ratio | 0.84379551 | 0.46708773 | 0.57941260 |
| `random_5x5_s0_92`, second largest | 0.83853282 | 0.43733744 | 0.55905564 |
| `random_5x5_s2_148`, highest prediction | 0.60776923 | 0.69727701 | 0.74444413 |
| `random_5x5_s2_0`, largest relative error | 0.00148720 | 0.44746807 | 0.56597482 |

[examples.json](examples.json) retains the exact rational input normals for these four bodies and their historical numerical labels. The true leaders are underpredicted, so the numerical evidence also contradicts treating either formula as an upper bound on this corpus. This lane does **not** claim an exact global-bound counterexample: it reuses historical scalar targets and does not certify their capacities. It establishes a broad numerical predictive failure, conditional on the original label contract.

This does not refute the separately tested local asymptotic statement in the canonical neighborhood, or other functions of the canonical geometry. The finite normalization and the local linear defect solve different obligations. Passing invariance controls supplies no reason for that tangent approximation to predict arbitrary pentagons. No further fit or feature expansion was launched after this failure.

## Verification and reproduction

The first ten geometries plus 60 controls took 1.04 seconds. The conservative projection including all those controls per body was 107 seconds; the full 1,024 prediction run with only ten control bodies took **8.24 seconds**. No expensive producer or package installation was needed.

For each of the first ten source-order bodies, controls apply paired positive- and negative-determinant linear maps, common scaling, independent translations, arbitrary normal-row permutations and factor interchange. The largest scaled change in $I$ is **$4.67\times10^{-15}$** across the 60 controls. This is numerical corroboration of the independently owned exact invariance argument.

[verify.py](verify.py) independently reconstructs all formula values, 20 error/rank/recall metric groups and the coordinate-control receipt. It replays the four exact rational example geometries through the pinned API and gradient bank. [verification.json](verification.json) records **PASS**. These checks do not certify historical capacities, prove a global inequality, or independently establish the contact packet's local theorem.

```bash
python3 patterns/upper-tail/canonical-transfer/verify.py
```

Geometry reproduction needs only Python's standard library and the registered source cache; it writes a new scratch prediction file:

```bash
python3 patterns/upper-tail/canonical-transfer/predict.py \
  --source /home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl \
  --canonical patterns/upper-tail/contact-stability/canonical.py \
  --contact patterns/upper-tail/contact-stability/predictions.json \
  --plan patterns/upper-tail/canonical-transfer/PLAN.md \
  --out /tmp/msc-math/upper-tail-20261017/canonical-transfer-reproduce
```

The separate scorer accepts explicit `--predictions`, `--expected-sha`, `--source`, `--width`, `--polar` and `--out` arguments. NumPy/SciPy are needed only for scoring; the run reused the existing cached environment. Reproducing the metric computation from the retained frozen predictions is:

```bash
uv run --offline --with numpy==2.5.4 --with scipy==1.18.1 python \
  patterns/upper-tail/canonical-transfer/score.py \
  --predictions patterns/upper-tail/canonical-transfer/frozen-predictions.json \
  --expected-sha 4d787bfe27fada28d0dce6a247c68d0551f46f3ce5eac3fcfebe66f363a9b853 \
  --source /home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl \
  --width patterns/product-shape/features.csv.gz \
  --polar patterns/upper-tail/polar-geometry/features.jsonl.gz \
  --out /tmp/msc-math/upper-tail-20261017/canonical-transfer-reproduce/results.json
```

The focused source inspection covers the canonical API/plan, the fixed contact gradient bank and reference, the complete retained pentagon input bucket and the matched width/polar feature tables. No fresh third-panel targets, generic-body experiment or broad literature search entered this lane. Root owns independent review, integration, commit and cleanup.

[review: /root/review_mixed]
