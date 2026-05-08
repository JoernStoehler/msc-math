# algebraic-numbers spec

This file is the normative crate contract. It has two parts:

- a feature contract: what the crate must provide;
- a semantic contract: what must stay true for those features to mean the
  right thing.

Executable checks live in `ACCEPTANCE.md`. Rationale and rejected alternatives
live in `DESIGN_NOTES.md`.

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
