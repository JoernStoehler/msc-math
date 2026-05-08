# algebraic-numbers development

This file is for maintainers. The README is for consumers.

## Instrumental Objective

Normative objective: define the small exact scalar API this branch keeps for
statically chosen real algebraic fields. Required properties: field choices are
explicit, equality/order decisions are exact, and ordinary Rust/nalgebra syntax
works for small vectors.

Scope boundary: this crate is not a general computer-algebra system. The
evidence target is that the current code and tests let a reviewer check that
exact code paths do not inherit floating-point tolerance choices.

This branch intentionally contracts the older workspace API. Downstream callers
of removed items such as `OrderedField`, `TanPiFifth`, `cmp_field`, and
`canonical_element` are migration/removal work outside this crate-local API
slice, not evidence that this crate should restore those surfaces.

That backchains to three local objectives:

- exact scalar semantics are explicit and reviewable;
- the public API remains close to the capability list in the spec below;
- adding broader algebraic functionality requires a current caller and a local
  rationale in this file.

## Spec

Normative claim: the crate is complete for the current crate-local target if and
only if it provides exactly these capabilities:

1. `ExactScalar`, an explicit opt-in trait for exact scalar values.
2. `RealAlgebraicField`, a static field specification with:
   - a monic minimal polynomial over `Q`;
   - a rational isolating interval for the chosen real root.
3. `Algebraic<F>`, an element of that field represented by rational
   coefficients in the basis `1, alpha, ..., alpha^(degree - 1)`.
4. Exact scalar operations on `Algebraic<F>`:
   - equality and ordering;
   - zero and one;
   - negation, addition, subtraction, multiplication, division;
   - assignment variants for addition, subtraction, multiplication, and
     division;
   - conversion from `i64` and `BigRational`.
5. Ordinary nalgebra container ergonomics, demonstrated by
   `Vector4<Algebraic<Sqrt5>>`.

The crate must not add capabilities outside that list unless there is a current
caller or a short note below explaining the scope change.

## Architecture

The public model has three pieces:

- `ExactScalar`: explicit opt-in marker for exact scalar values.
- `RealAlgebraicField`: static field data for one chosen real root `alpha`.
- `Algebraic<F>`: rational coefficients in the fixed basis
  `1, alpha, ..., alpha^(degree - 1)`.

Implementation files are split by responsibility:

- `algebraic_element.rs`: storage, constructors, equality, and ordering shell.
- `arithmetic_ops.rs`: Rust operator impls and rational conversions.
- `field_specification.rs`: field marker contract and chosen-root endpoints.
- `polynomial_arithmetic.rs`: polynomial reduction and inversion modulo the
  field polynomial.
- `sign_ordering.rs`: exact ordering decisions by rational interval refinement.
- `exact_scalar.rs`: explicit exact scalar trait and impls.

Design intent: this structure is predictable rather than abstract. The expected
maintenance path is to start with the file matching the behavior to change:
arithmetic, ordering, or field contracts.

## Semantic Guardrails

The contract is not satisfied if these facts stop being true:

- `ExactScalar` is explicit opt-in. Do not add a blanket impl.
- `f64` does not implement `ExactScalar`.
- `RealAlgebraicField` remains trusted static data, not a runtime constructor.
- `Algebraic<F>` values from different fields cannot be mixed by ordinary
  arithmetic operators.
- Equality is coefficient equality in the fixed basis, relying on the minimal
  polynomial contract.
- Ordering is exact; it must not depend on floating-point approximations or
  caller-provided tolerances.

## Verification

These commands verify the crate-local API slice. They do not verify workspace
callers that still import the removed older API.

Run:

```bash
cargo test -p algebraic-numbers
cargo clippy -p algebraic-numbers --all-targets -- -D warnings
cargo run -p algebraic-numbers --example q_sqrt5_vector
```

The `q_sqrt5_*` tests and example witness the current ergonomics target:
`Vector4<Qsqrt5> + Vector4<Qsqrt5>`, `root * root == 5`, conversion from
`i64` and `BigRational`, `zero()`, `one()`, assignment operators, exact
ordering around `2 < sqrt(5) < 3`, and division by a nonzero algebraic value.
They also run a fixed grid of Q[sqrt(5)] field-law checks and ordering checks
against rational interval witnesses.

The `kiss_api_style` test is a deliberately small regression guard for repeated
review feedback. It bans exact strings that previously made the public API or
consumer examples less KISS.

## Rejected Or Deferred Approaches

Keep future entries short: compare the accepted approach with the closest
rejected approach, then give the reason. Use labels such as `tried`,
`predicted`, `ergonomics`, `complexity`, `scope`, and `semantics` only when
they add information.

### Runtime Parent Objects

Rejected for now. Current evidence: no current test, example, or thesis-facing
consumer in this crate needs Sage-style runtime field construction. Predicted
cost: every element would need to carry or reference parent data, which makes
ownership, equality, and serialization less predictable. Static field markers
prevent ordinary arithmetic from mixing incompatible fields at compile time.

### A Single Dynamic `Algebraic` Enum

Rejected for now. Predicted cost: every operation would need dynamic field
compatibility checks. Ergonomics evidence: the current Qsqrt5 example is
explicit with `type Qsqrt5 = Algebraic<Sqrt5>`. Current scope is fixed real
fields, not arbitrary algebraic number normalization across fields.

### Fixed-Size Coefficient Arrays

Deferred. Rust limitation: stable Rust does not allow a straightforward
`[BigRational; field_degree::<F>()]` storage shape. Predicted cost: the runtime
length check is cheap relative to `BigRational` arithmetic and polynomial
operations. Ergonomics judgment: `Algebraic<F>` is simpler than
`Algebraic<F, const N: usize>` for current callers.

Breadcrumb: this can be revisited if profiling shows coefficient-vector
allocation matters.

### Blanket `impl ExactScalar for T where ...`

Rejected. Semantic reason: exactness is a promise, not just a set of operators.
Explicit impls make it locally checkable why `BigRational` and `Algebraic<F>`
are accepted while `f64` is not.

### Scalar Operator Mixing

Rejected for now. Ergonomics reason: `Qsqrt5::from(2) * Qsqrt5::root()` is
longer than `2 * root`, but the explicit conversion keeps arithmetic uniformly
inside the field type and avoids adding a matrix of integer/rational operator
impls.

### `f64` as an Exact Scalar

Rejected. Semantic reason: equality and ordering for rounded values do not
provide the exact decision guarantees expected by exact algorithms. Project
policy: approximate numeric code belongs in a separate API with explicit
tolerances or error bounds.

### Implementing nalgebra `RealField` or `ComplexField`

Rejected. API reason: nalgebra's stronger numeric traits include approximate
floating-point functionality such as epsilons, ulps, and transcendental
functions. Current need: nalgebra containers and ordinary arithmetic syntax.

### Float Evaluation For Ordering

Rejected. Semantic reason: evaluating at an `f64` approximation of `alpha`
loses exactness. Near-zero cases would require caller-chosen tolerances, which
is exactly what this crate avoids.

Accepted alternative: refine the rational isolating interval until interval
evaluation determines the ordering.

### One-Shot Interval Evaluation For Ordering

Rejected. Implementation evidence: the current ordering code uses a refinement
loop because the initial interval can contain both signs for the represented
polynomial. Returning indeterminate would leak interval-analysis concerns into
an exact ordered scalar API.

### Matrix Solve, Diagonalization, and Eigen APIs

Deferred. Current evidence: no immediate caller in this crate. Mathematical
risk: diagonalization over `Q[alpha]` may require explicit field extensions,
which this crate intentionally does not construct. Sequencing judgment: settle
exact scalar behavior before adding linear algebra algorithms.
