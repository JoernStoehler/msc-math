# Complete joint denominator extraction

Producer completed 2026-10-05 00:31 UTC under the
[selected joint-denominator contract](../../comparison/orientation-joint-contract.md).
Owned output is [extract.py](extract.py) and
[joint-denominators.json](joint-denominators.json). Independent acceptance is
owned by the lead/reviewer; this report records production, not its review.
No body feasibility, area minimization or HKO inequality is asserted here.

## Inputs and conventions

The producer reads the accepted
[preflight catalogue](../preflight/active-events.json), whose SHA-256 is retained
in the output. It does not rerun the preflight producer. The catalogue contains
all 1,350 original support choices, 121 distinct bilinear candidates per circuit
and the independently retained dominance witnesses. The survivor counts are
`[5,6,8,7,6]`. Equal polynomial candidates retain all original source labels.

With `F=Q(√5)`, `τ²=5−2√5` and `0≤u≤1/√5`, the scaled physical normals are

`G_x C_u N_i`, `C_u=[[1,−u],[τ²u,1]]`, `x>0`.

Candidate coefficient order is `(1,x,u,xu)`. These are covectors; the point
conjugate is not substituted. The common positive `cosθ` normal scale is
absorbed in supports, and the determinant is `1+τ²u²`. Physical area and body
optimization belong to a later stage. The complete fixed-Q template catalogue
has all ten original triangles and the same five closure circuits.

## Actual counts and cost

| Quantity | Actual result |
| --- | ---: |
| Individual event polynomials, all five circuits | 319 |
| Individual event orientations by circuit, endpoints included | `[2,4,14,8,5]` |
| Individual open orientation patches by circuit | `[1,3,13,7,4]` |
| Merged individual event orientations | 18 |
| Merged individual topology strips | 17 |
| Actual active x-break curves per topology strip | 14–18 |
| Cross-circuit active-curve comparisons processed | 1,659 |
| Persistent-equality comparison occurrences | 40 |
| Additional distinct cross-order orientations | 1 |
| Final joint event orientations / closed fibers | 19 |
| Final open orientation strips | 18 |
| Joint open x/u cells | 268 |
| Joint cells on closed orientation fibers | 258 |
| Closed fibers requiring a quadratic extension of F | 12 |

Individual extraction took **8.89 seconds** in its initial complete run.
The complete joint run took **165.047 seconds**, including all raw closed
fibers. The retained JSON is approximately **17.7 MB**. These are measured
costs, not forecasts for subsequent positivity work.

The sole extra cross-order root has monic polynomial over F

`u²−(9/11+19√5/55)u+(2/11+3√5/55)=0`.

Its lower-root embedding is isolated in
`(63√5/640, √5/10)` in the certificate. This is an actual order event between
active break curves; it was not inferred from the individual hull counts.

## Exact root adapter and resolved first stop

The original F-only engine stopped at circuit 1's candidate triple
`[61,76,119]`. The initial evidence is preserved in
[first-field-stop.json](first-field-stop.json). Its event discriminant is

`Δ=(18859−8434√5)/125`, with `Norm_F/Q(Δ)=101/15625`.

The rational norm is not a rational square, so Δ cannot be a square in F.
The event is therefore a genuine quadratic extension requirement, rather
than a numerical ordering failure. Its real root is isolated by exact
sign-changing rational endpoints and a strictly negative derivative. The
corresponding norm polynomial is

`25u⁴+20u³−26u²−4u+1`.

The bounded adapter resolves this requirement using one field at a time:

- An `EventRoot` retains a monic irreducible quadratic over F, a root branch
  and an exact sign-changing isolating interval. Its interval can be refined
  by exact bisection.
- On a closed fiber, every value has form `p+qα`, `p,q∈F`. Multiplication
  reduces by `α²+bα+c=0`; division uses the quadratic conjugate and norm.
- A nonzero linear value is signed by comparing α to the F-threshold `−p/q`.
  Irreducibility excludes equality with that threshold. Interval refinement
  terminates when the threshold is outside the isolator; zero values reduce
  exactly to zero coefficients.
- Distinct irreducible quadratic events are ordered by refining separate
  isolators until disjoint. Equal events have the same monic polynomial and
  branch. Different irreducible monic quadratics cannot share a root over F.
- Open-strip samples are chosen in F between the isolators, never by adding
  roots from different fields. Closed-fiber arithmetic remains in F(α).

Thus no global compositum or approximate sign oracle is used. Roots already
in F use the exact quadratic-field arithmetic directly. Outside-strip roots
are discarded only through exact endpoint/extremum signs or exact ordering.
Repeated roots, endpoint roots and identically-zero event polynomials have
explicit branches in the source. Persistent equalities are retained.

[adapter-tests.json](adapter-tests.json) records the retained tests and source
hash: first-root polynomial reduction, inverse, positive real embedding,
exact rational threshold comparisons, shared-root equality/hash, ordering
between two different quadratic fields, and field multiplication/division.
The same `test_adapter()` is called by future producer reruns.

## Coverage argument

Each individual envelope uses all survivor pair-slope and pair-intercept
polynomials and every survivor triple-concurrency polynomial. Hull changes
can occur only at a slope collision, a crossing through `x=0`, or a collision
of consecutive intersections. These account respectively for parallel/infinite
tail changes, clipping changes and triple-concurrency changes. Exact root
ordering splits the orientation strip at every such event. The raw hull at
one exact F-sample consequently determines the active sequence throughout
each event-free open strip. The accepted dominance witnesses exclude every
removed candidate from becoming an interior winner.

The producer merges all individual events before comparing curves. On each
resulting strip it extracts **only actual active** adjacent-line intersections,
whose x-coordinates are ratios of affine F-functions of u. Each denominator
is nonnegative at both strip endpoints and strictly positive at its sample;
affine interpolation proves strict positivity throughout the open strip.
Poles at excluded strip endpoints are handled by the closed fiber directly.

For every cross-circuit pair of these active curves, their equality numerator
has degree at most two. Every in-strip root is retained and ordered; an
identically-zero numerator records a persistent shared curve. The final
subdivision therefore has a fixed ordered union of all active x-breaks.
Its x-cells retain the attaining candidate IDs for all five denominators;
their bilinear coefficients and all template/vertex source choices resolve
directly in `circuits[*].candidates`. Shared curves are grouped rather than
duplicated into artificial cells.

At every final orientation event, the entire x-fiber is independently
recomputed from all **121 original distinct candidates per circuit**, including
their full 270 source choices. It is not copied from an adjacent open strip.
The certificate retains all five raw hulls, their ordered union, every joint
fiber cell, and raw D-vector values/ties at every union breakpoint and `x=0`.
Isolated or zero-width winners occur in those point vectors. Parallel
coincidences retain all whole-cell IDs. Infinite-tail candidates and all
maximal-slope candidates are distinguished explicitly.

Adjacent formulas agree along an individual break by the defining exact
intersection equation. Persistent shared curves are identified by a zero
equality polynomial; at isolated cross-order events, the raw fiber supplies
the simultaneous incidences. Values and all raw ties at sample boundary points
are retained for independent checking. The reviewer must verify these coverage
and agreement implications rather than accepting cell counts alone.

## Reproduction and write effects

```bash
python -B /workspaces/msc-math/docs/empirical-viterbo-design/night-20261005/orientation-wide/joint-denominators/extract.py
```

Python standard library only. The producer reads the accepted preflight JSON
and writes **only** its sibling `joint-denominators.json`; timings vary.
Field elements encode `["a","b"]=a+b√5`. Extended event roots retain their
monic F-polynomial, branch and isolator. Extended fiber values retain their
event-root embedding and two F-coefficients. `right:null` denotes the final
unbounded x-tail.

The separate initial-stop and adapter-test evidence files were retained by
bounded explicit evidence commands; the main producer does not write them.
No old producer, body enumeration, CAD, paid API or bivariate area-positivity
programme was executed. Remaining work in this allocation is independent
consequential review and lead integration; later positivity requires another
root selection.
