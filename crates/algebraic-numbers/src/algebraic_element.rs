use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::field_specification::{field_degree, RealAlgebraicField};
use crate::polynomial_arithmetic::inverse_mod_monic;
use crate::sign_ordering::sign_at_field_root;

/// Element of the statically chosen real algebraic field `Q[alpha]`.
///
/// The coefficient vector stores
/// `c[0] + c[1] alpha + ... + c[n - 1] alpha^(n - 1)`.
pub struct Algebraic<F: RealAlgebraicField> {
    pub(crate) coeffs: Vec<BigRational>,
    field: PhantomData<F>,
}

impl<F: RealAlgebraicField> Algebraic<F> {
    pub fn new<const N: usize>(coeffs: [BigRational; N]) -> Self {
        assert_eq!(N, field_degree::<F>());
        Self::from_coeffs_unchecked(coeffs.into_iter().collect())
    }

    pub(crate) fn from_rational(rational: BigRational) -> Self {
        let mut coeffs = vec![BigRational::zero(); field_degree::<F>()];
        coeffs[0] = rational;
        Self::from_coeffs_unchecked(coeffs)
    }

    pub fn root() -> Self {
        let degree = field_degree::<F>();
        if degree == 1 {
            let polynomial = F::polynomial();
            let root = -polynomial[0].clone();
            return Self::from_rational(root);
        }

        let mut coeffs = vec![BigRational::zero(); degree];
        coeffs[1] = BigRational::one();
        Self::from_coeffs_unchecked(coeffs)
    }

    pub(crate) fn inverse(&self) -> Self {
        assert!(!self.is_zero(), "cannot invert zero");
        Self::from_coeffs_unchecked(inverse_mod_monic::<F>(&self.coeffs))
    }

    pub(crate) fn from_coeffs_unchecked(coeffs: Vec<BigRational>) -> Self {
        debug_assert_eq!(coeffs.len(), field_degree::<F>());
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

impl<F: RealAlgebraicField> PartialOrd for Algebraic<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: RealAlgebraicField> Ord for Algebraic<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        let difference = self
            .coeffs
            .iter()
            .zip(&other.coeffs)
            .map(|(left, right)| left.clone() - right.clone())
            .collect::<Vec<_>>();

        sign_at_field_root::<F>(&difference)
    }
}
