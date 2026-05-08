use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::algebraic_element::Algebraic;
use crate::exact_scalar::ExactScalar;
use crate::field_specification::{field_degree, RealAlgebraicField};
use crate::polynomial_arithmetic::multiply_mod_field;

impl<F: RealAlgebraicField> Zero for Algebraic<F> {
    fn zero() -> Self {
        Self::from_coeffs_unchecked(vec![BigRational::zero(); field_degree::<F>()])
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().all(BigRational::is_zero)
    }
}

impl<F: RealAlgebraicField> One for Algebraic<F> {
    fn one() -> Self {
        let mut coeffs = vec![BigRational::zero(); field_degree::<F>()];
        coeffs[0] = BigRational::one();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> Neg for Algebraic<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_coeffs_unchecked(self.coeffs.into_iter().map(Neg::neg).collect())
    }
}

impl<F: RealAlgebraicField> Add for Algebraic<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let coeffs = self
            .coeffs
            .into_iter()
            .zip(rhs.coeffs)
            .map(|(left, right)| left + right)
            .collect();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> AddAssign for Algebraic<F> {
    fn add_assign(&mut self, rhs: Self) {
        for (left, right) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *left += right;
        }
    }
}

impl<F: RealAlgebraicField> Sub for Algebraic<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let coeffs = self
            .coeffs
            .into_iter()
            .zip(rhs.coeffs)
            .map(|(left, right)| left - right)
            .collect();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> SubAssign for Algebraic<F> {
    fn sub_assign(&mut self, rhs: Self) {
        for (left, right) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *left -= right;
        }
    }
}

impl<F: RealAlgebraicField> Mul for Algebraic<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(self, rhs)
    }
}

impl<F: RealAlgebraicField> MulAssign for Algebraic<F> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<F: RealAlgebraicField> Div for Algebraic<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        multiply(self, rhs.inverse())
    }
}

impl<F: RealAlgebraicField> DivAssign for Algebraic<F> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<F: RealAlgebraicField> From<BigRational> for Algebraic<F> {
    fn from(value: BigRational) -> Self {
        Self::from_rational(value)
    }
}

impl<F: RealAlgebraicField> From<i64> for Algebraic<F> {
    fn from(value: i64) -> Self {
        Self::from_rational(BigRational::from_integer(value.into()))
    }
}

impl<F: RealAlgebraicField> ExactScalar for Algebraic<F> {}

fn multiply<F: RealAlgebraicField>(left: Algebraic<F>, right: Algebraic<F>) -> Algebraic<F> {
    Algebraic::from_coeffs_unchecked(multiply_mod_field::<F>(&left.coeffs, &right.coeffs))
}
