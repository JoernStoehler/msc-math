//! Exact algebraic and rational control fixtures used by the spike.

use super::field::{rat, ExactOrderedField};
use super::geom::{ExactPolytope4D, ExactPolytopeError};
use super::pentagon::PentagonField;
use num_rational::BigRational;

/// A selected HKO sigma that current numerics uses as a capacity-achieving orbit.
pub const HKO_WINNING_SIGMA: &[usize] = &[1, 8, 7, 3, 4, 5, 9];

/// A selected HKO sigma with a rank-deficient exact KKT system.
pub const HKO_RANK_DEFICIENT_SIGMA: &[usize] = &[1, 7, 2, 8, 4, 6, 5];

/// Exact simplex control in `Q`.
pub fn exact_simplex() -> Result<ExactPolytope4D<BigRational>, ExactPolytopeError> {
    let z = rat(0);
    ExactPolytope4D::new(vec![
        [rat(-5), z.clone(), z.clone(), z.clone()],
        [z.clone(), rat(-5), z.clone(), z.clone()],
        [z.clone(), z.clone(), rat(-5), z.clone()],
        [z.clone(), z.clone(), z.clone(), rat(-5)],
        [rat(5), rat(5), rat(5), rat(5)],
    ])
}

/// Exact hypercube control in `Q`.
pub fn exact_hypercube() -> Result<ExactPolytope4D<BigRational>, ExactPolytopeError> {
    let z = rat(0);
    ExactPolytope4D::new(vec![
        [rat(1), z.clone(), z.clone(), z.clone()],
        [rat(-1), z.clone(), z.clone(), z.clone()],
        [z.clone(), rat(1), z.clone(), z.clone()],
        [z.clone(), rat(-1), z.clone(), z.clone()],
        [z.clone(), z.clone(), rat(1), z.clone()],
        [z.clone(), z.clone(), rat(-1), z.clone()],
        [z.clone(), z.clone(), z.clone(), rat(1)],
        [z.clone(), z.clone(), z.clone(), rat(-1)],
    ])
}

/// Exact HKO pentagon counterexample in `Q[tan(pi/5)]`.
pub fn exact_hko_pentagon() -> Result<ExactPolytope4D<PentagonField>, ExactPolytopeError> {
    let z = PentagonField::zero();
    let one = PentagonField::one();
    let t = PentagonField::generator();
    let t2 = t.clone() * t.clone();
    let t3 = t2.clone() * t.clone();

    let a = (PentagonField::one() + t2.clone()) / PentagonField::from_i64(4);
    let b = (PentagonField::from_i64(7) * t.clone() - t3.clone()) / PentagonField::from_i64(4);
    let sec36 = (PentagonField::from_i64(3) - t2.clone()) / PentagonField::from_i64(2);

    ExactPolytope4D::new(vec![
        [one.clone(), t.clone(), z.clone(), z.clone()],
        [-a.clone(), b.clone(), z.clone(), z.clone()],
        [-sec36.clone(), z.clone(), z.clone(), z.clone()],
        [-a.clone(), -b.clone(), z.clone(), z.clone()],
        [one.clone(), -t.clone(), z.clone(), z.clone()],
        [z.clone(), z.clone(), t.clone(), -one.clone()],
        [z.clone(), z.clone(), b.clone(), a.clone()],
        [z.clone(), z.clone(), z.clone(), sec36.clone()],
        [z.clone(), z.clone(), -b, a],
        [z.clone(), z.clone(), -t, -one],
    ])
}

/// Expected HKO capacity formula evaluated in `f64` for reporting checks.
pub fn hko_capacity_formula_f64() -> f64 {
    2.0 * (std::f64::consts::PI / 10.0).cos() * (1.0 + (std::f64::consts::PI / 5.0).cos())
}
