# Sufficient analytic bridge for one open facet-split patch with moving old normals

Lead derivation, 2026-10-04 23:34 UTC; independently accepted23:44 UTC.
The exact new-section cancellation and equality-family premises are supplied by
[the retained packet](facet-split/moving-old/). Independent consequential review
accepted both these premises and the assembled analytic implication; its retained
record is [independent-moving-old-review.md](independent-moving-old-review.md).
The existing P* F10 proof supplies its stated transverse decomposition and margins.
Acceptance is relative to those mathematical inputs, not formalization or a
novelty claim.

## Accepted open-patch theorem

There are neighborhoods of P* in the full ten-facet normal chart and of
`D0=(0,−149/100,1,0)` in direction space, and a common positive tau window,
such that adjoining the facet `a_new=a_0(A)+tau D`, with support height1,
gives ratio strictly below1 whenever tau>0. All ten old normals may move,
including nonproduct motions, both equality-family parameters and the remaining
quadratic transverse direction. Harmless symmetry actions are transported with
the direction patch. At tau0 the inherited F10 theorem gives ratio at most1.
All eleven facets are genuine on the chosen tau>0 patch.

The neighborhood and window are existential. This covers one open patch of
split directions; it does not establish the full all-direction F11 neighborhood.

## Precise sufficient premises

Use the existing full normal chart

    A=A(v,s)+z W(v,s)+Y y,

modulo translations, scaling and linear symplectic maps. Equality-family parameters
(v,s) lie in a small compact patch; y has22coordinates, z is the remaining scalar
transverse direction. The existing24positive touching upper sections U_j(A)
have rank22 gradients G_j, positive normalized cancellation weights and common
strict z curvature at the base, −13/432. Their first-order rows annihilate symmetry,
the two equality tangents and W throughout the parameter patch.

Fix a facet f and a split-direction patch D around one direction D0. Its cut
function D.x on the old facet has no zero values at facet vertices and cuts a
nonempty proper portion of that facet. The extra normal is

    a_new=a_f(A)+tau D,   tau>=0,

with every support height1. These strict sign conditions persist on one small
A,D patch. Cap volume admits a uniform expansion

    V_cut(A,tau,D)=V(A)−tau L(A,D)+O(tau²),

with L smooth on this patch. This follows by the stable face-sign triangulation,
or directly by integrating the cap height over the facet. It is not asserted
uniform over changing sign cells or cut fractions approaching zero.

Assume a new duplicated-label positive HK section U_*(A,tau,D) has been supplied,
with a regular rank-five closure minor and exact contact at tau=z=y=0 throughout
the equality-family patch. It has merged old-normal gradient G_* and

    G_* W=0,     partial_zz U_*|_{tau=z=y=0}<0.

Assume smoothly varying normalized strictly positive coefficients gamma,lambda_j satisfy

    gamma+sum_j lambda_j=1,
    gamma G_*+sum_j lambda_j G_j=0.

This is a relation among the MERGED old-normal gradients. At the base define ell(D)
by the new section's capacity upper action derivative −ell(D) in the split
parameter tau. For the old sections, with the new label unused, that derivative
is zero. Require the strict response margin

    gamma ell(D)>L(A(v,s),D).                            (M)

All assumptions must hold uniformly on one sufficiently small parameter/direction
patch, by exact identities and finite strict base predicates. A base-only span
check does not establish contact/cancellation across the equality family.
Choose these coefficients as functions of the equality parameters (v,s) and,
if needed, D, and hold them fixed when differentiating z,y,tau. No merely
continuous dependence on the transverse variables is used. A fixed positive
gamma and a nonsingular old-gradient minor give a smooth lambda continuation
once the added gradient lies in the same22-dimensional rowspace throughout
the family.

## Conclusion

Under these premises, there is one neighborhood of the base such that every
body in this chart with tau>0 and D in the direction patch has ratio below1.
For tau0 the existing F10 theorem gives ratio at most1. Thus this proves a genuine
coupled old-normal/moving-equality-family/new-facet theorem on the stated open
split-direction patch. It is not the whole F11 neighborhood, because directions
outside that patch and changing cut-fraction strata remain omitted.

## Uniform proof

For each old section, keep its new label unused and evaluate its capacity
upper score on the cut body. Its score is unchanged but the volume decreases,
so it remains an upper bound for the cut body's ratio. The existing cusp estimate
and uniform cap-volume expansion give constants alpha,C>0 with

    min_j(U_j^cut−1)<=−alpha |y|+C(|y|²+z²+tau).          (1)

Define the weighted mean of the new and old cut upper sections

    F=gamma U_*+sum_j lambda_j U_j^cut.

Every summand dominates the true ratio, so F does also. Exact contact and gradient
cancellation give F=1 and F_z=F_y=0 at tau=z=y=0. All constituent z curvatures
are strictly negative, so F_zz is bounded above by a negative constant. Since
the old volume is1/2 and every old score is1/2 at contact, the derivative in tau is

    F_tau=2L−2gamma ell<0.                              (2)

Here the factor2 comes from the base volume1/2 and capacity1. This is the reason
one must compare gamma ell with the actual volume coefficient; a favorable
response from one unbalanced section alone is insufficient.

Regular rational section functions and the stable cut-volume formulas now give
uniform b,m,C'>0 with

    F−1<=−b z²−m tau + C'[
       |y|²+|z||y|+(|z|+|y|)³+tau(|z|+|y|)+tau²].       (3)

Only the tau0 old-body sections need the ordinary third-order Taylor control:
the uniform cap expansion, smooth L and regular capacity sections separately
give the tau-linear coefficient with remainderO(tau(|z|+|y|)+tau²). Full joint
third-order differentiability of the cut volume at tau0 is unnecessary.

Choose M large enough that alpha M exceeds twice the C in(1). If
|y|>=M(z²+tau), estimate(1) is strictly negative for small variables. If
|y|<M(z²+tau), every positive term in(3) is o(z²+tau), uniformly; hence(3) is
strictly negative whenever z²+tau>0. In the remaining case z=tau0,y!=0, estimate(1)
is negative. This establishes one uniform neighborhood, including arbitrary
nonlinear paths and all relative scales of z,y,tau.

The argument does not require knowing a numerical radius. The full local group
coordinate map from the existing F10 proof has invertible differential, so the
same conclusion covers all old-normal motions, after the harmless group action,
provided the direction patch is transported by that action. No independent
normal or product restriction is introduced into the old body.

## Specific accepted premises

For facet0, D0=(0,−149/100,1,0), the adaptive stationary continuum gives a positive
section with old facet weight r=1/1200 and new weight delta=1/2400. Its ell is
1/480000, whereas L=1/86400000. Thus a cancellation coefficient gamma>1/180
would suffice. The retained exact positive relation has gamma=1/125 and

    gamma ell−L=11/2160000000>0.

The independent audit recomputed the merged full40-dimensional gradient,
normal-closure derivative, duplicate-label response and volume coefficient.

The extra old section's z curvature can be realized geometrically: along A_*+zW_*,
keep its p weights fixed and correct q weights linearly in z to preserve closure
and total mass. All within-q pairings vanish; all cross q/p pairings are unchanged;
within-p pairings are affine in z. Therefore its score is affine in z. The checked
condition G_*W=0 forces that score to be exactly1/2, so the upper ratio along this
line equals864/(864+13z²), with the same strict curvature as the existing sections.
The required positive weights persist near z0. The retained symbolic family
identities continue this section coherently through the equality-family patch.

## Failure boundary

The parameter-contact identities, positive cancellation, rank, new z curvature
and response margin are now accepted on this patch. Their validity does not
extend to arbitrary split directions or endpoint-relative limits. A failed
extension premise would block that extension, without showing an improving
F11 body. The named remaining endpoint requirements are recorded in
[the transverse cap reduction](transverse-cap-reduction.md).
