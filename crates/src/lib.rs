pub mod geom;
pub mod kkt;
pub mod algorithms;
pub mod constants;
pub mod dataset;
pub mod random;

#[cfg(test)]
#[path = "dataset_test.rs"]
mod dataset_test;

#[cfg(test)]
#[path = "random_test.rs"]
mod random_test;
