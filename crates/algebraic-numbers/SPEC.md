# algebraic-numbers spec

This file is the crate spec. It has two normative parts:

- a feature contract: what the crate must provide;
- a semantic contract: what must stay true for those features to mean the
  right thing.

The evidence checks at the end are not the spec. They are executable witnesses
for the current spec. Each item should add distinct information; duplicated
requirements make this a worse specification.

## Purpose

`algebraic-numbers` provides exact scalar types for computations in one
statically specified real algebraic field `Q[alpha]` at a time.

## Feature Contract

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
caller or a short `DESIGN_NOTES.md` entry explaining the scope change.

## Semantic Contract

The feature contract is not satisfied if these facts stop being true:

- `ExactScalar` is explicit opt-in. Do not add a blanket impl.
- `f64` does not implement `ExactScalar`.
- `RealAlgebraicField` remains trusted static data, not a runtime constructor.
- `Algebraic<F>` values from different fields cannot be mixed by ordinary
  arithmetic operators.
- Equality is coefficient equality in the fixed basis, relying on the minimal
  polynomial contract.
- Ordering and sign are exact; they must not depend on floating-point
  approximations or caller-provided tolerances.

## Evidence Checks

These checks are sufficient smoke evidence for the current feature and semantic
contracts. They do not replace review of the contracts above.

Run:

```bash
cargo test -p algebraic-numbers
cargo clippy -p algebraic-numbers --all-targets -- -D warnings
cargo run -p algebraic-numbers --example q_sqrt5_vector
```

The `q_sqrt5_*` tests and example must cover exactly the current public
ergonomics target:

- `Vector4<Qsqrt5> + Vector4<Qsqrt5>`;
- `alpha * alpha == 5` in `Q[sqrt(5)]`;
- `2 * alpha`, `alpha * BigRational`, and `BigRational * alpha`;
- exact sign and ordering around `2 < sqrt(5) < 3`;
- division by a nonzero `Algebraic<F>`.
