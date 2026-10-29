# Fresh assembled analytic audit of the HKO inward ascent

28 October 2026. Reviewer: `/root/hko_ascent_math_audit`.

**Verdict:** I find no mathematical gap in the assembled implication from the
fixed-bank derivative theorem, the characteristic-invariant repeated-word
reduction, and the complete simple-word certificate to a strict local
improvement of HKO. In particular, the variable-weight objection to repeated
deletion is resolved by a genuine geometric restriction, not by an invalid
general convexity assertion. The exact numerical certificate remains a
separate computational input, with the independent reviews identified below.

The conclusion is

\[
D^+\log\frac{c_{\rm EHZ}(K_t)^2}{2\operatorname{vol}_4(K_t)}
=\frac{20+64\sqrt5}{9375}>0.
\]

Consequently every sufficiently small positive parameter improves the ratio.
This disproves unrestricted Hausdorff local maximality at HKO. No particular
nonzero parameter or certified finite improvement size is supplied by this
derivative argument.

## 1. The difficult repeated-word step

Let a maximizing positive repeated word contain the same full normal at
positions r and s. Transfer weight by beta_r -> beta_r+z and
beta_s -> beta_s-z, on the complete positivity interval. Closure and total
mass are unchanged. The action is **affine**, since its quadratic coefficient
in z pairs the repeated normal with itself. Affineness alone is not
constancy. The repeated-word upper bound Q <= Q*, together with the interior
maximum Q(0)=Q*, proves constancy throughout the interval.

For a product of planar polygons with no antipodal facet normals, a positive
action word uses both factors. Each factor's positive closure spans its
plane. The augmented normal matrix consequently has rank five: a relation
v.a_i+c=0 on its support first gives c=0 on averaging by beta, then v=0.
Thus the KKT multiplier is unique and affine in z. Reconstruction places
the loop affinely in z and in the body for the whole interval.

For a repeated q-normal a=(n,0), let v=2TJa=(0,2Tn), with the convention
J(q,p)=(-p,q). Choose the cyclic start at the first duplicate. Every
fixed-weight segment translates with derivative either u or u+v, depending
on its location between the duplicate occurrences.

Every q-facet equation implies n_j.u_q=0, independent of which interval
contains the segment, because v_q=0. The q-normal span gives u_q=0.
Every p-facet equation implies either m_j.u_p=0 or
m_j.(u_p+v_p)=0. There are at least three distinct p-normal directions.
Partition their occurrences between the two intervals. At least one
interval contains two nonparallel directions; its equations force
u_p=0 or u_p=-v_p. Hence **u is either 0 or -v**.

This is exactly the extra fact needed for variable weights. Both duplicate
segments, including their moving endpoints and changing lengths, remain on
fixed affine lines parallel to Ja. Their own profiles are constant on those
lines. Their response is therefore

    (beta_r+z)c_r + (beta_s-z)c_s,

an affine function. Every other weight is fixed; its segment translates
affinely in z, so integrating a convex facet profile produces a convex
function of z. The total response is convex. No assertion that
beta(z) f(x(z)) is generally convex is being used.

Continuity on the compact facets extends response to the endpoints, where
the vanishing weight contributes zero and can be deleted. Endpoint loops
remain in the body and have Q=Q*. Their positive supports still have rank
five. At least one endpoint has response no smaller than the original.
Repeat until the word is simple. The p-normal case interchanges the factors.
Continuity of the profile on the disjoint labeled facet union passes the
bound to the weak closure of finite maximizing-word measures.

The previously exhibited convex quadratic counterexample is consistent with
this proof: that quadratic is not invariant on its own characteristic line.
Its duplicate contributions therefore need not be affine.

## 2. Fixed-bank derivative and the all-word interface

I rechecked the argument in `multicut/theorem.md`, rather than assuming a
formal Danskin theorem at a singular constrained optimizer.

For the fixed twenty-row bank there are finitely many simple permutations.
A maximizing sequence has a subsequence with fixed permutation and
convergent weights. At its base optimizer a positive invertible augmented
minor lets one solve five weights smoothly; the remaining weights give
local nonnegative coordinates. Comparing the objective at the same free
coordinates at t and 0 provides the upper difference-quotient bound.
Holding those coordinates fixed at any base optimizer provides the lower
bound. Compactness and the finite permutation set give the actual right
derivative. This does not need a nonsingular Hessian or an isolated optimizer.

With T=c(K), reconstruction is x_r=-T lambda+2TJ sum_{k<r} beta_k a_k.
Stationarity gives eta=2Q*=1/T and the active facet equations. Insertion of
any original facet at any loop point, compensated in a positive invertible
minor, gives reduced cost (a.x-1)/T <= 0. Thus these reconstructed loops are
in the body, including repeated words; this containment is needed above.

The midpoint derivative is

\[
Q'=T^{-1}\sum_r\beta_rD_r\cdot
 (x_r+T\beta_rJa_r).
\]

The self-pairing term in the direct derivative is present. Thus assigning one
bank row per segment bounds Q' above by T^{-1} times the profile response.
Conversely, a finite repeated maximizing base word can be subdivided at the
finitely many switches of the maximum-of-affine profile and continued by a
feasible section. The polar-loop inequality bounds its moving action by the
actual optimum, even if a moving row repeats. This proves the matching lower
bound. The sensitivity optimization itself still uses the fixed finite bank;
the repeated words are comparison candidates, not an infinite Danskin domain.

The resulting identities are

\[
Q'/Q_*=2\max_\mu\mu(f),\qquad
c'/c=-2\max_\mu\mu(f),\qquad
V'/V=-4\nu(f).
\]

Consequently D+ log sys = 4(nu(f)-max mu(f)). The action/energy convention
also checks: a loop with velocity 2Ja on intervals beta has action 2Q and
dual energy one, giving c <= 1/(2Q). I have not independently retrieved the
primary paper behind that established variational input in this audit.

## 3. Completeness of the finite simple optimization

Every simple word embeds in a full ten-label cyclic order by insertion of
zero weights. Cyclic action invariance under closure allows label zero first,
so 9! orders suffice without an orientation quotient.

Each planar normalized closure polytope has rank-three affine constraints.
Every extreme point therefore has at most three positive weights; one or two
cannot close, because there are no zero or antipodal normals. The exact
enumeration of all triples covers every vertex.

For a word with factor mass t and normalized factor weights u,v,
Q=t(1-t)B(u,v). Its value is an average of vertex-pair values after decomposing
u and v into closure vertices. The exact primitive bound on all 3000 oriented
six-words gives B <= 4Q*. Equality Q=Q* forces t=1/2 and every pair with
positive decomposition coefficient to be an equality pair. Those pairs form
a biclique. Extension to maximal bicliques and deletion of labels absent
from the entire chart preserve every optimizing family. The deduplication
key retains the cyclic word and both vertex masks, which is sufficient.

The exact catalogue reports only point and interval charts. On an interval,
weights and KKT positions are affine. Own-characteristic invariance makes
each hinge constant on its segment, so response is exactly

    sum_i (b_i+z d_i) max(0,h_i+z k_i).

All hinge roots, interval endpoints, and interior stationary points of
concave quadratic pieces suffice to maximize it. This is valid even though
response on such simple-word intervals need not be convex: here the
algorithm explicitly optimizes the full piecewise quadratic.

I read the exact producer and its supplied outputs. I did not independently
rerun the exact enumeration. `hko-intersection-falsifier/catalogue-coverage-audit.md`
records a separate Sage primitive calculation and independently implemented
Python scan matching the complete 190-chart set. `multicut/complete-simple/independent-review.md`
records the separate exact response rerun. Those remain the exact arithmetic
acceptance dependencies.

As a new cross-check, `independent_numeric.py` uses trigonometric normals,
fresh linear closure solves and least-squares KKT reconstruction, without
importing the producer's field or loop code. It recomputes all 190 supplied
chart maxima: maximum 0.008000000000000113, and maximum chartwise difference
from the exact output 1.60e-16. This is diagnostic corroboration, not an exact
proof or a fresh catalogue enumeration.

## 4. Explicit family, characteristic invariance, and normalization

Let c=cos(pi/5), J2(x,y)=(-y,x), and

    L_t(q,p)=((1-3t/5)q-c t J2 p,
              (1-3t/5)p+c t J2 q).

Every support-normalized pentagon facet normal has length 1/c. Pulling a
q-facet back through L_t gives row velocity
(-3n/5,c J2n)=(-3n/5,J2n/|n|). Pulling a p-facet back gives
(-c J2m,-3m/5)=(-J2m/|m|,-3m/5). These are exactly the rows in RESULT.md.
Including the original rows therefore gives K_t=H intersect L_t^{-1}H.

On its original facet each moved row has value ell-3/5, so the maximum with
the retained row is (ell-3/5)_+. For a q-facet Ja=(0,n), and
(J2n).n=0; for a p-facet Ja=(-m,0), and (J2m).m=0. Hence both hinge profiles
are invariant in their own characteristic directions and convex.

The referenced coordinate ell points toward a vertex of the opposite
unit-circumradius pentagon. A cap of depth delta=2/5 does not reach the
adjacent vertices, since delta < 1-cos(2pi/5). At depth s below the vertex
the cap width is 2s cot(pi/5). Therefore

\[
\operatorname{area}(cap)=\delta^2\cot(\pi/5),\qquad
\int_{cap}(\ell-1+\delta)=\delta^3\cot(\pi/5)/3.
\]

The latter is the hinge integral, not the cap area. Dividing it by the
pentagon area 5tau/(1+tau^2), and averaging the ten identical facet means,
gives

\[
\nu(f)=\frac{8}{625}-\frac{8\tau^2}{9375}.
\]

Each original facet has cone-volume mass 1/10; the hinge is imposed on all
ten facets. An independent clipping and triangular integration in
`independent_numeric.py` gives this same mean 0.012349556014932974.

The new inequalities define bounded full-dimensional bodies for all small
t, because the old bounded body contains their intersection and the origin
remains strictly feasible. They converge in Hausdorff distance to H. There
are open portions of every old facet with ell<3/5 and with ell>3/5, so both
the retained and split facets are genuine for sufficiently small positive t.
Thus the family has twenty genuine facets near zero, although at most twenty
would already suffice for the conclusion.

Finally tau^2=5-2sqrt(5), so

\[
4\left(\frac8{625}-\frac{8\tau^2}{9375}-\frac1{125}\right)
=\frac{20+64\sqrt5}{9375}>0.
\]

Existence and positivity of this right derivative give a positive delta_0
such that log sys(K_t)>log sys(H) for every 0<t<delta_0. No uniformity across
varying banks or growing facet counts is needed for this counterexample to
local maximality: this one fixed finite family already converges to H.

## Corrections and remaining limits

I reported a false parenthetical in the first repeated-word review: equal
normal vectors do not alone make the action invariant under transfer. Its
author was asked to replace it by the affine-plus-global-maximality argument.
The actual elimination proof already used that correct argument. The authors
also identified the cap-area versus hinge-integral wording correction and
clarified that a positive directional derivative does refute local maximality.
These are exposition repairs, not missing mathematical branches.

This audit does not establish novelty, a numerically explicit improving t,
an optimal member of the family, or global optimality of any resulting body.
It uses the classical HK/polar variational capacity formula and the separately
audited exact finite certificate. The numerical file is deliberately labeled
as a cross-check. Root owns retention, integration, commits and source/PDF
review; this agent has changed only its assigned temporary directory.
