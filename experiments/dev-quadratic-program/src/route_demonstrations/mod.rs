//! Executable demonstrations of why tempting simpler capacity routes fail.
//!
//! Nobody currently plans to import this code. Future consumers are expected to
//! read and run these tests, then copy-edit the small route fragments if they
//! need a simpler heuristic and do not care about the missing numerical
//! guarantees.
//!
//! Keep each file focused on one simplification or failure class. Do not build
//! a route-by-fixture dashboard here; add an ad-hoc test when a new interaction
//! matters.

mod literal_f64_pruning;
