# A covered, first-order-balanced decagon with capacity-preserving ascent

2026-10-04 UTC. Owner: geometry-discovery lead; derivation and exact certificate
by its active-restriction worker. Scope: fixed Q = regular P5, arbitrary planar
partner. Agent-derived, receiving-owner review pending; no thesis promotion or
literature novelty claim. First exact calculation completed at 23:12 UTC.

**Concrete result.** The regular decagon B that is optimal among centrally
symmetric partners is *not* a local maximum among arbitrary partners. Every
facet is covered by its complete active-fitting dual bank, and it satisfies
exact first-order facet-area balance. Nevertheless the active bank has rank
five, below the necessary N−2 = 8. An explicit nontranslation odd support
direction keeps **all ten** template-fitting scales exactly one and decreases
area at quadratic order. This separates the rank test from both facet coverage
and first-order balance on an actual fixed-pentagon partner.

The existing [symmetric-partner lemma](../../../desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md)
already identifies B and its sharp value in the symmetric class. Its
classification is not contradicted: the deformation here leaves that class.
The new evidence is the complete dual-face calculation, exact balance/rank
contrast, and certified arbitrary-partner deformation.

## Exact body, templates and direction

Use the [desk evaluator's convention](../../../desk/pentagon-arbitrary-partner/README.md):
Q has circumradius one, vertex angles π/5 + 2πj/5 and facet-normal angles
2πj/5. Put

\[
 \delta=\pi/5,\quad w=1+\cos\delta,\quad
 R=\frac1{2w},\quad r=R\cos(\delta/2),\quad H=R\tan\delta.
\]

Let e_k = (cos(kδ), sin(kδ)), and n_i = (cos((i+1/2)δ),
sin((i+1/2)δ)). Indices are modulo ten. The body

\[
 B=\{x:n_i\cdot x\le r\ \text{for }i=0,\ldots,9\}
   =\operatorname{conv}\{R e_k:k=0,\ldots,9\}
\]

is exactly (Q−Q)^polar/2 from the existing symmetric lemma.
The ten translation classes have centered representatives

\[
 T_k=\operatorname{conv}\{-R e_k, R e_k, HJ e_k\},
 \qquad k=0,\ldots,9,
\]

where J rotates counterclockwise by π/2. Their side lengths are 2R,
R sec δ and R sec δ, and their angles are δ, δ and 3δ. Their oriented
edges follow Q-normal directions; unit Q-perimeter follows from
cos δ (2R + 2R sec δ) = 2Rw = 1. Reversing the normal-edge order gives
the negative triangle; the five rotations and their negatives give exactly
these ten classes. No template is discarded.

The correspondence to the desk's exact enumeration labels is:

| Normal triple | Edge order (0,1,2): k | Edge order (0,2,1): k |
| --- | ---: | ---: |
| (0,1,3) | 6 | 1 |
| (0,2,3) | 0 | 5 |
| (0,2,4) | 4 | 9 |
| (1,2,4) | 8 | 3 |
| (1,3,4) | 2 | 7 |

The largest normal-closure weight selects the long-edge direction; the cyclic
edge order selects which side contains the apex. The resulting indices are
all ten k. An additional read-only function extraction from `evaluator.py`
numerically checked this interface mapping without importing its calibration
writer; the geometric derivation, not that floating comparison, establishes
the exact ten-class correspondence.

Choose

\[
 v=(1,0,0,0,0,-1,0,0,0,0),\qquad
 h_i(\varepsilon)=r(1+\varepsilon v_i),\qquad
 K_\varepsilon=\{x:n_i\cdot x\le h_i(\varepsilon)\}.
\]

We certify the entire interval |ε| ≤ 1/100. The sign-reversed body is
−K_ε, since v_(i+5) = −v_i. This direction is not a translation:
v_1 = v_2 = 0 would force the translating vector to vanish because n_1,n_2
are independent, contradicting v_0 = 1.

## Every fitting LP, its complete optimal face, and exact balance

For T_k at B, put its long-edge midpoint at zero. Its endpoints are opposite
decagon vertices, and its apex is strictly inside B. Thus the only tight
facets of the three-vertex placement are

\[
 I_k=\{k-1,k,k+4,k+5\}.
\]

These consist of two opposite normal pairs. Complementarity with this
optimal primal placement forces *every* optimal dual to have support in I_k.
The two normal directions are independent, so the closure equation forces
equal coefficients within each opposite pair. If s_ik = max_(x∈T_k) n_i·x,
the width of T_k in either of these pair directions is 2r. Therefore the
complete optimal dual face is the line segment between

\[
 d_{k-1}=\frac{e_{k-1}^{\rm coord}+e_{k+4}^{\rm coord}}{2r},
 \qquad
 d_k=\frac{e_k^{\rm coord}+e_{k+5}^{\rm coord}}{2r},
\]

where e_i^coord denotes the coordinate vector in support-number space,
not the planar vector e_k above. Opposite-pair labels are understood modulo
five, so the union of the ten optimal faces has exactly five distinct
vertices d_0,…,d_4. Each has nonnegative coefficients, N d_i = 0,
s_k·d_i = 1 for its corresponding templates, and h·d_i = 1. These primal
and dual witnesses establish r_k(B) = 1 for *every* k.

There is no hidden optimal-dual branch: the positive primal slacks outside
I_k exclude any positive coefficient there by complementarity. Closure and
normalization describe the whole remaining face explicitly. A single solver
dual per template would fail to establish this completeness.

The five pair vectors have disjoint two-coordinate supports and are linearly
independent. Their span has rank five. N = 10 and ker[n_0 … n_9] has dimension
eight, so the active span misses three required dimensions. The normalized
active convex hull has affine dimension four, whereas the complete closure
normalization plane has dimension seven; its target cannot be interior there.

Yet every facet is covered. At B all edge lengths are ℓ = 2R sin(δ/2),
and A_0 = 5rℓ. Thus

\[
 \frac{S_i}{2A_0}=\frac1{10r}
   =\left(\frac15\sum_{a=0}^4 d_a\right)_i.
\]

This is the exact first-order balance condition, with positive mixture
coefficients. Moreover d_a·v = 0 for all five a and S·v = 0. The control
therefore reaches the second-order curvature mechanism; it is not another
uncovered-facet example.

## Explicit all-ten placements and uniform slack bounds

Let z_k(ε) be the intersection of facets k−1 and k of K_ε. Since opposite
supports sum to 2r, solving the two intersection equations gives

\[
 z_k(\varepsilon)=R e_k+\varepsilon r q_k,\quad
 z_{k+5}(\varepsilon)=-R e_k+\varepsilon r q_k,
\]

where q_k solves n_(k−1)·q_k = v_(k−1), n_k·q_k = v_k. At most one
right-hand side is nonzero. Hence ||q_k|| is zero or 1/sin δ.
Translate T_k by m_k(ε) = εr q_k. Both long-edge endpoints become exactly
the two opposite vertices z_k and z_(k+5), so the edge fits without changing
scale. The apex becomes m_k + HJ e_k.

The polygon's fixed-fan edge lengths satisfy

\[
 \ell_i(h)=\frac{h_{i-1}+h_{i+1}-2\cos\delta\,h_i}{\sin\delta}.
\]

Only six lengths change; their slopes in t = rε have magnitudes at most
max(2 cot δ, csc δ) < 4. Since ℓ/r = 2tan(δ/2) > 3/5,
every edge remains longer than (14/25)r for |ε| ≤ 1/100. Thus all ten
inequalities remain genuine and the labelled chamber persists. The exact
certificate additionally checks every vertex against every facet.

For the apex, use the certified elementary bounds

\[
 \cos(\delta/2)>9/10,\qquad \tan\delta<3/4,
 \qquad \sin\delta>1/2.
\]

Consequently

\[
 \min_i h_i\ge\frac9{10}R(1-|\varepsilon|),\qquad
 \|m_k+HJ e_k\|\le R(3/4+2|\varepsilon|).
\]

Their difference is at least

\[
 R(3/20-29|\varepsilon|/10)\ge\frac{121}{1000}R>0.
\]

Thus every apex has at least this much slack in *every* facet inequality,
uniformly over all ten templates and the whole stated interval. Convexity
puts each entire translated triangle in K_ε.

Conversely the same two pair dual vertices remain feasible and have unchanged
objective h(ε)·d_a = 1. Hence **each** fitting scale is at most one. Together
with the explicit primal placements this proves

\[
 r_0(K_\varepsilon)=\cdots=r_9(K_\varepsilon)=1,
 \qquad c_{\rm EHZ}(K_\varepsilon\times_L Q)=1.
\]

The capacity equality uses [Balitskiy–Mitrofanov–Polyanskii, Theorem 3.3 and
Theorem 2.5](https://arxiv.org/html/2603.12495v2), with the unit-Q-perimeter
normal templates and one-factor scaling. It is not inferred from one
bottleneck fit. Every template has a primal containment and matching dual
upper witness; complete faces remain the same on this interval.

## Exact area and the strictly increasing ratio

For this normal fan the area quadratic is

\[
 A(h)=\csc\delta\sum_{i=0}^9 h_i h_{i+1}
       -\cot\delta\sum_{i=0}^9 h_i^2.
\]

Along v, the adjacent-coordinate quadratic term vanishes, Σv_i = 0 and
Σv_i² = 2. Therefore

\[
 A_\varepsilon=A_0-2\cot\delta\,r^2\varepsilon^2
  =A_0\left(1-\frac{2+\sqrt5}{5}\varepsilon^2\right).
\]

The first derivative is zero and the second derivative is strictly negative.
Using the exact symmetric-class value κ = (10+6√5)/25 gives

\[
 \operatorname{sys}(K_\varepsilon\times_L Q)
  =\frac{\kappa}{1-(2+\sqrt5)\varepsilon^2/5}>\kappa
 \quad(0<|\varepsilon|\le1/100).
\]

At ε = 1/100 the certificate gives the following approximate values for
readability; all equalities and signs use exact algebraic arithmetic:

| Quantity | ε = 0 | ε = 1/100 |
| --- | ---: | ---: |
| Every one of the ten fitting scales | 1 | 1 |
| Area | 0.224513988289792 | 0.224494967159466 |
| Ratio | 0.936656314599949 | 0.936735676119969 |
| Complete active dual rank | 5 | 5 |

This remains below one. No counterexample to Viterbo or improved global
fixed-pentagon bound is asserted.

## Exact executable evidence

Owned files:

- [verify_decagon_separator.py](verify_decagon_separator.py): standard-library
  executable, no LP solver or floating-point predicate.
- [decagon-separator-certificate.json](decagon-separator-certificate.json): all
  ten placements, their 300 facet slacks, 20 dual-vertex witnesses, all 100
  polygon vertex/facet slacks, rank/balance and area arithmetic.

Run from the repository root:

```sh
python3 docs/empirical-viterbo-design/night-20261005/geometry-discovery/fixed-q-rank/verify_decagon_separator.py
```

It writes only its sibling owned certificate. It ran successfully in about
0.21 seconds on this workstation. The field is Q(a), a = cos(π/10), with
16a⁴−20a²+5 = 0 and a rational isolating interval
(951056516295153/10¹⁵, 951056516295154/10¹⁵). Fractions reduce exact field
elements to four coefficients. Nonzero signs are certified by rational
interval evaluation; identically zero elements are checked coefficientwise.
The root interval is checked against the polynomial, whose derivative is
positive on [9/10,1]. Floating output is explanatory only.

The script reconstructs all canonical template representatives and verifies
the same complete ten-class geometry documented by `templates()` in the desk
evaluator. It does not rerun that evaluator's calibration writer or any
retained population producer. The interval proof above supplies uniformity;
the retained arithmetic verifies ε = 1/100 explicitly. Reflection relates
the opposite endpoint, and every placement/support slack is affine in ε.

## Scientific implication and falsifier

This control rules out interpreting covered facets or a positive first-order
balanced mixture as an extremality certificate. At B those diagnostics pass
exactly, while the omitted directions of the active span have strictly
negative area curvature at unchanged capacity. It also separates symmetric
optimality from arbitrary-partner optimality without numerical optimization.

A consequential adverse check would be a missing optimal dual outside the
displayed five pair vectors: it would invalidate the rank claim. The exact
primal slacks and complementary four-contact-face derivation rule this out.
A failed apex containment for any of the ten templates would invalidate the
capacity-constant deformation; the uniform positive bound and complete
certificate check every one.

The conditional N≤12 result for ten *uniquely* optimized templates is not used
here. Every template has two dual vertices, so this control explicitly exercises
nonuniqueness. It supplies neither a global side-count restriction nor the
unresolved arbitrary-partner HKO inequality. The required remaining step is
receiving-owner proof/certificate review before integration into the night's
result synthesis.
