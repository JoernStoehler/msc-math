//! Bridge between the KKT solver output and capacity accumulator input.

use crate::algorithms::orbit_search::{solve_orbit_sigma, OrbitAdmissibility, OrbitSolveBackend};
use crate::geom::polytope::Polytope4D;
use crate::kkt::{classify_margin, Solution};

/// Solve the KKT system for a `(polytope, permutation)` pair and convert it.
pub(super) fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let orbit = solve_orbit_sigma(polytope, perm, OrbitSolveBackend::SaddlePoint).ok()?;
    let verdict = match orbit.admissibility {
        OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => {
            classify_margin(orbit.beta_margin)
        }
        OrbitAdmissibility::IndeterminateF64 => classify_margin(orbit.beta_margin),
    };
    Some(Solution {
        verdict,
        q: orbit.q,
        beta: orbit.beta,
        margin: orbit.beta_margin,
    })
}
