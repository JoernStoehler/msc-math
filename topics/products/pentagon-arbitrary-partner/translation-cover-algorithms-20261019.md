# Computing a minimum-area cover of the ten pentagon triangles

19 October 2026. Bounded primary-source and repository reconnaissance prompted
by Jörn's question about directly minimizing the cover. This note distinguishes
an existing exact restricted solution, a published unrestricted algorithm, and
the bounded exploratory computation now assigned to `partner_geometry`.
It does not report an unrestricted optimum or a new theorem accepted by Jörn.

The answer to “is there a known algorithm?” is **yes**. The practical question
is which parts we can implement and verify within the present research window.
Restricting all hull edges to template-side directions is also more than a
speculative starting point here: that exact restricted problem was already
solved in a retained project packet.

## The unrestricted problem is already a finite optimization

Let `T_0,…,T_9` be the complete ten unit-Q-length normal triangles of the fixed
regular pentagon. Each has a fixed orientation and its own translation. The
[BMP criterion, Theorem 3.3 and §6.1](https://arxiv.org/html/2603.12495v2#S6.SS1)
identifies capacity-at-least-one partners with translation covers of these
triangles. Replace any cover by the convex hull of one placed copy of each
triangle. This preserves feasibility and cannot increase area. Thus the exact
objective is

```
F(t)=area(conv(∪_j(T_j+t_j))),     t_0=0.
```

There are 18 relative translation variables and at most 30 hull vertices.
No triangle-disjointness condition applies. Once placements are chosen,
feasibility as a capacity-one cover is automatic; capacity LPs are unnecessary
for evaluating this area objective.

A compact search domain follows directly, and is also supplied in
`mathematics/RESIDUAL_AND_OBSTRUCTIONS.md` in the archive identified below.
Anchor a nondegenerate template `T_0`, with edges `[v_i,v_(i+1)]` and vectors
`e_i`. Any hull point x in an area sublevel `F≤m_H` obeys

```
|det(e_i,x−v_i)|≤2m_H.
```

Each edge and x span a triangle inside the hull. These strip pairs bound two
independent linear functionals, hence bound all placed vertices and translations.
This is a domain containing every competitive placement, not an assertion that
every point in the domain is competitive. Exact hull-order descriptions and
area quadratics give a semialgebraic global decision problem. General real
algebraic decision procedures therefore exist, with no useful runtime inferred.

## A published exact algorithm applies to the actual problem

[Jung–Ahn, *Smallest Convex Hulls of Polygons*, ESA 2026](https://drops.dagstuhl.de/storage/00lipics/lipics-vol388-esa2026/LIPIcs.ESA.2026.44/LIPIcs.ESA.2026.44.pdf),
§3, Theorem 3, gives an exact translation algorithm for fixed k convex polygons
with n total vertices. Its bound is `O(n^η)`, `η=max{5,4k−8+δ}` for fixed `δ>0`.
Our instance has k=10, n=30, hence exponent `32+δ`. Overlap is allowed.
Their §2 real-RAM model treats solving fixed-degree polynomial systems in a
fixed number of variables as constant cost. Thus this is an applicable exact
algorithm, not a ready implementation or a measured cost estimate. It decomposes
translation space at hull-contact events, including three-polygon common
supporting lines. Their approximation bound has ε exponent −9.5 at k=10.

The paper also supplies a useful inner primitive: §5.1, Lemma 6 says that if all
movable polygons independently slide along one fixed common direction, area
is convex piecewise linear in their scalar shifts. This does not make the full
18-dimensional problem convex. No implementation accompanying that paper was
located in this bounded inspection; no exhaustive software search is claimed.

Finding one exact optimal placement would establish the minimum value. A full
classification of all minimizing bodies still needs an equality-set argument.
The BMP v2 discussion conjectures the HKO cover optimal and does not prove that
claim for arbitrary partners.

## The triangle-side-direction restricted problem is already solved

The source is the retained
[September consolidation ZIP](../../../docs/history/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip),
inside `incoming/hko_fixed_pentagon_results.zip`, whose root directory is
`hko_fixed_pentagon_results/`. Relevant internal files are:

```
mathematics/TEN_DIRECTION_PROOF.md
mathematics/RESIDUAL_AND_OBSTRUCTIONS.md
code/exact_model.py
code/discover_fan.py
code/make_fan_certificate.py
code/verify_fan.py
code/verify_body_witnesses.py
data/fan_certificate.json
evidence/final_verified_run/fan_exact_summary.json
```

The current [baseline comparison](../../empirical-studies/discovery-design/night-20261005/comparison/fan-baseline-check.md)
already records this result. The proof and producer sources were directly read
again for the present note; the enumeration was not rerun.

In the current fixed-Q convention the permitted outward normals are
`18°+36°k`, exactly those perpendicular to template-side directions. This class
allows arbitrary supports and vanished edges, including more than five sides.
The exact optimum is the capacity-one HKO area `m_H`; equality bodies are its
two signs `H` and `−H`, up to translation. Restoring scale gives the sharp ratio
`(3+sqrt(5))/5` within this fan. No reduction of arbitrary covers to this fan
is supplied.

The algorithm is concrete. Use ten nonnegative edge coefficients and two linear
closure equations. Farkas elimination turns the ten translation-fit problems
into 25 exact linear covering inequalities. Homogenization and a positive sum
section produce an eight-dimensional compact polytope. The retained certificate
contains 55 symmetry representatives, expanding to 797 vertices: 772 finite
ones and 25 recession directions. The exact verifier checks feasibility, vertex
rank and closure of the list under every incident edge; connectedness of the
polytope graph proves completeness. Square-root-area concavity under common-fan
Minkowski combinations transfers the vertex minimum to the full feasible set.
Zero-edge cases, recession directions and equality need these explicit steps.

`discover_fan.py` uses SciPy halfspace intersection only to propose vertices;
`make_fan_certificate.py` reconstructs exact representatives;
`verify_fan.py` supplies exact completeness. None is an unrestricted placement
optimizer. The retained exact summary records about 22 seconds for its verifier
in its historical environment; this is not a timing promise for the current
machine. It requires SymPy. Default Python in this checkout currently finds
NumPy but neither SymPy nor SciPy; no dependencies were installed for this note.

## Cheapest useful actions and the remaining proof gap

A fresh run of `verify_fan.py` against the retained certificate would check that
restricted baseline locally; regenerating its whole enumeration adds little
scientific value unless a concrete correspondence issue appears. Source reading
already makes the restricted optimum available as an inherited result, with its
agent-review boundary rather than new human acceptance.

The useful new reversible test is to let all 18 translations move and look for
a hull below `m_H`. The current `partner_geometry` owner has a separately bounded
probe: at most 12 deterministic starts, at most 300 area evaluations per start,
and at most 60 seconds process CPU. Candidate starts should include remote
placements, because [the existing local-rigidity note](local-cover-rigidity.md)
already supplies a separate local argument near the HKO placement. Its continuum
proof and review boundary should not be mistaken for a global certificate.

A practical inner step is common-direction cutting-plane optimization. Hold
base placements a_j fixed, choose a unit direction d, and put
`t_j=a_j+x_j d`, `x_0=0`. The hull's area is convex piecewise linear in x.
For a counterclockwise hull cycle p_i, a supporting affine coefficient is

```
g_j = (1/2) Σ_{i: vertex i belongs to template j}
                    det(p_(i−1)−p_(i+1),d).
```

Its cut is `F(x0)+g·(x−x0)≤F(x)`. A small LP over accumulated cuts can minimize
that direction's subproblem; changing d then gives another reversible step.
Alternating these subproblems is a heuristic for the full nonconvex objective,
not a global algorithm. Floating output requires exact reconstruction before
claiming a strict improved cover.

There is an independent reason these cuts remain valid at hull ties. Rotate
coordinates so d is horizontal. Each vertex's vertical coordinate stays fixed.
A current hull cycle splits into two vertical-monotone chains. After any of the
allowed shifts, both chains remain inside the new hull. Integrating their
horizontal separation gives a signed area no greater than the new hull area,
even if the chains cross. That signed area is affine in x and equals F at the
current placement. This proves the supporting-cut claim without assuming that
a generic 18-dimensional hull-order cell is convex. Numerical roundoff remains
a separate implementation issue.

Any hull below `m_H` would be a strict fixed-factor improvement once its input
and area comparison are certified, because it already contains all templates.
Failure to find one is only bounded search evidence. Closing the global theorem
requires a complete lower bound across the coupled contact strata or a verified
implementation of an applicable exhaustive algorithm. Restricting edge directions,
local minimum tests and successful optimization starts do not supply that step.

## Subsequent bounded rehearsal

The [matched probe packet](translation-cover-probe-20261019/README.md) records twelve deterministic unrestricted placement starts, no below-HKO candidate, cut-cap failures in the initial schedule, and numerical boxed-inner-gap closure in three refinements. This instantiates the parallel-slide primitive; it neither implements the exact global algorithm nor proves a minimum. No larger campaign is selected.
