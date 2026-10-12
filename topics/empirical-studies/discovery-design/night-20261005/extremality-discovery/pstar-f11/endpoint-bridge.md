# Uniform endpoint implication for the supplied moving p1 tilt

Lead derivation, 2026-10-05 00:07 UTC; independent assembled acceptance00:16 UTC.
The [retained exact section packet](endpoint-moving-normal) supplies all four
premises below. Independent review and arithmetic audits accept those premises,
the [global cap estimate](endpoint-cap-geometry.md) and the assembled analytic
implication: see [the review](independent-endpoint-moving-normal-review.md).
Acceptance is relative to the inherited HK feasible-section formula and P* F10
proof inputs, not formalization or a novelty claim.

## Accepted restricted endpoint theorem

There exist positive common eta,tau windows and an old-normal neighborhood in
the full F10 chart such that every genuine eleven-facet body obtained by

    A=A(v,s)+zW(v,s)+Yy,
    D(v,eta)=(0,−2(1−v+v²−eta),1,0),
    a_new=a0(A)+tau D(v,eta)

has ratio strictly below1, for all positive eta,tau in those windows. Both
equality parameters range in a common patch; all old normals may move,
including nonproduct motion. All coupled relative scales are covered. The
tilt family is transported under the same local symmetry normalization as
the F10 chart. The windows and neighborhood are existential, not numerical.
This is the specified moving right-p1 endpoint, not an all-direction F11
neighborhood theorem.

## Target and exact premises

Use the full old-normal chart

    A=A(v,s)+zW(v,s)+Yy,
    u=1−v+v²,
    D=(0,−2(u−eta),1,0),
    a_new=a0(A)+tau D,

where v,s lie in a small compact equality-family patch and eta,tau are
positive. Work modulo the same symmetry group as in the accepted F10 chart;
the specified tilt family is transported with that action. The old body has
volume V(A), the cut has volume `Vcut=V(A)−DeltaV`.

The F10 chart supplies a uniform actual deficit through old witnesses,

    min(old ratio upper witnesses) <= 1−k d,
    d=|y|+z²,  e=|z|+|y|.

The quantitative proof is retained in
[transverse-cap-reduction.md](transverse-cap-reduction.md). The cap estimate,
now proved globally rather than assumed, is

    DeltaV <= C tau(eta+e)³.                           (C)

The new section premises are as follows. Use the stationary family contact
with `r=eta/6`, and duplicate facet0 with weight `delta=r/2=eta/12`.

1. The stationary contact and regular pivot continuation extend rationally to
   eta0. All small base weights have positive lower bounds proportional to
   eta, and all other base weights have uniform positive lower bounds.
   Perturbations of the old weights are bounded by `C(e+tau eta)`; the new
   weight is fixed delta. Thus the section is positive when e/eta and tau
   are uniformly sufficiently small.
2. The merged contact gradient has a bounded smooth strictly positive
   balance with the old24 gradients, with one fixed gamma>0, uniformly on
   the equality-family patch and `0<=eta<=eta0`. Normalize its coefficients
   to sum one and hold them fixed when varying z,y,tau.
3. Its uncut-volume formal ratio section has the same strictly negative W
   curvature as the old sections. The coherent q-only W weight jet gives
   Q=1/2 along the actual W line for every family r, including the algebraic
   r0 limit. The common volume gives uniform negative curvature.
4. With uncut volume used in all formal ratio scores, the weighted mean
   F(A,eta,tau) satisfies the exact section response estimate

       F(A,eta,tau)−F(A,eta,0)
         <= −m tau eta²+C(tau eta e+tau² eta²),         (S)

   for uniform m>0. The supplied exact algebra gives
   `Qnew−Qold=tau delta L+tau²delta² H`, with regular bounded L,H,
   `L_family=eta−4ur`, and `L−L_family=O(e)`.
   Hence `ell=2delta L_family=(eta²/6)(1−2u/3)` is uniformly positive.
   The factor delta in every tau-linear old-motion term is essential.

These accepted mathematical premises are not conclusions from a sampled cut
grid. Their retained derivation and exact checker are owned by
`endpoint-moving-normal/`. The endpoint balance uses fixed gamma1/100000;
its algebraic base interval is `0<=r<=1/1200`, with r=eta/6. The final eta
window can shrink below1/200 to absorb the cap and analytic remainder terms.

## Outer region: old witnesses already control the cut

Capacity monotonicity multiplies each old ratio upper bound by `V/Vcut`.
Uniform boundedness and positive lower volume imply

    rho(cut) <= 1−k d+C0 DeltaV.

Thus the cut is strictly controlled whenever `d>0` and
`DeltaV<=k d/(2C0)`. Also `d>=c e²` for small variables.

If `e>=eta` and e>0, (C) gives `DeltaV<=8C tau e³`; choosing one small
common neighborhood makes this less than `k d/(2C0)`. Therefore every path
still requiring a new section has `e<eta` and

    d <= C1 tau eta³,
    |z| <= C2 sqrt(tau) eta^(3/2),
    |y| <= C2 tau eta³.                               (R)

This includes d0, so equality-family points are not discarded. No relative
motion hypothesis was imposed on the theorem: (R) follows exactly on the
paths not settled by old witnesses.

## Inner region: positive balanced section and scaled errors

In (R), `e/eta<=C[sqrt(tau eta)+tau eta²]`. Shrink the common eta,tau windows
so premise1 gives a genuinely positive feasible new section throughout this
region. All old sections remain capacity upper witnesses for the cut.

At tau0, contact and exact gradient balance give

    F(A,eta,0)−1
       <= −b z²+C2[|y|²+|z||y|+e³],                  (T)

uniformly also as eta tends to zero. This uses a regular rational section
at the algebraic endpoint; positivity is needed only at eta>0 in (R).
The common negative W curvature supplies b>0. Combining(T) and(S), and
then multiplying the mean by `V/Vcut`, gives an actual cut ratio upper bound

    rho(cut)−1 <= −b z²−m tau eta²
        +C3[|y|²+|z||y|+e³+tau eta e+tau²eta²+tau eta³].

The last term follows from (C) and e<eta. In (R), division of these positive
terms by the strictly positive `tau eta²` gives bounds of the respective
orders

    tau eta⁴,
    sqrt(tau) eta^(5/2),
    sqrt(tau) eta^(5/2),
    sqrt(tau eta)+tau eta²,
    tau,
    eta.

They tend uniformly to zero when the common windows shrink, regardless of
the relative rates of eta,tau,z,y. Choose them so their total is less than
m/2. Then `rho(cut)<1` throughout the inner region. The outer region was
already strict. This proves the stated common-window endpoint implication.

## Honest quantifier and residual domain

The implication proves a restricted endpoint theorem with all old normals
free in the full F10 chart, both equality parameters free, and arbitrary
coupled eta,tau approaches. It is about the specified transported p1 tilt,
not arbitrary new-normal direction. Restrict the conclusion to bodies with
eleven genuine facets. No explicit numerical neighborhood radius is supplied.
Other facets, q-edge/p2 tilt, the left fraction endpoint and general limiting
support strata remain outside this argument.
