//! Exact HKO local-maximum seed bank and control polytopes.

use algebraic_numbers::{Algebraic, OrderedField, Rational, TanPiFifth};
use symplectic::exact::ExactPolytope4D;

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

/// Exact HKO2024 polytope over `Q[tan(pi/5)]`.
pub fn exact_hko_polytope() -> ExactPolytope4D<Algebraic<TanPiFifth>> {
    let z = Algebraic::<TanPiFifth>::zero();
    let one = Algebraic::<TanPiFifth>::one();
    let t = Algebraic::<TanPiFifth>::generator();
    let t2 = t.clone() * t.clone();
    let t3 = t2.clone() * t.clone();

    let a = (Algebraic::<TanPiFifth>::one() + t2.clone()) / Algebraic::<TanPiFifth>::from_i64(4);
    let b = (Algebraic::<TanPiFifth>::from_i64(7) * t.clone() - t3.clone())
        / Algebraic::<TanPiFifth>::from_i64(4);
    let sec36 =
        (Algebraic::<TanPiFifth>::from_i64(3) - t2.clone()) / Algebraic::<TanPiFifth>::from_i64(2);

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
    .expect("exact HKO pentagon polytope")
}

/// Rational simplex control polytope for exact-path sanity checks.
pub fn exact_simplex_polytope() -> ExactPolytope4D<Rational> {
    let z = Rational::from_i64(0);
    ExactPolytope4D::new(vec![
        [Rational::from_i64(-5), z.clone(), z.clone(), z.clone()],
        [z.clone(), Rational::from_i64(-5), z.clone(), z.clone()],
        [z.clone(), z.clone(), Rational::from_i64(-5), z.clone()],
        [z.clone(), z.clone(), z.clone(), Rational::from_i64(-5)],
        [
            Rational::from_i64(5),
            Rational::from_i64(5),
            Rational::from_i64(5),
            Rational::from_i64(5),
        ],
    ])
    .expect("exact simplex control polytope")
}
