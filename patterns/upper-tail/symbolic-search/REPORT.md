# Sparse formulas retain ranking but miss the above-one level

A small symbolic search finds stable geometric rules with useful historical upper-tail errors. Their unchanged corpus-fitted coefficients nevertheless put **all 41 above-one fresh products below one**. The best product expression ranks the fresh challenge very well (Spearman 0.9879), so its failure concerns calibration as well as the distinction between a sampled upper decile and the actual near-HKO regime. This lane supplies neither a universal invariant law nor a global-maximality conjecture.

The method and success criteria were written in [PLAN.md](PLAN.md) before computation. This is retrospective nested validation on the old corpus: earlier feature scores were already visible. The fresh challenge truth file was not read by this lane until its selected formulas, coefficients and 60 predictions were frozen in [frozen-predictions.json](frozen-predictions.json), SHA256 `80500673dfb68400c26601d043caf1d9ee6463388bc901ee920c1b337b4a37de`. The parent supplied aggregate information about another lane's performance and the count of above-one challenge rows; no individual challenge ratio was used in selection or fitting.

## Geometry and finite grammar

For every body use four positive dimensionless primitives: normalized unsigned symplectic ridge sum $R$; symplectic eigenvalue ratio $\rho$ of the uniform distinct-vertex covariance; $A=\alpha((K-g)^\circ)\sqrt{V}$ with solid-volume centroid $g$ and $\alpha=\max_{i,j}|\omega(b_i,b_j)|$ on polar vertices; and the centered Mahler product $M=V\operatorname{vol}((K-g)^\circ)$. For products add the geometric two-bounce ratio ceiling $W$ and the product $D$ of the two factor area-asymmetry ratios. These are target-free; the last two depend on the product presentation. The covariance exclusion remains exactly the two preexisting rows `random_F8_s3_45` and `random_F12_s2_38`.

Normalize primitives by their values on the supplied rational HKO geometry. The relevant constants are
$$
 R_H=8.94427190999916,\quad \rho_H=1,\quad
 A_H=3.454915028125466,\quad M_H=12.433789428714357,\quad
 W_H=1.0472135954998352,\quad D_H=1.25.
$$
The reference capacity/ratio is not used to calibrate coefficients. Normalizing by any fixed positive constants preserves target independence; here it also makes a reference prediction easy to inspect.

The grammar contains each primitive, two-primitive monomials with fixed half/integer powers, arithmetic/quadratic/root means and their reciprocal means, minimum, maximum and ratio. Every expression $E$ receives exactly two fitted coefficients, $\log\widehat{\mathrm{sys}}=a+b\log E$. There are **130 expressions for all bodies** and **321 for products**, including algebraic duplicates. A constant, an unrestricted log-linear combination of the primitives, and the existing quadratic in $\log R,\log\rho$ are diagnostic baselines; they are not selected as grammar winners.

Outer validation holds out each exact generator bucket. Expression selection uses three inner bucket-hash folds and a loss giving equal weight to bulk and within-bucket upper-decile squared log error; only then are the two coefficients fitted on the outer training rows. A separate outer name-hash holdout repeats selection with inner name folds. Hashes use the first eight SHA256 hex digits. No outer test score changes the grammar or selects a formula. The final deployable expression uses the same inner bucket folds on the full old corpus. [search.py](search.py) retains all exact formulas, selection rules and coefficients.

Options considered were arbitrary polynomial/tree fitting, a few handpicked laws, and the finite sparse grammar. The first rewards flexibility without an explanation, the second risks missing a simple interaction, and raw bulk $R^2$ rewards fitting degeneration. The selected rule balances calibration in ordinary and upper-decile rows while requiring separate rank/recall and fresh near-HKO checks. No larger search was launched after the fresh failure.

## Selected rules and held-out performance

Every one of the 18 outer all-body folds selects the arithmetic mean of normalized $\rho$ and $A$; every one of the 10 outer product folds selects the minimum of normalized $M$ and $W$. Thus expression instability does not explain the fresh calibration error. The full-corpus rules are
$$
\widehat{\mathrm{sys}}_{\rm all}
=0.9279759874492577
\left[\frac{\rho/\rho_H+A/A_H}{2}\right]^{-0.9670670817758096},
$$
$$
\widehat{\mathrm{sys}}_{\rm product}
=0.9153914829371319
\left[\min\left(\frac{M}{M_H},\frac{W}{W_H}\right)\right]^{0.9582695866751867}.
$$

Errors below are absolute relative errors, not log errors. Tail ranking is the mean Spearman correlation among the true upper-decile rows in each held-out bucket. Recall is the mean fraction of that true decile recovered by selecting the same number of highest predictions within the bucket.

| Procedure and population | Bulk median / p90 error | Upper-decile median / p90 error | Tail rank | Top-decile recall |
|---|---:|---:|---:|---:|
| All-body nested bucket holdout, 14,334 rows | 12.61% / 30.82% | 8.67% / 21.96% | 0.422 | 0.690 |
| All-body nested name holdout, 2,765 rows | 12.35% / 30.81% | 8.80% / 21.88% | 0.343 | 0.682 |
| Product nested bucket holdout, 10,240 rows | 6.62% / 15.97% | 7.76% / 16.20% | 0.516 | 0.647 |
| Product nested name holdout, 2,013 rows | 6.48% / 16.25% | 8.08% / 17.56% | 0.431 | 0.679 |

In the held-out $5\times5$ bucket, the product rule's upper-decile median/p90 errors are 7.34%/11.50%, tail rank 0.592 and recall 0.592. These are not measurements on near-HKO bodies. Historical ratios remain the original numerical labels, not newly certified ratios.

For context, the unrestricted six-primitive product log-linear baseline achieves bulk log $R^2=0.99103$ but selected-tail log $R^2=-0.29856$; the selected sparse rule gives 0.99150 and 0.49768. The comparison supports reporting tail behavior separately. It does not establish a theorem or justify choosing another formula after inspecting test tails. [results.json](results.json) includes all buckets, inner rankings, exact fitted coefficients and diagnostics; [LOBO-predictions.jsonl.gz](LOBO-predictions.jsonl.gz) contains each outer prediction.

## Fresh nonregular near-HKO falsifier

The separately owned challenge contains five independent directions in each of full-normal, angular and support perturbations, at four fixed scales: 60 five-by-five products. Its exact rational closure oracle yields ratios from 0.8805656811 to 1.0466140347; **41 exceed one**. The challenge samples nonregular perturbations near HKO rather than only rotated regular products. The statistical split is blind to this lane's individual targets; this lane does not independently reprove the oracle's imported capacity reduction.

| Frozen rule | All 60 median / p90 error | Above-one 41 median / p90 error | All-60 Spearman | Predicted above one |
|---|---:|---:|---:|---:|
| All-body mean rule | 10.97% / 11.36% | 11.16% / 11.37% | 0.9204 | 0 |
| Product minimum rule | 12.47% / 12.58% | 12.53% / 12.58% | 0.9879 | 0 |

For `contact_support_s0_e3of1000`, the true ratio is **1.0466140347**, while the two frozen predictions are **0.9274353213** and **0.9148892587**. The HKO reference itself is predicted as 0.9279759874 and 0.9153914829 instead of $(3+\sqrt5)/5=1.0472135955$. The product fit underestimates that reference by 12.59%, essentially the same error as on the closest fresh perturbations. It therefore contains useful local ordering information without the correct near-maximal level.

The frozen strong-success conditions required median fresh error below 10% as well as held-out tail calibration, rank above 0.5 and recall above 0.5. Neither selected procedure satisfies all conditions. No recalibration to HKO, feature change, or further expression selection was made using these challenge targets. This falsifies the two selected corpus-trained procedures; it does not exclude every member of the grammar or every function of these primitives. [blind-results.json](blind-results.json) retains every fresh target, prediction and error.

## Proof prospect and a cheap obstruction

The product expression might suggest the HKO-calibrated envelope
$$
 \mathrm{sys}(P\times_LQ)\ \leq\
 \min\left\{\frac{S_H}{M_H}M(P\times_LQ),W(P,Q)\right\},
 \qquad S_H=(3+\sqrt5)/5.
$$
Its width branch is an existing valid ceiling. Its Mahler branch is **false**. Take $P=[-1,1]^2$ and $Q=\{|x|+|y|\leq1\}$. The existing theorem for two centrally symmetric planar factors gives $c=4$, while $V=8$, so $\mathrm{sys}=1$. The product polar is the free join of the planar polars, giving $V^\circ=4/3$ and $M=32/3$. For a regular pentagon product,
$$
 M_H=\frac{3125(3-\sqrt5)}{192},\qquad
 \frac{S_H}{M_H}\frac{32}{3}
 =\frac{1024(7+3\sqrt5)}{15625}
 =0.8983808529\ldots<1.
$$
Thus even an HKO calibration does not turn the selected Mahler/width shape into a universal product bound. The gap is about 0.102, larger than the 0.0472 gap between one and HKO. The symmetric-product theorem is attributed in the existing [product source check](../../../topics/products/pentagon-arbitrary-partner/source-check.md), as used by the earlier [exact shape controls](../../product-shape/REPORT.md). The counterexample is a falsifier for this proposed implication, not a new capacity theorem. [verify.py](verify.py) checks its rational bookkeeping and the strict inequality using $\sqrt5<9/4$.

No proof attempt for the fitted coefficients is warranted after the fresh failure. The best surviving interpretation is a compact historical predictor and a useful local ranking surrogate; neither supplies a global-maximality route.

## Verification, sources and reproduction

[verification.json](verification.json) records a PASS from the independent artifact checker: it reconstructs every retained LOBO median/p90 error, all fresh formula predictions and fresh errors, the count of 41 above-one targets, and the exact Mahler-envelope obstruction. On 32 retained product geometries, independent reconstructions of all six primitives match the input feature tables with maximum relative discrepancy $2.59\times10^{-11}$. This checks normalization and joins, not the validity of historical capacities or the fresh oracle theorem. The bounded full symbolic calculation took 8.60 seconds on the workstation, excluding earlier lanes' feature production.

The inspected sources are the entry/readme/rationale, original pattern gallery and success criteria, root target statement, polar feature producer, covariance feature table, earlier product geometry producer and width features, and the geometry-only fresh sample before freezing. No broad repository/literature survey was performed. NumPy/SciPy were reused from an existing local cached environment; there was no package installation, capacity producer, remote job or external action.

Run the retained check with installed NumPy:

```bash
python3 patterns/upper-tail/symbolic-search/verify.py
```

For full reproduction, use the retained polar feature table, the canonical registered source cache, and the separately retained fresh sample. The command only writes the supplied scratch output directory:

```bash
uv run --offline --with numpy==2.5.4 --with scipy==1.18.1 python \
  patterns/upper-tail/symbolic-search/search.py \
  --repo "$PWD" \
  --polar patterns/upper-tail/polar-geometry/features.jsonl.gz \
  --challenge patterns/upper-tail/contact-stability/sample.json \
  --plan patterns/upper-tail/symbolic-search/PLAN.md \
  --source /home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl \
  --out /tmp/msc-math/upper-tail-20261017/symbolic-reproduce
```

The input hashes are recorded in `results.json`. `--challenge` and `--truth` accept explicit alternative input paths when reproducing outside this checkout; matching sample hashes are checked. The scoring stage is separate from fitting and checks the frozen prediction digest before reading numerical targets:

```bash
uv run --offline --with numpy==2.5.4 --with scipy==1.18.1 python \
  patterns/upper-tail/symbolic-search/score.py \
  --predictions patterns/upper-tail/symbolic-search/frozen-predictions.json \
  --truth patterns/upper-tail/contact-stability/truth.json \
  --out /tmp/msc-math/upper-tail-20261017/symbolic-reproduce/blind-results.json
```

Root owns integration, final review, retention and commit. No questions were added by this lane.

[review: /root/review_mixed]
