# Geometry scout: compact inscribed pentagon core

Read-only bounded source scout, 17 October 2026; no numerical target labels read and no capacity/data analysis run. Sources inspected: README.md, OWNER-NOTES.md, patterns/OWNER-FOLLOWUP.md, patterns/upper-tail/ROUTE-TRIAGE.md, AGENTS.md.commentary.md, collaboration-rationale.md, topics/products/README.md and theorem references in pentagon-affine-products/research.tex. Current source dispositions supersede old discovery ranking.

## Three mechanisms and selection

1. **Circumscribed affinely regular pentagon surrogate.** Attractive because the exact affine-pentagon capacity theorem yields a rigorous upper bound and retains HKO. But it has an exact obstruction: affine regular pentagon products have capacity equal to the width value. Since width is monotone under containment, every such outer template has capacity at least the width of the original pair. Taking its minimum with width can never improve width. Reject before numerical analysis. The rubric 'rigorous and exact on HKO' would reward a useless upper bound here.
2. **Inscribed affinely regular pentagon core (recommended).** A lower surrogate measuring how much of the exact maximal family fits inside each factor. Uses positions, affine shape and relative alignment lost by unsigned ridge lists. Exact affine regular factors, including HKO, are reproduced; generic near-HKO cores have a mechanism for large ratios by monotonicity. A few finite geometry-selected orientations suffice, with no labels, regression, closure-support enumeration or tuned thresholds. Core coverage may be poor for triangle-like shapes; that is a meaningful bulk failure rather than grounds to tune.
3. **Minkowski asymmetry correction to width.** Compact, translation invariant and computationally cheap, with triangle and symmetric controls. But a scalar asymmetry cannot distinguish relative factor alignment, and any correction strictly below one at pentagon asymmetry destroys the small HKO excess. Thresholds calibrated to pentagon controls would risk inventing the desired result. Reject relative to a template with an actual monotonicity mechanism. The rubric 'cheap invariant with known endpoints' rewards this information deficiency.

Predictive relevance favors (2); proof meaning favors (1)/(2), with the exact obstruction eliminating (1); compute favors (2)/(3), with no endpoint fitting needed for (2). This is not an external novelty claim. I did not find the finite inscribed-core predictor in the inspected source account.

## Frozen implementable formula

For a planar factor with cyclic vertices x_i, let m=mean_i x_i and C=(2/n) sum_i (x_i-m)(x_i-m)^T. Let B be its positive square root and y_i=B^{-1}(x_i-m). Let P be the regular unit-circumradius pentagon with vertices v_j=(cos(2pi j/5),sin(2pi j/5)).

Candidate phases are exactly arg(y_i), reduced modulo 2pi/5. For each phase theta, let

s(theta)=min_facets k [ h_k / max_j (a_k dot R_theta v_j) ],

where a_k dot y <= h_k are the facet inequalities of the whitened polygon. All denominators are positive because P contains zero. Choose the candidate with largest s(theta), breaking ties by smallest reduced phase. Define the core P_K=m+G_K P with G_K=s(theta) B R_theta. It lies in K by direct finite inequalities. This is O(n^2*5) with at most six phase candidates in the requested retained families; no nonlinear solver is needed.

For regular-pentagon outward normals n_j (consistent with the chosen vertices), the existing exact theorem gives

c_core=(1+cos(pi/5))^2 / max_ij |n_i^T G_K^{-1} G_T^{-T} n_j|.

Use sys_core=c_core^2/(2 area(K) area(T)) as the frozen ratio predictor, and report capacity errors separately. The formula is a monotonic lower bound in exact arithmetic; numerical finite-containment checks and arithmetic tolerance must be stated. Compare absolute prediction errors with width, which is an upper bound. These are distinct estimators, not comparable one-sided guarantees.

For an affine regular pentagon, C=G G^T and B^{-1}G is orthogonal; each whitened vertex phase recovers exactly its regular vertex set, hence s=1 and the fitted core equals the input factor. Therefore the HKO control is retained at (3+sqrt(5))/5>1 without fitted constants. This survives arbitrary independent affine maps in exact arithmetic. For random upper tails, a large predicted ratio has a concrete interpretation: a near-HKO affine pentagon product already fits in the body with little area loss. A small predictor despite a large true ratio falsifies the claim that this compact pentagon core explains that tail.

No support additions, orientation-grid refinement, later phase alternatives, error-dependent tuning or target-based selection are part of this proposal. Root owns method selection, implementation, verification, retention and commit; standing owner owns shared integration.
