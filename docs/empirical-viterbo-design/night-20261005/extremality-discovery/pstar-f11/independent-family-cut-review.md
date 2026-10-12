# Independent review: fixed-cut conclusions along the P* equality family

2026-10-04 23:24 UTC. Reviewer: `conditional_mechanism`. Receiving owner:
`extremality_discovery_lead`. Bounded renewal of the preceding independent
vertex review; no new broad research, producer output replacement, or thesis
promotion.

**Accepted:** on one sufficiently small common equality-family parameter patch
around `(v,s) = (1/2,1/12)`, every body `E(v,s)` has strict ratio reduction for
each sufficiently shallow fixed direction exposing codimension at least two,
ratio at most one for shallow facet-parallel cuts, and is a cuts extremizer.
Cut depth remains quantified separately for each body and direction.

## Exact family ridge predicates independently checked

Read `cut-strata/verify-family.py` and its retained
`cut-strata/family-ridge-margins.json`. The verifier reconstructs the 12
positive primitive tight words over `QQ(v,s)` from the accepted family vertex
weights. For every base-selected mixed pair, it preserves its positive ordered
adjacency, checks rational edge slice densities, and compares the base margin
with the exact base certificate.

An independent read-only scratch audit,
`/tmp/msc-math/pstar-independent-vertex-review/family-ridges.py`, checked every
one of the 24 retained family rows against the existing rational family
normals and vertices. It independently recomputed normalization, normal
closure, direct pair-sum `Q = 1/2`, adjacent symplectic pairing, and the edge
density identities. It also checked nonvanishing rational denominators and
strict positivity of the old weights, pairings and densities at the base.
All checks passed; the maximum base density/pairing product is `5/8`.

Reviewed family-ridge packet hash:

```text
a609b579604916486bae03ddff3beebecde7e5c58228310dd09478fd788d13fb
```

The audit checked the square of each density against the coordinate edge/normal
ratio and selected its positive base sign. This validates
`R = edge_length / |row_normal|` without introducing floating-point norms.
Facet-incidence identities make the edge perpendicular to the normal, and
regularity preserves the chosen positive sign near the base.

## Why a common parameter patch follows

There are finitely many rational functions. Their denominators are nonzero at
the base; old primitive weights, selected ordered pairings, and edge densities
are strictly positive there. All 24 products satisfy
`Rq Rp omega ≤ 5/8 < 1`. Continuity therefore supplies one sufficiently small
parameter patch on which all these predicates hold simultaneously, including
`Rq Rp omega < 1`. This inference uses a finite strict margin, not sampling of
the parameter plane or constancy of the margin functions.

The previously reviewed adjacency surgery is an algebraic identity for arbitrary
normals and weights satisfying the stated closure and normalization. Inserting
`w = t a + (1−t)b` between the positive adjacent positions gives

\[
 Q=\tfrac12-\omega(a,b)t(1-t)\delta^2.
\]

For each fixed family point and `0 < t < 1`, feasibility persists for small
δ. The exact action bound and ridge cap expansion then give ratio coefficient

\[
 \frac{Rq\,Rp-1/\omega(a,b)}{t(1-t)}<0.
\]

Thus every fixed mixed-ridge direction is accepted on that common shape patch.
No new catalogue-completeness assumption enters: the retained primitive words
need only be feasible and tight, not all minimizing words.

## Assembly with the other face strata

Intersect this patch with the accepted family vertex-witness patch and a small
patch on which the existing F10 theorem applies. The original family body has
constant capacity one and volume one half, as supplied by that existing proof.

- Family closed-cone witnesses preserve zero-weight feasibility and `C ≥ 0`;
  they give strict capacity reduction for every fixed nonfacet direction.
  Vertices and codimension-three edges have higher-order cap volume, hence
  strict shallow-cut ratio reduction.
- For a pure-factor ridge, adding its fixed normal preserves product structure.
  The complete product circuit capacity envelope is finite and affine in the
  new support depth when the other factor is fixed. Continuity to depth zero
  and strict capacity reduction force a negative active slope. Linear capacity
  loss then dominates quadratic cap-volume loss. This argument applies at each
  fixed family point and does not require the active circuit list to be constant
  across shape parameters.
- Facet-parallel cuts have exactly ten genuine facets and stay in the existing
  F10 neighborhood when sufficiently shallow. Thus their ratio is at most one.
  Their strictly smaller volume gives capacity strictly below one.

This covers every face stratum. For any interior hyperplane and either retained
side, a shallower parallel cut contains that piece and already has capacity
below one. Capacity monotonicity therefore gives strict capacity reduction for
both pieces of every such hyperplane. Each `E(v,s)` in the common parameter
patch is consequently a cuts extremizer.

## Accepted boundary

The common neighborhood is in the **equality-family shape parameters**. The
small cut depth may depend on the body and normal direction; this review gives
no depth uniform over changing directions, no arbitrary moving-old-normal
result, and no full F11 Hausdorff-neighborhood theorem. It also gives no cuts
additivity or symplectic-ball classification. Mathematical status is independent
agent acceptance relative to the existing F10 proof and HK formula, not Jörn
review or a novelty claim. Bounded review complete; stop for integration.
