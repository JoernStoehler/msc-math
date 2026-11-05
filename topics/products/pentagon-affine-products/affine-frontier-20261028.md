# Independent affine width formula for every odd regular pair

28 October 2026. New analytic proof by `/root/affine_assessment`, with the
extreme-triangle scalar estimate independently developed by
`/root/affine_assessment/uniform_norm_route`. **Draft: assembled mathematical
review pending.** No human acceptance, novelty determination, or manuscript
selection is asserted. The argument below is independent of the exploratory
floating-point probes. Its only finite tensor-containment input is the already
reviewed 5×7 certificate; 5×5 uses the existing analytic identities.

## The statement and its existing interfaces

For odd integers `n,m>=5`, put

\[
 n_i^{(j)}=(\cos(2\pi i/j),\sin(2\pi i/j)),\quad
 h_j=\cos(\pi/j),\quad s_j=(1+h_j)^{-1},\quad
 D_j=\operatorname{conv}\{\pm n_i^{(j)}\}.
\]

Let `P_j` have these outward unit facet normals, common facet height `h_j`,
and circumradius one. Then

\[
 \boxed{\mathcal T_6(P_n,P_m)\subseteq\mathcal T_4(P_n,P_m).}       \tag{1}
\]

Containment is strict in the four-dimensional interior unless `n=m=5`.
Consequently, for every invertible real `G,H` and every translations `a,b`,

\[
 \boxed{c_{\mathrm{EHZ}}((a+GP_n)\times_L(b+HP_m))
 =\frac{(1+h_n)(1+h_m)}
 {\max_{i<n,j<m}|(n_i^{(n)})^TG^{-1}H^{-T}n_j^{(m)}|}.}           \tag{2}
\]

Here the maintained [research note](research.tex), Sections 2–3, supplies the
sparse reduction, four-/six-block decomposition, affine covariance, and
capacity implication. The relevant definitions are

\[
 W_j=(P_j-P_j)^\circ=s_jD_j,\qquad
 \mathcal T_4=\frac{s_ns_m}{2}\mathcal D_{nm},\qquad
 \mathcal D_{nm}=\operatorname{conv}\{\pm n_i^{(n)}(n_k^{(m)})^T\},
\]
\[
 T=\frac12(x_1y_1^T-x_3y_2^T)\in\mathcal T_6,
 \qquad \sum_i x_i=\sum_i y_i=0.
\]

The steps of each genuine closure triangle have the form
`x_i=gamma_i n_{r_i}^{(n)}/h_n`, where `gamma_i>0` and `sum gamma_i=1`;
thus their Euclidean perimeter is `R_n=1/h_n`. The analogous statements
hold for `y`. All twelve cyclic alternating orders must be covered, and are.
The [source contract](source-contract.md) remains the imported capacity
interface; this note proves its new uniform tensor-containment premise.

The method refines the triangle placements in the
[odd-regular symmetric-partner proof](../pentagon-arbitrary-partner/odd-regular-symmetric-partner.md).
The improvement is to retain the polygonal gauge in one factor and use the
strictly positive shortest partner side. This controls the full tensor,
where the earlier [circle-deficit argument](odd-regular-angular-width.md)
controlled only its angular projection.

## 1. A polygonal translation-deficit bound

Write `||z||_D` for the Minkowski gauge of the centrally symmetric body `D`.
For any translation vector `c`, put

\[
 p_1=x_1,\quad p_2=-x_3,\quad p_3=0,\qquad
 r_i=\|p_i-c\|_{D_n},\qquad \ell_i=|y_i|.
\]

Closure of `y` gives the exact matrix identity

\[
 2T=\sum_{i=1}^3(p_i-c)y_i^T.                                \tag{3}
\]

Each `y_i/ell_i` is a normal of the second polygon, while
`(p_i-c)/r_i` belongs to `D_n` whenever `r_i>0`. Expanding a convex
representation in `D_n` proves

\[
 T\in\frac12\left(\sum_i r_i\ell_i\right)\mathcal D_{nm}.     \tag{4}
\]

Zero terms can simply be omitted. In particular,

\[
 \sum_i r_i\ell_i<s_ns_m
 \quad\Longrightarrow\quad T\in\operatorname{int}\mathcal T_4.\tag{5}
\]

For strictness, `mathcal D_nm` is centrally symmetric and spans all 2×2
matrices because each set of polygon normals spans the plane. It therefore
contains zero in its four-dimensional interior; multiplying it by any
nonnegative scalar smaller than one puts it inside its interior.

The three points `p_i` have side lengths `|x_1|,|x_2|,|x_3|`.
The argument is independent of which partner side is paired to which point,
so every alternating order is included. We may assume `n<=m`: exchanging
factors transposes the matrix families (up to a harmless sign/order change).
Set

\[
 b=\pi/n,\quad a=\pi/m,\quad h=\cos b,\quad
 q=\tan(b/2),\quad v=\tan(a/2)\le q,\quad c_n=\cos(b/2).
\]

The body `D_n` contains the Euclidean disk of radius `c_n`, hence
`||z||_Dn<=|z|/c_n`. Also

\[
 \frac{R_m}{2s_m}=\frac1{1-v^2}.                             \tag{6}
\]

A useful consequence is that any circle containing the point triangle with
radius `r` yields strict containment if

\[
 2r<s_nc_n(1-v^2).                                         \tag{7}
\]

Indeed (4) then has `sum r_i ell_i<=rR_m/c_n<s_ns_m`.

## 2. The triangle facts used below

Every interior angle of a genuine regular `j`-gon closure triangle is a
positive odd multiple of `pi/j`. To see this, order its three edge directions
cyclically. Their positive index gaps `k_1,k_2,k_3` sum to `j` and are less
than `j/2`, because the origin lies strictly inside their positive closure.
The interior angles are `pi-2pi k_i/j`. No angle is a right angle for odd `j`.
Changing the traversal only changes the placement or reflection of the same
side-length triangle.

For any triangle with minimum angle at least `a`, the shortest side fraction
`p=min ell_i/sum ell_i` satisfies

\[
 p\ge\frac{\sin(a/2)}{1+\sin(a/2)}
   =v(\sqrt{1+v^2}-v)\ge v(1-v).                            \tag{8}
\]

For example, for its opposite angle `A>=a`, the sine rule gives

\[
 \frac{\sin A}{\sin A+\sin B+\sin C}
 =\frac{\sin(A/2)}{\sin(A/2)+\cos((B-C)/2)}
 \ge\frac{\sin(a/2)}{1+\sin(a/2)}.
\]

For an acute triangle with perimeter `R_n` and angles at least `b`, its
circumradius obeys

\[
 R_c\le\frac1{2h(1+h+\sin b)}.                            \tag{9}
\]

The sine rule reduces this to `sum sin A>=1+cos b+sin b`.
Concavity of sine on the compact angle polygon `b<=A,B,C<=pi/2`,
`A+B+C=pi`, makes its minimum occur at a permutation of
`(b,pi/2-b,pi/2)`. These facts also appear in the existing angular proof.

## 3. The extreme obtuse triangle

First suppose the point triangle has angles `(b,b,pi-2b)`. Its longest side
has length `L=s_n`: the other two sides have length `L/(2h)` and perimeter
normalization gives `1=hL(1+1/h)=L(1+h)`.
Center that longest side at the origin. Since it is parallel to a polygon
normal, its endpoints have `D_n`-gauge exactly `s_n/2`. The apex has
Euclidean distance `(s_n/2)tan b`, hence gauge at most `(s_n/2)g_n`, where

\[
 g_n=\frac{\tan b}{\cos(b/2)}.
\]

Whenever `g_n<1`, the side paired to the apex has length at least `pR_m`,
regardless of order. Thus (4) is controlled by

\[
 \frac{\sum_i r_i\ell_i}{s_ns_m}
 \le\frac{1-p(1-g_n)}{1-v^2}.                              \tag{10}
\]

By (8), this is strictly below one whenever

\[
 g_n<\frac{1-2v}{1-v}.                                    \tag{11}
\]

For `n>=7`, the elementary bounds retained in the angular proof and research
note give `v<23/100`, `tan(pi/7)<49/100`, and `cos(pi/14)>9/10`.
Monotonicity then gives

\[
 g_n\le g_7<49/90<54/77<\frac{1-2v}{1-v}.                  \tag{12}
\]

The cosine bound follows directly from `cos x>=1-x²/2` and `pi<22/7`;
`tan(pi/14)<23/100` is proved in Section 4 of the angular note, and
`tan(pi/7)<49/100` in the research note's rational heptagon bounds.

For `n=5,m>=9`, `v<=tan(pi/18)<9/50`. The last inequality follows from
`pi/18<11/63` and `tan x<=x/(1-x²/2)`. Moreover

\[
 g_5^2=14-6\sqrt5<49/81,
 \qquad
 g_5<7/9<32/41<\frac{1-2v}{1-v}.                           \tag{13}
\]

One exact check is `sqrt5>2233/1000`, which gives
`14-6sqrt5<301/500<49/81`. Therefore all these extreme triangles satisfy
strict full tensor containment. The remaining pairs `5×5` and `5×7` will
use the existing proofs in Section 6 below.

## 4. Every other obtuse triangle

Let its obtuse angle be `alpha` and its other angles `beta,gamma`. Put
`t=(beta+gamma)/2<pi/4`, `d=|beta-gamma|/2<=t-b`.
Since `beta,gamma` are positive odd multiples of `b`, `t` is a positive
integer multiple of `b`. The case `t=b` was Section 3; otherwise `t>=2b`,
which forces `b<pi/8` and therefore `n>=9`.

If `L` is the longest side, the sine rule and perimeter normalization give

\[
 L=\frac1{h(1+\cos d/\cos t)}.
\]

Since `0<=d<=t-b`,

\[
 \frac{\cos d}{\cos t}
 \ge\frac{\cos(t-b)}{\cos t}
 =h+\tan t\sin b
 \ge h+\tan(2b)\sin b
 =\frac h{\cos(2b)}.
\]

Thus, writing `h=cos b`,

\[
 L\le\frac{2h^2-1}{h(2h-1)(h+1)}
   <\frac{\sqrt2h}{(1+h)^{3/2}}
   =s_nc_n(1-q^2)\le s_nc_n(1-v^2).                       \tag{14}
\]

Here is a fully elementary verification of the strict middle inequality.
All factors are positive because `n>=9`. Squaring and clearing denominators
reduces it to

\[
 2h^4(2h-1)^2-(2h^2-1)^2(1+h)>0.
\]

The left side is `(h-1)Q(h)`, where

\[
 Q(h)=8h^5-4h^4-6h^3-2h^2+2h+1.
\]

We have `9/10<h<1`, since `b<=pi/9<22/63` and
`cos b>=1-b²/2>9/10`. Put `u=1-h`, so `0<u<1/10`. Then

\[
 Q(1-u)=-1-4u+36u^2-58u^3+36u^4-8u^5
 \le -1+36/100+36/10000<0.
\]

As `h-1<0`, the required product is strictly positive. An obtuse triangle
lies in the circle having its longest side as diameter, so `r=L/2` and
(14) imply (7).

## 5. Acute triangles

There is no acute pentagon closure triangle, because its three angles are
positive odd multiples of `pi/5` summing to `pi`; its only shape is
`(pi/5,pi/5,3pi/5)`. We may therefore assume `n>=7`, so `0<q<23/100`.
We claim

\[
 \frac1{h(1+h+\sin b)}
 <\frac{\sqrt2h}{(1+h)^{3/2}}
 =s_nc_n(1-q^2).                                          \tag{15}
\]

After multiplying positive factors, this says
`h²(1+q)>cos(b/2)`. Substituting
`h=(1-q²)/(1+q²)` and squaring positive sides reduces it to

\[
 (1-q^2)^4(1+q)^2-(1+q^2)^3>0.
\]

The left side is `q F(q)`, where

\[
 F(q)=2-6q-8q^2-q^3+12q^4+q^5-8q^6-3q^7+2q^8+q^9.
\]

For `0<=q<=23/100`, discarding positive terms gives

\[
 F(q)\ge2-6(23/100)-8(23/100)^2-(23/100)^3
           -8(23/100)^6-3(23/100)^7>0.
\]

The final strict comparison is rational (its value exceeds `18/100`).
Combining (9) and (15) yields
`2R_c<s_nc_n(1-q²)<=s_nc_n(1-v²)`. The circumcircle therefore satisfies
(7), establishing strict containment for all acute cases.

## 6. Completion, scope, and stronger consequences

Every closure triangle is either acute or obtuse, and the three cases above
are exhaustive. They prove strict tensor containment for all odd
`m>=n>=7` and for `n=5,m>=9`. The pentagon tensor identities in
[research.tex](research.tex), Section 4, prove `5×5` containment; the
[reviewed exact 5×7 certificate](tensor-certificate-review-20261022.md)
proves strict containment for `5×7`. Exchanging factors completes (1).
The maintained affine covariance and capacity decomposition then give (2).

The new argument also provides analytic proofs for the previously finite
`7×7` and `5×9` results; their
[existing certificates](tensor-extensions-20261022/report.md) remain valid
independent evidence. It does not require replaying any certificate producer.

The strict cases have a further local consequence. Since the width generators
and closure-triple tensors depend continuously on labelled dual vertices,
and regular odd polygons have no opposite normals, no collinear dual triples,
and no zero triple barycentric coordinates, the feasible triple list remains
constant in a sufficiently small neighbourhood. Finite strict full-dimensional
containment then persists there. Consequently each pair except `5×5` has an
open labelled nonregular neighbourhood on which the width-body formula holds
for every independent pair of invertible linear transformations. No explicit
neighbourhood radius is claimed. This is stronger than the earlier uniform-
rotation neighbourhood, and follows from the same finite-continuity argument
now applied to four-dimensional tensor sets.

No sharp affine systolic ratio is optimized in this proof. Formula (2) supplies
an exact finite denominator for that separate question. The proof does not
claim width equality for arbitrary planar polygon pairs, arbitrary affine
perturbations that change the labelled combinatorics, or unrestricted convex
bodies. Nor does it settle human proof acceptance or novelty.

## Review and reproducibility record

The proof is analytic and can be checked from the displayed identities and
linked maintained sources. New scratch material is under
`/tmp/msc-math/autonomous-20261028/affine/`. The preliminary numerical
`nuclear_probe.py` tested a different, insufficient bound based on two
Euclidean incircles; it is not an input to this proof and does not certify
containment. That test failed for unequal pairs, motivating retention of the
polygonal gauge. No numerical search result is promoted to a theorem here.

Fresh assembled-proof reviews: pending. Root owns shared-source integration,
status, commits, cleanup, and any thesis-selection decision. This note is the
only maintained file owned by the present worker.
