use crate::{dual_vertices_rational_strings, orbit_scalars_from_result, SysLandscapePolytopeCache};
use symplectic::database::SigmaAction;
use symplectic::OrbitSearchResult;

use super::polytope_key;
use super::rows::{AscentEventRow, ComputedPolytopeRow};

pub struct ComputedPolytopeMeta<'a> {
    pub phase: Option<usize>,
    pub iteration: Option<usize>,
    pub role: &'a str,
    pub step_type: Option<&'a str>,
    pub t_fraction: Option<f64>,
    pub t_actual: Option<f64>,
    pub accepted_in_iteration: bool,
    pub became_run_final: bool,
}

impl<'a> ComputedPolytopeMeta<'a> {
    pub fn role(role: &'a str) -> Self {
        Self {
            phase: None,
            iteration: None,
            role,
            step_type: None,
            t_fraction: None,
            t_actual: None,
            accepted_in_iteration: false,
            became_run_final: false,
        }
    }
}

pub struct ComputedPolytopeRecorder {
    dataset: &'static str,
    run_id: String,
    seed_index: usize,
    next_ordinal: usize,
    rows: Vec<ComputedPolytopeRow>,
    events: Vec<AscentEventRow>,
}

impl ComputedPolytopeRecorder {
    pub fn new(dataset: &'static str, run_id: &str, seed_index: usize) -> Self {
        Self {
            dataset,
            run_id: run_id.to_string(),
            seed_index,
            next_ordinal: 0,
            rows: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        meta: ComputedPolytopeMeta<'_>,
        polytope: &SysLandscapePolytopeCache,
        capacity: &OrbitSearchResult,
        volume: f64,
        sys: f64,
    ) -> usize {
        let result_idx = self.rows.len();
        let result_id = format!("{}:{}:{:06}", self.dataset, self.run_id, self.next_ordinal);
        self.next_ordinal += 1;
        let event_id = result_id.clone();
        let polytope_key = polytope_key(polytope);
        self.rows.push(ComputedPolytopeRow {
            result_id: result_id.clone(),
            dataset: self.dataset.to_string(),
            run_id: self.run_id.clone(),
            seed_index: self.seed_index,
            phase: meta.phase,
            iteration: meta.iteration,
            role: meta.role.to_string(),
            step_type: meta.step_type.map(str::to_string),
            t_fraction: meta.t_fraction,
            t_actual: meta.t_actual,
            accepted_in_iteration: meta.accepted_in_iteration,
            became_run_final: meta.became_run_final,
            dual_vertices_rational: dual_vertices_rational_strings(polytope),
            facet_count: polytope.facet_count(),
            capacity: capacity.capacity(),
            volume,
            sys,
            sigmas: vec![SigmaAction {
                perm: capacity.best_sigma().to_vec(),
                action: capacity.capacity(),
            }],
            orbit_scalars: orbit_scalars_from_result(capacity),
        });
        self.events.push(AscentEventRow {
            event_id,
            dataset: self.dataset.to_string(),
            run_id: self.run_id.clone(),
            seed_index: self.seed_index,
            phase: meta.phase,
            iteration: meta.iteration,
            role: meta.role.to_string(),
            step_type: meta.step_type.map(str::to_string),
            t_fraction: meta.t_fraction,
            t_actual: meta.t_actual,
            accepted_in_iteration: meta.accepted_in_iteration,
            became_run_final: meta.became_run_final,
            polytope_key,
        });
        result_idx
    }

    pub fn mark_accepted(&mut self, result_idx: usize) {
        self.rows[result_idx].role = "line_search_accepted".to_string();
        self.rows[result_idx].accepted_in_iteration = true;
        self.events[result_idx].role = "line_search_accepted".to_string();
        self.events[result_idx].accepted_in_iteration = true;
    }

    pub fn into_outputs(self) -> (Vec<ComputedPolytopeRow>, Vec<AscentEventRow>) {
        (self.rows, self.events)
    }
}
