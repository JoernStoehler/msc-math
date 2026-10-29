# Independent audit: sharp four-simplex bound

Auditor: simplex_independent_audit. Checked 2026-10-28, approximately 22:47–22:54 UTC. Scope: the assembled Markdown argument, the current LaTeX section, the exact LOP4 checker, and actual local Haim–Kislev and Chaidez–Hutchings primary-source TeX. This is mathematical review, not a novelty or publication-priority claim. No author files edited.

## Conclusion

No mathematical error found in the proposed inequality or its union-of-triangles equality locus. The reduction includes every nondegenerate four-simplex, including ones with Lagrangian faces; it does not depend on generic Reeb dynamics. The elementary LOP4 proof below makes the key polytope input self-contained. Exact enumeration independently agrees. The final assembled manuscript should still be reviewed after integration.

## Capacity and volume normalization

Inspected `topics/literature/sources/hk2017/EHZ-polytopes.tex`, Theorem `formula_theorem`, directly. Its formula is `c = 1/(2 max Q)` over every facet permutation and nonnegative closure weights. Its displayed word orientation is reversed from the project convention, but reversal permutes the full candidate set. No capacity equality is inferred from merely feasible words.

For K0={z_i<=1,-sum z_i<=1}, volume is 5^4/24. The five transformed facet normals have unique normalized closure weights 1/5. Cyclic rotation changes the ordered quadratic by the pairing of a moved vector with the total sum, hence by zero. Fixing facet5 last leaves exactly24 possible orders. Its own contribution vanishes. Thus c(TK0)=25/(2S), vol(TK0)=625|det T|/24 and sys=3|Pf(Omega)|/S².

The newer barycentric-gradient version is cleaner and also correct. With x_i=-grad lambda_i, the weight factors cancel the facet normalizations; c=1/(2S). The barycentric map takes K to conv(0,e1,...,e4) with absolute determinant |Pf(Omega)|, so vol=1/(24|Pf(Omega)|). Every nondegenerate alternating matrix is the symplectic Gram matrix of a basis, so equality forms are realized by actual simplices. Equal Gram matrices give symplectically equivalent labeled simplices up to translation.

## Independent elementary proof of the LOP4 input

Let H be the polytope in six coordinates y_ij given by 0<=y_ij<=1 and 0<=y_ij-y_ik+y_jk<=1 for the four triples i<j<k. It contains all permutation incidence vectors. We prove all its vertices are such vectors.

Every vertex has an active coordinate bound. Without one, at most one inequality for each triple can be active, giving at most four independent active constraints in six dimensions. Relabel the items, reversing an edge if necessary, so the active bound is y12=1. On this face write

    (a,b,c,d,e) = (y13,y14,y23,y24,y34).

After removing inequalities redundant with the unit box, the constraints are

    0 <= a,b,c,d,e <= 1,
    c <= a, d <= b,
    0 <= a-b+e <= 1,
    0 <= c-d+e <= 1.

The four nontrivial row types are

    r1=(1,0,-1,0,0),
    r2=(0,1,0,-1,0),
    r3=(1,-1,0,0,1),
    r4=(0,0,1,-1,1).

Multiply these rows respectively by (+1,-1,-1,+1). Each column has one +1 and one -1. The resulting matrix is a directed-graph incidence matrix, hence totally unimodular. For completeness, each square subdeterminant is 0 or ±1: if a column has at most one nonzero entry, expand inductively; if every column has two, the row sum is zero and the determinant vanishes. Restoring row signs, appending duplicate negatives, and appending unit-coordinate rows preserve this property. Since all right-hand sides are integral, any nonsingular active subsystem has determinant ±1 and an integral solution. Therefore every vertex of this face, and hence of H, is integral.

A 0/1 tournament satisfying these triangle inequalities has no directed triangle; a tournament without directed triangles is transitive (take a shortest directed cycle to rule out longer cycles). It therefore comes from a permutation. H is exactly LOP4. Centering and polarity give the20 claimed polar vertices.

This proves the input without an exhaustive computation. The author's checker was also rerun successfully: 38,760 bases considered; 8,000 nonsingular; 3,000 feasible bases; 24 distinct integral vertices; each inequality has affine facet dimension5 and12 tight order vertices.

## Pfaffian and equality audit

For the20 polar vertices, the Hodge-star representation is correct, as is B(v,w)=signed determinant of the four vector factors. Every B is0 or±1. Positive adjacency is triangle-free because pairwise disjoint pairs cannot fit in five indices. The signed lift has no4-cycle: different unsigned pairs have at most one common disjoint pair, while opposite signs have no common positive neighbor. Every vertex has three positive neighbors. The coordinate permutation used to reverse Pfaffian sign preserves the order support function.

The weighted graph bound has the correct factor1/4. Equality requires both (i) no negative-interaction pair with positive weights and (ii) a graph maximizer. First-order variations on the positive support give weighted degree1/2. Taking any supported edge partitions the total mass into its two neighbor sets, each of mass1/2; every cross-edge must occur. No4-cycle then makes the support a star. Conversely, leaves of any positive star have intersecting underlying pairs, so their interactions are zero, not negative. This establishes the complete claimed locus.

One can also prove the triangle intersections without linear programming. Average any two convex representations of a common point; their combined support must again be a star. A representation using at least two leaves uniquely determines its center. Thus distinct equality triangles can meet only at representations supported on one edge, i.e. its midpoint. Equal midpoints of different edges would either have a nonstar union of supports or a shared center and unequal remaining endpoints. Hence distinct triangles meet exactly when their centers are adjacent, then only at their common edge midpoint.

A small independent exact symmetry calculation found the same cubic graph with12 five-cycles, and the A5 simplex-relabeling action is transitive on all20 centers, with C3 stabilizer cycling the three leaves. The author has another worker developing the analytic group-action statement; that refinement is not required for the bound or the20-triangle classification.

## Explicit equality simplices and a geometric interpretation

Use interleaved symplectic coordinates with omega=e1*∧e2*+e3*∧e4*. For mu,nu>=0 and mu+nu<=1 set

    u1=(1,0,nu,-nu),
    u2=(0,1,-mu,mu),
    u3=(0,0,1,0),
    u4=(0,0,0,1),
    K_mu,nu=conv(0,u1,u2,u3,u4).

Let U have these columns and X=U^{-T}. Then

    X=(e1,e2,-nu e1+mu e2+e3,nu e1-mu e2+e4),

and the negative barycentric-gradient Gram matrix is

    Omega=e12+e34+mu(e13-e14)+nu(e23-e24).

Thus vol(K)=1/24 and c(K)=1/4 throughout the parameter triangle. Its ten facet-gradient pairing magnitudes are1 four times, mu twice, nu twice, and 1-mu-nu twice. A two-face F_i intersect F_j is Lagrangian exactly when the pairing of its two facet gradients is zero: for a two-plane N, its Euclidean orthogonal complement is Lagrangian iff N is Lagrangian. Consequently parameter-interior simplices have no Lagrangian two-faces; open edges have exactly two; corners have four. This makes the nonstandard equality family geometrically visible.

## Semantics and limitations

The inequality resolves exactly the empirical four-simplex observation in Chaidez–Hutchings §1.5. Their local primary-source introduction explicitly labels the universal bound experimental, suggests a Haim–Kislev proof, and already states that families of nonstandard maximizers exist. Thus a nontrivial equality family alone is not new. The complete proof and complete equality classification are the consequential claims here.

I read the supplied novelty memo but did not independently inspect Leipold's full dissertation or the withdrawn Zediker source. Their source/priority status remains with the source investigator. This review does not establish absence of another proof.

The optional identification of the signed graph as the dodecahedral graph is not needed; either supply a short identification/citation or omit that ornamental assertion. The graph properties actually used are proved directly.
