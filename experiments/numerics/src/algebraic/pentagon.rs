//! Pentagon-field alias for the algebraic exactness spike.
//!
//! The experiment now uses the shared `algebraic-numbers` implementation of
//! `Q[t]/(t^4 - 10 t^2 + 5)` with the distinguished real root `t = tan(pi/5)`.
//!
//! TODO: add [def:...] to formal math for the `Q[t]/(t^4 - 10 t^2 + 5)` power
//! basis used here.
//! TODO: add [lem:...] to formal math for the interval-refinement sign test in
//! the distinguished real embedding `t = tan(pi/5)`.

use algebraic_numbers::{Algebraic, TanPiFifth};

/// Exact pentagon-field element in basis `1, t, t^2, t^3`.
pub type PentagonField = Algebraic<TanPiFifth>;

#[cfg(test)]
mod tests {
    use super::PentagonField;
    use crate::algebraic::field::ExactOrderedField;

    #[test]
    fn generator_satisfies_minimal_polynomial_exactly() {
        let t = PentagonField::generator();
        let poly = t.clone() * t.clone() * t.clone() * t.clone()
            - PentagonField::from_i64(10) * t.clone() * t.clone()
            + PentagonField::from_i64(5);
        assert_eq!(poly, PentagonField::zero());
    }

    #[test]
    fn multiplication_reduces_to_canonical_basis() {
        let t = PentagonField::generator();
        let lhs = t.clone() * t.clone() * t.clone() * t.clone();
        let rhs = PentagonField::from_i64(10) * t.clone() * t - PentagonField::from_i64(5);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn sign_classification_resolves_nonzero_elements() {
        let t = PentagonField::generator();
        let sec = (PentagonField::from_i64(3) - t.clone() * t.clone()) / PentagonField::from_i64(2);

        assert!(t.is_positive());
        assert!(sec.is_positive());
        assert!(PentagonField::from_i64(-1).is_negative());
    }

    #[test]
    fn sec36_formula_squares_to_one_plus_t_squared() {
        let t = PentagonField::generator();
        let sec = (PentagonField::from_i64(3) - t.clone() * t.clone()) / PentagonField::from_i64(2);
        let lhs = sec.clone() * sec;
        let rhs = PentagonField::one() + t.clone() * t;
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn inverse_recovers_one() {
        let t = PentagonField::generator();
        let elem = PentagonField::from_i64(1) + PentagonField::from_frac(1, 2) * t;
        let inv = PentagonField::one() / elem.clone();
        assert_eq!(elem * inv, PentagonField::one());
    }
}
