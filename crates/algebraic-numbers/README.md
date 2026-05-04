# algebraic-numbers

Small companion crate for arithmetic and a few tiny linear-algebra routines over
ordered real algebraic extensions of `Q`.

The main API shape is:

- `Rational = BigRational`
- `OrderedField`
- `StaticFieldSpec`
- `Algebraic<S>`

Named fields are defined by a marker/spec type:

```rust
use num_rational::BigRational;
use algebraic_numbers::{Algebraic, OrderedField, StaticFieldSpec};

struct SqrtTwo;

impl StaticFieldSpec for SqrtTwo {
    fn name() -> &'static str { "Q(sqrt(2))" }
    fn generator_name() -> &'static str { "s" }
    fn minimal_polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-2).into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(1.into()),
        ]
    }
    fn isolating_interval() -> (BigRational, BigRational) {
        (
            BigRational::from_integer(1.into()),
            BigRational::from_integer(2.into()),
        )
    }
}

type SqrtTwoField = Algebraic<SqrtTwo>;

let s = SqrtTwoField::generator();
let value = (SqrtTwoField::one() + s.clone()) * (SqrtTwoField::one() + s.clone());
assert_eq!(
    value,
    SqrtTwoField::from_i64(3) + SqrtTwoField::from_i64(2) * s
);
```

The crate also ships one named field used by the HKO pentagon case:

- `TanPiFifth`

with:

```rust
type TanPiFifthField = algebraic_numbers::Algebraic<algebraic_numbers::TanPiFifth>;
```

Included helpers:

- `cmp_field`, `min_field`, `max_field`
- `dot`
- `solve_square`
- `rank_rows`
- `canonical_element`

## Trust Boundary

This crate is a durable exact-arithmetic dependency, not a temporary experiment
helper. Public callers already exercise it through `tests/api_consumption.rs`,
and exact experiment artifacts use its canonical serialization.

Current public surface:

- scalar API: `Algebraic<S>`, `OrderedField`, `Rational`, `StaticFieldSpec`
- named field: `TanPiFifth`
- linear algebra: `solve_square`, `rank_rows`, `SolveResult`
- ordering/sign: `Sign`, `cmp_field`, `min_field`, `max_field`
- persistence: `canonical_element`, `CanonicalElement`
- validation: `validate_field_spec`, `FieldSpecError`

`validate_field_spec` is the ordinary error-returning field-spec checker.
`Algebraic<S>` constructors use an internal cached assertion path and panic if a
static field spec is invalid or if the validation cache is poisoned. That is an
invariant failure for compile-time field definitions, not a recoverable
runtime-data error.

## Serialization Contract

`canonical_element` is the stable row format for one scalar value. It stores:

- the field name,
- the basis labels,
- the canonical coefficient vector,
- exact rational coefficients as numerator/denominator strings.

Do not replace this with debug formatting or `f64` output in exact artifacts.
Schema changes need a migration plan for existing JSONL consumers.

## Formal Status

The implementation has focused tests for field laws, sign classification,
canonicalization, serialization, and tiny linear algebra. Some proof links are
still intentionally local TODOs because the corresponding formal write-up does
not yet exist:

- field-spec validation: minimal-polynomial plus isolating-interval contract
- Sturm root counting and Sturm sequence sign-variation arguments
- Bezout/invertibility for nonzero field elements
- sign determination by interval refinement
- uniqueness and approximation of the selected real root

Treat these as formal-reference gaps, not as permission to weaken the exact
arithmetic contract. Route mathematical approval or proof text through
`formal/` and the relevant task bundle before changing the public semantics.

Tests:

- unit tests for field laws, sign, canonicalization, serialization, and tiny
  linear algebra
- consumer-style API tests
- smoke tests

Benches:

- scalar add/sub/mul/div
- sign
- dot products
- tiny linear solves
