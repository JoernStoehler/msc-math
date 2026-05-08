use algebraic_numbers::{Algebraic, RationalInterval, RealAlgebraicField};
use nalgebra::Vector4;
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
