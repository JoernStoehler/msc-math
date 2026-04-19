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
