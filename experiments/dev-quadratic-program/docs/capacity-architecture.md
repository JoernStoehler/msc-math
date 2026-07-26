# Four-dimensional capacity architecture

Status: target architecture for the current unmerged QP production migration.
Implemented and missing interfaces are marked explicitly below.

This document starts with the mathematical questions asked by ordinary
consumers and works inward toward algorithms, numerical kernels, and retained
evidence. `production-migration-plan.md` separately owns migration order and
acceptance gates.

## Outer mathematical questions

The architecture supports four recurring questions.

### Systolic ratio

> Given a supported four-dimensional polytope, compute its systolic ratio to a
> requested relative tolerance so later mathematical or datascience work does
> not need to reason about capacity algorithms or numerical outliers.

```text
systolic_ratio(polytope, relative_tolerance)
                 |
       +---------+---------+
       |                   |
    volume              capacity
       |                   |
       +------ outward error composition
                           |
             tolerance-checked scalar result
```

Every emitted result must satisfy the requested tolerance. An unsupported
input, unresolved numerical case, nonpositive volume, or overly wide interval
is an error or rejected dataset row, not a plausible-looking scalar.

The current migration implements certified capacity bounds but does **not**
yet provide the outer systolic-ratio composition or its tolerance-checked
result.

### Scalar capacity

> Given validated dual vertices, compute capacity without deciding which
> general or product algorithm to call.

```text
validated dual vertices
          |
          v
 automatic capacity dispatch
     |                |
 structural        general
 product           polytope
     |                |
 exact closure     HK candidate search
 vertex route      + certified f64 predicates
                   + lazy exact fallback
     |                |
     +--- certified capacity result
```

Implemented:

- `CapacityInput4d::try_from_dual_vertices`;
- `CapacityInput4d::capacity`;
- exact product dispatch;
- an exact rational product capacity and outward binary64 bounds;
- certified outward binary64 bounds for the general route.

Still missing:

- a convenience operation that returns a representative binary64 value only
  after checking a caller-supplied relative tolerance.

### Capacity minimizers and nearby branches

> Which discrete words attain capacity, or lie within a requested action
> window?

```text
validated input
      |
      +--> exact minimizing discrete words
      |       - sigma
      |       - exact action
      |       - named candidate-family guarantee
      |
      +--> exact action window
              - all covered words within the requested gap
              - exact actions
```

Implemented:

- `CapacityInput4d::qp_minimizers`;
- `QpCandidateFamily4d::{GeneralHk, ProductClosureVertex}`;
- exact `sigma` and action for every returned candidate.

The product family means closure-vertex winners under the product theorem. It
does not claim to parameterize every physical orbit or every point in a
degenerate within-word solution family.

Still missing:

- the exact action-window interface and endpoint semantics.

### One-word KKT analysis

> Given `(a, sigma)`, analyze or solve that KKT problem without knowing why the
> caller selected `sigma`.

This operation is independent of capacity search:

```text
                         sigma enumeration
                                |
                                v
validated a -------> analyze_sigma(a, sigma)
                                |
             +------------------+------------------+
             |                  |                  |
       capacity search     orbit recovery     branch derivative
       and fallback        in primal space    for sys_sigma(a)
```

It must not depend on:

- the total candidate set;
- whether `sigma` came from HK enumeration, a product theorem, or a caller;
- whether the caller is computing capacity, recovering an orbit, or
  differentiating a branch; or
- whether the caller intends to resolve an indeterminate result exactly.

Implemented:

- `CapacityInput4d::solve_sigma_exact`, returning exact `beta`, `q`, `mu`, and
  `xi` for one valid requested word when a strictly positive exact solution
  exists.

Still missing:

- a public certified binary64 one-word analysis;
- ternary predicate results and certified enclosures for useful intermediate
  values;
- derivative and geometric-recovery adapters.

The intended binary64 operation performs no automatic exact fallback.
Capacity search, diagnostics, and branch computations can then make different
fallback decisions without changing the numerical analysis.

An indicative result shape is:

```text
SigmaKktAnalysis4d
    optional certified beta intervals
    optional certified q, mu, and xi intervals
    beta strictly positive: yes / no / indeterminate
    q positive: yes / no / indeterminate
    applicability or indeterminate reason
```

Only mathematically stable predicates belong in this shared result. Search
counts, candidate provenance, capacity-window membership, and timing belong to
their callers or tracing.

For a singular KKT system, these statements remain distinct:

- one selected solution has positive `beta`;
- some solution in the affine solution set has positive `beta`.

A binary64 method that cannot certify the relevant solution-set statement
returns indeterminate. The exact method may then analyze the solution space
and select a positive witness.

## Input truth and validation

Three input meanings must never be conflated.

```text
exact binary64 polytope
    the stored dyadic coordinates define the mathematical object

error-enclosed source polytope
    an intended mathematical object lies within supplied coordinate bounds

uncertified approximation
    useful for exploration but not for trusted numerical labels
```

The implemented API supports the first meaning. Its exact arithmetic treats
each stored binary64 coordinate as the exact dyadic rational it represents.
The certified general predicates include subsequent assembly and arithmetic
roundoff. They do not currently propagate uncertainty between an intended
source polytope and its stored approximation.

`CapacityInput4d::try_from_dual_vertices` currently establishes:

- finite four-dimensional coordinates;
- at most sixteen facets;
- exact binary64-rational polytope validity and incidence;
- origin interior and irredundancy through the exact geometry constructor;
- primal and dual infinity norms in `[1e-3, 1e3]`;
- exact transition signs and structural-product classification; and
- the current general-route candidate resource limit.

The type therefore means “valid and supported as a capacity input,” not merely
“some geometrically valid polytope.” Batch producers validate once, record
rejections, and then use assuming-valid operations.

If a concrete consumer needs source-coordinate uncertainty, add an explicit
enclosure type and propagate that uncertainty through the relevant output.
Do not strengthen the meaning of the existing binary64 result through
documentation alone.

## Outward results and tolerance

An outer consumer normally wants one scalar, while the trusted numerical
contract naturally produces an interval. Expose both without encouraging
unchecked midpoint use:

```text
capacity_certificate(input)
    -> bounds, route/provenance, exact product value when available

capacity_value(input, relative_tolerance)
    -> representative value only if the bounds meet the tolerance
```

For a positive volume interval `[V_lower, V_upper]` and capacity interval
`[c_lower, c_upper]`, outward systolic-ratio composition uses

```text
[c_lower^2 / (2 V_upper), c_upper^2 / (2 V_lower)].
```

The durable layer that owns both volume and capacity should own this
composition. It should not be reimplemented independently in every dataset
producer.

A trusted dataset producer enforces:

```text
every emitted row:
    input validation succeeded
    volume and capacity are finite and positive
    requested relative tolerance is met

otherwise:
    emit or record an explicit rejected row and reason
```

The retained producer audit records at least the maximum relative interval
width, fallback count, rejection count, and worst cases. This turns “the
dataset contains no numerically bad outliers” into a checked producer
invariant rather than social knowledge carried between agents.

## Internal algorithm boundary

General and product routes share validated input and public result contracts,
not their mathematical implementation.

```text
CapacityInput4d
      |
      +--> product.rs
      |       closure vertices, sparse supports, exact resolution
      |
      +--> general.rs
              HK words, short-word exact handling
              certified curvature and cyclic inheritance
              guarded binary64 KKT analysis
              lazy exact resolution
```

An ordinary capacity caller should not need to know about:

- `LBL^T`, LU, or symmetric eigendecomposition;
- inverse-defect enclosures;
- certified curvature obstructions;
- cyclic subword inheritance;
- short-word determinant rejection;
- product closure vertices; or
- exact-fallback staging.

Those are implementation and review concerns. Automatic dispatch belongs
behind the scalar and minimizer interfaces.

Generic matrix factorization and exact linear-system machinery belong under
`crates/symplectic/src/kkt` and `crates/algebraic-numbers`. The mapping from
dual vertices and `sigma` to a four-dimensional orbit/QP interpretation
belongs at the capacity/orbit layer rather than in generic linear algebra.

## Evidence boundary

Runtime dependencies point inward toward production code. Trust evidence
points conceptually toward it but remains outside the production API:

```text
formal numerical and combinatorial arguments
                     |
                     v
           production implementation
                     ^
                     |
 exact controls, numerical audits, adversarial fixtures, profiles, ablations
```

Production:

- `crates/symplectic/src/algorithms/capacity_4d/`;
- focused public API, property, and regression tests;
- ordinary tracing where useful.

Retained experimental evidence:

- `tools/general_algorithm_ablation/`;
- `tools/product_closure_route/`;
- selected readable copies under `src/selected_route/`;
- exact, numerical, adversarial, and performance packets.

Theory:

- `formal/hk2017-qp-precision.tex`;
- `formal/product-qp-six-facet-reduction.tex`.

Ordinary consumers do not import the experiment crate. A production maintainer
uses correspondence tests to check that a deliberate algorithm edit is
reflected in the readable implementation and retained evidence.

## Consumer reading surfaces

The current inventory suggests these approximate future reading burdens. They
count independent task contexts, not simultaneously active agents.

### Systolic-ratio and scalar-capacity consumers

Roughly ten to twenty consumer tasks should need only:

- one validated-input example;
- the scalar tolerance/error contract;
- rejected-input handling; and
- result provenance when writing retained rows.

They should not read the capacity algorithms. Examples include datascience
producers, ranking experiments, known-value checks, random/product surveys,
and thesis-facing scalar computations.

### Minimizer consumers

Roughly four to eight tasks additionally need:

- the returned candidate-family meaning;
- tie semantics;
- exact action access; and
- deterministic selection when only one returned word is needed.

Examples include bounce-count producers, all-minimum verification, and
selected orbit construction.

### Window, derivative, and recovery consumers

Roughly three to six tasks additionally need:

- exact action-window semantics;
- context-free one-word analysis;
- exact one-word resolution;
- derivative or geometric-recovery contracts; and
- singular-solution semantics where relevant.

Examples include ascent, branch switching, local orbit geometry, and
near-active diagnostics.

### Algorithm maintainers and reviewers

Approximately one to three QP-maintenance tasks should need the production
general/product implementations. Independent mathematical, numerical, or
performance reviews additionally read the relevant formal statement and
retained evidence packet.

Old full-branch and heuristic orbit searches remain readable experiment
controls only where the branch landscape or old numerical behavior is the
subject. Their result types are not the ordinary capacity interface.

## Current implementation map

```text
crates/symplectic/src/algorithms/capacity_4d/mod.rs
    public capacity, minimizer, and exact-one-sigma facade

crates/symplectic/src/algorithms/capacity_4d/input.rs
    validation, exact transition data, product classification

crates/symplectic/src/algorithms/capacity_4d/general.rs
    selected general implementation

crates/symplectic/src/algorithms/capacity_4d/product.rs
    selected structural-product implementation

crates/symplectic/src/exact/orbit.rs
    context-free exact one-word KKT solving

crates/symplectic/tests/public_capacity_api.rs
    caller-shaped contract examples and regressions

experiments/dev-quadratic-program/tools/general_algorithm_ablation/
    general-route verification, numerics, ablation, and profiling

experiments/dev-quadratic-program/tools/product_closure_route/
    product-route exact, adversarial, and correspondence evidence
```

## Migration consequences

Before broad consumer migration:

1. expose a tolerance-checked scalar convenience operation;
2. expose context-free certified binary64 one-word analysis;
3. implement the smallest exact action-window contract required by current
   consumers;
4. complete derivative/recovery adapters needed by ordinary branch consumers;
5. run the retained correctness and numerical evidence gates; and
6. freeze the public contracts before handing the API map to the active
   optimizer worktree.

Consumer migration proceeds by mathematical output:

- scalar capacity and systolic ratio;
- exact minimizing words;
- exact action windows;
- one-word KKT payloads;
- branch derivatives or recovered geometry; and
- intentionally retained heuristic diagnostics.

A source file that uses several outputs may partly migrate while retaining an
explicitly named experiment-local diagnostic. Successful compilation with an
unchanged legacy call is not evidence that its mathematical output was
migrated.

The thesis-facing description is written from these outer mathematical
questions inward. Detailed solver internals are included only to the extent
needed to justify correctness, numerical reliability, and the measured
algorithmic improvement.
