use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::field_specification::{field_degree, RealAlgebraicField};

/// Multiply two field elements represented by coefficient vectors and reduce
/// modulo the field polynomial.
pub(crate) fn multiply_mod_field<F: RealAlgebraicField>(
    left: &[BigRational],
    right: &[BigRational],
) -> Vec<BigRational> {
    let degree = field_degree::<F>();
    let mut product = vec![BigRational::zero(); 2 * degree - 1];

    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            product[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }

    reduce_monic::<F>(product)
}

pub(crate) fn inverse_mod_monic<F: RealAlgebraicField>(coeffs: &[BigRational]) -> Vec<BigRational> {
    // Inversion is extended Euclid in Q[x] modulo the field polynomial.
    // This is enough for a fixed field Q[alpha]; constructing larger fields is
    // intentionally outside this crate's current scope.
    let mut old_r = F::polynomial();
    let mut r = coeffs.to_vec();
    trim(&mut old_r);
    trim(&mut r);

    let mut old_t = Vec::new();
    let mut t = vec![BigRational::one()];

    while !r.is_empty() {
        let (quotient, remainder) = polynomial_div_rem(&old_r, &r);
        old_r = r;
        r = remainder;

        let next_t = polynomial_sub(&old_t, &polynomial_mul(&quotient, &t));
        old_t = t;
        t = next_t;
    }

    assert_eq!(
        old_r.len(),
        1,
        "element is not invertible modulo field polynomial"
    );
    let gcd_constant = old_r.pop().expect("length checked above");
    let inverse = old_t
        .into_iter()
        .map(|coeff| coeff / gcd_constant.clone())
        .collect();
    reduce_monic::<F>(inverse)
}

pub(crate) fn polynomial_eval(coeffs: &[BigRational], x: &BigRational) -> BigRational {
    let mut result = BigRational::zero();
    for coeff in coeffs.iter().rev() {
        result *= x.clone();
        result += coeff.clone();
    }
    result
}

fn reduce_monic<F: RealAlgebraicField>(mut coeffs: Vec<BigRational>) -> Vec<BigRational> {
    let degree = field_degree::<F>();
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

    coeffs.resize(degree, BigRational::zero());
    coeffs
}

fn polynomial_sub(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    let len = left.len().max(right.len());
    let mut result = vec![BigRational::zero(); len];
    for i in 0..len {
        if i < left.len() {
            result[i] += left[i].clone();
        }
        if i < right.len() {
            result[i] -= right[i].clone();
        }
    }
    trim(&mut result);
    result
}

fn polynomial_mul(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }

    let mut result = vec![BigRational::zero(); left.len() + right.len() - 1];
    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            result[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }
    trim(&mut result);
    result
}

fn polynomial_div_rem(
    numerator: &[BigRational],
    denominator: &[BigRational],
) -> (Vec<BigRational>, Vec<BigRational>) {
    assert!(!denominator.is_empty(), "division by zero polynomial");

    let mut remainder = numerator.to_vec();
    trim(&mut remainder);
    if remainder.len() < denominator.len() {
        return (Vec::new(), remainder);
    }

    let mut quotient = vec![BigRational::zero(); remainder.len() - denominator.len() + 1];
    let denominator_leading = denominator.last().expect("denominator is nonempty").clone();

    while !remainder.is_empty() && remainder.len() >= denominator.len() {
        let offset = remainder.len() - denominator.len();
        let quotient_coeff =
            remainder.last().expect("remainder is nonempty").clone() / denominator_leading.clone();
        quotient[offset] = quotient_coeff.clone();

        for (i, denominator_coeff) in denominator.iter().enumerate() {
            remainder[offset + i] -= quotient_coeff.clone() * denominator_coeff.clone();
        }
        trim(&mut remainder);
    }

    trim(&mut quotient);
    (quotient, remainder)
}

fn trim(poly: &mut Vec<BigRational>) {
    while poly.last().is_some_and(BigRational::is_zero) {
        poly.pop();
    }
}
