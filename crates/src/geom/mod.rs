pub mod polytope;
pub mod skeleton;
pub mod symplectic_form;
pub mod volume;
pub mod polygon;
pub mod lagrangian_product;
pub mod cross_product_4d;
pub mod validation;
pub mod rational_arithmetic;
pub mod vertex_enumeration;
pub mod qhull;
pub mod reeb_trajectory;
pub mod known_polytopes;
pub mod test_utils;

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;

#[cfg(test)]
#[path = "construction_validation_test.rs"]
mod construction_validation_test;

#[cfg(test)]
#[path = "skeleton_test.rs"]
mod skeleton_test;

#[cfg(test)]
#[path = "symplectic_form_test.rs"]
mod symplectic_form_test;

#[cfg(test)]
#[path = "volume_test.rs"]
mod volume_test;

#[cfg(test)]
#[path = "volume_properties_test.rs"]
mod volume_properties_test;

#[cfg(test)]
#[path = "polygon_test.rs"]
mod polygon_test;

#[cfg(test)]
#[path = "lagrangian_product_test.rs"]
mod lagrangian_product_test;

#[cfg(test)]
#[path = "cross_product_4d_test.rs"]
mod cross_product_4d_test;

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;

#[cfg(test)]
#[path = "rational_arithmetic_test.rs"]
mod rational_arithmetic_test;

#[cfg(test)]
#[path = "vertex_enumeration_test.rs"]
mod vertex_enumeration_test;

#[cfg(test)]
#[path = "vertex_enumeration_linalg_test.rs"]
mod vertex_enumeration_linalg_test;

#[cfg(test)]
#[path = "reeb_trajectory_test.rs"]
mod reeb_trajectory_test;
