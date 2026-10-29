# Independent audit: finite multi-cut derivative at HKO

## Verdict

The fixed finite split-bank derivative proposed in duality.md is sound in outline. The HKO rank argument removes the active-constraint degeneracy that would invalidate a generic Danskin formula. The exact formula is meaningful new progress; it does not yield unrestricted Hausdorff local maximality because no uniform margin/remainder across increasing cut counts has been proved.

## Rank and parametric optimization

For a feasible weight vector with Q>0, both q- and p-factor weights are nonzero: otherwise every symplectic pairing vanishes. Closure gives a positive zero-sum relation in each planar pentagon normal set. Such a relation cannot be collinear, since there is no antipodal pair, so the positive support spans R4. The normalization coordinate makes the augmented closure matrix rank five: if u·a_i+s=0 on the positive support, summing against beta gives s=0, then spanning gives u=0. This applies to every positive-Q optimizer after deleting its zero coordinates.

For any fixed finite moving row list, there are finitely many simple HK words. Given maximizers at t_k→0, pass to a fixed word and beta_k→beta_0. Choose five positive coordinates I of beta_0 with invertible augmented minor and solve them as smooth functions beta_I(t,z) of the other coordinates. At z_k=(beta_k)_J, both beta(t_k,z_k) and beta(0,z_k) are feasible for large k, since the solved coordinates converge to the strictly positive beta_0I. Hence

    Q*(t_k)-Q*(0) ≤ Q(t_k,z_k)-Q(0,z_k).

The quotient on the right converges to a section slope at beta_0. Conversely, fixing z_0 at any base optimizer gives a feasible section for small t and supplies the matching lower bound. Thus the right derivative is the maximum section slope over base optimizers. This argument does not use Hessian invertibility; optimizer continua and singular Hessians cause no problem. Minor choices give the same slope because their section velocities differ by a feasible tangent annihilated by the stationary objective gradient.

A wording point: sensitivity optimization should remain over finitely many simple words on the fixed finite split bank. Repeated rows are comparison candidates used below, not an iteratively enlarged optimization.

## Loop reconstruction and derivative constants

With omega(u,v)=(Ju)·v, stationarity gives p_i=partial_beta_i Q=lambda·a_i+nu, and Euler homogeneity gives nu=2Q=1/T, where T=(2Q)^-1. Define x_0=-T lambda and use velocity 2J a_i for time T beta_i. The start of segment i is x_i=x_0+2T sum_{j<i} beta_j J a_j. The closure identity gives p_i=2 sum_{j<i} beta_j omega(a_j,a_i), so a_i·x_i=T(p_i-lambda·a_i)=1; the active segment lies in its facet.

Containment in all facets follows by insertion. At any time insert an actual facet normal u with infinitesimal positive weight, splitting the segment there if needed. Correct the five positive coordinates to preserve closure and normalization. Optimality implies reduced cost r=p_new-lambda·u-nu≤0. The prefix formula is p_new=2 sum_before beta_j omega(a_j,u); therefore u·x(s)=T(p_new-lambda·u)≤T nu=1. This holds for every facet and time, so the reconstructed loop lies on the boundary.

The direct derivative from moving row i is

    -2 beta_i omega(D_i,prefix_i)-beta_i^2 omega(D_i,a_i).

The self-pairing term is necessary; together the terms equal beta_i D_i·(2J prefix_i+beta_i J a_i). Combining moving closure with stationarity yields

    Q'=T^-1 sum_i beta_i D_i·x_i^mid,
    Q'/Q=2 sum_i beta_i D_i·x_i^mid.

I found no sign or factor error.

## Exact finite-bank contact formula

For split rows a_i+tD_i,r, put f_i(x)=max_r D_i,r·x. Every simple optimizer word on the finite bank chooses a label per segment. Its response is bounded above by the integral of f on its reconstructed loop. Conversely, take any base maximizing word and split its straight segments at the finitely many switches of the affine functions D_i,r·x(s). Assign the maximizing label on each subinterval. The midpoint rule integrates the affine profile exactly.

This refined loop is a valid Clarke dual comparison for every t: for any closed weighted word, its action is 2Q; after action normalization its dual energy is at most 1/(2Q), because every listed row has support value at most one. Thus Q≤1/(2c_EHZ). The refined candidate therefore lower-bounds the true maximum's derivative. Combining both directions gives

    Q*'(0+)/Q*(0)=2 sup_gamma Avg_gamma f.

The supremum ranges over finite base maximizing loops of the fixed bank; compact closure of their time measures is harmless for continuous f.

The radial polytope volume derivative is V'/V=-4 nu(f), with nu=(4V)^-1 sum_i dS/|a_i| on the disjoint labeled facets. Since c=1/(2Q*) and sys=c²/(2V),

    D+ log sys = 4 (nu(f)-sup_gamma mu_gamma(f)).

The constants check.

## Minimax and limits

For a profile class C, first-order nonincrease for every f∈C is exactly sup_gamma mu_gamma(f)≥nu(f) for every f. If C is the cone of continuous convex functions on labeled facets, this is equivalent to some mixture in the closed convex hull of the orbit measures dominating nu in convex order. For a finite list f_1,...,f_k, if the response-vector convex set misses the nonnegative orthant, separation gives nonnegative alpha_j with every mixture giving negative response to sum alpha_j f_j, contradiction. Compactness and the finite-intersection property then yield one common dominating mixture. Since affine functions with both signs lie in C, facet masses and barycenters must match too.

This is a directional first-order criterion, not an unrestricted local theorem. No uniform strict margin or Taylor remainder as cut count grows has been established. A zero derivative is also compatible with second-order strict maximality.

The retained 26 feasible-section words do not catalogue all HKO systoles. Their sampled moment responses exceed normalized facet volume for the tested quadratic, quartic, and sextic profiles, but this does not prove convex-order domination. No unrestricted HKO conclusion follows from that finite sample.

## Audit of the polished theorem draft

I read the complete `/tmp/msc-math/restart/multicut/theorem.md` at SHA-256 `a35b58d7e2a3a62eec85e910e97b4d1501657b5b413f7fa69999ca8660999885`. The rank, sensitivity, insertion, midpoint, and volume arguments match the checks above. The statement's occupation measures are on the disjoint union of the original facet copies; duplicate split rows determine the maximizing label in the profile, but do not create new volume facets at the base point. This is the needed distinction at ridges.

The quantifiers are consistent: allowing arbitrary finite row velocities D includes affine old-row motion, and the full convex profile cone follows by uniform approximation by finite maxima of affine functions. Correction to my earlier qualification: retaining an old row as D=0 still suffices to test arbitrary convex profiles. For any profile f on all facets, choose one common constant C so f_i+C≥0 on every compact facet. Each shifted profile is still convex and is a finite maximum of affine functions; on F_i its affine terms are represented as D·x using a_i·x=1. The zero row then realizes max(0,f_i+C)=f_i+C. Since μ and ν are both probabilities, the common constant cancels from their response difference. Thus the pure inward cone already yields the full convex-order criterion.

I found no fatal gap in the exact fixed-bank derivative claim. The weak closure of the orbit measures is compact because it is a closed set of probabilities on the compact finite disjoint union of facets. The max over it exists for continuous f. Repeated rows are only needed for comparison candidates and are covered by the polar-loop bound; the sensitivity envelope remains finite for a fixed finite split bank. The unresolved distinction is the one the draft states: a separate uniform estimate is needed as cut slopes/counts grow to turn directional facts into a Hausdorff neighborhood theorem.
