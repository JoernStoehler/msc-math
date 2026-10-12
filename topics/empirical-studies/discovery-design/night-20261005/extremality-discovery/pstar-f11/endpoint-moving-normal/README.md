# Uniform endpoint section for moving old normals

Producer retention, 2026-10-05 00:14 UTC. Exact algebra and analytic premises
for the supplied right-p1 tilt; independently accepted at 00:16 UTC. See
[the independent review](../independent-endpoint-moving-normal-review.md). The final ratio
implication and cap geometry belong to [../endpoint-bridge.md](../endpoint-bridge.md)
and [../endpoint-cap-geometry.md](../endpoint-cap-geometry.md).
This packet does not claim an unrestricted F11 neighborhood theorem.

```bash
uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/endpoint-moving-normal/verify.py
```

The checker rebuilds the inherited stationary continuum and full annihilators,
then checks endpoint regularity, the whole-interval positive balance, the
family response, and the universal bilinear factor identity. It performs no
optimizer call, samples no cut grid, and writes no evidence. Its inherited
premises come from the existing non-HKO F10 proof and the independently
accepted moving-old packet. Exact algebra does not replace the compactness
and feasibility arguments below.

## Contact, endpoint weights and fixed regular pivot

Use `A=A(v,s)+zW(v,s)+Yy`, with `e=|z|+|y|`, and the stationary word
`sigma=(0,6,4,8,2,1,9)` from the inherited packet. Put

    u=1-v+v², D=(0,-2(u-eta),1,0), r=eta/6, delta=r/2=eta/12.

The added label 10 is inserted immediately before label 0, with weight delta;
label 0 retains weight `b0-delta`. Both copies at tau=0 have the same normal,
so merging them recovers exactly the old stationary section. All supports
remain one and the word contains eight distinct facet labels when tau>0.

At the rational equality base the original stationary weights are

    (r, 1/4, 1/4-r/2, 1/8, 7r/4, 1/4-9r/4, 1/8).

For `0<=r<=1/1200`, the five constant-sized weights are at least 1/8.
The split old label 0 and new label 10 weights are both `r/2=eta/12`, and old label 2's weight
is `7r/4=7eta/24`. In the equality family, old label 2's weight is identically
`r f(v,s)` with f(base)=7/4, while b0=r identically. All other weights are
regular, positive functions of `(v,s,r)` on a common small parameter patch.
Consequently, after shrinking that patch, every constant-sized weight is
at least 1/16 and every small old/new weight at least `eta/24`.

The old mass-and-closure matrix is `C(A)=[A_sigma^T;1^T]`. Keep the original
nonpivot weights fixed and solve old positions `I=(0,2,3,4,6)` for closure
and mass. At the equality base

    det C_I = -4608/49.

This determinant is independent of r and remains bounded away from zero on
one compact equality-family patch and one old-normal neighborhood. Neither
the inverse pivot, the stationary weights, nor the W jet has a pole at r=0.
These statements are checked from the rational formulas, including the
denominators of all W-jet entries.

Let `b(A)` be the ordinary uncut old section produced by this pivot
continuation from the family contact. Let

    k(A,D) = -C_I(A)^(-1) (D^T,0)^T.

The exact split section, for arbitrary old normals A in that neighborhood,
is obtained by adding `tau delta k` to the old pivot positions, subtracting
delta from old label 0, and adding new label 10 with its fixed weight delta. Closure and
mass hold exactly: the new edge contributes `tau delta D`, canceled by
`C_I tau delta k`. All these functions are smooth and uniformly bounded
through eta=0, tau=0 and r=0. The old-normal continuation changes old weights
by O(e); splitting changes them by O(tau eta).

There are therefore uniform constants C>0 and c>0 for which all old weights
of the split section are bounded below by their family values minus
`C(e+tau eta)`. For example, choose `e/eta` and tau so

    C(e/eta+tau) < 1/48.

Then the small old weights stay above eta/48; the new weight is exactly
eta/12. The constant-sized weights stay positive after an additional common
smallness choice. Positivity is asserted for eta>0 in this relative region;
the regular algebraic eta=0 extension need not be a positive section.

## Positive balanced contact through the endpoint

Let G(v,s) be the 24 old formal-ratio gradient rows from the F10 proof, and
g(v,s,r) the merged contact gradient. The inherited checker proves over
QQ(v,s,r) that g annihilates all 15 symmetry columns, both equality-family
derivatives and W. The 18 kernel columns have rank 18 at the base, while G
has rank 22 and preserves that rank locally. Hence g lies in the old row
space for every r, including r=0. This uses the full kernel, not a span test
only at one positive fraction.

The original open-patch balance with gamma=1/125 did not provide a positive
continuation down to r=0. The retained endpoint balance instead uses the
fixed smaller coefficient

    gamma=1/100000.

Its 24 base old coefficients lambda_i(r) are explicit polynomials in the
certificate. They satisfy, identically in r,

    G_base^T lambda(r)+gamma g_base(r)^T=0,
    sum lambda(r)+gamma=1.

Every Bernstein coefficient on the entire interval `0<=r<=1/1200` is
positive. Their minimum is exactly

    121944266997578428442112785276353699 /
    1492993404840628075799618970132183758500 > 0.

This supplies a uniform positive lower bound, approximately 8.17e-5, rather
than positivity at isolated fractions. The old certificate's positive
normalized null relation is independently checked too.

For a constructive parameter continuation, augment `G(v,s)^T` by the mass
row. Its rank is23: if the mass row were in the gradient-row span it would
contradict the positive normalized old null relation. The retained fixed
23-column/23-row minor is nonzero at the base. Hold the remaining old
coefficient equal to its retained base polynomial lambda_i(r), solve that
minor for the other 23 coefficients, and keep gamma fixed. Because the target
g lies in the old row space, the solved equations imply the full 40 gradient
equations; normalization supplies the remaining independent equation.

The matrix inverse is independent of r. The solution is jointly regular in
`(v,s,r)` on a common patch. Uniform positivity on the compact base r interval
and continuity give a common smaller equality-family patch on which every
coefficient remains strictly positive for every r in that interval. Thus the
merged gradient admits a smooth uniformly positive normalized balance through
eta=0. Hold its coefficients fixed while varying z,y,tau.

## Uniform W curvature

Coordinates are ordered `(q1,q2,p1,p2)`: labels 0 through 5 are q facets,
and labels 6 through 9 are p facets. The word-position pivots
`I=(0,2,3,4,6)` correspond to facet labels `(0,4,8,2,9)`, so their q
labels are `(0,4,2)` and their p labels are `(8,9)`. The nonpivot p label
is 6. At the base, the retained W weight jet in word-position order is
`(-7/216,0,77/864,0,-49/864,0,0)`. The inherited verifier checks zero
weight variation for every `lab>=6` and verifies exact mass and closure.
Thus the q-only W jet changes only the three q pivot weights. It is
regular at r=0 and fixes both nonpivot weights and all p weights. Consequently
the above closure pivot realizes this jet, including the fixed duplicate
weight. Every varying weighted edge lies in the q Lagrangian plane, and the
exact linear and quadratic Q variations vanish. Along the actual W line,

    Q=1/2

identically for all family r, including the algebraic limit r=0. At the base
the uncut-volume formal ratio is `864/(864+13z²)`, with second derivative
`-13/432`, independent of r. A common compact-neighborhood argument preserves
strict negative curvature. The positive balanced mean therefore retains
uniform negative W curvature, as do the old sections in the F10 proof.

## Structural factor identity for arbitrary old normals

For fixed A, write the baseline split weighted edges as X_j. The exact section
above changes every old pivot edge by `tau delta k_j a_sigma_j`, changes the
new edge by `tau delta D`, and leaves all other edges unchanged. Thus every
edge is exactly `X_j+tau delta J_j`, with uniformly regular J_j. The HK
quadratic pair sum gives the universal identity

    Qnew-Qold = tau delta L + tau² delta² H,
    L = sum_{i<j}[omega(J_i,X_j)+omega(X_i,J_j)],
    H = sum_{i<j} omega(J_i,J_j).

Here Qold is the ordinary uncut section: duplication at tau=0 adds only a
zero self-pair, and all cross-pairs merge. This argument holds for arbitrary
nearby old normals, not just product normals. No generic forty-variable
rational expansion is necessary; the checker verifies the bilinear polynomial
identity and independently constructs the family edge coefficients. L can
depend smoothly on delta; that is harmless and does not remove its outer
delta factor.

The exact family calculation gives

    L_family=eta-4ur,
    H_family=the regular rational function retained in certificate.json.

The checker rebuilds both directly from the word, inverse closure pivot and
symplectic pair sum over QQ(v,s,r,eta,delta). H_family is independent of r
and delta. The generic L,H are bounded and regular; uniform smoothness gives
`L-L_family=O(e)` with no inverse eta or r factor. With r=eta/6,
delta=eta/12, and a family patch chosen so u<=4/5, the capacity first loss is

    ell=2delta L_family=eta²(1-2u/3)/6 >= 7eta²/90.

Thus the positive response does not vanish faster than eta² at the endpoint.

Use the uncut volume in all formal ratio scores. The section score is
`U=1/(8 V(A) Q²)`. Q and V stay uniformly positive. Taylor expansion of
this smooth scalar function, with the exact structural identity above, gives
for the added score

    U(A,eta,tau)-U(A,eta,0)
      <= -m0 tau eta²+C(tau eta e+tau² eta²),

with uniform m0>0. Changes of the prefactor from its family value contribute
only O(tau eta² e), absorbed in the displayed scaled error. The fixed positive
gamma balance therefore gives the same inequality for the mean, with some
uniform m>0. In particular no unfactored O(tau e) remainder is used.

## Interface supplied to the integrating proof

This packet establishes all four new-section premises of
[../endpoint-bridge.md](../endpoint-bridge.md): regular eta=0 extension and
positive feasibility when e/eta and tau are small; uniform positive balance;
negative W curvature; and the scaled response estimate. The lead's cap
estimate and old-witness dichotomy force every unresolved path into
`e/eta=O(sqrt(tau eta)+tau eta²)`, precisely where this feasibility estimate
applies. Multiplication by the old/cut volume ratio is left to that bridge.

The eta interval `0<eta<=1/200` is an algebraic control interval for the
certificate. It is not an explicit theorem neighborhood: the analytic proof
can shrink it and the equality-family/old-normal/tau patches without giving
numerical radii. The target remains the specified transported right-p1 tilt
with eleven genuine facets. Other tilt directions, other old facets and the
left endpoint are outside this packet.
