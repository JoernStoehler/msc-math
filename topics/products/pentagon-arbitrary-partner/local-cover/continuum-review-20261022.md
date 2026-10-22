# Local-cover continuum review, 22 October 2026

This bounded mathematical review addresses sections 1–3 of
[local-cover-rigidity.md](../local-cover-rigidity.md). It starts from checkout
`5193274f835bc1e48108fe43a0377fc3edc00904`, following the
[preliminary inspection](preliminary-continuum-check-20261022.md).
Owner: `/root/msc_today_scope/continuum_review`, with receiving/integration
owner `/root/msc_today_scope`. Actual source inspection began at
2026-10-22 13:40:03 UTC; the bounded mathematical review reached its
substantive result at 13:51:43 UTC. Only this note is this reviewer's edit scope.

**Result.** The uniform polygon-loop lower estimate is valid. The disk-and-chord
proof below supplies it without a generic-position assumption or deletion of
self-crossings. Positive slot marginals and full difference rank imply strict
linear growth by a direct quantitative inequality. Unique template placements
then transfer the local minimum to every nearby convex-body cover, with no
bound on its number of facets. These are mathematical implications proved here;
the pentagon's finite hypotheses are supplied by the inspected exact checker
and its [fresh verification report](verification/20261022/report.md).

The remaining capacity interface is explicit: the complete fixed-pentagon
template-cover functional must equal `c_EHZ(K ×_L Q)` with the stated
normalization. The repository attributes this to the BMP normal-triangle
criterion. This review does not independently verify that external theorem or
promote the result to the manuscript. The original note's section 4
global-buffer claim, global
optimality, novelty and human acceptance are outside this review.

## 1. A uniform lower estimate by truncating at vertex disks

Write `[x,y]=det(x,y)`, let `J(x,y)=(-y,x)`, and let `S(L)` denote the signed
shoelace area of an oriented polygonal loop, allowing repeated vertices.
Let

\[
 H=\operatorname{conv}\{v_0,\ldots,v_{n-1}\},\qquad n\geq3,
\]

where the listed vertices form a strictly convex counterclockwise polygon.
Indices are cyclic. Strict convexity here means that these are its extreme
vertices and no adjacent sides are collinear.

**Lemma 1 (uniform loop bound).** There are `r₀>0` and `C_H<∞`, depending
only on `H`, such that, for `0≤r<r₀` and arbitrary points

\[
 |a_i-v_i|\leq r,\qquad |b_i-v_i|\leq r,
\]

the loop `L=(a₀,b₀,a₁,b₁,…,a_(n−1),b_(n−1))` satisfies

\[
 A\bigl(\operatorname{conv}\{a_i,b_i:i=0,\ldots,n-1\}\bigr)
 \geq S(L)-C_Hr^2.                                      \tag{1}
\]

The same lower bound holds for any larger convex hull containing these
points. Other points need not satisfy an interior-point hypothesis.

**Proof.** The case `r=0` is immediate. Denote the long segment from `b_i`
to `a_(i+1)` by `E_i`. Every point of `E_i` is within distance `r` of the
corresponding point on `[v_i,v_(i+1)]`, by affine interpolation of endpoints.
Consequently, for sufficiently small `r`, nonadjacent long segments are
disjoint: the finitely many disjoint nonadjacent original sides have positive
separation. For `n=3` there are no such pairs.

There is also a constant `C₀`, uniform in all endpoint choices, such that the
intersection of the supporting lines of `E_(i−1)` and `E_i` is within `C₀r`
of `v_i`. To see this explicitly, set

\[
 d_-=a_i-b_{i-1},\qquad d_+=a_{i+1}-b_i.
\]

The intersection solves `a_i+s d_-=b_i+t d_+`. The matrix with columns
`d_-,-d_+` tends to the invertible matrix with columns
`v_i-v_(i−1),-(v_(i+1)-v_i)`. Its inverse is uniformly bounded for small
`r`; the right-hand side `b_i-a_i` has norm at most `2r`. Thus `|s|+|t|`
is bounded by a constant times `r`, and the bounded lengths of `d_±` give
the claimed bound. Taking the maximum over finitely many vertices makes
`C₀` common to all `i`.

Fix `R>max{1,C₀}` and let `D_i` be the closed disk of radius `Rr` about
`v_i`. Shrink `r₀` so that the disks are pairwise disjoint and a long
segment never meets a disk at a nonincident vertex. The latter follows
from the positive distances between each original vertex and each
nonincident original side, since the perturbed side is within `r` of
the original side. These choices use only `H` and the fixed `R`.

Starting at `b_i` strictly inside `D_i`, the segment `E_i` exits `D_i` at
a unique point `q_i^+` and later enters `D_(i+1)` at a unique point
`q_(i+1)^-`, before reaching `a_(i+1)` strictly inside that disk.
Convexity of each disk and disjointness of the two disks give uniqueness
and the stated order. Replace, at each vertex, the portion

\[
 q_i^-\longrightarrow a_i\longrightarrow b_i\longrightarrow q_i^+
\]

by the straight chord `[q_i^-,q_i^+]`. Let `L'` be the resulting loop.

This new polygon is simple, including when `a_i=b_i`. Indeed:

* Its retained long segments lie outside all open vertex disks. Nonadjacent
  ones cannot meet by the original-side separation. Adjacent ones cannot
  meet because their supporting-line intersection lies strictly inside
  their common disk, at distance at most `C₀r<Rr` from its center.
* The endpoints `q_i^-` and `q_i^+` are distinct: equality would put that
  supporting-line intersection on the circle of radius `Rr`.
* Every chord lies strictly inside its own disk apart from its endpoints,
  by strict convexity of the disk. Thus different chords do not meet, and
  a chord meets retained long segments only at its prescribed endpoints.

All vertices of `L'` lie on original long segments and hence belong to the
convex hull in (1). The polygon and its enclosed region therefore lie in
that hull. Moreover `q_i^±→v_i` uniformly as `r→0`, so the signed shoelace
area of `L'` tends uniformly to `A(H)>0`. Shrinking `r₀` once more makes
`S(L')>0`. Since `L'` is simple, it is counterclockwise and

\[
 S(L')=A(L')\leq A\bigl(\operatorname{conv}\{a_i,b_i\}\bigr).
\]

The signed-area difference at vertex `i` is the signed area of the closed
four-point loop `(q_i^-,a_i,b_i,q_i^+)`. Translate its coordinates by `−v_i`;
each coordinate has norm at most `Rr`, so each of its four determinant
terms has absolute value at most `R²r²`. Its signed area has absolute
value at most `2R²r²`. Additivity of the oriented shoelace integral,
including subdivision of a straight segment, now gives

\[
 |S(L)-S(L')|\leq2nR^2r^2.
\]

This proves (1), with `C_H=2nR²`. No perturbation to general position,
winding-number claim for a self-intersecting loop, or independent movement
of coincident labels was used. ∎

**Corollary 1 (the first-order lower model).** Put `a_i=v_i+α_i` and
`b_i=v_i+β_i`. Exact expansion of the shoelace polynomial gives

\[
 S(L)=A(H)+\sum_i\bigl(c_i^-\cdot\alpha_i+c_i^+\cdot\beta_i\bigr)+Q,
\]
\[
 c_i^-={J(v_{i-1}-v_i)\over2},\qquad
 c_i^+={J(v_i-v_{i+1})\over2},\qquad |Q|\leq nr^2.        \tag{2}
\]

There are `2n` quadratic determinant terms, each bounded by `r²/2`.
Combining (1) and (2) gives a common remainder bound
`n(2R²+1)r²`, independent of the choices of incoming and outgoing labels.
In particular, if each label is displaced by a translation `u_j`, and
`g_s` denotes the row obtained from any choice `s` of one incident label
at every slot, then the hull of all translated template vertices satisfies

\[
 A(P(u))\geq A(H)+\max_s g_s\cdot u-C\|u\|_2^2.          \tag{3}
\]

Here a common translation has been removed, and
`r=max_j|u_j|≤‖u‖₂`. Arbitrary coupling between the displacements, including
using one label in several slots, is allowed: Lemma 1 holds for every
actual endpoint configuration. It only needs the selected points to belong
to `P(u)`.

## 2. What the compressed finite certificate proves

The following argument makes explicit the interface between the exact
50-variable certificate and (3); it does not infer a continuum claim from
an arithmetic `PASS` alone.

**Lemma 2 (positive marginals and difference rank).** Let slot `k` have rows
`a_kj∈Rᵈ`, with `j=0,…,m_k−1`. Suppose there are strictly positive numbers
`p_kj` satisfying

\[
 \sum_jp_{kj}=1\quad\hbox{for every }k,\qquad
 \sum_{k,j}p_{kj}a_{kj}=0.                               \tag{4}
\]

Let `D` be the matrix whose rows are `a_kj−a_k0`, for all `k` and `j>0`.
If `D` has rank `d`, there is `γ>0` such that

\[
 F(u):=\sum_k\max_j a_{kj}\cdot u
      =\max_s\Bigl(\sum_k a_{k,s_k}\Bigr)\cdot u
      \geq\gamma\|u\|_2.                               \tag{5}
\]

**Proof.** Let `μ=min p_kj>0` and let `ω_k(u)` be the oscillation
`max_j a_kj·u−min_j a_kj·u`. By (4),

\[
 F(u)=\sum_k\left(\max_j a_{kj}\cdot u-
                   \sum_jp_{kj}a_{kj}\cdot u\right)
       \geq\mu\sum_k\omega_k(u).
\]

Each component `(a_kj−a_k0)·u` has absolute value at most `ω_k(u)`.
With `M=max_k(m_k−1)>0`,

\[
 \|Du\|_2^2\leq M\sum_k\omega_k(u)^2
             \leq M\left(\sum_k\omega_k(u)\right)^2.
\]

Full column rank gives `σ_min(D)>0`, so (5) holds with

\[
 \gamma={\mu\,\sigma_{\min}(D)\over\sqrt M}>0.           \tag{6}
\]

The equality between the two maxima in (5) is just independent choice of
one row at each slot, not independence of the physical translations. ∎

For the pentagon, `d=18`, there are ten slots with five choices each,
and `D` has forty rows. Thus (6) gives
`γ=μ σ_min(D)/2`. This is an existential positive constant, not a computed
exclusion radius. The checker supplies

\[
 \mu={33\over218}-{39\sqrt5\over1090}>0,
 \qquad \operatorname{rank}D=18.
\]

The original product-distribution argument is also valid: the positive
marginals give positive weight to every full row, their mean is zero,
and the within-slot differences span the full space. The direct inequality
above proves precisely the needed coercivity without enumerating `5¹⁰`
rows or separately appealing to a convex-hull interior theorem.

Combining Lemma 2 with (3) gives, for all sufficiently small anchored `u`,

\[
 A(P(u))\geq m+\gamma\|u\|_2-C\|u\|_2^2
          \geq m+{\gamma\over2}\|u\|_2,                 \tag{7}
\]

where `m=A(H)`. Choose the neighborhood inside both the radius of Lemma 1
and `‖u‖₂≤γ/(2C)`. This proves a uniform neighborhood inequality, including
nonlinear paths and varying directions.

**Checker-to-lemma correspondence.** The inspected
[verifier](verify_local_cover.py) uses `(x,y/τ)` coordinates, constructs
the anchored rows by putting `c_i^-` or `c_i^+` into the chosen triangle's
two coordinates, and uses zero for the anchored triangle `0`. Its
`groups` are exactly the slot rows of Lemma 2. Its first eighteen equations
are the vector equation in (4), and its last ten equations are the slot
sums. Its `diffs` matrix is precisely `D`. The signs of both coefficients
agree with (2).

The fixture's `H` is a positive-scale quarter-turn of a regular pentagon;
the additional coordinate map has positive determinant. Its listed order
is therefore strictly convex and counterclockwise, as required by Lemma 1.
The exact hull and incidence checks identify the five copies at each of
its vertices. The remaining template vertices do not require a separate
strict-interiority check, by the stronger formulation of Lemma 1.

The fresh Sol rerun records rank `E=28`, exact `Ep=b`, fifty positive
marginals, rank `D=18`, base hull equality, and the endpoint pairs used
below. It agrees with the retained September output except elapsed time.
The construction using `(EEᵀ)⁻¹` is a way of obtaining (4); rank `E=28`
is not an additional continuum hypothesis once (4) has been checked.
SymPy and the small field/geometry implementation remain the finite
arithmetic trust boundary. No floating-point minimum or sample direction
is used in (7).

## 3. Unique placements and the arbitrary-body transfer

**Lemma 3 (an exposed difference fixes a translation).** Let `H` be a
compact convex body and let `a,b` be two points of a fixed template `T`.
Suppose `d=a−b` is an exposed point of `H−H`, and at least one translate
of `T` is contained in `H`. Then that translation is unique.

**Proof.** Choose a linear functional `ℓ` whose maximizing face in
`H−H` is `{d}`. If `F_+` and `F_-` are respectively the maximizing and
minimizing faces of `ℓ` on `H`, then `F_+−F_-={d}`. Both faces are
singletons: fixing an element of one face makes the difference map
injective on the other. Write them `{x_+}` and `{x_-}`. For any feasible
translation `t`,

\[
 \ell(a+t)-\ell(b+t)=\ell(d)
   =\max_H\ell-\min_H\ell.
\]

The two bounding inequalities for `a+t,b+t∈H` must both be equalities.
Thus `a+t=x_+`, `b+t=x_-`, which determines `t`. ∎

Every vertex of the polygon `H−H` is exposed. The verifier finds such a
pair in each of the ten templates and checks their fixture endpoints.
Consequently the ten fixture translations `t_j*` are the unique feasible
translations into `H`. The proof concerns an actual vertex of `H−H`;
merely lying on its boundary would not suffice.

**Lemma 4 (uniform proximity of every feasible placement).** Let finitely
many fixed nonempty compact templates `T_j` have unique translations
`t_j*` into a compact body `H`. For every `η>0` there is `δ>0` such that
if a compact convex body `K` satisfies `d_H(K,H)<δ`, then every choice
of translations with `T_j+t_j⊆K` satisfies `max_j|t_j−t_j*|<η`.

**Proof.** Otherwise choose `K_l→H` and offending translations. Pass to
a subsequence with one fixed offending label `j`. Fix `x_j∈T_j`.
Since `x_j+t_j(l)∈K_l` and the `K_l` are uniformly bounded, the translations
are bounded. A subsequence converges to `t`. For every `x∈T_j`,
`dist(x+t_j(l),H)≤d_H(K_l,H)`, so `x+t∈H`. Hence `T_j+t⊆H`, contrary
to uniqueness and `|t_j(l)−t_j*|≥η`. ∎

**Theorem 1 (local area rigidity of arbitrary covers).** Assume (7), the
unique translations of Lemma 3, and
`H=conv ⋃_j(T_j+t_j*)`. Then there is a Hausdorff neighborhood of `H`
such that every compact convex body `K` in that neighborhood containing
a translate of every `T_j` satisfies

\[
 A(K)\geq A(H),
\]

with equality exactly for translates of `H` in the neighborhood.

**Proof.** Choose any feasible translations and set

\[
 a=t_0-t_0^*,\qquad u_j=t_j-t_j^*-a.
\]

Then `u₀=0`. Lemma 4 makes the entire finite-dimensional vector `u`
small enough for (7), since `|u_j|≤2max_k|t_k−t_k*|`. Crucially, the
anchored hull satisfies

\[
 P(u)\subseteq K-a,
\]

so the containing body is translated together with the hull. Therefore

\[
 A(K)=A(K-a)\geq A(P(u))\geq m+{\gamma\over2}\|u\|_2.
\]

Equality forces `u=0` and hence `H+a⊆K`. A proper containment of compact
convex bodies when the smaller body has interior strictly increases area:
an exterior point of the larger body, a separating support line of the
smaller body, and an interior disk of the smaller body yield a
positive-area portion of their convex hull beyond that line. Thus area
equality implies `K=H+a`. Conversely such translates have area `m`. ∎

This fixes the original note's abbreviated anchoring inclusion. It also
shows that no continuous selection of optimal placements is needed:
Lemma 4 controls every feasible choice, and no polygonal approximation
or facet bound is used. The linear modulus in (7) concerns the anchored
placement vector. Theorem 1 does not assert a linear area modulus in
Hausdorff distance for arbitrary bodies.

## 4. Normalization can be proved at the covering level

Define the purely geometric functional

\[
 C_{\mathcal T}(K)=\max\{\lambda\geq0:
     \text{for each }j\text{ there is }t_j
     \text{ with }\lambda T_j+t_j\subseteq K\}.
\]

Here `K` has nonempty interior and the templates are nondegenerate compact
triangles. The maximum exists: scale is bounded by the ratio of diameters
for any one triangle, and after choosing a point of each template all
translations in a maximizing sequence are bounded; pass to a subsequence.
It is positive since sufficiently small copies fit in an interior disk.

Translation invariance, monotonicity and degree-one homogeneity of
`C_T` follow directly from its definition. Also `C_T(K)≥1` if and only
if `K` contains a translate of every unscaled template. For the direction
from a larger fitted scale to scale one, homothetically contract each
fitted copy about one of its own points; convexity of that copy keeps the
smaller translated copy inside it.

The endpoint-pair certificate gives `C_T(H)=1` without another capacity
calibration. Feasibility gives `C_T(H)≥1`. If `d∈T_j−T_j` is the certified
vertex of `H−H`, a functional exposing `d` has
`ℓ(d)=h_(H−H)(ℓ)>0`. Containment of `λT_j+t` in `H` would give
`λd∈H−H`, hence `λ≤1`.

Continuity near `H` also follows without importing a capacity-continuity
theorem. Translate the coordinates so `B(0,ρ)⊆H`, with `ρ>0`. If
`d_H(K,H)≤δ<ρ`, the support functions give

\[
 (1-\delta/\rho)H\subseteq K\subseteq(1+\delta/\rho)H.
\]

Indeed `|h_K-h_H|≤δ` on unit directions and `h_H≥ρ`. Monotonicity and
homogeneity imply `1−δ/ρ≤C_T(K)≤1+δ/ρ`. Thus

\[
 K\to H\quad\Longrightarrow\quad
 C_T(K)\to1,\qquad K/C_T(K)\to H.
\]

Apply Theorem 1 to the last body, which is a feasible capacity-one cover
in the geometric sense. It follows that, locally,

\[
 {C_T(K)^2\over A(K)}\leq{1\over A(H)},                 \tag{8}
\]

with equality exactly when `K` is a translate of a positive homothet of
`H`. Conversely all such bodies give equality by homogeneity and
translation invariance.

**External capacity interface.** If the complete fixed-pentagon template
criterion identifies `C_T(K)` with `c_EHZ(K ×_L Q)` for every planar
convex body with interior, then (8) is exactly the claimed local systolic
ratio inequality, after dividing by the fixed factor `2A(Q)`. The
[repository contract](../research-charter.md#evaluator-contract) states
that identification and its support-length and factor conventions;
[the source note](../local-cover-rigidity.md) attributes it to
Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v2, Theorems 2.5 and 3.3.
Those external hypotheses and the completeness interpretation of the
constructed normal templates have not been independently re-audited in
this bounded review. They remain explicit inputs, not conclusions of the
finite rerun or Lemmas 1–4.

The charter displays the regular pentagon with vertex angles `π/5+2πi/5`,
whereas the fixture uses `2πi/5`. The capacity input here concerns the
fixture's `Q` and its own templates. Moving between these conventions
requires the corresponding common rotation of both factors and a relabeling;
the common orthogonal map preserves the Lagrangian symplectic form.

All finite rows were checked in `(x,y/τ)` coordinates. Apply the inverse
linear map to the covering statements before the capacity identification:
area is multiplied by a fixed positive determinant, Hausdorff neighborhoods
map to neighborhoods, and Euclidean norms are equivalent under this fixed
invertible map. Thus the strict inequalities and equality classification
transfer. No symplectic invariance of a one-factor coordinate change is
being asserted.

## 5. Disposition and precise remaining boundary

The crossing/deletion route in the preliminary note is plausible, but its
topological and limiting steps are unnecessary: Lemma 1 proves the required
uniform estimate directly, including coincident labels. The positive
marginal/rank interface and placement-to-body passage have complete proofs
above. Taken with the finite predicates reported in the fresh exact rerun,
they establish the local geometric covering result and (8).

The source note needs the simultaneous-translation notation in Theorem 1
and can replace its loop argument with Lemma 1. Its capacity consequence
is justified conditional on the named external criterion; this review
does not convert that imported input into an independently checked source
theorem. The analogous conclusion at `−H` follows once the template bank
is identified as closed under reflection up to translation. This property
follows directly from the two edge orders in the fixture: if
`e₁+e₂+e₃=0`, the two vertex sets are `{0,e₁,e₁+e₂}` and
`{0,e₁,e₁+e₃}={0,e₁,−e₂}`. The second set is `e₁` minus the first;
after centroid normalization they are exact negatives. Thus reflection
permutes the ten template classes, and reflecting a nearby cover of `−H`
reduces to Theorem 1 at `H`. No additional continuum argument is needed.

No numerical radius, global competing-cover exclusion, the original note's
section 4 buffer,
global optimum, novelty conclusion or manuscript acceptance follows from
this review. Integration and any broader assembled/source review belong
to the receiving owner.
