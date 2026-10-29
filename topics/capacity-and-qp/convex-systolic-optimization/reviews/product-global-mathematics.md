# Independent audit: every partner of a corner-truncated parallelogram

Auditor `/root/truncated_conjecture_audit`, 28 October 2026. Parent `/root` owns integration, commits and cleanup. Source checked: BMP arXiv:2603.12495v2, Definition 3.2, Theorem 3.3, Lemma 4.3 (the actual triangulation-cover statement), Lemma 7.1 (including its proof), and the product-capacity symmetry in Section 2.2. Author's exact weak proof `/tmp/msc-math/restart/product-extremizers/truncated-global-section.tex` read and approved at 23:34 UTC. This audit supplements the earlier counterexample audit, not a proof of optimality of the newly proposed covers.

## Verdict

The proposed weak universal Viterbo inequality is correct. The complete written weak proof was checked, not just its outline. There is also a valid strictness route below, avoiding the source's omitted quadrilateral equality case analysis. It proves the strict inequality for every full-dimensional convex partner.

Throughout, 0<A,B<1, g=A+B-1>0, Q=[0,1]^2 intersect {Ax+By<=1}, and c(K x_L Q)=1. Write C=AB/(2AB-g^2). The complete four-template list T,-T,H,V and all normalization conventions were independently established in the earlier audit. A corner-truncated parallelogram is affinely equivalent to this Q; the inverse-transpose map on the partner preserves the capacity and product of areas.

## Weak proof checks

1. **Lattice covering really follows.** Split each A-by-B lattice rectangle along its rising diagonal. The two cells are T and a translate of -T. Thus every triangular cell fits by translation into K. BMP Lemma 4.3 says every translate of K meets this triangulation's vertex set, which is exactly what is needed to deduce K+Lambda=R^2, Lambda=A Z e1+B Z e2. No orientation or common-translation requirement is missing.

2. **Rectangle branch.** If K contains an axis A-by-B rectangle, take support-maximizing points in the four outward coordinate directions. The caps based on the four rectangle sides have disjoint interiors by BMP Lemma 7.1. The lemma's polygonal hypothesis causes no gap: apply it to the convex hull of the rectangle and those four points, inside K. Since the two unit segments imply coordinate widths at least 1,

   |K| >= AB+B(width_x-A)/2+A(width_y-B)/2 >= (A+B)/2.

   This is strictly larger than C, because the difference is

   g[A(1-A)+B(1-B)]/[2(2AB-g^2)] > 0.

3. **Forest branch.** If no such rectangle fits, every shifted induced lattice graph is acyclic. Any graph cycle contains a simple rectilinear grid cycle. Its bounded interior contains an elementary grid cell, and the closure of that cell lies in the convex hull of the cycle vertices, hence in K-t. This contradicts the branch hypothesis. Covering gives at least one vertex for every t, including boundary shifts; hence E_x(t)+E_y(t)<=N(t)-1. Graphs are finite because K is compact.

4. **Periodization.** Integrating over a half-open fundamental rectangle gives integral N=|K| and integral E_x=|K intersect (K+Ae1)|, likewise vertically. One may obtain the minus translation first; its area equals the plus-translation overlap area by translation. Boundaries are null sets and do not affect the integrals. Consequently

   |K|-AB >= |K intersect (K+Ae1)|+|K intersect (K+Be2)|.

5. **Homothetic overlaps.** If [u,v] is the unit horizontal segment with v-u=e1, then (1-A)K+A v lies in K and, after subtracting Ae1, also lies in K. Therefore it lies in K intersect (K+Ae1). Its area is (1-A)^2|K|. The corresponding vertical inclusion has factor (1-B)^2. Combining these gives

   [1-(1-A)^2-(1-B)^2]|K| >= AB.

   The coefficient is 2AB-g^2>0, so |K|>=C. Since |Q|=(2AB-g^2)/(2AB), this is precisely the desired weak capacity-volume bound.

## Strictness: elementary equality analysis

Suppose equality |K|=C holds. The rectangle branch is already strict, so all equalities below occur in the forest branch.

### First reduce to a polygon

Choose placements of the four templates and let P be the convex hull of their ten vertices. Then P is a cover and P subset K. The established weak inequality gives |P|>=C=|K|, so P=K: two full-dimensional nested compact convex bodies of equal area coincide. Thus K is a polygon. This avoids requiring any smoothness or polygonal assumption on the original partner.

Both overlap lower bounds must be equalities, since their sum is bounded above by |K|-AB, which equals [(1-A)^2+(1-B)^2]|K|. In particular,

K intersect (K+Ae1)=(1-A)K+A v,

where v is the right endpoint of the chosen horizontal unit segment [u,v]. Equality of areas upgrades the homothetic inclusion to equality of convex sets.

### Facets must meet the unit-segment endpoints

For every genuine facet normal n of K, the homothetic polygon on the right has a genuine facet with normal n. Thus the corresponding inequality of the intersection on the left is active, with support value

h_K(n)+A min(0,n dot e1).

Equating support values yields

h_K(n)=n dot v-min(0,n dot e1)=max(n dot u,n dot v).

Hence every facet line contains u or v. A point of a convex polygon lies on at most two genuine facets, so K has at most four facets. The same statement holds for the endpoints of the unit vertical segment.

If K has four facets, u and v must be distinct opposite vertices: each must account for two of the four facets, and adjacent vertices would account for only three. The vertical endpoints likewise form a pair of opposite vertices. These are the two different diagonals, of vectors e1 and e2. Therefore |K|=|det(e1,e2)|/2=1/2. But C>1/2 because g>0. This excludes the quadrilateral case. Consequently K must be a triangle.

This argument does NOT assert that unit segments in a triangle must be edges. That tempting shortcut is false: a unit maximal chord may join a vertex to an interior point of its opposite edge. The valid four-facet counting argument above is only used in the quadrilateral case.

### A triangular equality partner would make Q a translational tile

Use c(K x_L Q)=c(Q x_L K), as stated in BMP Section 2.2. Write K=t+M Delta, where Delta=conv{0,e1,e2} and M is invertible. The paired inverse-transpose transformation in the two factor planes reduces the swapped product to Q' x_L Delta with Q'=M^T Q. Its capacity is 1 and

|Q'|=|det M| |Q|=2|K||Q|=1.

The unit normal templates for Delta are the two right triangles that split a unit square along its rising diagonal. Applying BMP Lemma 4.3 again gives Q'+Z^2=R^2. Integration of the covering multiplicity over a unit square, whose integral is |Q'|=1, forces multiplicity 1 almost everywhere. The interiors of distinct translates are therefore disjoint: any nonempty intersection of two interiors would be open and have positive area. Thus Q' is a convex lattice tile.

A convex polygon which tiles the plane by translations must have an edge parallel and oppositely oriented to each of its edges. To see the needed local fact, choose a relative-interior point of an edge avoiding the locally finite set of translated vertices and transverse edge intersections. Coverage just outside the original tile forces a neighboring tile across this point. Disjoint interiors force its boundary edge there to be parallel and oppositely oriented. All tiles are translates of the same polygon, so that opposite edge must occur in the polygon itself.

Our Q has two horizontal edges, two vertical edges, and a single oblique truncation edge. There is no opposite parallel edge for that last edge. This property is preserved by the invertible map M^T. Thus Q' cannot be a translational tile, a contradiction.

Hence equality is impossible, and every partner satisfies c(K x_L Q)^2 < 2|K||Q|.

## Dependencies and limits

The only imported mathematical mechanisms are the checked BMP normal-template/capacity theorem, triangulation-cover lemma, cap lemma, and factor symmetry. No use of BMP's unproved-in-text quadrilateral equality classification is needed for the strict upgrade. The weak proof's graph integration and the strict equality reduction are independent elementary arguments. This proves the strict inequality half of BMP Conjecture 6.1 while the separately audited cover family disproves its proposed optimizer. It does not identify the true optimal partner or quantitative gap.
