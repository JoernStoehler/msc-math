pub mod permutations;
pub mod orbit_recovery;
pub mod generate_capacity_fixtures;

#[cfg(test)]
#[path = "permutations_test.rs"]
mod permutations_test;

#[cfg(test)]
#[path = "orbit_recovery_test.rs"]
mod orbit_recovery_test;

#[cfg(test)]
#[path = "literature_test.rs"]
mod literature_test;

#[cfg(test)]
#[path = "kkt_edge_cases_test.rs"]
mod kkt_edge_cases_test;

#[cfg(test)]
#[path = "pruning_test.rs"]
mod pruning_test;

#[cfg(test)]
#[path = "regression_test.rs"]
mod regression_test;

#[cfg(test)]
#[path = "conformality_test.rs"]
mod conformality_test;

#[cfg(test)]
#[path = "symplectic_invariance_test.rs"]
mod symplectic_invariance_test;

#[cfg(test)]
#[path = "capacity_derivative_test.rs"]
mod capacity_derivative_test;
