# Four-dimensional QP production migration

Status: Batch 1 scalar extraction and the Batch 1b exact-minimizer/on-demand
one-sigma slice are implemented and locally verified in
`qp-production-migration`; not merged to Main. Action-window support, consumer
migration, and the remaining evidence/polish batches remain planned.

## Critical path

```text
current checkpoint
  -> freeze rich QP API
  -> correctness and retained-evidence gate
  -> non-optimizer consumer migration ---+
  -> optimizer-owner API handoff --------+-> thesis rewrite -> final review
```

The active optimizer session owns its worktree and custom branch models. This
branch owns the QP production API, its retained QP evidence, non-optimizer
consumer migration, and the eventual QP thesis rewrite. Do not edit optimizer
files concurrently.

Evidence organization is part of the correctness gate, not an independent
polish project. Build-time optimization and approximately one-percent runtime
differences are not open work: reconsider either only if a measured
dataset-scale or iteration-critical cost becomes material. Before opening a
new strand expected to exceed twenty wall minutes or five dollars of shadow
API cost, record the thesis outcome it can change, the first cheap check, the
total cap, and the stopping condition.

For the outer-to-inner interface and consumer reading surfaces, see
`capacity-architecture.md`.

## Outcome

Move the selected four-dimensional scalar-capacity algorithms from
`experiments/dev-quadratic-program/` into the importable `symplectic` crate
without losing their proof, numerical, exact-reference, adversarial, or
performance evidence.

Batch 1 migrated one mathematical output: the scalar capacity of a validated
four-dimensional input. It did **not** migrate the existing orbit-search
contract or make `OrbitSearchResult` a wrapper around the new result.

The current production surface exposes exactly:

- `GeneralCapacity4d`: one outward binary64 interval `[c_lower, c_upper]` for
  the capacity. It exposes no winning `sigma`, `beta`, `q`, multiplier, or
  search count.
- `ProductCapacity4d`: the exact rational capacity of the binary64 input and
  its outward binary64 interval.
- `QpMinimizers4d`: every tied exact minimizing word in either the complete
  transition-pruned general HK family or the product theorem's
  **closure-vertex word** family. Each candidate contains `sigma` and exact
  action. It is not a representation of every KKT solution when a degenerate
  within-word face contains non-vertex solutions, and it contains no
  non-minimizing action window, `beta`, `mu`, `xi`, or geometric trajectory.
- `solve_sigma_exact`: on-demand exact `beta`, `q`, `mu`, and `xi` for one
  valid requested word, using the dyadic full-rank fast path and the complete
  rank/kernel fallback.

The existing orbit APIs have different, weaker or conditional guarantees:

- `OrbitSearchResult` contains retained f64 KKT records
  `(sigma, beta, action interval, q diagnostic, optional mu, optional xi,
  admissibility)` plus retained-set minima and iteration count.
  `OrbitGuaranteeMode::{BoundSafe,MinimaSafe,AllSafe}` resolves selected
  `IndeterminateF64` records but does not recheck `AdmissibleF64`, and its f64
  action windows are not proved error bounds. It therefore does not currently
  certify a global capacity, a complete set of minimizers, or a complete
  action-gap window.
- `CertifiedOrbitSearchResult` exact-resolves candidates from a caller-supplied
  stream and can return all exact minimizers or an exact action-gap window
  **within that stream**. A global claim additionally requires a proof that the
  supplied stream covers the relevant HK candidate family. Its exact orbit
  record contains `sigma`, `beta`, `q`, and action, but not `mu`, `xi`, or a
  recovered trajectory.

Consequently no caller may manufacture an `OrbitSearchResult` from a scalar
capacity result, label a legacy best word as a certified minimizer merely
because both computations ran on the same input, or treat product
closure-vertex winners as a complete physical-orbit set. The orbit-output part
of this migration must separately specify candidate-family coverage,
tie/window semantics, KKT fields, degeneracy semantics, and geometric
recovery.

## End state and merge boundary

The current scalar extraction is an internal checkpoint, not a merge-ready
half-migration. The migration is not complete merely because `Capacity4d`
exists and old callers still compile. Proposing that state for merge would
leave two plausible ordinary capacity architectures, make it unclear which
result future code should trust, and silently transfer the orbit-output design
and consumer audit to a future session.

The intended end state for this migration is:

1. `CapacityInput4d` is the ordinary validated entry point.
2. One shared search core owns validation, candidate enumeration, pruning, and
   predicate semantics. A narrow request/output mode determines whether it
   stops with the capacity certificate or materializes exact candidate
   records. It is not a generic solver/observer framework.
3. `capacity`, `general_capacity`, and `product_capacity` are thin scalar
   wrappers over that core. General capacity returns certified outward
   binary64 bounds; product capacity also returns the exact rational value
   already computed by its KKT-free route. The general scalar request does not
   exact-resolve its winning word. The product route already has the maximizing
   words while comparing exact contenders, but its scalar result discards
   them and does not construct KKT payloads.
4. Callers that need minimizing words use `qp_minimizers` and receive lean
   `(sigma, action_exact)` records under its stated candidate-family and tie
   guarantees. Exact action-window support remains a named migration item for
   the inventoried window consumers; it is not implied by the minimizer API.
5. Exact one-sigma KKT solving is an on-demand method on the same validated
   input. Given a returned `sigma`, it returns exact `beta`, `q`, `mu`, and
   `xi`. Capacity differentiation and geometric recovery consume that payload.
   Search results do not carry every `beta` merely because an internal f64 or
   exact solve produced one.
6. Existing f64 `OrbitSearchResult` searches remain only where the experiment
   explicitly studies their numerical/branch behavior, or as verification and
   performance controls. Ordinary consumers do not retain them merely because
   the new API omitted a needed field.
7. Every retained old call site states which experiment-specific output
   requires it. A successful compile with an unchanged old call is not a
   migration decision.

Consumer inspection settles the meaning of “all minimizers” here. The
all-minimum verification producer counts and serializes one row per returned
`sigma`; the ascent code builds one gradient per returned `OrbitKktData`; and
the pentagon producer dumps a finite list of tied returned records. None asks
for a parameterization of every positive `beta` or every physical orbit for a
rank-deficient word. The production search therefore returns all covered
minimizing discrete words, and the on-demand exact solver selects one exact
positive KKT witness for a requested word. Continuous within-word solution
families are YAGNI for this API and remain experiment-local if later studied.

No merge proposal is made until the consumer inventory has classified every
old call site as migrated, a named experiment/control, or an explicit blocker.
If external branch coordination ever forces a scalar-first merge, that is a
deliberate incomplete phase: `docs/project-status.md` must name the unresolved
orbit-output phase and its affected callers, and the next migration work must
be scheduled immediately. That exception is not the selected plan for this
worktree.

### Compared orbit-output architectures

| Approach | Benefit | Cost / failure mode | Decision |
| --- | --- | --- | --- |
| Merge the scalar API and keep ordinary callers on `OrbitSearchResult` indefinitely | Cheapest immediate patch | Two ordinary answers with different guarantees; new scalar correctness does not transfer to old winning words or derivatives; every future caller must rediscover the distinction | Reject |
| Make every scalar call return capacity, all branches, KKT multipliers, derivatives, and trajectories | One apparent entry point | Pays enumeration, exact payload, and recovery costs when only a scalar is needed; product degeneracy cannot honestly be represented as one finite “all orbits” list | Reject |
| Preserve the existing `OrbitSearchResult` shape but fill it from the new routes | Minimizes consumer edits | General scalar route has no winner payload; product closure winners have no complete physical-orbit semantics, `mu`, `xi`, or legacy iteration meaning; optional/dummy fields would lie | Reject |
| Expose only capacity plus a low-level one-sigma solve and let every caller build its own minimizer/window search | Small production API | Repeats candidate-coverage, tie, pruning, and fallback logic in consumers; recreates the ad hoc wrappers this migration is meant to remove | Reject for ordinary callers; retain low-level primitives for experiments |
| Shared search core with typed scalar/rich wrappers and on-demand exact one-sigma solve | Enumeration, pruning, and predicate semantics have one owner; each caller requests and pays only for its output | The internal request boundary must keep exact materialization out of scalar mode, and each public result still needs its own contract tests | Select |
| Keep old full-branch searches only in experiments that intentionally measure non-minimal branches, old thresholds, or old solver behavior | Preserves valuable evidence without presenting it as the production route | Requires explicit naming and retained controls | Select as an exception, not the default consumer path |

### Remaining richer-output checks

Before broad consumer edits:

- **General correctness:** retain each accepted candidate's certified action
  interval, identify every word whose interval can meet the exact minimum or
  requested gap, and exact-solve only those words before returning
  `(sigma, action_exact)`. Check against the complete exact all-candidate
  control on retained general, singular, tied, and adversarial cases. Measure
  contender count and exact-solve time. If the contender set is routinely
  large, reconsider the output algorithm instead of silently falling back to
  the old search.
- **Product:** determine whether the returned closure-vertex winners are the
  exact finite certificate set required by product scalar/bounce consumers and
  whether the on-demand exact one-sigma method returns a consistent
  `beta, q, mu, xi` for each returned `sigma`. The separate optimizer worktree
  owns whether such witnesses generate the branch set needed by
  product ascent. This is not a capacity/minimizer-set gate for the present
  migration: the optimizer intentionally maintains custom positive,
  transition-blocked, and beta-nonpositive branch models.

### Predicted compute budget

The 2026-07-25 local anchor processed 13,891 retained long words in 40.5 ms with
the selected route and retained 271 exact-positive candidates. The older
generic rational control took about 2.0 s to exact-solve 285 product survivors;
this is only a scale anchor because the matrices/population differ. It implies:

- the faithful richer-output plus affected numerical/performance packet should
  ordinarily stay below one core-hour locally; and
- stop and inspect rather than launching a broad run if one retained packet
  exceeds ten wall minutes or the contender filter retains thousands of exact
  solves unexpectedly.

Do not allocate a 64-core LICCA hour for the initial migration checks. The
current producer is mostly serial and the local packet is too small to amortize
allocation and transfer overhead. LICCA becomes useful after the API freezes
if full datascience-corpus regeneration, a broad exact-all control, or many
independent adversarial cases require several core-hours. Estimate that stage
from the measured per-case distribution rather than reserving 64 core-hours in
advance.

## Current progress

The old overlapping-worktree gate was resolved by creating
`qp-production-migration` from Main `c8b4b1068` and integrating the diagnostic
research through `0917c4bce`. The stale dirty worktree was not edited.

The current checkpoint provides:

- `CapacityInput4d` with exact binary64-rational geometry validation, exact
  product classification, the `[1e-3, 1e3]` primal/dual infinity-norm contract,
  a sixteen-facet limit, and a 100,000-cycle general-route resource cap;
- automatic exact product dispatch and a forced general route;
- outward general capacity bounds, exact product capacity, exact tied
  minimizing words with an explicit family label, and on-demand exact
  one-sigma KKT payloads;
- reciprocal production/experiment implementation pointers and correspondence
  tests; and
- producer-level comparison against exact predicates and the integrated
  baseline.

Observed extraction checks:

- all 13 public capacity API tests and both selected-route correspondence tests
  pass;
- the 365 non-ignored `symplectic` library tests pass;
- the release workspace build passes;
- the general verification packet has 8/8 production comparisons with zero
  bound mismatches and zero exact predicate/radius violations;
- the 88-case product packet has zero production capacity or winner
  disagreements, zero wrong determinate signs, and zero objective/weight
  interval violations; and
- after removing timing and new correspondence fields, its stable JSON payload
  equals the integrated pre-extraction baseline.

On the retained random `F5` case, the scalar request takes `97 us` while the
one-exact-minimizer request takes `5.21 ms`; exact output therefore does not
regress scalar callers. On the triangle product, scalar and two-minimizer
requests both take about `1.51 ms`, because the product route already identifies
its exact maximizing words.

The adversarial implementation review found that the original constructor
accepted a 16-facet crosspolytope whose exact transition graph has
3,420,783,196 cycles. Eager materialization would require over 82 GB for vector
headers alone. A length-ordered prefix-pruning spike and a short-support
shortcut both failed to finish that case within two minutes; exact
length-six curvature supplied 6,272 obstructions among 52,400 cycles but did
not cheaply settle the remaining search. Those spikes were discarded.
Batch 1 therefore soft-rejects a non-product stream after counting 100,001
cycles without storing it. Forced general evaluation of a structural product
applies the same cap at the public method boundary, while automatic product
dispatch bypasses the general stream. This is a resource-safety boundary, not
a theorem or evidence that larger streams lack an answer. A future general
algorithm may replace the cap only after a bounded crosspolytope-style
regression passes.

The next non-commutative action is to specify the smallest exact action-window
contract required by the inventoried window consumers, then implement and
test its endpoint and candidate-family semantics before editing those
consumers. Consumers that need only scalar capacity, exact minimizing words,
or one exact KKT payload no longer wait on that work.

### Optimizer coordination boundary

The active `optimizer-runs-implementation` worktree owns
`experiments/dev-gradient-ascent/optimizer-runs` and related optimizer
experiments. Its branch model currently:

- requests an admissible action window from the shared orbit search;
- solves named admissible words and differentiates their f64 KKT payloads;
- deliberately enumerates transition-blocked words; and
- has an unrestricted f64 KKT route whose beta sign is descriptive rather than
  an exclusion criterion.

This migration must not refactor those files concurrently. After the
`CapacityInput4d` search and exact one-sigma API are stable, send the optimizer
owner a migration map:

1. use the new scalar/search API for physical capacity and admissible discrete
   candidate windows;
2. use the exact one-sigma method only where an exact positive branch payload
   serves the optimizer;
3. retain the optimizer-local transition-blocked and beta-nonpositive models
   as explicit heuristics rather than expanding the production API for them;
4. review rank-deficient within-word derivative semantics in the optimizer's
   own branch-model contract; and
5. update evaluator schema/cache provenance before that optimizer branch
   merges.

The optimizer worktree need not wait idle for this migration, and the QP
migration need not absorb its custom branch research. The handoff is required
before the optimizer branch merges against the new QP API.

## Mathematical acceptance gate

`formal/product-qp-six-facet-reduction.tex` is agent-derived, independently
agent-reviewed, and remains enclosed in `unverified` because Jörn has not
checked the proof. On 2026-07-24 Jörn nevertheless accepted the theorem for
production as an explicit risk decision if the retained datascience product
population supported it. The retained check then found a literal length-six
cached winner for every one of 10,240 producer geometries, with zero
length-seven-or-longer winners; see
`tools/product_closure_route/RESULTS.md`.

This resolves the production decision without mislabelling the proof as
Jörn-reviewed. Batch 1 may expose exact automatic product-capacity dispatch
under the theorem's stated hypotheses and the product route's exact
binary64-input contract. If later mathematical review finds a gap or the
retained check ceases to reproduce, stop the product migration and retain only
the general route; do not weaken the product result label to bypass that gate.

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
- a framework parameterized over arbitrary solvers, predicates, observers, and
  output policies costs abstraction, proof surface, hot-path branches, and
  copy-edit friction on every future investigation.

The latter is currently more expensive. This cost model is local to the
selected QP routes, not a repo-wide requirement to duplicate ordinary helpers.

| Alternative | Benefit | Long-term cost / failure mode | Decision |
| --- | --- | --- | --- |
| Keep the selected route in `dev-quadratic-program` and wrap it from `symplectic` | Experiment code remains easy to edit | Reverses the dependency direction: production would depend on an experiment package | Reject |
| Keep a readable selected implementation in `dev-quadratic-program` and an optimized corresponding implementation in `symplectic` | Gives experiments a stable copy-editable starting point while production remains optimized | Semantic changes must be applied or checked in two cross-linked files | Select |
| Publish low-level enumeration/KKT pieces and let each consumer independently assemble its ordinary route | Every caller can instrument anything | Ad hoc copies have no named correspondence contract; current `combinatorial-cells` and `hko-local-maximum` helpers demonstrate how old threshold semantics can survive unnoticed | Reject as the ordinary consumer path; deliberate experiment variants remain welcome |
| Give the selected search core one narrow output request and typed wrappers | Keeps enumeration, pruning, and predicate semantics aligned while avoiding discarded exact work | Requires explicit tests that scalar mode does not materialize exact records and that rich mode preserves the same candidate decisions | Select |
| Parameterize one kernel by arbitrary solver, predicate, pruning, observer, and output-policy traits | Can express many ablations without copying | Makes the production proof surface generic, spreads research choices through hot code, and makes ordinary reading harder | Reject until two maintained production consumers demonstrate a real shared variation |
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
|   |-- mod.rs                 # public facade, search/wrapper result contracts
|   |-- input.rs               # validated input and derived exact geometry
|   |-- general.rs             # selected general route
|   |-- product.rs             # conditional on product theorem acceptance
|   `-- interval.rs            # shared outward arithmetic, if both routes use it
|-- exact/orbit.rs             # reusable exact one-sigma KKT solve
|-- hk2017/                    # existing enumeration and orbit route
|-- billiard/                  # existing legacy orbit route/control
`-- orbit_search.rs            # existing orbit payload/recovery contracts
```

Start flat. Split `general.rs` or `product.rs` only when extraction identifies a
named mathematical component that can be read and tested independently.
Avoiding speculative folders keeps the proof-to-code map local. The production
implementation may optimize structure independently; the experiment-owned
readable implementation is the intended copy-editable base.

The implemented caller shape is:

```rust
let input = CapacityInput4d::try_from_dual_vertices(&dual_vertices)?;
let capacity = input.capacity()?; // thin scalar request; no exact payloads

let minimizers = input.qp_minimizers()?;
for candidate in minimizers.candidates() {
    use_sigma_and_exact_action(candidate.sigma(), candidate.action_exact());
}

let orbit = input
    .solve_sigma_exact(minimizers.candidates()[0].sigma())?
    .expect("a returned minimizer has a positive exact KKT witness");
use_exact_beta_q_mu_xi(&orbit);
```

The current public contracts are:

- `CertifiedQpCandidate4d { sigma, action_exact }`, where membership in this
  type means strict exact `beta > 0` and exact `q > 0`; there is no redundant
  `admissibility: true` field;
- `QpCandidateFamily4d`, distinguishing general HK words from product
  closure-vertex words;
- `solve_sigma_exact`, returning the existing exact one-sigma KKT payload
  `(sigma, beta, q, mu, xi)`.

Action-window and all-admissible selectors are not implemented merely for API
symmetry. Add one only with a current consumer and its exact endpoint/coverage
contract.

The exact output scalar is `BigRational`. Although every supplied binary64
coordinate is dyadic, divisions in the exact KKT solve produce general
rationals, so a dyadic result type is insufficient. Algebraic-number output is
unnecessary for this binary64-rational linear/QP contract.

`CapacityInput4d` owns the small validated input and derived binary64-rational
geometry so repeated search, scalar, and one-sigma calls do not repeat
validation or ask callers to keep parallel f64/rational/incidence arguments
consistent. Ordinary automatic dispatch uses exact structural-product
classification. Explicit general/product methods remain for verification and
route-comparison callers.

The convenient scalar API is either a method on `CapacityInput4d` or a
raw-input function that performs validation itself. Do not expose a public raw
`capacity(&[Vector4<f64>])` whose safety depends on an undocumented prior call.
The validated token is the compile-visible prior check. General outward bounds
and the product route's exact rational capacity remain available when the
distinction matters.

Caller-controlled values select returned output, not mathematical validity.
Strict `beta > 0`, exact `q > 0`, omega/adjacency pruning justified by the
candidate-coverage proof, and facet-incidence pruning are production
invariants, not booleans a caller may accidentally disable. Experiments may
copy the readable route and vary those policies deliberately. The ordinary
public search exposes only output choices such as an exact action gap,
capacity multiple, or all admissible discrete candidates.

Do not put profiling counters or fallback counts in the public mathematical
result. Use opt-in `tracing` for production timing/phase observability. Retain
detailed counters and exact intermediate audits in the development packet.

Do not add a builder, solver-policy trait, observer trait, or public
validity/pruning toggle in the first API. The experiment package can adapt the
public results to a small local comparison row or copy the readable route for a
policy experiment without making those research choices part of the production
contract.

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

When the product theorem is accepted, automatic dispatch uses exact
structural-product classification. A confirmed non-product takes the general
route. Once an input is classified as an exact structural product, a
product-route arithmetic or invariant failure is reported; it must not silently
fall back to the general route and erase the failed specialized contract.
Without product acceptance, no automatic product dispatch exists.

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
checkpoint at each non-commutative boundary.

The required boundaries/checkpoints are:

1. product mathematical acceptance or the explicit general-only branch;
2. chosen integration base before editing overlapping route code;
3. faithful scalar production contract;
4. accepted richer-output contract, or an explicit replan if its bounded spike
   fails;
5. accepted evidence harness/negative controls before optimizing the system
   they judge;
6. reviewed optimized implementation before consumer migration and final
   measurements; and
7. consumer/schema checkpoint before thesis integration.

These boundaries create pre-migration, faithful-scalar, faithful-rich-output,
evidence-harness, optimized/evidence, consumer/schema, and thesis checkpoints.
They do not require one commit per result type or call site: the coherent
implementation and migration patches should still be broad enough for compiler
feedback to locate mistakes cheaply.

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

Batch boundaries do not prohibit test-first work. Evidence tests, exact
comparisons, mutations, and consumer compile witnesses may be written or run in
Batch 1 and may intentionally begin red. The evidence-harness checkpoint means
only that the harness must demonstrate its own detection behavior before
optimization relies on it; it is not a rule to postpone tests until Batch 2.

### Batch 1: readable and faithful production routes

Start a fresh migration worktree from the chosen integration commit after the
overlapping worktree has a stable disposition. Record that commit as the
pre-migration checkpoint.

Before editing, run and retain both canonical packets on the integrated base:

```text
experiments/dev-quadratic-program/tools/general_algorithm_ablation/run.sh \
  /tmp/qp-integrated-baseline/general
cargo run -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route -- \
  --samples=5 --timing-repeats=1 \
  > /tmp/qp-integrated-baseline/product.jsonl
```

These are the extraction baseline. A historical packet from an older commit is
not a substitute because the overlapping validation/geometry work can change
the measured contract.

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

The production product implementation, exact-winner public surface, and
automatic product dispatch are included only if the mathematical acceptance
gate is satisfied. Otherwise keep the readable experimental product route and
its controls while completing the general production migration.

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
cargo test -p exp-qp-general-algorithms --release --lib
cargo test -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route
experiments/dev-quadratic-program/tools/general_algorithm_ablation/run.sh \
  /tmp/qp-faithful-production/general
cargo run -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route -- \
  --samples=5 --timing-repeats=1 \
  > /tmp/qp-faithful-production/product.jsonl
```

The post-extraction producers must exercise each migrated readable/production
pair on the same retained inputs and emit its core capacity/certificate
correspondence. Compare regenerated fields with the integrated baseline, not
only process exit status. Commit the faithful production migration only when
the actual producer packets and required correspondence pass. The binary's
small unit-test surface is not a substitute for the retained 88-case product
producer when product production migration is enabled; in the general-only
branch that producer remains an experimental baseline/control rather than
production correspondence.

If compilation or the caller examples show that the architecture is wrong,
do not repair around it. Create a fresh worktree from the pre-migration
checkpoint and cherry-pick only independent proven pieces such as tests,
fixtures, or arithmetic helpers.

Initial cost prediction: roughly 1--3 agent hours plus the retained producer
runs. Re-estimate from the first targeted and workspace builds rather than
defending this estimate.

### Batch 1b: richer discrete-certificate routes

Implemented:

- a lean exact discrete-candidate record containing `sigma` and exact action;
- a general exact-minimizer result that returns every tied HK word in the
  enumerated candidate family;
- product result naming that says “closure-vertex winner”;
- an on-demand exact one-sigma method returning `beta`, `q`, `mu`, and `xi`
  for a returned word;
- no claim to enumerate a continuous family of positive KKT solutions for one
  rank-deficient word, and no claim that product closure-vertex winners are all
  physical trajectories.

Still required in this batch:

- choose and implement the smallest exact action-window contract that covers
  the inventoried window consumers; do not add `AllAdmissible` merely for
  symmetry;
- derivative and recovery adapters that consume the exact one-sigma payload
  without rerunning the global search; and
- complete exact controls for word-set equality, ties, gap endpoints,
  rank-deficient words, derivative intermediates, and recovery inputs.

The scalar and rich methods call the same selected search core with different
output requests. The scalar request stops after producing its capacity
certificate and does not exact-materialize candidate records. The rich request
retains contenders and exact-resolves only the requested output set. Typed
public wrappers keep their different result guarantees explicit without
duplicating enumeration, pruning, or predicate semantics.

Stop before consumer edits if contender isolation, exact KKT payload recovery,
candidate coverage, or performance fails its existing checks. “Keep all
ordinary callers on legacy search” is not the fallback.

### Batch 2: evidence wiring and measured optimization

Against the committed faithful production route, make one evidence/optimization
batch that:

- runs readable-versus-production correspondence across retained general,
  product, scaling, near-singular, and adversarial cases;
- checks every determinate f64 predicate and relevant intermediate against
  exact binary64 arithmetic;
- checks candidate-stream retention, capacity, known values, invariances,
  general/product overlap, old-route controls, and FG agreement where the
  accepted production contracts overlap;
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

First run the new evidence harness against the faithful implementation,
demonstrate that the selected negative controls fail for the intended reasons,
and commit the accepted harness/results as an evidence checkpoint. This
prevents a shared harness/optimization mistake from validating itself.

Then profile the faithful production route once broadly and make one grouped
optimization patch for measured costs. Re-run the complete affected evidence
packet after that patch. Keep detailed metrics with their producers rather than
copying them into prose.

Final performance comparisons use matched input and validation boundaries,
warmed interleaved repetitions where noise matters, toolchain/dependency
provenance, and explicit oracle/readable/faithful/optimized route labels.
Different packet timing scopes must not be presented as direct speed ratios.

Obtain independent proof-to-code, numerical-contract, evidence-scope,
performance-comparison, and public-API review at the end of this batch. Commit
the optimized/evidence checkpoint only after repair or explicit deferral of
findings. No consumer schema migration precedes this checkpoint.

Initial cost prediction: roughly 1--2 agent hours plus retained runs and review,
with a stop/replan if a determinate predicate disagrees with exact arithmetic,
candidate completeness fails, or optimization gives an unexplained regression.

### Batch 3: consumers, durable evidence homes, and thesis

Classify each output field, not each crate. One executable may use the new
capacity for its objective while retaining a deliberately separate orbit
diagnostic. The following table states Batch 1's current coverage and what
Batch 1b must add:

| Requested output | General scalar route | Product scalar route | Required disposition |
| --- | --- | --- | --- |
| Certified capacity | outward interval | exact rational plus outward interval | migrate |
| One minimizing word | exact through `qp_minimizers` | exact closure-vertex word through `qp_minimizers` | migrate callers that need only a discrete word |
| All tied discrete words | exact for the complete transition-pruned general HK family | complete for the product route's enumerated closure-vertex words | migrate with the returned family label preserved |
| All positive KKT solutions or physical trajectories | unavailable | unavailable under within-word/closure-face degeneracy | no inventoried ordinary consumer requests this object; keep it out of the production API unless a concrete experiment later defines it |
| All discrete candidates within action gap `delta` | unavailable | unavailable | Batch 1b adds this for the covered general candidate family; a product version is added only if an inventoried ordinary caller needs it |
| exact `beta` for a chosen word | on-demand through `solve_sigma_exact` | on-demand through `solve_sigma_exact` | the common search record remains lean |
| exact `q`, closure multiplier `mu`, normalization multiplier `xi` | on-demand through `solve_sigma_exact` | on-demand through `solve_sigma_exact` | these values remain properties of a chosen word, not the global search result |
| Capacity derivative for one branch | unavailable | not directly returned | requires `sigma`, `beta`, `q`, and `mu` from one consistent KKT solution |
| Clarke subgradient / switching model | unavailable | unavailable | requires a complete active-branch set plus one consistent derivative per branch |
| Breakpoints, dwell times, closure/inside/action residuals | unavailable | unavailable | run geometric recovery from a chosen `sigma`, `beta`, and action; this certifies neither minimizer completeness nor recovery tolerances by itself |
| Candidate counts, f64 fallback counts, and phase timing | tracing only | tracing only | use tracing or an experiment implementation; do not reinterpret them as `OrbitSearchResult::iterations` |

The known high-impact consumers then have the following concrete disposition:

| Consumer | Fields/semantics used now | Batch 3 decision |
| --- | --- | --- |
| `sys-landscape/src/ascent/compute.rs` | capacity point value; all returned branches tied within `1e-9`; each branch's `sigma`, `beta`, `q`, and `mu`; Clarke/maximin derivative construction | Do not pretend the scalar API covers it. Migrate its word set to Batch 1b's exact search, then call the exact one-sigma method for each selected word. Product migration additionally requires the product-subdifferential claim. Otherwise this is an explicit blocker, not an accepted ordinary legacy caller. |
| `sys-landscape/gradient-ascent-observed-general` | a caller-chosen action window; every retained near-active branch; derivatives; returned count and iterations | Migrate its mathematical window/derivative input to Batch 1b. Retain old counts/timings only as separately named diagnostics. If the experiment intentionally compares the old heuristic window, that comparison mode remains but is not its ordinary capacity backend. |
| `sys-landscape/src/datascience_cache.rs` and computed-polytope rows | capacity, volume, `sys`, one `SigmaAction`, and `OrbitScalars` (`iterations`, returned count, beta margin, q diagnostic, multiplier-presence flags, admissibility flags) | Split the schema. Store the new capacity certificate independently and use Batch 1b for a general winner when the producer needs one. Do not fill `OrbitScalars` from the new route. Retain a legacy diagnostic block only when a named analysis consumes those old-route diagnostics. |
| `sys-landscape/random-product-sample` and regular-product rotated sweeps | capacity, `sys`, one best `sigma`, bounce count derived from that `sigma`, and legacy iteration count | Migrate capacity and the chosen winner/bounce to `ProductCapacity4d`; choose the lexicographically first returned winner deterministically. Drop or rename the old iteration field in regenerated rows unless the analysis needs a legacy-search comparison. Production tracing, not a fabricated iteration count, measures the new route. |
| `regular-products/pentagon-rotation-empirics` minima mode | all numerically tied returned orbits and per-orbit `sigma`, `beta`, action bounds, `q`, q diagnostic, admissibility, and bounce count | Do not replace with product winners: the producer claims an orbit-branch dataset, not merely closure-vertex maximizers. Run the new product capacity as a scalar cross-check and retain the existing branch producer with its legacy numerical scope stated. |
| `regular-products/pentagon-rotation-empirics` three-bounce/branch-landscape modes | admissible non-minimal branches and per-sigma solve outcomes | No scalar migration; these modes intentionally study the branch landscape. |
| `sys-datascience/equal-budget-product-search` | capacity/`sys` for ranking plus counts and support lengths of the returned legacy orbit payload; full `OrbitSearchResult` in cache exports | Use the exact product capacity for ranking. Move returned-word counts/support lengths into an optional, explicitly legacy diagnostic block or omit them from new production runs; they are not properties of the product scalar algorithm. Cache eligibility must include the route/schema fingerprint, so an old `OrbitSearchResult` row cannot satisfy a new exact-product query. |
| `dev-gradient-ascent/optimizer-runs` on `optimizer-runs-implementation` | admissible action windows and KKT derivatives, plus custom transition-blocked and beta-nonpositive branch models | Do not edit concurrently. After the API freezes, hand off the migration map above. Only its physical capacity/admissible-window calls migrate; its deliberately broader branch heuristics remain optimizer-local. |
| `hko-local-maximum` and `combinatorial-cells` instrumented helpers | depending on the binary: best word/subset, all accepted f64 branches, uncertain best action, number of valid branches, or best/second-best gap | Migrate capacity/best-word users to the new scalar or Batch 1b certificate. Retain instrumented routes only in binaries whose stated subject is the old acceptance policy, all non-minimal accepted branches, or old best/second-best diagnostics. A crate-level wrapper replacement is forbidden because the binaries have different contracts. |
| verification and performance packets | independent old/exact controls, intermediate predicates, recovery, candidate counts, or matched timing | Keep both old and new routes under explicit names. These are evidence/control consumers, not migration debt. |

For rows that contain both computations, use two tagged blocks rather than
shared optional fields:

```text
capacity_certificate:
  general { lower, upper, contract_version }
  product { exact_numerator, exact_denominator, lower, upper,
            closure_vertex_winners, contract_version }

legacy_orbit_diagnostic:
  { backend, guarantee_mode, action_gap, OrbitSearchResult, contract_version }
```

The second block never inherits the word “winner” from the first. If a producer
needs to assert that a legacy word attains the new capacity, it must exact-solve
that word and compare its exact action with the exact product capacity, or use a
Batch 1b certified general winner result. Interval overlap alone is not
equality.

Historical JSONL remains immutable. A regenerated schema that previously had
one `capacity: f64` must store the capacity bounds (and the exact product
rational when applicable). A compatibility `capacity: f64` may remain only
when its rounding rule and approximate status are named. Likewise:

- with exact positive volume `V`, general `sys` bounds are
  `[c_lower^2/(2V), c_upper^2/(2V)]`;
- product `sys` is exact when both product capacity and volume are retained as
  rationals; and
- a producer that converts volume or capacity to f64 must label `sys` as an
  approximation rather than certified.

Root `cargo check --workspace` does not cover every experiment. Before editing
consumers, inventory every `Cargo.toml` and capacity/orbit call site, compare
that inventory with root `cargo metadata`, and explicitly check affected
standalone manifests with `cargo check --manifest-path`. Currently known
standalone surfaces requiring explicit compile/semantic disposition include:

- `alternative-source-transfer`;
- `equal-budget-product-search`;
- `extreme-scalar-rejection-proposer`;
- `generic-ridge-tail-stage1-target`;
- `ridge-endpoint-path`; and
- `ridge-symmetry-completion`.

Because legacy APIs remain available, successful compilation is not evidence
that semantic migration is complete. The call-site/schema inventory must state
which route contract each retained use intentionally consumes.

Make one migration/schema patch that:

- migrates every field whose contract is covered by the table above;
- leaves each uncovered orbit field on a separately named legacy or exact
  producer rather than synthesizing it from the scalar result;
- adds one explicit compatibility/regeneration policy for retained caches and
  JSONL instead of per-producer ad hoc fields;
- adds a route-contract/version fingerprint to resumable-cache eligibility,
  including the requested backend, rather than accepting a row by `poly_id`
  alone;
- establishes exact/outward volume and `sys` composition before any consumer
  claims certified `sys`; otherwise keeps `sys` explicitly approximate;
- removes duplicated ordinary wrappers made obsolete by the public API;
- migrates ordinary active-set, gap-window, and derivative consumers to the
  Batch 1b result or reports them as blockers before any merge proposal; and
- retains old branch-landscape or trajectory searches only when that old/full
  branch object is the stated subject of the experiment, not as an accidental
  dependency of an ordinary capacity caller.

After the consumer build/test feedback, promote only stable reusable evidence
to the verification/numerics/performance homes, update formal proof/code
correspondence, and rewrite the thesis algorithm/numerics/performance-complexity
explanation from the final implementation and retained evidence. Do not publish
pre-optimization measurements as final comparisons.

Run the workspace checks, affected consumer suites, retained schema
round-trips/regeneration checks, affected standalone-manifest checks, and the
final production evidence packet. A round-trip alone does not establish cache
validity: test that backend/version mismatches cannot hit an old row.

Distinguish immutable historical artifacts from resumable compute caches.
Historical rows retain their original route/provenance labels; resumable caches
must reject or explicitly migrate legacy rows without the current contract
fingerprint.
Obtain final code/API/consumer/thesis-evidence review, then commit the consumer
and thesis checkpoints separately so either can be reused or redone without
salvaging the other.

Initial cost prediction: roughly 2--4 agent hours depending on schema
compatibility and how many consumers require richer certificates or intentional
legacy diagnostics. Re-estimate after the workspace build identifies the
actual compile surface.

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
- Exact minimizer and discrete action-window results are part of this
  migration. Geometric trajectory recovery remains a separate transformation
  because it has different inputs, costs, non-uniqueness, and tolerance
  semantics; this is an explicit end-state decision, not deferred migration.
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
