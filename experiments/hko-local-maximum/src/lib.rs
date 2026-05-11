//! Shared helpers for exp-hko-local-maximum experiments.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

pub mod exact_bank;
pub mod instrumented_search;

pub use exact_bank::{
    exact_hko_dual_vertices, exact_simplex_dual_vertices, ExactBankEntry, ExactBankTarget,
    HkoExactScalar, EXACT_BANK_ENTRIES, HKO_FLOAT_WINNING_SIGMA, HKO_NEAR_OPTIMAL_SIGMA_A,
    HKO_NEAR_OPTIMAL_SIGMA_B, HKO_RANK_DEFICIENT_SIGMA, HKO_WINNING_SIGMA, SIMPLEX_CONTROL_SIGMA,
};
pub use instrumented_search::{ehz_capacity_instrumented, InstrumentedOrbitSearch};

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}
