# algebraic-numbers design notes

This file records only non-obvious design choices. Keep future entries short:
compare the accepted approach with the closest rejected approach, then give the
reason. Use labels such as `tried`, `predicted`, `ergonomics`, `complexity`,
`scope`, and `semantics` only when they add information.

## Accepted Shape

Use a statically chosen real field marker `F` and store elements as rational
coefficient vectors in the basis `1, alpha, ..., alpha^(degree - 1)`.

Reasons: thesis code currently needs fixed real fields, not a general
computer-algebra parent hierarchy. `type Qsqrt5 = Algebraic<Sqrt5>` works
cleanly with `nalgebra::Vector4<Qsqrt5>`. Arithmetic stays local: coefficient
operations plus reduction modulo the monic defining polynomial.

## Rejected Or Deferred Approaches

### Runtime Parent Objects

Rejected for now.

Reasons: we do not need Sage-style runtime field construction. Every element
would need to carry or reference parent data, which makes ownership, equality,
and serialization less predictable. Static field markers prevent accidental
mixing of incompatible fields at compile time.

### A Single Dynamic `Algebraic` Enum

Rejected for now.

Reasons: every operation would need dynamic field compatibility checks. Values
in one known field become less explicit at call sites. This crate is for fixed
real fields, not arbitrary algebraic number normalization across fields.

### Associated Const Degree With Fixed-Size Arrays

Deferred.

Reasons: stable Rust makes `coeffs: [BigRational; F::DEGREE]` awkward because
associated consts from type parameters cannot be used freely as array lengths.
The runtime length check is cheap relative to `BigRational` arithmetic and
polynomial operations. `Algebraic<F>` is simpler than
`Algebraic<F, const N: usize>`.

Breadcrumb: this can be revisited if profiling shows coefficient-vector
allocation matters.

### Blanket `impl ExactScalar for T where ...`

Rejected.

Reasons: exactness is a semantic promise, not just a set of operators. Explicit
impls make it obvious why `BigRational` and `Algebraic<F>` are accepted while
`f64` is not.

### `f64` as an Exact Scalar

Rejected.

Reasons: equality and ordering for rounded values do not provide the exact
decision guarantees expected by exact algorithms. Approximate numeric code
should use a separate API with explicit tolerances or error bounds.

### Implementing nalgebra `RealField` or `ComplexField`

Rejected.

Reasons: nalgebra's stronger numeric traits are shaped around approximate
floating-point functionality such as epsilons, ulps, and transcendental
functions. We only need nalgebra containers and ordinary arithmetic syntax.

### Float Sign Checks for Ordering

Rejected.

Reasons: evaluating at an `f64` approximation of `alpha` loses exactness.
Near-zero cases would require caller-chosen tolerances, which is exactly what
this crate avoids.

Accepted alternative: refine the rational isolating interval until interval
evaluation determines the sign.

### One-Shot Interval Evaluation for Sign

Rejected.

Reasons: the implementation needs a refinement loop because the initial
interval can contain both signs for the represented polynomial. Returning
indeterminate would leak interval-analysis concerns into an exact ordered
scalar API.

### Matrix Solve, Diagonalization, and Eigen APIs

Deferred.

Reasons: no immediate caller. Diagonalization over `Q[alpha]` may require
explicit field extensions, which this crate intentionally does not construct.
Exact scalar behavior should settle before adding linear algebra algorithms.
