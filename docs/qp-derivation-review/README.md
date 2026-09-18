# Independent check of the proposed QP derivation

18 September 2026. **No logical gap found in the two capacity inequalities or
in the following global-maximizer realization proposition**, relative to the
explicitly imported nonsmooth dual-action principle. The assembled
simple-minimizer argument was also read and does not use the QP formula it is
later used to derive. No source patch is necessary for this scoped result.

This is a mathematical source review, not acceptance of chapter prose, a
runtime check, or a new audit of all later product-reduction theorems.

## Inputs and applicability

The checked proposed source is
[`../qp-flow-integration/proposed/04-quadratic-program.tex`](../qp-flow-integration/proposed/04-quadratic-program.tex)
from commit `c69a4ee8`. Its real prerequisites are the interrupted production
files under `/tmp/msc-math-thesis-review-20260917/thesis/candidate`, not the
older frozen chapter selected by this research checkout's default build.
Their exact hashes are in `inputs.json`.

The proposed main selects the recovered preliminaries, whose final input is
`recovered/thesis-candidate/02-duality.tex`, and the new top-level
`03-generalized-reeb-orbits-polytopes.tex`. These are the foundations reviewed.
Switching to a different preliminaries overlay requires checking that the same
normalizations and imported theorem contracts survive.

The normalized rows must be actual irredundant facets of a bounded,
full-dimensional convex polytope containing zero in its interior. Irredundancy
is stated in the orbit chapter and inherited by the QP chapter's “written as in”
reference. It matters for identifying facet labels after reconstruction.

## Independent derivation

With J = [[0,-I],[I,0]], omega(u,v) = <Ju,v> and R_i = 2Ja_i,

- -JR_i = 2a_i;
- omega(R_i,R_j) = 4 omega(a_i,a_j);
- h_K(a_i) = 1 because the normalized row defines an attained facet;
- H_K*(y) = h_K(y)^2/4, hence H_K*(-JR_i) = 1.

The conjugate identity remains valid without central symmetry: zero in the
interior makes h_K(y) nonnegative, so maximizing r h_K(y) - r^2 over r >= 0
indeed gives h_K(y)^2/4. No absolute value or symmetry assumption is missing.

For a closed piecewise affine curve with displacements d_k = ell_k w_k,
direct integration of lambda = (q dp-p dq)/2 gives

    2 A(z) = sum_{j<k} omega(d_j,d_k).

The initial-point contribution is zero by closure; the within-segment term is
zero by skew-symmetry. This fixes the earlier-first sign rather than importing
an ordering convention by analogy with HK.

For feasible beta with q = Q_sigma(beta) > 0, use durations T beta_k with
T = 1/(2q). Closure follows from sum beta_k a_sigma(k) = 0. Directly,

    A(z) = 2 T^2 q = T,       I_K(z) = T sum beta_k = T.

Thus z is feasible for the actual free-period dual problem A(z)=T, regardless
of whether its image can be translated into K. The dual infimum gives
c <= 1/(2q). Since c and q are positive, multiplying by 2cq gives
q <= 1/(2c). Nonpositive q satisfy the same upper bound without any attempted
zero/negative period construction. Positivity of c follows from the ordinary
capacity normalization/monotonicity for a body containing a ball.

For the reverse bound, the supplied simple minimizer has action and period c,
distinct pure facet directions, and positive durations tau_k summing to c.
Set beta_k = tau_k/c. Closure gives feasibility, and the same integration gives

    c = A(gamma) = 2 c^2 Q_sigma(beta),
    Q_sigma(beta) = 1/(2c) > 0.

This supplies positivity and the lower bound at once. The finite family of
words and closed subsets of the simplex give compact feasible sets, so there
is also no unattained-supremum issue. Equivalently the witness already attains
the bound proved for every candidate.

As a normalization cross-check, the proposed cube example has word
(e1,e3,-e1,-e3), weights (x,y,x,y), Q=2xy. At x=y=1/4,
Q=1/8, T=4, and the velocity rectangle has side lengths 2 and action 4.
The later-first objective would have the opposite sign for this named word.

## No circular realization step

The first inequality needs only dual feasibility, not boundary placement.
The second needs an independently available simple minimizer, not realization
of an arbitrary feasible word. Only after Q_max = 1/(2c) is proved does the
realization proposition identify a global optimizer as a dual minimizer.

For that optimizer I=T, so the reconstructed multiplier nu=I/T is one.
The formula gamma(t)=sqrt(nu) z(t/nu)+b/sqrt(nu) therefore reduces to a
translation. On a segment with velocity R_i, the inclusion reads

    a_i in conv{a_j : F_j active at gamma}.

An irredundant normalized row is an extreme point of the polar. It cannot be a
convex combination of other active rows, so F_i is active. This justifies the
preserved facet word; closure alone would not. Zero-duration entries are
removed and distinctness is preserved. “Simple” here has the expressly stated
facet-direction meaning, not a newly asserted topological embedding property.

## Foundation chain checked

The simple-minimizer proof starts from a capacity-realizing generalized
characteristic provided by duality. Its approximation, splitting and merging
steps use the independently proved shoelace identity. Pure velocities keep
I=T; duration rescaling by T/A gives action, period and dual value T^2/A.
Minimality forces A<=T, while approximation gives liminf A>=T. The finite
word/dwell-time compactness step then yields a minimizing pure loop and dual
reconstruction places it on the boundary. None of these steps invokes the QP
formula or its global-maximizer realization proposition.

The actual duality text has consistent period conversion: for A(z)=T,
w(s)=z(Ts)/sqrt(2T) has A(w)=1/2 and I(w)=I(z)/2. Translation removes the mean
without changing either quantity. This agrees with the fixed-period
nonsmooth formulation in the local primary source
`papers/hk2017/EHZ-polytopes.tex`, lemma `dual_bijection_lemma`, which gives
A(gamma)=2I(w). Its theorem `simple_loop_theorem` supplies individual facet
normal directions, each once; positive reparametrization fixes their speeds
at 2Ja_i. I read those primary-source statements, not merely the prior scout.

The previously closed scalar HK convention check is retained at
`/workspaces/msc-math/.worktrees/pentagon-affine-integration/formal/pentagon-affine-products/source-contract.md`.
It agrees with this independent internal derivation: height-weight conversion
has no extra factor, and word reversal reconciles the source's later-first
formula. Scalar formula agreement is a consistency check, not a premise in
either inequality above.

## Remaining scope

The nonsmooth existence/multiplier theorem remains an explicitly imported
foundation; this review does not replace its published proof. It checked the
normalization conversion and local reconstruction arithmetic needed here.
The subsequent billiard/product-family arguments, computational exclusions,
Rust correspondence and prose quality retain their separate owners. No
capacity computation, source replacement, or production build was performed.
