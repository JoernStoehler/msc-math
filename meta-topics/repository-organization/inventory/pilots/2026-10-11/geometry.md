# Geometry early unit — 11 October 2026

## Result and boundary

G1 consolidates the three Boolean vertex–facet incidence scans into
`euclidean-polytopes::facet_intersection_is_nonempty_from_vertex_facet_incidence`.
The public `symplectic::exact::facet_intersection_is_nonempty_exact` and crate-local
`geom::vertex_enumeration::facet_intersection_is_nonempty_from_incidence` remain
compatibility adapters. The matrix remains F×F, symmetric, with a false diagonal;
an off-diagonal entry is true exactly when an input vertex lies on both indexed
facets. Input column identity and ordering are preserved, including duplicate
and empty columns. No constructor, facet-validation policy, vertex ordering,
producer or retained dataset was changed. This is an incidence-only primitive,
not a geometric proof that a Boolean matrix describes a valid 4-polytope.

The shared implementation checks each unordered facet pair once, stopping at
the first common vertex. This avoids a regression on dense or repeated rows
found in review of an initial vertex-wise scan. No speedup has been measured.

Changed files for G1:

- `crates/euclidean-polytopes/src/faces.rs`
- `crates/euclidean-polytopes/tests/faces.rs`
- `crates/symplectic/src/exact/polytope.rs`
- `crates/symplectic/src/geom/vertex_enumeration/mod.rs`
- `crates/symplectic/src/geom/vertex_enumeration/tests.rs`

## Before behavior and checks

Before editing, all three source functions independently used the same
`i != j && any vertex incident to both i and j` rule. The Euclidean face
suite passed **5/5** before edits. The existing exact simplex and HKO tests
and 28 rational vertex-enumeration tests passed during the initial build, but
that compilation overlapped the first edit; they are not claimed as a clean
before-code execution. Existing face tests established simplex all-pair
intersections and cube opposite-facet exclusions. The new test records a
hand-checkable 3×5 incidence with duplicate columns, an empty column, and
both asymmetric empty shapes (V=0,F>0 and V>0,F=0).

After the final helper revision:

| Command | Outcome |
| --- | --- |
| `cargo test -p euclidean-polytopes --test faces` | 6 passed |
| `cargo test -p symplectic exact::polytope::tests` | 2 passed |
| `cargo test -p symplectic geom::vertex_enumeration::tests::rational_hypercube_facet_intersections_preserve_opposite_pairs` | 1 passed |
| `cargo check -p exp-qp-error-evidence --bin qp-retained-exact` | passed; representative external consumer |
| `rustfmt --check --edition 2021 --config skip_children=true` on the five changed Rust files | passed |
| `cargo test -p symplectic geom::vertex_enumeration::tests::rational_and_generic_exact_cube_agree_on_vertices_and_incidence` | 1 passed; canonical exact cube vertices and indexed incidence agree |
| `cargo test -p symplectic geom::vertex_enumeration::tests::constructor_contract_diagnostics_for_small_inputs -- --nocapture` | 1 passed; table below records diagnostics |
| `cargo test -p symplectic geom::vertex_enumeration::tests` | 31 passed after diagnostic additions |

The `geom::known_polytopes` group also passed **11/11** before the final
helper revision. A fresh read-only Sol reviewer approved the revised diff:
the arbitrary Boolean-matrix formula, adapter signatures, input facet
positions, empty shapes and representative consumers agree. Review found
no remaining correctness or contract blocker; tests and compilation do not
prove mathematical correctness of all constructors or downstream claims.
An initial `cargo fmt --all -- --check` passed before another worker's
`topics/empirical-studies/polytope-invariant-table/table_builder.rs` edit; a later global
run flagged only that separately owned file. The scoped format check above
passed after all geometry edits.

## G3 first shared exact enumeration path

After G1 review, the coordinator approved one more bounded slice. The
Euclidean crate already had the generic normalized-halfspace vertex kernel in
`src/polar.rs`. It now exports
`vertices_from_normalized_halfspaces_exact`, which returns finite vertices and
exact input-row incidence for `{x : a_i·x <= 1}` without enforcing the origin
interior or irredundancy policy. It supports unbounded inputs with finite
vertices and returns an empty `0×F` incidence matrix when no vertices exist.
The existing checked polar wrapper still checks origin interior and panics on
contract failure; the assuming wrapper still panics on empty enumeration.
Their original tracing spans, assertion points and timing fields remain.

`symplectic::exact::exact_vertices_with_incidence` now reuses that existing
Euclidean kernel. Its order of checks remains: facet count, exact zero row,
boundedness, empty enumeration (`NoVertices`), then incident affine rank
(`RedundantFacet`). The symplectic error enum and `omega0` remain local.
No second Euclidean enumeration module was introduced. The optimized rational
path in `geom/vertex_enumeration` is still separate, with its own integer
scaling and f64 prefilters.

Before changing G3, the old symplectic implementation's ordered facet
descriptors were captured for the simplex (5), cube (16) and algebraic HKO
(25) in `/tmp/msc-math/geometry-g3/before-order.txt`. The post-change output
matched all three ordered lists exactly. The retained test
`exact_vertex_order_preserves_legacy_lexicographic_facets` freezes those
lists, including HKO over its algebraic field. Mathematically, coordinate
deduplication in the Euclidean kernel matches the old descriptor
deduplication for enumerated vertices: every accepted candidate uniquely
solves four independent tight rows, so two candidates have the same complete
tight-row set exactly when they have the same coordinates. Both paths scan
four-tuples in lexicographic input order and keep the first accepted vertex.

Additional focused checks after G3:

| Command | Outcome |
| --- | --- |
| `cargo test -p euclidean-polytopes --test polar_vertices` | 19 passed after latest zero/duplicate-row additions |
| `cargo test -p symplectic exact::polytope::tests` | 4 passed, including order and early error precedence |
| `cargo test -p symplectic geom::vertex_enumeration::tests::constructor_contract_diagnostics_for_small_inputs -- --nocapture` | passed; G2 observations unchanged |
| `cargo test -p symplectic geom::vertex_enumeration::tests::rational_and_generic_exact_cube_agree_on_vertices_and_incidence` | passed |
| `cargo check -p exp-qp-error-evidence --bin qp-retained-exact` | passed |

The generic Euclidean and old symplectic kernels both used
`algebraic_numbers::solve_linear_system`; the migration is an ownership and
behavior-preservation result, not independent mathematical confirmation from
two solvers. The optimized rational route remains a distinct implementation,
although it and the generic route now share the G1 facet-intersection scan.
A fresh Sol reviewer approved G3 without an introduced correctness blocker.
Subscriber-level tracing and the legacy `NoVertices`/`RedundantFacet` error
branches lack separate direct regression assertions; the latter outcomes are
recorded by the G2 diagnostic fixture without freezing disputed error policy.

## G2 contract differences found; no validation-policy migration

The retained diagnostic fixture in
`crates/symplectic/src/geom/vertex_enumeration/tests.rs` uses one eight-facet
cube and the following small modifications. `f64 input` is only the
f64-to-rational input adapter; `Ok(n)` there means n rows converted, not that
polytope construction succeeded. The duplicate and rank-two outcomes are
printed, without assertions that their present classifications are desirable.

| Input | Generic exact | Rational pipeline | f64 input adapter |
| --- | --- | --- | --- |
| Cube plus identical first row | `Ok(16 vertices)` | `Ok(16 vertices)` | `DuplicateHalfspaces(0,8)` |
| Cube plus `(1/2,0,0,0)` | `RedundantFacet(8)` | `RedundantFacet(8)` | `Ok(9 rows)` |
| Cube plus zero row | `ZeroDualVertex(8)` | `ZeroDualVertex(8)` | `ZeroDualVertex(8)` |
| Rank-two `e1,-e1,e2,-e2,e1+e2` | `NoVertices` | `Unbounded` | `Ok(5 rows)` |
| Full-rank unbounded `e1,-e1,e2,e3,e4` | `Unbounded` | `Unbounded` | `Ok(5 rows)` |

- `faces.rs` accepts any Boolean incidence matrix and never validates
  coordinates, rank, boundedness, duplicate inequalities or cyclic face order.
  Its edge/2-face helpers use incidence-count thresholds only. Consumers of
  ordered polygons need their separate geometric treatment.
- `exact::polytope` accepts ordered exact scalar rows for `a_i·x <= 1`,
  rejects fewer than five and exact zero rows, checks boundedness, then
  requires affine rank three among vertices incident to each input facet.
  It has no duplicate-row error. Equal input inequalities can share the same
  rank-three incident set and pass this criterion; the criterion does not
  establish distinct or irredundant supporting hyperplanes in that case.
- The rational pipeline has an explicit input rank-four test in
  `geom/vertex_enumeration/boundedness.rs` and returns `Unbounded` for
  lower rank. Its exact rational path likewise has no duplicate-row check.
  The f64 adapter separately rejects nonfinite components, norm below
  `1e-15`, and raw near-equal rows at relative `1e-8`; these are input
  representation policies, not an equivalence test for proportional rows.
  Error enums and conversion behavior therefore differ from the generic
  exact route.
- Generic exact `check_bounded` uses suffix triples from four-combinations,
  omitting **every** triple containing row zero, and does not explicitly test
  overall rank. The retained
  rank-two fixture verifies the resulting `NoVertices`/`Unbounded` error
  divergence. A preliminary cone argument suggests suffix triples suffice
  **conditional on overall rank four**: excluding row zero leaves either a
  rank-three set whose kernel line has one feasible orientation, or a
  rank-four pointed recession cone. In the latter case, any feasible nonzero
  direction implies an extreme ray of that cone also satisfies row zero;
  three independent remaining rows support that ray. This is reasoning to
  verify in the proof source, not a promoted theorem. A seeded exact-integer
  scratch Python search (seed `20261011`, integer coordinates sampled uniformly
  from `[-2,2]`, 200,000 draws each at F=5,6,7,8, skipping zero rows and
  rank-deficient sets) found no mismatch between all triples and suffix
  triples. Its script and output were
  not retained; this exploratory observation is neither reproducible evidence
  nor proof.
  A temporary integration probe separately observed `Err(NoVertices)` for
  the rank-two rows (1/1 passed) and was removed after the retained fixture
  superseded it.

Proof owner was notified that the boundedness lemma needs rank four and
independent triples, and that duplicate rows break a naive irredundancy
equivalence. These G2 differences block a uniform constructor/error contract
until separately specified and tested. They do not affect the incidence-only
G1 formula. The two exact routes now share that G1 incidence scan, so agreement
of their facet-intersection matrices is not independent evidence for the scan.
