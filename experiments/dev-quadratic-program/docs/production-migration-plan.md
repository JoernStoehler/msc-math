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
3. **Reference and control routes:** the general exact-all-visited-sigma route
   is independent of the selected f64 predicates. The product exact-all audit
   is only a coupled interval-pruning control because it shares support and
   cyclic-order enumeration with the hybrid. Independent product controls are
   the general exact KKT route, old product route, known values, and
   mathematical/code review.
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
applicable exact controls; agreement between two related implementations is
not treated as independent correctness evidence. In particular,
hybrid-versus-exact product agreement checks interval pruning, not shared
support or cyclic-order enumeration.

An experiment that only needs production timings calls the production route
and uses `tracing`. An experiment that needs detailed counters can use the
readable selected implementation. An experiment that changes a cutoff,
predicate, factorization, or pruning rule copies the readable implementation
locally and compares the variant against both production output and the
applicable independent controls.

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

`CapacityInput4d` must establish these conditions, not merely preserve the
current f64 validator's `AcceptedAmbiguous` status. Resolve ambiguous geometry
with exact binary64-rational validation when available; otherwise return a
validation-indeterminate error. Do not construct the validated token from an
ambiguous report.

Keep input validation, explicit-route applicability, exact-fallback failure,
and internal-invariant errors distinguishable. The caller spike should cover at
least invalid geometry, validation indeterminacy, non-structural product when a
product route is explicitly requested, exact fallback failure, and internal
invariant failure.

Unsupported f64 arithmetic assumptions, including unavailable gradual
underflow, route to complete exact fallback rather than becoming caller-visible
failure. Report an arithmetic/fallback error only if the required exact route
cannot run. For a fully validated bounded input and complete candidate stream,
finding no positive capacity candidate contradicts the mathematical contract
and is an internal invariant failure, not ordinary mathematical non-success.

Automatic dispatch uses exact structural-product classification. A confirmed
non-product takes the general route. Once an input is classified as an exact
structural product, a product-route arithmetic or invariant failure is
reported; it must not silently fall back to the general route and erase the
failed specialized contract.

Ordinary `sys` producers may treat failure to construct this token as a hard
precondition failure after their own soft input-filter stage. The numerical
route itself should return explicit errors rather than panic on input.

Near-products stay on the general route. Rounding them into exact products is a
separate changed-input preprocessing operation.

### Arithmetic dependency boundary

The reviewed general numerical theorem assumes pinned `nalgebra 0.33.3` for
certified products and `nalgebra 0.35.0` for Bunch--Kaufman factorization,
together with the recorded `matrixmultiply`, dimension, finite-value, and
gradual-underflow conditions. The development package currently imports
`nalgebra 0.35` under the `nalgebra035` alias, while `symplectic` has only the
workspace `nalgebra 0.33` dependency.

Production extraction therefore requires an explicitly pinned aliased 0.35.0
dependency unless a separate review chooses to change the arithmetic contract.
Do not replace or upgrade either version incidentally during migration. Any
dependency, target, compiler-arithmetic, or dimension change triggers review
of `rem:kkt-batched-binary64-contract` and reruns the general numerical packet.

## Execution batches and checkpoints

The migration should use the compiler and tests as navigation. Do not turn
every logical dependency into a separate edit/review cycle. After the
coordination gate clears, use three large coherent batches with one committed
checkpoint between them.

The only required non-commutative boundaries are:

1. choose the integration base before editing overlapping route code;
2. establish the production contract before consumer schemas and thesis prose
   depend on it; and
3. stabilize the implementation before recording final performance,
   complexity, and thesis claims.

Within a batch, prefer a broad patch followed by hard feedback:

```text
targeted cargo check
-> workspace cargo check
-> focused tests
-> exact/correspondence/numerical producers
-> release profiling
```

Use grep primarily for surfaces the compiler cannot identify: copied
implementations, JSONL/cache schemas, documentation claims, and semantic
consumers. Do not polish code that fails because the chosen boundary is wrong.

### Batch 1: readable and faithful production routes

Start a fresh migration worktree from the chosen integration commit after the
overlapping worktree has a stable disposition. Record that commit as the
pre-migration checkpoint.

Make one coherent implementation patch containing:

- the selected general kernel extracted from the ablation binary into
  `src/selected_route/general.rs`;
- the selected product kernel separated from its coupled exact-pruning audit
  into `src/selected_route/product.rs`;
- faithful corresponding implementations under
  `symplectic::algorithms::capacity_4d`;
- reciprocal readable/production correspondence headers;
- the pinned aliased arithmetic dependencies;
- exact binary64-rational validation, input/applicability/fallback/invariant
  errors, exact structural-product dispatch, and complete exact fallback;
- the provisional validated-input and route-specific-result API;
- compile-checked ordinary, forced-route, validation-error, exact-winner, and
  tracing caller examples;
- focused public tests and crate README/DEVELOPMENT documentation.

The caller examples compare the two serious public shapes inside this batch:
validated input plus a result enum versus raw free functions plus a certificate
enum. Keep only the clearer compiled shape; do not retain two permanent
convenience layers.

The first implementation is faithful, not newly optimized. Preserve the
canonical general capacity intervals/decision/fallback counters and the product
exact outputs/zero-violation fields. Check copy-editability by making and
compiling one disposable local cutoff variant, then delete it.

Use build failures to locate missing imports, privacy mistakes, dependency
boundaries, and caller assumptions. Then run:

```text
cargo check -p symplectic -p exp-dev-quadratic-program
cargo check --workspace
cargo test -p symplectic --release --lib
cargo test -p symplectic --release --test public_capacity_api
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-general-algorithm-ablation
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route
```

Compare regenerated evidence fields, not only process exit status. Commit the
faithful production migration only when this hard feedback passes.

If compilation or the caller examples show that the architecture is wrong,
do not repair around it. Create a fresh worktree from the pre-migration
checkpoint and cherry-pick only independent proven pieces such as tests,
fixtures, or arithmetic helpers.

Initial cost prediction: roughly 1--3 agent hours plus the retained producer
runs. Re-estimate from the first targeted and workspace builds rather than
defending this estimate.

### Batch 2: evidence wiring and measured optimization

Against the committed faithful production route, make one evidence/optimization
batch that:

- runs readable-versus-production correspondence across retained general,
  product, scaling, near-singular, and adversarial cases;
- checks every determinate f64 predicate and relevant intermediate against
  exact binary64 arithmetic;
- checks candidate-stream retention, capacity, known values, invariances,
  general/product overlap, old-route controls, and FG agreement where the
  contracts overlap;
- keeps the product exact-all route labelled as a coupled interval-pruning
  audit and uses the independent product controls named above;
- exercises unavailable gradual underflow and every public validation,
  applicability, fallback, and invariant route;
- profiles phase times, counts, memory if material, latency/fallback tails,
  facet-count/conditioning scaling, and representative product/general cases;
- records the simple/exact/old/faithful/optimized ablation and the review-code
  surface; and
- uses selected negative controls or mutations to demonstrate that critical
  predicate, rounding, pruning, and fallback regressions are detected.

Profile the faithful production route once broadly, then make one grouped
optimization patch for measured costs. Re-run the complete affected evidence
packet after that patch. Keep detailed metrics with their producers rather than
copying them into prose.

Obtain independent proof-to-code, numerical-contract, evidence-scope,
performance-comparison, and public-API review at the end of this batch. Commit
the optimized/evidence checkpoint only after repair or explicit deferral of
findings. No consumer schema migration precedes this checkpoint.

Initial cost prediction: roughly 1--2 agent hours plus retained runs and review,
with a stop/replan if a determinate predicate disagrees with exact arithmetic,
candidate completeness fails, or optimization gives an unexplained regression.

### Batch 3: consumers, durable evidence homes, and thesis

Use the workspace build plus source inspection to classify all consumers:

- **Scalar-only:** consumes only the capacity certificate.
- **Dual-route:** needs the new scalar certificate plus legacy sigma, orbit,
  bounce, iteration, derivative, or trajectory payload.
- **Orbit-sensitive:** depends on complete minimizer/window/branch semantics.

Current source inspection already classifies `sys-landscape` computed payloads,
random-product/datascience producers, and regular-product sweeps as at least
dual-route. They store best sigmas, orbit scalars, bounce counts, iterations, or
tied orbit lists. Sparse product winners do not automatically replace those
contracts.

Make one migration/schema patch that:

- migrates all scalar-only consumers;
- gives compatible dual-route consumers separately provenance-labelled scalar
  certificates and legacy orbit payloads;
- adds one explicit compatibility/regeneration policy for retained caches and
  JSONL instead of per-producer ad hoc fields;
- establishes exact/outward volume and `sys` composition before any consumer
  claims certified `sys`; otherwise keeps `sys` explicitly approximate;
- removes duplicated ordinary wrappers made obsolete by the public API; and
- leaves genuinely orbit-sensitive consumers on the explicitly labelled legacy
  route until a separately proved/tested output extension exists.

After the consumer build/test feedback, promote only stable reusable evidence
to the verification/numerics/performance homes, update formal proof/code
correspondence, and rewrite the thesis algorithm/numerics/performance-complexity
explanation from the final implementation and retained evidence. Do not publish
pre-optimization measurements as final comparisons.

Run the workspace checks, affected consumer suites, retained schema
round-trips/regeneration checks, and the final production evidence packet.
Obtain final code/API/consumer/thesis-evidence review, then commit the consumer
and thesis checkpoints separately so either can be reused or redone without
salvaging the other.

Initial cost prediction: roughly 2--4 agent hours depending on schema
compatibility and how many consumers remain dual-route. Re-estimate after the
workspace build identifies the actual compile surface.

## Recovery policy

Git commits are experimental checkpoints, not sunk-cost commitments. Prefer a
fresh worktree from the preceding accepted checkpoint when:

- an API boundary requires widespread adapters merely to compile;
- a migration obscures rather than clarifies the mathematical algorithm;
- a schema design cannot state provenance without optional-field ambiguity;
- a performance rewrite changes numerical semantics unexpectedly; or
- repairing the batch costs more than reapplying its independently useful
  pieces.

Cherry-pick tests, exact controls, fixtures, and documentation only when they
remain valid independently of the rejected architecture. Git history is enough
for discarded process; do not preserve a bad implementation as a maintained
variant.

## Evidence ownership after migration

- `symplectic`: optimized selected production kernels, public contracts,
  focused regression and property tests, and ordinary tracing.
- `experiments/dev-quadratic-program`: exact intermediate audits, adversarial
  fixtures, independent and coupled reference/control routes with their scope
  stated, readable selected implementations, correspondence tests, actual
  ablation variants, profiling counters, producer commands, retained JSONL,
  and interpretation.
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

This plan receives independent adversarial review before Batch 1. Obtain
independent proof-to-code/evidence review after Batch 2 before consumer
migration, then final code/API/consumer/thesis-evidence review after Batch 3.
Review the mathematical and evidence contracts, not only Rust style.

Stop and replan if:

- the concurrent migration changes exact transition enumeration, validation,
  or arithmetic dependencies;
- a production predicate gives a wrong determinate result against exact
  arithmetic;
- product dispatch would require approximate block classification;
- moving the selected kernel causes an unexplained correctness or material
  performance regression; or
- consumer migration would erase or mislabel an orbit/window/schema contract.

No further algorithm search is a prerequisite for Batches 1--3.
