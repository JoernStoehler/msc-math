# algebraic-numbers design notes

This file records short approach comparisons for decisions that are easy to
forget and expensive to rediscover. Keep entries brief, but include the reason
type when it is not obvious from the code.

Reason labels:

- `tried`: a local implementation or usage example was tried.
- `predicted`: rejected by engineering judgment before implementation.
- `ergonomics`: example use was awkward or unclear.
- `complexity`: implementation or maintenance cost was too high.
- `scope`: outside the current exact real scalar API.
- `semantics`: the API would imply a mathematical contract we cannot honestly
  provide.

## Accepted Shape

Use a statically chosen real field marker `F` and store elements as rational
coefficient vectors in the basis `1, alpha, ..., alpha^(degree - 1)`.

Reasons:

- `scope`: thesis code currently needs fixed real fields, not a general
  computer-algebra parent hierarchy.
- `ergonomics`: `type Qsqrt5 = Algebraic<Sqrt5>` works cleanly with
  `nalgebra::Vector4<Qsqrt5>`.
- `complexity`: arithmetic stays local: coefficient operations plus reduction
  modulo the monic defining polynomial.

## Rejected Or Deferred Approaches

### Runtime Parent Objects

Rejected for now.

Reasons:

- `scope`: we do not need Sage-style runtime field construction.
- `complexity`: every element would need to carry or reference parent data,
  which makes ownership, equality, and serialization less predictable.
- `ergonomics`: static field markers prevent accidental mixing of incompatible
  fields at compile time.

### A Single Dynamic `Algebraic` Enum

Rejected for now.

Reasons:

- `complexity`: every operation would need dynamic field compatibility checks.
- `ergonomics`: values in one known field become less explicit at call sites.
- `scope`: this crate is for fixed real fields, not arbitrary algebraic number
  normalization across fields.

### Associated Const Degree With Fixed-Size Arrays

Deferred.

Reasons:

- `complexity`: stable Rust makes `coeffs: [BigRational; F::DEGREE]`
  awkward because associated consts from type parameters cannot be used freely
  as array lengths.
- `predicted`: the runtime length check is cheap relative to `BigRational`
  arithmetic and polynomial operations.
- `ergonomics`: `Algebraic<F>` is simpler than `Algebraic<F, const N: usize>`.

Breadcrumb: this can be revisited if profiling shows coefficient-vector
allocation matters.

### Blanket `impl ExactScalar for T where ...`

Rejected.

Reasons:

- `semantics`: exactness is a semantic promise, not just a set of operators.
- `ergonomics`: explicit impls make it obvious why `BigRational` and
  `Algebraic<F>` are accepted while `f64` is not.

### `f64` as an Exact Scalar

Rejected.

Reasons:

- `semantics`: equality and ordering for rounded values do not provide the
  exact decision guarantees expected by exact algorithms.
- `scope`: approximate numeric code should use a separate API with explicit
  tolerances or error bounds.

### Implementing nalgebra `RealField` or `ComplexField`

Rejected.

Reasons:

- `semantics`: nalgebra's stronger numeric traits are shaped around approximate
  floating-point functionality such as epsilons, ulps, and transcendental
  functions.
- `scope`: we only need nalgebra containers and ordinary arithmetic syntax.

### Float Sign Checks for Ordering

Rejected.

Reasons:

- `semantics`: evaluating at an `f64` approximation of `alpha` loses exactness.
- `predicted`: near-zero cases would require caller-chosen tolerances, which is
  exactly what this crate avoids.

Accepted alternative: refine the rational isolating interval until interval
evaluation determines the sign.

### One-Shot Interval Evaluation for Sign

Rejected.

Reasons:

- `tried`: the implementation needs a refinement loop because the initial
  interval can contain both signs for the represented polynomial.
- `semantics`: returning indeterminate would leak interval-analysis concerns
  into an exact ordered scalar API.

### Matrix Solve, Diagonalization, and Eigen APIs

Deferred.

Reasons:

- `scope`: no immediate caller.
- `complexity`: diagonalization over `Q[alpha]` may require explicit field
  extensions, which this crate intentionally does not construct.
- `maintainability`: exact scalar behavior should settle before adding linear
  algebra algorithms.
