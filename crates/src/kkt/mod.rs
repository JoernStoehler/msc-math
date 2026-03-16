pub mod qp_assembly;
pub mod saddle_point_solver;
pub mod constraint_solver;
pub mod beta_feasibility;
pub mod projection_solver;
pub mod rational_solver;

#[cfg(test)]
#[path = "qp_assembly_test.rs"]
mod qp_assembly_test;

#[cfg(test)]
#[path = "saddle_point_solver_test.rs"]
mod saddle_point_solver_test;

#[cfg(test)]
#[path = "constraint_solver_test.rs"]
mod constraint_solver_test;

#[cfg(test)]
#[path = "beta_feasibility_test.rs"]
mod beta_feasibility_test;

#[cfg(test)]
#[path = "projection_solver_test.rs"]
mod projection_solver_test;

#[cfg(test)]
#[path = "rational_solver_test.rs"]
mod rational_solver_test;
