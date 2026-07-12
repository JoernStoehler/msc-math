//! Arm-private, hard-budget target evaluation for the S0 product-search packet.
//!
//! A target attempt is a request for a full `sys` computation after candidate
//! construction has succeeded.  The budget gate deliberately precedes the
//! cache lookup: repeated candidates, cache hits, and failed computations all
//! consume an attempt.  Construction failures are outside this type because
//! they have no `SysLandscapePolytopeCache` to query; callers record their
//! count in `ProposalMeta::construction_rejections_before` and may use
//! `record_construction_rejection` for the arm-level total.

use crate::chart::ProductChart;
use crate::model::{Arm, CacheStatus, ProposalMeta, ProposalRole, TARGET_BUDGET};
use exp_sys_landscape::{
    dual_vertices_rational_strings, polytope_key, ExpensiveComputationCache, SysComputation,
    SysLandscapePolytopeCache,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;
use symplectic::{OrbitAdmissibility, OrbitSearchResult};

/// Compact target-attempt record.  The complete orbit result belongs only in
/// the corresponding [`CacheExportRow`] for a successful cache miss.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetEvaluationRow {
    pub candidate_id: String,
    pub polytope_key: Option<String>,
    pub poly_id: Option<String>,
    pub arm: Arm,
    pub replicate: usize,
    pub attempt_index: usize,
    pub generation: Option<usize>,
    pub trajectory: Option<usize>,
    pub iteration: Option<usize>,
    pub proposal_index: usize,
    pub construction_attempt: usize,
    pub construction_sequence_index: usize,
    pub construction_rejections_before: usize,
    pub role: ProposalRole,
    pub parent_candidate_id: Option<String>,
    pub elite_set_id: Option<String>,
    pub became_next_state: bool,
    pub evaluation_status: EvaluationStatus,
    pub cache_status: CacheStatus,
    pub wall_time_ms: f64,
    pub capacity: Option<f64>,
    pub volume: Option<f64>,
    pub sys: Option<f64>,
    pub capacity_iterations: Option<u64>,
    pub raw_returned_word_count: Option<usize>,
    pub raw_admissible_word_count: Option<usize>,
    pub distinct_cyclic_class_count: Option<usize>,
    pub support_lengths: Vec<usize>,
    pub product_chart: Option<ProductChart>,
}

/// Whether the charged full computation succeeded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationStatus {
    Success,
    Failure,
}

/// A complete, self-contained successful-miss cache row.
///
/// The fields duplicate the existing expensive-cache row with arm/replicate
/// ownership attached, so cache files can be exported without consulting any
/// other arm's cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheExportRow {
    pub arm: Arm,
    pub replicate: usize,
    pub polytope_key: String,
    /// Ordinary shared f64-bit geometry identity, duplicated here so target
    /// rows cannot forge a self-consistent ID detached from cache geometry.
    pub poly_id: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity_result: OrbitSearchResult,
    pub volume: f64,
    pub sys: f64,
}

/// Result of one query attempt.  `Exhausted` does not create a row or call the
/// oracle, because no target attempt is available to charge.
#[derive(Clone, Debug)]
pub enum QueryOutcome {
    Success {
        row_index: usize,
        computation: SysComputation,
    },
    Failure {
        row_index: usize,
    },
    Exhausted,
}

impl QueryOutcome {
    pub fn row_index(&self) -> Option<usize> {
        match self {
            Self::Success { row_index, .. } | Self::Failure { row_index } => Some(*row_index),
            Self::Exhausted => None,
        }
    }
}

/// Injectable full-computation boundary.  The production implementation is
/// `ExpensiveComputationCache`; a deterministic synthetic implementation is
/// provided for evaluator-only tests and runner smoke tests.
pub trait SysComputationOracle {
    fn compute(&mut self, polytope: &SysLandscapePolytopeCache) -> Option<SysComputation>;
}

impl SysComputationOracle for ExpensiveComputationCache {
    fn compute(&mut self, polytope: &SysLandscapePolytopeCache) -> Option<SysComputation> {
        ExpensiveComputationCache::compute(self, polytope)
    }
}

impl<F> SysComputationOracle for F
where
    F: FnMut(&SysLandscapePolytopeCache) -> Option<SysComputation>,
{
    fn compute(&mut self, polytope: &SysLandscapePolytopeCache) -> Option<SysComputation> {
        self(polytope)
    }
}

/// Deterministic oracle useful for target-free tests.
#[derive(Clone, Debug, Default)]
pub struct SyntheticOracle {
    responses: HashMap<String, Option<SysComputation>>,
    calls: Vec<String>,
}

impl SyntheticOracle {
    pub fn with_response(
        mut self,
        polytope_key: impl Into<String>,
        response: Option<SysComputation>,
    ) -> Self {
        self.responses.insert(polytope_key.into(), response);
        self
    }

    pub fn call_keys(&self) -> &[String] {
        &self.calls
    }
}

impl SysComputationOracle for SyntheticOracle {
    fn compute(&mut self, polytope: &SysLandscapePolytopeCache) -> Option<SysComputation> {
        let key = polytope_key(polytope);
        self.calls.push(key.clone());
        self.responses.get(&key).cloned().flatten()
    }
}

/// One evaluator and one initially empty successful-result cache for an arm
/// and replicate.  Do not share an instance between arms or replicates.
pub struct ArmEvaluator<O = ExpensiveComputationCache> {
    arm: Arm,
    replicate: usize,
    oracle: O,
    successful_cache: HashMap<String, SysComputation>,
    cache_rows: HashMap<String, CacheExportRow>,
    target_rows: Vec<TargetEvaluationRow>,
    next_target_row_to_drain: usize,
    pending_cache_rows: Vec<CacheExportRow>,
    attempts_used: usize,
    construction_rejections: usize,
}

impl ArmEvaluator<ExpensiveComputationCache> {
    /// Construct the production evaluator with an arm-private empty cache.
    pub fn empty(arm: Arm, replicate: usize) -> Self {
        Self::new(arm, replicate, ExpensiveComputationCache::empty())
    }
}

impl<O> ArmEvaluator<O>
where
    O: SysComputationOracle,
{
    pub fn new(arm: Arm, replicate: usize, oracle: O) -> Self {
        Self {
            arm,
            replicate,
            oracle,
            successful_cache: HashMap::new(),
            cache_rows: HashMap::new(),
            target_rows: Vec::new(),
            next_target_row_to_drain: 0,
            pending_cache_rows: Vec::new(),
            attempts_used: 0,
            construction_rejections: 0,
        }
    }

    pub fn arm(&self) -> Arm {
        self.arm
    }

    pub fn replicate(&self) -> usize {
        self.replicate
    }

    pub fn attempts_used(&self) -> usize {
        self.attempts_used
    }

    pub fn remaining_budget(&self) -> usize {
        TARGET_BUDGET - self.attempts_used
    }

    pub fn is_exhausted(&self) -> bool {
        self.attempts_used == TARGET_BUDGET
    }

    pub fn construction_rejections(&self) -> usize {
        self.construction_rejections
    }

    /// Record a rejected construction without touching the target budget or
    /// oracle.  Its detailed reason is owned by the proposer/lineage packet.
    pub fn record_construction_rejection(&mut self) {
        self.construction_rejections += 1;
    }

    /// Charge and evaluate one successfully constructed candidate.
    ///
    /// `meta.arm` and `meta.replicate` must identify this evaluator.  The
    /// check prevents a caller from accidentally writing a row into a shared
    /// cache under another arm's label.
    pub fn evaluate(
        &mut self,
        meta: ProposalMeta,
        polytope: &SysLandscapePolytopeCache,
    ) -> QueryOutcome {
        assert_eq!(meta.arm, self.arm, "proposal arm must match evaluator arm");
        assert_eq!(
            meta.replicate, self.replicate,
            "proposal replicate must match evaluator replicate"
        );
        if self.is_exhausted() {
            return QueryOutcome::Exhausted;
        }

        // This is intentionally before the cache lookup and oracle call.
        self.attempts_used += 1;
        let attempt_index = self.attempts_used;
        let key = polytope_key(polytope);
        let started = Instant::now();

        let (cache_status, computation) = match self.successful_cache.get(&key) {
            Some(cached) => (CacheStatus::Hit, Some(cached.clone())),
            None => match self.oracle.compute(polytope) {
                Some(computation) => (CacheStatus::Miss, Some(computation)),
                None => (CacheStatus::FailedMiss, None),
            },
        };
        let wall_time_ms = started.elapsed().as_secs_f64() * 1_000.0;

        let row = match computation.as_ref() {
            Some(computation) => {
                if cache_status == CacheStatus::Miss {
                    self.insert_successful_miss(&key, polytope, computation.clone());
                }
                target_row_success(
                    meta,
                    key,
                    attempt_index,
                    cache_status,
                    wall_time_ms,
                    computation,
                    polytope,
                )
            }
            None => target_row_failure(meta, key, attempt_index, wall_time_ms, polytope),
        };
        let row_index = self.target_rows.len();
        self.target_rows.push(row);

        match computation {
            Some(computation) => QueryOutcome::Success {
                row_index,
                computation,
            },
            None => QueryOutcome::Failure { row_index },
        }
    }

    /// Mutate a retained target row, for example after a complete local search
    /// grid selects it as the next state.  If the row has not yet been drained,
    /// the updated value is what `drain_target_rows` returns.
    pub fn target_row_mut(&mut self, row_index: usize) -> Option<&mut TargetEvaluationRow> {
        self.target_rows.get_mut(row_index)
    }

    pub fn target_rows(&self) -> &[TargetEvaluationRow] {
        &self.target_rows
    }

    /// All successful-miss cache rows in deterministic exact-key order.
    pub fn cache_rows(&self) -> Vec<CacheExportRow> {
        let mut rows: Vec<_> = self.cache_rows.values().cloned().collect();
        rows.sort_by(|left, right| left.polytope_key.cmp(&right.polytope_key));
        rows
    }

    /// Rows accumulated since the last call.  This is intended for append-only
    /// safe flushing after every target attempt.
    pub fn drain_target_rows(&mut self) -> Vec<TargetEvaluationRow> {
        let rows = self.target_rows[self.next_target_row_to_drain..].to_vec();
        self.next_target_row_to_drain = self.target_rows.len();
        rows
    }

    /// Successful-miss cache rows accumulated since the last call.
    pub fn drain_cache_rows(&mut self) -> Vec<CacheExportRow> {
        std::mem::take(&mut self.pending_cache_rows)
    }

    pub fn into_oracle(self) -> O {
        self.oracle
    }

    fn insert_successful_miss(
        &mut self,
        key: &str,
        polytope: &SysLandscapePolytopeCache,
        computation: SysComputation,
    ) {
        let cache_row = CacheExportRow {
            arm: self.arm,
            replicate: self.replicate,
            polytope_key: key.to_owned(),
            poly_id: packet_poly_id(polytope),
            dual_vertices_rational: dual_vertices_rational_strings(polytope),
            facet_count: polytope.facet_count(),
            capacity_result: computation.capacity.clone(),
            volume: computation.vol,
            sys: computation.sys,
        };
        self.successful_cache.insert(key.to_owned(), computation);
        self.pending_cache_rows.push(cache_row.clone());
        let previous = self.cache_rows.insert(key.to_owned(), cache_row);
        assert!(previous.is_none(), "successful cache miss must be new");
    }
}

fn target_row_success(
    meta: ProposalMeta,
    polytope_key: String,
    attempt_index: usize,
    cache_status: CacheStatus,
    wall_time_ms: f64,
    computation: &SysComputation,
    polytope: &SysLandscapePolytopeCache,
) -> TargetEvaluationRow {
    let compact = compact_orbit_payload(&computation.capacity);
    TargetEvaluationRow {
        candidate_id: meta.candidate_id,
        polytope_key: Some(polytope_key),
        poly_id: Some(packet_poly_id(polytope)),
        arm: meta.arm,
        replicate: meta.replicate,
        attempt_index,
        generation: meta.generation,
        trajectory: meta.trajectory,
        iteration: meta.iteration,
        proposal_index: meta.proposal_index,
        construction_attempt: meta.construction_attempt,
        construction_sequence_index: meta.construction_sequence_index,
        construction_rejections_before: meta.construction_rejections_before,
        role: meta.role,
        parent_candidate_id: meta.parent_candidate_id,
        elite_set_id: meta.elite_set_id,
        became_next_state: false,
        evaluation_status: EvaluationStatus::Success,
        cache_status,
        wall_time_ms,
        capacity: Some(computation.capacity.min_action),
        volume: Some(computation.vol),
        sys: Some(computation.sys),
        capacity_iterations: Some(computation.capacity.iterations),
        raw_returned_word_count: Some(compact.raw_returned_word_count),
        raw_admissible_word_count: Some(compact.raw_admissible_word_count),
        distinct_cyclic_class_count: Some(compact.distinct_cyclic_class_count),
        support_lengths: compact.support_lengths,
        product_chart: ProductChart::from_polytope(polytope).ok(),
    }
}

fn target_row_failure(
    meta: ProposalMeta,
    polytope_key: String,
    attempt_index: usize,
    wall_time_ms: f64,
    polytope: &SysLandscapePolytopeCache,
) -> TargetEvaluationRow {
    TargetEvaluationRow {
        candidate_id: meta.candidate_id,
        polytope_key: Some(polytope_key),
        poly_id: Some(packet_poly_id(polytope)),
        arm: meta.arm,
        replicate: meta.replicate,
        attempt_index,
        generation: meta.generation,
        trajectory: meta.trajectory,
        iteration: meta.iteration,
        proposal_index: meta.proposal_index,
        construction_attempt: meta.construction_attempt,
        construction_sequence_index: meta.construction_sequence_index,
        construction_rejections_before: meta.construction_rejections_before,
        role: meta.role,
        parent_candidate_id: meta.parent_candidate_id,
        elite_set_id: meta.elite_set_id,
        became_next_state: false,
        evaluation_status: EvaluationStatus::Failure,
        cache_status: CacheStatus::FailedMiss,
        wall_time_ms,
        capacity: None,
        volume: None,
        sys: None,
        capacity_iterations: None,
        raw_returned_word_count: None,
        raw_admissible_word_count: None,
        distinct_cyclic_class_count: None,
        support_lengths: Vec::new(),
        product_chart: ProductChart::from_polytope(polytope).ok(),
    }
}

/// Packet-local, independently reproducible f64 geometry identity. Coordinates
/// use normalized IEEE-754 little-endian bytes in facet order; the analyzer
/// reconstructs the same bytes from the exact rational cache payload.
fn packet_poly_id(polytope: &SysLandscapePolytopeCache) -> String {
    let mut hasher = Sha256::new();
    for vertex in &polytope.dual_vertices_f64 {
        for coordinate in vertex.iter() {
            let normalized = if *coordinate == 0.0 { 0.0 } else { *coordinate };
            assert!(
                normalized.is_finite(),
                "packet poly_id requires finite geometry"
            );
            hasher.update(normalized.to_bits().to_le_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

#[derive(Debug)]
struct CompactOrbitPayload {
    raw_returned_word_count: usize,
    raw_admissible_word_count: usize,
    distinct_cyclic_class_count: usize,
    support_lengths: Vec<usize>,
}

/// Extract row diagnostics from the already returned payload only.  In
/// particular, this does not trigger a second capacity/action-window search.
fn compact_orbit_payload(result: &OrbitSearchResult) -> CompactOrbitPayload {
    let raw_returned_word_count = result.orbits.len();
    let admissible_words: Vec<&[usize]> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| orbit.sigma.as_slice())
        .collect();
    let raw_admissible_word_count = admissible_words.len();
    // This describes the raw returned payload, while the separate admissible
    // count records how many of those words were usable for the scalar route.
    let distinct_cyclic_class_count = result
        .orbits
        .iter()
        .map(|orbit| canonical_cyclic_rotation(&orbit.sigma))
        .collect::<BTreeSet<_>>()
        .len();
    let support_lengths = result
        .orbits
        .iter()
        .map(|orbit| orbit.sigma.len())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    CompactOrbitPayload {
        raw_returned_word_count,
        raw_admissible_word_count,
        distinct_cyclic_class_count,
        support_lengths,
    }
}

/// Lexicographically least cyclic rotation of a nonempty returned orbit word.
fn canonical_cyclic_rotation(word: &[usize]) -> Vec<usize> {
    assert!(!word.is_empty(), "returned orbit words must be nonempty");
    (0..word.len())
        .map(|start| {
            word.iter()
                .cycle()
                .skip(start)
                .take(word.len())
                .copied()
                .collect()
        })
        .min()
        .expect("nonempty word has a rotation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use exp_sys_landscape::SysLandscapePolytopeCache;
    use nalgebra::Vector2;
    use symplectic::{OrbitKktData, OrbitSearchResult};

    fn polytope() -> SysLandscapePolytopeCache {
        let normals = vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(-1.0, 0.0),
            Vector2::new(0.0, -1.0),
            Vector2::new(0.8, 0.6),
        ];
        let heights = vec![1.0; 5];
        SysLandscapePolytopeCache::from_lagrangian_product(&normals, &heights, &normals, &heights)
            .expect("test product constructs")
    }

    fn computation() -> SysComputation {
        let orbit = |sigma, admissibility| OrbitKktData {
            sigma,
            beta: vec![1.0; 3],
            beta_margin: 1.0,
            action: 2.0,
            action_lower: 2.0,
            action_upper: 2.0,
            q: 0.25,
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility,
        };
        SysComputation {
            capacity: OrbitSearchResult {
                orbits: vec![
                    orbit(vec![3, 1, 2], OrbitAdmissibility::AdmissibleExact),
                    orbit(vec![2, 3, 1], OrbitAdmissibility::AdmissibleF64),
                    orbit(vec![4, 5, 6], OrbitAdmissibility::IndeterminateF64),
                ],
                min_action: 2.0,
                min_action_lower: 2.0,
                min_action_upper: 2.0,
                iterations: 17,
            },
            vol: 4.0,
            sys: 0.5,
        }
    }

    fn meta(index: usize) -> ProposalMeta {
        ProposalMeta {
            candidate_id: format!("s0v1-{index:024x}"),
            arm: Arm::Iid,
            replicate: 0,
            generation: None,
            trajectory: None,
            iteration: None,
            proposal_index: index,
            construction_attempt: 0,
            construction_sequence_index: index,
            construction_rejections_before: 0,
            role: ProposalRole::Iid,
            parent_candidate_id: None,
            elite_set_id: None,
        }
    }

    fn evaluator_with(response: Option<SysComputation>) -> ArmEvaluator<SyntheticOracle> {
        let p = polytope();
        ArmEvaluator::new(
            Arm::Iid,
            0,
            SyntheticOracle::default().with_response(polytope_key(&p), response),
        )
    }

    #[test]
    fn successful_miss_preserves_payload_and_compacts_orbit_words() {
        let p = polytope();
        let mut evaluator = evaluator_with(Some(computation()));
        let QueryOutcome::Success {
            row_index,
            computation: returned,
        } = evaluator.evaluate(meta(0), &p)
        else {
            panic!("configured oracle should succeed");
        };
        assert_eq!(returned.capacity.orbits.len(), 3);
        let row = &evaluator.target_rows()[row_index];
        assert_eq!(row.cache_status, CacheStatus::Miss);
        assert_eq!(row.raw_returned_word_count, Some(3));
        assert_eq!(row.raw_admissible_word_count, Some(2));
        assert_eq!(row.distinct_cyclic_class_count, Some(2));
        assert_eq!(row.support_lengths, vec![3]);
        assert!(row.product_chart.is_some());
        assert_eq!(evaluator.cache_rows().len(), 1);
        assert_eq!(evaluator.cache_rows()[0].capacity_result, returned.capacity);
        evaluator
            .target_row_mut(row_index)
            .unwrap()
            .became_next_state = true;
        assert!(evaluator.target_rows()[row_index].became_next_state);
        assert!(evaluator.drain_target_rows()[0].became_next_state);
    }

    #[test]
    fn failures_are_charged_but_not_cached_and_construction_rejection_is_not_queried() {
        let p = polytope();
        let mut evaluator = evaluator_with(None);
        evaluator.record_construction_rejection();
        assert_eq!(evaluator.construction_rejections(), 1);
        assert_eq!(evaluator.remaining_budget(), TARGET_BUDGET);
        assert!(matches!(
            evaluator.evaluate(meta(0), &p),
            QueryOutcome::Failure { .. }
        ));
        assert!(matches!(
            evaluator.evaluate(meta(1), &p),
            QueryOutcome::Failure { .. }
        ));
        assert_eq!(evaluator.attempts_used(), 2);
        assert!(evaluator.cache_rows().is_empty());
        assert_eq!(evaluator.into_oracle().call_keys().len(), 2);
    }

    #[test]
    fn cache_hit_is_charged_and_cache_export_reconciles_with_successful_misses() {
        let p = polytope();
        let mut evaluator = evaluator_with(Some(computation()));
        assert!(matches!(
            evaluator.evaluate(meta(0), &p),
            QueryOutcome::Success { .. }
        ));
        assert!(matches!(
            evaluator.evaluate(meta(1), &p),
            QueryOutcome::Success { .. }
        ));
        let rows = evaluator.target_rows();
        assert_eq!(rows[0].cache_status, CacheStatus::Miss);
        assert_eq!(rows[1].cache_status, CacheStatus::Hit);
        assert_eq!(evaluator.attempts_used(), 2);
        let successful_miss_keys: BTreeSet<_> = rows
            .iter()
            .filter(|row| row.cache_status == CacheStatus::Miss)
            .map(|row| {
                row.polytope_key
                    .clone()
                    .expect("successful row has an exact key")
            })
            .collect();
        let cache_keys: BTreeSet<_> = evaluator
            .cache_rows()
            .into_iter()
            .map(|row| row.polytope_key)
            .collect();
        assert_eq!(cache_keys, successful_miss_keys);
        assert_eq!(evaluator.drain_target_rows().len(), 2);
        assert_eq!(evaluator.drain_cache_rows().len(), 1);
        assert!(evaluator.drain_target_rows().is_empty());
        assert!(evaluator.drain_cache_rows().is_empty());
        assert_eq!(evaluator.into_oracle().call_keys().len(), 1);
    }

    #[test]
    fn exactly_256_attempts_are_charged_then_the_hard_gate_stops_queries() {
        let p = polytope();
        let mut evaluator = evaluator_with(Some(computation()));
        for index in 0..TARGET_BUDGET {
            assert!(matches!(
                evaluator.evaluate(meta(index), &p),
                QueryOutcome::Success { .. }
            ));
        }
        assert!(evaluator.is_exhausted());
        assert_eq!(evaluator.remaining_budget(), 0);
        assert!(matches!(
            evaluator.evaluate(meta(TARGET_BUDGET), &p),
            QueryOutcome::Exhausted
        ));
        assert_eq!(evaluator.target_rows().len(), TARGET_BUDGET);
        assert_eq!(evaluator.into_oracle().call_keys().len(), 1);
    }
}
