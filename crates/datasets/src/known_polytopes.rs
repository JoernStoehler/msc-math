/// Re-export from `geom::known_polytopes` (canonical source).
///
/// This module exists to host dataset-specific integration tests
/// (validation checks on known polytopes).
pub use geom::known_polytopes::*;

#[cfg(test)]
#[path = "known_polytopes_test.rs"]
mod known_polytopes_test;
