use algebraic_numbers::{Algebraic, RealAlgebraicField};
use nalgebra::Vector4;
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

fn main() {
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
}
