# Independent review of the 5×7 tensor certificate, 22 October 2026

Reviewer: `/root/msc_today_scope/continuum_review`. This is a separate bounded
review of Sol's new certificate, beginning at 2026-10-22 13:56:15 UTC, after
the local-cover work. Assigned scope: interval arithmetic, finite enumeration,
exact convex-representation argument and correspondence with the maintained
[matrix reduction](research.tex). No checker, producer or manuscript was
edited by this reviewer.

**Verdict: scoped mathematical acceptance.** The final checker and certificate
establish

\[
 \mathcal T_6(P_5,P_7)\subset\operatorname{int}\mathcal T_4(P_5,P_7)
 \quad\text{in the full four-dimensional matrix space}.                 \tag{1}
\]

This is stronger than the previously established angular projection result.
Combined with the maintained sparse decomposition, affine covariance and
imported Haim–Kislev formula, it yields the independent-affine 5×7 capacity
formula stated below. This review does not establish novelty, human acceptance,
a global systolic optimum in this affine family, or manuscript readiness.

## Reviewed bytes and independent replay

The final input packet was
`/tmp/msc-math/tensor-certificate/20261022/final/`; the coordinator retained
the same packet in [tensor-certificate-20261022/](tensor-certificate-20261022/report.md).
The [checker](tensor-certificate-20261022/check_certificate.py),
[weights](tensor-certificate-20261022/weights.json) and
[result](tensor-certificate-20261022/check-result.json) have these identities:

| Artifact | SHA-256 |
| --- | --- |
| `check_certificate.py` | `a183fc7a8da82859694d54a9fb8ecef870695587291af2c4912a12e31c3ed4dc` |
| `weights.json` | `cd21c3246cb962ae2b4d59d12492856c28f19ea1782eb620c40063c2edf861fc` |
| `check-result.json` | `880e64e72ffeed8fdc290f5c22f67e17912f85ad727acb53c8497cb1b74534d7` |

I read the original checker completely, inspected the final changes, and
independently replayed the final bytes after copying only the checker and
weights into the fresh directory
`/tmp/msc-math/tensor-certificate/20261022/independent-review-replay/`.
The invocation was

```sh
env PYTHONDONTWRITEBYTECODE=1 PYTHONOPTIMIZE=0 python3 \
  /tmp/msc-math/tensor-certificate/20261022/independent-review-replay/check_certificate.py
```

It exited zero with `PASS`; the resulting JSON has exactly the result hash
above. The final checker uses its own directory for inputs and rejects
optimized Python before relying on assertions. These resolve the original
draft's hardcoded temporary path and optimization-mode reproducibility issues.
The following mathematical review, rather than the replay alone, supports
the verdict.

## 1. Rigorous enclosures

The interval class represents `[lo/S,hi/S]`, with exact integer endpoints
and `S=10³⁰`. Rational constants are enclosed by floor and ceiling.
Addition and negation are exact on these endpoints. Multiplication takes
the extrema of all four endpoint products and rounds outward after division
by `S`. Division first proves that the denominator interval excludes zero;
the four exact rational endpoint quotients then give extrema which are again
rounded outward. Python `Fraction` normalizes denominators to be positive,
so the floor/ceiling operations have the required direction even for negative
values. No floating-point operation enters these routines.

The arctangent sums use positive arguments `1/5` and `1/239`; their terms
strictly decrease in magnitude. The partial sum and the partial sum plus
the first omitted term therefore enclose the exact arctangent. The linear
combination of endpoints has the correct signs for Machin's identity
`π=16 atan(1/5)−4 atan(1/239)`. The identity itself follows from the tangent
addition formula and the angle's quadrant, so it is an exact mathematical
input, not a numerical value supplied by a library.

All calls to `trig` in this checker have nonnegative rational multiples of
`π`. Thus `left≤right`, and `(right−left)/2` is the correct argument error.
The polynomial is evaluated exactly at the rational midpoint. For cosine,
the polynomial has degree 80 and Taylor's theorem bounds the error by
`|x|⁸¹/81!`. For sine its degree is 81, so the Taylor bound is
`|x|⁸²/82!`; the checked condition `|x|<82` makes this no larger than the
same `|x|⁸¹/81!` bound used in the code. Adding the argument radius is
valid because both functions have derivative bounded in absolute value
by one on the real line. Final conversion to the fixed-point intervals
rounds outward. Thus every normal, height and subsequent tensor enclosure
contains the intended exact trigonometric value.

The interval Gaussian elimination also has a valid inclusion interpretation.
Follow the ordinary exact Gauss–Jordan operations on the exact matrix `M`.
At each step the program's intervals contain the corresponding exact entries.
Every chosen pivot interval excludes zero, so the exact pivot is nonzero.
Row swaps, divisions and eliminations preserve the inclusion invariant.
Consequently elimination succeeds on the exact matrix, proving it invertible,
and the returned right-hand block encloses its exact inverse. Dependence
between interval entries may widen the enclosure but cannot invalidate it.
An approximate residual test is not being substituted for invertibility.

## 2. Exact closure triples and all 840 tensors

The code uses the source's definitions

\[
 n_i^{(n)}=(\cos(2\pi i/n),\sin(2\pi i/n)),\quad
 h_n=\cos(\pi/n),\quad u_i=n_i^{(n)}/h_n.
\]

For each sorted triple `a,b,c` of dual vertices, write

\[
 r=(\det(b,c),\det(c,a),\det(a,b)),\qquad
 \gamma=r/(r_1+r_2+r_3).
\]

The exact identities

\[
 \det(b,c)a+\det(c,a)b+\det(a,b)c=0,
 \qquad \sum_i\gamma_i=1
\]

show that these are the desired closure weights. The program's test that
the interval sum of steps contains zero is only a consistency check; it
would not by itself prove equality. The determinant identity supplies the
exact equality needed by the argument.

Three distinct dual vertices lie on a circle and are affinely independent,
so the closure distribution on that support is unique. Every positive
three-point distribution is a vertex of the normalized closure polytope.
Conversely, its vertices have support at most three by the source's rank
argument. For odd `n=5,7` there are no opposite normals, hence no two-point
closure vertices; a one-point closure is impossible. Thus enumerating all
index triples is complete.

The checker examines all ten pentagon and thirty-five heptagon triples.
It proves five and fourteen respectively have all weights positive, and
proves each remaining triple has a negative weight. All divisions are
certified away from zero. This agrees with the geometric cyclic-gap
classification: `(1,2,2)` for five and `(1,3,3)` or `(2,2,3)` for seven.

For a fixed pair, a six-block word alternates three individual steps from
each factor. Cyclic invariance permits one designated first-factor step
to be first. There remain two orders of its other steps and six orders
of the second-factor steps. The code enumerates all twelve and checks
the certificate's triple and order labels against this independent
enumeration. It rejects a missing row or any extra row. The total is
`5×14×12=840`.

The target computed for each order is exactly

\[
 T=\tfrac12(x_1y_1^T-x_3y_2^T),                         \tag{2}
\]

with row-major vectorization. This matches equation `eq:six-tensor` of
`research.tex`, including the factor `1/2`. Cyclic invariance here is a
matrix identity: closure makes the cyclic change vanish against every
bilinear pairing, not just against rotations. There is therefore no
projection to two-dimensional angular coefficient space hidden in the
enumeration.

## 3. Generator normalization and exact corrected weights

For odd `n`, the vertices of `(P_n−P_n)^polar` are
`±n_i^(n)/(1+h_n)`: the supports of `P_n` in opposite facet-normal
directions are `h_n` and `1`. Thus the code's seventy matrices

\[
 C_j\in\left\{\pm{n_i^{(5)}(n_k^{(7)})^T\over
                       2(1+h_5)(1+h_7)}:i<5,k<7\right\}                \tag{3}
\]

generate exactly `T₄`, as defined in the source. Separate signs on the
two factors only reproduce the overall signs already included in (3).

Set `\widehat C_j=(vec(C_j),1)` and `\widehat T=(vec(T),1)`. For the
five distinct basis indices `[0,1,18,38,14]`, let `M` have columns
`\widehat C_j`. Its invertibility has been certified as above. Given the
seventy supplied rational numbers `q_j`, define, as exact real quantities,

\[
 \delta=M^{-1}\left(\widehat T-\sum_jq_j\widehat C_j\right).
\]

Add its five components to the five corresponding `q_j` and leave all
other weights alone. This definition proves the augmented identity

\[
 \sum_jw_jC_j=T,\qquad\sum_jw_j=1                      \tag{4}
\]

exactly. The weights need not themselves be rational. The checker encloses
the exact residual, inverse and correction, and proves positivity of each
corrected weight; all unmodified weights are separately certified rational
numbers at least `1/100000`. Its global lower bound is

\[
 w_j\geq
 {1249999796152923685268743\over125000000000000000000000000000}>0.
\]

Thus (4) is an exact convex representation of every target. Small numerical
residuals or LP status are not proof premises. Those numerical computations
only proposed the rational `q_j` which the checker subsequently validated.

Invertibility of the augmented five-column matrix also proves that the
generators affinely span four-dimensional matrix space. A convex combination
with strictly positive weights on all generators belongs to the interior
of their hull: a supporting linear functional at a boundary point would
have to be maximal on every positive-weight generator, contradicting full
affine span. This proves the strict containment in (1), rather than merely
membership in a lower-dimensional affine hull.

## 4. Consequence through the maintained decomposition

The source's tensor covariance sends both families under independent
`G,H∈GL(2,R)` by the invertible map

\[
 T\longmapsto G^{-T}TH^{-1}.
\]

Hence (1) persists under those transformations, including reflections and
shears. Through the maintained four-block/six-block decomposition and
[Haim–Kislev source contract](source-contract.md), it gives

\[
 c_{\mathrm{EHZ}}\bigl((a+GP_5)\times_L(b+HP_7)\bigr)
 ={(1+h_5)(1+h_7)\over
   \max_{0\leq i<5,\,0\leq j<7}
   |(n_i^{(5)})^TG^{-1}H^{-T}n_j^{(7)}|}.               \tag{5}
\]

The numerator and reciprocal factor follow from the `1/2` in (3) and
the source formula `c=1/(2Q_max)`. This is a global capacity formula
inside this specific independent-affine family; it is not a claim about
arbitrary pentagon/heptagon pairs. Nor does (5) alone optimize the systolic
ratio over `G,H`.

I inspected the maintained decomposition, the word/order formulas and
the covariance when checking this implication. I did not re-audit the
published Haim–Kislev proof; its already documented source contract remains
the imported capacity interface. The new contribution validated here is
the full matrix certificate and its applicability to that decomposition.
No material mathematical defect was found in the final reviewed bytes.
The substantive review and independent replay were complete by
2026-10-22 14:01:12 UTC. This establishes no 7×7 or uniform odd-polygon
tensor-containment statement.
