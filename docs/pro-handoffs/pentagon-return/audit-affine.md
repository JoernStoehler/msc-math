# Independent audit: affine pentagon theorem

Scope: direct mathematical reading of `received/math/research.tex`, Sections 1–5 (with the affine rotation formula in Section 6). Compared the sparse reduction with the original supplied `../pentagon-generalization/math/product-six-facet-reduction.tex`. This review does not use numerical support searches or the returned verification script as evidence of completeness.

## Conclusion

I found **no internal mathematical gap** in the principal affine-pentagon capacity formula, the sharp affine-class systolic bound, or its equality classification. The argument is an exact, finite reduction plus two explicit convex-combination identities. In particular, it does not extrapolate the rotational case to arbitrary affine factors from angular support computations.

This conclusion is conditional on the stated Haim–Kislev capacity formula, whose original paper normalization was not independently re-audited in this assignment. The sparse reduction agrees with the previously supplied proof and is reconstructed below. Novelty, the later heptagon results, stability, and the imported symmetric-product interpretation are outside this report. No human acceptance is asserted.

## 1. Normalization and sparse reduction

The steps used in the proof are normalized **factor** steps `x_i = gamma_i u_i`, with factor mass one. Actual product weights multiply them by `1/2`. This distinction is maintained throughout.

For a fixed facet order, the objective factors as `t(1-t) B(gamma,eta)`, because same-factor symplectic pairings vanish. At a positive global maximum one can replace each factor distribution successively by a vertex of its feasible closure face: the objective is linear in that factor, and no replacement can exceed the global maximum. These faces are coordinate faces of the full closure polytope, so their vertices remain full vertices. Three scalar equalities imply support at most three. Positivity then makes `t=1/2` optimal. This proves existence of a sparse maximizer, not sparsity of every maximizer.

Moving the first weighted covector to the end changes the objective by minus twice its symplectic pairing with the sum of the others. Closure makes that change zero. Thus cyclic grouping is legitimate. With at most three facets per factor there are only two, four, or six cyclic alternating blocks. Six blocks require all six individual entries. Fixing one distinguished q entry leaves exactly two orders for the remaining q entries and six permutations of the p entries: twelve orders.

Independently summing pairings for `x1,y1,x2,y2,x3,y3`, using both closure sums, gives `Q=(x1.y1-x3.y2)/2`. The coefficient is consequently `(x1 y1^T-x3 y2^T)/2`, with no missing factor of two. The four-block word has `Q=x.y/2`. Together with `c=1/(2 max Q)`, this gives the reciprocal width-pairing formula stated in the return.

## 2. Width-body realization

For a facet dual vertex `u_i`, irredundancy gives `h_A(u_i)=1`. A closure step `x=gamma_i u_i` therefore has `h_A(x)=gamma_i` and `h_A(-x)<=1-gamma_i`, proving inclusion in `(A-A)^polar`.

The converse is substantive and checks out. In two dimensions, every edge-normal direction of `A-A` comes from an edge of `A` or `-A`, since its exposed face is `F_A(n)-F_A(-n)`. A vertex of the polar therefore lies at a signed facet normal divided by the width. On the opposite ray, `-n/h_A(-n)` lies on an edge or vertex of `A^polar`; representing it by that face's endpoints supplies a two- or three-point positive closure. The chosen `u_i=n/h_A(n)` is not on the line supporting the opposite polar face: its pairing with that face's primal support point is negative rather than one. Thus the three-point case is affinely independent, hence a closure vertex. Vanishing endpoint coefficient reduces to the two-point case.

This handles parallel opposite facets and degenerate choices of the opposite supporting face without silently requiring general position. Positivity of both support heights is ensured by placing the origin in the interior.

For a sparse distribution with at most three entries, a proper grouped sum is an individual step or its negative. Conversely, a singled-out step and all remaining steps can form the two groups in its factor. Independent choices for the two factors give feasible four-block words. Expanding two independent convex combinations proves that their matrix convex hull is exactly the stated tensor hull of the width bodies. There is no relaxation gap here.

## 3. Exact pentagon containment and exhaustiveness

Positive closure triples must have cyclic index gaps at most two, because a gap larger than pi excludes positive closure. Three positive integer gaps summing to five are therefore cyclic rotations of `(1,2,2)`: exactly five triples. No antipodal pair exists. The displayed weights solve closure and sum to one; their steps are `s(e,a,b)` with `a=lambda n2`, `b=lambda n3`, and `e+a+b=0`.

For `Dmat=conv{+/- n_i n_j^T}`, the two matrix identities are exact consequences of

- `lambda n1=e+n2`,
- `lambda n4=e+n3`,
- `n2+n3=-lambda^(-1)e`,
- `lambda^2+lambda=1`.

The first identity's unsigned coefficients sum to `2 lambda^2+lambda^3=1`; the second's sum to `2 lambda^2<1`. Negative signs belong to signed generators of `Dmat`, not to negative convex weights. Since zero is in this centrally symmetric hull, the second identity is a valid convex combination too.

I checked each entry of the six-case order table. For fixed `w=a`, the first four are immediate from reflection fixing `e` and exchanging `a,b`. The last two reduce as follows:

- `e a^T-a b^T = -e e^T+b b^T = -F T1 F`;
- `e b^T-a a^T = -e e^T+b a^T = -F T1`.

These follow by substituting `e=-a-b`. Left reflection treats `w=b`; independent pentagon rotations treat every closure-triple pair. All preserve `Dmat`. Thus the argument exhausts the 25 triple pairs and their twelve orders algebraically. It is not an empirical observation about selected maximizers.

## 4. Independent affine changes

Under `A -> G A`, normalized dual steps transform by `G^(-T)` and their weights remain unchanged. For the other factor use `H^(-T)`. The matrix transformation is therefore `T -> G^(-T) T H^(-1)`. It transforms both the four-block hull and the six-block candidates by the same invertible linear map, preserving containment.

Pairing transformed steps at zero rotation gives `n_i^T G^(-1) H^(-T) n_j`, exactly the returned `M`. Hence `max Q = s^2 m(M)/2`, and the capacity is `(1+h)^2/m(M)`. Sign choices are available in the centrally symmetric width bodies and feasible four-block grouping. This covers orientation reversal without a positivity restriction on determinants. Translations are removed using capacity translation invariance before the proof; singular factors are explicitly excluded.

For the later rotation profile, `(R_theta H)^(-T)=R_theta H^(-T)`, giving `G^(-1)R_theta H^(-T)`. Expanding the rotation as `I cos(theta)+J sin(theta)` verifies the displayed coefficient order. It is important not to commute `G` or `H` with the rotation.

## 5. Sharp area bound and equality

Since factor areas multiply by `|det G det H|`, the ratio becomes
`w^4/(2 S^2) * |det M|/m(M)^2`.

The scalar pairing bounds on all signed decagon vertices imply, by bilinearity, `(M/m(M))D subset D^polar`. The decagon polar is `sec(alpha) R_alpha D`, with `alpha=pi/10`. Area monotonicity yields the determinant bound `sec(alpha)^2`. The stated constants reduce to `(3+sqrt(5))/5`.

For full-dimensional compact convex bodies, a proper containment cannot have equal area; otherwise a point outside the smaller closed body and a nearby interior region supply positive area difference. Thus equality is equivalent to equality of the bodies, not merely of their support maxima at a few directions.

The induced linear automorphism of the regular decagon permutes vertices and preserves or reverses cyclic adjacency. Once images of two adjacent vertices are selected, the linear map is uniquely determined and agrees with a dihedral orthogonal symmetry. Therefore the equality class `M=t R_alpha U`, `t>0`, `U in Sym(D)`, is correct. Conversely these matrices have the requisite normalized image; in particular `m(R_alpha)=cos(alpha)` gives attainment.

## Integration cautions and remaining checks

1. Audit the original Haim–Kislev theorem's conventions against the thesis once, centrally. This report reconstructs the factors and signs downstream but does not replace that source-contract check.
2. State **existence** of a four-block maximizer. The proof does not rule out six-block maximizers attaining the same value; the returned document explicitly gives such a tie later.
3. Keep the domain explicit: independently affinely regular pentagons. The ratio is not an upper bound for arbitrary polygon products, and no nonsingular limit argument is needed for the stated theorem.
4. Keep angular containment distinct from full matrix containment. It is the latter, proved here by the two identities, that permits arbitrary independent affine transformations.
5. Check attribution/novelty separately. Mathematical soundness of this argument does not determine whether the affine theorem is already in the literature.

No new computational certificate, arbitrary-precision experiment, or exhaustive numeric search is needed to fill a gap in the audited principal proof: none was found. Further review should concentrate on the imported formula and independent verification of this concise exact chain, rather than enlarging a numerical sample.
