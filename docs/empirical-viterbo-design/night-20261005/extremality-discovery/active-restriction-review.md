# Independent bounded review: sparse active-pair hull preservation

2026-10-04 23:00 UTC. Reviewer: `conditional_mechanism`, receiving owners:
`extremality_discovery_lead` and `scientific_compare_initial`.

Reviewed only the implication in
`../geometry-discovery/active-restriction.md` that every globally active HK
weight lies in the convex hull of globally active closure-vertex pairs, and
its interface to the fixed-normal balance target. Read the existing
`formal/product-qp-six-facet-reduction.tex` to check the precise word/face
conventions. No experiment, formal-source edit, or thesis promotion.

**Verdict: accepted on the stated domain; no material mathematical gap found.**
The extension preserves the convex hull of the entire maximizing weight set,
including dense and singular weight families. It does not classify every
physical minimizing trajectory.

## Independent reconstruction of the implication

Take nondegenerate planar factors with zero in their interiors, actual dual
facet vertices, and the complete distinct-label HK family in the project's
audited orientation. Let `M > 0` be its global maximum. The complete-feasibility
bound used below is supplied by [Haim–Kislev, Theorem 1.1](https://arxiv.org/html/1712.03494v3).
Theorem 1.5 is not needed to prove this algebraic hull statement.

1. For any active \((\sigma,\beta)\), the block closure equations split into
   q- and p-factor closure. If one factor has zero mass, all nonzero weighted
   rows lie in one Lagrangian subspace and the objective is zero. Thus both
   masses are positive. Write \(\beta=(t x,(1-t)y)\) with \(x\in C_Q\),
   \(y\in C_P\). Same-factor pairings vanish, giving
   \(Q_\sigma(\beta)=t(1-t)B_\sigma(x,y)\). Activity makes `B` positive. Holding
   `x`, `y`, and the word fixed while varying `t` preserves all HK constraints;
   unless `t = 1/2`, the feasible choice `t = 1/2` would exceed `M`.
   Consequently **every** active HK weight has masses `1/2`, not merely one
   sparse selected maximizer.

2. Restrict each closure polytope to the support of `x` or `y`. Setting
   nonnegative coordinates to zero defines a face. Every vertex of that
   restricted polytope is therefore a vertex of the full factor polytope.
   Finite convex decompositions \(x=\sum_a\lambda_a x_a\) and
   \(y=\sum_b\mu_b y_b\) exist, and every positive-coefficient vertex remains
   supported within the original corresponding support.

3. Keep the same full padded permutation \(\sigma\). Each
   \((x_a/2,y_b/2)\) satisfies nonnegativity, global normalization, and both
   block closure equations. It is a feasible **complete** HK datum regardless
   of whether a particular implementation's block enumerator would retain
   that word. Thus every score is at most `M`. Bilinearity gives

   \[
   M=\sum_{a,b}\lambda_a\mu_b
           Q_\sigma(x_a/2,y_b/2),\qquad
   \sum_{a,b}\lambda_a\mu_b=1.
   \]

   A weighted average of numbers at most `M` equals `M` only when every
   positive-coefficient term equals `M`. Hence all contributing vertex pairs
   are globally active **for this same padded order**.

4. Delete zero positions from that order. Every deleted position contributes
   zero to normalization, closure, and the pairwise objective; the relative
   order of retained labels is unchanged. The resulting distinct-facet word
   is admitted by the complete HK family and has the same score. This proves
   that each contributing pair meets the definition of `E`. Finally,

   \[
   \beta=\sum_{a,b}\lambda_a\mu_b(x_a/2,y_b/2).
   \]

   This establishes \(B\subset\operatorname{conv}E_{1/2}\); the reverse
   convex-hull inclusion holds because every element of \(E_{1/2}\) is itself
   an active HK weight. The claimed hull equality follows.

No general position, uniqueness, reduced-Hessian definiteness, strict gap
between optimizer families, or smooth branch model enters this argument. The
support bound is genuinely factorwise: three scalar factor equations imply at
most three positive coordinates per closure vertex. The conclusion does not
make every active weight sparse.

## Check of the finite balance interface

At the fixed base polytope, positive diagonal rescaling relates the two
conventions:
\(\beta_i=h_i\alpha_i\). This is the same rescaling as in
`formal/active-orbit-facet-coverage.tex`. Therefore the normal-weight balance
target \(S/(4V)\) becomes the dual-weight target
\(\beta_i^*=h_iS_i/(4V)\), and convex-hull membership is preserved.

For a q-facet of a planar product, \(S_i=\ell_i A_P\), so
\(\beta_i^*=h_i\ell_i/(4A_Q)=\tau_i^Q/2\); the p-block calculation is
identical. Applying the hull identity and multiplying both blocks by two
gives exactly

\[
 (\tau^Q,\tau^P)\in\operatorname{conv}\{(x,y):(x,y)\in E\}.
\]

One common probability distribution on the active pair set is required for
both marginals. Separate feasibility in the two projected hulls would not
establish this statement. Positive edge lengths and support numbers make each
target component positive. Since each vertex pair covers at most three facets
of each factor, a unique active pair is indeed impossible at a support-chart
local maximum if either factor has more than three facets.

## Accepted boundary and remaining dependency

The hull proposition is accepted for the **globally maximizing HK weights in
the complete finite-word QP** of the stated planar Lagrangian product. It does
not transfer automatically to a floating-point winner list, a pruned word
family without completeness proof, non-Lagrangian factors, merely stationary
KKT witnesses, or dwell distributions of arbitrary generalized minimizing
characteristics with repeated labels/mixed facet velocities.

The local-maximality implication additionally relies on the existing
fixed-normal active-weight balance theorem, currently marked agent-proved
awaiting Jörn review. That inherited status is already disclosed in the
reviewed note. The new hull step itself introduces no smooth-envelope
assumption and does not inherit the regularity limitation of the separate
curvature–mobility probe.

This review did not audit the numerical examples or a producer implementation;
neither is needed for the accepted implication. The author may retain the
claimed exact hull preservation and finite necessary balance interface with
these boundaries.
