# Independent audit: uniform gap and nearby partners

## Verdict

The corollary in `truncated-neighborhood-section.tex` is valid, conditional on
the separately audited strict all-partner theorem. The width bounds give the
compactness/attainment step; normalization and first-factor homogeneity give
the claimed uniform gap; Hausdorff inclusions give the neighborhood result
with the stated squared factor. I found no gap in the scaling or neighborhood
argument.

## Checks

1. **Width bound and attainment.** A placed vertical unit segment at `x=x0`
   lies between the minimum- and maximum-`x` support points. The triangles
   from the segment to those two points lie in opposite half-planes and have
   disjoint interiors. Their areas sum to
   `width_x(K)/2`, hence `width_x(K) <= 2 area(K)`. The horizontal segment
   gives `width_y(K) <= 2 area(K)` in the same way. A bounded-area minimizing
   sequence can therefore be translated to a uniformly bounded family
   (normalize one vertex of its placed `T` at the origin). Blaschke selection
   gives a Hausdorff-convergent subsequence. The template translations are
   bounded because each template has a vertex in the bounded body; subsequences
   of those translations converge, so all four containments persist. The
   limit contains the nondegenerate `T`, hence is still a convex body. Area
   continuity proves attainment.

2. **Strict gap.** Let `m_Q` be the attained minimum area among bodies
   containing all four unit templates. Every such body has capacity at least
   one. Applying the strict theorem gives
   `1 <= c_EHZ(K x_L Q)^2 < 2 area(K) area(Q)`, so
   `m_Q > 1/(2 area(Q))`. For arbitrary `K` with capacity `c>0`, scaling its
   first factor by `1/c` gives a capacity-one cover, so
   `area(K)/c^2 >= m_Q`. Thus
   `c^2 <= area(K)/m_Q = (1-delta_Q) 2 area(K) area(Q)`, where
   `delta_Q=1-1/(2 m_Q area(Q))` is strictly between zero and one. Capacity
   zero is trivial. This justifies a single gap uniform over all partners K.

3. **Hausdorff neighborhood.** After translating Q to put the origin in its
   interior, sufficiently close convex bodies satisfy
   `(1-epsilon)Q subset Q' subset (1+epsilon)Q`. The lower inclusion implies
   `area(Q') >= (1-epsilon)^2 area(Q)`; monotonicity and homogeneity give
   `c(K x_L Q') <= (1+epsilon)c(K x_L Q)`. The ratio is therefore bounded by
   `((1+epsilon)/(1-epsilon))^2 (1-delta_Q)`, which is less than one by the
   chosen epsilon. The same epsilon works for every K.

4. **Bodies represented in the neighborhood.** Small generic perturbations
   of the five vertices remain convex and remove the parallel-edge equalities;
   smooth strictly convex approximations to Q converge in Hausdorff distance.
   Both therefore occur in every sufficiently small Hausdorff neighborhood.

## Scope

The argument proves existence of `delta_Q` and of a neighborhood; it gives no
explicit values. It relies on the already audited pointwise strict theorem,
and does not assert an optimizer classification.
