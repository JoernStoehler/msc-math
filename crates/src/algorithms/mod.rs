pub mod capacity_accumulator;
pub mod facet_adjacency;
pub mod hk2017;
pub mod billiard;
pub mod tube;

#[cfg(test)]
#[path = "capacity_accumulator_test.rs"]
mod capacity_accumulator_test;

#[cfg(test)]
#[path = "facet_adjacency_test.rs"]
mod facet_adjacency_test;
