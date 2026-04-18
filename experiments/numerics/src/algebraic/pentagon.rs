//! Exact arithmetic in the pentagon field.
//!
//! The field is
//! `Q[t]/(t^4 - 10 t^2 + 5)` with the distinguished real root
//! `t = tan(pi/5) in (1/2, 1)`. Coefficients are stored in the canonical power
//! basis `1, t, t^2, t^3`.

use super::field::{rat, ExactOrderedField, ExactSign};
use super::named_field::NamedFieldTag;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Hard-coded `f64` approximation of `tan(pi/5)` for reporting only.
const TAN_PI_5_F64: f64 = 0.726_542_528_005_360_9;

/// Exact pentagon-field element in basis `1, t, t^2, t^3`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PentagonField {
    coeffs: [BigRational; 4],
}

impl PentagonField {
    /// Construct from canonical basis coefficients.
    pub fn from_coeffs(coeffs: [BigRational; 4]) -> Self {
        Self { coeffs }
    }

    /// Generator `t = tan(pi/5)`.
    pub fn generator() -> Self {
        Self::from_coeffs([
            <BigRational as Zero>::zero(),
            <BigRational as One>::one(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ])
    }

    /// Canonical coefficient access.
    pub fn coeffs(&self) -> &[BigRational; 4] {
        &self.coeffs
    }

    fn eval_f64(&self, x: f64) -> f64 {
        let coeffs: Vec<f64> = self
            .coeffs
            .iter()
            .map(|c| {
                let numer = c.numer().to_f64().unwrap_or(f64::NAN);
                let denom = c.denom().to_f64().unwrap_or(1.0);
                numer / denom
            })
            .collect();
        coeffs[0] + coeffs[1] * x + coeffs[2] * x * x + coeffs[3] * x * x * x
    }

    fn minimal_polynomial(x: &BigRational) -> BigRational {
        let x2 = x.clone() * x.clone();
        x2.clone() * x2.clone() - rat(10) * x2 + rat(5)
    }

    fn root_interval() -> (BigRational, BigRational) {
        (
            BigRational::new(BigInt::from(1), BigInt::from(2)),
            <BigRational as One>::one(),
        )
    }

    fn bisect_root_interval(mut lo: BigRational, mut hi: BigRational) -> (BigRational, BigRational) {
        let two = rat(2);
        let mid = (lo.clone() + hi.clone()) / two;
        let val = Self::minimal_polynomial(&mid);
        // `x^4 - 10x^2 + 5` is strictly decreasing on `(0, sqrt(5))`, so on
        // our distinguished interval `(1/2, 1)` the sign pins down the root side.
        if <BigRational as Signed>::is_positive(&val) {
            lo = mid;
        } else {
            hi = mid;
        }
        (lo, hi)
    }

    fn interval_eval(&self, lo: &BigRational, hi: &BigRational) -> (BigRational, BigRational) {
        let x2_lo = lo.clone() * lo.clone();
        let x2_hi = hi.clone() * hi.clone();
        let x3_lo = x2_lo.clone() * lo.clone();
        let x3_hi = x2_hi.clone() * hi.clone();

        let powers = [
            (<BigRational as One>::one(), <BigRational as One>::one()),
            (lo.clone(), hi.clone()),
            (x2_lo, x2_hi),
            (x3_lo, x3_hi),
        ];

        let mut lower = <BigRational as Zero>::zero();
        let mut upper = <BigRational as Zero>::zero();
        for (coeff, (pow_lo, pow_hi)) in self.coeffs.iter().zip(powers.iter()) {
            if <BigRational as Signed>::is_negative(coeff) {
                lower += coeff.clone() * pow_hi.clone();
                upper += coeff.clone() * pow_lo.clone();
            } else {
                lower += coeff.clone() * pow_lo.clone();
                upper += coeff.clone() * pow_hi.clone();
            }
        }
        (lower, upper)
    }

    fn inverse(&self) -> Self {
        assert!(!self.is_zero(), "attempted to invert zero in PentagonField");

        let basis = [
            Self::from_coeffs([
                <BigRational as One>::one(),
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
            ]),
            Self::from_coeffs([
                <BigRational as Zero>::zero(),
                <BigRational as One>::one(),
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
            ]),
            Self::from_coeffs([
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
                <BigRational as One>::one(),
                <BigRational as Zero>::zero(),
            ]),
            Self::from_coeffs([
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
                <BigRational as Zero>::zero(),
                <BigRational as One>::one(),
            ]),
        ];

        let columns: [PentagonField; 4] = std::array::from_fn(|j| self.clone() * basis[j].clone());
        let mut aug: Vec<Vec<BigRational>> = (0..4)
            .map(|row| {
                let mut line: Vec<BigRational> = (0..4).map(|col| columns[col].coeffs[row].clone()).collect();
                line.push(if row == 0 {
                    <BigRational as One>::one()
                } else {
                    <BigRational as Zero>::zero()
                });
                line
            })
            .collect();

        for col in 0..4 {
            let pivot_row = (col..4)
                .find(|&row| !<BigRational as Zero>::is_zero(&aug[row][col]))
                .expect("nonzero pentagon-field element should yield invertible multiplication matrix");
            aug.swap(col, pivot_row);
            let pivot = aug[col][col].clone();
            for row in (col + 1)..4 {
                if <BigRational as Zero>::is_zero(&aug[row][col]) {
                    continue;
                }
                let factor = aug[row][col].clone() / pivot.clone();
                for j in col..=4 {
                    let correction = aug[col][j].clone() * factor.clone();
                    aug[row][j] -= correction;
                }
            }
        }

        let mut solution = [
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ];
        for row in (0..4).rev() {
            let mut rhs = aug[row][4].clone();
            for col in (row + 1)..4 {
                rhs -= aug[row][col].clone() * solution[col].clone();
            }
            solution[row] = rhs / aug[row][row].clone();
        }

        Self::from_coeffs(solution)
    }
}

impl std::ops::Add for PentagonField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_coeffs(std::array::from_fn(|i| self.coeffs[i].clone() + rhs.coeffs[i].clone()))
    }
}

impl std::ops::Sub for PentagonField {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_coeffs(std::array::from_fn(|i| self.coeffs[i].clone() - rhs.coeffs[i].clone()))
    }
}

impl std::ops::Neg for PentagonField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_coeffs(std::array::from_fn(|i| -self.coeffs[i].clone()))
    }
}

impl std::ops::Mul for PentagonField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coeffs = [
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ];
        for i in 0..4 {
            for j in 0..4 {
                coeffs[i + j] += self.coeffs[i].clone() * rhs.coeffs[j].clone();
            }
        }

        for degree in (4..=6).rev() {
            let carry = coeffs[degree].clone();
            if <BigRational as Zero>::is_zero(&carry) {
                continue;
            }
            coeffs[degree - 2] += rat(10) * carry.clone();
            coeffs[degree - 4] -= rat(5) * carry;
            coeffs[degree] = <BigRational as Zero>::zero();
        }

        Self::from_coeffs([coeffs[0].clone(), coeffs[1].clone(), coeffs[2].clone(), coeffs[3].clone()])
    }
}

impl std::ops::Div for PentagonField {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl ExactOrderedField for PentagonField {
    fn zero() -> Self {
        Self::from_coeffs([
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ])
    }

    fn one() -> Self {
        Self::from_coeffs([
            <BigRational as One>::one(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ])
    }

    fn from_big_rational(value: BigRational) -> Self {
        Self::from_coeffs([
            value,
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
            <BigRational as Zero>::zero(),
        ])
    }

    fn sign(&self) -> ExactSign {
        if self
            .coeffs
            .iter()
            .all(|coeff| <BigRational as Zero>::is_zero(coeff))
        {
            return ExactSign::Zero;
        }

        let (mut lo, mut hi) = Self::root_interval();
        for _ in 0..256 {
            let (lower, upper) = self.interval_eval(&lo, &hi);
            if <BigRational as Signed>::is_positive(&lower) {
                return ExactSign::Positive;
            }
            if <BigRational as Signed>::is_negative(&upper) {
                return ExactSign::Negative;
            }
            (lo, hi) = Self::bisect_root_interval(lo, hi);
        }

        panic!(
            "failed to resolve PentagonField sign after interval refinement: {:?}",
            self.coeffs
        );
    }

    fn to_f64(&self) -> f64 {
        self.eval_f64(TAN_PI_5_F64)
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        self.coeffs.to_vec()
    }

    fn field_tag() -> NamedFieldTag {
        NamedFieldTag::PentagonTanPiFifth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic::field::ExactOrderedField;

    #[test]
    fn generator_satisfies_minimal_polynomial_numerically() {
        let t = PentagonField::generator();
        let p = t.clone() * t.clone() * t.clone() * t.clone()
            - PentagonField::from_i64(10) * t.clone() * t.clone()
            + PentagonField::from_i64(5);
        assert_eq!(p.sign(), ExactSign::Zero);
    }

    #[test]
    fn sec36_formula_squares_to_one_plus_t_squared() {
        let t = PentagonField::generator();
        let sec = (PentagonField::from_i64(3) - t.clone() * t.clone()) / PentagonField::from_i64(2);
        let lhs = sec.clone() * sec;
        let rhs = PentagonField::one() + t.clone() * t;
        assert_eq!(lhs, rhs);
        assert!(rhs.to_f64() > 1.0);
    }

    #[test]
    fn sign_classification_resolves_nonzero_elements() {
        let t = PentagonField::generator();
        let sec = (PentagonField::from_i64(3) - t.clone() * t.clone()) / PentagonField::from_i64(2);
        assert_eq!(sec.sign(), ExactSign::Positive);
        assert_eq!((-sec).sign(), ExactSign::Negative);
    }

    #[test]
    fn multiplication_reduces_to_canonical_basis() {
        let t = PentagonField::generator();
        let t4 = t.clone() * t.clone() * t.clone() * t.clone();
        let expected = PentagonField::from_i64(10) * t.clone() * t.clone() - PentagonField::from_i64(5);
        assert_eq!(t4, expected);
    }

    #[test]
    fn inverse_recovers_one() {
        let t = PentagonField::generator();
        let elem = PentagonField::from_i64(1) + PentagonField::from_frac(1, 2) * t;
        let inv = elem.clone().inverse();
        assert_eq!(elem * inv, PentagonField::one());
    }
}
