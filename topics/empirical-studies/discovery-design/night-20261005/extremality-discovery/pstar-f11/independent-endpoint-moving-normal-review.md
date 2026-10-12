# Independent review of the moving p1 endpoint theorem

Reviewer `conditional_mechanism`; receiving owner `extremality_discovery_lead`.
Actual review start 2026-10-05 00:01 UTC; accepted at 00:16 UTC.
Owned output is this review and its independent audit; no producer edits.

**Accept the assembled restricted endpoint theorem.** There are one common
old-normal neighborhood of P*, a small equality-family patch, and positive
windows eta0,tau0 such that every genuine eleven-facet body in the full old
normal chart

```text
A=A(v,s)+zW(v,s)+Yy,
u(v)=1−v+v²,
D(v,eta)=(0,−2(u(v)−eta),1,0),
a_new=a0(A)+tau D(v,eta),
0<eta<=eta0, 0<tau<=tau0,
```

has ratio strictly below one. Old normals and both equality parameters can
move freely in the chart, including all relative eta,tau,z,y scales. The
specified tilt is transported with the same local symmetry chart as the F10
proof. Eta zero is an algebraic limit used for uniform estimates, not an
eleven-genuine-facet theorem point.

Reviewed and accepted [the cap proof](endpoint-cap-geometry.md),
[the exact section packet](endpoint-moving-normal/README.md), its verifier and
certificate, and [the assembled implication](endpoint-bridge.md). Acceptance
is independent agent review relative to the HK feasible-section capacity
formula and the existing P* F10 proof, chart and rank/curvature estimates. It
is not formalization, Jörn review, a novelty claim or full F11.

## Independent executable audit

[independent-endpoint-moving-normal-audit.py](independent-endpoint-moving-normal-audit.py)
imports no producer code. It reconstructs all ten equality-family normals,
reads the earlier accepted stationary-contact weight data, solves the old
five-pivot closure jet, and differentiates the eight-label HK pair sum directly
over `QQ(v,s,r,eta,delta)`. It also checks the global cap basis and determinant.
The retained audit initially passed at 00:08 UTC and its expanded balance
audit passed at 00:16 UTC:

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/independent-endpoint-moving-normal-audit.py
```

It verifies exactly

```text
DQ = delta(eta−4ur),    u=1−v+v²,
ell = eta²(1−2u/3)/6,  r=eta/6, delta=eta/12,
Q(0)=1/2,
the tau² coefficient is delta² R(v,s,eta), independent of r,delta,
D.x = eta+(u−eta)t0−uv t6−u(1−v)t9,
det[e_q1;a0;a6;a9] = −2/[u v(1−v)].
```

The expanded audit independently recomputes the full forty-component added
gradient as a polynomial in r. It differentiates the direct HK score through
the old closure pivots and supplies the volume gradient by product facet
integration. This matches the endpoint packet's gradient and proves its full
polynomial balance with gamma `1/100000`. It reverse-reconstructs each base
coefficient polynomial in the Bernstein basis from endpoint and midpoint
values, rather than using the producer's conversion, and checks the retained
nonzero augmented rank-twenty-three minor. The exact smallest Bernstein
coefficient agrees with the certificate:

```text
121944266997578428442112785276353699 /
1492993404840628075799618970132183758500 > 0
```

The endpoint certificate reviewed has SHA256

```text
aab9d33504722a91e2c11c92d637dc9bf9b3706caa6217d8023c2f04e01b4ee9
```

The retained producer checker also passed, rebuilding the inherited symbolic
stationary contact, full family annihilators and W jet before its endpoint
checks:

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/endpoint-moving-normal/verify.py
```

The independent arithmetic treats the existing old gradient rows as input;
the retained producer checker recomputes them through the inherited verifier.
No arbitrary forty-component old-normal perturbation is certified by sampling.
Its general mixed-factor implication is a structural argument reviewed below.
An initial
unoptimized SymPy audit was stopped after approximately 35 seconds of CPU;
the fraction-field version completed in approximately 0.6 seconds. No broad
producer or experiment was run.

## Accepted global cap geometry

The global slack route was supplied by this reviewer and retained by the lead.
Its actual moving coordinates are `(q1,t0,t6,t9)`, with
`ti=1−ai(A).x`. On a fixed small compact equality-family patch, the determinant
stays away from zero and inverse matrices are bounded. Old-normal movement
of norm `e=|z|+|y|` changes all coefficients in the D decomposition by `O(e)`
uniformly even at eta zero. The p-slack coefficients stay strictly positive,
and all old bodies have a common bound on q1.

For a removed point, `t0<tau D.x`; hence nonnegative actual slacks obey

```text
(1−tau b0)t0 +tau b6 t6+tau b9 t9 <= C tau(eta+e),
1−tau b0>=1/2, b6,b9>=b*>0.
```

Thus the entire removed set lies in a global box of lengths
`O(1), O(tau(eta+e)), O(eta+e), O(eta+e)`. Its Jacobian is uniformly bounded,
proving `DeltaV<=C tau(eta+e)^3`. This proof includes q-edge endpoints and
requires no projection onto a facet or clipping sign chamber. Facet labels
6 and 9, rather than 6 and 8, are the two facets at the moving right p vertex.
Genuine-eleven-facet assumptions restrict the final target, not this cap bound.

## Accepted positive section, balance and scaled remainder

The packet's stationary weights, multipliers, W jet and added gradient have
no pole at r zero. Independent denominator checks pass at the base endpoint.
The old closure pivot is independent of r and nonsingular. The two small
old weights at labels 0 and 2 have eta slopes `1/6` and `7/24`; after
duplication the old-0 and new-10 weights each have slope `1/12`. All other
weights have positive constant limits. Continuity on a compact family/r
patch therefore gives the stated lower bounds proportional to eta for small
weights and constant lower bounds for the others.

The stronger interior coefficient gamma `1/125` failed to keep its attempted
old coefficients positive near r zero. This is correctly retained as an
adverse attempt, rather than treated as a theorem premise. The replacement
gamma `1/100000` has an exact polynomial positive balance for every
`0<=r<=1/1200`. The eighteen independent kernel columns and old gradient
rank twenty-two put the added family gradient in the old row space also
at r zero. The fixed augmented minor has rank twenty-three. Holding its
remaining coefficient at its base polynomial and solving the minor gives
a regular parameter continuation satisfying all forty equations plus mass.
Uniform base positivity over the compact r interval and continuity give one
common family patch with positive coefficients. They are held fixed when
varying the transverse variables or tau; no uncontrolled derivative of the
balancing coefficients enters the Taylor proof.

For arbitrary nearby old normals A, keep the original old nonpivot weights
fixed and the new weight delta fixed. The old five-pivot closure matrix
depends on A and is independent of tau. Its inverse is uniformly regular.
The added weight correction is exactly `tau delta k(A,D)` and cancels the
new weighted normal's closure perturbation. Thus every weighted edge is
exactly `X_j+tau delta J_j`, with regular bounded coefficients. Bilinearity
of the HK pair sum proves, on arbitrary old-normal motions,

```text
Qnew−Qold = tau delta L +tau² delta² H.
```

At tau zero the consecutive identical-normal copies merge exactly into
Qold; their self-pair vanishes. The direct family calculation above gives
`L_family=eta−4ur`. Uniform smoothness then gives
`L−L_family=O(e)` through eta zero. No unscaled tau e term occurs. With
`r=eta/6`, `delta=eta/12`, and u bounded above by `4/5`, the capacity first
loss is at least `7eta²/90`.

The old continuation changes weights by `O(e)`, and the exact added-facet
correction is `O(tau eta)`. Hence positive feasibility holds whenever e/eta
and tau are uniformly small. The q-only W jet preserves mass, closure and
Q exactly along the W line, fixes all nonpivot weights, and is realized by
this same closure pivot. Its regularity at r zero and common volume give
the inherited curvature `-13/432` at the base, independent of r. A common
family patch preserves strict negative curvature.

Finally `U=1/(8V(A)Q²)` has uniformly positive denominators on the formal
section neighborhood. Its smooth scalar Taylor expansion applied to the
structural identity above gives the claimed
`−m0 tau eta²+O(tau eta e+tau²eta²)` response. Changes of its prefactor
contribute `O(tau eta² e)` and are absorbed. The fixed positive gamma
produces the same estimate for the balanced mean with some m>0. Formal
smoothness at eta zero is used for uniform constants; feasibility as an
actual HK upper witness is needed only in the positive-eta forced region.
All four section premises of the lead implication are established.

## Accepted uniform analytic implication

The existing F10 cusp and positive cancelling mean imply the uniform old-body
deficit `rho(A)<=1−k(|y|+z²)`. Capacity monotonicity then controls every cut
with cap loss small compared with this deficit. If `e>=eta`, the cubic cap
bound and `|y|+z²>=c e²` give that comparison in one common small window.
All unresolved paths, including equality-family points with zero deficit,
therefore satisfy

```text
|y|+z² <= C tau eta³,
|z| <= C sqrt(tau) eta^(3/2),
|y| <= C tau eta³.
```

In this region `e/eta<=C(sqrt(tau eta)+tau eta²)`, uniformly small. This
preserves section weights of size eta using the proved closure correction
bound `O(e+tau eta)`.

The established regular family-balanced section has negative W curvature
and the exact split response estimate
`−m tau eta²+O(tau eta e+tau²eta²)`. The balanced mean at tau zero has zero
first-order old-normal gradient and negative z² term. Multiply its formal
uncut-volume ratio by `V/Vcut` only after establishing positive section
feasibility. The positive errors are

```text
|y|², |z||y|, e³, tau eta e, tau²eta², tau eta³.
```

Dividing them by `tau eta²` in the unresolved region gives, respectively,
`O(tau eta^4)`, `O(sqrt(tau)eta^(5/2))`,
`O(sqrt(tau)eta^(5/2))`,
`O(sqrt(tau eta)+tau eta²)`, `O(tau)`, and `O(eta)`.
They vanish uniformly as the common windows shrink. No relative-rate
assumption is imposed on eta, tau, or old-normal motion. The negative response
then establishes strict ratio loss throughout the unresolved region, and the
old witnesses handle its complement. The specified p1 tilt must be transported
with the symmetry chart, as in the lead statement.

## Accepted boundary and reproduction limits

The certified base interval `r<=1/1200`, corresponding to `eta<=1/200`, is
an algebraic balance control interval. It is not an explicit theorem window.
The final common eta and tau windows and old-normal/family neighborhoods
must be shrunk to absorb constants relative to the small fixed gamma. Their
existence is proved, but no numerical radii are supplied. In particular this
review does not promote the entire eta interval to a uniform moving-body
theorem with an explicit tau bound.

The accepted statement concerns one transported p1 endpoint family with free
old normals and equality parameters. Other facets, q-edge/p2 tilt, the left
fraction endpoint and general support-stratum limits remain outside it. This
review claims neither full F11 nor novelty, formalization or manuscript
coverage. The exact audit and checker reproduce algebraic predicates; the
uniform cap, feasibility, smooth continuation and final implication are
mathematical arguments, not output of an exhaustive cut computation. Review
complete; integration and residual-domain reporting belong to the lead.
