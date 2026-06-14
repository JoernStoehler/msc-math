//! f64 tube construction and search for the flow-graph algorithm.
//!
//! These routines are development evidence and production-path candidates; they
//! are not exact certificates for `c_EHZ` without the exact fallback wrapper.

use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use crate::algorithms::flow_graph::exact_tube::{
    resolve_closed_word_exact, ExactClosedTubeError, ExactClosedWordOutcome, ExactFlatTubeInput,
};
use crate::algorithms::flow_graph::words::{all_distinct, is_simple_closable_word, plus_depth};
use crate::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use crate::geom::symplectic_form::{j4, omega0};
use nalgebra::{DMatrix, Matrix2, Vector2, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::info;

#[cfg(test)]
use crate::algorithms::flow_graph::words::{
    cached_words_contain, counts_by_plus_depth, enumerate_transition_pruned_words,
    half_cache_depth, split_closed_word_into_half_words, word_has_allowed_transitions,
};
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[cfg(test)]
#[path = "tests_e2e_prediction.rs"]
mod tests_e2e_prediction;

const EPS_DET: f64 = 1e-10;
const EPS_CONTAINS: f64 = 1e-8;
pub const DEFAULT_OMEGA_STABILITY_EPS: f64 = 1e-12;

static POLYGON_IS_EMPTY_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_CONTAINS_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_CONTAINS_HALFSPACE_CHECKS: AtomicU64 = AtomicU64::new(0);
static POLYGON_INTERSECT_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_WITH_HALFSPACE_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_PULLBACK_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_IMAGE_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_VERTICES_CALLS: AtomicU64 = AtomicU64::new(0);
static POLYGON_VERTEX_PAIR_CHECKS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct F64PolygonMetrics {
    pub is_empty_calls: u64,
    pub contains_calls: u64,
    pub contains_halfspace_checks: u64,
    pub intersect_calls: u64,
    pub with_halfspace_calls: u64,
    pub pullback_calls: u64,
    pub image_calls: u64,
    pub vertices_calls: u64,
    pub vertex_pair_checks: u64,
}

pub fn reset_f64_polygon_metrics() {
    for counter in polygon_metric_counters() {
        counter.store(0, Ordering::Relaxed);
    }
}

pub fn take_f64_polygon_metrics() -> F64PolygonMetrics {
    F64PolygonMetrics {
        is_empty_calls: POLYGON_IS_EMPTY_CALLS.swap(0, Ordering::Relaxed),
        contains_calls: POLYGON_CONTAINS_CALLS.swap(0, Ordering::Relaxed),
        contains_halfspace_checks: POLYGON_CONTAINS_HALFSPACE_CHECKS.swap(0, Ordering::Relaxed),
        intersect_calls: POLYGON_INTERSECT_CALLS.swap(0, Ordering::Relaxed),
        with_halfspace_calls: POLYGON_WITH_HALFSPACE_CALLS.swap(0, Ordering::Relaxed),
        pullback_calls: POLYGON_PULLBACK_CALLS.swap(0, Ordering::Relaxed),
        image_calls: POLYGON_IMAGE_CALLS.swap(0, Ordering::Relaxed),
        vertices_calls: POLYGON_VERTICES_CALLS.swap(0, Ordering::Relaxed),
        vertex_pair_checks: POLYGON_VERTEX_PAIR_CHECKS.swap(0, Ordering::Relaxed),
    }
}

fn polygon_metric_counters() -> [&'static AtomicU64; 9] {
    [
        &POLYGON_IS_EMPTY_CALLS,
        &POLYGON_CONTAINS_CALLS,
        &POLYGON_CONTAINS_HALFSPACE_CHECKS,
        &POLYGON_INTERSECT_CALLS,
        &POLYGON_WITH_HALFSPACE_CALLS,
        &POLYGON_PULLBACK_CALLS,
        &POLYGON_IMAGE_CALLS,
        &POLYGON_VERTICES_CALLS,
        &POLYGON_VERTEX_PAIR_CHECKS,
    ]
}

/// f64 tube construction errors.
///
/// These are numerical construction outcomes, not certified exact outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F64TubeError {
    InvalidFacet,
    InvalidWord,
    InvalidCutoff,
    UnsupportedZeroOmegaTransition,
    NumericallyUnstableOmegaTransition,
    SingularFaceFrame,
    UnsupportedDegenerateTransition,
    IncompatibleTubes,
    SingularTubeMap,
    SingularFixedPointMap,
    NumericallyIndeterminatePolygon,
    NoOrbit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F64Predicate {
    True,
    False,
    Indeterminate,
}

/// Flat f64 polytope inputs for tube construction.
///
/// The exact pipeline should produce `facet_intersection_is_nonempty` and
/// `omega_signs`; this f64 struct only consumes them.
#[derive(Clone, Copy, Debug)]
pub struct FlatTubeInput<'a> {
    pub dual_vertices: &'a [Vector4<f64>],
    pub facet_intersection_is_nonempty: &'a DMatrix<bool>,
    pub omega_signs: &'a DMatrix<i8>,
}

impl<'a> FlatTubeInput<'a> {
    pub fn new(
        dual_vertices: &'a [Vector4<f64>],
        facet_intersection_is_nonempty: &'a DMatrix<bool>,
        omega_signs: &'a DMatrix<i8>,
    ) -> Self {
        assert_eq!(
            facet_intersection_is_nonempty.shape(),
            omega_signs.shape(),
            "facet_intersection_is_nonempty and omega_signs must have the same shape"
        );
        assert_eq!(
            dual_vertices.len(),
            facet_intersection_is_nonempty.nrows(),
            "one matrix row/column is required per dual vertex"
        );
        assert_eq!(
            facet_intersection_is_nonempty.nrows(),
            facet_intersection_is_nonempty.ncols(),
            "facet_intersection_is_nonempty must be square"
        );
        Self {
            dual_vertices,
            facet_intersection_is_nonempty,
            omega_signs,
        }
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }

    /// Reject polytopes outside the generic affine-tube input class.
    ///
    /// The primitive tube formula treats transitions as affine maps. If a
    /// geometrically possible two-face has exact omega sign zero, that primitive
    /// becomes relation-valued/free-time instead of a single affine map.
    pub fn validate_no_geometric_zero_omega_transitions(&self) -> Result<(), F64TubeError> {
        for i in 0..self.facet_count() {
            for j in 0..self.facet_count() {
                if self.facet_intersection_is_nonempty[(i, j)] && self.omega_signs[(i, j)] == 0 {
                    return Err(F64TubeError::UnsupportedZeroOmegaTransition);
                }
            }
        }
        Ok(())
    }

    /// Reject geometrically possible transitions whose f64 omega is too small.
    ///
    /// The f64 primitive formulas divide by omega. Near-zero values are rejected
    /// instead of tracked with large relative error bounds.
    pub fn validate_f64_omega_stability(&self, min_abs_omega: f64) -> Result<(), F64TubeError> {
        if !min_abs_omega.is_finite() || min_abs_omega < 0.0 {
            return Err(F64TubeError::InvalidCutoff);
        }
        for i in 0..self.facet_count() {
            for j in 0..self.facet_count() {
                if self.facet_intersection_is_nonempty[(i, j)]
                    && omega0(&self.dual_vertices[i], &self.dual_vertices[j]).abs() < min_abs_omega
                {
                    return Err(F64TubeError::NumericallyUnstableOmegaTransition);
                }
            }
        }
        Ok(())
    }
}

/// Two-dimensional affine coordinate chart for `H_i cap H_j`.
#[derive(Clone, Debug, PartialEq)]
pub struct FaceFrame {
    first: usize,
    second: usize,
    base: Vector4<f64>,
    u: Vector4<f64>,
    v: Vector4<f64>,
}

impl FaceFrame {
    pub fn point(&self, coords: Vector2<f64>) -> Vector4<f64> {
        self.base + self.u * coords[0] + self.v * coords[1]
    }

    pub fn coords(&self, point: &Vector4<f64>) -> Vector2<f64> {
        let delta = point - self.base;
        Vector2::new(self.u.dot(&delta), self.v.dot(&delta))
    }

    pub fn linear_coords(&self, vector: &Vector4<f64>) -> Vector2<f64> {
        Vector2::new(self.u.dot(vector), self.v.dot(vector))
    }

    pub fn pair(&self) -> (usize, usize) {
        (self.first, self.second)
    }
}

/// Closed halfspace `normal . x <= rhs` in local two-face coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct Halfspace2 {
    normal: Vector2<f64>,
    rhs: f64,
}

impl Halfspace2 {
    pub fn new(normal: Vector2<f64>, rhs: f64) -> Self {
        Self { normal, rhs }
    }
}

/// f64 polygon represented as an intersection of closed halfspaces.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2 {
    inequalities: Vec<Halfspace2>,
}

impl Polygon2 {
    pub fn new(inequalities: Vec<Halfspace2>) -> Self {
        Self { inequalities }
    }

    pub fn inequality_count(&self) -> usize {
        self.inequalities.len()
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.is_empty_trinary(), F64Predicate::True)
    }

    pub fn is_empty_trinary(&self) -> F64Predicate {
        POLYGON_IS_EMPTY_CALLS.fetch_add(1, Ordering::Relaxed);
        let candidates = self.near_feasible_boundary_candidates();
        if candidates.is_empty() {
            return F64Predicate::True;
        }

        let centroid = candidates.iter().copied().sum::<Vector2<f64>>() / candidates.len() as f64;
        if self.contains_trinary(&centroid) == F64Predicate::True {
            F64Predicate::False
        } else {
            F64Predicate::Indeterminate
        }
    }

    pub fn contains(&self, point: &Vector2<f64>) -> bool {
        self.max_normalized_violation(point).unwrap_or(0.0) <= EPS_CONTAINS
    }

    pub fn contains_trinary(&self, point: &Vector2<f64>) -> F64Predicate {
        POLYGON_CONTAINS_CALLS.fetch_add(1, Ordering::Relaxed);
        POLYGON_CONTAINS_HALFSPACE_CHECKS
            .fetch_add(self.inequalities.len() as u64, Ordering::Relaxed);
        match self.max_normalized_violation(point) {
            None => F64Predicate::True,
            Some(max_violation) if max_violation <= -EPS_CONTAINS => F64Predicate::True,
            Some(max_violation) if max_violation > EPS_CONTAINS => F64Predicate::False,
            Some(_) => F64Predicate::Indeterminate,
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        POLYGON_INTERSECT_CALLS.fetch_add(1, Ordering::Relaxed);
        let mut inequalities = self.inequalities.clone();
        inequalities.extend(other.inequalities.iter().cloned());
        Self::new(inequalities)
    }

    pub fn with_halfspace(&self, halfspace: Halfspace2) -> Self {
        POLYGON_WITH_HALFSPACE_CALLS.fetch_add(1, Ordering::Relaxed);
        let mut inequalities = self.inequalities.clone();
        inequalities.push(halfspace);
        Self::new(inequalities)
    }

    pub fn pullback(&self, affine: &Affine2) -> Self {
        POLYGON_PULLBACK_CALLS.fetch_add(1, Ordering::Relaxed);
        Self::new(
            self.inequalities
                .iter()
                .map(|h| {
                    Halfspace2::new(
                        affine.matrix.transpose() * h.normal,
                        h.rhs - h.normal.dot(&affine.offset),
                    )
                })
                .collect(),
        )
    }

    pub fn image_under(&self, affine: &Affine2) -> Result<Self, F64TubeError> {
        POLYGON_IMAGE_CALLS.fetch_add(1, Ordering::Relaxed);
        let inverse = affine
            .matrix
            .try_inverse()
            .ok_or(F64TubeError::SingularTubeMap)?;
        Ok(Self::new(
            self.inequalities
                .iter()
                .map(|h| {
                    Halfspace2::new(
                        inverse.transpose() * h.normal,
                        h.rhs + h.normal.dot(&(inverse * affine.offset)),
                    )
                })
                .collect(),
        ))
    }

    pub fn vertices(&self) -> Vec<Vector2<f64>> {
        POLYGON_VERTICES_CALLS.fetch_add(1, Ordering::Relaxed);
        self.near_feasible_boundary_candidates()
    }

    fn near_feasible_boundary_candidates(&self) -> Vec<Vector2<f64>> {
        let mut vertices = Vec::new();
        for i in 0..self.inequalities.len() {
            for j in (i + 1)..self.inequalities.len() {
                POLYGON_VERTEX_PAIR_CHECKS.fetch_add(1, Ordering::Relaxed);
                let a = &self.inequalities[i];
                let b = &self.inequalities[j];
                let det = det2(a.normal[0], a.normal[1], b.normal[0], b.normal[1]);
                if det.abs() < EPS_DET {
                    continue;
                }
                let x = (a.rhs * b.normal[1] - a.normal[1] * b.rhs) / det;
                let y = (a.normal[0] * b.rhs - a.rhs * b.normal[0]) / det;
                let candidate = Vector2::new(x, y);
                if self.contains(&candidate)
                    && !vertices
                        .iter()
                        .any(|seen: &Vector2<f64>| (seen - candidate).norm() < EPS_CONTAINS)
                {
                    vertices.push(candidate);
                }
            }
        }
        vertices
    }

    fn max_normalized_violation(&self, point: &Vector2<f64>) -> Option<f64> {
        let mut max_violation = f64::NEG_INFINITY;
        for h in &self.inequalities {
            let normal_norm = h.normal.norm();
            if normal_norm <= EPS_DET {
                if h.rhs < -EPS_CONTAINS {
                    return Some(f64::INFINITY);
                }
                continue;
            }
            max_violation = max_violation.max((h.normal.dot(point) - h.rhs) / normal_norm);
        }
        max_violation.is_finite().then_some(max_violation)
    }
}

/// Local affine map `x -> matrix * x + offset`.
#[derive(Clone, Debug, PartialEq)]
pub struct Affine2 {
    matrix: Matrix2<f64>,
    offset: Vector2<f64>,
}

impl Affine2 {
    pub fn identity() -> Self {
        Self {
            matrix: Matrix2::identity(),
            offset: Vector2::zeros(),
        }
    }

    pub fn apply(&self, point: Vector2<f64>) -> Vector2<f64> {
        self.matrix * point + self.offset
    }

    pub fn inverse(&self) -> Result<Self, F64TubeError> {
        if self.matrix.determinant().abs() < EPS_DET {
            return Err(F64TubeError::SingularTubeMap);
        }
        let matrix = self
            .matrix
            .try_inverse()
            .ok_or(F64TubeError::SingularTubeMap)?;
        Ok(Self {
            offset: -(matrix * self.offset),
            matrix,
        })
    }

    pub fn then(&self, next: &Self) -> Self {
        Self {
            matrix: next.matrix * self.matrix,
            offset: next.matrix * self.offset + next.offset,
        }
    }
}

/// Affine scalar function `x -> coeff . x + constant`.
#[derive(Clone, Debug, PartialEq)]
pub struct AffineScalar2 {
    coeff: Vector2<f64>,
    constant: f64,
}

impl AffineScalar2 {
    pub fn evaluate(&self, point: Vector2<f64>) -> f64 {
        self.coeff.dot(&point) + self.constant
    }

    pub fn pullback(&self, affine: &Affine2) -> Self {
        Self {
            coeff: affine.matrix.transpose() * self.coeff,
            constant: self.coeff.dot(&affine.offset) + self.constant,
        }
    }
}

/// Redundant f64 tube data for debugging and validation.
#[derive(Clone, Debug, PartialEq)]
pub struct F64Tube {
    sequence: Vec<usize>,
    start_frame: FaceFrame,
    end_frame: FaceFrame,
    start_polygon: Polygon2,
    end_polygon: Polygon2,
    start_to_end: Affine2,
    end_to_start: Affine2,
    action_on_start: AffineScalar2,
    action_on_end: AffineScalar2,
    cutoff: f64,
}

impl F64Tube {
    pub fn sequence(&self) -> &[usize] {
        &self.sequence
    }

    pub fn start_polygon(&self) -> &Polygon2 {
        &self.start_polygon
    }

    pub fn end_polygon(&self) -> &Polygon2 {
        &self.end_polygon
    }

    pub fn is_empty(&self) -> bool {
        self.start_polygon.is_empty()
    }

    pub fn is_empty_trinary(&self) -> F64Predicate {
        self.start_polygon.is_empty_trinary()
    }

    pub fn action_at_start(&self, point: Vector2<f64>) -> f64 {
        self.action_on_start.evaluate(point)
    }

    pub fn action_at_end(&self, point: Vector2<f64>) -> f64 {
        self.action_on_end.evaluate(point)
    }

    pub fn start_pair(&self) -> (usize, usize) {
        self.start_frame.pair()
    }

    pub fn end_pair(&self) -> (usize, usize) {
        self.end_frame.pair()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct F64ClosedOrbit {
    pub facets: Vec<usize>,
    pub action: f64,
    pub breakpoints: Vec<Vector4<f64>>,
    pub segment_times: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolvedClosedOrbitSource {
    F64,
    Exact,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedClosedOrbit {
    pub facets: Vec<usize>,
    pub action: f64,
    pub source: ResolvedClosedOrbitSource,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapacityF64Result {
    pub capacity_action: f64,
    pub action_threshold: f64,
    pub orbits: Vec<ResolvedClosedOrbit>,
    pub diagnostic: F64FlowGraphSearchResult,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CapacityF64Error {
    Numerical(F64TubeError),
    ExactClosedWord {
        sigma: Vec<usize>,
        error: ExactClosedTubeError,
    },
    ExactUnsupportedPositiveSingular {
        sigma: Vec<usize>,
        singular_status: &'static str,
        min_action: Option<BigRational>,
        max_action: Option<BigRational>,
    },
    ExactActionNotRepresentable {
        sigma: Vec<usize>,
        action: BigRational,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F64ClosedCycleErrorStep {
    BuildClosedTube,
    SolveClosedTube,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F64ClosedCycleError {
    pub step: F64ClosedCycleErrorStep,
    pub error: F64TubeError,
}

#[derive(Clone, Debug, PartialEq)]
pub enum F64ClosedCycleOutcome {
    Orbit(F64ClosedOrbit),
    NoOrbit,
    Error(F64ClosedCycleError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct F64ClosedCycleRecord {
    pub sigma: Vec<usize>,
    pub outcome: F64ClosedCycleOutcome,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FacePolygonSnapshot {
    pub pair: [usize; 2],
    pub vertices: Vec<[f64; 2]>,
    pub inequalities: Vec<HalfspaceSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HalfspaceSnapshot {
    pub normal: [f64; 2],
    pub rhs: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AffineSnapshot {
    pub matrix: [[f64; 2]; 2],
    pub offset: [f64; 2],
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AffineScalarSnapshot {
    pub coeff: [f64; 2],
    pub constant: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TubeFaceSnapshot {
    pub pair: [usize; 2],
    pub polygon: FacePolygonSnapshot,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TubeFaceFixedPointSnapshot {
    pub pair: [usize; 2],
    pub role: String,
    pub point: Option<[f64; 2]>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FixedPointSnapshot {
    pub status: String,
    pub point: Option<[f64; 2]>,
    pub line_point: Option<[f64; 2]>,
    pub line_direction: Option<[f64; 2]>,
    pub action: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TubeVisualizationSnapshot {
    pub sequence: Vec<usize>,
    pub start_pair: [usize; 2],
    pub end_pair: [usize; 2],
    pub start_polygon: FacePolygonSnapshot,
    pub end_polygon: FacePolygonSnapshot,
    pub intermediate_polygons: Vec<TubeFaceSnapshot>,
    pub fixed_points_on_faces: Vec<TubeFaceFixedPointSnapshot>,
    pub start_to_end: AffineSnapshot,
    pub end_to_start: AffineSnapshot,
    pub action_on_start: AffineScalarSnapshot,
    pub action_on_end: AffineScalarSnapshot,
    pub fixed_point: FixedPointSnapshot,
    pub cutoff: f64,
}

/// f64 closed-tube search result.
///
/// This is an implementation work surface, not a certified capacity result.
/// no `F64ClosedCycleOutcome::Error` records is the current caller-side condition for
/// treating `best_action` as the f64 flow-graph candidate on the searched polytope.
#[derive(Clone, Debug, PartialEq)]
pub struct F64FlowGraphSearchResult {
    pub best_action: Option<f64>,
    pub orbits: Vec<F64ClosedOrbit>,
    pub closed_cycles: Vec<F64ClosedCycleRecord>,
}

impl F64FlowGraphSearchResult {
    fn new() -> Self {
        Self {
            best_action: None,
            orbits: Vec::new(),
            closed_cycles: Vec::new(),
        }
    }

    pub fn checked_closed_word_count(&self) -> usize {
        self.closed_cycles.len()
    }

    pub fn no_orbit_count(&self) -> usize {
        self.closed_cycles
            .iter()
            .filter(|record| matches!(record.outcome, F64ClosedCycleOutcome::NoOrbit))
            .count()
    }

    pub fn closed_cycle_error_count(&self) -> usize {
        self.closed_cycles
            .iter()
            .filter(|record| matches!(record.outcome, F64ClosedCycleOutcome::Error(_)))
            .count()
    }

    pub fn has_closed_cycle_errors(&self) -> bool {
        self.closed_cycle_error_count() > 0
    }

    pub fn first_closed_cycle_error(&self) -> Option<F64ClosedCycleError> {
        self.closed_cycles
            .iter()
            .find_map(|record| match record.outcome {
                F64ClosedCycleOutcome::Error(error) => Some(error),
                F64ClosedCycleOutcome::Orbit(_) | F64ClosedCycleOutcome::NoOrbit => None,
            })
    }

    fn action_cutoff(&self, threshold: f64) -> f64 {
        self.best_action
            .map(|best| best + threshold)
            .unwrap_or(f64::INFINITY)
    }

    fn push_orbit(&mut self, orbit: F64ClosedOrbit, threshold: f64) {
        match self.best_action {
            Some(best) if orbit.action + EPS_CONTAINS < best => {
                self.best_action = Some(orbit.action);
                self.orbits
                    .retain(|old| old.action <= orbit.action + threshold + EPS_CONTAINS);
                self.orbits.push(orbit);
            }
            Some(best) if orbit.action <= best + threshold + EPS_CONTAINS => {
                self.orbits.push(orbit);
            }
            None => {
                self.best_action = Some(orbit.action);
                self.orbits.push(orbit);
            }
            _ => {}
        }
    }

    fn push_closed_cycle(&mut self, sigma: &[usize], outcome: F64ClosedCycleOutcome) {
        self.closed_cycles.push(F64ClosedCycleRecord {
            sigma: sigma.to_vec(),
            outcome,
        });
    }

    fn push_closed_cycle_error(
        &mut self,
        sigma: &[usize],
        step: F64ClosedCycleErrorStep,
        error: F64TubeError,
    ) {
        match error {
            F64TubeError::InvalidFacet
            | F64TubeError::InvalidWord
            | F64TubeError::InvalidCutoff
            | F64TubeError::IncompatibleTubes => {
                panic!("flow-graph search generated invalid closed cycle {sigma:?}: {error:?}")
            }
            F64TubeError::UnsupportedZeroOmegaTransition => {
                panic!(
                    "polytope-level zero-omega validation was bypassed for closed cycle {sigma:?}"
                )
            }
            F64TubeError::NumericallyUnstableOmegaTransition
            | F64TubeError::SingularFaceFrame
            | F64TubeError::UnsupportedDegenerateTransition
            | F64TubeError::SingularTubeMap
            | F64TubeError::SingularFixedPointMap
            | F64TubeError::NumericallyIndeterminatePolygon
            | F64TubeError::NoOrbit => {
                self.push_closed_cycle(
                    sigma,
                    F64ClosedCycleOutcome::Error(F64ClosedCycleError { step, error }),
                );
            }
        }
    }
}

pub fn primitive_tube_f64(
    input: &FlatTubeInput<'_>,
    facets: [usize; 3],
    cutoff: f64,
) -> Result<F64Tube, F64TubeError> {
    validate_cutoff(cutoff)?;
    validate_facets(input, &facets)?;

    let [previous, current, next] = facets;
    let start_frame = face_frame(input, previous, current)?;
    let end_frame = face_frame(input, current, next)?;
    let start_face = face_polygon(input, &start_frame);

    if !input.facet_intersection_is_nonempty[(previous, current)]
        || !input.facet_intersection_is_nonempty[(current, next)]
    {
        return empty_primitive(input, facets, cutoff);
    }
    if omega0(
        &input.dual_vertices[previous],
        &input.dual_vertices[current],
    )
    .abs()
        < DEFAULT_OMEGA_STABILITY_EPS
        || omega0(&input.dual_vertices[current], &input.dual_vertices[next]).abs()
            < DEFAULT_OMEGA_STABILITY_EPS
    {
        return Err(F64TubeError::NumericallyUnstableOmegaTransition);
    }
    if input.omega_signs[(previous, current)] < 0 || input.omega_signs[(current, next)] < 0 {
        return empty_primitive(input, facets, cutoff);
    }

    let duals = input.dual_vertices;
    let reeb = 2.0 * (j4() * duals[current]);
    let denom = duals[next].dot(&reeb);
    if denom.abs() < EPS_DET {
        if face_function_supremum(&start_face, &start_frame, &duals[next]) < 1.0 - EPS_CONTAINS {
            return empty_primitive(input, facets, cutoff);
        }
        return Err(F64TubeError::UnsupportedDegenerateTransition);
    }

    let tau_coeff = Vector2::new(
        -duals[next].dot(&start_frame.u),
        -duals[next].dot(&start_frame.v),
    ) / denom;
    let tau_const = (1.0 - duals[next].dot(&start_frame.base)) / denom;
    let start_to_end = primitive_affine_map(&start_frame, &end_frame, reeb, tau_coeff, tau_const);
    let end_face = face_polygon(input, &end_frame);
    let mut start_polygon = start_face.intersect(&end_face.pullback(&start_to_end));
    start_polygon = start_polygon.with_halfspace(Halfspace2::new(-tau_coeff, tau_const));
    if cutoff.is_finite() {
        start_polygon =
            start_polygon.with_halfspace(Halfspace2::new(tau_coeff, cutoff - tau_const));
    }
    match start_polygon.is_empty_trinary() {
        F64Predicate::True => return empty_primitive(input, facets, cutoff),
        F64Predicate::False => {}
        F64Predicate::Indeterminate => return Err(F64TubeError::NumericallyIndeterminatePolygon),
    }
    if input.omega_signs[(previous, current)] == 0 {
        return Err(F64TubeError::UnsupportedDegenerateTransition);
    }

    tube_from_parts(
        facets.to_vec(),
        start_frame,
        end_frame,
        start_polygon,
        start_to_end,
        AffineScalar2 {
            coeff: tau_coeff,
            constant: tau_const,
        },
        cutoff,
    )
}

pub fn intersect_tubes_f64(first: &F64Tube, second: &F64Tube) -> Result<F64Tube, F64TubeError> {
    if first.end_pair() != second.start_pair() {
        return Err(F64TubeError::IncompatibleTubes);
    }
    let start_to_end = first.start_to_end.then(&second.start_to_end);
    let start_polygon = first
        .start_polygon
        .intersect(&second.start_polygon.pullback(&first.start_to_end));
    let action_on_start = AffineScalar2 {
        coeff: first.action_on_start.coeff
            + first.start_to_end.matrix.transpose() * second.action_on_start.coeff,
        constant: first.action_on_start.constant
            + second.action_on_start.coeff.dot(&first.start_to_end.offset)
            + second.action_on_start.constant,
    };
    let cutoff = first.cutoff.min(second.cutoff);
    let mut sequence = first.sequence.clone();
    sequence.extend(second.sequence.iter().skip(2).copied());

    tube_from_parts(
        sequence,
        first.start_frame.clone(),
        second.end_frame.clone(),
        start_polygon,
        start_to_end,
        action_on_start,
        cutoff,
    )
}

/// Build one f64 tube by direct primitive construction or middle splitting.
///
/// Returns `Ok(None)` when the f64 geometry proves the tube empty. Returned
/// errors are f64 construction failures; they are not exact impossibility
/// certificates and must not be counted as empty tubes.
pub fn build_tube_for_word_f64(
    input: &FlatTubeInput<'_>,
    word: &[usize],
    cutoff: f64,
) -> Result<Option<F64Tube>, F64TubeError> {
    validate_cutoff(cutoff)?;
    validate_facets(input, word)?;
    if !is_simple_closable_word(word) {
        return Err(F64TubeError::InvalidWord);
    }
    match word.len() {
        0..=2 => Err(F64TubeError::InvalidWord),
        3 => {
            let tube = primitive_tube_f64(input, [word[0], word[1], word[2]], cutoff)?;
            match tube.is_empty_trinary() {
                F64Predicate::True => Ok(None),
                F64Predicate::False => Ok(Some(tube)),
                F64Predicate::Indeterminate => Err(F64TubeError::NumericallyIndeterminatePolygon),
            }
        }
        _ => {
            let total_plus = plus_depth(word).expect("word length checked");
            let left_plus = total_plus / 2;
            if left_plus == 0 || left_plus == total_plus {
                return Err(F64TubeError::InvalidWord);
            }
            let left_len = left_plus + 2;
            let right_start = left_len - 2;
            let Some(left) = build_tube_for_word_f64(input, &word[..left_len], cutoff)? else {
                return Ok(None);
            };
            let Some(right) = build_tube_for_word_f64(input, &word[right_start..], cutoff)? else {
                return Ok(None);
            };
            let tube = intersect_tubes_f64(&left, &right)?;
            match tube.is_empty_trinary() {
                F64Predicate::True => Ok(None),
                F64Predicate::False => Ok(Some(tube)),
                F64Predicate::Indeterminate => Err(F64TubeError::NumericallyIndeterminatePolygon),
            }
        }
    }
}

pub fn closed_tube_for_sigma_f64(
    input: &FlatTubeInput<'_>,
    sigma: &[usize],
    cutoff: f64,
) -> Result<Option<F64Tube>, F64TubeError> {
    if sigma.len() < 2 || !all_distinct(sigma) {
        return Err(F64TubeError::InvalidWord);
    }
    let mut word = sigma.to_vec();
    word.push(sigma[0]);
    word.push(sigma[1]);
    build_tube_for_word_f64(input, &word, cutoff)
}

/// Enumerate transition-pruned simple closed words and solve their f64 tubes.
///
/// This is a development and experiment path. It rejects the current input
/// genericity and f64 stability failures before enumeration. Per-cycle f64
/// construction/solving errors are recorded as `F64ClosedCycleOutcome::Error`;
/// generated invalid words/facets/cutoffs are programmer bugs and panic at the
/// validation boundary.
pub fn diagnose_f64_closed_words(
    input: &FlatTubeInput<'_>,
    action_threshold: f64,
) -> Result<F64FlowGraphSearchResult, F64TubeError> {
    if !action_threshold.is_finite() || action_threshold < 0.0 {
        return Err(F64TubeError::InvalidCutoff);
    }
    input.validate_no_geometric_zero_omega_transitions()?;
    input.validate_f64_omega_stability(DEFAULT_OMEGA_STABILITY_EPS)?;

    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        input.facet_intersection_is_nonempty,
        input.omega_signs,
    );
    let mut result = F64FlowGraphSearchResult::new();

    for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
        let cutoff = result.action_cutoff(action_threshold);
        match closed_tube_for_sigma_f64(input, sigma, cutoff) {
            Ok(Some(tube)) => match solve_closed_tube_f64(input, &tube) {
                Ok(Some(orbit)) => {
                    result.push_orbit(orbit.clone(), action_threshold);
                    result.push_closed_cycle(sigma, F64ClosedCycleOutcome::Orbit(orbit));
                }
                Ok(None) => result.push_closed_cycle(sigma, F64ClosedCycleOutcome::NoOrbit),
                Err(error) => result.push_closed_cycle_error(
                    sigma,
                    F64ClosedCycleErrorStep::SolveClosedTube,
                    error,
                ),
            },
            Ok(None) => result.push_closed_cycle(sigma, F64ClosedCycleOutcome::NoOrbit),
            Err(error) => result.push_closed_cycle_error(
                sigma,
                F64ClosedCycleErrorStep::BuildClosedTube,
                error,
            ),
        }
    });

    result
        .orbits
        .sort_by(|left, right| left.action.total_cmp(&right.action));
    Ok(result)
}

pub fn capacity_f64(
    input: &FlatTubeInput<'_>,
    exact_input: &ExactFlatTubeInput<'_>,
    action_threshold: f64,
) -> Result<CapacityF64Result, CapacityF64Error> {
    if !action_threshold.is_finite() || action_threshold < 0.0 {
        return Err(CapacityF64Error::Numerical(F64TubeError::InvalidCutoff));
    }
    let diagnostic =
        diagnose_f64_closed_words(input, action_threshold).map_err(CapacityF64Error::Numerical)?;
    let mut orbits: Vec<ResolvedClosedOrbit> = diagnostic
        .orbits
        .iter()
        .map(|orbit| ResolvedClosedOrbit {
            facets: orbit.facets.clone(),
            action: orbit.action,
            source: ResolvedClosedOrbitSource::F64,
        })
        .collect();

    for record in &diagnostic.closed_cycles {
        let F64ClosedCycleOutcome::Error(_) = record.outcome else {
            continue;
        };
        let (result, _) =
            resolve_closed_word_exact(exact_input, &record.sigma).map_err(|error| {
                CapacityF64Error::ExactClosedWord {
                    sigma: record.sigma.clone(),
                    error,
                }
            })?;
        match result.outcome {
            ExactClosedWordOutcome::EmptyTube
            | ExactClosedWordOutcome::ZeroActionNoOrbit { .. } => {}
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                let action_f64 = action.to_f64().ok_or_else(|| {
                    CapacityF64Error::ExactActionNotRepresentable {
                        sigma: record.sigma.clone(),
                        action: action.clone(),
                    }
                })?;
                orbits.push(ResolvedClosedOrbit {
                    facets: record.sigma.clone(),
                    action: action_f64,
                    source: ResolvedClosedOrbitSource::Exact,
                });
            }
            ExactClosedWordOutcome::UnsupportedPositiveSingular {
                singular_status,
                min_action,
                max_action,
            } => {
                return Err(CapacityF64Error::ExactUnsupportedPositiveSingular {
                    sigma: record.sigma.clone(),
                    singular_status,
                    min_action,
                    max_action,
                });
            }
        }
    }

    let capacity_action = orbits
        .iter()
        .map(|orbit| orbit.action)
        .min_by(f64::total_cmp)
        .expect("flow-graph invariant failed: capacity_f64 found no orbit with action > 0");
    let action_cutoff = capacity_action + action_threshold;
    orbits.retain(|orbit| orbit.action <= action_cutoff + EPS_CONTAINS);
    orbits.sort_by(|left, right| left.action.total_cmp(&right.action));

    Ok(CapacityF64Result {
        capacity_action,
        action_threshold,
        orbits,
        diagnostic,
    })
}

pub fn solve_closed_tube_f64(
    input: &FlatTubeInput<'_>,
    tube: &F64Tube,
) -> Result<Option<F64ClosedOrbit>, F64TubeError> {
    if tube.start_pair() != tube.end_pair() {
        return Ok(None);
    }
    match tube.is_empty_trinary() {
        F64Predicate::True => return Ok(None),
        F64Predicate::False => {}
        F64Predicate::Indeterminate => return Err(F64TubeError::NumericallyIndeterminatePolygon),
    }
    let lhs = tube.start_to_end.matrix - Matrix2::identity();
    let lhs_det = lhs.determinant();
    if lhs_det.abs() < EPS_DET {
        let diagnostic = singular_fixed_point_diagnostic(&lhs, &tube.start_to_end.offset);
        info!(
            sequence = ?tube.sequence,
            lhs_det,
            rhs_norm = diagnostic.rhs_norm,
            first_column_norm = diagnostic.first_column_norm,
            second_column_norm = diagnostic.second_column_norm,
            null_norm = diagnostic.null_norm,
            null_residual = diagnostic.null_residual,
            inconsistency_threshold = diagnostic.inconsistency_threshold,
            inconsistent = diagnostic.inconsistent,
            cause = diagnostic.cause,
            "flow_graph_singular_fixed_point"
        );
        if diagnostic.inconsistent {
            return Ok(None);
        }
        let overlap = tube.start_polygon.intersect(tube.end_polygon());
        if matches!(overlap.is_empty_trinary(), F64Predicate::True) {
            return Ok(None);
        }
        return Err(F64TubeError::SingularFixedPointMap);
    }
    let start_coords = lhs
        .try_inverse()
        .ok_or(F64TubeError::SingularFixedPointMap)?
        * (-tube.start_to_end.offset);
    match tube.start_polygon.contains_trinary(&start_coords) {
        F64Predicate::True => {}
        F64Predicate::False => return Ok(None),
        F64Predicate::Indeterminate => return Err(F64TubeError::NumericallyIndeterminatePolygon),
    }
    let action = tube.action_at_start(start_coords);
    if !action.is_finite() || action <= EPS_CONTAINS || action > tube.cutoff + EPS_CONTAINS {
        return Ok(None);
    }
    let cyclic_word = &tube.sequence[..tube.sequence.len() - 2];
    let start_point = tube.start_frame.point(start_coords);
    orbit_from_cyclic_word_f64(input, cyclic_word, start_point, action)
}

pub fn face_polygon_snapshot_f64(
    input: &FlatTubeInput<'_>,
    first: usize,
    second: usize,
) -> Result<FacePolygonSnapshot, F64TubeError> {
    let frame = face_frame(input, first, second)?;
    Ok(polygon_snapshot(
        [first, second],
        &face_polygon(input, &frame),
    ))
}

pub fn closed_tube_visualization_snapshot_f64(
    input: &FlatTubeInput<'_>,
    sigma: &[usize],
    cutoff: f64,
) -> Result<Option<TubeVisualizationSnapshot>, F64TubeError> {
    let Some(tube) = closed_tube_for_sigma_f64(input, sigma, cutoff)? else {
        return Ok(None);
    };
    Ok(Some(tube_visualization_snapshot(input, &tube)?))
}

fn tube_visualization_snapshot(
    input: &FlatTubeInput<'_>,
    tube: &F64Tube,
) -> Result<TubeVisualizationSnapshot, F64TubeError> {
    let mut intermediate_polygons = Vec::new();
    let sequence = tube.sequence();
    for index in 0..sequence.len() - 1 {
        let pair = [sequence[index], sequence[index + 1]];
        let polygon = if index == 0 {
            tube.start_polygon.clone()
        } else if index == sequence.len() - 2 {
            tube.end_polygon.clone()
        } else {
            let prefix = build_tube_for_word_f64(input, &sequence[..index + 2], tube.cutoff)?
                .ok_or(F64TubeError::NoOrbit)?;
            tube.start_polygon.image_under(&prefix.start_to_end)?
        };
        intermediate_polygons.push(TubeFaceSnapshot {
            pair,
            polygon: polygon_snapshot(pair, &polygon),
            role: if index == 0 {
                "start".to_string()
            } else if index == sequence.len() - 2 {
                "end".to_string()
            } else {
                "intermediate".to_string()
            },
        });
    }

    let fixed_point = fixed_point_snapshot(tube);
    let fixed_points_on_faces =
        fixed_points_on_faces(input, tube, fixed_point.point, &intermediate_polygons)?;

    Ok(TubeVisualizationSnapshot {
        sequence: tube.sequence.clone(),
        start_pair: pair_array(tube.start_pair()),
        end_pair: pair_array(tube.end_pair()),
        start_polygon: polygon_snapshot(pair_array(tube.start_pair()), &tube.start_polygon),
        end_polygon: polygon_snapshot(pair_array(tube.end_pair()), &tube.end_polygon),
        intermediate_polygons,
        fixed_points_on_faces,
        start_to_end: affine_snapshot(&tube.start_to_end),
        end_to_start: affine_snapshot(&tube.end_to_start),
        action_on_start: affine_scalar_snapshot(&tube.action_on_start),
        action_on_end: affine_scalar_snapshot(&tube.action_on_end),
        fixed_point,
        cutoff: tube.cutoff,
    })
}

fn fixed_points_on_faces(
    input: &FlatTubeInput<'_>,
    tube: &F64Tube,
    start_point: Option<[f64; 2]>,
    face_polygons: &[TubeFaceSnapshot],
) -> Result<Vec<TubeFaceFixedPointSnapshot>, F64TubeError> {
    let Some(start_point) = start_point else {
        return Ok(face_polygons
            .iter()
            .map(|face| TubeFaceFixedPointSnapshot {
                pair: face.pair,
                role: face.role.clone(),
                point: None,
            })
            .collect());
    };
    let start_point = Vector2::new(start_point[0], start_point[1]);
    let sequence = tube.sequence();
    let mut points = Vec::with_capacity(face_polygons.len());
    for (index, face) in face_polygons.iter().enumerate() {
        let point = if index == 0 {
            start_point
        } else if index == sequence.len() - 2 {
            tube.start_to_end.apply(start_point)
        } else {
            let prefix = build_tube_for_word_f64(input, &sequence[..index + 2], tube.cutoff)?
                .ok_or(F64TubeError::NoOrbit)?;
            prefix.start_to_end.apply(start_point)
        };
        points.push(TubeFaceFixedPointSnapshot {
            pair: face.pair,
            role: face.role.clone(),
            point: Some(vector2_array(point)),
        });
    }
    Ok(points)
}

fn polygon_snapshot(pair: [usize; 2], polygon: &Polygon2) -> FacePolygonSnapshot {
    FacePolygonSnapshot {
        pair,
        vertices: polygon.vertices().into_iter().map(vector2_array).collect(),
        inequalities: polygon
            .inequalities
            .iter()
            .map(|halfspace| HalfspaceSnapshot {
                normal: vector2_array(halfspace.normal),
                rhs: halfspace.rhs,
            })
            .collect(),
    }
}

fn affine_snapshot(affine: &Affine2) -> AffineSnapshot {
    AffineSnapshot {
        matrix: [
            [affine.matrix[(0, 0)], affine.matrix[(0, 1)]],
            [affine.matrix[(1, 0)], affine.matrix[(1, 1)]],
        ],
        offset: vector2_array(affine.offset),
    }
}

fn affine_scalar_snapshot(affine: &AffineScalar2) -> AffineScalarSnapshot {
    AffineScalarSnapshot {
        coeff: vector2_array(affine.coeff),
        constant: affine.constant,
    }
}

fn fixed_point_snapshot(tube: &F64Tube) -> FixedPointSnapshot {
    let lhs = tube.start_to_end.matrix - Matrix2::identity();
    let rhs = -tube.start_to_end.offset;
    if lhs.determinant().abs() >= EPS_DET {
        if let Some(inverse) = lhs.try_inverse() {
            let point = inverse * rhs;
            return FixedPointSnapshot {
                status: if tube.start_polygon.contains(&point) {
                    "point_inside_start_polygon".to_string()
                } else {
                    "point_outside_start_polygon".to_string()
                },
                point: Some(vector2_array(point)),
                line_point: None,
                line_direction: None,
                action: Some(tube.action_at_start(point)),
            };
        }
    }

    let diagnostic = singular_fixed_point_diagnostic(&lhs, &tube.start_to_end.offset);
    if diagnostic.inconsistent {
        return FixedPointSnapshot {
            status: diagnostic.cause.to_string(),
            point: None,
            line_point: None,
            line_direction: None,
            action: None,
        };
    }

    let row = if Vector2::new(lhs[(0, 0)], lhs[(0, 1)]).norm()
        >= Vector2::new(lhs[(1, 0)], lhs[(1, 1)]).norm()
    {
        (Vector2::new(lhs[(0, 0)], lhs[(0, 1)]), rhs[0])
    } else {
        (Vector2::new(lhs[(1, 0)], lhs[(1, 1)]), rhs[1])
    };
    let normal = row.0;
    let normal_norm_squared = normal.norm_squared();
    if normal_norm_squared < EPS_DET * EPS_DET {
        return FixedPointSnapshot {
            status: "all_start_polygon_points_fixed".to_string(),
            point: None,
            line_point: Some([0.0, 0.0]),
            line_direction: Some([1.0, 0.0]),
            action: None,
        };
    }
    let line_point = normal * (row.1 / normal_norm_squared);
    let line_direction = Vector2::new(-normal[1], normal[0]) / normal.norm();
    FixedPointSnapshot {
        status: diagnostic.cause.to_string(),
        point: None,
        line_point: Some(vector2_array(line_point)),
        line_direction: Some(vector2_array(line_direction)),
        action: None,
    }
}

fn pair_array(pair: (usize, usize)) -> [usize; 2] {
    [pair.0, pair.1]
}

fn vector2_array(vector: Vector2<f64>) -> [f64; 2] {
    [vector[0], vector[1]]
}

#[derive(Clone, Copy, Debug)]
struct SingularFixedPointDiagnostic {
    rhs_norm: f64,
    first_column_norm: f64,
    second_column_norm: f64,
    null_norm: f64,
    null_residual: f64,
    inconsistency_threshold: f64,
    inconsistent: bool,
    cause: &'static str,
}

fn singular_fixed_point_diagnostic(
    lhs: &Matrix2<f64>,
    offset: &Vector2<f64>,
) -> SingularFixedPointDiagnostic {
    let rhs = -offset;
    let rhs_norm = rhs.norm();
    let first_column = Vector2::new(lhs[(0, 0)], lhs[(1, 0)]);
    let second_column = Vector2::new(lhs[(0, 1)], lhs[(1, 1)]);
    let first_column_norm = first_column.norm();
    let second_column_norm = second_column.norm();
    let left_null = if first_column.norm() >= second_column.norm() {
        Vector2::new(first_column[1], -first_column[0])
    } else {
        Vector2::new(second_column[1], -second_column[0])
    };
    let null_norm = left_null.norm();
    if null_norm < EPS_DET {
        let inconsistent = rhs_norm > EPS_CONTAINS;
        return SingularFixedPointDiagnostic {
            rhs_norm,
            first_column_norm,
            second_column_norm,
            null_norm,
            null_residual: rhs_norm,
            inconsistency_threshold: EPS_CONTAINS,
            inconsistent,
            cause: if inconsistent {
                "rank_zero_inconsistent"
            } else {
                "rank_zero_consistent"
            },
        };
    }
    let normalized_left_null = left_null / null_norm;
    let null_residual = normalized_left_null.dot(&rhs).abs();
    let inconsistency_threshold = 1e-7 * (1.0 + rhs_norm);
    let inconsistent = null_residual > inconsistency_threshold;
    SingularFixedPointDiagnostic {
        rhs_norm,
        first_column_norm,
        second_column_norm,
        null_norm,
        null_residual,
        inconsistency_threshold,
        inconsistent,
        cause: if inconsistent {
            "rank_one_inconsistent"
        } else {
            "rank_one_consistent"
        },
    }
}

fn orbit_from_cyclic_word_f64(
    input: &FlatTubeInput<'_>,
    word: &[usize],
    start_point: Vector4<f64>,
    action: f64,
) -> Result<Option<F64ClosedOrbit>, F64TubeError> {
    let mut breakpoints = vec![start_point];
    let mut segment_times = Vec::with_capacity(word.len());
    let mut point = start_point;

    for i in 0..word.len() {
        let current = word[(i + 1) % word.len()];
        let next = word[(i + 2) % word.len()];
        let reeb = 2.0 * (j4() * input.dual_vertices[current]);
        let denom = input.dual_vertices[next].dot(&reeb);
        if denom.abs() < EPS_DET {
            return Err(F64TubeError::UnsupportedDegenerateTransition);
        }
        let tau = (1.0 - input.dual_vertices[next].dot(&point)) / denom;
        if tau < -EPS_CONTAINS {
            return Ok(None);
        }
        point += reeb * tau;
        segment_times.push(tau.max(0.0));
        if i + 1 < word.len() {
            breakpoints.push(point);
        }
    }
    if (point - start_point).norm() > 1e-6 {
        return Ok(None);
    }
    let time_sum: f64 = segment_times.iter().sum();
    if (time_sum - action).abs() > 1e-6 * (1.0 + action.abs()) {
        return Ok(None);
    }
    Ok(Some(F64ClosedOrbit {
        facets: word.to_vec(),
        action,
        breakpoints,
        segment_times,
    }))
}

fn tube_from_parts(
    sequence: Vec<usize>,
    start_frame: FaceFrame,
    end_frame: FaceFrame,
    start_polygon: Polygon2,
    start_to_end: Affine2,
    action_on_start: AffineScalar2,
    cutoff: f64,
) -> Result<F64Tube, F64TubeError> {
    let end_to_start = start_to_end.inverse()?;
    let end_polygon = start_polygon.image_under(&start_to_end)?;
    let action_on_end = action_on_start.pullback(&end_to_start);
    Ok(F64Tube {
        sequence,
        start_frame,
        end_frame,
        start_polygon,
        end_polygon,
        start_to_end,
        end_to_start,
        action_on_start,
        action_on_end,
        cutoff,
    })
}

fn empty_primitive(
    input: &FlatTubeInput<'_>,
    facets: [usize; 3],
    cutoff: f64,
) -> Result<F64Tube, F64TubeError> {
    let start_frame = face_frame(input, facets[0], facets[1])?;
    let end_frame = face_frame(input, facets[1], facets[2])?;
    let contradiction = Polygon2::new(vec![
        Halfspace2::new(Vector2::new(1.0, 0.0), 0.0),
        Halfspace2::new(Vector2::new(-1.0, 0.0), -1.0),
    ]);
    Ok(F64Tube {
        sequence: facets.to_vec(),
        start_frame,
        end_frame,
        start_polygon: contradiction.clone(),
        end_polygon: contradiction,
        start_to_end: Affine2::identity(),
        end_to_start: Affine2::identity(),
        action_on_start: AffineScalar2 {
            coeff: Vector2::zeros(),
            constant: 0.0,
        },
        action_on_end: AffineScalar2 {
            coeff: Vector2::zeros(),
            constant: 0.0,
        },
        cutoff,
    })
}

fn primitive_affine_map(
    start: &FaceFrame,
    end: &FaceFrame,
    reeb: Vector4<f64>,
    tau_coeff: Vector2<f64>,
    tau_const: f64,
) -> Affine2 {
    let image_base = start.base + reeb * tau_const;
    let image_u = start.u + reeb * tau_coeff[0];
    let image_v = start.v + reeb * tau_coeff[1];
    Affine2 {
        matrix: Matrix2::from_columns(&[end.linear_coords(&image_u), end.linear_coords(&image_v)]),
        offset: end.coords(&image_base),
    }
}

fn face_polygon(input: &FlatTubeInput<'_>, frame: &FaceFrame) -> Polygon2 {
    Polygon2::new(
        input
            .dual_vertices
            .iter()
            .map(|a| {
                Halfspace2::new(
                    Vector2::new(a.dot(&frame.u), a.dot(&frame.v)),
                    1.0 - a.dot(&frame.base),
                )
            })
            .collect(),
    )
}

fn face_function_supremum(polygon: &Polygon2, frame: &FaceFrame, functional: &Vector4<f64>) -> f64 {
    polygon
        .vertices()
        .into_iter()
        .map(|coords| functional.dot(&frame.point(coords)))
        .fold(f64::NEG_INFINITY, f64::max)
}

fn face_frame(
    input: &FlatTubeInput<'_>,
    first: usize,
    second: usize,
) -> Result<FaceFrame, F64TubeError> {
    validate_facets(input, &[first, second])?;
    let a = input.dual_vertices[first];
    let b = input.dual_vertices[second];
    let solve = best_coordinate_split(&a, &b)?;
    let base = solve_with_free(&a, &b, solve, [0.0, 0.0], [1.0, 1.0])?;
    let raw_u = solve_with_free(&a, &b, solve, [1.0, 0.0], [0.0, 0.0])?;
    let raw_v = solve_with_free(&a, &b, solve, [0.0, 1.0], [0.0, 0.0])?;

    let u_norm = raw_u.norm();
    if u_norm < EPS_DET {
        return Err(F64TubeError::SingularFaceFrame);
    }
    let u = raw_u / u_norm;
    let v_raw = raw_v - u * u.dot(&raw_v);
    let v_norm = v_raw.norm();
    if v_norm < EPS_DET {
        return Err(F64TubeError::SingularFaceFrame);
    }
    let v = v_raw / v_norm;

    Ok(FaceFrame {
        first,
        second,
        base,
        u,
        v,
    })
}

fn best_coordinate_split(a: &Vector4<f64>, b: &Vector4<f64>) -> Result<[usize; 2], F64TubeError> {
    let mut best = None;
    for r in 0..4 {
        for s in (r + 1)..4 {
            let det = det2(a[r], a[s], b[r], b[s]).abs();
            if best
                .map(|(_, best_det): ([usize; 2], f64)| det > best_det)
                .unwrap_or(true)
            {
                best = Some(([r, s], det));
            }
        }
    }
    let (solve, det) = best.ok_or(F64TubeError::SingularFaceFrame)?;
    if det < EPS_DET {
        return Err(F64TubeError::SingularFaceFrame);
    }
    Ok(solve)
}

fn solve_with_free(
    a: &Vector4<f64>,
    b: &Vector4<f64>,
    solve: [usize; 2],
    free_values: [f64; 2],
    rhs: [f64; 2],
) -> Result<Vector4<f64>, F64TubeError> {
    let free: Vec<usize> = (0..4)
        .filter(|idx| *idx != solve[0] && *idx != solve[1])
        .collect();
    let mut x = Vector4::zeros();
    x[free[0]] = free_values[0];
    x[free[1]] = free_values[1];
    let reduced_rhs = [
        rhs[0] - a[free[0]] * free_values[0] - a[free[1]] * free_values[1],
        rhs[1] - b[free[0]] * free_values[0] - b[free[1]] * free_values[1],
    ];
    let det = det2(a[solve[0]], a[solve[1]], b[solve[0]], b[solve[1]]);
    if det.abs() < EPS_DET {
        return Err(F64TubeError::SingularFaceFrame);
    }
    x[solve[0]] = (reduced_rhs[0] * b[solve[1]] - a[solve[1]] * reduced_rhs[1]) / det;
    x[solve[1]] = (a[solve[0]] * reduced_rhs[1] - reduced_rhs[0] * b[solve[0]]) / det;
    Ok(x)
}

fn validate_cutoff(cutoff: f64) -> Result<(), F64TubeError> {
    if cutoff.is_nan() || cutoff < 0.0 {
        return Err(F64TubeError::InvalidCutoff);
    }
    Ok(())
}

fn validate_facets(input: &FlatTubeInput<'_>, facets: &[usize]) -> Result<(), F64TubeError> {
    if facets.iter().any(|&facet| facet >= input.facet_count()) {
        return Err(F64TubeError::InvalidFacet);
    }
    Ok(())
}

fn det2(a00: f64, a01: f64, a10: f64, a11: f64) -> f64 {
    a00 * a11 - a01 * a10
}
