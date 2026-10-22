# Next mathematical obstructions after the 5×7 certificate

Bounded source-reasoning follow-up by `/root/msc_today_scope/continuum_review`,
started 2026-10-22 14:10:23 UTC. Owned output: this note only. Sources read:
[research.tex](research.tex), the [tensor review](tensor-certificate-review-20261022.md),
the [certificate](tensor-certificate-20261022/report.md),
[all-odd angular theorem](odd-regular-angular-width.md), and
[nonagon closure classification](nonagon-angular-width.md).
No producer, larger-pair computation or manuscript edit ran.
Source reasoning and the final self-check ended at 14:17:30 UTC
(7 minutes 7 seconds); receiving-owner review remains separate.

The three routes answer different questions. An analytic certificate explains
why full tensor containment holds and might generalize. Affine area optimization
asks for a sharp systolic bound inside the newly solved 5×7 capacity family.
A larger-pair certificate tests where the containment phenomenon extends.
The present certificate does not settle the latter two questions.

This inspection finds a useful distinction: **the best rotated 5×7 pair is
not the best independently affine pair.** A direct increasing deformation is
proved below. Thus affine optimization has a concrete new obstruction, rather
than merely recovering the existing rotation maximum.

## 1. Affine-ratio optimization is a finite determinant problem

Write `D_n=conv{±n_i^(n)}`, `h_n=cos(π/n)`, `S_n=A(P_n)`, and set

\[
 B_{57}={(1+h_5)^2(1+h_7)^2\over2S_5S_7},\qquad
 m(M)=\max_{i<5,j<7}|(n_i^{(5)})^TMn_j^{(7)}|.
\]

The established capacity formula and area transformation give

\[
 \operatorname{sys}(GP_5\times_L HP_7)
 =B_{57}{|\det M|\over m(M)^2},\qquad M=G^{-1}H^{-T}.
\]

Every nonsingular `M` is realizable, and homogeneity reduces the optimization
to the compact four-dimensional polytope

\[
 \mathcal P=\{M: |(n_i^{(5)})^TMn_j^{(7)}|\leq1\},       \tag{1}
\]

with seventy linear inequalities in four matrix entries. Compactness follows
by selecting two independent normals from each factor, which bounds all
four entries in corresponding bases. The optimum is positive, since small
scalar matrices are feasible, so a maximizing matrix is nonsingular.
Reflection symmetry permits restriction from `|det M|` to positive `det M`.

Equivalently, (1) says `M D_7⊆D_5^polar`. Thus the optimization is the
largest-area linear image of a regular fourteen-gon contained in a fixed
regular decagon. The elementary area estimate

\[
 \det M\leq {A(D_5^\circ)\over A(D_7)}                  \tag{2}
\]

is **strict at the true optimum**: equality of areas for nested convex bodies
would force `M D_7=D_5^polar`, impossible for nonsingular `M` because the
vertex counts are fourteen and ten. Compactness makes the maximum attained.
This is exactly why the sharp 5×5 area argument cannot be copied verbatim.
Equation (2) supplies a ceiling, not the sharp answer.

### A concrete affine improvement over every rotation

Put `θ=π/70`, `t=tan θ`, `β=4π/7`, and

\[
 M_0={R_\theta\over\cos\theta}
     =\begin{pmatrix}1&-t\\t&1\end{pmatrix},\qquad
 N=\begin{pmatrix}0&0\\\sin\beta&-\cos\beta\end{pmatrix}.
\]

The rotation profile shows `M₀` realizes the largest normalized determinant
among rotations, namely `sec²(π/70)`. There are exactly two active absolute
pairings in (1) at `M₀`: `(i,j)=(0,0)` with value `+1`, and `(4,2)` with
value `−1`. Indeed the residues

\[
 2\pi(i/5-j/7)\pmod\pi
\]

run once through the thirty-five multiples of `π/35`. The two closest
residues to `θ` are `0` and `π/35`; their index pairs are respectively
`(0,0)` and `(4,2)`, since `7·4−5·2=18` and `36≡1 mod 35`.

The perturbation preserves both active pairings exactly. It has `N₁₁=0`,
and `N n_2^(7)=0` because that normal is `(cos β,sin β)`.
All other inequalities are strict at `M₀`, so they remain strict for all
sufficiently small positive `ε`. Hence `m(M₀+εN)=1`. But

\[
 \det(M_0+\varepsilon N)
 =1+t^2+\varepsilon(-\cos\beta+t\sin\beta)>1+t^2,
\]

since `π/2<β<π` and `t>0`. This proves a strict affine improvement over
the best rotation without numerical optimization. It does **not** locate
the global affine optimum or classify its equality cases.

### The cheapest global reduction

A maximizer of `det` over `𝒫` lies on a face of dimension at most two.
To prove this, let `V` be the tangent space of a face of dimension at
least three. It intersects the two-dimensional conformal subspace
`{aI+bJ}` nontrivially. For a nonzero `N` in that intersection,
`det N=a²+b²>0`. At any point in the relative interior of the face,

\[
 \det(M+sN)=\det M+s\,\operatorname{cof}(M):N+s^2\det N
\]

has a feasible strict increase for some arbitrarily small signed `s`.
Thus such a point cannot maximize. A global maximizing point lies in
the relative interior of its minimal face, proving the claim.

Consequently a finite exact route is available: enumerate the genuine faces
of dimensions zero, one and two; optimize the restricted quadratic on each;
compare their values and equality sets. Interior stationary points of a
two-face must be included. **Checking only polytope vertices is insufficient
without a further argument.** This needs new work but not a new general
convex-geometric theorem. Dihedral symmetry and the two active constraints
above offer a small initial contact-family investigation before any exhaustive
enumeration.

## 2. Analytic explanation of strict tensor containment

Independent rotations reduce the 840 labelled tensors to at most twenty-four
representatives: one pentagon closure type, two heptagon closure types and
twelve orders. Reflections may reduce this further, but twenty-four is a
safe bound without another orbit calculation. The source already provides
the explicit isosceles triples, so these are concrete matrices, not an
unspecified search over shapes.

An exact symbolic route need not use the full field of all sine/cosine
coordinates. In each factor apply

\[
 L_n=\operatorname{diag}(1,1/\sin(2\pi/n)).
\]

Then `sin(2πk/n)/sin(2π/n)` is a polynomial in `cos(2π/n)`, and
`h_n=−cos(2π((n−1)/2)/n)` for odd `n`. All transformed data for the pair
therefore lie in the compositum
`Q(cos(2π/5),cos(2π/7))`, of degree six. The tensor map
`T↦L₅ T L₇ᵀ` is invertible and preserves containment; no capacity invariance
of these coordinate maps is required.

There is already a quantitative fact available from the certificate. Let
`μ>0` be its common certified lower bound on all seventy convex weights,
for which `70μ<1`.
The signed generators satisfy `Σ_j C_j=0`. Subtracting `μ` from every
weight shows

\[
 T/(1-70\mu)\in\mathcal T_4
\]

for every certified target. Thus the certificate gives a uniform homothetic
buffer, not merely unquantified interior membership. A subsequent attempt
could seek short degree-six identities for modestly inflated representatives,
then recover strictness by contraction toward the interior point zero.
Carathéodory supplies representations with at most five generators for
membership, but does not guarantee strictly positive simplex coordinates
for every interior target; the inflation step avoids relying on that false
shortcut.

Favorable evidence is the small number of closure types, explicit source
coordinates and strict certified buffer. Unfavorable evidence is that the
existing witness uses dense weights and proves one fixed pair; it reveals
no monotonicity in the number of sides. The prior circle-deficit argument
controls only the angular projection. It cannot be upgraded to full tensor
containment merely by reusing its scalar margin.

The cheapest crux is one explicit identity for the closest representative
of each heptagon type, with a proof that symmetries cover the chosen order
classes. Success gives a readable proof ingredient; failure to find short
identities would favor retaining the checked finite proof and seeking a
different structural inequality. Neither outcome settles a uniform odd-pair
theorem.

## 3. A larger pair as a discriminator

There are two materially different small targets; neither was run here.

| Pair | Source-derived size | What it tests |
| --- | --- | --- |
| 7×7 | Four closure-type pairs; `14²·12=2352` labelled tensors and 98 signed generators | Whether removing the pentagon factor preserves full containment within the already explicit isosceles family. |
| 5×9 | Thirty nonagon triples, hence 1800 labelled tensors and 90 signed generators | Whether full containment survives a genuinely scalene closure type `(2,3,4)`, absent from 5×7. |

The nonagon count is nine triples of type `(1,4,4)`, eighteen of scalene
type `(2,3,4)` including both cyclic orientations, and three of type
`(3,3,3)`, as follows from the source's cyclic-gap classification.
Both targets have established angular containment, which is a useful
convention check but not evidence sufficient for full matrix containment.

A strict interior numerical discriminator can be followed by the existing
rational-interval correction method, with new completeness and sign checks.
An outside numerical target should instead lead to a rigorous separating
functional and one explicit violated inequality. Near-boundary behavior
should stop the strict-positive-weight route for diagnosis; the 5×5 control
already shows that legitimate boundary contact can occur. A single additional
pair would extend the finite result or refute a proposed generalization, not
prove uniform containment for all odd pairs.

## 4. Comparison, bounded outputs and resource uncertainty

These are proposed follow-up allowances, not execution already authorized
by this note or promises of completion. Actual quota cost is unknown.

| Route | First bounded useful output | Planning allowance and stop point |
| --- | --- | --- |
| Analytic explanation | Symmetry-reduced representatives and one short exact inflated-target identity, or a concrete reason the attempted identity family fails | About 30–60 minutes of symbolic reasoning/review; stop before expanding to a uniform theorem without a working mechanism. |
| Sharp affine ratio | Analyze the two-face containing `M₀`, follow its increasing determinant to new contact constraints, and identify an exact candidate or an unresolved contact family | About 30–60 minutes for that local contact analysis; a complete face enumeration/equality proof needs a separate estimate after its size is known. |
| Larger-pair discriminator | One pair with strict inside/outside/boundary evidence; if robust, a checkable interval certificate or separating witness | Roughly 15–30 minutes including source mapping and independent review. Existing 5×7 producer/check runs took seconds, but numerical degeneracy and proof review dominate uncertainty; no linear runtime extrapolation is guaranteed. |

If the intended result is a sharp extremal theorem, the determinant route
addresses it directly and now has a proved ascent direction. If the intended
result is a structural extension across odd polygons, 5×9 tests the scalene
obstruction while 7×7 is the simpler isosceles continuation. If a concise
explanatory proof is the priority, the twenty-four-representative, degree-six
route targets that need. These objectives should determine the next choice;
none of the three routes dominates all of them.
