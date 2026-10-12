//! Helpers shared by regular-product experiments.
//!
//! This package is separate from `exp-sys-landscape` because the regular-product
//! side result has different thesis ownership from the hostile-landscape
//! searches.

pub mod capacity;
pub mod paths;
pub mod product_polytope_cache;
pub mod volume;

pub use capacity::{product_minimum, ProductMinimum};
pub use paths::{experiment_path, package_root};
pub use product_polytope_cache::ProductPolytopeCache;
pub use volume::exact_volume_reference_as_f64;
