# A uniform nonproduct facet-split theorem across the full cut-fraction interval

Lead derivation: extremality_discovery_lead, 2026-10-04 23:29 UTC.
Status: accepted by independent consequential review at23:36 UTC; see
[review](../independent-facet-fraction-review.md). This is agent acceptance
relative to the inherited HK formula and P* geometry, not formalization or a
novelty claim.
This is a new restricted F11 result. It does not establish the full moving-normal
F11 neighborhood. The selected proof group owns this packet.

## Theorem

Let P* be the rational hexagon–quadrilateral product in the existing F10 proof,
with capacity1 and volume1/2. In coordinates(q1,q2,p1,p2), define

    b(c,tau)=(0,2−2c tau,tau,0),
    K(c,tau)=P* intersect {b(c,tau).x<=1}.

Then

    −1/4<c<3/4,  0<tau<=1/100  =>  rho(K(c,tau))<1.

All these bodies have eleven genuine facets, with a new mixed q/p facet. The
same tau window works over the ENTIRE cut-fraction interval, even when c tends
to either endpoint along an arbitrary function of tau. Thus this is a coupled
endpoint result, stronger than a separate sufficiently-shallow statement for
each c. It is only this one tilt direction at the fixed base P*: arbitrary
facet tilts, moving old normals and equality-family drift remain unproved.

The new facet splits the old top q facet q2=1/2, on which its cut function is
p1−c. Because the p factor's p1 range is[−1/4,3/4], the new facet is genuine and
the old q facet still has an open retained part exactly in the displayed open
c interval. Small tau leaves all other old facets genuine.

## Why a fixed finite section bank failed

The specialist found that all twenty relevant sections in the existing selected
F10 finite bank have facet-response contact coordinate at most48/65<149/200.
For the concrete split p1−149/200, every duplicated-section response is therefore
nonpositive while removed volume has a positive first derivative. This falsifies
that finite bank's sufficiency, not local maximality or the desired inequality.

A stationary continuum repairs the failure. Its primitive word is

    sigma=(0,6,4,8,2,1,9),
    b(r)=(r,1/4,1/4−r/2,1/8,7r/4,1/4−9r/4,1/8),
    0<r<1/9.

Its exact Q is1/2. Its response-contact p1 coordinate is3/4−3r, which approaches
the exposed p vertex as r tends to0. The weight must depend on the actual cut
fraction; one fixed positive weight cannot serve arbitrarily small fractions.
The specialist owns the finite-bank failure and independent continuum derivation
in the sibling facet-split/ packet.

## Exact feasible eight-label section

Write eta=3/4−c in(0,1) and choose

    r=min(eta/6,1/12),   delta=3r/4.

Duplicate facet0 next to itself, inserting the new label10, with word

    (0,10,6,4,8,2,1,9).

At tau0, the weights are

    (r−delta,delta,1/4,1/4−r/2,1/8,7r/4,1/4−9r/4,1/8).

Fix the new weightdelta. Correct the old weights by tau times the following
vector, listed on old labels(0,6,4,8,2,1,9):

    (r(14c−3)/24, 9r/32, −7r(4c+3)/96,
     9r/32, −7r(4c+3)/96, 0, 0).

This is the exact solution using the first five old-label columns of the
normal-closure-and-total-mass matrix. The standalone verifier checks normal
closure and total weight1 identically. Every coordinate remains positive over
the full c interval and0<tau<=1/100: coordinates proportional to r have positive
quotients, and all other coordinates have strictly positive corner minima on
the r,c,tau rectangle. They are multiaffine, so those corner checks are complete.

Direct pairwise expansion gives the exact score

    Q=1/2 + delta(eta−3r) tau −21r²(4c+3)tau²/64.

Here eta−3r>0. The negative quadratic term is uniformly too small to destroy
Q positivity. The HK capacity formula therefore gives

    c(K)<=1/(2Q),   rho(K)<=1/(8 V(K) Q²).

The first-order capacity loss is ell=2delta(eta−3r). With the adaptive choice,

    ell=eta²/8                  if0<eta<=1/2,
    ell=(eta−1/4)/8             if1/2<=eta<1.

This already dominates the leading removed-volume coefficient. The next finite
check upgrades that response comparison to a uniform interval in tau.

## Exact cap volume

At a fixed momentum p with p1>c, the top-q cut height is

    h=tau(p1−c)/(2−2c tau).

It is less than1/2 throughout the stated box. The q-factor slice width at height
h below its top is5/6+2h/3, so its removed area is5h/6+h²/3. Consequently

    V(K)=1/2−(5/6)tau I1/(2−2c tau)
              −(1/3)tau² I2/(2−2c tau)²,
    Ij=integral_{p in T,p1>c}(p1−c)^j dp.

The p-factor slice density is1+4p1 on[−1/4,0] and1−4p1/3 on[0,3/4]. Thus I1,I2
are exact piecewise polynomials; the verifier derives them by direct integration.
For c>=0 they are2eta³/9 andeta⁴/9. The exact negative-c formulas are retained
in certificate.json. This accounts for the ENTIRE cut-fraction interval rather
than only a triangular neighborhood of the rightmost p vertex.

## Exact uniform inequality certificate

To prove rho<1, it suffices to show8VQ²−1>0. Multiply by the positive denominator
(2−2c tau)². On three eta intervals,

    [0,1/2], [1/2,3/4], [3/4,1],

the adaptive r and moment formulas are fixed. The cleared polynomial is divided
by tau eta² on the first interval and by tau on the other two. These removed
factors are strictly positive in the genuine-cut domain. Map each eta interval
to p in[0,1] and tau to u/100,u in[0,1]. Every tensor Bernstein coefficient of
the resulting polynomial is strictly positive. Degrees and minimum coefficients:

| eta interval | degree in p,u | minimum exact Bernstein coefficient |
|---|---|---|
|0 to1/2|8,5|24664083392486821/39813120000000000|
|1/2 to3/4|6,5|24664083392486821/159252480000000000|
|3/4 to1|6,5|93697767728461649/1698693120000000000|

A polynomial in Bernstein form is a convex combination of its coefficients
on the closed unit box. Strict positivity proves the cleared inequality on all
three boxes, hence rho<1 for every stated c,tau, including coupled endpoint
approaches. The full exact coefficient lists are retained in certificate.json.
No sampled grid, floating sign estimate or unspecified gap is used.

The closed box endpoints are used only as algebraic limits. At c=−1/4 the old
q facet is no longer genuine, and at c=3/4 the extra facet is redundant; the
theorem itself uses the open c interval and does not apply an eleven-genuine-facet
HK formula at those degenerate endpoints.

## Reproduction and acceptance boundary

    uv run --script topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/facet-fraction/verify.py

The verifier imports no experiment producer. It reads only the retained rational
P* geometry, recomputes the positive section and exact cap-volume moments, and
reconstructs all Bernstein coefficients. Its output is local certificate.json.
The scientific implication also uses the HK feasible-action formula and the
existing exact base geometry; those are explicit mathematical inputs.

The substantive next rival is a tilt with p2 or q-edge-coordinate dependence,
which introduces the additional quadratic phase term in the duplicated-label
response. Passing this slice does not justify a full F11 claim. A moving-normal
proof must additionally combine its cut-depth margin with the F10 equality
parameters and quadratic transverse direction, with uniform feasibility.
