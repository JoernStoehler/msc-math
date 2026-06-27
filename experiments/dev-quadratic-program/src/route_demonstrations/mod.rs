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
//!
//! Current executable rungs:
//!
//! - `literal_f64_pruning`: literal f64 predicates can silently prune a real
//!   transition and return the wrong capacity.
//! - `conservative_pruning_still_f64`: keeping indeterminate transitions fixes
//!   that pruning miss on the same fixture.
//! - `f64_value_not_certificate`: a correct-looking f64 scalar can still leave
//!   the minimizing set undecided.
//! - `retained_candidate_fallback_limit`: exact fallback over retained
//!   candidates is exact only for the candidate set it receives.
//! - `guarded_route_safe_refusal`: guarded routes should reject or request
//!   fallback rather than inventing a scalar on invalid/ambiguous inputs.
//!
//! The exact-all-visited route still needs a good executable cost
//! demonstration. A timing assertion in a unit test would be brittle; use a
//! performance target or an ignored/manual demo when the needed cost threshold
//! is clear.

mod conservative_pruning_still_f64;
mod f64_value_not_certificate;
mod guarded_route_safe_refusal;
mod literal_f64_pruning;
mod retained_candidate_fallback_limit;
