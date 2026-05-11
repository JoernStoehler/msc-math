//! Exact HKO local-maximum seed bank and control polytopes.

use algebraic_numbers::{Algebraic, ExactScalar, RealAlgebraicField};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

pub enum TanPiFifth {}

impl RealAlgebraicField for TanPiFifth {
    fn polynomial() -> Vec<BigRational> {
        vec![rat(5), rat(0), rat(-10), rat(0), rat(1)]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (rat(0), rat(1))
    }
}

pub type PentagonField = Algebraic<TanPiFifth>;

pub trait HkoExactScalar: ExactScalar {
    fn to_f64(&self) -> f64;
    fn canonical_coeffs(&self) -> Vec<BigRational>;
}

impl HkoExactScalar for BigRational {
    fn to_f64(&self) -> f64 {
        ToPrimitive::to_f64(self).expect("HKO exact rational should fit in f64")
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        vec![self.clone()]
    }
}

impl<F: RealAlgebraicField> HkoExactScalar for Algebraic<F> {
    fn to_f64(&self) -> f64 {
        algebraic_to_f64(self)
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        self.coefficients().to_vec()
    }
}

/// Hand-picked exact certification bank seed reused across HKO exact consumers.
pub const HKO_WINNING_SIGMA: &[usize] = &[1, 8, 7, 3, 4, 5, 9];
pub const HKO_RANK_DEFICIENT_SIGMA: &[usize] = &[1, 7, 2, 8, 4, 6, 5];
pub const HKO_FLOAT_WINNING_SIGMA: &[usize] = &[0, 1, 7, 3, 9, 5];
pub const HKO_NEAR_OPTIMAL_SIGMA_A: &[usize] = &[0, 1, 7, 6, 3, 9];
pub const HKO_NEAR_OPTIMAL_SIGMA_B: &[usize] = &[0, 6, 7, 2, 3, 9];
pub const SIMPLEX_CONTROL_SIGMA: &[usize] = &[0, 2, 1, 3, 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactBankTarget {
    HkoPentagon,
    SimplexControl,
}

impl ExactBankTarget {
    pub fn polytope_name(self) -> &'static str {
        match self {
            Self::HkoPentagon => "hko_pentagon",
            Self::SimplexControl => "simplex_control",
        }
    }

    pub fn exact_field(self) -> &'static str {
        match self {
            Self::HkoPentagon => "q_tan_pi_fifth",
            Self::SimplexControl => "rational",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExactBankEntry {
    pub row_name: &'static str,
    pub sigma_label: &'static str,
    pub target: ExactBankTarget,
    pub sigma: &'static [usize],
}

pub const EXACT_BANK_ENTRIES: &[ExactBankEntry] = &[
    ExactBankEntry {
        row_name: "hko_exact_winning_sigma",
        sigma_label: "winning_sigma",
        target: ExactBankTarget::HkoPentagon,
        sigma: HKO_WINNING_SIGMA,
    },
    ExactBankEntry {
        row_name: "hko_exact_rank_deficient_sigma",
        sigma_label: "rank_deficient_sigma",
        target: ExactBankTarget::HkoPentagon,
        sigma: HKO_RANK_DEFICIENT_SIGMA,
    },
    ExactBankEntry {
        row_name: "hko_float_best_sigma",
        sigma_label: "current_float_best_sigma",
        target: ExactBankTarget::HkoPentagon,
        sigma: HKO_FLOAT_WINNING_SIGMA,
    },
    ExactBankEntry {
        row_name: "hko_near_optimal_sigma_a",
        sigma_label: "near_optimal_sigma_a",
        target: ExactBankTarget::HkoPentagon,
        sigma: HKO_NEAR_OPTIMAL_SIGMA_A,
    },
    ExactBankEntry {
        row_name: "hko_near_optimal_sigma_b",
        sigma_label: "near_optimal_sigma_b",
        target: ExactBankTarget::HkoPentagon,
        sigma: HKO_NEAR_OPTIMAL_SIGMA_B,
    },
    ExactBankEntry {
        row_name: "simplex_control_best_sigma",
        sigma_label: "best_sigma",
        target: ExactBankTarget::SimplexControl,
        sigma: SIMPLEX_CONTROL_SIGMA,
    },
];

/// Exact HKO2024 dual vertices over `Q[tan(pi/5)]`.
pub fn exact_hko_dual_vertices() -> Vec<Vector4<PentagonField>> {
    let z = PentagonField::zero();
    let one = PentagonField::one();
    let t = PentagonField::root();
    let t2 = t.clone() * t.clone();
    let t3 = t2.clone() * t.clone();

    let a = (PentagonField::one() + t2.clone()) / PentagonField::from(4);
    let b = (PentagonField::from(7) * t.clone() - t3.clone()) / PentagonField::from(4);
    let sec36 = (PentagonField::from(3) - t2.clone()) / PentagonField::from(2);

    vec![
        Vector4::new(one.clone(), t.clone(), z.clone(), z.clone()),
        Vector4::new(-a.clone(), b.clone(), z.clone(), z.clone()),
        Vector4::new(-sec36.clone(), z.clone(), z.clone(), z.clone()),
        Vector4::new(-a.clone(), -b.clone(), z.clone(), z.clone()),
        Vector4::new(one.clone(), -t.clone(), z.clone(), z.clone()),
        Vector4::new(z.clone(), z.clone(), t.clone(), -one.clone()),
        Vector4::new(z.clone(), z.clone(), b.clone(), a.clone()),
        Vector4::new(z.clone(), z.clone(), z.clone(), sec36.clone()),
        Vector4::new(z.clone(), z.clone(), -b, a),
        Vector4::new(z.clone(), z.clone(), -t, -one),
    ]
}

/// Rational simplex control dual vertices for exact-path sanity checks.
pub fn exact_simplex_dual_vertices() -> Vec<Vector4<BigRational>> {
    let z = rat(0);
    vec![
        Vector4::new(rat(-5), z.clone(), z.clone(), z.clone()),
        Vector4::new(z.clone(), rat(-5), z.clone(), z.clone()),
        Vector4::new(z.clone(), z.clone(), rat(-5), z.clone()),
        Vector4::new(z.clone(), z.clone(), z.clone(), rat(-5)),
        Vector4::new(rat(5), rat(5), rat(5), rat(5)),
    ]
}

fn rat(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn algebraic_to_f64<F: RealAlgebraicField>(value: &Algebraic<F>) -> f64 {
    if value.is_zero() {
        return 0.0;
    }

    let mut lower = BigRational::from_integer((-1).into());
    let mut upper = BigRational::from_integer(1.into());
    while Algebraic::<F>::from(lower.clone()) > value.clone() {
        lower *= BigRational::from_integer(2.into());
    }
    while Algebraic::<F>::from(upper.clone()) < value.clone() {
        upper *= BigRational::from_integer(2.into());
    }

    for _ in 0..80 {
        let midpoint = (lower.clone() + upper.clone()) / BigRational::from_integer(2.into());
        if Algebraic::<F>::from(midpoint.clone()) <= value.clone() {
            lower = midpoint;
        } else {
            upper = midpoint;
        }
    }

    ToPrimitive::to_f64(&((lower + upper) / BigRational::from_integer(2.into())))
        .expect("bounded algebraic approximation should fit in f64")
}
