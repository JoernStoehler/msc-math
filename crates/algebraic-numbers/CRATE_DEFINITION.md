# algebraic-numbers crate definition

This crate is in scope if and only if each item below stays true.

## Definition

`algebraic-numbers` provides exact scalar types for computations in one
statically specified real algebraic field `Q[alpha]` at a time.

The crate is complete for the current thesis use when it provides:

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

Anything else is out of scope until a concrete caller needs it.

## Non-Goals

The crate must not become:

- a runtime computer-algebra parent/ring system;
- a system for automatically constructing larger fields;
- an approximate numeric layer;
- an `f64` exact-scalar adapter;
- a nalgebra `RealField`/`ComplexField` implementation;
- a matrix solve, eigenvalue, or diagonalization crate.

## Actionable Acceptance Checks

A change preserves the crate definition when all applicable checks pass:

```bash
cargo test -p algebraic-numbers
cargo clippy -p algebraic-numbers --all-targets -- -D warnings
cargo run -p algebraic-numbers --example q_sqrt5_vector
```

The test/example surface must continue to show:

- `Vector4<Qsqrt5> + Vector4<Qsqrt5>`;
- `alpha * alpha == 5` in `Q[sqrt(5)]`;
- `2 * alpha`, `alpha * BigRational`, and `BigRational * alpha`;
- exact sign and ordering around `2 < sqrt(5) < 3`;
- division by a nonzero `Algebraic<F>`.

## Guardrails Against Gameable Compliance

A change is not acceptable merely because the commands pass. It must also keep
these semantic facts true:

- `ExactScalar` is explicit opt-in. Do not add a blanket impl.
- `f64` does not implement `ExactScalar`.
- `RealAlgebraicField` remains trusted static data, not a runtime constructor.
- `Algebraic<F>` values from different fields cannot be mixed by ordinary
  arithmetic operators.
- Equality is coefficient equality in the fixed basis, relying on the minimal
  polynomial contract.
- Ordering and sign are exact; they must not depend on floating-point
  approximations or caller-provided tolerances.
- Adding a new capability requires either a current caller or a short entry in
  `DESIGN_NOTES.md` explaining why the scope changed.
