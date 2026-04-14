//! Bridge from KKT solver output to HK2017 accumulator input.
//!
//! Keeps conversion logic separate from enumeration so capacity loops in `mod.rs`
//! only coordinate traversal and pruning.

use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktResult};
use crate::kkt::{classify_margin, Solution};

/// Solve the KKT system for a (polytope, permutation) pair and convert the
/// result into a `Solution` for the accumulator.
pub(crate) fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let kkt = solve_kkt_for(polytope, perm).feasible()?;
    Some(kkt_result_to_solution(kkt))
}

/// Convert a `KktResult` (saddle-point solver output) to a `Solution` (accumulator input).
///
/// Maps: q_corrected -> q, beta -> beta, min(beta) -> margin, classify_margin -> verdict.
pub(crate) fn kkt_result_to_solution(result: KktResult) -> Solution {
    let margin = result.beta.iter().copied().fold(f64::INFINITY, f64::min);
    Solution {
        verdict: classify_margin(margin),
        q: result.q_corrected,
        beta: result.beta,
        margin,
    }
}
