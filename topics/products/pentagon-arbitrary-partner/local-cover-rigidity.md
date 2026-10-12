# Local rigidity of the regular-pentagon normal-triangle cover

Status: new mathematical derivation in the present session, with an executed exact
finite certificate. The continuum argument below is supplied explicitly for
independent review; an exact arithmetic PASS is not itself a formalization.
The global arbitrary-partner problem is NOT solved.

## Statement

Let Q be the fixed regular pentagon in `pentagon_fixture.py`, and let
T_0,...,T_9 be its ten normal triangles. Let H be the capacity-one HKO partner,
A(H)=m, and let t_j^* be the fixture's placements, with
conv(union_j(T_j+t_j^*))=H.

There are constants c,epsilon>0 such that, after removing a common translation,

    A(conv(union_j(T_j+t_j^*+u_j))) >= m + c ||u||

whenever u_0=0 and ||u||<epsilon. In particular, these placements are an
isolated local area minimum modulo common translation.

Consequently H x Q is locally maximal among products K x Q with K an ARBITRARY
planar convex body (no facet bound). Equality locally consists precisely of
translations and positive homothets of H. The corresponding statement holds
at -H. This is not local maximality for general nonproduct four-bodies.

The field computation is in coordinates (x,y/tau), tau=tan(pi/5). This fixed
invertible planar coordinate change preserves the covering question, local
minimality, and all area ratios. It is not invoked as a symplectic symmetry
of a single factor.

## 1. A local polygonal lower-bound lemma

Let v_0,...,v_{n-1} be a strictly convex counterclockwise polygon. At each v_i
there are finitely many labelled points, initially coincident with v_i, which
are displaced by at most r. Additional points strictly inside the original
polygon may also be included in the convex hull P(u).

For each i choose an incoming copy x_i^- and an outgoing copy x_i^+ from the
cluster at v_i. They may be the same copy. Consider the cyclic polygonal loop

    x_0^-, x_0^+, x_1^-, x_1^+, ..., x_(n-1)^-, x_(n-1)^+.

Its signed shoelace area has the expansion

    A(H) + sum_i [c_i^- dot u_i^- + c_i^+ dot u_i^+] + O(r^2),
    c_i^- = J(v_(i-1)-v_i)/2,
    c_i^+ = J(v_i-v_(i+1))/2,

where J is counterclockwise quarter-turn. Uniformly over the finitely many
choices,

    A(P(u)) >= signed_area(loop) - C r^2.                 (1)

One must NOT replace (1) by an unconditional assertion that the signed area of
an arbitrary self-intersecting polygon is bounded by its convex hull area.

Here is why the local estimate is valid. The long segments between successive
clusters stay close to distinct sides of the fixed strictly convex polygon.
For sufficiently small r, non-neighboring such segments cannot intersect.
An intersection of neighboring long segments lies within C_0 r of their common
original vertex: solve the intersection of two lines whose limiting directions
are independent. Short within-cluster segments lie in these same small disks.
The only possible proper self-crossing near v_i is between the incoming and
outgoing long segments. If they meet at p, the short connector and the portions
from p to x_i^- and x_i^+ form a triangle inside that disk. Delete this small
loop and join the two long segments at p. This changes signed area by at most
C_2 r^2. The n vertex disks are disjoint; along a long side the possible
crossings at its two ends occur in the correct order for small r. Deleting
these at most n small loops leaves a simple counterclockwise polygon, entirely
inside P(u). Its area is at most A(P(u)), which proves (1). Coincidences and
collinear overlaps follow by a limiting perturbation. This avoids any global
claim about arbitrary winding-number polygons.

Expanding the exact shoelace polynomial proves the displayed linear part.
Thus if g_s are the finitely many resulting gradient rows,

    A(P(u)) >= A(H) + max_s g_s dot u - C' ||u||^2.        (2)

Only a lower first-order model is needed; no assertion about the full hull's
exact Hessian or a globally valid polygon bank is used.

## 2. The HKO incidence and the compressed certificate

Each of the five vertices of H is occupied by one vertex from each of exactly
five of the ten placed triangles. In the fixture's labels the incidence sets
are

    {0,1,5,8,9}, {1,4,5,6,7}, {0,1,2,3,7},
    {3,6,7,8,9}, {2,3,4,5,9}.

Translations u_j belong to R^2 and u_0=0, so the space has dimension 18.
There are ten choices of an endpoint slot (incoming/outgoing at each H vertex),
with five possible triangle labels at each slot. A row g_s is a sum of ten slot
rows. Naively the bank has 5^10=9,765,625 rows. They need not be enumerated.

Choose, for each slot k, strictly positive probabilities p_kj summing to one.
Require the sum of the expected slot rows to vanish in R^18. These are 28 linear
equations in 50 unknowns: 18 for gradient cancellation and 10 for slot sums.
The following deterministic exact construction succeeds. Write the equations
E p=b, set p0=(1/5,...,1/5), and set

    p = p0 + E^T (E E^T)^(-1) (b-E p0).

The verifier reconstructs all geometry and checks the following over the ordered
field Q(sqrt(5)):

* E has row rank 28;
* E p=b exactly;
* all 50 probabilities are positive, with minimum

      33/218 - 39 sqrt(5)/1090 = 0.07137004484...;

* differences of rows within individual slots have rank 18.

Taking the product distribution over the ten independent slots gives a strictly
positive convex combination of EVERY full row g_s, with mean zero. The affine
span of these rows is all of R^18 by the difference-rank check. Therefore zero
is in the interior of their convex hull. Compactness of the unit sphere gives

    max_s g_s dot u >= gamma ||u||,    gamma>0.           (3)

The rank also has a simple geometric explanation. If every within-slot
difference annihilates u, the two independent side coefficients at a vertex
force all five incident triangle translations to agree. The displayed incidence
hypergraph is connected, so all ten translations agree; anchoring u_0 makes
them zero.

Combining (2) and (3), then shrinking the neighborhood, proves

    A(P(u)) >= m + (gamma/2)||u||.

This is a uniform neighborhood argument, including nonlinear paths and directions
approaching other directions. It is not a raywise numerical test.

## 3. From placements to arbitrary nearby convex bodies

Every triangle has a pair of endpoints whose difference is a vertex of H-H.
The checker verifies this and their exact placement at H vertices. Such a pair
has a unique placement inside H. To see this, expose its difference vertex by
a linear functional: both corresponding exposed faces of H are singletons,
so the two endpoints are forced. The triangle's translation is then forced.

Consequently each T_j has precisely the fixture translation in H. If K_l tends
to H and contains translates T_j+t_j(l), boundedness gives subsequential limits
of the translations; every limit must be the unique fixture placement.
Therefore all such translations tend to t_j^* (uniform closeness follows by
the usual contradiction/compactness argument).

For a feasible cover K sufficiently close to H, choose its ten translations,
remove their common translation, and apply the placement theorem to their hull
P subset K. It yields A(K)>=A(P)>=m. Equality forces the placements to be the
common translate of the fixture, so H+z subset K; equality of areas of nested
convex bodies with interior gives K=H+z.

For the systolic ratio, normalize K by dividing its q-factor by c(K x Q).
This puts the product capacity at one, stays near H by continuity, and does not
change rho. The normal-triangle theorem identifies these normalized bodies with
feasible covers. The area statement proves the ratio bound and its local
equality assertion. Restoring the normalization permits positive homothets.

## 4. Consequence for the global residual: a positive buffer

Assume the supplied sharp theorem for the prescribed ten-direction fan and the
exact difference-body slice has been independently accepted. Restrict to the
compact anchored translation domain containing every cover of area <=m.

Any hypothetical different cover at area <=m lies a positive distance from H
and -H: the local theorem excludes neighborhoods of both equality placements.
The hypothetical competing set is therefore compact after normalization.
It is disjoint from the closed fan class and from the locus P-P=H-H by those
restricted theorems. Hence there exists an existential positive buffer:

    distance(P, prescribed-fan class) >= eta,
    A(P-P) >= A(H-H)+eta',

for every such competitor, with eta,eta'>0. If no competitor exists the statement
is vacuous. The constants are NOT numerically computed. They cannot be silently
used as numerical pruning tolerances.

This removes a qualitative uniformity issue at the already solved slices. It
neither excludes remote non-fan competitors nor proves the global bound.

## Reproduction and trust boundary

    python verify_local_cover.py --fixture-code pentagon_fixture.py --out fresh.json

Requires SymPy. The checker reuses the supplied exact triangle fixture and
checks new equations; it does not certify the winding argument by computation.
The numerical pilot is ancillary and is not used in the proof. This proof and
its geometric lemma should receive an independent mathematical review before
being promoted to an accepted manuscript theorem.

External input for the capacity interpretation: Balitskiy--Mitrofanov--Polyanskii,
Triangle covering problems and the Viterbo inequality in the plane,
arXiv:2603.12495v2, Theorems 2.5 and 3.3.
https://arxiv.org/html/2603.12495v2
