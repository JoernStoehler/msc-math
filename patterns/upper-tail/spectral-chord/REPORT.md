# Principal spectral closure directions lose reference continuity

17 October 2026. A new target-free selector uses the leading singular directions
of the signed bilinear HK payoff to move both factors' closure weights along
feasible chords. Its ideal mathematical form is invariant under the product
symmetries and exact at HKO, but **is discontinuous there**: a one-coordinate
normal deformation has a strictly positive limiting capacity-ceiling error.
The cause is splitting a repeated leading singular value and discarding the
other mode. This is an obstruction to principal-mode compression, rather than
a claim that every spectral feature or every nonsmooth invariant fails.

A complete frozen pass over 1,024 retained pentagon products supplies a useful
complementary feasible bound: it improves the existing width–triangle minimum
on 121 bodies, including an exact capacity-bound reduction exceeding 10%. An
exploratory combination improves the upper-decile median/p90 ratio errors from
3.88%/14.99% to 2.72%/9.07%. That combination was formed after scoring; it is not
a prospective validation. The frozen numerical selector has a fixed singular-value
clustering tolerance and rationally rounded rays. A separate exact above-one
example verifies its finite output misses capacity.
No selected thesis source or production capacity implementation changes. No
speed advantage over the existing sparse exact product evaluator is established;
the value here is the signed-mode obstruction, rather than a faster algorithm.

## End state, alternatives and frozen selector

The desired end state was a small signed geometric mechanism explaining broad
upper tails while retaining a plausible sharp bound implication. Exact
feasibility, the positive reference, nonregular above-one products and retained
random tails cannot be skipped: each eliminates a different shortcut. The
[frozen plan](PLAN.md) records the options and later root-authorized broad pass.

| Option | Possible value | What a misleading rubric rewards | Selection |
|---|---|---|---|
| Spectral objective upper relaxation | Compress signed interactions | Upper Q gives a lower capacity bound, the wrong direction for maximality | Reject |
| Canonical surface-area closure | Direct mixed-area feasible bound | Repeats the fixed-closure HKO obstruction | Already rejected; no new scan |
| Conditional transport closure | Polyhedral compatibility | Repeats the existing conditional LP route | No new variant |
| Principal spectral feasible chords | Adapt both factor closures to signed phase | HKO degeneracy can hide loss of transverse modes | Frozen and tested |

For centroid-normalized facet covectors $a_i,b_j$, put

$$C_A=\{\alpha\ge0:\mathbf1^T\alpha=1,\ A^T\alpha=0\},\qquad
C_B=\{\gamma\ge0:\mathbf1^T\gamma=1,\ B^T\gamma=0\}.$$

Let $\alpha_0,\gamma_0$ be their cone-volume closures: each weight is the area
of the triangle joining the centroid to that facet divided by factor area.
Let $P_A,P_B$ be Euclidean orthogonal projectors onto their closure tangent
spaces, namely the kernels of $[\mathbf1;A^T]$ and $[\mathbf1;B^T]$.
For each of the two existing angular words define

$$S_{ij}=\tfrac14\varepsilon_{ij}a_i\cdot b_j,\qquad M=P_A S P_B.$$

The signed word matrices are an HK component imported from the sparse-product
and two-shadow reductions. The selector does not enumerate closure vertices,
support pairs, or new exact capacities.

Project every A coordinate axis into the **entire leading left singular
subspace** of $M$. Pair each nonzero ray $u$ with $M^Tu$. Also project every B
coordinate axis into the leading right subspace and pair it with $Mv$.
For each paired ray, intersect its lines through $\alpha_0,\gamma_0$ with the
closure polytopes and evaluate all four endpoint pairs. Include the canonical
centers and take the largest actual HK value across both words. Its reciprocal
$C_{\rm sp}=1/(2Q_{\rm sp})$ is a capacity upper bound because every candidate
is feasible. Thus $C_{\rm sp}^2/(2\operatorname{area}A\operatorname{area}B)$
is a ratio upper bound, with no asserted global HKO ceiling.

Using all projected coordinate axes avoids choosing an arbitrary basis inside
a multiple singular subspace. The ideal candidate set is equivariant under
facet permutations; including both factor roles makes it equivariant under
factor interchange. Translations are removed by centroids. Under a paired
linear product symplectic map, closure weights, their Euclidean tangent
projectors and cross-factor dot products remain unchanged. The angular words
are transported or interchanged/reversed by the common planar linear map;
reversal/reflection conventions are covered by the two signed variants.
Positive conformal changes multiply S by a positive scalar and leave the
selected rays unchanged. Hence the ideal ratio bound is a product invariant,
not an invariant of arbitrary four-dimensional presentations after forgetting
the splitting.

The frozen implementation includes singular values within relative $10^{-10}$ of
the largest, rounds unit ray components to multiples of $2^{-30}$, applies the
exact rational closure projector, and uses exact rational endpoints. This
certifies feasibility, but is only an approximation to the ideal invariant.
The known-counterexample control agrees to displayed precision after arbitrary
facet permutations, factor swap, translations, and positive/negative paired
GL maps. This finite check does not prove exact invariance of rounded rays.

## Exact directional discontinuity of the ideal selector

Write $s=\sqrt5$ and use the existing transformed HKO reference

$$A_0=\left[(1,0),((s-1)/4,1),(-(s+1)/4,(s-1)/2),
 (-(s+1)/4,-(s-1)/2),((s-1)/4,-1)\right],\quad B_0=JA_0.$$

Its imported exact capacity is five. Replace only the first A normal by
$(1,t)$, for $t>0$, leaving B unchanged, and recenter both factors at their
area centroids. All five facets persist for sufficiently small t. Let the two
projected payoffs be $M_\pm(t)$ and $H_\pm(t)=M_\pm(t)M_\pm(t)^T$.
At zero the closure tangent has dimension two and

$$H_+(0)=\frac{7-3s}{40}P_A(0),\qquad H_-(0)=\frac1{20}P_A(0).$$

For the positive shadow the reference rays are the five A directions and
their negatives, hence no rays coincide. In the negative shadow the four
unchanged matched directions stay tied identically; their cross payoffs vanish
identically, so tie order has no effect. The moved direction $(1,t)$ splits
counterclockwise from its fixed matched ray at first order. Centroid
normalization multiplies each normal by a positive scalar and changes none of
these angular facts. A change at the angular cut only changes a cyclic word
representative, whose Q is unchanged by closure. The right-hand word matrices
therefore have the value/derivative expansions used below.

Thus the two nonzero singular values tie in each word. Exact evaluation of all
projected-coordinate chord candidates gives $Q_{\rm sp}(0)=1/10$, hence
$C_{\rm sp}(0)=5$.

The following calculation uses the two columns numbered 3 and 4 (zero-based)
of $P_A(0)$ as a basis $E$ of that tangent plane, with Gram matrix $G=E^TE$.
The first-order operators
$K_\pm=G^{-1}E^T H_\pm'(0)E$ are

$$K_+=\begin{pmatrix}(-31+13s)/300&(-31+13s)/300\\
(-7+2s)/150&(31-13s)/300\end{pmatrix},$$

$$K_-=\begin{pmatrix}(4-s)/300&(-11-s)/300\\
(-2+s)/150&(-4-5s)/300\end{pmatrix}.$$

Their positive discriminants are

$$D_+=\frac19-\frac{278s}{5625}>0,\qquad
D_-=\frac7{2250}-\frac{s}{11250}>0.$$

These are Gram-self-adjoint operators even though the displayed bases are not
orthonormal. Distinct eigenvalues imply a simple leading singular direction
for every sufficiently small positive t. Its limiting line is the eigenline
of $K_\pm$ for $(\operatorname{tr}K_\pm+\sqrt{D_\pm})/2$.
For a simple leading singular value all the nonzero projected-coordinate
seeds give this same line; A- and B-side seeds therefore yield the same four
corner pairs. Chord endpoints depend continuously on a nonzero zero-sum ray,
including when one limiting component vanishes. The canonical center cannot
exceed their maximum, since it is a convex combination of the four corners.

Exact endpoint evaluation in $\mathbb Q(s,\sqrt{D_\pm})$ gives limiting Q
values

$$q_+=-\frac1{10}+\frac{3s}{50}+\frac{30+21s}{29}\sqrt{D_+},$$

$$q_-=-\frac{21}{40}-\frac{47s}{200}+
 \left(\frac{45}4+\frac{9s}2\right)\sqrt{D_-}.$$

Consequently

$$\lim_{t\downarrow0}C_{\rm sp}(t)
=\min\{1/(2q_+),1/(2q_-)\}
=5.0166580570\ldots>5.015>5=C_{\rm sp}(0).$$

The two separate limits are $5.0429651607\ldots$ and
$5.0166580570\ldots$. True capacity tends five by containment and homogeneity.
Thus exact HKO calibration alone conceals an order-one directional error of
this principal-mode architecture; it has no faithful local expansion there.
The constant size is incidental. Keeping all spectral modes or introducing a
cluster rule changes the architecture and is not tested as a repair here.

[limit.py](limit.py) propagates exact value/derivative pairs through polygon
vertices, centroid normalization, tangent projection and the signed objective.
It verifies both double eigenvalues, the restricted operators, discriminant
signs, exact HKO recovery, all limiting corners and the strict bound above.
[limit.json](limit.json) retains matrices, fields and witnesses. The proof uses
standard simple-eigenline perturbation of the restricted two-dimensional
operator; no capacity oracle enters it. A separate read-only agent independently checked the field signs, restricted
eigen-equations, endpoint identities and the displayed limiting Q formulas;
this is agent review, without human proof acceptance.

A fixed numerical clustering tolerance changes the very-small-scale limit:
when the gap eventually falls below that tolerance, the implementation includes
both modes again. Therefore the theorem concerns the ideal exact-leading-space
selector. The observed finite implementation errors and transition scales are
reported separately; a literal discontinuity at t=0 is not claimed for it.

## Exact finite above-one miss of the rounded implementation

Replace s in the reference by $223606797749979/10^{14}$ and set the first A
normal to $(1,10^{-6})$. The retained numerical eigenspace seeds, rounded and
projected exactly as frozen, give

$$C_{\rm sp}=5.0166575887\ldots.$$

Every reference primal coordinate lies strictly between -2 and 2. The l1 error
of each rational facet normal from its exact reference is less than
$\delta/2$ for $\delta=10^{-5}$. The reference polar therefore contains the l1
ball of radius 1/2, so support functions give

$$(1-\delta)K_H^\circ\subset K^\circ\subset(1+\delta)K_H^\circ.$$

After polarity, capacity monotonicity and degree-two homogeneity give

$$\frac5{(1+\delta)^2}\le c(K)\le\frac5{(1-\delta)^2},$$

namely $[4.9999000015\ldots,5.0001000015\ldots]$. Exact factor areas imply
$\operatorname{sys}(K)>1.0471717080\ldots>1$. In particular the selected
capacity ceiling exceeds true capacity by more than 0.279944%.
The selected ratio ceiling is $1.0542028399\ldots>21/20>\rho_H$,
so the proposed geometric inequality $C_{\rm sp}^2/(2V)\le\rho_H$ is
false. This does not refute global HKO maximality for true capacity. This finite
certificate does not rely on historical numerical labels or a new capacity search.

[verify.py](verify.py) independently uses only standard-library rational
arithmetic and the existing ordered quadratic field. It reconstructs all 80
rounded-ray chord corners, checks 82 direct four-dimensional HK values,
reference vertex bounds, facet errors, volume and the capacity bracket.
[verification.json](verification.json) records PASS. This independently
certifies the supplied finite rounded-ray output and its miss; it does not
certify the numerical eigenspace computation itself. [obstruction.json](obstruction.json)
retains all rounded seeds and exact selected witnesses.

## Retrospective predictive controls and broad comparison

The known exact transfer counterexample `random_5x5_s0_92` gives spectral ratio
ceiling 0.922899 against true ratio 0.838533: a roughly 4.91% capacity miss.
All twelve already retained nonregular local controls (six directions at two
scales) remain included, regardless of their ratio; paired rows are correlated. These labels were exposed before method selection,
so they supply retrospective diagnostics, not prospective successes.

The local twelve have median/p90 ratio-ceiling errors 3.46%/7.01%; the six
above-one controls have 1.99%/3.19%. Width is stronger on all twelve. When the
ceilings are used as point predictions, all six below-one controls are predicted
above one; as mathematical upper bounds those values are simply inconclusive.
This does not supply a faithful local predictor.

The complete retained 5x5 bucket contains 1,024 bodies. All spectral geometry
predictions were completed and hashed before target joining. The raw frozen
prediction hash is `81e6836510348e1870c850a0aa00e04d61d27248e7595e4de8e3c9d813834d3e`.
Its historical ratios range from approximately 0.00149 to 0.84380; these are
random-distribution upper tails, distinct from the constructed above-one panel.
The spectral rule and its combination $\min(C_2,C_{\rm sp})$ were frozen
before this pass. A later root-requested benchmark join adds the existing
minimum of width and minimum-area supporting triangles. The minimum of all
three is **exploratory and post-score**; no selector was tuned or rerun.

| Rule, retained upper decile (103 bodies) | Median / p90 ratio error | Within-tail Spearman | Decile recall |
|---|---:|---:|---:|
| Existing width ceiling | 8.66% / 35.18% | 0.512 | 0.563 |
| Existing minimum of width and triangles | 3.88% / 14.99% | 0.746 | 0.825 |
| Frozen spectral ceiling | 9.60% / 20.61% | 0.614 | 0.777 |
| Frozen minimum of width and spectral | 5.65% / 13.15% | 0.804 | 0.845 |
| Exploratory minimum of width, triangles and spectral | 2.72% / 9.07% | 0.843 | 0.893 |
| Existing canonical reciprocal | 12.09% / 24.46% | −0.008 | 0.282 |

The frozen width–spectral combination has a worse median error than the existing
width–triangle minimum. The complementarity appears when their minima are
combined, rather than through standalone spectral superiority. At the top 5%,
the old and exploratory three-way median/p90 errors are 4.41%/15.43% and
3.91%/8.20%; at the top 2% they are 7.25%/16.26% and 4.72%/11.83%.
The top-2% three-way recall is 0.762, compared with 0.619 for width–triangle.
These are retrospective matched comparisons without a population guarantee.
[broad-scores.json](broad-scores.json) and [incumbent-scores.json](incumbent-scores.json)
retain all model rows and metrics. This comparison concerns the named
width–triangle baseline; other retained selectors have separate evidence.

Spectral improves width alone on 248/1,024 bodies, but only **121/1,024** improve
the stronger width–triangle baseline. Of those, 22 have a capacity-bound decrease
exceeding 5%. In the historical upper decile, 29/103 improve the stronger
baseline and three have decreases exceeding 5%. The largest exact improvement
against that baseline is 10.2607%, described below. This is a change in a
feasible upper bound, not a new capacity value or improvement of the best known
systolic ratio.

### A coupled signed competitor missed by independent triangle area

For `random_5x5_s1_229`, the independently selected minimum-area supporting
triangles use A facets $\{1,2,4\}$ and B facets $\{1,3,4\}$ (zero-based).
Their retained exact capacity bound is $4.9417602445\ldots$; width gives the
slightly stronger $4.9380925057\ldots$. The new positive-shadow chord instead
uses four positive weights per factor:

$$\alpha\approx(0.307146,0.065085,0.365941,0,0.261828),$$
$$\gamma\approx(0,0.222834,0.184896,0.222721,0.369549).$$

The word is $[0,1,9,2,5,3,6,7,4,8]$, with B labels offset by five.
Its exact feasible bound is $4.4314115968\ldots$. [cut.py](cut.py) verifies
rationally that this decreases **both** the triangle bound and the old minimum
by more than 10%; [cut.json](cut.json) retains the exact fractions.

The leading positive-shadow projected singular values are approximately 0.174280
and 0.010383. The selected coupled chord brings in A facet 0 and B facet 2,
which are absent from the two independently chosen area-minimizing triangles.
Their signed cross-factor interactions enter M and hence the simultaneous chord
directions. An independent factor-area criterion does not see that interaction.
No claim that these four-support closures are globally optimal follows; the
existing sparse theorem still supplies some global pair with at most three
positive weights per factor. The material finding is a concrete additional
feasible competitor supplied without support-pair enumeration or conditional LP.

All broad best witnesses are retained in original source-normal coordinates in
[broad-best-witnesses.jsonl.gz](broad-best-witnesses.jsonl.gz), one per body;
[broad-frozen-summary.json](broad-frozen-summary.json) binds them to the frozen
raw output and contains all 1,024 scalar predictions. The 126.44 MB raw output
remains in scratch; the selected witness gzip is 9.31 MB. Its exact closures use
integer numerators with a common denominator per factor. The independent
standard-library [verify_broad.py](verify_broad.py) checks all 2,048 original
closures, all 1,024 HK objectives and all 1,024 exact area products in 4.35 seconds;
[broad-verification.json](broad-verification.json) records PASS and zero
floating-summary discrepancy. It does not independently certify numerical
singular-space selection, capacity optimality, metrics or historical targets.

Historical capacity and ratio labels retain their numerical status. The new
exact feasible bounds and volumes concern the specified dyadic input bodies;
they do not retrospectively certify the historical oracle labels. The broad
pass took 754.13 seconds elapsed, with observed CPU time approximately equal to
wall time. No further scan or mode repair followed these results.

## Verification, sources and retention

All new files began under `/tmp/msc-math/upper-tail-next/20261017/duality/`.
The root owns review, integration and commit. No production capacity search,
original producer rerun, fitting, tuning, cloud job, network, reset consumption,
thesis edit, push, merge or external message occurred. NumPy ran from an offline
uv environment using its existing cached package; no download occurred.
Account spending and quota were not queried and remain unknown.

Inspected sources were this branch's repository/research/product/empirical and
collaboration guides; the fixed-closure, conditional-coupling,
signed-prefix-response, LP-seeded-response, contact-obstruction and
mixed-area-order packets; and their focused sparse-product conventions. This
is a bounded local source comparison, not an exhaustive literature search or
external novelty claim. The exact HKO capacity, sparse HK reduction and
capacity monotonicity/homogeneity are imported facts with their source limits.

Reproduce the independent finite certificate and exact ideal-limit calculation:

```sh
python3 -B patterns/upper-tail/spectral-chord/verify.py --repo "$PWD"
python3 -B patterns/upper-tail/spectral-chord/limit.py
```

These commands replace only their named packet receipts. To audit the broad
witnesses, supply the locally materialized registered `random-product.jsonl`
whose SHA256 is `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`:

```sh
python3 -B patterns/upper-tail/spectral-chord/verify_broad.py --source "$SOURCE"
python3 -B patterns/upper-tail/spectral-chord/cut.py --source "$SOURCE" --repo "$PWD"
```

`probe.py` reproduces the full geometry-only raw predictions in a fresh scratch
directory containing the plan; `score.py` performs the separately hashed target
join, and `compare_incumbent.py` adds the explicitly post-score benchmark join.
`compact.py` packages the unchanged selected witnesses. The retained manifest
binds selected packet bytes; hashes in the receipts bind the registered source,
frozen raw predictions and imported field implementation.

[review: /root/spectral_review]
