/// Re-export from `geom::known_polytopes`.
///
/// The canonical definitions live in `geom::known_polytopes`. This module
/// re-exports everything so existing `datasets::known_polytopes::*` imports
/// continue to work.
pub use geom::known_polytopes::*;

#[cfg(test)]
#[path = "known_polytopes_test.rs"]
mod known_polytopes_test;
