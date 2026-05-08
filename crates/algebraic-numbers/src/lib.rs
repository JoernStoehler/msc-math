use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_rational::BigRational;
use num_traits::{One, Zero};

pub trait ExactScalar:
    Clone
    + Debug
    + Eq
    + Zero
    + One
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
{
}

impl ExactScalar for BigRational {}

pub trait RealAlgebraicField: 'static {
    const DEGREE: usize;

    fn polynomial() -> Vec<BigRational>;
}

pub struct Algebraic<F: RealAlgebraicField> {
    coeffs: Vec<BigRational>,
    field: PhantomData<F>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadDegree {
    pub expected: usize,
    pub actual: usize,
}

impl<F: RealAlgebraicField> Algebraic<F> {
    pub fn new(coeffs: Vec<BigRational>) -> Result<Self, BadDegree> {
        if coeffs.len() != F::DEGREE {
            return Err(BadDegree {
                expected: F::DEGREE,
                actual: coeffs.len(),
            });
        }

        Ok(Self::from_coeffs_unchecked(coeffs))
    }

    pub fn coeffs(&self) -> &[BigRational] {
        &self.coeffs
    }

    fn from_coeffs_unchecked(coeffs: Vec<BigRational>) -> Self {
        debug_assert_eq!(coeffs.len(), F::DEGREE);
        Self {
            coeffs,
            field: PhantomData,
        }
    }
}

impl<F: RealAlgebraicField> Clone for Algebraic<F> {
    fn clone(&self) -> Self {
        Self::from_coeffs_unchecked(self.coeffs.clone())
    }
}

impl<F: RealAlgebraicField> Debug for Algebraic<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Algebraic")
            .field("coeffs", &self.coeffs)
            .finish()
    }
}

impl<F: RealAlgebraicField> PartialEq for Algebraic<F> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

impl<F: RealAlgebraicField> Eq for Algebraic<F> {}

impl<F: RealAlgebraicField> Zero for Algebraic<F> {
    fn zero() -> Self {
        Self::from_coeffs_unchecked(vec![BigRational::zero(); F::DEGREE])
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().all(BigRational::is_zero)
    }
}

impl<F: RealAlgebraicField> One for Algebraic<F> {
    fn one() -> Self {
        let mut coeffs = vec![BigRational::zero(); F::DEGREE];
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
        let degree = F::DEGREE;
        let mut product = vec![BigRational::zero(); 2 * degree - 1];

        for (i, left) in self.coeffs.into_iter().enumerate() {
            for (j, right) in rhs.coeffs.iter().enumerate() {
                product[i + j] += left.clone() * right.clone();
            }
        }

        reduce_monic::<F>(product)
    }
}

impl<F: RealAlgebraicField> MulAssign for Algebraic<F> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<F: RealAlgebraicField> ExactScalar for Algebraic<F> {}

fn reduce_monic<F: RealAlgebraicField>(mut coeffs: Vec<BigRational>) -> Algebraic<F> {
    let degree = F::DEGREE;
    let polynomial = F::polynomial();

    assert_eq!(polynomial.len(), degree + 1);
    assert_eq!(polynomial[degree], BigRational::one());

    while coeffs.len() > degree {
        let leading = coeffs.pop().expect("length checked above");
        if leading.is_zero() {
            continue;
        }

        let offset = coeffs.len() - degree;
        for i in 0..degree {
            coeffs[offset + i] -= leading.clone() * polynomial[i].clone();
        }
    }

    Algebraic::from_coeffs_unchecked(coeffs)
}
