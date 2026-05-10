//! Euclidean convex-polytope helpers for thesis Rust code.
//!
//! This crate is being introduced before the implementation migration so the
//! public target can be reviewed separately from code motion. The current
//! accepted surface is the crate-level documentation in `README.md` and the
//! maintainer plan in `DEVELOPMENT.md`.
//!
//! Scope boundary: this crate is for ordinary Euclidean convex geometry in
//! ambient `R^4`, including lower-dimensional polytopes in affine subspaces of
//! `R^4`. Symplectic forms, Reeb orbits, capacity algorithms, and KKT assembly
//! remain in the `symplectic` crate or experiment-owned code.
