# A count-independent multicut lifting theorem at HKO

New derivation, 16 October 2026. This is an analytic proof conditional on two
accepted repository inputs specified below. Computational evidence is separate.
Use the repository convention

    rho(K) = c_EHZ(K)^2 / (2 Vol_4(K)).

## Statement

Let H=P(A0) be the HKO pentagon product, with its ten normalized facet rows.
Choose finitely many normals b_j^0 on the boundary of H's polar such that each
exposes a primal face F_j of dimension at most one. There exist neighborhoods
U_j of b_j^0, and constants epsilon,c>0, with the following property.

Normalize the old ten rows to the accepted local symmetry slice, write
r=||A-A0||, and set

    P(A)={x : a_i dot x <= 1, i=0,...,9},
    K(A,B)=P(A) intersect intersection_(b in B) {x : b dot x <= 1},
    delta=max_(b in B) (h_P(A)(b)-1)_+.

Here B is **any finite set, of any cardinality**, contained in union_j U_j;
set delta=0 when B is empty. If r+delta<epsilon, then

    rho(K(A,B))/rho(H)-1 <= -c (r+delta^2).              (T)

The constants are independent of |B|. In particular this class admits no
improving sequence even if the number of new facets tends to infinity.
Equality requires r=delta=0, hence K=H in the slice. Restoring translation,
positive dilation and linear symplectic changes gives the usual equality
orbit, whenever the normalized representation satisfies these hypotheses.

A compact set S of polar-boundary normals each exposing a primal vertex or
edge is covered by finitely many such U_j. Thus the same theorem applies
uniformly to added normals sufficiently close to any fixed such S. The
compactness/separation condition matters: it excludes approach to the polar
one-skeleton, whose points expose primal faces of dimension at least two.
The theorem supplies no constants uniform as this separation tends to zero.

## Accepted inputs and their exact scope

1. The sharp ten-facet local theorem, in its local slice, gives

       rho(P(A))/rho(H)-1 <= -kappa r,     kappa>0.     (1)

   This is the accepted F10 theorem, not merely non-strict maximality. Its
   finite feasible-section certificate and neighborhood implication are owned
   by [the theorem packet](../theorem/README.md) and the repository's
   [feasible-section proof](../../../../formal/hko-feasible-section-upper-branches.tex).

2. Every HKO primal vertex v has a positive touching HK word with rank-five
   closure/normalization constraints and a contact insertion slot. Inserting
   a new row b with weight z, while correcting five old weights, gives

       Q(b,z)=Q0+2 Q0 (v dot b-1) z+R_v(b) z^2.        (2)

   This identity and all-vertex coverage are the previously verified
   vertex-insertion construction summarized in
   [local-cut-lifting.md](../../../products/pentagon-arbitrary-partner/local-cut-lifting.md)
   and audited in `inherited/previous_math/SETUP_AND_AUDIT.md`, §4, inside the
   [accepted F11 archive](../f11-return-20261004/hko_f11_complete_proof.zip).
   The same archive's `inherited/code/exact_core.py` reconstructs the
   geometry, positive seeds and insertion polynomials. This note does not
   claim a new execution of the full exact F11 replay.

The finite-word capacity principle says that feasible weights with Q>0 give
c_EHZ(K)<=1/(2Q). They need not be optimal weights, and unused rows of the
polar cause no problem. This is the only capacity computation needed below.

## 1. Uniform quadratic capacity loss from one deepest cut

All b in the finitely many sufficiently small U_j lie in a bounded set.
For each of the finitely many HKO vertices, the correction coefficients in
(2) are bounded linear functions of (b,1), and R_v(b) is a polynomial of
degree at most two. Their norms are uniformly bounded. The finite old words
have a positive minimum weight.

Put delta0(b)=(h_H(b)-1)_+. For delta0(b)>0 choose a maximizing vertex v
and z=mu delta0(b), with one fixed sufficiently small mu>0. Feasibility and
positivity of old weights hold for all sufficiently small delta0(b), and

    Q(b,z) >= Q0 + (2 Q0 mu-M mu^2) delta0(b)^2
            >= Q0 (1+a delta0(b)^2)                    (3)

for a>0 independent of b. When delta0(b)=0 take the old word with the new
row unused. Consequently

    c_EHZ(H intersect {b dot x<=1})
        <= c_EHZ(H)/(1+a delta0(b)^2).                (4)

For a finite B choose b maximizing delta0(b), and put
Delta0=max_B delta0(b). Monotonicity immediately yields

    c_EHZ(K(A0,B)) <= c_EHZ(H)/(1+a Delta0^2).         (5)

One does **not** sum capacity improvements or combine several inserted words.
A single deepest cut suffices, regardless of how many other cuts are present.

## 2. Union volume loss without a factor |B|

Shrink the old-row chart so P(A) is simple with the HKO face lattice. Label
its vertices continuously; F_j(A) is the same labelled vertex/edge as F_j.
Because b_j^0 exposes precisely F_j, all old vertices outside F_j have a
strict support gap. Shrink U_j and the A-chart. There is one g>0 such that
for every j, b in U_j and vertex w outside F_j(A),

    h_P(A)(b)-b dot w >= g.                           (6)

The maximizing vertices lie in F_j(A). Uniform boundedness of vertex
coordinates and all the finitely many charts gives this assertion, including
tilts that expose only a subface of F_j(A).

If x in P(A) is removed by row b in U_j, express x as a convex combination
of P(A)'s vertices. Writing lambda_out for the total weight outside F_j(A),

    g lambda_out <= h_P(A)(b)-b dot x
                  < h_P(A)(b)-1 <= delta.             (7)

For delta<g, the remaining weight is positive. Normalize its convex
combination to y in F_j(A); then dist(x,F_j(A))<=C delta, using the uniform
bound on diam P(A). Therefore **the entire union of removed caps** lies in

    union_j {x : dist(x,F_j(A)) <= C delta}.           (8)

There are finitely many faces in (8), independent of the number of rows B.
A tube of radius t around a bounded segment in R^4 has volume O(t^3),
uniformly for bounded segment length; a vertex ball has volume O(t^4).
Consequently

    Vol(P(A) minus K(A,B)) <= C1 delta^3.              (9)

The constants depend on the fixed exposure charts, not on |B|. Overlap of
caps only decreases the union volume. The same proof at A=A0 gives
Vol(H minus K(A0,B))<=C1 Delta0^3.

## 3. Two complementary upper estimates

Put S=rho(K(A,B))/rho(H)-1. Capacity monotonicity, (1), (9), and the uniform
positive volume of P(A) give, after reducing epsilon,

    S <= -kappa r+C2 delta^3.                         (10)

At A0, (5) and (9) give

    rho(K(A0,B))/rho(H)
        <= (1+a Delta0^2)^(-2) (1-C3 Delta0^3)^(-1)
        <= 1-a1 Delta0^2                              (11)

for a1>0 and all sufficiently small Delta0.

We next justify an error bound uniform in B. Smooth stability of P(A)'s
vertices gives multiplicative inclusions with factors 1+O(r) between P(A)
and H. Intersecting with any family of the same halfspaces b dot x<=1
preserves the needed scaled inclusions, because each halfspace contains zero:

    (1-C4 r) K(A0,B) subset K(A,B)
        subset (1+C4 r) K(A0,B).                     (12)

Homogeneity and monotonicity of capacity, and volume scaling, imply

    |rho(K(A,B))-rho(K(A0,B))| <= C5 r.               (13)

These ratios are uniformly bounded: the rows b stay in a bounded shell,
so all bodies contain a common ball, and all are uniformly bounded. No
Lipschitz constant depends on the number of halfspaces.

Support functions satisfy |h_P(A)(b)-h_H(b)|<=C6 r uniformly in this shell.
Hence |delta-Delta0|<=C6 r. On a fixed bounded small interval this also gives
|delta^2-Delta0^2|<=C7 r. Combining with (11)--(13),

    S <= L r-a1 delta^2                              (14)

for one fixed L>0. No differentiability of the actual capacity is asserted.

## 4. Absorption and equality

Multiply (10) by L+kappa and (14) by kappa, then add:

    (L+2 kappa) S <= -kappa^2 r-kappa a1 delta^2
                      +(L+kappa) C2 delta^3.         (15)

For sufficiently small delta, the last term is at most
(kappa a1/2) delta^2. This proves (T), taking the smaller of the two
resulting positive constants. Every estimate is uniform in the cut count
and in coupled rates r,delta->0; there is no raywise-to-uniformity step.
If S=0 then r=delta=0, so P(A)=H and no row of B cuts it. Conversely that
configuration is H and has equality.

## 5. What this proves about necessary features

For HKO, a polar face of dimension l exposes a primal face of dimension
3-l. Thus the untreated normals are exactly those approaching polar vertices
or polar edges, corresponding to primal facets or two-faces. Compact sets of polar-boundary points each exposing a primal vertex or
edge are covered by (T).

Suppose a sequence of improving polytopes tends to HKO, and each is represented
by ten core facet rows tending to A0 plus further rows. If those additional
rows, after discarding uniformly redundant interior rows, remain in one fixed
compact family of vertex/edge exposure charts, (T) is a contradiction.
Consequently an improving sequence must violate that separation: extra polar
vertices must approach the polar one-skeleton. This is a **necessary condition**,
not existence of such an improving sequence.

The accepted F≤11 proof handles one extra normal even at those harder strata.
Its cancellation certificates do not automatically supply a joint response
for two split rows. For primal two-faces, cap volume is O(delta^2), matching
the capacity exponent; for facets it is O(delta). The absorption in (15)
therefore fails exactly where a new coefficient inequality or a joint
multisplit feasible-section argument is required. Neither failure proves a
counterexample or disproves local maximality for F≥12.

## 6. An exact redundancy result for a tempting multicut panel

A simple four-polytope has four facet rows a_1,...,a_4 incident to any vertex.
Define its vertex cut normal b_v=(a_1+...+a_4)/4. Each of its four incident
edges has an associated normal b_e equal to the mean of its three incident
facet rows. Each a_i appears in exactly three of those four means, so

    b_v = (1/4) sum_(e incident to v) b_e.             (16)

For a common radial multiplier 1+t, the vertex inequality
(1+t)b_v dot x<=1 therefore follows from the four incident edge inequalities
(1+t)b_e dot x<=1. If all edge cuts are present, adding all vertex cuts does
nothing, for every t>=0. This is an exact negative result about that proposed
sampling enlargement, independent of capacity.

[verify_geometry.py](verify_geometry.py) checks all 25 identities over the
accepted packet's exact ordered algebraic field, all 25 vertex and 50 edge
exposure incidences, and 1,750 positive support gaps. Their minimum is
(5+tau^2)/16 approximately 0.3454915. These are finite geometric checks,
not a numerical neighborhood radius. The numerical 75-inequality panel in
[probe.py](probe.py) consequently has the same body as the edge-only panel;
its 25 extra vertex inequalities are redundant. It does not supply a distinct
75-new-facet family.
