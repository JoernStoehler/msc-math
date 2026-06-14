use nalgebra::{DMatrix, Vector4};
use std::time::Instant;
use symplectic::algorithms::{
    billiard::for_each_sigma_from_facets, hk2017::for_each_sigma_pruned_by_transition,
};
use symplectic::{
    solve_orbit_sigma_saddle_point, OrbitAdmissibility, OrbitKktData, OrbitSolveError,
};

#[derive(Default)]
pub(crate) struct CandidateSolveSummary {
    pub(crate) iterations: u64,
    pub(crate) kkt_solve_ms: f64,
    pub(crate) inadmissible_count: usize,
    pub(crate) numerical_failure_count: usize,
    admissible: Vec<OrbitKktData>,
    indeterminate: Vec<OrbitKktData>,
}

impl CandidateSolveSummary {
    pub(crate) fn best_admissible(&self) -> Option<&OrbitKktData> {
        self.admissible.first()
    }

    pub(crate) fn admissible_count(&self) -> usize {
        self.admissible.len()
    }

    pub(crate) fn indeterminate_count(&self) -> usize {
        self.indeterminate.len()
    }

    pub(crate) fn action_gap(&self) -> Option<f64> {
        (self.admissible.len() >= 2).then(|| self.admissible[1].action - self.admissible[0].action)
    }

    pub(crate) fn indeterminate_overlaps_best_interval(&self) -> bool {
        self.admissible
            .first()
            .map(|best| {
                self.indeterminate.iter().any(|candidate| {
                    candidate.action_lower <= best.action_upper
                        && best.action_lower <= candidate.action_upper
                })
            })
            .unwrap_or(!self.indeterminate.is_empty())
    }

    fn record_sigma(&mut self, dual_vertices: &[Vector4<f64>], sigma: &[usize]) {
        self.iterations += 1;
        let started = Instant::now();
        let solved = solve_orbit_sigma_saddle_point(dual_vertices, sigma);
        self.kkt_solve_ms += started.elapsed().as_secs_f64() * 1000.0;
        match solved {
            Ok(orbit) => self.record_orbit(orbit),
            Err(OrbitSolveError::Inadmissible) => self.inadmissible_count += 1,
            Err(OrbitSolveError::NumericalFailure) => self.numerical_failure_count += 1,
        }
    }

    fn record_orbit(&mut self, orbit: OrbitKktData) {
        match orbit.admissibility {
            OrbitAdmissibility::AdmissibleF64 => self.admissible.push(orbit),
            OrbitAdmissibility::IndeterminateF64 => self.indeterminate.push(orbit),
            OrbitAdmissibility::AdmissibleExact => {
                unreachable!("pure f64 path calls only the f64 saddle-point solver")
            }
        }
    }

    fn sort_by_action(&mut self) {
        self.admissible
            .sort_by(|a, b| a.action.total_cmp(&b.action));
        self.indeterminate
            .sort_by(|a, b| a.action_lower.total_cmp(&b.action_lower));
    }
}

pub(crate) fn solve_transition_pruned_candidates(
    dual_vertices: &[Vector4<f64>],
    transition_is_allowed: &DMatrix<bool>,
) -> CandidateSolveSummary {
    let mut summary = CandidateSolveSummary::default();
    for_each_sigma_pruned_by_transition(transition_is_allowed, |sigma| {
        summary.record_sigma(dual_vertices, sigma);
    });
    summary.sort_by_action();
    summary
}

pub(crate) fn solve_billiard_candidates_summary(
    dual_vertices: &[Vector4<f64>],
    q_facet_indices: &[usize],
    p_facet_indices: &[usize],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    transition_is_allowed: &DMatrix<bool>,
) -> CandidateSolveSummary {
    let mut summary = CandidateSolveSummary::default();
    for_each_sigma_from_facets(
        q_facet_indices,
        p_facet_indices,
        facet_intersection_is_nonempty,
        transition_is_allowed,
        |sigma| summary.record_sigma(dual_vertices, sigma),
    );
    summary.sort_by_action();
    summary
}
