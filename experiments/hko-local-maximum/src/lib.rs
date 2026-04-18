//! Shared helpers for exp-hko-local-maximum experiments.
//!
//! Shared experiment-local helpers for the HKO local-maximum program.

use real_algebraic::{Algebraic, OrderedField, Rational, TanPiFifth};
use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::algorithms::{OrbitAdmissibility, OrbitKktData};
use symplectic::exact::ExactPolytope4D;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

pub type TanPiFifthField = Algebraic<TanPiFifth>;

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

#[derive(Debug, Clone)]
pub struct InstrumentedOrbitSearch {
    pub capacity: f64,
    pub capacity_uncertain: f64,
    pub orbits: Vec<OrbitKktData>,
    pub iterations: u64,
}

fn action_bounds_from_q(q: f64, q_error_bound: f64) -> (f64, f64) {
    let q_upper = q + q_error_bound;
    let action_lower = 0.5 / q_upper;
    let q_lower = q - q_error_bound;
    let action_upper = if q_lower > EPS_Q_POSITIVE {
        0.5 / q_lower
    } else {
        f64::INFINITY
    };
    (action_lower, action_upper)
}

/// Enumerate all "valid" HK2017 orbits for the HKO local-maximum experiments.
///
/// These binaries intentionally keep the stricter `beta > EPS_BETA_POSITIVE`
/// validity policy rather than adopting the richer library collector semantics.
pub fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedOrbitSearch> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);

    let mut orbits: Vec<OrbitKktData> = Vec::new();
    let mut best_uncertain_action: Option<f64> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_feasible_cycle(perm, &adj) {
                    return;
                }
                iterations += 1;

                if let KktOutcome::Feasible(kkt_result) = solve_kkt_for(polytope, perm) {
                    let q_val = kkt_result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta = &kkt_result.beta;
                    let beta_min = beta.iter().copied().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;
                    let (action_lower, action_upper) =
                        action_bounds_from_q(q_val, kkt_result.q_error_bound);

                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push(OrbitKktData {
                            sigma: perm.to_vec(),
                            beta: beta.clone(),
                            beta_margin: beta_min,
                            action,
                            action_lower,
                            action_upper,
                            q: q_val,
                            q_error_bound: kkt_result.q_error_bound,
                            mu: Some(
                                kkt_result
                                    .mu
                                    .as_slice()
                                    .try_into()
                                    .expect("closure multiplier must stay 4D"),
                            ),
                            xi: Some(kkt_result.xi),
                            admissibility: OrbitAdmissibility::AdmissibleF64,
                        });
                    }

                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain_action.is_none_or(|a| action < a);
                        if update {
                            best_uncertain_action = Some(action);
                        }
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.action.total_cmp(&b.action));
    let capacity = orbits[0].action;
    let capacity_uncertain = best_uncertain_action.unwrap_or(capacity);

    Some(InstrumentedOrbitSearch {
        capacity,
        capacity_uncertain,
        orbits,
        iterations,
    })
}

/// Exact HKO2024 polytope over `Q[tan(pi/5)]`.
pub fn exact_hko_polytope() -> ExactPolytope4D<TanPiFifthField> {
    let z = TanPiFifthField::zero();
    let one = TanPiFifthField::one();
    let t = TanPiFifthField::generator();
    let t2 = t.clone() * t.clone();
    let t3 = t2.clone() * t.clone();

    let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from_i64(4);
    let b = (TanPiFifthField::from_i64(7) * t.clone() - t3.clone()) / TanPiFifthField::from_i64(4);
    let sec36 = (TanPiFifthField::from_i64(3) - t2.clone()) / TanPiFifthField::from_i64(2);

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
