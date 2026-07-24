# Four-dimensional QP production migration

Status: staged plan after selection of the reviewed general and product scalar
capacity routes and a caller/ownership architecture review. Public names remain
provisional until the stage-0 caller spike. This plan does not authorize a Main
merge.

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

## Architecture comparison

The first version of this plan chose `capacity_4d/` plus one public result enum
without comparing enough ownership alternatives. The following comparison is
now part of the migration decision. The initial comparison also overestimated
the cost of maintaining two explicitly linked implementations.

### Maintenance cost model

The primary maintainers of this code are agents. Reading one additional focused
Rust file and applying the same local semantic edit in two named places is
cheap. Reciprocal file-header comments make the relationship discoverable;
correspondence tests expose many missed updates; and Git history makes a missed
counterpart edit easy to reconstruct. A human maintainer would pay a more
noticeable duplicate-read/edit cost, so the correspondence should remain
limited to one readable base and one optimized production implementation.

For this route, do not apply a generic “duplication must drift” argument. The
important comparison is:

- explicit duplicated algorithms cost cheap repeated edits; while
- a configurable shared framework costs abstraction, proof surface, hot-path
  branches, and copy-edit friction on every future investigation.

The latter is currently more expensive. This cost model is local to the
selected QP routes, not a repo-wide requirement to duplicate ordinary helpers.

| Alternative | Benefit | Long-term cost / failure mode | Decision |
| --- | --- | --- | --- |
| Keep the selected route in `dev-quadratic-program` and wrap it from `symplectic` | Experiment code remains easy to edit | Reverses the dependency direction: production would depend on an experiment package | Reject |
| Keep a readable selected implementation in `dev-quadratic-program` and an optimized corresponding implementation in `symplectic` | Gives experiments a stable copy-editable starting point while production remains optimized | Semantic changes must be applied or checked in two cross-linked files | Select |
| Publish low-level enumeration/KKT pieces and let each consumer independently assemble its ordinary route | Every caller can instrument anything | Ad hoc copies have no named correspondence contract; current `combinatorial-cells` and `hko-local-maximum` helpers demonstrate how old threshold semantics can survive unnoticed | Reject as the ordinary consumer path; deliberate experiment variants remain welcome |
| Parameterize one kernel by solver, predicate, pruning, observer, and output-policy traits | Can express many ablations without copying | Makes the production proof surface generic, spreads research choices through hot code, and makes ordinary reading harder | Reject until two maintained production consumers demonstrate a real shared variation |
| Put the kernel in a new internal workspace crate and re-export it from `symplectic` | Production and experiments can share a lower layer | Adds another public dependency/API boundary but does not remove the need to distinguish production semantics from experimental variants | Defer; reconsider only if extraction finds a substantial independently reusable numerical kernel |
| Use feature-gated diagnostics or a public observer callback in production | Rich counters without route copies | Adds control flow and API combinations to the correctness-critical path; feature combinations become another test matrix | Reject for now |

The selected ownership model has four distinct artifacts:

1. **Readable selected implementation:** a concrete, instrumentable
   implementation in `dev-quadratic-program`. It is the preferred
   copy-editable starting point and is maintained to match production
   mathematical semantics, not production code structure or runtime.
2. **Optimized production implementation:** the importable `symplectic`
   implementation used by ordinary consumers.
3. **Independent references:** exact enumeration/solves and adversarial
   fixtures in `dev-quadratic-program`. These must not share the production
   or readable-base predicate/pruning code whose correctness they test.
4. **Experiment variants:** local copies made from the readable selected
   implementation when an actual experiment changes route semantics. The
   variant records its source commit and intentional differences.

The readable and production files name each other in their headers. Those
headers state which mathematical outputs and predicates must correspond and
which differences are intentional, such as storage, batching, instrumentation,
or factorization reuse. A change to either header-listed semantic contract
requires checking both files and the correspondence suite.

The correspondence suite compares capacities/certificates across the retained
general, product, scaling, near-singular, and adversarial cases. Predicate and
intermediate numerical correctness are checked separately against the
independent exact audits; agreement between two related implementations is not
treated as independent correctness evidence.

An experiment that only needs production timings calls the production route
and uses `tracing`. An experiment that needs detailed counters can use the
readable selected implementation. An experiment that changes a cutoff,
predicate, factorization, or pruning rule copies the readable implementation
locally and compares the variant against both production output and the
independent exact reference.

## Intended production surface

Keep the scalar routes together by consumer purpose while preserving their
different certificates:

```text
experiments/dev-quadratic-program/src/
`-- selected_route/
    |-- mod.rs                 # readable result/counter adapters
    |-- general.rs             # readable selected general algorithm
    `-- product.rs             # readable selected product algorithm
```

```text
crates/symplectic/src/algorithms/
|-- capacity_4d/
|   |-- mod.rs                 # public facade, dispatch, and result contracts
|   |-- input.rs               # validated input and derived exact geometry
|   |-- general.rs             # selected general route
|   |-- product.rs             # selected closure-vertex product route
|   `-- interval.rs            # shared outward arithmetic, if both routes use it
|-- hk2017/                    # existing enumeration and orbit route
|-- billiard/                  # existing legacy orbit route/control
`-- orbit_search.rs            # existing orbit payload/recovery contracts
```

Start flat. Split `general.rs` or `product.rs` only when extraction identifies a
named mathematical component that can be read and tested independently.
Avoiding speculative folders keeps the proof-to-code map local. The production
implementation may optimize structure independently; the experiment-owned
readable implementation is the intended copy-editable base.

The provisional caller shape is:

```rust
let input = CapacityInput4d::try_from_dual_vertices(&dual_vertices)?;
let result = input.capacity()?;

match result {
    Capacity4d::General(result) => use_certified_bounds(result.bounds()),
    Capacity4d::Product(result) => use_exact_binary64(result.capacity_exact()),
}
```

`CapacityInput4d` owns the small validated input and derived binary64-rational
geometry so repeated route calls do not repeat validation or ask callers to
keep parallel f64/rational/incidence arguments consistent. It offers explicit
general and product methods for verification and route-comparison callers;
ordinary automatic dispatch uses the exact structural-product classification.

`Capacity4d` is an enum because the certificates really differ:

- `GeneralCapacity4d` exposes certified outward bounds. It does not present an
  unqualified f64 point as the authoritative answer.
- `ProductCapacity4d` exposes the exact binary64-rational capacity and sparse
  exact winners.

The enum provides only operations that preserve both contracts, principally
outward bounds and route identity. Any midpoint/representative f64 conversion
must be explicitly named as an approximation. Do not weaken both variants into
one struct with optional exact fields, and do not give an interval-only result
a method whose name implies exact scalar capacity.

Do not put profiling counters or fallback counts in the public mathematical
result. Use opt-in `tracing` for production timing/phase observability. Retain
detailed counters and exact intermediate audits in the development packet.

Do not add a builder, solver-policy trait, observer trait, or public cutoff
option in the first API. The selected route has no current production caller
that needs those choices. The experiment package can adapt the public results
to a small local comparison row without making that adapter part of the
production contract.

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

### 0. Compile the caller API spike

Before moving the selected kernels, replace the current development-only
consumer sketch with a small compile-checked sketch of the proposed production
surface. Cover these real caller shapes:

- validate once, compute the automatically selected scalar certificate, and
  compute `sys` bounds;
- force both routes on an exact structural product for agreement testing;
- report a soft validation error;
- access exact product witnesses without adding optional fields to the general
  result; and
- time one production call with tracing while keeping route counters out of the
  mathematical result.

Compare two caller APIs in this spike:

1. validated input plus route-specific result enum (current prediction); and
2. raw free functions returning one common result with a certificate enum.

Prefer the first unless the compile-checked callers show that the validated
input is ceremony without reuse or that matching the result enum causes
repeated conversions. Do not implement both as permanent convenience layers.

Completion evidence is the checked-in caller example plus a short decision
paragraph here naming the observed friction. Estimated cost after the
coordination gate: 15--30 minutes.

### 1. Stabilize the selected code in the development packet

After the coordination gate clears, extract only the selected general kernel
from the 4,000-line ablation binary into an importable
`experiments/dev-quadratic-program/src/` module. Keep baselines, losing
variants, producers, counters, and audit logic in the tool.

The product route is already importable in
`src/product/closure_vertex_capacity.rs`; separate its selected kernel from its
independent exact-reference and audit-only types during the same pass.

These extracted modules become the readable selected implementations. They
must read as ordinary concrete algorithms, not miniature frameworks. Retain
useful counters when they do not obscure the algorithm. A small helper shared
by the two readable routes is allowed when it expresses the same arithmetic
operation and proof contract; a helper shared by a selected implementation and
its exact oracle is suspect because it can make a shared bug pass an audit.

Do one disposable copy-editability check after extraction: copy the focused
general module into a temporary experiment-local variant, change one real
ablation choice such as the obstruction cutoff, and compile it. Record only the
dependency friction, then delete the variant. This checks that the promised
base is actually easy to adapt before production is built around it.

Completion evidence:

- canonical general packet unchanged in capacity intervals, exact decisions,
  rejection counts, and fallback counts;
- product `sample5.jsonl` unchanged in exact outputs and zero-violation fields;
- one real local variant can be produced by copying a focused readable module
  without importing an experimental framework;
- focused package tests and clippy pass; and
- representative route time has no unexplained material regression.

This is a useful halfway stopping point. It yields readable, reviewed kernels
without touching production or consumers.

Estimated critical-path cost after the coordination gate: 1--2 agent hours,
plus roughly two minutes of retained producer runtime.

### 2. Add corresponding optimized kernels and the scalar production API

Implement the corresponding optimized kernels in `capacity_4d/`, starting from
the readable selected implementations. Keep the readable versions, exact
reference enumeration, and numerical audits in the experiment packet. Add
reciprocal correspondence headers before optimizing code structure.

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
generated results, not only exit status. The experiment packet must run the
readable and production implementations on the same retained inputs and compare
their certificates. Run the development library tests separately; if the
unrelated default artifact scan still sees an unfetched LFS pointer, record
that environment omission rather than treating it as a route failure.

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

- scalar assertions use `Capacity4d`;
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

- `symplectic`: optimized selected production kernels, public contracts,
  focused regression and property tests, and ordinary tracing.
- `experiments/dev-quadratic-program`: exact intermediate audits, adversarial
  fixtures, independent exact references, readable selected implementations,
  correspondence tests, actual ablation variants, profiling counters, producer
  commands, retained JSONL, and interpretation.
- `formal`: proofs and arithmetic contracts.
- consumer experiment folders: only output-specific agreement/schema tests.

Do not duplicate the ablation study in the crate or erase it after the selected
kernel moves.

## Deferred architecture questions

- A reusable observer/policy layer is deferred until repeated concrete edits
  show that explicit readable variants are more expensive. Under the current
  agent-maintenance cost model, two variants needing the same hook is not by
  itself enough.
- A separate numerical-kernel crate is deferred until extraction demonstrates
  a component with consumers beyond this capacity route.
- Orbit/minimizer-window recovery remains a separate API because its payload
  and completeness contract differ from scalar capacity.
- Whether `CapacityInput4d` should reuse a future general polytope value type is
  deferred. No such public validated type currently exists in the crate, so
  inventing a project-wide geometry abstraction during this migration would
  mix unrelated maintenance with the route promotion.

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
