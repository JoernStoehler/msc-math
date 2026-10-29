# Independent audit: simultaneous splitting derivative and finite HKO banks

Auditor: simplex_independent_audit. 2026-10-28, approximately23:00–23:14 UTC. Read the complete current `theorem.md`, `finite-bank-obstruction.md`, prior internal derivative review, actual Haim–Kislev primary-source formula/dual principle, and retained accepted F11 proof. Author files were not edited.

## Verdict

The exact fixed finite splitting derivative and convex-order minimax criterion are mathematically sound under the stated full-rank hypothesis. That hypothesis holds for HKO by the stated product/no-antipodes argument. The pure facet-word finite-bank obstruction is correct, including its cap constant. It does not disprove HKO local maximality.

The original broad phrase “polygonal characteristics” needed care: general mixed normal-cone velocities need not preserve the exposing functional. The author narrowed the unconditional proposition to pure facet words. A separate argument below extends the obstruction to arbitrary polygonal **shortest** generalized characteristics at HKO, with canonical labeled occupation measures. It does not extend the unconditional nonshortest version.

## Exact derivative

- A closed normalized repeated word gives action2Q and dual energy1; after action normalization its energy is1/(2Q). Therefore Q<=1/(2c). Redundant rows with support at most1 only strengthen the bound. Translation to zero mean, if required by the chosen dual-principle formulation, leaves both quantities unchanged.
- For a fixed finite moving row bank, HK can optimize over its finitely many permutations even when some rows are redundant: genuine facet rows supply the lower bound, and the repeated-word dual bound supplies the upper bound. Capacity continuity gives convergence of maximizing values.
- The positive invertible closure-normalization minor gives the exact local chart used in the draft. Comparing F(t,z_t) with F(0,z_t) is legitimate because the solved positive coordinates stay positive. Subsequences and compactness give the matching envelope derivative without Hessian nonsingularity or a smooth optimizing branch.
- Deleting zero weights is essential and is done. With p_r=partial_beta_r Q, stationarity gives p_r=lambda·a_r+eta, eta=2Q=1/T. Closure gives p_r=2a_r·Js_r; hence x_r=-Tlambda+2TJs_r lies on its labeled facet.
- Inserting a new normal at any point and correcting the positive minor has reduced derivative (a_k·x-1)/T. Global repeated-word optimality forces this nonpositive. This proves all facet inequalities and fixes the characteristic's physical placement; it is stronger than merely solving the active equalities.
- The direct row derivative includes the self term -beta_r² omega(D_r,a_r). Combining the closure derivative with stationarity gives Q'=T^{-1}sum beta_r D_r·x_mid,r. Dropping the self term would be an error; the current draft retains it.
- Upper sensitivity is bounded by the max-affine profile integral. Conversely, subdividing any base optimal segment at its finitely many profile switches supplies a repeated-word feasible comparison with exactly that integral. Repetition is allowed by the dual bound, regardless of the original finite bank's simple-word enumeration. Thus q'/q=2 max_mu mu[f] exactly.
- Cone-volume normalization is total mass1, with V'/V=-2n nu[f]. Therefore the log systolic derivative is exactly2n(nu[f]-max_mu mu[f]); in R4 the factor is4. Common homothety f=C gives zero, an independent normalization check.

Full rank at HKO is automatic: positive Q needs both Lagrangian factors; positive closure in a planar set with no antipodal normals spans that plane. A positive closure relation then promotes ordinary rank4 to augmented rank5. This argument also covers repeated words and supports obtained after deleting zero weights.

## Measures, compactness and minimax

The finite disjoint union of closed facets is compact. Pure-word occupations are probabilities on it, including correctly labeled mass on ridges. Its weak closure is compact, so continuous facetwise profile integrals attain maxima. No assertion is made that a maximizing weak limit is itself one finite word.

The finite-dimensional separation argument for a finite list of convex profiles is correct: separating the compact response set from the positive orthant gives nonnegative coefficients, whose sum is another convex profile. Compactness and the finite intersection property give one common dominating ensemble, not a profile-dependent ensemble.

Continuous convex functions on a compact polytope are uniformly approximable by finite maxima of affine functions. Each affine function on a facet is D·x because a_i·x=1. The common-constant shift correctly shows that retaining every old facet (pure inward cuts) still tests the full convex cone. Both signs of facetwise affine functions impose per-facet masses and first moments.

These are directional assertions for fixed finite profiles. They provide neither a uniform remainder for increasing cut families nor an unrestricted Hausdorff neighborhood theorem.

## Pure-word finite-bank obstruction and quantitative strengthening

For a0=(1,tan alpha,0,0), w=(-sin alpha,cos alpha), alpha=pi/5, the p-vertex w uniquely maximizes ell=w·p at1. Every pure facet0 segment moves parallel (1,tan alpha), orthogonal to w, so ell is constant. A nonconstant segment cannot lie in the singleton exposing slice at ell=1. Finitely many such segments therefore have a strict common gap.

The one-facet hinge f0=(ell-r)_+, fi=0 elsewhere, vanishes on the bank and has positive cone-volume mean. It is realized by the single added row a0+tD, D=(-r,-r tan alpha,w1,w2). Thus the obstruction already concerns a single added facet, not only increasingly many simultaneous facets.

The cap integral is correct. For0<delta=1-r<1-cos(2alpha), its slice width at depth z is2z cot alpha. Thus integral_W f0=(cot alpha)delta³/3. Each HKO facet has cone-volume mass1/10, yielding

    nu[f] = cot(alpha) delta³ /(30 area(W)).

One can bound the missed cap directly from the positive word weights, without reconstructing orbit placements. The whole pentagon lies in the tangent wedge at w, whose slice width is at most2z cot alpha at every depth z. A canonical action-T facet0 segment of weight beta has p-length2T beta sec alpha. Consequently

    1-ell >= T beta sec(alpha) tan(alpha).

For a finite shortest-word bank with minimum positive facet0 weight beta_min, choosing delta below T beta_min sec(alpha)tan(alpha) gives exact zero response. This quantifies why successful direction-dependent families must allow vanishing segment weights.

## Extension to mixed polygonal systoles

This section is an independent extension of the scope-checked pure-word argument. The author independently reached the same argument during review.

Let gamma be any polygonal generalized shortest characteristic of H=P×_L W. Refine its finitely many straight segments at changes of relative face. H is simple, so the active normals on every stratum are linearly independent. On a constant-velocity piece, the nonnegative coefficients expressing -J dot(gamma) in those normals are therefore constant and unique. Remove constant pieces. Reparametrize to normalized action speed:

    dot(gamma)=2T J sum_i theta_i a_i,
    theta_i>=0, sum_i theta_i=1,

with all positive labels active. Define the canonical labeled occupation by integrating theta_i times the point mass at gamma(s) on the ith facet copy. This agrees with the pure-word occupation.

On each piece, its positive generators are pairwise symplectically orthogonal:

1. If the q-coordinate is a polygon vertex, its q-velocity is zero, so all p-normal coefficients vanish; the positive generators all belong to the q-plane. The p-vertex case is symmetric.
2. If q and p each lie in edge interiors, write their normals u,v. Tangency gives c_p u·v=c_q u·v=0. For a nonconstant piece this forces u·v=0, unless only one factor is active because the other coordinate is interior. Hence positive generators commute symplectically.
3. Remaining cases have only one active generator or one Lagrangian factor. A vertex×vertex piece is constant.

Replace each mixed straight piece by pure pieces with these weights, in any order. Their directions span an isotropic subspace, so this preserves the piece endpoints and its symplectic action. All original positive-label facet equations remain valid throughout the replacement, because moving in one generator annihilates the other active labels. The resulting closed pure word has Q=1/(2T)=Q*, hence is globally optimal.

Its active facet equalities at the original physical starting point determine the stationarity multiplier lambda=-gamma(0)/T. Full rank makes this multiplier unique. The insertion reconstruction argument therefore places the maximizing word at exactly that original point and proves that **every ordering** stays inside H. This is why shortestness is required for the mixed extension.

Now take a piece with positive facet0 coefficient. If either endpoint had p=w, order its positive facet0 piece first or last at that endpoint. That would produce a positive pure a0 segment on ell=1, impossible. Since ell is affine on the original straight piece, its maximum is below1 throughout the closed piece. A finite polygonal systole has finitely many such pieces; a finite bank again has a strict gap. The same hinge separates its canonical labeled occupations from cone-volume.

Moreover these mixed occupation measures belong to M: replace each mixed piece by increasingly fine repeated commuting pure pieces. Action and optimality are unchanged, reconstructed placement stays inside H, paths converge uniformly, and labeled occupation measures converge weakly to the mixed occupation. Infinite-piece limits may have support accumulating at ell=1, so no obstruction to the full closed ensemble follows.

## Prior source and evidence boundaries

The retained accepted F11 proof already says in Section4: “A fixed finite bank of positive weights cannot handle all redundant limits.” It then uses cut-dependent internal weights approaching zero. Source: `topics/variation-and-local-maxima/hko-local-maximum/f11-return-20261004/hko_f11_complete_proof.md`, around line81. The new argument gives a particularly clean geometric separation and a quantitative dwell-time bound; the need for continuous families was not previously unknown in this project.

A small exploratory f64 script `corner_orbits.py` examined generalized corner cycles on the25 product vertices. It found shortest vertex cycles at the known HKO action, while its mixed cycles touching the special exposed vertex with positive facet0 coefficient had larger action. This was only a sanity check and is not used in any proof. Output: `corner_orbits.json`.

No broad literature-priority search, independent rerun of the F11 certificate, or unrestricted local-maximality claim is part of this review. The final integrated wording and any mixed-segment extension should receive assembled-text review after the author incorporates the definitions above.
