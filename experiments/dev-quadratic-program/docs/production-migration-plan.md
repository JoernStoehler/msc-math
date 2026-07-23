# Four-dimensional QP production migration

Status: staged plan after selection of the reviewed general and product scalar
capacity routes. This plan does not authorize a Main merge.

## Outcome

Move the selected four-dimensional scalar-capacity algorithms from
`experiments/dev-quadratic-program/` into the importable `symplectic` crate
without losing their proof, numerical, exact-reference, adversarial, or
performance evidence.

The migration is intentionally split from orbit-set migration:

- general non-products: certified capacity interval with exact fallback;
- exact structural products: exact binary64-rational capacity and sparse exact
  maximizing witnesses;
- legacy orbit search: retained for callers that need beta, closure
  multipliers, derivatives, near-active branches, or a candidate window.

Neither selected scalar route currently returns every minimizing or
near-minimizing orbit. A common `OrbitSearchResult` would therefore misstate
their contracts.

## Current coordination gate

`/workspaces/msc-math/.worktrees/qp-certified-curvature-route` currently has
uncommitted changes to the same development validation, geometry, f64-route,
performance, and audit files that a direct extraction would touch. Do not edit
that worktree or start a parallel extraction against its old base.

Wait until that work is committed, abandoned, or otherwise given a stable
landing commit. Then create a fresh migration worktree from the chosen
integration commit and integrate `qp-kkt-fallback-diagnostic` (whose algorithm
checkpoint is `e2430b80`) with the accepted changes from the concurrent
migration.

Resolve the integration before changing algorithms. Rerun the two canonical
packets after conflict resolution; a clean compile is not evidence that the
numerical correspondence survived.

This gate blocks code movement, not API/consumer planning.

## Intended production surface

Keep the scalar routes together by consumer purpose while preserving their
different certificates:

```text
crates/symplectic/src/algorithms/
|-- capacity_4d/
|   |-- mod.rs                 # explicit dispatch and common scalar access
|   |-- input.rs               # validated-input boundary and soft errors
|   |-- general/
|   |   |-- mod.rs             # exact transition stream + selected route
|   |   |-- verified_solve.rs  # normwise/batched enclosure + exact fallback
|   |   |-- curvature.rs       # certified obstruction + cyclic inheritance
|   |   `-- interval.rs        # outward binary64 operations
|   `-- product/
|       |-- mod.rs             # closure-vertex capacity route
|       `-- interval.rs        # product-specific outward operations
|-- hk2017/                    # existing enumeration and orbit route
|-- billiard/                  # existing legacy orbit route/control
`-- orbit_search.rs            # existing orbit payload/recovery contracts
```

This is a target ownership layout, not a demand to split small files
prematurely. Start with `mod.rs` plus one implementation file per route; split
only when extraction shows a real single-concern boundary.

The public result should be an enum rather than one weakened common struct:

```text
ScalarCapacity4d::General(GeneralCapacity4d)
ScalarCapacity4d::Product(ProductCapacity4d)
```

Both variants expose an f64 scalar and outward interval. The product variant
also exposes the exact binary64-rational capacity and sparse exact winners.
Route identity remains visible.

Do not put profiling counters or fallback counts in the public mathematical
result. Use opt-in `tracing` for production timing/phase observability. Retain
detailed counters and exact intermediate audits in the development packet.

### Input boundary

The production constructor should soft-error while checking the agreed domain:

- four-dimensional, finite, bounded, full-dimensional polytope;
- origin in the interior and every supplied facet is geometrically relevant;
- at most sixteen facets;
- every dual and recovered primal vertex has infinity norm in
  `[1e-3, 1e3]`; and
- exact structural q/p blocks before product dispatch.

The validated token should derive binary64-rational data from the supplied f64
coordinates. It must not silently substitute a caller's algebraic or source
rational coordinates: both selected certificates concern the exact dyadic
values represented by the binary64 input.

Ordinary `sys` producers may treat failure to construct this token as a hard
precondition failure after their own soft input-filter stage. The numerical
route itself should return explicit errors rather than panic on input.

Near-products stay on the general route. Rounding them into exact products is a
separate changed-input preprocessing operation.

## Stages

### 1. Stabilize the selected code in the development packet

After the coordination gate clears, extract only the selected general kernel
from the 4,000-line ablation binary into an importable
`experiments/dev-quadratic-program/src/` module. Keep baselines, losing
variants, producers, counters, and audit logic in the tool.

The product route is already importable in
`src/product/closure_vertex_capacity.rs`; separate its production kernel from
audit-only types during the same pass.

Completion evidence:

- canonical general packet unchanged in capacity intervals, exact decisions,
  rejection counts, and fallback counts;
- product `sample5.jsonl` unchanged in exact outputs and zero-violation fields;
- focused package tests and clippy pass; and
- representative route time has no unexplained material regression.

This is a useful halfway stopping point. It yields readable, reviewed kernels
without touching production or consumers.

Estimated critical-path cost after the coordination gate: 1--2 agent hours,
plus roughly two minutes of retained producer runtime.

### 2. Move kernels and add the scalar production API

Move rather than copy the selected kernels into `capacity_4d/`. Keep exact
reference enumeration and numerical audits in the experiment packet, now
calling the production implementation for the system under test.

Add direct public tests for:

- known capacities and literature fixtures;
- exact product/general agreement where both routes cheaply apply;
- facet reordering, cyclic rotation, and power-of-two scaling;
- invalid/non-product/near-product routing;
- exact-zero and near-singular cases;
- every determinate predicate against exact binary64 arithmetic; and
- unsupported gradual-underflow behavior taking complete exact fallback.

Run:

```text
cargo test -p symplectic --release --lib
cargo test -p symplectic --release --test public_capacity_api
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route
```

Then rerun the canonical general and product evidence producers. Compare
generated results, not only exit status. Run the development library tests
separately; if the unrelated default artifact scan still sees an unfetched LFS
pointer, record that environment omission rather than treating it as a route
failure.

Estimated cost: 1--2 agent hours. Stop if the concurrent migration changed the
validation or candidate-stream contract enough that the existing proofs no
longer apply.

### 3. Migrate one scalar-only vertical slice

Use `experiments/verification` first. It already owns cross-route correctness,
has no durable cache schema to migrate, and can compare the new scalar result
with the legacy HK/billiard orbit route on the same fixtures.

Do not initially delete its old orbit helpers. Add the scalar route beside
them, migrate scalar assertions, and keep orbit/minimizer assertions on the
legacy or certified-orbit path.

The vertical slice passes when:

- scalar assertions use `ScalarCapacity4d`;
- route selection is visible in failures/output;
- old and new capacities agree on the overlap fixtures;
- the new exact/interval contract is asserted, not converted immediately to an
  unlabelled `f64`; and
- the verification package and `symplectic` suites pass.

Estimated cost: 30--60 minutes.

### 4. Migrate remaining scalar consumers by risk

Recommended order:

1. scalar-only verification and regular-product computations;
2. random/general and random-product producers;
3. `sys-landscape` scalar computation and cache misses;
4. other datascience producers and method-local recomputation.

Before changing a retained cache or JSONL producer, add route/certificate
provenance. Existing fields such as `backend = auto|billiard` and
`capacity_source` do not state whether a value is heuristic, interval-certified,
or exact for binary64 input. Treat this as a schema migration with explicit
old-row compatibility or regeneration, not a field rename.

Estimated cost: 2--4 agent hours depending on cache compatibility. This work
can be planned now but should wait for the other large code migration to land.

### 5. Defer orbit-sensitive consumers

Do not migrate these by discarding their extra outputs:

- gradient ascent and derivative computation;
- local/branch behavior and combinatorial-cell experiments;
- near-active/minimizer-window diagnostics;
- geometric orbit recovery; and
- flow-graph comparisons that require an exact candidate set.

They need a separate output extension:

- for products, recover the required KKT/geometric payload only for the sparse
  exact winners or a requested exact action window;
- for general inputs, retain capacity-relevant candidates during the certified
  route and exact-resolve a caller-requested action window.

That extension is a new algorithm/API question with its own exact completeness
and performance checks. Until it is solved, keep the existing orbit route
explicitly labelled and avoid claiming those consumers use the new scalar
certificate.

## Evidence ownership after migration

- `symplectic`: selected kernels, public contracts, focused regression and
  property tests, ordinary tracing.
- `experiments/dev-quadratic-program`: exact intermediate audits, adversarial
  fixtures, ablation variants, profiling counters, producer commands, retained
  JSONL, and interpretation.
- `formal`: proofs and arithmetic contracts.
- consumer experiment folders: only output-specific agreement/schema tests.

Do not duplicate the ablation study in the crate or erase it after the selected
kernel moves.

## Review and stop conditions

Obtain independent review after stage 1 extraction and after the first
production vertical slice. Review the proof-to-code mapping, not only Rust
style.

Stop and replan if:

- the concurrent migration changes exact transition enumeration, validation,
  or arithmetic dependencies;
- a production predicate gives a wrong determinate result against exact
  arithmetic;
- product dispatch would require approximate block classification;
- moving the selected kernel causes an unexplained correctness or material
  performance regression; or
- a chosen consumer actually needs an orbit set rather than a scalar
  certificate.

No further algorithm search is a prerequisite for stages 1--3.
