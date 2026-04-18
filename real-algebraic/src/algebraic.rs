//! Purpose: generic algebraic scalar implementation for fields `Q[t] / (p(t))`.
//! Context: named field specifications instantiate this one implementation via
//! `Algebraic<S>` instead of each field reimplementing arithmetic by hand.

use crate::field::OrderedField;
use crate::sign::Sign;
use crate::spec::StaticFieldSpec;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Algebraic element in the field described by `S`.
pub struct Algebraic<S: StaticFieldSpec> {
    coeffs: Vec<BigRational>,
    _marker: PhantomData<S>,
}

impl<S: StaticFieldSpec> Clone for Algebraic<S> {
    fn clone(&self) -> Self {
        Self {
            coeffs: self.coeffs.clone(),
            _marker: PhantomData,
        }
    }
}

impl<S: StaticFieldSpec> std::fmt::Debug for Algebraic<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Algebraic")
            .field("field", &S::name())
            .field("coeffs", &self.coeffs)
            .finish()
    }
}

impl<S: StaticFieldSpec> PartialEq for Algebraic<S> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

impl<S: StaticFieldSpec> Eq for Algebraic<S> {}

impl<S: StaticFieldSpec> PartialOrd for Algebraic<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: StaticFieldSpec> Ord for Algebraic<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_real(other)
    }
}

impl<S: StaticFieldSpec> Algebraic<S> {
    /// Construct from canonical-basis coefficients.
    pub fn from_coeffs(coeffs: Vec<BigRational>) -> Self {
        Self {
            coeffs: reduce_modulus::<S>(coeffs),
            _marker: PhantomData,
        }
    }

    /// The residue class of the generator `t`.
    pub fn generator() -> Self {
        let degree = extension_degree::<S>();
        let mut coeffs = vec![rat_zero(); degree + 1];
        coeffs[1] = rat_one();
        Self::from_coeffs(coeffs)
    }

    /// Canonical coefficient access.
    pub fn coeffs(&self) -> &[BigRational] {
        &self.coeffs
    }

    fn inverse(&self) -> Self {
        assert!(
            !self.coeffs.iter().all(Zero::is_zero),
            "zero has no multiplicative inverse"
        );
        let modulus = normalize_monic::<S>(S::minimal_polynomial());
        let (gcd, s_coeffs, _) = poly_extended_gcd(self.coeffs.clone(), modulus.clone());
        assert!(
            poly_degree(&gcd) == 0 && !Zero::is_zero(&gcd[0]),
            "nonzero element should be invertible in a field extension"
        );
        let scale = gcd[0].clone();
        let inv_coeffs: Vec<BigRational> = s_coeffs
            .into_iter()
            .map(|coeff| coeff / scale.clone())
            .collect();
        Self::from_coeffs(inv_coeffs)
    }
}

impl<S: StaticFieldSpec> OrderedField for Algebraic<S> {
    fn zero() -> Self {
        Self::from_coeffs(vec![rat_zero()])
    }

    fn one() -> Self {
        Self::from_coeffs(vec![rat_one()])
    }

    fn from_rational(value: BigRational) -> Self {
        Self::from_coeffs(vec![value])
    }

    fn field_name() -> &'static str {
        S::name()
    }

    fn basis_labels() -> Vec<String> {
        basis_labels::<S>()
    }

    fn sign(&self) -> Sign {
        if self.coeffs.iter().all(Zero::is_zero) {
            return Sign::Zero;
        }

        let modulus = normalize_monic::<S>(S::minimal_polynomial());
        let (mut lo, mut hi) = S::isolating_interval();
        let mut lo_sign = sign_rational(&eval_poly(&modulus, &lo));
        let mut hi_sign = sign_rational(&eval_poly(&modulus, &hi));
        assert_ne!(
            lo_sign,
            Sign::Zero,
            "isolating interval lower endpoint hits a root"
        );
        assert_ne!(
            hi_sign,
            Sign::Zero,
            "isolating interval upper endpoint hits a root"
        );
        assert_ne!(
            lo_sign, hi_sign,
            "isolating interval endpoints must bracket the chosen real root"
        );

        for _ in 0..512 {
            let interval =
                eval_interval_horner(&self.coeffs, &Interval::new(lo.clone(), hi.clone()));
            if <BigRational as Signed>::is_positive(&interval.lo) {
                return Sign::Positive;
            }
            if <BigRational as Signed>::is_negative(&interval.hi) {
                return Sign::Negative;
            }

            let mid = (lo.clone() + hi.clone()) / BigRational::from_integer(2.into());
            let mid_value = eval_poly(&modulus, &mid);
            let mid_sign = sign_rational(&mid_value);
            if mid_sign == Sign::Zero {
                return sign_rational(&eval_poly(&self.coeffs, &mid));
            }
            if mid_sign == lo_sign {
                lo = mid;
                lo_sign = mid_sign;
            } else {
                hi = mid;
                hi_sign = mid_sign;
            }
            let _ = hi_sign;
        }

        panic!("failed to determine sign after interval refinement");
    }

    fn cmp_real(&self, other: &Self) -> Ordering {
        match (self.clone() - other.clone()).sign() {
            Sign::Negative => Ordering::Less,
            Sign::Zero => Ordering::Equal,
            Sign::Positive => Ordering::Greater,
        }
    }

    fn to_f64(&self) -> f64 {
        let root = approximate_root_f64::<S>();
        self.coeffs
            .iter()
            .enumerate()
            .map(|(idx, coeff)| {
                ToPrimitive::to_f64(coeff).unwrap_or(f64::NAN) * root.powi(idx as i32)
            })
            .sum()
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        self.coeffs.clone()
    }
}

impl<S: StaticFieldSpec> std::ops::Add for Algebraic<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<'a, S: StaticFieldSpec> std::ops::Add<&'a Algebraic<S>> for Algebraic<S> {
    type Output = Algebraic<S>;

    fn add(self, rhs: &'a Algebraic<S>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Add<Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn add(self, rhs: Algebraic<S>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Add<&Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn add(self, rhs: &Algebraic<S>) -> Self::Output {
        let degree = extension_degree::<S>();
        let coeffs = (0..degree)
            .map(|idx| self.coeffs[idx].clone() + rhs.coeffs[idx].clone())
            .collect();
        Algebraic::from_coeffs(coeffs)
    }
}

impl<S: StaticFieldSpec> std::ops::Sub for Algebraic<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<'a, S: StaticFieldSpec> std::ops::Sub<&'a Algebraic<S>> for Algebraic<S> {
    type Output = Algebraic<S>;

    fn sub(self, rhs: &'a Algebraic<S>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Sub<Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn sub(self, rhs: Algebraic<S>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Sub<&Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn sub(self, rhs: &Algebraic<S>) -> Self::Output {
        let degree = extension_degree::<S>();
        let coeffs = (0..degree)
            .map(|idx| self.coeffs[idx].clone() - rhs.coeffs[idx].clone())
            .collect();
        Algebraic::from_coeffs(coeffs)
    }
}

impl<S: StaticFieldSpec> std::ops::Mul for Algebraic<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl<'a, S: StaticFieldSpec> std::ops::Mul<&'a Algebraic<S>> for Algebraic<S> {
    type Output = Algebraic<S>;

    fn mul(self, rhs: &'a Algebraic<S>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Mul<Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn mul(self, rhs: Algebraic<S>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Mul<&Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn mul(self, rhs: &Algebraic<S>) -> Self::Output {
        let degree = extension_degree::<S>();
        let mut coeffs = vec![rat_zero(); 2 * degree.saturating_sub(1) + 1];
        for i in 0..degree {
            for j in 0..degree {
                coeffs[i + j] += self.coeffs[i].clone() * rhs.coeffs[j].clone();
            }
        }
        Algebraic::from_coeffs(coeffs)
    }
}

impl<S: StaticFieldSpec> std::ops::Div for Algebraic<S> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl<'a, S: StaticFieldSpec> std::ops::Div<&'a Algebraic<S>> for Algebraic<S> {
    type Output = Algebraic<S>;

    fn div(self, rhs: &'a Algebraic<S>) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Div<Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn div(self, rhs: Algebraic<S>) -> Self::Output {
        self.div(&rhs)
    }
}

impl<S: StaticFieldSpec> std::ops::Div<&Algebraic<S>> for &Algebraic<S> {
    type Output = Algebraic<S>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: &Algebraic<S>) -> Self::Output {
        self.clone() * rhs.inverse()
    }
}

impl<S: StaticFieldSpec> std::ops::Neg for Algebraic<S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl<S: StaticFieldSpec> std::ops::Neg for &Algebraic<S> {
    type Output = Algebraic<S>;

    fn neg(self) -> Self::Output {
        let coeffs = self.coeffs.iter().map(|coeff| -coeff.clone()).collect();
        Algebraic::from_coeffs(coeffs)
    }
}

fn extension_degree<S: StaticFieldSpec>() -> usize {
    let modulus = normalize_monic::<S>(S::minimal_polynomial());
    assert!(
        modulus.len() >= 2,
        "minimal polynomial must have degree at least one"
    );
    modulus.len() - 1
}

fn basis_labels<S: StaticFieldSpec>() -> Vec<String> {
    let degree = extension_degree::<S>();
    let symbol = S::generator_name();
    let mut labels = Vec::with_capacity(degree);
    labels.push("1".to_string());
    for power in 1..degree {
        if power == 1 {
            labels.push(symbol.to_string());
        } else {
            labels.push(format!("{symbol}^{power}"));
        }
    }
    labels
}

fn normalize_monic<S: StaticFieldSpec>(poly: Vec<BigRational>) -> Vec<BigRational> {
    let mut poly = trim(poly);
    assert!(!poly.is_empty(), "minimal polynomial must be nonzero");
    let leading = poly.last().cloned().expect("nonempty polynomial");
    assert!(
        !<BigRational as Zero>::is_zero(&leading),
        "minimal polynomial leading coefficient must be nonzero"
    );
    if leading != rat_one() {
        for coeff in &mut poly {
            *coeff /= leading.clone();
        }
    }
    poly
}

fn reduce_modulus<S: StaticFieldSpec>(coeffs: Vec<BigRational>) -> Vec<BigRational> {
    let modulus = normalize_monic::<S>(S::minimal_polynomial());
    let degree = modulus.len() - 1;
    let mut coeffs = trim(coeffs);
    if coeffs.len() <= degree {
        coeffs.resize(degree, rat_zero());
        return coeffs;
    }

    for current_degree in (degree..coeffs.len()).rev() {
        let carry = coeffs[current_degree].clone();
        if <BigRational as Zero>::is_zero(&carry) {
            continue;
        }
        for idx in 0..degree {
            coeffs[current_degree - degree + idx] -= carry.clone() * modulus[idx].clone();
        }
    }
    coeffs.truncate(degree);
    coeffs.resize(degree, rat_zero());
    trim(coeffs)
        .into_iter()
        .chain(std::iter::repeat_with(rat_zero))
        .take(degree)
        .collect()
}

fn trim(mut coeffs: Vec<BigRational>) -> Vec<BigRational> {
    while coeffs.last().is_some_and(Zero::is_zero) {
        coeffs.pop();
    }
    if coeffs.is_empty() {
        vec![rat_zero()]
    } else {
        coeffs
    }
}

fn poly_degree(poly: &[BigRational]) -> usize {
    poly.iter()
        .rposition(|coeff| !<BigRational as Zero>::is_zero(coeff))
        .unwrap_or(0)
}

fn eval_poly(coeffs: &[BigRational], x: &BigRational) -> BigRational {
    coeffs
        .iter()
        .rev()
        .fold(rat_zero(), |acc, coeff| acc * x.clone() + coeff.clone())
}

#[derive(Clone)]
struct Interval {
    lo: BigRational,
    hi: BigRational,
}

impl Interval {
    fn new(lo: BigRational, hi: BigRational) -> Self {
        assert!(lo <= hi, "interval endpoints must satisfy lo <= hi");
        Self { lo, hi }
    }

    fn point(value: BigRational) -> Self {
        Self {
            lo: value.clone(),
            hi: value,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.lo.clone() + other.lo.clone(),
            self.hi.clone() + other.hi.clone(),
        )
    }

    fn mul(&self, other: &Self) -> Self {
        let cands = [
            self.lo.clone() * other.lo.clone(),
            self.lo.clone() * other.hi.clone(),
            self.hi.clone() * other.lo.clone(),
            self.hi.clone() * other.hi.clone(),
        ];
        let lo = cands.iter().cloned().min().expect("four candidates");
        let hi = cands.iter().cloned().max().expect("four candidates");
        Self::new(lo, hi)
    }
}

fn eval_interval_horner(coeffs: &[BigRational], x: &Interval) -> Interval {
    coeffs
        .iter()
        .rev()
        .fold(Interval::point(rat_zero()), |acc, coeff| {
            acc.mul(x).add(&Interval::point(coeff.clone()))
        })
}

fn sign_rational(value: &BigRational) -> Sign {
    if <BigRational as Zero>::is_zero(value) {
        Sign::Zero
    } else if <BigRational as Signed>::is_positive(value) {
        Sign::Positive
    } else {
        Sign::Negative
    }
}

fn poly_add(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    let len = left.len().max(right.len());
    let mut out = vec![rat_zero(); len];
    for idx in 0..len {
        let l = left.get(idx).cloned().unwrap_or_else(rat_zero);
        let r = right.get(idx).cloned().unwrap_or_else(rat_zero);
        out[idx] = l + r;
    }
    trim(out)
}

fn poly_sub(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    let len = left.len().max(right.len());
    let mut out = vec![rat_zero(); len];
    for idx in 0..len {
        let l = left.get(idx).cloned().unwrap_or_else(rat_zero);
        let r = right.get(idx).cloned().unwrap_or_else(rat_zero);
        out[idx] = l - r;
    }
    trim(out)
}

fn poly_mul(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    if left.iter().all(Zero::is_zero) || right.iter().all(Zero::is_zero) {
        return vec![rat_zero()];
    }
    let mut out = vec![rat_zero(); left.len() + right.len() - 1];
    for i in 0..left.len() {
        for j in 0..right.len() {
            out[i + j] += left[i].clone() * right[j].clone();
        }
    }
    trim(out)
}

fn poly_div_rem(
    dividend: Vec<BigRational>,
    divisor: Vec<BigRational>,
) -> (Vec<BigRational>, Vec<BigRational>) {
    let dividend = trim(dividend);
    let divisor = trim(divisor);
    assert!(
        !divisor.iter().all(Zero::is_zero),
        "polynomial division by zero"
    );
    if poly_degree(&dividend) < poly_degree(&divisor) {
        return (vec![rat_zero()], dividend);
    }

    let mut remainder = dividend;
    let divisor_degree = poly_degree(&divisor);
    let divisor_leading = divisor[divisor_degree].clone();
    let mut quotient = vec![rat_zero(); poly_degree(&remainder) - divisor_degree + 1];

    while !remainder.iter().all(Zero::is_zero) && poly_degree(&remainder) >= divisor_degree {
        let remainder_degree = poly_degree(&remainder);
        let shift = remainder_degree - divisor_degree;
        let factor = remainder[remainder_degree].clone() / divisor_leading.clone();
        quotient[shift] += factor.clone();
        for (idx, coeff) in divisor.iter().enumerate().take(divisor_degree + 1) {
            remainder[idx + shift] -= factor.clone() * coeff.clone();
        }
        remainder = trim(remainder);
    }

    (trim(quotient), trim(remainder))
}

fn poly_extended_gcd(
    a: Vec<BigRational>,
    b: Vec<BigRational>,
) -> (Vec<BigRational>, Vec<BigRational>, Vec<BigRational>) {
    let mut old_r = trim(a);
    let mut r = trim(b);
    let mut old_s = vec![rat_one()];
    let mut s = vec![rat_zero()];
    let mut old_t = vec![rat_zero()];
    let mut t = vec![rat_one()];

    while !r.iter().all(Zero::is_zero) {
        let (q, new_r) = poly_div_rem(old_r.clone(), r.clone());
        old_r = r;
        r = new_r;

        let new_s = poly_sub(&old_s, &poly_mul(&q, &s));
        old_s = s;
        s = new_s;

        let new_t = poly_sub(&old_t, &poly_mul(&q, &t));
        old_t = t;
        t = new_t;
    }

    (trim(old_r), trim(old_s), trim(old_t))
}

fn approximate_root_f64<S: StaticFieldSpec>() -> f64 {
    let modulus = normalize_monic::<S>(S::minimal_polynomial());
    let (mut lo, mut hi) = S::isolating_interval();
    let mut lo_sign = sign_rational(&eval_poly(&modulus, &lo));

    for _ in 0..256 {
        let mid = (lo.clone() + hi.clone()) / BigRational::from_integer(2.into());
        let mid_value = eval_poly(&modulus, &mid);
        let mid_sign = sign_rational(&mid_value);
        if mid_sign == Sign::Zero {
            return ToPrimitive::to_f64(&mid).unwrap_or(f64::NAN);
        }
        if mid_sign == lo_sign {
            lo = mid;
            lo_sign = mid_sign;
        } else {
            hi = mid;
        }
    }

    let midpoint = (lo + hi) / BigRational::from_integer(2.into());
    ToPrimitive::to_f64(&midpoint).unwrap_or(f64::NAN)
}

fn rat_zero() -> BigRational {
    <BigRational as Zero>::zero()
}

fn rat_one() -> BigRational {
    <BigRational as One>::one()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::named_fields::TanPiFifth;

    type TanPiFifthField = Algebraic<TanPiFifth>;

    #[test]
    fn tan_pi_fifth_generator_satisfies_defining_polynomial() {
        let t = TanPiFifthField::generator();
        let poly = TanPiFifth::minimal_polynomial();
        let value = poly
            .iter()
            .enumerate()
            .fold(TanPiFifthField::zero(), |acc, (idx, coeff)| {
                let mut term = TanPiFifthField::from_rational(coeff.clone());
                for _ in 0..idx {
                    term = term * t.clone();
                }
                acc + term
            });
        assert!(value.is_zero());
    }

    #[test]
    fn inverse_recovers_one() {
        let t = TanPiFifthField::generator();
        let value = TanPiFifthField::one() + TanPiFifthField::from_frac(1, 2) * t;
        let inv = value.inverse();
        assert_eq!(value * inv, TanPiFifthField::one());
    }

    #[test]
    fn comparison_orders_values_in_the_real_embedding() {
        let t = TanPiFifthField::generator();
        assert!(t > TanPiFifthField::from_frac(1, 2));
        assert!(t < TanPiFifthField::one());
    }

    #[test]
    fn multiplication_reduces_to_the_canonical_basis() {
        let t = TanPiFifthField::generator();
        let t4 = t.clone() * t.clone() * t.clone() * t.clone();
        let expected =
            TanPiFifthField::from_i64(10) * t.clone() * t.clone() - TanPiFifthField::from_i64(5);
        assert_eq!(t4, expected);
    }

    #[test]
    fn sign_classification_distinguishes_negative_zero_and_positive() {
        let t = TanPiFifthField::generator();
        assert_eq!(TanPiFifthField::zero().sign(), Sign::Zero);
        assert_eq!(t.sign(), Sign::Positive);
        assert_eq!((-t).sign(), Sign::Negative);
    }

    #[test]
    fn equivalent_representations_canonicalize_to_the_same_value() {
        let t = TanPiFifthField::generator();
        let left = t.clone() * t.clone() * t.clone() * t.clone();
        let right =
            TanPiFifthField::from_i64(10) * t.clone() * t.clone() - TanPiFifthField::from_i64(5);
        assert_eq!(left.coeffs(), right.coeffs());
    }
}
