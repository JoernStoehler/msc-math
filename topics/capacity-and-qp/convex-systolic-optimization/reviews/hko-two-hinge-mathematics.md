# Independent audit: two characteristic-invariant affine hinges

29 October 2026. Reviewer `/root/hko_ascent_math_audit`; root owns selection,
integration and commits.

**PASS.** For every nonnegative affine hinge on a single labeled HKO facet
that is invariant along its characteristic direction,

    R(f) >= 2 nu(f).

Consequently two such retained-row cuts have nonpositive first systolic
variation, on either distinct facets or the same facet. The latter uses the
actual profile max(f,g), not f+g. No finite-neighborhood conclusion, general
twelve-facet local maximality, or statement about non-invariant hinges is
established by this result.

## Reviewed inputs and replay

The initial proof hash was
`1941cb119eda0c78d5cb142f71ece934249d88d92029fe38316ba07be0c5095f`.
The author corrected the last quotient comparison's interval to k<=z<1,
avoiding division by zero at z=1, which was already covered by the zero-hinge
case. That intermediate proof had hash
`a123270903582957852e104afc87d26f1d618b386788f8c9cc44417b212e99d6`.
The subsequently added same-facet statement and review status agree with
the approved reader module and were included in the final PASS. I reconfirmed
the actual source hash at 01:10 UTC; the final approved amplification-proof.md
hash is:

    7fcf285f7627e57459de71aea5f97591f7bb17325c270fb1f66ba4f772265247

The complete reader module, including literal family endpoint vectors and
the same-facet extension, was reviewed at hash:

    two-hinge-obstruction.tex
    e20b6fc6a49ac3cf04cf56299249ff667d00e0e6234c6c501faa21cba1674d94

I ran the initial exact checker, hash
`96db97f933be8190f9dd3723b1ad6f34cfbb5c487761fcf68ae86905e79d7d21`,
with a fresh output path. All four-family, projection and scalar predicates
passed in 0.17 seconds. Output:
`/tmp/msc-math/restart/hko-ascent-audit/two-hinge-amplification-replay.json`.
I separately compared every printed word and endpoint weight vector against
that exact output, reversing endpoint order when needed so y increases.
All four literal tables match.

The final checker adds an explicit --source option, selects raw or expected/
JSON inputs, imports the specified exact_core.py, and disables bytecode
writes. I inspected that portability-only delta. The mathematical checker
body is unchanged. Current checker hash:

    82976ec9ac4d7026f91469e37bbcaff43fd53ce8fb0172b4bc410554d9435c78

The existing exact_core arithmetic is a declared reused dependency. The
numerical probes in this directory were not used as proof premises.

## Quotient coordinates and measures

On the q-facet with normal n_0=(1,tau), let N=|n_0|=sqrt(5)-1 and
e=J2 n_0/N. The unit normal coordinate in q is fixed. Its orthogonal edge
coordinate ranges from -sin(pi/5) to sin(pi/5), so the normalized x ranges
over [-1,1]. The characteristic direction is (0,n_0); invariance removes
exactly the partner coordinate parallel to n_0. Every remaining affine
function therefore has form ax+by+c. Its constant term is unrestricted,
because a normal component of D is constant on the facet.

The partner pentagon is -J2 P. In its cyclic vertex order the normalized
projection levels are (-1,-1,k,1,k), k=sqrt(5)-2. The transverse coordinates
at those vertices are (-sin36, sin36, sin72, 0, -sin72). Thus the two
boundary slices between successive levels have affine widths, with nonzero
bottom width, matching the two density formulas. There is no missing
intermediate vertex level or unaccounted projection atom.

Uniform facet measure is the product of uniform edge measure and uniform
partner-pentagon area. Hence x is uniform on [-1,1] and independent of y.
The y-coordinate change contributes its half-range h to the projected
density. The checker includes that Jacobian and divides by the pentagon
area. Exact integration gives mass one and mean

    mu=-1+2sqrt(5)/5.

Every original facet has normalized cone-volume mass 1/10. Therefore
nu(f)=(1/10)E f(x,y). There is no additional factor from the two endpoints
of the edge or from the two partner slice intervals.

## The four families are actual shortest characteristics

The verifier checks the full closure and normalization at both endpoints,
nonnegative endpoint weights, strictly positive midpoint weights, and action
Q=tau/5 at both endpoints and the midpoint. The objective is quadratic in
the interpolation parameter; those three action checks imply it is constant
throughout the interval. Every coordinate is positive in its interior,
because it is an affine interpolation of nonnegative endpoints and is
positive at the midpoint.

One fixed independent five-row augmented system reconstructs the multiplier
at both endpoints, and the full stationarity equations are checked on every
row. Gradients and reconstructed loop vertices are affine in the weights,
so stationarity holds throughout the interval. Every vertex is checked
against every original facet inequality; each labeled segment's endpoints
also satisfy its own facet equation. Convexity then places the entire segment
in that facet and in H. Thus these are geometric characteristics, not just
formal feasible weights. Their constant action equals the known HKO capacity.

For the facet-zero segment, x=+1 or -1 is checked exactly, y spans the claimed
interval affinely, and its weight is exactly A(y+3) or B(1-y). The radial
coordinate is constant along that segment. The zero-weight endpoint at y=1
is harmless: deleting that segment or taking the weak occupation limit gives
the same zero contribution. At y=k the two distinct weights belong to
different valid families; no continuity between those families is assumed.

The profile is zero on every other **labeled** facet copy. Hence each tested
family contributes beta_0 f(x,y), even when the geometric segment also lies
on another facet at a ridge. This agrees with the labeled occupation measure
in the derivative theorem.

Only these four families' existence is needed. Although the checker locates
their endpoint data in the catalogue, it rechecks the requisite predicates
and the reader module prints the complete literal data. No assertion that
these four families, or all 190 catalogue charts, exhaust orbit responses
enters this lower-bound proof.

## Scalar inequality: all parameter regimes

Write b0=sqrt(5)/10 for the lower density coefficient, distinct from the
arbitrary hinge slope. The claimed bound is I(h)<=5M(h), where M uses both
weight curves with their respective closed intervals.

For zero slope, a nonpositive constant gives h=0. A positive constant is
handled by beta_L(k)=b0 and 5b0=sqrt(5)/2>1. For nonzero slope, positive
homogeneity reduces to the decreasing hinge (z-y)_+ or increasing hinge
(y-z)_+, with every real z allowed. Thus offsets, signs and degeneracies
are covered by the following exhaustive cases.

**Decreasing hinge.** For z<=-1 the integral vanishes. On [-1,1], convexity
in the threshold z gives the endpoint-secant bound
I(z)<=(z+1)(1-mu)/2. The y=-1 family value is 2A(z+1), and
20A-(1-mu)=(5-sqrt(5))/10>0 proves the comparison, including both endpoints.

For z>=1 the full-support integral is z-mu. On
[1,z0], z0=2sqrt(5)-1, the chosen y=(z-3)/2 really lies in [-1,k]. Its
family response is A(z+3)^2/4. Expanding both sides verifies the exact square

    (5A/4)(z+3)^2-(z-mu)=(5A/4)(z-z*)^2,
    z*=1+4sqrt(5)/5.

For z>=z0, the y=k response is b0(z-k). The comparison's derivative in z
is 5b0-1>0, and its value at z0 is already nonnegative by the preceding
square identity. There is no uncovered large-positive-threshold regime.

**Increasing hinge.** For z>=1 the hinge vanishes, including z=1. For
z<=-1 its integral is mu-z. The difference 5b0(k-z)-(mu-z) decreases as z
increases and has its minimum on that half-line at -1, with value
(25-9sqrt(5))/10>0.

For -1<=z<=k, direct integration gives

    I(z)=mu-z+b0[(z+1)^2+(z+1)^3/6].

I independently recovered this formula by setting s=y+1 and integrating
(z+1-s)(s+2) from 0 to z+1. On [-1,0], the comparison with 5b0(k-z) has
second derivative -b0(z+3)<0. Its two endpoint differences are exactly
(25-9sqrt(5))/10 and (210-91sqrt(5))/60, both positive. A concave function
lies above its endpoint chord, so this proves the entire interval.

On [0,k], y=(1+z)/2 lies in [k,1], because 0<k<1/2. Comparing with
5B(1-z)^2/4 has second derivative 5B/2-b0(z+3), bounded above by
(25-19sqrt(5))/80<0. Its endpoint differences expand to
(555-233sqrt(5))/480 and (7/12)B(1-k)^2, both positive. The same concavity
argument applies.

For k<=z<1, only the upper density contributes. Its integral is
C(1-z)^3/6 and the candidate response is B(1-z)^2/4>0. Thus division is
legitimate and I/M<=2(C/B)(1-z)/3<=8/3<5, using (C/B)(1-k)=4. The omitted
endpoint z=1 was already handled by the zero case. All displayed signs,
endpoint values, the second-derivative bound and the square coefficients
were checked algebraically; the exact replay also checks the listed scalar
predicates. The original markdown's inclusion of z=1 in this quotient
display was a notation error, now corrected, rather than a missing case.

## From the scalar bound to amplification and two cuts

Convexity in x gives E_x f(x,y)<=[f(-1,y)+f(1,y)]/2. For each of those
two one-dimensional hinges, I<=5M and the corresponding two actual shortest
families give M<=R(f). The witnesses for the two endpoints may differ;
this is harmless because both are bounded by the same supremum R(f).
Therefore E f<=5R(f), and the factor 1/10 in nu yields R(f)>=2nu(f).
The already checked facet-transitive rotation and symplectic factor exchange
preserve the geometric invariance condition and transfer the result to every
original facet.

For distinct facets, f+g is the actual retained-row splitting profile.
Nonnegativity gives R(f+g)>=max(Rf,Rg)>=2max(nu f,nu g)>=nu(f+g).
For the same facet, the actual profile is h=max(f,g). Then

    R(h)>=max(Rf,Rg)>=2max(nu f,nu g)
         >=nu f+nu g>=nu(h).

The exact finite-bank derivative theorem therefore gives nonpositive first
variation in both cases. This reasoning does not require a common responding
orbit, a common occupation ensemble, or completeness of a finite orbit bank.
It also does not turn zero first variation into a finite-neighborhood result.

## Disposition

The full reader module is mathematically consistent with the checked proof,
including its literal endpoint vectors and same-facet extension. It is ready
for root's selected integration and manuscript review. The numerical probes
remain discovery records only. No canonical source edits or new delegation
were made by this reviewer.
