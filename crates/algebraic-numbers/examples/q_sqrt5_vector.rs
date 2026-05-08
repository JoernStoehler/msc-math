use algebraic_numbers::{Algebraic, RationalInterval, RealAlgebraicField};
use nalgebra::Vector4;
use num_rational::BigRational;

enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        // Low-to-high coefficients for t^2 - 5.
        vec![q(-5), q(0), q(1)]
    }

    fn isolating_interval() -> RationalInterval {
        // Select the positive root sqrt(5).
        RationalInterval::new(q(2), q(3))
    }
}

type Qsqrt5 = Algebraic<Sqrt5>;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn a(rational: i64, sqrt5_coeff: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![q(rational), q(sqrt5_coeff)]).unwrap()
}

fn main() {
    let alpha = Qsqrt5::alpha();
    let left = Vector4::new(alpha.clone(), a(1, 1), a(2, 0), a(0, -1));
    let right = Vector4::new(2 * alpha, a(4, -1), a(0, 3), a(5, 1));

    assert_eq!(
        left + right,
        Vector4::new(a(0, 3), a(5, 0), a(2, 3), a(5, 0))
    );
}
