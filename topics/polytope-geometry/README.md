# Polytope geometry

This topic owns the ordinary four-dimensional convex geometry supporting
capacity algorithms: normalized halfspaces, polar bodies, exact vertex
construction and indexed incidence, integer determinant predicates, and
Euclidean volume. Symplectic signs, Reeb dynamics, optimization and orbit
search belong to their own consumers.

## Mathematical account

An ordered row list represents `K = {x : a_i · x <= 1}`. Its normalization
places the origin strictly inside whenever the intersection is bounded and
full-dimensional; row identity remains significant even when rows coincide.
[Exact polar geometry](exact-polar-geometry.tex) owns the reusable definitions
and proofs, with the original theorem labels and review wrappers:

- `prop:boundedness-iff` and `lem:positive-span` connect the recession cone,
  boundedness and positive span. `lem:bounded-triples` requires rank four and
  tests linearly independent triples; a dependent triple does not determine a
  unique normal line.
- `lem:vertex-enumeration` enumerates intersections of four independent active
  constraints. A nonsimple vertex can have more than four tight listed rows.
- `lem:irredundancy` distinguishes supporting a facet (incident affine rank
  three) from individual irredundancy. The latter also requires that the
  listed row is not repeated identically.
- `prop:integer-cramer` and the rational-scaling lemmas express candidates and
  feasibility gaps through integer determinants after positive common-denominator
  scaling. Implementations choose an LCM; the identities permit any positive
  common denominator.

These statements retain their local `unverified` markers. The boundedness
endpoint and unique-row qualifications were corrected in the independently
agent-reviewed 11 October pilot; that review is not Jörn's mathematical
approval. The [foundations report](../../meta-topics/repository-organization/inventory/pilots/2026-10-11/foundations.md)
records the exact changes and remaining review boundary. Random sampling
policy stays with [the sampling proof source](../../formal/random-polytope-boundedness.tex),
and [floating orientation filters](../../formal/f64-orientation-sign-filters.tex)
are a separate numeric proof packet.

## Implementations and their different contracts

The reusable API and operation-level preconditions are maintained in
[euclidean-polytopes](../../crates/euclidean-polytopes/README.md).
[The source](../../crates/euclidean-polytopes/src/lib.rs) remains a conventional
Cargo package at its current path; this topic account owns scientific routing.

| Route | What it provides | Validation boundary |
| --- | --- | --- |
| Euclidean generic exact normalized-halfspace kernel | finite vertices and exact input-row incidence | raw enumeration permits duplicate, redundant and unbounded inputs, including no vertices |
| Euclidean checked polar wrapper | vertices of the normalized polar | checks origin interior; invalid contract panics |
| Symplectic generic exact constructor | shared exact enumeration with local result/error type | checks facet count, zero rows, legacy cone boundedness, nonempty enumeration, then incident affine rank three |
| Symplectic optimized rational constructor | integer-scaled enumeration with floating prefilters and rational results | separate rank-four, boundedness and facet policy; not a wrapper around the generic exact pipeline |
| Symplectic f64-to-rational input adapter | conversion of input rows | rejects nonfinite, tiny-norm and near-duplicate raw rows before construction |
| Incidence-only combinatorics | indexed lists, facet intersections and face candidates | shape/combinatorial inputs, without certifying coordinates, geometric dimension or polygon order |

The [generic constructor](../../crates/symplectic/src/exact/polytope.rs) and
[optimized rational implementation](../../crates/symplectic/src/geom/vertex_enumeration/mod.rs)
share vertex/facet identities where their contracts agree. They do not have a
uniform validation or error contract. Identical rows can pass either exact
facet-rank validator, while the f64 input adapter rejects near-equal rows.
The legacy generic boundedness scan omits triples containing row zero and has
no separate full-rank check; the rational route explicitly rejects lower rank.
The recorded rank-two input gives `NoVertices` versus `Unbounded`. A proof of
the complete triple criterion is not a proof of every implementation shortcut.

The 11 October consolidation reused the Euclidean generic vertex kernel and
incidence-only facet-intersection scan through compatibility adapters. The
[geometry report](../../meta-topics/repository-organization/inventory/pilots/2026-10-11/geometry.md) records
ordered simplex/cube/algebraic-HKO fixtures, diagnostics and focused checks.
Agreement of callers sharing a kernel is behavior evidence, not independent
confirmation of the kernel.

The shared `determinant_4d_integer` now owns the identical integer cofactor
calculation formerly repeated in the Euclidean rational polar and symplectic
rational enumeration paths. It accepts four ordered integer rows and returns
the signed exact determinant, including zero for singular input. Its ownership
move does not unify floating filters, boundedness tests, error policies or
full enumeration algorithms. Further elementary dot/cross/scaling duplication
remains; it is not a reason to merge policy-bearing constructors.

Known-incidence volume helpers use reliable vertex/facet incidence and coned
simplices. Exact volume uses exact determinant arithmetic; floating volume
checks finite coordinates but has caller-owned incidence and decomposition
assumptions. Incidence-count edge and two-face candidates alone do not certify
polygon cyclic order; the volume helpers perform the ordering they need.

## Consumers and bounded reproduction

Capacity/QP, [flow graphs](../flow-graphs/README.md), known fixtures and
experiment-local caches consume these geometric primitives. Geometry does not
supply a capacity theorem or validate a producer's retained data population.
`capacity_4d::PolytopeGeometry4d` exposes public mutable fields: guarantees from
a checked constructor apply to that constructor's output, not arbitrary
struct literals or later mutations. Downstream code must preserve the matrix,
row-identity and geometry relationships it relies on.
The crate [API account](../../crates/euclidean-polytopes/README.md) is the
canonical operation contract; this file explains how its mathematical sources
and policy-bearing consumers fit together.

From the repository root, focused checks for the shared determinant and its
Cramer consumers are:

```bash
cargo test -p euclidean-polytopes --test integer_determinant
cargo test -p euclidean-polytopes polar::tests::integer_scaled
cargo test -p symplectic geom::vertex_enumeration::tests::integer_cramer_exact_coordinates_simplex
```

The first compares the cofactor kernel against the permutation definition and
checks orientation, singularity and integers beyond fixed-width arithmetic.
The latter checks exact candidate/feasibility computations. These are focused
regressions, not a rerun of retained experiment tables or certificates.

## Source disposition

`formal/exact-polar-geometry.tex` is a compatibility input for the canonical
source here; labels occur only in this topic's proof packet. The Rust package
and symplectic adapters remain in place to preserve Cargo and public API
consumers. Full constructor-policy unification, proofs of legacy shortcut
coverage, and new approximate APIs are deferred scientific/API work, reopened
when their contracts and callers are selected explicitly.
