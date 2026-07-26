//! Faithful production copy of the selected verified general QP route.
//!
//! Keep the mathematical semantics in sync with
//! `experiments/dev-quadratic-program/src/selected_route/general.rs`. The
//! experiment copy retains route variants and counters used by
//! `tools/general_algorithm_ablation`; production fixes the chosen
//! LBLT/hybrid route. The correspondence suite must compare capacity bounds
//! after a semantic change to either file.

use crate::exact::ExactOrbitKktData;
use crate::geom::rational_arithmetic::f64_to_rational;
use crate::kkt::qp_assembly::{
    build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices,
};
use crate::kkt::rational_solver::solve_kkt_exact;
use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Vector4};
use nalgebra035::{DMatrix as DMatrix35, DVector as DVector35};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

const INERTIA_RELATIVE_FLOOR: f64 = 1e-12;
type GeneralRouteCase = (String, Vec<Vector4<f64>>, Vec<Vec<usize>>);

// ── Shared route data ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecisionKind {
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug)]
struct Decision {
    kind: DecisionKind,
    // These legacy experiment field names have different status by guard:
    // certified radii for OutwardCertified and BatchedAnalyticEnvelope,
    // unverified estimates for EmpiricalThenExact, and None for exact
    // decisions. Do not expose them through a shared production result type.
    action: Option<f64>,
    beta_radius: Option<f64>,
    q_radius: Option<f64>,
    q_lower: Option<f64>,
    q_upper: Option<f64>,
    exact_fallback: bool,
}

#[derive(Clone, Debug)]
struct FactorData {
    solution: Vec<f64>,
    inverse: DMatrix<f64>,
}

#[derive(Clone, Debug)]
struct CurvatureProposal {
    direction: Vec<f64>,
}

#[derive(Clone, Debug)]
struct Obstruction {
    labels: Vec<usize>,
    mask: u16,
}

#[derive(Clone, Debug, Default)]
struct RouteStats {
    words: usize,
    inherited_rejections: usize,
    direct_obstructions: usize,
    obstruction_proposals: usize,
    obstruction_unknown: usize,
    lblt_factorizations: usize,
    guarded_decisions: usize,
    exact_fallbacks: usize,
    short_exact_solves: usize,
    short_interval_rejections: usize,
    accepted: usize,
    rejected: usize,
    max_beta_radius: f64,
    max_q_radius: f64,
    best_action: Option<f64>,
    best_action_lower: Option<f64>,
    best_action_upper: Option<f64>,
    elapsed: Duration,
    direct_by_length: BTreeMap<usize, usize>,
    inherited_by_length: BTreeMap<usize, usize>,
    fallback_by_length: BTreeMap<usize, usize>,
    lookup_time: Duration,
    factor_time: Duration,
    obstruction_time: Duration,
    guard_time: Duration,
    guard_phases: GuardPhaseStats,
    exact_time: Duration,
    short_exact_time: Duration,
    selected_decisions: Vec<Decision>,
}

pub(super) struct GeneralExactCandidate {
    pub(super) witness: ExactOrbitKktData<BigRational>,
}

pub(super) struct GeneralExactSelection {
    pub(super) capacity_exact: BigRational,
    pub(super) candidates: Vec<GeneralExactCandidate>,
}

pub(super) struct GeneralSolveOutput {
    pub(super) bounds: (f64, f64),
    pub(super) exact_selection: Option<GeneralExactSelection>,
}

enum GeneralOutputRequest {
    CapacityOnly,
    ExactWithinCapacityMultiple(BigRational),
}

#[derive(Clone, Debug, Default)]
struct GuardPhaseStats {
    entries_time: Duration,
    residual_time: Duration,
    defect_time: Duration,
    decision_time: Duration,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            lo: next_down(self.lo + rhs.lo),
            hi: next_up(self.hi + rhs.hi),
        }
    }

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self {
            lo: next_down(lo),
            hi: next_up(hi),
        }
    }

    fn abs_upper(self) -> f64 {
        next_up(self.lo.abs().max(self.hi.abs()))
    }

    fn is_valid_finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }
}
fn run_selected_route(cases: &[GeneralRouteCase]) -> RouteStats {
    debug_assert_eq!(
        cases.len(),
        1,
        "the production result retains decisions for exactly one input"
    );
    let started = Instant::now();
    let gradual_underflow = gradual_underflow_available();
    let mut stats = RouteStats::default();
    for (_, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let mut cache = Vec::<Obstruction>::new();
        let mut order = (0..words.len()).collect::<Vec<_>>();
        order.sort_by_key(|&index| (words[index].len(), index));
        let mut case_decisions = vec![rejected_exact_decision(); words.len()];
        for index in order {
            let word = &words[index];
            stats.words += 1;
            // Every f64 certificate in the batched route, including short-word
            // rejection and curvature pruning, assumes gradual underflow. If
            // the arithmetic environment lacks it, bypass the entire f64
            // route and exact-resolve the original candidate stream.
            if !gradual_underflow {
                let phase_started = Instant::now();
                let decision = exact_decision(&exact_duals, word);
                stats.exact_time += phase_started.elapsed();
                stats.exact_fallbacks += 1;
                *stats.fallback_by_length.entry(word.len()).or_default() += 1;
                record_decision(&mut stats, decision);
                case_decisions[index] = decision;
                continue;
            }
            if word.len() < 5 {
                let phase_started = Instant::now();
                let decision = if certified_short_inconsistent(duals, word) {
                    stats.short_interval_rejections += 1;
                    rejected_exact_decision()
                } else {
                    stats.short_exact_solves += 1;
                    short_exact_decision(&exact_duals, word)
                };
                stats.short_exact_time += phase_started.elapsed();
                record_decision(&mut stats, decision);
                case_decisions[index] = decision;
                continue;
            }
            let phase_started = Instant::now();
            let inherited = contains_certified_subword(word, &cache);
            stats.lookup_time += phase_started.elapsed();
            if inherited {
                stats.inherited_rejections += 1;
                *stats.inherited_by_length.entry(word.len()).or_default() += 1;
                stats.rejected += 1;
                case_decisions[index] = rejected_exact_decision();
                continue;
            }

            let discover = word.len() >= 6;
            stats.lblt_factorizations += 1;
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
            let factor = if discover {
                // Inertia needs only the Bunch--Kaufman factorization. Try the
                // certified curvature rejection before solving for beta and a
                // full inverse; direct obstructions never use either result.
                let matrix35 =
                    DMatrix35::from_column_slice(matrix.nrows(), matrix.ncols(), matrix.as_slice());
                let rhs35 = DVector35::from_column_slice(rhs.as_slice());
                let phase_started = Instant::now();
                let decomposition = matrix35.lblt();
                let positive = positive_inertia(&decomposition.d());
                stats.factor_time += phase_started.elapsed();

                let obstruction_started = Instant::now();
                if positive > 5 && has_certified_rank_five_constraints(duals, word) {
                    stats.obstruction_proposals += 1;
                    if let Some(proposal) = reduced_curvature_proposal(duals, word) {
                        if certify_curvature(duals, word, &proposal.direction) {
                            stats.direct_obstructions += 1;
                            *stats.direct_by_length.entry(word.len()).or_default() += 1;
                            cache.push(Obstruction {
                                labels: word.clone(),
                                mask: label_mask(word),
                            });
                            stats.rejected += 1;
                            case_decisions[index] = rejected_exact_decision();
                            stats.obstruction_time += obstruction_started.elapsed();
                            continue;
                        }
                    }
                    stats.obstruction_unknown += 1;
                }
                stats.obstruction_time += obstruction_started.elapsed();

                let phase_started = Instant::now();
                let factor = decomposition
                    .solve(&rhs35)
                    .zip(decomposition.solve(&DMatrix35::identity(matrix.nrows(), matrix.ncols())))
                    .and_then(|(solution, inverse35)| {
                        let inverse = DMatrix::from_column_slice(
                            inverse35.nrows(),
                            inverse35.ncols(),
                            inverse35.as_slice(),
                        );
                        finite_factor_data(solution.as_slice(), inverse)
                    });
                stats.factor_time += phase_started.elapsed();
                factor
            } else {
                let phase_started = Instant::now();
                let factor = factor_system(&matrix, &rhs);
                stats.factor_time += phase_started.elapsed();
                factor
            };

            let phase_started = Instant::now();
            let guarded = factor.as_ref().and_then(|data| {
                certify_direct_solution_hybrid_profiled(
                    duals,
                    word,
                    &matrix,
                    data,
                    gradual_underflow,
                    &mut stats.guard_phases,
                )
            });
            stats.guard_time += phase_started.elapsed();
            let decision = if let Some(decision) = guarded {
                decision
            } else {
                let phase_started = Instant::now();
                let decision = exact_decision(&exact_duals, word);
                stats.exact_time += phase_started.elapsed();
                decision
            };
            if decision.exact_fallback {
                stats.exact_fallbacks += 1;
                *stats.fallback_by_length.entry(word.len()).or_default() += 1;
            } else {
                stats.guarded_decisions += 1;
            }
            stats.max_beta_radius = stats
                .max_beta_radius
                .max(decision.beta_radius.unwrap_or(0.0));
            stats.max_q_radius = stats.max_q_radius.max(decision.q_radius.unwrap_or(0.0));
            record_decision(&mut stats, decision);
            case_decisions[index] = decision;
        }
        record_case_capacity_interval(&mut stats, &case_decisions);
        stats.selected_decisions = case_decisions;
    }
    stats.elapsed = started.elapsed();
    stats
}

fn record_decision(stats: &mut RouteStats, decision: Decision) {
    match decision.kind {
        DecisionKind::Accept => stats.accepted += 1,
        DecisionKind::Reject => stats.rejected += 1,
    }
}

fn record_case_capacity_interval(stats: &mut RouteStats, decisions: &[Decision]) {
    let accepted = decisions
        .iter()
        .filter(|decision| decision.kind == DecisionKind::Accept)
        .collect::<Vec<_>>();
    let Some(case_action) = accepted
        .iter()
        .filter_map(|decision| decision.action)
        .min_by(f64::total_cmp)
    else {
        return;
    };
    stats.best_action = Some(
        stats
            .best_action
            .map_or(case_action, |current| current.min(case_action)),
    );

    let Some(q_max_lower) = accepted
        .iter()
        .filter_map(|decision| decision.q_lower)
        .max_by(f64::total_cmp)
    else {
        return;
    };
    let Some(q_max_upper) = accepted
        .iter()
        .filter_map(|decision| decision.q_upper)
        .max_by(f64::total_cmp)
    else {
        return;
    };
    if q_max_upper.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return;
    }
    let action_lower = next_down(0.5 / q_max_upper);
    let action_upper = if q_max_lower > 0.0 {
        next_up(0.5 / q_max_lower)
    } else {
        f64::INFINITY
    };
    stats.best_action_lower = Some(
        stats
            .best_action_lower
            .map_or(action_lower, |current| current.min(action_lower)),
    );
    stats.best_action_upper = Some(
        stats
            .best_action_upper
            .map_or(action_upper, |current| current.min(action_upper)),
    );
}

fn factor_system(matrix: &DMatrix<f64>, rhs: &DVector<f64>) -> Option<FactorData> {
    let matrix35 = DMatrix35::from_column_slice(matrix.nrows(), matrix.ncols(), matrix.as_slice());
    let rhs35 = DVector35::from_column_slice(rhs.as_slice());
    let factor = matrix35.lblt();
    let solution = factor.solve(&rhs35)?;
    let inverse35 = factor.solve(&DMatrix35::identity(matrix.nrows(), matrix.ncols()))?;
    let inverse =
        DMatrix::from_column_slice(inverse35.nrows(), inverse35.ncols(), inverse35.as_slice());
    finite_factor_data(solution.as_slice(), inverse)
}

fn finite_factor_data(solution: &[f64], inverse: DMatrix<f64>) -> Option<FactorData> {
    if solution.iter().any(|value| !value.is_finite())
        || inverse.iter().any(|value| !value.is_finite())
    {
        return None;
    }
    Some(FactorData {
        solution: solution.to_vec(),
        inverse,
    })
}

// ── Certified direct predicates ──────────────────────────────────────────

fn positive_inertia(d: &DMatrix35<f64>) -> usize {
    let scale = d.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let floor = scale * INERTIA_RELATIVE_FLOOR;
    let mut positive = 0;
    let mut index = 0;
    while index < d.nrows() {
        if index + 1 < d.nrows() && d[(index + 1, index)] != 0.0 {
            let a = d[(index, index)];
            let b = d[(index + 1, index)];
            let c = d[(index + 1, index + 1)];
            let centre = 0.5 * (a + c);
            let spread = 0.5 * (a - c).hypot(2.0 * b);
            positive += usize::from(centre + spread > floor);
            positive += usize::from(centre - spread > floor);
            index += 2;
        } else {
            positive += usize::from(d[(index, index)] > floor);
            index += 1;
        }
    }
    positive
}

fn certify_direct_solution_hybrid_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    certify_direct_solution_normwise_profiled(
        duals,
        word,
        matrix,
        factor,
        gradual_underflow,
        phases,
    )
    .or_else(|| {
        certify_direct_solution_batched_profiled(
            duals,
            word,
            matrix,
            factor,
            gradual_underflow,
            phases,
        )
    })
}

/// Uses the same inverse-defect theorem as the entrywise batched enclosure,
/// but bounds the four auxiliary positive products by induced infinity norms.
///
/// The central residual and defect are still evaluated as ordinary matrix
/// products. For nonnegative matrices, submultiplicativity gives
/// `|| |A| |B| ||_inf <= ||A||_inf ||B||_inf`; outward operations turn this
/// into a valid upper bound for both rounding magnitudes and input-interval
/// propagation. This is looser than forming those four products entrywise but
/// avoids their runtime cost.
fn certify_direct_solution_normwise_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    if !gradual_underflow {
        return None;
    }
    let size = matrix.nrows();
    let phase_started = Instant::now();
    let entry_radius_norm = exact_kkt_entry_radius_inf_norm(duals, word)?;
    let matrix_norm = matrix_inf_norm_up(matrix);
    let inverse_norm = matrix_inf_norm_up(&factor.inverse);
    let solution_norm = factor
        .solution
        .iter()
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    if !matrix_norm.is_finite()
        || !entry_radius_norm.is_finite()
        || !inverse_norm.is_finite()
        || !solution_norm.is_finite()
    {
        return None;
    }
    let (gamma, underflow) = dot_product_error_parameters(size)?;
    phases.entries_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let solution = DMatrix::from_column_slice(size, 1, &factor.solution);
    let mut residual_centre = matrix * &solution;
    residual_centre[(size - 1, 0)] -= 1.0;
    let residual_centre_norm = residual_centre
        .iter()
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    // The final subtraction of b contributes one to the augmented dot-product
    // magnitude in its single nonzero row.
    let residual_rounding = add_up(
        mul_up(gamma, add_up(mul_up(matrix_norm, solution_norm), 1.0)),
        underflow,
    );
    let residual_input = mul_up(entry_radius_norm, solution_norm);
    let residual_norm = add_up(
        residual_centre_norm,
        add_up(residual_rounding, residual_input),
    );
    phases.residual_time += phase_started.elapsed();
    if !residual_norm.is_finite() {
        return None;
    }

    let phase_started = Instant::now();
    let defect_centre = DMatrix::identity(size, size) - matrix * &factor.inverse;
    let defect_centre_norm = matrix_inf_norm_up(&defect_centre);
    // Summing the per-entry dot-product bounds across one row contributes
    // ||K||_inf ||R||_inf. The identity contributes one in that row, and the
    // per-entry underflow allowance occurs `size` times.
    let defect_rounding = add_up(
        mul_up(gamma, add_up(mul_up(matrix_norm, inverse_norm), 1.0)),
        mul_up(size as f64, underflow),
    );
    let defect_input = mul_up(entry_radius_norm, inverse_norm);
    let defect_norm = add_up(defect_centre_norm, add_up(defect_rounding, defect_input));
    phases.defect_time += phase_started.elapsed();
    if defect_norm.partial_cmp(&1.0) != Some(std::cmp::Ordering::Less) {
        return None;
    }

    let phase_started = Instant::now();
    let decision = decision_from_certified_norms_with_inverse_norm(
        word,
        factor,
        residual_norm,
        defect_norm,
        inverse_norm,
    );
    phases.decision_time += phase_started.elapsed();
    decision
}

/// Experimental batched enclosure for the same inverse-defect theorem used by
/// `certify_direct_solution_profiled`.
///
/// Matrix products are ordinary f64 operations. Their exact real values are
/// enclosed afterward using a conservative dot-product rounding factor, an
/// explicit gradual-underflow allowance, and outward f64 reductions. Exact
/// rational arithmetic is not used here.
fn certify_direct_solution_batched_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    if !gradual_underflow {
        return None;
    }
    let size = matrix.nrows();
    let phase_started = Instant::now();
    let exact_entries = exact_kkt_intervals(duals, word);
    let entry_radii = DMatrix::from_fn(size, size, |row, col| {
        interval_radius_around(exact_entries[row * size + col], matrix[(row, col)])
    });
    if entry_radii.iter().any(|value| !value.is_finite()) {
        return None;
    }
    phases.entries_time += phase_started.elapsed();

    let abs_matrix = matrix.map(|value| value.abs());
    let abs_inverse = factor.inverse.map(|value| value.abs());
    let (gamma, underflow) = dot_product_error_parameters(size)?;

    let phase_started = Instant::now();
    let solution = DMatrix::from_column_slice(size, 1, &factor.solution);
    let abs_solution = solution.map(|value| value.abs());
    let mut residual_centre = matrix * &solution;
    residual_centre[(size - 1, 0)] -= 1.0;
    let residual_magnitude = positive_product_upper(&abs_matrix, &abs_solution, gamma, underflow)?;
    let residual_input = positive_product_upper(&entry_radii, &abs_solution, gamma, underflow)?;
    let mut residual_norm = 0.0_f64;
    for row in 0..size {
        // Treat the final subtraction of b as the last operation of an
        // augmented dot product. Only the final row has |b_i| = 1.
        let augmented_magnitude = add_up(
            residual_magnitude[(row, 0)],
            usize::from(row == size - 1) as f64,
        );
        let arithmetic_error = add_up(mul_up(gamma, augmented_magnitude), underflow);
        let residual_upper = add_up(
            next_up(residual_centre[(row, 0)].abs()),
            add_up(arithmetic_error, residual_input[(row, 0)]),
        );
        if !residual_upper.is_finite() {
            return None;
        }
        residual_norm = residual_norm.max(residual_upper);
    }
    phases.residual_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let defect_centre = DMatrix::identity(size, size) - matrix * &factor.inverse;
    let defect_magnitude = positive_product_upper(&abs_matrix, &abs_inverse, gamma, underflow)?;
    let defect_input = positive_product_upper(&entry_radii, &abs_inverse, gamma, underflow)?;
    let mut defect_norm = 0.0_f64;
    for row in 0..size {
        let mut row_sum = 0.0;
        for col in 0..size {
            // The subtraction from I is likewise the last operation of an
            // augmented dot product. Its additional magnitude is one on the
            // diagonal and zero elsewhere.
            let augmented_magnitude =
                add_up(defect_magnitude[(row, col)], usize::from(row == col) as f64);
            let arithmetic_error = add_up(mul_up(gamma, augmented_magnitude), underflow);
            let defect_upper = add_up(
                next_up(defect_centre[(row, col)].abs()),
                add_up(arithmetic_error, defect_input[(row, col)]),
            );
            if !defect_upper.is_finite() {
                return None;
            }
            row_sum = add_up(row_sum, defect_upper);
        }
        defect_norm = defect_norm.max(row_sum);
    }
    phases.defect_time += phase_started.elapsed();
    if defect_norm.partial_cmp(&1.0) != Some(std::cmp::Ordering::Less) {
        return None;
    }

    let phase_started = Instant::now();
    let decision = decision_from_certified_norms(word, factor, residual_norm, defect_norm);
    phases.decision_time += phase_started.elapsed();
    decision
}

/// Checks the two runtime modes that invalidate the subnormal error allowance:
/// flush-to-zero for subnormal outputs and denormals-are-zero for inputs.
///
/// `black_box` is essential: this is an arithmetic-environment check, not a
/// constant identity for LLVM to fold during a release build.
#[inline(never)]
fn gradual_underflow_available() -> bool {
    let minimum_normal = black_box(f64::MIN_POSITIVE);
    let half = black_box(0.5_f64);
    let expected_half_normal = f64::from_bits(1_u64 << 51);
    let half_normal = black_box(minimum_normal * half);

    let minimum_subnormal = black_box(f64::from_bits(1));
    let one = black_box(1.0_f64);
    let preserved_subnormal = black_box(minimum_subnormal * one);

    half_normal == expected_half_normal && preserved_subnormal == f64::from_bits(1)
}

fn interval_radius_around(interval: Interval, centre: f64) -> f64 {
    if interval.lo == centre && interval.hi == centre {
        0.0
    } else {
        next_up(
            (interval.lo - centre)
                .abs()
                .max((interval.hi - centre).abs()),
        )
    }
}

fn dot_product_error_parameters(term_count: usize) -> Option<(f64, f64)> {
    // A full machine epsilon, rather than half an epsilon, deliberately
    // overestimates the unit roundoff. The operation count also treats every
    // multiply and add separately, so fused operations only improve the bound.
    let operation_count = 2 * term_count;
    let scaled = mul_up(operation_count as f64, f64::EPSILON);
    if scaled.partial_cmp(&1.0) != Some(std::cmp::Ordering::Less) {
        return None;
    }
    let gamma = next_up(scaled / next_down(1.0 - scaled));
    let underflow = mul_up((2 * operation_count) as f64, f64::from_bits(1));
    Some((gamma, underflow))
}

/// Upper bound for an exact nonnegative matrix product from its ordinary f64
/// evaluation. For each dot product, `fl(s) >= (1-gamma)s-underflow`.
fn positive_product_upper(
    left: &DMatrix<f64>,
    right: &DMatrix<f64>,
    gamma: f64,
    underflow: f64,
) -> Option<DMatrix<f64>> {
    let computed = left * right;
    let denominator = next_down(1.0 - gamma);
    if denominator.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None;
    }
    let upper = computed.map(|value| next_up(add_up(value, underflow) / denominator));
    upper.iter().all(|value| value.is_finite()).then_some(upper)
}

fn decision_from_certified_norms(
    word: &[usize],
    factor: &FactorData,
    residual_norm: f64,
    defect_norm: f64,
) -> Option<Decision> {
    let inverse_norm = matrix_inf_norm_up(&factor.inverse);
    decision_from_certified_norms_with_inverse_norm(
        word,
        factor,
        residual_norm,
        defect_norm,
        inverse_norm,
    )
}

fn decision_from_certified_norms_with_inverse_norm(
    word: &[usize],
    factor: &FactorData,
    residual_norm: f64,
    defect_norm: f64,
    inverse_norm: f64,
) -> Option<Decision> {
    let inverse_bound = next_up(inverse_norm / next_down(1.0 - defect_norm));
    let beta_radius = mul_up(inverse_bound, residual_norm);
    let beta = &factor.solution[..word.len()];
    let beta_min = beta.iter().copied().fold(f64::INFINITY, f64::min);

    if beta_min < -beta_radius {
        return Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: None,
            q_lower: None,
            q_upper: None,
            exact_fallback: false,
        });
    }
    if beta_min.partial_cmp(&beta_radius) != Some(std::cmp::Ordering::Greater) {
        return None;
    }

    // The last KKT component is the normalization multiplier xi. At an exact
    // solution, stationarity and sum(beta)=1 give 2Q + xi = 0. The same
    // componentwise solution radius therefore encloses Q without another
    // quadratic-form evaluation or a separate perturbation formula.
    let xi = factor.solution[word.len() + 4];
    let xi_interval = Interval {
        lo: next_down(xi - beta_radius),
        hi: next_up(xi + beta_radius),
    };
    let q = Interval::point(-0.5).mul(xi_interval);
    if !q.is_valid_finite() {
        return None;
    }
    let q_lower = q.lo;
    let q_upper = q.hi;
    let q_centre = -0.5 * xi;
    let q_radius = next_up((q_centre - q_lower).abs().max((q_upper - q_centre).abs()));
    if q_lower > 0.0 {
        Some(Decision {
            kind: DecisionKind::Accept,
            action: Some(0.5 / q_centre),
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        })
    } else if q_upper <= 0.0 {
        Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        })
    } else {
        None
    }
}

fn exact_decision(exact_duals: &[[num_rational::BigRational; 4]], word: &[usize]) -> Decision {
    match solve_kkt_exact(exact_duals, word) {
        Some(result) if result.q_exact.is_positive() => {
            exact_positive_decision_from_q(&result.q_exact, true)
        }
        _ => Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: None,
            q_radius: None,
            q_lower: None,
            q_upper: None,
            exact_fallback: true,
        },
    }
}

fn exact_positive_decision_from_q(q: &BigRational, exact_fallback: bool) -> Decision {
    debug_assert!(q.is_positive());
    let action_exact = BigRational::one() / (q.clone() + q.clone());
    let (q_lower, q_upper) = exact_rational_to_f64_interval(q);
    Decision {
        kind: DecisionKind::Accept,
        action: Some(action_exact.to_f64().unwrap_or(f64::INFINITY)),
        beta_radius: None,
        q_radius: None,
        q_lower: Some(q_lower),
        q_upper: Some(q_upper),
        exact_fallback,
    }
}

fn exact_rational_to_f64_interval(value: &BigRational) -> (f64, f64) {
    let rounded = value.to_f64().unwrap_or_else(|| {
        if value.is_positive() {
            f64::INFINITY
        } else {
            f64::NEG_INFINITY
        }
    });
    if rounded == f64::INFINITY {
        return (0.0, f64::INFINITY);
    }
    if rounded == f64::NEG_INFINITY {
        return (f64::NEG_INFINITY, 0.0);
    }
    let rounded_exact =
        BigRational::from_float(rounded).expect("every finite f64 is an exact rational");
    match rounded_exact.cmp(value) {
        std::cmp::Ordering::Less => (rounded, next_up(rounded)),
        std::cmp::Ordering::Equal => (rounded, rounded),
        std::cmp::Ordering::Greater => (next_down(rounded), rounded),
    }
}

// ── Short-word exact route ───────────────────────────────────────────────

/// A nonzero (m+1)-minor of [C | d] proves rank([C | d]) > rank(C), hence
/// exact inconsistency. The interval contains the determinant of the exact
/// binary64 input, so this is a one-sided certificate, not a tolerance test.
fn certified_short_inconsistent(duals: &[Vector4<f64>], word: &[usize]) -> bool {
    let columns = word.len() + 1;
    if columns > 5 {
        return false;
    }
    let omissions = if columns == 5 {
        vec![None]
    } else {
        (0..5).map(Some).collect::<Vec<_>>()
    };
    for omitted_row in omissions {
        let rows = if let Some(omitted_row) = omitted_row {
            // For m=3 this enumerates the five 4-row subsets by their omitted
            // row. Other short lengths are not emitted by the cycle iterator.
            (0..5)
                .filter(|&row| row != omitted_row)
                .take(columns)
                .collect::<Vec<_>>()
        } else {
            (0..5).collect::<Vec<_>>()
        };
        if rows.len() != columns {
            continue;
        }
        let matrix = rows
            .iter()
            .map(|&row| {
                (0..columns)
                    .map(|col| {
                        if col == word.len() {
                            Interval::point(usize::from(row == 4) as f64)
                        } else {
                            Interval::point(constraint_entry(duals, word, row, col))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let determinant = interval_determinant(&matrix);
        if determinant.lo > 0.0 || determinant.hi < 0.0 {
            return true;
        }
    }
    false
}

fn interval_determinant(matrix: &[Vec<Interval>]) -> Interval {
    match matrix.len() {
        0 => Interval::point(1.0),
        1 => matrix[0][0],
        size => (0..size).fold(Interval::point(0.0), |sum, col| {
            let minor = matrix[1..]
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .filter_map(|(index, &value)| (index != col).then_some(value))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let term = matrix[0][col].mul(interval_determinant(&minor));
            if col % 2 == 0 {
                sum.add(term)
            } else {
                sum.sub(term)
            }
        }),
    }
}

/// For m < 5, full-column-rank C makes the affine feasible set a point.
/// Solving only C beta = d is therefore sufficient: stationarity multipliers
/// exist because C^T is onto, and positivity/Q can be decided exactly.
fn short_exact_decision(exact_duals: &[[BigRational; 4]], word: &[usize]) -> Decision {
    let m = word.len();
    let matrix = DMatrix::from_fn(5, m, |row, col| {
        if row == 4 {
            BigRational::one()
        } else {
            exact_duals[word[col]][row].clone()
        }
    });
    let mut rhs = DVector::from_element(5, BigRational::zero());
    rhs[4] = BigRational::one();
    let beta = match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => {
            return rejected_exact_decision();
        }
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => particular,
        // Rank-deficient short supports are outside this shortcut's premise;
        // the general exact solver remains the complete fallback.
        LinearSystemSolution::Consistent { .. } => return exact_decision(exact_duals, word),
    };
    if !beta.iter().all(BigRational::is_positive) {
        return rejected_exact_decision();
    }
    let mut q = BigRational::zero();
    for i in 0..m {
        for j in i + 1..m {
            q += beta[i].clone()
                * beta[j].clone()
                * omega_exact(&exact_duals[word[i]], &exact_duals[word[j]]);
        }
    }
    if !q.is_positive() {
        return rejected_exact_decision();
    }
    exact_positive_decision_from_q(&q, false)
}

fn rejected_exact_decision() -> Decision {
    Decision {
        kind: DecisionKind::Reject,
        action: None,
        beta_radius: None,
        q_radius: None,
        q_lower: None,
        q_upper: None,
        exact_fallback: false,
    }
}

fn omega_exact(left: &[BigRational; 4], right: &[BigRational; 4]) -> BigRational {
    left[0].clone() * right[2].clone() - left[2].clone() * right[0].clone()
        + left[1].clone() * right[3].clone()
        - left[3].clone() * right[1].clone()
}

// ── Exact KKT-entry enclosures ───────────────────────────────────────────

fn exact_kkt_intervals(duals: &[Vector4<f64>], word: &[usize]) -> Vec<Interval> {
    let m = word.len();
    let size = m + 5;
    let mut entries = vec![Interval::point(0.0); size * size];
    for i in 0..m {
        for j in i + 1..m {
            let value = omega_interval(&duals[word[i]], &duals[word[j]]);
            entries[i * size + j] = value;
            entries[j * size + i] = value;
        }
        for dim in 0..4 {
            let value = Interval::point(duals[word[i]][dim]);
            entries[i * size + m + dim] = value;
            entries[(m + dim) * size + i] = value;
        }
        entries[i * size + m + 4] = Interval::point(1.0);
        entries[(m + 4) * size + i] = Interval::point(1.0);
    }
    entries
}

/// Infinity norm of the entrywise distance between the exact binary64-input
/// KKT matrix and its ordinary f64 assembly.
///
/// Constraint, identity, and zero entries are copied exactly. Only the omega
/// block incurs roundoff. A single bound from the largest coordinate and word
/// length avoids allocating or scanning a dense interval matrix.
fn exact_kkt_entry_radius_inf_norm(duals: &[Vector4<f64>], word: &[usize]) -> Option<f64> {
    let coordinate_bound = word
        .iter()
        .flat_map(|&label| duals[label].iter())
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    let pair_magnitude = mul_up(4.0, mul_up(coordinate_bound, coordinate_bound));
    let (gamma, underflow) = dot_product_error_parameters(4)?;
    let per_entry = add_up(mul_up(gamma, pair_magnitude), underflow);
    let row_entries = word.len().saturating_sub(1) as f64;
    let norm = mul_up(row_entries, per_entry);
    norm.is_finite().then_some(norm)
}

fn q_interval_and_h_norm(duals: &[Vector4<f64>], word: &[usize], beta: &[f64]) -> (Interval, f64) {
    let mut q = Interval::point(0.0);
    let mut row_sums = vec![0.0; word.len()];
    for i in 0..word.len() {
        for j in i + 1..word.len() {
            let omega = omega_interval(&duals[word[i]], &duals[word[j]]);
            q = q.add(
                Interval::point(beta[i])
                    .mul(Interval::point(beta[j]))
                    .mul(omega),
            );
            let magnitude = omega.abs_upper();
            row_sums[i] = add_up(row_sums[i], magnitude);
            row_sums[j] = add_up(row_sums[j], magnitude);
        }
    }
    (q, row_sums.into_iter().fold(0.0, f64::max))
}

// ── Certified curvature obstructions and inheritance ─────────────────────

fn reduced_curvature_proposal(duals: &[Vector4<f64>], word: &[usize]) -> Option<CurvatureProposal> {
    let qp = build_qp_from_dual_vertices(duals, word);
    // nalgebra returns the thin right factor for a 5 x m matrix. Padding to a
    // square-or-tall matrix exposes every right-null direction when m > 5.
    let mut padded = DMatrix::zeros(word.len().max(5), word.len());
    padded.view_mut((0, 0), (5, word.len())).copy_from(&qp.c);
    let svd = padded.svd(true, true);
    let max_sv = svd.singular_values.iter().copied().fold(0.0, f64::max);
    let floor = (max_sv * 1e-12).max(1e-14);
    let rank = svd
        .singular_values
        .iter()
        .filter(|&&value| value > floor)
        .count();
    let nullity = word.len().saturating_sub(rank);
    if nullity == 0 {
        return None;
    }
    let vt = svd.v_t?;
    let mut basis = DMatrix::zeros(word.len(), nullity);
    for col in 0..nullity {
        for row in 0..word.len() {
            basis[(row, col)] = vt[(rank + col, row)];
        }
    }
    let reduced = basis.transpose() * qp.h * &basis;
    let eigen = reduced.symmetric_eigen();
    let (index, value) = eigen
        .eigenvalues
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, left), (_, right)| left.total_cmp(right))?;
    if value <= 0.0 {
        return None;
    }
    let direction = (&basis * eigen.eigenvectors.column(index))
        .iter()
        .copied()
        .collect();
    Some(CurvatureProposal { direction })
}

fn certify_curvature(duals: &[Vector4<f64>], word: &[usize], direction: &[f64]) -> bool {
    // Implements lem:kkt-certified-curvature-direction: project the numerical
    // proposal into ker(C) through a verified right-inverse bound, then prove
    // the exact projected direction retains positive H-curvature.
    if direction.len() != word.len() || direction.iter().any(|value| !value.is_finite()) {
        return false;
    }
    let mut residual_norm = 0.0_f64;
    for row in 0..5 {
        let mut residual = Interval::point(0.0);
        for (col, &component) in direction.iter().enumerate() {
            residual = residual.add(
                Interval::point(constraint_entry(duals, word, row, col))
                    .mul(Interval::point(component)),
            );
        }
        residual_norm = residual_norm.max(residual.abs_upper());
    }
    let Some(inverse_bound) = constraint_right_inverse_bound(duals, word) else {
        return false;
    };
    let correction = mul_up(inverse_bound, residual_norm);
    let (half_quadratic, h_norm) = q_interval_and_h_norm(duals, word, direction);
    let quadratic = Interval::point(2.0).mul(half_quadratic);
    let direction_norm = direction.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let error = mul_up(
        word.len() as f64,
        mul_up(
            h_norm,
            add_up(
                mul_up(2.0, mul_up(direction_norm, correction)),
                mul_up(correction, correction),
            ),
        ),
    );
    quadratic.lo > error
}

fn has_certified_rank_five_constraints(duals: &[Vector4<f64>], word: &[usize]) -> bool {
    constraint_right_inverse_bound(duals, word).is_some()
}

fn constraint_right_inverse_bound(duals: &[Vector4<f64>], word: &[usize]) -> Option<f64> {
    if word.len() < 5 {
        return None;
    }
    for a in 0..word.len() - 4 {
        for b in a + 1..word.len() - 3 {
            for c in b + 1..word.len() - 2 {
                for d in c + 1..word.len() - 1 {
                    for e in d + 1..word.len() {
                        if let Some(bound) = inverse_bound_for_pivots(duals, word, [a, b, c, d, e])
                        {
                            return Some(bound);
                        }
                    }
                }
            }
        }
    }
    None
}

fn inverse_bound_for_pivots(
    duals: &[Vector4<f64>],
    word: &[usize],
    pivots: [usize; 5],
) -> Option<f64> {
    let matrix = DMatrix::from_fn(5, 5, |row, col| {
        constraint_entry(duals, word, row, pivots[col])
    });
    let inverse = matrix.clone().try_inverse()?;
    if inverse.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let inverse_norm = matrix_inf_norm_up(&inverse);
    let mut defect_norm = 0.0_f64;
    for row in 0..5 {
        let mut row_sum = 0.0;
        for col in 0..5 {
            let mut product = Interval::point(0.0);
            for mid in 0..5 {
                product = product.add(
                    Interval::point(matrix[(row, mid)]).mul(Interval::point(inverse[(mid, col)])),
                );
            }
            let defect = Interval::point(usize::from(row == col) as f64).sub(product);
            row_sum = add_up(row_sum, defect.abs_upper());
        }
        defect_norm = defect_norm.max(row_sum);
    }
    if defect_norm.partial_cmp(&1.0) != Some(std::cmp::Ordering::Less) {
        return None;
    }
    Some(next_up(inverse_norm / next_down(1.0 - defect_norm)))
}

fn constraint_entry(duals: &[Vector4<f64>], word: &[usize], row: usize, col: usize) -> f64 {
    if row == 4 {
        1.0
    } else {
        duals[word[col]][row]
    }
}

fn omega_interval(left: &Vector4<f64>, right: &Vector4<f64>) -> Interval {
    Interval::point(left[0])
        .mul(Interval::point(right[2]))
        .sub(Interval::point(left[2]).mul(Interval::point(right[0])))
        .add(Interval::point(left[1]).mul(Interval::point(right[3])))
        .sub(Interval::point(left[3]).mul(Interval::point(right[1])))
}

fn contains_certified_subword(word: &[usize], cache: &[Obstruction]) -> bool {
    // lem:kkt-cyclic-obstruction-inheritance permits exactly the
    // cyclic-order-preserving embeddings recognized below.
    let word_mask = label_mask(word);
    let mut positions = [usize::MAX; 16];
    for (position, &label) in word.iter().enumerate() {
        assert!(label < 16, "bit-mask lookup requires F <= 16");
        positions[label] = position;
    }
    cache.iter().any(|obstruction| {
        obstruction.labels.len() < word.len()
            && word_mask & obstruction.mask == obstruction.mask
            && cyclic_order_is_preserved(&obstruction.labels, &positions)
    })
}

fn label_mask(word: &[usize]) -> u16 {
    word.iter().fold(0u16, |mask, &label| {
        assert!(label < 16, "bit-mask lookup requires F <= 16");
        mask | (1u16 << label)
    })
}

/// For distinct labels on a circle, one cyclic order embeds in another iff
/// their positions have exactly one cyclic descent.
fn cyclic_order_is_preserved(labels: &[usize], positions: &[usize; 16]) -> bool {
    labels
        .iter()
        .zip(labels.iter().cycle().skip(1))
        .take(labels.len())
        .filter(|(left, right)| positions[**left] > positions[**right])
        .count()
        == 1
}

// ── Outward binary64 helpers and diagnostics ─────────────────────────────

fn matrix_inf_norm_up(matrix: &DMatrix<f64>) -> f64 {
    (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols()).fold(0.0, |sum, col| {
                add_up(sum, next_up(matrix[(row, col)].abs()))
            })
        })
        .fold(0.0, f64::max)
}

fn next_up(value: f64) -> f64 {
    if value.is_nan() || value == f64::INFINITY {
        return value;
    }
    if value == 0.0 {
        return f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits + 1 } else { bits - 1 })
}

fn next_down(value: f64) -> f64 {
    if value.is_nan() || value == f64::NEG_INFINITY {
        return value;
    }
    if value == 0.0 {
        return -f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits - 1 } else { bits + 1 })
}

fn add_up(left: f64, right: f64) -> f64 {
    next_up(left + right)
}

fn mul_up(left: f64, right: f64) -> f64 {
    next_up(left * right)
}

fn exact_binary64_dual_vertex_arrays(duals: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
    duals
        .iter()
        .map(|vertex| {
            [
                f64_to_rational(vertex[0]),
                f64_to_rational(vertex[1]),
                f64_to_rational(vertex[2]),
                f64_to_rational(vertex[3]),
            ]
        })
        .collect()
}

pub(crate) fn solve_selected_general(
    duals: &[Vector4<f64>],
    words: Vec<Vec<usize>>,
) -> Option<(f64, f64)> {
    solve_selected_general_requested(duals, words, GeneralOutputRequest::CapacityOnly)
        .expect("the scalar request never exact-resolves contenders")
        .map(|output| output.bounds)
}

pub(super) fn solve_selected_general_minimizers(
    duals: &[Vector4<f64>],
    words: Vec<Vec<usize>>,
) -> Result<Option<GeneralSolveOutput>, Vec<usize>> {
    solve_selected_general_requested(
        duals,
        words,
        GeneralOutputRequest::ExactWithinCapacityMultiple(BigRational::one()),
    )
}

pub(super) fn solve_selected_general_action_window(
    duals: &[Vector4<f64>],
    words: Vec<Vec<usize>>,
    maximum_action_multiple: BigRational,
) -> Result<Option<GeneralSolveOutput>, Vec<usize>> {
    debug_assert!(maximum_action_multiple >= BigRational::one());
    solve_selected_general_requested(
        duals,
        words,
        GeneralOutputRequest::ExactWithinCapacityMultiple(maximum_action_multiple),
    )
}

fn solve_selected_general_requested(
    duals: &[Vector4<f64>],
    words: Vec<Vec<usize>>,
    request: GeneralOutputRequest,
) -> Result<Option<GeneralSolveOutput>, Vec<usize>> {
    let cases = vec![(String::new(), duals.to_vec(), words)];
    let result = run_selected_route(&cases);
    let Some(bounds) = result.best_action_lower.zip(result.best_action_upper) else {
        return Ok(None);
    };
    let maximum_action_multiple = match request {
        GeneralOutputRequest::CapacityOnly => {
            return Ok(Some(GeneralSolveOutput {
                bounds,
                exact_selection: None,
            }));
        }
        GeneralOutputRequest::ExactWithinCapacityMultiple(value) => value,
    };

    let words = &cases[0].2;
    let maximum_lower = result
        .selected_decisions
        .iter()
        .filter(|decision| decision.kind == DecisionKind::Accept)
        .filter_map(|decision| decision.q_lower)
        .max_by(f64::total_cmp)
        .expect("a positive general result has an accepted q interval");
    let exact_duals = exact_binary64_dual_vertex_arrays(duals);
    let mut resolved = vec![None; words.len()];
    for (index, (word, decision)) in words.iter().zip(&result.selected_decisions).enumerate() {
        if decision.kind != DecisionKind::Accept
            || decision.q_upper.is_none_or(|upper| upper < maximum_lower)
        {
            continue;
        }
        let Some(exact) = solve_kkt_exact(&exact_duals, word) else {
            return Err(word.clone());
        };
        if !exact.q_exact.is_positive() {
            return Err(word.clone());
        }
        resolved[index] = Some(exact);
    }
    let maximum_q = resolved
        .iter()
        .flatten()
        .map(|exact| &exact.q_exact)
        .max()
        .expect("a positive general result has an exact contender")
        .clone();
    let capacity_exact = BigRational::one() / (maximum_q.clone() + maximum_q.clone());
    let minimum_q_in_window = &maximum_q / maximum_action_multiple;

    let mut candidates = Vec::new();
    for (index, (word, decision)) in words.iter().zip(&result.selected_decisions).enumerate() {
        if decision.kind != DecisionKind::Accept
            || decision.q_upper.is_some_and(|upper| {
                BigRational::from_float(upper)
                    .is_some_and(|upper_exact| upper_exact < minimum_q_in_window)
            })
        {
            continue;
        }
        let exact = match resolved[index].take() {
            Some(value) => value,
            None => {
                let Some(exact) = solve_kkt_exact(&exact_duals, word) else {
                    return Err(word.clone());
                };
                if !exact.q_exact.is_positive() {
                    return Err(word.clone());
                }
                exact
            }
        };
        if exact.q_exact < minimum_q_in_window {
            continue;
        }
        candidates.push(GeneralExactCandidate {
            witness: ExactOrbitKktData {
                sigma: word.clone(),
                beta: exact.beta,
                q: exact.q_exact,
                mu: Vector4::new(
                    exact.mu[0].clone(),
                    exact.mu[1].clone(),
                    exact.mu[2].clone(),
                    exact.mu[3].clone(),
                ),
                xi: exact.xi,
            },
        });
    }
    candidates.sort_by(|left, right| {
        left.witness
            .action()
            .cmp(&right.witness.action())
            .then_with(|| left.witness.sigma.cmp(&right.witness.sigma))
    });

    Ok(Some(GeneralSolveOutput {
        bounds,
        exact_selection: Some(GeneralExactSelection {
            capacity_exact,
            candidates,
        }),
    }))
}
