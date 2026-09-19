# Ridge concentration is constrained by circulation

2026-09-18. Mathematical argument independently checked by a bounded read-only
agent; not yet reviewed by Jörn. No literature novelty claim. Historical data
suggested the question and supply checks, not its proof. This result concerns
geometry and descriptor interpretation, not capacity or the systolic ratio.

For every genuine two-dimensional ridge E of a bounded full-dimensional convex
four-polytope K, put a_E = |integral_E omega|, S = sum a_E, and T = sum a_E^2.
These are the unsigned polygon symplectic areas used by the repository. A
common normalization by sqrt(volume) cancels from every ratio below.

**Result.** S and T are positive. The inverse-Simpson effective ridge count
N2 = S^2/T satisfies N2 > 3. Every ridge has a_E/S <= 1/3. For a Lagrangian
product of two full-dimensional convex polygons, or for any centrally symmetric
convex four-polytope, N2 > 4 and a_E/S <= 1/4.
The constants 3 and 4 are sharp infima; the strict effective-count bounds
cannot be attained by a full-dimensional body in the corresponding class.

The repository's `ridge_symp_area_effective_face_count` is instead the Shannon
count N1 = exp(-sum p_E log p_E), where p_E=a_E/S. Since N1 >= N2, the same
strict lower bounds and sharp infima apply to N1. Do not identify these two
features or copy the N2 formulas below as formulas for N1.

## Why the inequalities hold

Orient the boundary three-facets coherently. Every ridge lies in exactly two
facets and receives opposite induced boundary orientations. Stokes' theorem,
applied to the constant closed two-form omega on each facet, says that its
signed ridge integrals sum to zero.

The facet-adjacency graph has one node per three-facet and one edge per ridge.
It is simple: two distinct convex facets cannot share two separate ridges.
The signed integrals therefore form a circulation. Delete zero-flux edges and
direct each remaining edge according to the sign of its flux. Its positive
weights a_E form a nonnegative circulation, hence decompose into directed
simple cycles: a = sum_C t_C 1_C with t_C > 0.

If g is the girth of the nonzero support graph, every such cycle has length at
least g. For any edge e,

    S = sum_C |C| t_C >= g sum_{C containing e} t_C = g a_e.

Consequently

    max a_e <= S/g,
    T <= (max a_e) S <= S^2/g,
    N2 >= g.

Directed girth can replace undirected girth for a potentially stronger bound.
The simple graph gives g >= 3. In a planar Lagrangian product, ridges lying
entirely in one factor direction have zero symplectic area. Every remaining
ridge joins a q-facet to a p-facet, so the nonzero support graph is bipartite
and g >= 4. This product improvement is not claimed after a nonsymplectic
rotation out of Lagrangian product position.

Every facet has some nonzero incident flux. Indeed, omega restricts to a
nonzero two-form on its three-dimensional tangent space, since a symplectic
four-space has no three-dimensional isotropic subspace. Using a volume form
on that three-space, the restricted two-form is the flux form of a nonzero
constant vector. If its flux vanished on every boundary face, every face
normal would be perpendicular to that vector, contradicting boundedness of
the three-dimensional facet.

Equality N2=g requires equality in both inequalities for T: precisely g
positive edges, all equally weighted, forming one directed g-cycle. But all
facet nodes must have positive incident flux. A four-polytope has at least
five facets; a product of polygons has at least six. Thus equality at 3 or 4
is impossible. In contrast, equality in the max-share inequality can involve
multiple shortest cycles sharing the maximizing edge, and is not excluded.

## The stronger constant also holds under central symmetry

This extension does not require a product structure. In any graph circulation,
every matching M carries at most half the total mass: each directed simple
cycle contains at most half its edges in M, and the same holds after summing
the cycle decomposition.

If K is centrally symmetric, a ridge e and its antipode -e have equal unsigned
symplectic areas. Their corresponding graph edges have four distinct facet
endpoints, since opposite facets cannot intersect. Thus they form a matching,
and

    2 a_e = a_e + a_{-e} <= S/2.

It follows that max a_e/S <= 1/4 and N2 >= 4. Equality for N2 would leave just
four equally weighted positive edges. Conservation gives every incident node
positive indegree and outdegree, so these four edges form a directed four-cycle
and cover only four facet nodes. They cannot cover all nodes: every facet has
nonzero incident flux and a centrally symmetric four-polytope has at least
eight facets. Hence N2 > 4, and consequently N1 > 4. The box
degeneration below proves sharpness of this infimum too.

The proof needs equality of unsigned antipodal areas, not a choice of
orientation-preserving graph involution. It also records a stronger general
constraint useful for structured representations: total area on any matching
of ridges is at most S/2.

## Explicit sharpness and numerical checks

In coordinates (q1,q2,p1,p2), consider

    conv{0,e1,e3,epsilon e2,epsilon e4}, epsilon > 0.

Its ten ridge areas are three copies of 1/2, three copies of epsilon^2/2,
and four zeros. Therefore

    N2 = 3 (1+epsilon^2)^2 / (1+epsilon^4) -> 3.

For aligned rectangle factors with side lengths 1 and epsilon, the 24 ridge
areas are four copies of 1, four copies of epsilon^2, and sixteen zeros:

    N2 = 4 (1+epsilon^2)^2 / (1+epsilon^4) -> 4.

Both computations are checked with exact rational arithmetic by
`ridge_circulation.py`. The Shannon count in either example is
n(1+t)t^(-t/(1+t)), with n=3 or 4 and t=epsilon^2, and tends to n as well.

The historical table has complete ridge ordering on all 14,336 rows. The
retained replay finds no violations. Among 4,096 generic rows, the smallest
N2 is 3.2118763117 and largest max-share is 0.3322003851. Among 10,240 product
rows, these are 4.0596706590 and 0.2500000000000001 respectively. The final
roundoff-sized excess is not a mathematical violation. Full witnesses and
input SHA256 are in `ridge-circulation-check.json`.

Reproduce from the repository root:

```sh
python docs/empirical-viterbo-design/desk/relation-discovery/ridge_circulation.py \
  --historical /home/joern/.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl \
  --out docs/empirical-viterbo-design/desk/relation-discovery/ridge-circulation-check.json
```

The script reconstructs T from the population mean and standard deviation,
checks total=count*mean, and requires zero face-ordering failures. It parses
the existing JSON rows but does not use their stored sys values. The source
implementation silently excludes ordering failures from its summaries; these
inequalities must not be tested against an incomplete face list as though it
were a complete circulation. Numerical thresholding can likewise destroy
the graph conservation law.

## What this changes

The concentration descriptors have mathematically constrained ranges, with a
stronger constraint for the original product population. That is information
a generic feature-mining algorithm would otherwise relearn from samples.
Concentration cannot be interpreted as arbitrarily many ridges collapsing
while a single ridge retains all the symplectic area. Near-minimal N2 suggests
near-concentration around a short circulation, but quantitative stability or
a capacity consequence has not been established here.

The statement also applies to new generic-body representations whenever all
ridge fluxes are available. It does not require a preferred generator,
covariance feature, active orbit, or expensive target calculation.
