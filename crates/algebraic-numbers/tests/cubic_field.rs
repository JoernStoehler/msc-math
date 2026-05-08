use algebraic_numbers::{Algebraic, RealAlgebraicField};
use num_rational::BigRational;

enum CubeRoot2 {}

impl RealAlgebraicField for CubeRoot2 {
    fn polynomial() -> Vec<BigRational> {
        // t^3 - 2
        vec![q(-2), q(0), q(0), q(1)]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (q(1), q(2))
    }
}

type Qcuberoot2 = Algebraic<CubeRoot2>;

#[test]
fn cubic_root_reduces_and_orders_exactly() {
    let root = Qcuberoot2::root();

    assert_eq!(
        root.clone() * root.clone() * root.clone(),
        Qcuberoot2::from(2)
    );
    assert!(root.clone() > Qcuberoot2::from(1));
    assert!(root.clone() < Qcuberoot2::from(2));
    assert_eq!(Qcuberoot2::from(2) / root.clone(), root.clone() * root);
}

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}
