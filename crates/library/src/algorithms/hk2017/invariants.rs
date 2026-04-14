//! KKT feasibility invariants and conversion helpers for HK2017 search.

use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktResult};
use crate::kkt::{classify_margin, Solution};

pub(super) fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let kkt = solve_kkt_for(polytope, perm).feasible()?;
    Some(kkt_result_to_solution(kkt))
}

fn kkt_result_to_solution(result: KktResult) -> Solution {
    let margin = result.beta.iter().copied().fold(f64::INFINITY, f64::min);
    Solution {
        verdict: classify_margin(margin),
        q: result.q_corrected,
        beta: result.beta,
        margin,
    }
}
