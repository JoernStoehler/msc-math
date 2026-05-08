# algebraic-numbers

Exact scalar crate for real algebraic numbers in a statically chosen field
`Q[alpha]`.

Use this crate when a computation needs exact scalar arithmetic and exact
ordering in one known real algebraic field. Do not use it as a general
computer-algebra system.

## Public API

```rust
use algebraic_numbers::{
    Algebraic,
    ExactScalar,
    RationalInterval,
    RealAlgebraicField,
    Sign,
};
```

- `ExactScalar`: explicit opt-in trait for exact scalar types.
- `RealAlgebraicField`: static field specification for one chosen real root.
- `Algebraic<F>`: element of `Q[alpha]` for the field marker `F`.
- `RationalInterval`: rational isolating interval for the chosen root.
- `Sign`: exact sign result, one of `Negative`, `Zero`, `Positive`.

`BigRational` and `Algebraic<F>` implement `ExactScalar`.

## Defining A Field

Define a zero-sized marker and implement `RealAlgebraicField`.

```rust
use algebraic_numbers::{Algebraic, RationalInterval, RealAlgebraicField};
use num_rational::BigRational;
use num_traits::{One, Zero};

enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-5).into()),
            BigRational::zero(),
            BigRational::one(),
        ]
    }

    fn isolating_interval() -> RationalInterval {
        RationalInterval::new(q(2), q(3))
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
- `isolating_interval()` must contain exactly the chosen real root.
- Interval endpoints must be rational and must not be roots.

For `sqrt(5)`, the polynomial is `x^2 - 5`, represented as `[-5, 0, 1]`, and
the isolating interval `(2, 3)` selects the positive root.

## Constructing Values

`Algebraic<F>::new` takes coefficients in the basis
`1, alpha, ..., alpha^(degree - 1)`.

```rust
fn a(rational: i64, sqrt5_coeff: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![q(rational), q(sqrt5_coeff)]).unwrap()
}

let alpha = Qsqrt5::alpha();
let one_plus_two_sqrt5 = a(1, 2);
let rational = Qsqrt5::from(3);
```

Invalid coefficient-vector length returns `BadDegree`.

## Arithmetic And Order

Supported operations on `Algebraic<F>`:

- `==`, `<`, `>`, `Ord`;
- `sign() -> Sign`;
- `Zero`, `One`;
- unary `-`;
- `+`, `-`, `*`, `/`;
- assignment variants such as `+=`, `*=`;
- scalar mixing with `i64` and `BigRational`.

Examples:

```rust
let alpha = Qsqrt5::alpha();

assert_eq!(alpha.clone() * alpha.clone(), a(5, 0));
assert_eq!(2 * alpha.clone(), a(0, 2));
assert_eq!(alpha.clone() * q(3), a(0, 3));
assert_eq!(q(3) * alpha.clone(), a(0, 3));
assert_eq!((alpha.clone() - 2).sign(), Sign::Positive);
assert!(alpha > a(2, 0));
```

Division by zero panics, matching the current operator API.

## nalgebra Containers

`Algebraic<F>` is intended to work as a nalgebra scalar for ordinary container
syntax.

```rust
use nalgebra::Vector4;

let alpha = Qsqrt5::alpha();
let left = Vector4::new(alpha.clone(), a(1, 1), a(2, 0), a(0, -1));
let right = Vector4::new(2 * alpha, a(4, -1), a(0, 3), a(5, 1));

assert_eq!(
    left + right,
    Vector4::new(a(0, 3), a(5, 0), a(2, 3), a(5, 0))
);
```

Current tests live under `tests/q_sqrt5_*`. The runnable example
`examples/q_sqrt5_vector.rs` contains the full version of the code above.

## Current Scope Limits

- no runtime parent/ring objects;
- no automatic construction of larger fields;
- no `f64` exact scalar implementation;
- no nalgebra `RealField`/`ComplexField` implementation;
- no matrix solve, eigenvalue, or diagonalization layer.

Developer spec, architecture notes, verification commands, and rejected
alternatives live in `DEVELOPMENT.md`.
