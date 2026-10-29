# Independent audit of complete simple-word coverage

28 October2026,23:49UTC. Reviewer/implementer: geometry_frontiers. No gap found in the catalogue coverage argument. A separately written exact-field and full-order implementation agrees with the supplied190-chart set, not merely its counts.

## Mathematical coverage

1. Every feasible simple HK word on the ten HKO normals extends to a full permutation by inserting zero-weight normals. Cyclic invariance of the action allows the full permutation to start with label0, giving exactly9!=362880 orders.
2. Each factor's normalized closure set is {u≥0:Σu=1,Σu n=0}. It is a bounded2-dimensional polytope. Its extreme points have support at most3 because the three affine constraint rows have rank3. No support of size1 or2 closes, because there are no zero or antipodal normals. Enumerating the positive solutions on all three-element supports therefore gives **all** vertices. There are exactlyfive in each factor.
3. Every full product weight is (t u,(1−t)v). Same-factor pairings vanish at the base HKO form, so Q=t(1−t)Fσ(u,v), withFσ bilinear. Expressu,v as convex combinations of their five closure vertices. At t=1/2, each pair term is the action of the induced six-normal cyclic word. The exact3000-term check shows each≤Q*, with50 equalities.
4. If Q=Q*>0, the bounds Fσ≤4Q* andt(1−t)≤1/4 force t=1/2. In any such convex decomposition every positively weighted vertex-pair term must equalQ*: otherwise their weighted mean would be strictly smaller. Thus the positive decomposition supports form a complete bipartite subgraph of the5×5 winning-pair graph.
5. Every biclique is contained in a maximal biclique. The producer enumerates these by all nonempty row subsetsR, formsC=N(R), then retainsR=N(C). These are precisely the nonempty inclusion-maximal bicliques: maximality forces both equalities, and either equality closure prevents enlargement. Their products conv{u_i:i∈R}×conv{v_j:j∈C} cover every maximizing weight pair.
6. For each maximal biclique, delete labels outside the union of its triangular supports. Those labels have zero weight for every point in its chart. This deletion preserves action, loop geometry and response. Deduplicating by the cyclic restricted word and the two vertex masks therefore loses no optimizing weight.

The resulting exact chart set has50 vertex×vertex charts on six labels,70 edge×vertex and70 vertex×edge charts on seven labels. No chart contains more than two closure vertices in either factor, and none varies in both factors. The response optimizer can consequently use a single interval parameter for every nontrivial chart. The coverage argument does not depend on calling those pairs geometric edges; the convex hull of two vertices is the required interval in either case.

## Independent execution

`independent_catalogue.py` in this directory uses Sage's number-field implementation forQ(tau), rather than the producer's custom field code. It reconstructs the ten explicit normals and solves every three-normal closure system; it then recomputes the3000 six-word actions and verifies exact equality of the50 winning words with the producer.

For the full scan it uses Python permutations and independently written maximal-rectangle logic, rather than the producer's C++ Galois-closure implementation. It visits all362880 cyclic orders, builds their induced primitive graphs, enumerates all row rectangles, discards those contained in a larger rectangle, and builds the chart set after zero-label removal. Exact set comparison against `catalogue.json` passes.

Independent counts:

-3000 primitive words;50 winners;
-362880 full cyclic orders;79494 containing maximizers;
-121 nonempty winning graphs;
-190 charts:50 support6,140 support7.

The run completed in13.03seconds. Output: `independent-catalogue-summary.json`.

## Scope and interfaces

The separate `repeated-word-elimination.md`, independently checked in `repeated-word-review.md`, reduces the selected flow-invariant convex radial profile from all repeated maximizing words to these simple ones. It is necessary for the global orbit-response conclusion; the catalogue alone does not prove that arbitrary minimizing loops are simple.

The present audit establishes complete simple-word/weight coverage and independently checks all primitive arithmetic. It does not independently reproduce the190-chart piecewise-quadratic response optimization; multicut_variation/derivative_review owns that audit. I read its formula and found the expected construction: affine KKT loop vertices, flow-constant hinge values, all hinge breakpoints, interval endpoints and all interior concave-quadratic stationary maxima.

The original710seed-family catalogue is not used as completeness evidence here.
