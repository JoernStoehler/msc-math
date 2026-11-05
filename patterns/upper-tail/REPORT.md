# A sparse triangle envelope removes 25 of 26 width false positives

A geometry-only selection of one circumscribed triangle per factor produces
**exact upper bounds below one for 25 of the 26 retained 5×5, 5×6 and 6×6
products whose width ceiling exceeds one**. Every rejected upper bound is at most
**0.8089260667**. The same selection preserves the known HKO above-one
configuration in a separate numerical control. The lone retained survivor
shows that this selection does not replace full capacity computation.

The subsequent [frozen full-population tail diagnostic](FROZEN-TAIL.md) is now complete: 3,072 rows, 484 improved width bounds, but top-2% capacity gaps up to 12.74%. The exact 25/26 rejection remains valid; the fixed selector is stopped as a uniformly tight upper-tail explanation. The next-task proposal below is preserved as the pre-execution rationale, not an outstanding assignment.

This is a finite geometric compression of the missing asymmetric competitor,
not a new global systolic bound or a theorem of HKO maximality. The underlying
sparse product formula is existing mathematics. A preceding odd-moment model
probe failed and was stopped; its result is supporting method selection.

## Success picture, alternatives and selection

The useful end state was a cheap shape descriptor or bound that resolves the
upper-tail question hidden by strong pooled scores. Source identity cannot be
skipped because historical targets and exact input geometry have different
evidence status. Explicit controls cannot be skipped because a predictor can
miss relative position. Tail holdouts cannot be replaced by pooled R² because
degeneration accounts for most target spread. Once the selected contraction
model failed, a bound was preferable to further fitting.

| Prior | Promise | Main failure mode | Decision |
|---|---|---|---|
| Covariance/ridge eccentricity | Established bulk association | Existing exact moment-angle obstruction | Do not retune |
| Width with scalar area asymmetry | Exact two-bounce ceiling plus area cost | Ignores the asymmetric shorter competitor | Baseline |
| Odd cross-factor shape contractions | Fifth contraction sees the pentagon angle | A global average can miss a localized competitor | Bounded test failed; stop |
| Minimum-area circumscribed-triangle envelope | Actual capacity upper bound from selected feasible supports | Selected supports may miss the decisive competitor | Selected 26-row plus HKO bound test |
| Closure-weighted face areas | Jörn's new area-sum hint; feasible weights could encode closure | Generic weights do not establish a bound | Independent probe owned elsewhere |

Rubrics considered were bulk R² (wrongly rewards low-ratio spread), tail R²
alone (can overpenalize restricted variance), best sampled ratio (rewards
polishing), and explanatory content plus a falsifiable threshold. The last
rubric selected the envelope after the contraction failure. Capacity/A₂,
minimizing words and class minima never enter the geometric selection or
predictors: those would contain target-search information.

## The exact selected envelope

Write the original rational factors as
P={x:uᵢ·x≤1}, Q={y:vⱼ·y≤1}. For each factor, keep every bounded triangle
obtained by selecting three original supporting halfplanes. Among them choose
the triangle(s) with **least area**, using exact comparisons. The retained
26 bodies have one minimizing triangle per factor. Selection does not read
historical capacity or systolic ratio.

For selected Tₚ⊇P and Tq⊇Q, monotonicity gives

\[
 c(P\times_LQ)\le c(T_p\times_LT_q).
\]

A bounded supporting triangle has a unique positive normalized closure vector
α with Σαᵢ=1 and Σαᵢuᵢ=0, and similarly γ for the second factor. The existing
[product sparse-reduction corollary](../../topics/products/product-qp-six-facet-reduction.tex)
assigns weights α/2 and γ/2 and evaluates the 120 cyclic orders of six labels.
For each order σ let Qσ be the Haim–Kislev quadratic objective. Then

\[
 C_T=\frac1{2\max_\sigma Q_\sigma},\qquad
 U=\min\{A_2,C_T\},\qquad
 \operatorname{sys}(P\times_LQ)\le
 \frac{U^2}{2\operatorname{area}(P)\operatorname{area}(Q)}.
\]

The denominator uses the **original** areas, not the outer triangles' areas.
A selected subset supplies only an upper bound, never a completeness claim
for the original polygon product or the three-bounce class.

A particularly small certificate suffices for each exact rejection. Retain
one maximizing selected-triangle word, its six rational weights and its exact
positive Q. These are also feasible original-body Haim–Kislev data, so
Q≤Qmax and c(original)≤1/(2Q). Thus the exact rejection predicate is simply

\[
 8Q^2\operatorname{area}(P)\operatorname{area}(Q)\ge1.
\]

The verifier needs neither historical capacity labels nor a proof that the
retained word is optimal. The complete selected-triangle search explains
how the witnesses were found; feasibility alone certifies their bound.

## Results and the surviving obstruction

The scope was exactly the 26 retained 5×5, 5×6 and 6×6 products with s₂>1,
plus the separate known HKO control. No additional body search was performed.

| Check | Outcome |
|---|---:|
| Width survivors tested | 26 |
| Selected triangle bound tightens width | 26/26 |
| Exact ratio upper bound ≤1 | **25/26** |
| Largest upper bound among the 25 rejected | **0.8089260667** |
| Selected-triangle cyclic words, rational bodies | 3,120 |

The largest width peak, `random_5x6_s2_220`, drops from a width ratio ceiling
**1.32956814** to an exact selected-triangle ratio bound **0.52615025**.
This resolves a large part of the width-screen obstruction with one
geometry-selected pair, rather than a small score improvement.

The one survivor is `random_5x6_s2_150`:

- width ratio ceiling: **1.0144154992**;
- selected-triangle ratio upper bound: **1.0100801360**;
- inherited historical numerical ratio: **0.7946614734**.

The last line is descriptive, not a premise of the exact bound. Its inherited numerical ratio indicates a missed competitor; the exact
certificate itself leaves above-one unresolved. No further supports were
chosen after seeing this failure. The rule rejects
most of this particular width-selected population, but does not establish a
universal law or complete upper-tail ordering.

For circumradius-one regular pentagons at relative angle π/10, all five
minimum-area outer triangles per factor tie. Their 25 pairs require 3,000
floating word evaluations. The selected triangle bound is
**1.0472135954999575**, agreeing with the imported exact known HKO ratio
(3+√5)/5=1.047213595499958… to rounding. This HKO control is **numerical**;
it is excluded from the rational certificate count and exact verifier.
Preservation of the above-one possibility is also consistent with the bound's
monotonicity. No new exact HKO calculation is claimed here.

The result supports sparse closure/area geometry as a useful route beyond the
width ceiling. It does not yet justify a substantial universal-bound programme.
Stop the failed moment-fitting family.

A useful **bounded next owner task** is to freeze this exact triangle selector,
without fitting or adding supports, then evaluate all 3,072 retained 5×5, 5×6
and 6×6 products. The new binary crux is whether the same selected-support rule
remains tight in each bucket's actual upper tail, rather than merely rejecting
width-screen false positives. Report per-bucket top-10%, top-5% and top-2%
distributions of U/c(historical), the worst gaps and support-pair failures;
compare against width alone and preserve the same HKO control. These are
retrospective bound-tightness diagnostics, not a new predictor or prospective
validation. Freeze choices before examining the additional outcomes. Do not
optimize triangle selection or train a surrogate on the expanded results.
The measured final26-row run suggests a few local minutes, but that is an
extrapolation; cap preliminary execution at five wall minutes and retain any
failure instead of launching a full capacity pipeline. A numerical rejection
count across this full set would add little by itself, because width already
rejects every row outside the original 26. Tail tightness and failure geometry
are the consequential outcomes.

Compare this task with the independently probed symplectic face-area weights;
feasible dual-triangle barycentric weights are a natural way to make such
weights respect closure. Exhausting all sparse supports would just recover
the existing capacity algorithm and would not by itself add an explanation.

## Supporting failed prior: a pentagon-aware cross invariant

Let X,Y be independently uniform on distinct factor extreme vertices, centered
at their separate vertex means. Define

\[
 m_k=\mathbb E[(X\cdot Y)^k]/\mathbb E[(X\cdot Y)^2]^{k/2}.
\]

Dot products are unchanged by P↦AP, Q↦A⁻ᵀQ. Centering and normalization give
translation and independent positive-scale invariance. Factor interchange
preserves mₖ; **central reflection** of one factor changes odd signs, so their
absolute values remain unchanged. These are product-aware invariants, not
invariants under every symplectic map.

For unit regular pentagons at relative angle θ, root-of-unity averaging gives
E[(X·Y)²]=1/2 and E[(X·Y)⁵]=cos(5θ)/16. Therefore
2√2|m₅|=|cos5θ|, which exactly reconstructs the symmetry-reduced angle and the **known** ratio
profile through d=arccos(2√2|m₅|)/5. The mixed moment has degree five in each
factor, total degree ten; it does not contradict the earlier degree-four
moment obstruction. This is a short deduction from the existing formula,
not a new capacity theorem.

On all 10,240 retained products, OLS on log(sys) used
(1, log s₂, (log s₂)², log(αPαQ)) as baseline and additionally |m₃|,|m₅| as the
chosen six-coefficient correction. Two single-feature ablations are retained.
Each exact side-count bucket was held out in turn; a deterministic name-hash
split additionally tested represented buckets. No tuning followed inspection.

| Bucket-held-out model | Bulk log R² | Top-decile log R² | Top-decile within-bucket rank correlation | Top-decile recall |
|---|---:|---:|---:|---:|
| Width calibration | .98543 | −1.67811 | .37782 | .63010 |
| Width + asymmetry | .99165 | −.25125 | .43757 | .71359 |
| Plus both odd contractions | .99171 | −.22721 | .44405 | .71942 |

Each bucket's selected target decile contains 103/1,024 rows. Rank correlations
rerank inside selected sets and pool percentiles. Recall selects the highest
103 predictions from the whole held-out bucket and measures target-decile
overlap. Tail R² uses a pooled selected-target mean and does not imply each
bucket has negative R². On the name-hash split, the correction changes tail
ordering **.31455→.31341** and recall **.70260→.69760**. This supplies no
consistent material improvement. It rejects the selected simple correction,
not every nonlinear use of moments or every distribution. Full coefficients,
scores and representative rows remain in [results.json](results.json).

## Verification, reproduction and evidence limits

The registered source cache was verified offline. SHA-256:
`66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`.
The independent width-feature gzip is pinned at
`dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f`.
All 10,240 names, side counts and inherited numerical ratios join exactly.
Historical labels retain their existing numerical/corroboration qualifications;
none is a premise of the 25 exact rational-body rejections.

The triangle discovery code uses exact rational supporting-line intersections,
closure predicates, area comparisons, weights and objectives for all 26 bodies.
[verify_triangle_certificate.py](verify_triangle_certificate.py) independently
reconstructs original areas from **all feasible pairwise line intersections**
and a rational monotone convex hull, then checks one feasible weighted word
per body. It reports PASS: 26 rational witnesses and 25 exact rejections. It
neither repeats the optimization nor imports discovery geometry routines.

```sh
# Bound discovery: standard library only; HKO is explicitly numerical.
python3 patterns/upper-tail/triangle_envelope.py \
  --source /path/to/verified-cache/files/random-product.jsonl \
  --width patterns/product-shape/features.csv.gz \
  --out /tmp/msc-math/upper-tail/envelope.json
# Independent rational certificate verification, no optimization.
python3 patterns/upper-tail/verify_triangle_certificate.py \
  --source /path/to/verified-cache/files/random-product.jsonl \
  --certificate patterns/upper-tail/triangle-envelope.json
# Supporting moment probe: NumPy needed.
OPENBLAS_NUM_THREADS=1 python3 patterns/upper-tail/analyze.py \
  --source /path/to/verified-cache/files/random-product.jsonl \
  --width patterns/product-shape/features.csv.gz \
  --out /tmp/msc-math/upper-tail/moments
```

The final envelope pass took 1.46 seconds; its independent verifier about
0.5 seconds. The two moment passes took about four seconds each and matched
all non-timing JSON fields and feature CSV bytes. Exact selected-triangle
capacities were newly computed; **no original-body full capacity producer,
volume producer, cloud job or network operation** was run. Scratch feature CSVs
stay regenerable under `/tmp/msc-math/thesis-owner/2026-10-17/upper-tail/`.

Inspected sources: root entry/rationale, patterns success/feature/product
reports and scripts, exact triangular controls, width account, sparse product
proof and regular tensor report. This is a focused bounded probe, not an
exhaustive repository or literature survey. No external novelty claim is made.

[review: /root/kai_packet]

[review: /root/owner_review]

[review: /root/batch_review]
