# algebraic-numbers development

This file is for maintainers. The README is for consumers.

## Contract

The crate is complete for the current thesis use if and only if it provides
exactly these capabilities:

1. `ExactScalar`, an explicit opt-in trait for exact scalar values.
2. `RealAlgebraicField`, a static field specification with:
   - a degree;
   - a monic minimal polynomial over `Q`;
   - a rational isolating interval for the chosen real root.
3. `Algebraic<F>`, an element of that field represented by rational
   coefficients in the basis `1, alpha, ..., alpha^(degree - 1)`.
4. Exact scalar operations on `Algebraic<F>`:
   - equality and ordering;
   - sign;
   - zero and one;
   - negation, addition, subtraction, multiplication, division;
   - convenient interaction with `i64` and `BigRational`.
5. Ordinary nalgebra container ergonomics, demonstrated by
   `Vector4<Algebraic<Sqrt5>>`.

The crate must not add capabilities outside that list unless there is a current
caller or a short note below explaining the scope change.

## Semantic Guardrails

The contract is not satisfied if these facts stop being true:

- `ExactScalar` is explicit opt-in. Do not add a blanket impl.
- `f64` does not implement `ExactScalar`.
- `RealAlgebraicField` remains trusted static data, not a runtime constructor.
- `Algebraic<F>` values from different fields cannot be mixed by ordinary
  arithmetic operators.
- Equality is coefficient equality in the fixed basis, relying on the minimal
  polynomial contract.
- Ordering and sign are exact; they must not depend on floating-point
  approximations or caller-provided tolerances.

## Verification

Run:

```bash
cargo test -p algebraic-numbers
cargo clippy -p algebraic-numbers --all-targets -- -D warnings
cargo run -p algebraic-numbers --example q_sqrt5_vector
```

The `q_sqrt5_*` tests and example should witness the current ergonomics target:
`Vector4<Qsqrt5> + Vector4<Qsqrt5>`, `alpha * alpha == 5`, scalar mixing with
`i64` and `BigRational`, exact sign/order around `2 < sqrt(5) < 3`, and
division by a nonzero algebraic value.

## Rejected Or Deferred Approaches

Keep future entries short: compare the accepted approach with the closest
rejected approach, then give the reason. Use labels such as `tried`,
`predicted`, `ergonomics`, `complexity`, `scope`, and `semantics` only when
they add information.

### Runtime Parent Objects

Rejected for now. We do not need Sage-style runtime field construction. Every
element would need to carry or reference parent data, which makes ownership,
equality, and serialization less predictable. Static field markers prevent
accidental mixing of incompatible fields at compile time.

### A Single Dynamic `Algebraic` Enum

Rejected for now. Every operation would need dynamic field compatibility
checks. Values in one known field become less explicit at call sites. This
crate is for fixed real fields, not arbitrary algebraic number normalization
across fields.

### Associated Const Degree With Fixed-Size Arrays

Deferred. Stable Rust makes `coeffs: [BigRational; F::DEGREE]` awkward because
associated consts from type parameters cannot be used freely as array lengths.
The runtime length check is cheap relative to `BigRational` arithmetic and
polynomial operations. `Algebraic<F>` is simpler than
`Algebraic<F, const N: usize>`.

Breadcrumb: this can be revisited if profiling shows coefficient-vector
allocation matters.

### Blanket `impl ExactScalar for T where ...`

Rejected. Exactness is a semantic promise, not just a set of operators.
Explicit impls make it obvious why `BigRational` and `Algebraic<F>` are
accepted while `f64` is not.

### `f64` as an Exact Scalar

Rejected. Equality and ordering for rounded values do not provide the exact
decision guarantees expected by exact algorithms. Approximate numeric code
should use a separate API with explicit tolerances or error bounds.

### Implementing nalgebra `RealField` or `ComplexField`

Rejected. nalgebra's stronger numeric traits are shaped around approximate
floating-point functionality such as epsilons, ulps, and transcendental
functions. We only need nalgebra containers and ordinary arithmetic syntax.

### Float Sign Checks for Ordering

Rejected. Evaluating at an `f64` approximation of `alpha` loses exactness.
Near-zero cases would require caller-chosen tolerances, which is exactly what
this crate avoids.

Accepted alternative: refine the rational isolating interval until interval
evaluation determines the sign.

### One-Shot Interval Evaluation for Sign

Rejected. The implementation needs a refinement loop because the initial
interval can contain both signs for the represented polynomial. Returning
indeterminate would leak interval-analysis concerns into an exact ordered
scalar API.

### Matrix Solve, Diagonalization, and Eigen APIs

Deferred. No immediate caller. Diagonalization over `Q[alpha]` may require
explicit field extensions, which this crate intentionally does not construct.
Exact scalar behavior should settle before adding linear algebra algorithms.
