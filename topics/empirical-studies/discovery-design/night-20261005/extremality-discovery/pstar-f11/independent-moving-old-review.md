# Independent review of the moving-old open facet-split theorem

Accepted 2026-10-04 23:44 UTC; review retained at 23:47 UTC. Reviewer:
`conditional_mechanism`. Receiving owner: `extremality_discovery_lead`.
Reviewed [the analytic bridge](moving-old-bridge.md) and the
[exact premises packet](facet-split/moving-old/README.md), its verifier and
certificate. No producer or lead-source changes are owned by this review.

**Accept the assembled local theorem:** there are neighborhoods of the rational
base P* in the full ten-facet normal chart and of
`D0=(0,-149/100,1,0)` such that adding the support-height-one normal
`a_new=a_0(A)+tau D`, for sufficiently small `tau>0`, gives ratio strictly
below one. Old normals, both equality parameters and the tilt direction can
move together within these neighborhoods. At `tau=0` the existing F10 theorem
gives ratio at most one. No numerical neighborhood radius is established.

Acceptance is independent agent review relative to the HK feasible-section
upper bound and the existing non-HKO P* F10 chart, rank, curvature and local
proof. It is not formalization, Jörn review, a novelty claim, or proof of the
whole F11 neighborhood.

## Exact evidence and independent arithmetic

The retained checker passed:

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/facet-split/moving-old/verify.py
```

The separate read-only audit originally at
`/tmp/msc-math/pstar-independent-vertex-review/moving-old-base.py` is retained
as [independent-moving-old-audit.py](independent-moving-old-audit.py). It
imports no retained producer or `family_exact` module. It reconstructs the
base normal/vertex geometry from the vertex-cone certificate and independently
differentiates the direct HK pair sum through the five closure pivots. Product
facet integration independently supplies the full forty-component volume
gradient. The resulting added ratio gradient matches the retained extra row.
It checks all positive old coefficients, normalization and the full
forty-component gradient balance with `gamma=1/125`.

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/independent-moving-old-audit.py
```

The audit also evaluates the duplicated-label word directly, verifies closure
and total mass along its explicit weight jet, and recomputes both the capacity
response and the cap first derivative by integration. It passes, including a
read-only rerun at 23:46 UTC:

```text
ell = 1/480000
L = 1/86400000
gamma ell - L = 11/2160000000 > 0
certificate SHA256:
2ebf8b1dfe30092b78e2d544217f97bee0248bd980bb972076e38369ad954264
```

This arithmetic audit treats the existing old gradient rows as its input;
the packet verifier independently recomputes those rows. The parameter-family
identities below are checked by that retained verifier and independently
reviewed mathematically, rather than by a second symbolic implementation of
the entire family proof.

## Equality-family continuation and genuine feasible upper sections

The stationary equations hold over `QQ(v,s,r)`, including normalization,
closure and `Q=1/2`. At `r=1/1200` the weights are strictly positive and the
five old closure pivots have determinant `-4608/49`. Splitting facet 0 into
two consecutive occurrences with new weight `1/2400` gives eight distinct
labels and leaves a nonsingular closure minor. Fixing the nonpivot weights
therefore defines an actual regular positive feasible section for nearby old
normals and new facets. A formal first jet alone would not supply this fact.

The duplicated gradient merged over the two copies is the old stationary
gradient. Its annihilation of fifteen symmetry tangents, two equality-family
tangents and the maintained W direction is exact throughout the parameter
patch. The existing proof supplies eighteen independent kernel columns and
old gradient rank twenty-two. The added gradient consequently lies in the
old row space there.

The positive base balance extends smoothly with gamma fixed. In particular,
the augmented old matrix consisting of twenty-two independent quotient
gradient equations and normalization has rank twenty-three: otherwise the
normalization row would be a combination of gradient equations, contradicting
the existing normalized positive old null relation. One free old coefficient
may be held fixed while solving the remaining twenty-three. Positivity and
the strict response margin persist after shrinking the parameter and direction
patches. This establishes family cancellation, not merely a base span check.

## W curvature and cap regularity

The q-only weight jet preserves total mass and closure along `A(v,s)+zW(v,s)`
exactly. W is supported on the p labels, while the changing weights are on q
labels, so no omitted product of the weight jet with W appears. Every varying
weighted edge lies in the q Lagrangian plane. Direct identities make both
the linear and quadratic terms of Q vanish. Hence `Q=1/2` along the actual
line, with positive weights for small z. The pivot continuation realizes this
same section, including the fixed duplicate weight. At the base its upper
ratio is `864/(864+13z^2)`, with curvature `-13/432`; continuity preserves
strict negative W curvature near the equality family.

For D0, `D0.x=p1-149/200` is nonzero at all eight vertices of the old facet
and takes both signs. P* is simple and these strict predicates persist nearby.
The cut thus has a stable sign chamber. At a vertex involving both old and
new facet equations, their difference gives `D.x=0` after division by tau.
Its intersection matrix is nonsingular because the crossed edge has opposite
endpoint signs. The other vertex matrices are ordinary nonsingular old
vertex matrices at tau zero. A fixed triangulation gives regular one-sided
volume formulas. The cap derivative is the facet integral of the positive
part, smooth in the old-normal and tilt patch. This supplies uniform
`V_cut=V-tau L+O(tau^2)` and the mixed remainder control used by the bridge.
It does not assert such regularity across changing sign chambers.

## Uniform analytic implication and coverage

Every section is a feasible capacity upper witness, so its ratio upper
function and any positive normalized mean dominate the true ratio. Merged
gradient cancellation eliminates all first-order old-normal terms in the
mean. Its tau derivative is `2L-2 gamma ell<0`, and its W curvature is
strictly negative. The existing old-section minimum retains its linear cusp
in the twenty-two remaining transverse coordinates y. Uniform bounds are
therefore

```text
min_j(U_j^cut-1) <= -alpha |y| + C(|y|^2+z^2+tau),
F-1 <= -b z^2-m tau
       + C'(|y|^2+|z||y|+(|z|+|y|)^3+tau(|z|+|y|)+tau^2).
```

Choose M large. For `|y| >= M(z^2+tau)`, the cusp bound is negative in a
sufficiently small neighborhood. In the complementary region every positive
mean remainder is `o(z^2+tau)` uniformly, so the mean is negative when
`z^2+tau>0`. The remaining `z=tau=0, y!=0` case is covered by the cusp.
This handles arbitrary coupled paths, without an assumed relative rate of
old-normal motion and facet birth. The full local group chart inherited from
the F10 proof covers all old-normal motions; transporting D under the group
action preserves the stated direction-patch interpretation.

The accepted theorem covers this open facet-0 direction patch, including old
normal and equality-family drift. Other facets, arbitrary tilt directions,
fraction endpoints, and changing support strata are outside this review.
The quantitative endpoint cap note remains conditional on its stated cap and
scaled remainder premises. Review complete; integration belongs to the lead.
