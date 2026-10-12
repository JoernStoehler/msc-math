# A quantitative reduction of endpoint motion to the cap-loss scale

Lead analytic derivation, 2026-10-04 23:39 UTC. Conditional reduction, not a
new F11 theorem. It records a useful next bridge beyond the selected open
facet-split patch. It inherits the F10 chart and upper-section estimates used
in [moving-old-bridge.md](moving-old-bridge.md).
Independent reviewer `conditional_mechanism` accepted the conditional algebra
at23:41 UTC; the endpoint cap and remainder premises remain unproved.

## The F10 estimates give a quantitative transverse deficit

In the normal chart `A=A(v,s)+zW(v,s)+Yy`, the old cusp witnesses and their
positive cancelling mean give, uniformly near a compact equality-family patch,

    min_j(U_j−1) <= −a|y|+C(|y|²+z²),
    F_old−1 <= −b z²+C(|y|²+|z||y|+(|z|+|y|)³),

where `a,b>0`. Each of these functions is an upper bound for the old body's
ratio. Choose a fixed large M. If `|y|>=M z²`, the first bound is at most
`−a|y|/2` after shrinking the neighborhood and making M large enough. If
`|y|<M z²`, the second bound is at most `−b z²/2` after shrinking it again.
Consequently there is `k>0` such that

    rho(A) <= min(min_j U_j,F_old) <= 1−k d,
    d=|y|+z².

The two equality parameters are absent from d because they move along the
contact family. This is a quadratic, not linear, transverse estimate.

If a new facet removes volume `DeltaV` from the old body, all these old
capacity witnesses remain feasible with the new label unused. Thus their ratio
upper bounds multiply by `V(A)/(V(A)−DeltaV)`. Uniform positive lower volume
and bounded old upper scores give

    rho(A_cut) <= 1−k d+C0 DeltaV.

Therefore cuts with `DeltaV<=k d/(2C0)` are already strictly controlled whenever
`d>0`. Any unresolved path must have old-normal deficit `d=O(DeltaV)`.
This follows from the actual transverse F10 margin; capacity decrease alone
would not yield this comparison.
Equivalently, capacity monotonicity supplies this multiplication directly,
including when an old inequality becomes redundant after cutting.

## Endpoint cap estimate would force relative smallness automatically

Consider the right endpoint of the accepted one-tilt slice. At the equality
base the cut-fraction size is `eta=3/4−c`, and the cap volume is order
`tau eta³`. Put `e=|z|+|y|`. For small variables `d>=c0 e²` for a constant
`c0>0`. Suppose a local cap-geometry argument establishes uniformly

    DeltaV <= C1 tau (eta+e)³.                         (E)

This is a **premise requiring proof for the chosen moving-normal chart**;
the fixed-P* cap formula by itself does not establish it. Constants must stay
bounded when equality parameters and the selected endpoint direction move.

If `e>=eta`, then `DeltaV<=8C1 tau e³`, whereas `d>=c0 e²`. For sufficiently
small common `tau,e`, the old witnesses strictly control the cut. Hence an
unresolved path has `e<eta`. Now `DeltaV<=8C1 tau eta³`, and failure of the
old-witness sufficient condition implies

    d <= C2 tau eta³,
    |z| <= C3 sqrt(tau) eta^(3/2),
    |y| <= C4 tau eta³.

In particular `e/eta` tends uniformly to zero as `tau,eta` shrink. Thus an
endpoint proof need not impose `|z|<<eta` as an extra restriction: a uniform
cap estimate and the old F10 deficit may force it precisely on the paths that
still need a new section. This could preserve weights proportional to eta
against additive old-normal closure corrections of order e.

## What remains to turn this into an endpoint theorem

Besides (E), one needs a fraction-adapted touching continuum with uniformly
bounded gradient balance, positive feasibility in the relative region, and
weighted new response at least `m tau eta²`. Its old-motion and tau remainders
must have a proved scale, for example `O(tau eta e+tau² eta²)` plus the usual
cubic transverse terms. In the relative region above these are smaller than
`tau eta²`; cancellation and strict z curvature can then give a uniform loss.
An unscaled `O(tau e)` term need not become small relative to the response, since
`e/eta²` is not uniformly small: it is bounded by `sqrt(tau/eta)` and can
diverge. Therefore that weaker error estimate is insufficient for the stated
endpoint argument.

No exact checker or accepted moving-normal cap estimate presently supplies
these endpoint premises. The selected open patch avoids this endpoint and is
reviewed separately. General facet tilt, phase terms and other support strata
still require their own analysis.
