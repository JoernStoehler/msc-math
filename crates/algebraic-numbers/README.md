# algebraic-numbers

Exact scalar crate for real algebraic numbers in a statically chosen field
`Q[alpha]`.

Use this crate when a computation needs exact scalar arithmetic and exact
ordering in one known real algebraic field. Do not use it as a general
computer-algebra system.

## Public API

- `ExactScalar`: explicit opt-in trait for exact scalar types.
- `RealAlgebraicField`: static field specification for one chosen real root.
- `Algebraic<F>`: element of `Q[alpha]` for the field marker `F`.

`BigRational` and `Algebraic<F>` implement `ExactScalar`.

## Defining A Field

Define a zero-sized marker and implement `RealAlgebraicField`.

```rust
use algebraic_numbers::{Algebraic, RealAlgebraicField};
use num_rational::BigRational;

enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        // Low-to-high coefficients for t^2 - 5.
        vec![q(-5), q(0), q(1)]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        // Select the positive root sqrt(5).
        (q(2), q(3))
    }
}

type Qsqrt5 = Algebraic<Sqrt5>;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}
```

Field contract:

- `polynomial()` returns coefficients in low-to-high order.
- The polynomial must be the monic minimal polynomial of `alpha`.
- `isolating_interval()` returns `(lower, upper)` and must contain exactly the
  chosen real root.
- Interval endpoints must be rational and must not be roots.

For `sqrt(5)`, the polynomial is `x^2 - 5`, represented as `[-5, 0, 1]`, and
the isolating interval `(2, 3)` selects the positive root.

## Constructing Values

Use `From<i64>` for rational integers and `root()` for the selected real root.
Use `new([..])` only when coefficient-basis construction is clearer than
ordinary arithmetic.

```rust
let one_plus_two_sqrt5 = Qsqrt5::from(1) + Qsqrt5::from(2) * Qsqrt5::root();
let same_value = Qsqrt5::new([q(1), q(2)]);

assert_eq!(one_plus_two_sqrt5, same_value);
assert_eq!(Qsqrt5::from(3), Qsqrt5::new([q(3), q(0)]));
```

Wrong coefficient-array length panics. It is a constructor bug, not a
runtime case callers are expected to recover from.

## Arithmetic And Order

Supported operations on `Algebraic<F>`:

- `==`, `<`, `>`, `Ord`;
- `Zero`, `One`;
- unary `-`;
- `+`, `-`, `*`, `/`;
- assignment variants such as `+=`, `*=`;
- conversion from `i64` and `BigRational`.

Examples:

```rust
assert_eq!(Qsqrt5::root() * Qsqrt5::root(), Qsqrt5::from(5));
assert_eq!(
    Qsqrt5::from(2) * Qsqrt5::root(),
    Qsqrt5::new([q(0), q(2)])
);
assert_eq!(
    Qsqrt5::root() * Qsqrt5::from(q(3)),
    Qsqrt5::new([q(0), q(3)])
);
assert_eq!(
    Qsqrt5::from(q(3)) * Qsqrt5::root(),
    Qsqrt5::new([q(0), q(3)])
);
assert!(Qsqrt5::root() - Qsqrt5::from(2) > Qsqrt5::from(0));
assert!(Qsqrt5::root() > Qsqrt5::from(2));
```

Division by zero panics, matching the current operator API.

## nalgebra Containers

`Algebraic<F>` is intended to work as a nalgebra scalar for ordinary container
syntax.

```rust
use nalgebra::Vector4;

let left = Vector4::new(
    Qsqrt5::root(),
    Qsqrt5::from(1) + Qsqrt5::root(),
    Qsqrt5::from(2),
    -Qsqrt5::root(),
);
let right = Vector4::new(
    Qsqrt5::from(2) * Qsqrt5::root(),
    Qsqrt5::from(4) - Qsqrt5::root(),
    Qsqrt5::from(3) * Qsqrt5::root(),
    Qsqrt5::from(5) + Qsqrt5::root(),
);

assert_eq!(
    left + right,
    Vector4::new(
        Qsqrt5::from(3) * Qsqrt5::root(),
        Qsqrt5::from(5),
        Qsqrt5::from(2) + Qsqrt5::from(3) * Qsqrt5::root(),
        Qsqrt5::from(5),
    )
);
```

## Not Provided

- no runtime parent/ring objects;
- no automatic construction of larger fields;
- no `f64` exact scalar implementation;
- no nalgebra `RealField`/`ComplexField` implementation;
- no matrix solve, eigenvalue, or diagonalization layer.
