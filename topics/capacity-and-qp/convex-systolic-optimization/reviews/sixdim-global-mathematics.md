# Independent audit of the sharp global six-simplex theorem

Auditor: `/root/sixdim_global_math_audit`. Date: 2026-10-28, 23:39–23:48 UTC.
Receiving/integration owner: `/root`; mathematical author: `/root/combinatorial_frontiers`.

## Verdict

**PASS, as an exact computer-assisted theorem.** The assembled argument establishes

    max_{S6(Omega)<=1} |Pf(Omega)| = (63+5 sqrt(5))/1922,

and therefore, for every nondegenerate six-dimensional simplex K,

    c_EHZ(K)^3 / (6 Vol6(K)) <= (945+75 sqrt(5))/1922.

Equality is one unlabeled simplex class modulo translations, positive homotheties, and linear symplectomorphisms, represented by

    Omega* = a(t135-t246)+b(e12+e34+e56),
    a=(13+3sqrt(5))/62, b=(6-sqrt(5))/31.

This is a global conclusion, stronger than the previously audited strict-local theorem. No unresolved mathematical implication remains in the chain audited here. The completeness of the 35-constraint order tangent cone is established computationally by two agreeing exact polyhedral backends; this report does not call that a backend-free handwritten classification or a formal machine proof. Novelty, bibliographic priority, final integrated prose/PDF and human acceptance are outside this audit.

Frozen copies of the assembled proof and optional LaTeX module are `reviewed-assembly.md` and `reviewed-extension.tex` in this directory. They agree with the argument below. The LaTeX module still needs retained-code references in the eventual paper; no new mathematical result beyond this audited chain is introduced there.

## Sources inspected and independently checked

- Author's `sixdim-global-assembly.md`, `sixdim-global-extension.tex`, `rank-order/global-signed/conv70-proof.md`, and its complete checker source.
- Author's `rank-order/pyramid-audit/REPORT.md` and complete checker source.
- Coverage worker's `PROOF.md`, `full_classification.py`, `dual_coverage.py`, and exact result JSONs.
- Frozen local theorem `sixdim-ready-20261028-2315.tex`, plus the separate prior audit `/tmp/msc-math/restart/sixdim-audit/audit.md`.
- Project HK normalization owner `topics/capacity-and-qp/proofs/hk2017-qp-conventions.tex`.

My `independent_algebra.py` imports none of the author's code. It computes triple coefficients as determinants of six edge vectors, separately checks them against a perfect-matching Pfaffian expansion, enumerates maximal compatible supports by increasing-index recursion rather than Bron–Kerbosch, reconstructs the pyramid quadratic, and checks the two equality sign-changing relabelings. Its output is `independent_algebra.out`.

I also replayed the complete coverage scripts in this directory with output destinations redirected here, preserving the author's outputs. `coverage_replay.out` and the two local JSON files retain the exact replay. Both backends gave 186 and 18 tangent rays, and every orbit/endpoint/score predicate passed. This replay checks execution, while the scrutiny below supplies the missing logical implications that execution alone would not establish.

## 1. Signed mass merging and packing classification

For each triangle T, the associated two-form is decomposable. If T and U share two labels, choose their common edge as a wedge factor: b_T wedge b_U=0. Consequently no cubic coefficient can involve both variables. A form's own square vanishes as well.

After antipodal cancellation, a decomposition has coefficient mass L<=1. At a nonzero maximum L must equal one: if L<1 then Omega/L is still in D and has strictly larger absolute Pfaffian. Choose a maximizer with minimum positive support. Transferring fixed-sign mass between incompatible types makes Pfaffian affine in the transfer parameter; convexity of its absolute value permits an endpoint without lowering the objective. That endpoint removes a support type. Thus a sparse maximizer has linear triple support. No unsupported positivity assumption on signed coefficients is used.

The human classification is valid:

- A degree-zero label would require every pair on the remaining six labels to be covered; the equation 2d=5 is impossible.
- For a degree-one label v in vab, all pairs on the other four labels R must be covered. Without an R-contained block, six other blocks are required, saturating all 21 pairs and contradicting degree one. With an R-contained block xyz, the remaining point w needs three separate blocks to cover wx,wy,wz; each needs a or b, repeating wa or wb. Neither argument assumes an unproved maximal-packing list.
- Degrees lie in {2,3}, so at least five blocks occur. Six blocks leave a simple three-edge graph with all degrees even, hence a triangle, which could be added. Five blocks force degree sequence (3,2,2,2,2,2,2), with the advertised matching-plus-bipartition structure. Seven blocks form the unique order-seven Steiner triple system.

For three blocks of a linear family, union size <=6 places their forms in a difference space of dimension <=5, so the determinant coefficient vanishes. Union size seven forces the six selected edge vectors to form a spanning tree: a disconnected three-block linear family would use at least eight labels. A spanning-tree reduced incidence determinant has magnitude one. This justifies the claimed cubic support and coefficient magnitudes without relying on the coefficient census.

Independent exact output: 6,545 triple coefficients, 5,810 zero, 392 positive, 343 negative; maximal packings consist of 420 five-block and 30 seven-block configurations. All five-block support polynomials have central-pair/outer-triple shape.

For F32, AM–GM gives G(t)=t^3/27+t(1-t)^2/4. Its endpoint and two critical values give the unique maximum at t=(18-3sqrt(5))/31, with all five weights positive. For Fano, deleting a zero variable leaves four terms transversal to three pairs, so the boundary value is <=1/27. At an interior maximum, Euler/Lagrange gives each derivative=3P; their sum is the sum of all pair products, implying 21P<=3/7 and P<=1/49. Thus the Fano case is strictly smaller than q*.

## 2. The polyhedral coverage and the simultaneous-cut issue

Q=D polar is full-dimensional and bounded: it includes a neighborhood of zero and its constraints include all coordinate bounds. The matrices representing the six adjacent label transpositions have determinant +/-1, preserve all 70 inequalities, and act contragrediently on Q. The two generated feasible orbits have sizes 720 and 840 and are disjoint; active rank 15 proves all candidate points are actual vertices.

At each representative, the tangent cone is exactly the cone defined by the active inequalities. Its extreme rays are the incident edge directions. The exact first blocking constraint on each ray gives its actual neighbor in Q. The complete computed ray sets land entirely in the candidate set. Symmetry transports this to every candidate, so that set is closed under all graph neighbors. Connectedness of a bounded polytope's graph then proves that no additional vertex exists. This is a valid completeness argument, not an extrapolation from sampled vertices.

The 840 primal fence forms and 840 exceptional dual vertices have a unique matched violating pair at value 6/5; all other exceptional pairings are <=4/5. More explicitly, the unique-match map is equivariant and surjective between two transitive orbits of equal cardinality, hence bijective. Orders satisfy every fence cut.

The sequential-cut induction is essential and is now present in both proof accounts. At a remaining exceptional vertex p_j, all earlier cuts are strict, so its local tangent cone is unchanged. Every original segment to its 18 order neighbors survives because both endpoints satisfy every earlier cut. Since the polytope only shrinks, those ray endpoints cannot extend. Thus the full incident-edge list remains unchanged until the cut matched to p_j. Its cutting hyperplane meets those 18 crossing edges exactly at existing order vertices. A halfspace cut can create new vertices only on crossing edges, so it deletes p_j and creates none. Induction gives the order polytope after all cuts; polarity gives C=conv(D,840 fences).

For the union-of-pyramids bridge, every pair of distinct fence apices has midpoint in D, tested on all classified Q vertices. Replacing equal masses of two apices by their midpoint reduces the number of positive fence weights at each step, so finitely many steps leave at most one fence. A fixed fence violates just one D facet and satisfies all others, so its hull with D is D plus the pyramid over that facet. This supplies the whole cover used by the analytic bound.

One genuine wording error was found and repaired by the coverage owner: a fence does NOT strictly satisfy every other D facet; it has equality on 18 order facets. The correct fact is strict satisfaction of the other exceptional facets and weak satisfaction of the order facets. The proof needs only the unique violated facet and survives unchanged.

## 3. Pyramid cubic and strictness

My perfect-matching reconstruction independently gives Pf(A)=4 and the linear coefficient 2/25 for Pf(tA/5+sX). For the 18 active signed rank-two forms, the quadratic coefficient is x^T B x/10. Its entry census is {0:138,1:162,2:24}. The entries equal to two form six disjoint P3 paths. Each within-path block of J-B is exactly the rank-one square (e_center-e_leaf1-e_leaf2)(...)^T. Every cross-path entry is nonnegative. Therefore 0<=Q(X)<=1/10 for all nonnegative coefficients of total mass one, even when the representation of X is not unique.

The absolute-value inequality is legitimate because the first three coefficients are nonnegative; only the final Pf(X) may change sign, and its magnitude is <=q*. Coefficientwise comparison with q*(t+s)^3 is strict for t>0: 4/125<q*, 2/25<3q*, and 1/10<3q*. The t^3 term guarantees strictness also when s=0. With t+s=1 this excludes every point outside D from the global equality locus. The S7 action preserves absolute Pfaffian, so the canonical inequality covers every pyramid.

## 4. Equality: why sparse existence really gives a full classification

The minimal-support argument alone would not classify all optimizers. The additional strict-local argument does so.

Start from any global maximizer. Each mass transfer starts at an interior split whose absolute affine Pfaffian already attains the global upper bound. Such an affine function must be constant throughout the split. Thus every transfer traces a segment of global maxima; if its endpoints represent the same form, that segment is simply constant in form space. After finitely many steps the support is linear. It cannot be Fano because 1/27<q*. F32 equality forces all five golden weights, so no lower-support endpoint occurs.

Relative to the canonical positive F32 polynomial, equality in the triangle inequality forces the two central signs to coincide and the three outer signs to coincide. There are exactly four patterns. The permutation (1 3)(2 4) negates both triangle generators and permutes two coordinate generators; (1 2)(3 4)(5 6) negates the whole canonical weighted form. My checker verifies both identities. Together with arbitrary relabeling of the unsigned packing, all endpoints are S7 images of Omega*.

The existing strict-local theorem holds in the FULL C, not only its five-vertex face. Its positive combination of every active order normal forces zero first derivative precisely in the four-dimensional face tangent; its Hessian is negative definite there. I re-read the exact frozen proof, including the limiting-direction Taylor argument, and found no hidden uniform first-order-gap assumption.

A nonconstant segment of global maximizers cannot have a strict local maximizer as endpoint. Working backward along the finite merging path forces every earlier form to equal that endpoint. Hence ALL normalized maximizers, not merely sparse ones, lie in the S7 orbit of Omega*.

## 5. Geometric normalization and equivalence

Let lambda_1,...,lambda_7 be the simplex barycentric functions and g_i their gradients. They sum to zero. For an interior origin, the outward normalized facet rows are -g_i/lambda_i(0), and the unique normalized closure weights are lambda_i(0). Thus the height factors cancel in the HK objective, leaving ordered pairings omega(g_i,g_j). Cyclic invariance lets label 7 be last, and its contribution is omega(sum_{i<=6}g_i,g_7)=0. Therefore the objective is exactly S6, and capacity is 1/(2S6).

The affine barycentric coordinate map has Jacobian determinant det(g_1,...,g_6), while its image is the standard coordinate simplex of volume 1/6!. With Omega_ij=omega(g_i,g_j), the absolute Pfaffian is that absolute determinant. Consequently Vol=1/(720|Pf Omega|), and the ratio is 15|Pf Omega|/S6^3. Every nonsingular alternating form arises as such a symplectic Gram matrix, hence from some simplex.

A positive physical homothety multiplies Omega by a positive scalar, so normalizing by S6 correctly removes scale. Equal normalized Gram matrices give a symplectic map between gradient bases and hence an affine symplectic equivalence of the corresponding simplices after translation. The entire S7 orbit merely relabels barycentric gradients. Because -Omega* is already in this orbit, there is no extra antisymplectic equality class.

## Delivery and retention boundary

The theorem can now replace the local-only scope statement, provided the integrated writeup retains the complete finite coverage mechanism and points readers to the retained code. The proof is not supported by a bare count of known linear-ordering facets, nor by numerical optimizer agreement. Root owns retention, source references, final prose/PDF checks, integration and cleanup. No repository files were edited by this audit.
