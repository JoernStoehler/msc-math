use algebraic_numbers::{Algebraic, RealAlgebraicField};
use num_rational::BigRational;

enum QWithNamedRoot {}

impl RealAlgebraicField for QWithNamedRoot {
    fn polynomial() -> Vec<BigRational> {
        // t - 7 selects the rational root 7.
        vec![q(-7), q(1)]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (q(6), q(8))
    }
}

type Q = Algebraic<QWithNamedRoot>;

#[test]
fn root_of_degree_one_field_is_the_rational_root() {
    assert_eq!(Q::root(), Q::from(7));
}

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}
