# Fixed regular pentagon, arbitrary convex partner

2026-09-18. Scientific owner: empirical desk subagent `pentagon_partner_research`,
under the empirical desk root. Jörn selected this class as a population in which
to discover geometric patterns; extremal search is one possible use, not the
mandate. The first research window has now executed a heterogeneous collection and
adaptive interpretation; start with [its results](first-window.md) and the
[independently checked symmetric-partner lemma](symmetric-partner-lemma.md).
The charter and kickoff proposal below preserve the initial reasoning; their
future-tense collection plan is superseded by the executed first-window record. No thesis/formal files are owned here.

Follow-up: [source/novelty boundary](source-check.md) and
[equal-fit harmonic families with different geometry](harmonic-fibers.md).
The latter resolves a representation ambiguity using seven controlled bodies,
including the cross-line moment counterexample as a separate radial control.

## What is worth investigating first

**Build a diverse collection of partners and examine how directional shape,
asymmetry, and finite triangle-fitting geometry constrain one another.**
The fixed factor makes capacities cheap and interpretable while leaving the
partner's side count and geometry unrestricted. Keep capacities, fitting data,
and capacity-free geometry as separate views of the same raw bodies.

The first collection should let us encounter unexpected relations, while
supporting two concrete questions:

1. Which geometric changes are invisible to the complete fitting vector, and
   how much can area, directional widths, or continuous moments change within
   such equivalence classes? The vector determines capacity, but it need not
   determine the geometry. This could lead to geometric reductions for extremal
   problems, or useful examples separating proposed sufficient descriptions.
2. Along fixed-width asymmetry paths, which parts of the directional shape
   control changes in the fitting vector? Is a proposed effect already forced
   by concavity/symmetry, or does it separate bodies with the same width data?
   A sharp geometric criterion or a new inequality between the response and
   asymmetry would be more interesting than a fitted scalar correlation.

These are initial questions, not compulsory outcomes. A target-free relation
among widths, area, polar geometry, or moments can earn investigation on its own.
No claim that either question is novel or likely to produce a theorem has been
established. Do not recreate historical branch-count/incidence classifiers or
interpret fit contacts as billiard-orbit labels.

A consequential endpoint is the minimum-area covering problem for this fixed
pentagon. The HKO partner is conjecturally optimal among arbitrary convex
partners. The new independent-affine-pentagon theorem settles a proper subclass,
not this problem. We need not solve a comprehensive model of all fitting vectors
before pursuing an area inequality suggested by examples: attack that inequality
directly if it becomes the useful consequence.

## Evaluator contract

Source: [Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1),
Definition 3.2, Theorem 3.3, Problem 3.4 and §6.1. Their criterion says that
capacity is at least one exactly when every normal unit-perimeter triangle
translates into K. For a regular pentagon there are ten such translation classes.
The minimal-area cover is explicitly unresolved in §6.1.

Conventions: Q is the regular pentagon of circumradius one, with vertices at
angles pi/5 + 2*pi*i/5 and outward unit facet normals n_i at 2*pi*i/5. Its facet
height is h = cos(pi/5). Both factors have the same Euclidean coordinate axes;
the product is Lagrangian. The Q-length of a vector is its support value h_Q(v),
not its gauge in Q. Factor exchange is allowed by the capacity symmetry; the
implementation evaluates K as table and Q as fixed norm factor.

Template construction: enumerate triples of normals having strictly positive
weights w_i with sum w_i n_i=0 and sum w_i=1. The three edges w_i n_i/h close
and have total Q-length one. Their two cyclic order types give two translated
triangle classes. Five positive triples yield ten templates. Q has no opposite
normal pair, so there are no segment templates. `templates()` records both the
normal IDs and edge order, avoiding an implicit orientation convention.

For K={x:A_i x <= b_i}, set s_ij=max_{v in T_j} A_i v. Then

```
r_j(K) = max lambda
         subject to A_i t + lambda*s_ij <= b_i for every facet i,
                    t in R^2, lambda >= 0.
c(K × Q) = min_j r_j(K).
```

Derivation: translation containment is exactly the displayed inequalities.
The largest common scale of all templates fitting independently is min r_j.
Applying the source criterion to K/lambda and using degree-one scaling in one
factor identifies that scale with capacity. The two-factor ratio is
c² / (2 area(K) area(Q)); area(Q)=5 sin(2*pi/5)/2.

At positive optimal scale the dual is

```
r_j(K) = min b·y,  y >= 0,  A^T y = 0,  s_j·y = 1.
```

These are only 10 LPs with three primal variables. The implementation stores
all primal placements, slacks, contact facets, dual weights, primal/dual residuals,
and gaps. This is floating-point evaluation, not rigorous certification. A
scientifically consequential candidate needs exact reconstruction or interval
bounds and a completeness check. Tiny or ill-conditioned bodies need rescaling
and tolerance analysis; the present calibration covers ordinary well-scaled
inputs. Invalid/degenerate hulls and failed solves raise errors rather than
silently producing a capacity. A future batch runner must retain failures.

## Deductions that should not be mistaken for empirical discoveries

The following follow directly from the fitting representation; novelty is not
claimed.

* Translations preserve every r_j and dilation multiplies each by the scale.
  Reflecting K through zero permutes the ten entries, as does rotation through
  pi/5. In particular capacity has that rotational period for arbitrary K.
* Each r_j is concave under Minkowski interpolation: interpolate two feasible
  placements and their scales. Capacity, as the minimum of these concave
  functions, is concave too. For a fixed set of facet normals, LP duality makes
  each fitting scale and capacity concave piecewise-linear functions of the
  offsets on the valid bounded-body domain. Ties and degeneracies need explicit
  handling; one solver's chosen optimal basis is not a canonical stratum label.
* Along K_t=(1-t)K+t(-K), the difference body K_t-K_t is fixed. Every directional
  width is therefore fixed. Capacity is concave and symmetric about t=1/2, so
  its nondecrease toward that midpoint is forced, not a discovery. Pairwise
  template asymmetry vanishes for centrally symmetric K; the converse has not
  been established and must not be assumed.
* Choose an optimal placement of r_j(K) T_j inside K for each j and let L be
  their convex hull. Then L is contained in K and has at most 30 vertices.
  Containment gives r_j(L)<=r_j(K); its included copy of r_j(K)T_j gives the
  reverse inequality. Thus L preserves the ENTIRE vector and capacity. Its
  area cannot increase. This is an elementary per-body reduction, not a new
  minimality theorem. The chosen hull is generally nonunique; differences
  between solver choices are not geometric invariants. Our irregular-pentagon
  calibration happened to return L=K in area and vertex count, so it is not
  evidence of useful compression in a population.

That last deduction makes fitting-vector collisions unavoidable in principle
once suitable outer modifications are allowed, but says nothing about their
quantitative geometric freedom. The interesting issue is the structure of that
freedom or a sharp lower-area realization, not merely finding a collision.

## Proposed first collection, after root review

Use a small collection of **base bodies**, followed by grouped controlled paths;
transformed relatives are not independent samples. Start with 24 independently
specified partners: 8 hulls of planar point clouds (disk and Gaussian source
laws), 8 bounded half-plane intersections with irregular angles/offsets, 4
centrally symmetric zonotopes, and 4 polygons approximating smooth asymmetric
support functions. Record input point count separately from actual side count.
Point clouds and half-plane sampling favor different geometry; neither is a
uniform law on polygons. Reject or retain generation failures explicitly.

Add four translated-template hulls as a different, deliberately engineered
population, without optimizing them. Include exact-family controls from the
calibration separately. Side counts should span few-sided and finely resolved
shapes; a high-side-count approximant is not merely another regular polygon.

Select four visibly different bases without looking at sys. On each compare a
few points along: fixed-width symmetrization; a single-corner truncation; and
relative rotation. A fourth path, Minkowski addition of a segment, provides an
anisotropic width change. Do not multiply every body by every path. Preserve
lineage, raw unnormalized coordinates, generator parameters and random seeds.
Analysis may use unit-area representatives but must retain the scale map.

Measurements to implement before that collection:

| View | Initial information | Interpretation |
|---|---|---|
| Directional geometry | Sampled support/width functions with angles, odd support after an explicit centroid convention, facet angles and lengths | Euclidean and representative-dependent; retain alignment with Q |
| Integral geometry | Area, perimeter, continuous uniform-area covariance and third moments | Distinct from extreme-vertex covariance; continuous moments need polygon integration |
| Dual geometry | Polar area after centroid translation, area product | Explicitly center-dependent; do not call this the Santaló product |
| Symplectic/fitting | Capacity, all ten r_j, dual/contact geometry, area and sys separately | Ten fits determine c but contact labels are not orbit labels |
| Construction response | Finite changes along the controlled paths, template placement hulls | Noncanonical hull choices tracked, not silently identified with shape invariants |

Initially inspect aligned directional fields and contrasting shapes alongside
small feature-to-feature tables. Remove scaling identities and the forced laws
above before selecting surprises. Do not choose all follow-up bodies by a high
systolic ratio. A simple geometric separation can be more informative than a
large correlation. Use selected new geometries to distinguish proposed meanings;
freeze a statement before calling later samples a validation.

## Checkpoint and next action

[Implementation](evaluator.py) and [retained calibration](calibration.json) are
ready. Fifteen calibration/control bodies took 0.222 seconds inside the final
run; first 13-body run took 0.198 seconds. Typical 10-LP calls took about 14–17 ms
here. These are local small-polygon timings, not a promise for arbitrary size.
Maximum analytic/metamorphic/duality discrepancy was 1.78e-15. Checks cover four
regular-pentagon rotations, a shear/stretch, orientation reversal, translations,
dilation, sign reversal, pi/5 rotation, triangle/quadrilateral theorem controls,
an independent rectangle extent calculation, and inner-hull vector preservation.
The affine checks use the separately audited formula in
`formal/pentagon-affine-products/README.md` in the pentagon-affine-integration
worktree; no duplicate proof audit was performed.

**Recommended next work:** root review of the evaluator and this scientific
scope, then implement the genuinely different geometric measurements and create
the small collection above. Expect implementation/interpretation, not capacity
runtime, to dominate; allow a 1–2 hour first discovery session, returning sooner
if a concrete scientific judgment is needed. This is an estimate, not measured
end-to-end cost or an authorization request. Do not start that larger session
under this checkpoint mandate.

Stop/branch rules: evaluator contradiction stops acquisition for repair; an
apparent relation forced by the deductions above is recorded and retired;
a relation confined to one generator asks for a different construction, not a
more complex model; a concrete inequality or separating family switches work
toward derivation/simplification. If the collection yields only smooth scalar
trends with no defensible mathematical question, show the unresolved geometry
and ask which contrast interests Jörn before expanding it. A request for a
full extremal optimizer requires a separate reason beyond inexpensive LPs.

No additional kickoff judgment from Jörn is needed: he already chose the broad
class. Do not make him approve routine generator counts or rediscover evaluator
facts. Bring him a concrete pattern/conjecture and our assessment when his
mathematical interest can change the next substantial work.

## Reproduce

```
uv run --with numpy==2.5.3 --with scipy==1.18.1 python \
  docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/evaluator.py
```

This overwrites only the owned calibration JSON. Original execution used
`uv run --with numpy --with scipy` and the recorded versions were 2.5.3/1.18.1.
Source checked online on 2026-09-18. Calibration output includes raw vertices,
templates and library versions. No historical table, cache geometry or secret
contents were read; no general capacity producer ran. Root will make the
coherent commit with thread provenance after review.
