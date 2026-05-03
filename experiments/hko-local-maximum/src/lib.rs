//! Shared helpers for exp-hko-local-maximum experiments.

pub mod exact_bank;
pub mod instrumented_search;

pub use exact_bank::{
    exact_hko_polytope, exact_simplex_polytope, ExactBankEntry, ExactBankTarget,
    EXACT_BANK_ENTRIES, HKO_FLOAT_WINNING_SIGMA, HKO_NEAR_OPTIMAL_SIGMA_A,
    HKO_NEAR_OPTIMAL_SIGMA_B, HKO_RANK_DEFICIENT_SIGMA, HKO_WINNING_SIGMA, SIMPLEX_CONTROL_SIGMA,
};
pub use instrumented_search::{ehz_capacity_instrumented, InstrumentedOrbitSearch};
