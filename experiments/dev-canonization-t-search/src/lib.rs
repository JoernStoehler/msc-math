use euclidean_polytopes::volume_from_incidence_f64;
use nalgebra::{DMatrix, Matrix2, Matrix4, Vector4};
use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::cmp::Ordering;
use std::panic::{catch_unwind, AssertUnwindSafe};

pub mod candidates;
pub mod metrics;

const ANALYTIC_CENTER_MAX_ITER: usize = 50;
const ANALYTIC_CENTER_TOL: f64 = 1e-12;
const MIN_SLACK: f64 = 1e-10;
pub const RESIDUAL_FAILURE_THRESHOLD: f64 = 1e-5;

#[derive(Clone, Debug)]
pub struct Case {
    pub case_id: String,
    pub duals: Vec<Vector4<f64>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SummaryStats {
    pub mean: f64,
    pub median: f64,
    pub p90: f64,
    pub max: f64,
}

#[derive(Clone, Debug)]
pub struct CandidateOutput {
    /// Canonicalized normalized facet-normal rows. A registered `T` candidate
    /// must preserve the row count and return coordinate rows, not an invariant
    /// matrix with unrelated dimensions.
    pub duals: Vec<Vector4<f64>>,
    /// `ok` means the candidate claims its construction succeeded on this
    /// input. Any other status is a mathematically valid non-success or an f64
    /// prototype failure; the stochastic report counts these separately.
    pub status: &'static str,
}

#[derive(Clone, Copy)]
pub struct CandidateSpec {
    pub label: &'static str,
    pub canonicalize: fn(&[Vector4<f64>]) -> CandidateOutput,
}

#[derive(Clone, Copy)]
pub struct MetricSpec {
    pub label: &'static str,
    pub distance: fn(&[Vector4<f64>], &[Vector4<f64>]) -> f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct CandidateMetricSummary {
    pub candidate: String,
    pub metric: String,
    pub families: std::collections::BTreeMap<String, FamilySummary>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FamilySummary {
    pub raw: SummaryStats,
    pub candidate_canonicalized: SummaryStats,
    pub ok_ok_candidate_canonicalized: Option<SummaryStats>,
    pub base_status_denominator: usize,
    pub base_status_counts: std::collections::BTreeMap<String, usize>,
    pub transformed_status_denominator: usize,
    pub transformed_status_counts: std::collections::BTreeMap<String, usize>,
    pub residual_pair_count: usize,
    pub ok_ok_pair_count: usize,
    pub residual_failure_threshold: f64,
    pub total_failures_above_threshold: usize,
    pub largest_failures: Vec<FailureExample>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FailureExample {
    pub case_id: String,
    pub sample_index: usize,
    pub raw_distance: f64,
    pub canonicalized_distance: f64,
    pub base_candidate_status: String,
    pub transformed_candidate_status: String,
}

#[derive(Clone, Copy, Debug)]
pub enum TransformFamily {
    Scale,
    Translation,
    FacetPermutation,
    ScaleTranslationPermutation,
    SymplecticBlock,
    SymplecticExp,
    FullGroupSample,
}

impl TransformFamily {
    pub fn label(self) -> &'static str {
        match self {
            Self::Scale => "scale",
            Self::Translation => "translation",
            Self::FacetPermutation => "facet_permutation",
            Self::ScaleTranslationPermutation => "scale_translation_permutation",
            Self::SymplecticBlock => "sampled_block_symplectic",
            Self::SymplecticExp => "sampled_sp4_exp",
            Self::FullGroupSample => "sampled_full_group",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub scale: f64,
    pub translation_radius: f64,
    pub permute: bool,
    pub symplectic: Option<Matrix4<f64>>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: 1.0,
            translation_radius: 0.0,
            permute: false,
            symplectic: None,
        }
    }
}

pub fn accepted_random_cases(count: usize, seed: u64) -> Vec<Case> {
    let mut cases = Vec::with_capacity(count);
    let mut attempt = 0;
    while cases.len() < count {
        let facet_count = 8 + 2 * (cases.len() % 4);
        if let Ok(duals) =
            symplectic::random::generate_dual_vertices(facet_count, 0.55, 1.85, seed, attempt)
        {
            cases.push(Case {
                case_id: format!("accepted-random-{attempt:06}-F{facet_count}"),
                duals,
            });
        }
        attempt += 1;
        assert!(
            attempt < 200_000,
            "failed to generate {count} accepted random cases before attempt limit"
        );
    }
    cases
}

pub fn analytic_center(duals: &[Vector4<f64>]) -> (Vector4<f64>, &'static str) {
    let mut center = Vector4::zeros();
    for _ in 0..ANALYTIC_CENTER_MAX_ITER {
        let mut gradient = Vector4::zeros();
        let mut hessian = Matrix4::zeros();
        let mut min_slack = f64::INFINITY;
        for dual in duals {
            let slack = 1.0 - dual.dot(&center);
            min_slack = min_slack.min(slack);
            if slack <= MIN_SLACK {
                return (Vector4::zeros(), "nonpositive_slack");
            }
            let weighted = dual / slack;
            gradient += weighted;
            hessian += weighted * weighted.transpose();
        }
        let Some(step) = hessian.lu().solve(&gradient) else {
            return (Vector4::zeros(), "singular_hessian");
        };
        let decrement = gradient.dot(&step);
        if !decrement.is_finite() {
            return (Vector4::zeros(), "nonfinite_newton");
        }
        if decrement < ANALYTIC_CENTER_TOL {
            return (center, "ok");
        }
        let mut step_size = 1.0;
        loop {
            let candidate = center - step_size * step;
            let candidate_min_slack = duals
                .iter()
                .map(|dual| 1.0 - dual.dot(&candidate))
                .fold(f64::INFINITY, f64::min);
            if candidate_min_slack > MIN_SLACK {
                center = candidate;
                break;
            }
            step_size *= 0.5;
            if step_size <= 1e-12 {
                return (Vector4::zeros(), "line_search_failed");
            }
        }
        if min_slack <= MIN_SLACK {
            return (Vector4::zeros(), "nonpositive_slack");
        }
    }
    (center, "max_iter")
}

pub fn translate_duals(
    duals: &[Vector4<f64>],
    center: &Vector4<f64>,
) -> Result<Vec<Vector4<f64>>, String> {
    let mut translated = Vec::with_capacity(duals.len());
    for dual in duals {
        let denominator = 1.0 - dual.dot(center);
        if !denominator.is_finite() || denominator <= MIN_SLACK {
            return Err("translation center is not safely interior".to_string());
        }
        translated.push(dual / denominator);
    }
    Ok(translated)
}

pub fn volume_one_duals_f64(duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let volume = volume_from_normalized_duals_f64(duals)?;
    if !volume.is_finite() || volume <= 0.0 {
        return None;
    }
    let dual_scale = volume.powf(0.25);
    Some(duals.iter().map(|dual| dual * dual_scale).collect())
}

pub fn volume_from_normalized_duals_f64(duals: &[Vector4<f64>]) -> Option<f64> {
    let vertices = enumerate_vertices_f64(duals)?;
    if vertices.len() < 5 {
        return None;
    }
    let incidence = approximate_incidence(duals, &vertices);
    catch_unwind(AssertUnwindSafe(|| {
        volume_from_incidence_f64(&vertices, &incidence).ok()
    }))
    .ok()
    .flatten()
}

fn enumerate_vertices_f64(duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    if duals.len() < 5
        || !duals
            .iter()
            .all(|dual| dual.iter().all(|value| value.is_finite()))
    {
        return None;
    }
    let mut vertices = Vec::new();
    for i in 0..duals.len() {
        for j in (i + 1)..duals.len() {
            for k in (j + 1)..duals.len() {
                for l in (k + 1)..duals.len() {
                    let matrix = Matrix4::new(
                        duals[i][0],
                        duals[i][1],
                        duals[i][2],
                        duals[i][3],
                        duals[j][0],
                        duals[j][1],
                        duals[j][2],
                        duals[j][3],
                        duals[k][0],
                        duals[k][1],
                        duals[k][2],
                        duals[k][3],
                        duals[l][0],
                        duals[l][1],
                        duals[l][2],
                        duals[l][3],
                    );
                    if matrix.determinant().abs() < 1e-12 {
                        continue;
                    }
                    let Some(candidate) = matrix.lu().solve(&Vector4::repeat(1.0)) else {
                        continue;
                    };
                    if !candidate.iter().all(|value| value.is_finite()) {
                        continue;
                    }
                    let feasible = duals.iter().all(|dual| {
                        let tolerance = 1e-8 * (1.0 + dual.norm() * candidate.norm());
                        dual.dot(&candidate) <= 1.0 + tolerance
                    });
                    if feasible
                        && vertices.iter().all(|known: &Vector4<f64>| {
                            (known - candidate).norm() > 1e-7 * (1.0 + known.norm())
                        })
                    {
                        vertices.push(candidate);
                    }
                }
            }
        }
    }
    Some(vertices)
}

fn approximate_incidence(duals: &[Vector4<f64>], vertices: &[Vector4<f64>]) -> DMatrix<bool> {
    DMatrix::from_fn(vertices.len(), duals.len(), |row, col| {
        let value = duals[col].dot(&vertices[row]);
        let tolerance = 1e-7 * (1.0 + duals[col].norm() * vertices[row].norm());
        (value - 1.0).abs() <= tolerance
    })
}

pub fn candidate_canonicalize(duals: &[Vector4<f64>]) -> CandidateOutput {
    let (center, status) = analytic_center(duals);
    let shifted = if status == "ok" {
        translate_duals(duals, &center).unwrap_or_else(|_| duals.to_vec())
    } else {
        duals.to_vec()
    };
    let rms = rms_dual_norm(&shifted);
    assert!(rms.is_finite() && rms > 0.0, "bad rms scale");
    let mut normalized = shifted
        .into_iter()
        .map(|dual| dual / rms)
        .collect::<Vec<_>>();
    normalized.sort_by(compare_vectors_lexicographically_for_candidates);
    CandidateOutput {
        duals: normalized,
        status,
    }
}

pub fn rms_dual_norm(duals: &[Vector4<f64>]) -> f64 {
    (duals.iter().map(|dual| dual.norm_squared()).sum::<f64>() / duals.len() as f64).sqrt()
}

pub fn compare_vectors_lexicographically_for_candidates(
    left: &Vector4<f64>,
    right: &Vector4<f64>,
) -> Ordering {
    for index in 0..4 {
        let left_rounded = (left[index] * 1e12).round();
        let right_rounded = (right[index] * 1e12).round();
        match left_rounded.total_cmp(&right_rounded) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    Ordering::Equal
}

/// Diagnostic distance, not a proved mathematical metric.
///
/// It is symmetric and row-permutation insensitive. It can under-report
/// failures when distinct facets are very close.
pub fn nearest_neighbor_rms(a: &[Vector4<f64>], b: &[Vector4<f64>]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    let forward = a
        .iter()
        .map(|left| {
            b.iter()
                .map(|right| (left - right).norm())
                .fold(f64::INFINITY, f64::min)
        })
        .collect::<Vec<_>>();
    let backward = b
        .iter()
        .map(|right| {
            a.iter()
                .map(|left| (left - right).norm())
                .fold(f64::INFINITY, f64::min)
        })
        .collect::<Vec<_>>();
    let forward_mean =
        forward.iter().map(|value| value * value).sum::<f64>() / forward.len() as f64;
    let backward_mean =
        backward.iter().map(|value| value * value).sum::<f64>() / backward.len() as f64;
    let scale = 1.0_f64.max(rms_dual_norm(a)).max(rms_dual_norm(b));
    (((forward_mean + backward_mean) / 2.0).sqrt()) / scale
}

pub fn sample_transform(family: TransformFamily, rng: &mut ChaCha8Rng) -> Transform {
    match family {
        TransformFamily::Scale => Transform {
            scale: rng.gen_range(-1.4_f64..1.4).exp(),
            ..Transform::default()
        },
        TransformFamily::Translation => Transform {
            translation_radius: rng.gen_range(0.02..0.24),
            ..Transform::default()
        },
        TransformFamily::FacetPermutation => Transform {
            permute: true,
            ..Transform::default()
        },
        TransformFamily::ScaleTranslationPermutation => Transform {
            scale: rng.gen_range(-1.2_f64..1.2).exp(),
            translation_radius: rng.gen_range(0.02..0.2),
            permute: true,
            ..Transform::default()
        },
        TransformFamily::SymplecticBlock => Transform {
            symplectic: Some(random_symplectic_block(rng)),
            ..Transform::default()
        },
        TransformFamily::SymplecticExp => Transform {
            symplectic: Some(random_symplectic_exp(rng)),
            ..Transform::default()
        },
        TransformFamily::FullGroupSample => Transform {
            scale: rng.gen_range(-1.2_f64..1.2).exp(),
            translation_radius: rng.gen_range(0.02..0.18),
            permute: true,
            symplectic: Some(random_symplectic_exp(rng)),
        },
    }
}

pub fn transform_duals(
    duals: &[Vector4<f64>],
    transform: &Transform,
    rng: &mut ChaCha8Rng,
) -> Vec<Vector4<f64>> {
    let mut transformed = duals.to_vec();
    if let Some(symplectic) = &transform.symplectic {
        let inverse = symplectic
            .try_inverse()
            .expect("sampled symplectic block is invertible");
        transformed = transformed
            .iter()
            .map(|dual| inverse.transpose() * dual)
            .collect();
    }
    if transform.scale != 1.0 {
        transformed = transformed
            .iter()
            .map(|dual| dual * transform.scale)
            .collect();
    }
    if transform.translation_radius > 0.0 {
        let mut direction = Vector4::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        direction /= direction.norm();
        let max_dot = transformed
            .iter()
            .map(|dual| dual.dot(&direction))
            .fold(f64::NEG_INFINITY, f64::max);
        let radius = if max_dot <= 1e-12 {
            transform.translation_radius
        } else {
            transform.translation_radius.min(0.35 / max_dot)
        };
        transformed = translate_duals(&transformed, &(radius * direction))
            .expect("sampled translation should stay interior");
    }
    if transform.permute {
        transformed.shuffle(rng);
    }
    transformed
}

/// Samples only a low-dimensional block subgroup of `Sp(4)`.
///
/// The matrix is `diag(A, A^{-T})`. This is useful for falsifying a candidate
/// that already fails on this subgroup, but it is not Haar-like random sampling
/// from all of `Sp(4)`.
pub fn random_symplectic_block(rng: &mut ChaCha8Rng) -> Matrix4<f64> {
    let theta = rng.gen_range(-std::f64::consts::PI..std::f64::consts::PI);
    let shear = rng.gen_range(-0.9..0.9);
    let stretch = rng.gen_range(-0.55_f64..0.55);
    let rotation = Matrix2::new(theta.cos(), -theta.sin(), theta.sin(), theta.cos());
    let diagonal = Matrix2::new(stretch.exp(), 0.0, 0.0, (-0.4 * stretch).exp());
    let upper = Matrix2::new(1.0, shear, 0.0, 1.0);
    let a = rotation * diagonal * upper;
    let a_inv_t = a.try_inverse().expect("2x2 block invertible").transpose();
    let mut out = Matrix4::zeros();
    out.fixed_view_mut::<2, 2>(0, 0).copy_from(&a);
    out.fixed_view_mut::<2, 2>(2, 2).copy_from(&a_inv_t);
    out
}

/// Samples from a full-dimensional local family in `Sp(4)`.
///
/// If `H` is symmetric, then `X = J H` is in the symplectic Lie algebra. The
/// exponential `exp(X)` lies in the identity component of `Sp(4)`. Since
/// `Sp(4,R)` is connected, there is no additional finite component quotient to
/// sample. This is not Haar-like sampling on the noncompact group, but unlike
/// the block sampler it moves in all ten Lie-algebra directions.
pub fn random_symplectic_exp(rng: &mut ChaCha8Rng) -> Matrix4<f64> {
    let x = random_sp4_lie_algebra_element(rng);
    x.exp()
}

pub fn random_sp4_lie_algebra_element(rng: &mut ChaCha8Rng) -> Matrix4<f64> {
    let mut h = Matrix4::zeros();
    for row in 0..4 {
        for col in row..4 {
            let value = rng.gen_range(-0.65..0.65);
            h[(row, col)] = value;
            h[(col, row)] = value;
        }
    }
    let j = standard_symplectic_matrix();
    j * h
}

pub fn standard_symplectic_matrix() -> Matrix4<f64> {
    Matrix4::new(
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
        -1.0, 0.0, 0.0, 0.0, //
        0.0, -1.0, 0.0, 0.0,
    )
}

pub fn symplectic_defect(matrix: &Matrix4<f64>) -> f64 {
    let j = standard_symplectic_matrix();
    (matrix.transpose() * j * matrix - j).norm()
}

pub fn symplectic_lie_algebra_defect(matrix: &Matrix4<f64>) -> f64 {
    let j = standard_symplectic_matrix();
    (matrix.transpose() * j + j * matrix).norm()
}

pub fn summarize(values: &[f64]) -> SummaryStats {
    assert!(!values.is_empty(), "cannot summarize empty values");
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    SummaryStats {
        mean: sorted.iter().sum::<f64>() / sorted.len() as f64,
        median: quantile(&sorted, 0.5),
        p90: quantile(&sorted, 0.9),
        max: *sorted.last().unwrap(),
    }
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let index = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[index]
}

pub fn score_family(
    cases: &[Case],
    family: TransformFamily,
    candidate: CandidateSpec,
    metric: MetricSpec,
    samples_per_case: usize,
    rng: &mut ChaCha8Rng,
) -> FamilySummary {
    let mut raw_distances = Vec::new();
    let mut canonicalized_distances = Vec::new();
    let mut ok_ok_canonicalized_distances = Vec::new();
    let mut failures = Vec::new();
    let mut base_status_counts = std::collections::BTreeMap::new();
    let mut transformed_status_counts = std::collections::BTreeMap::new();
    for case in cases {
        let base_canon = (candidate.canonicalize)(&case.duals);
        increment_status(&mut base_status_counts, base_canon.status);
        for sample_index in 0..samples_per_case {
            let transform = sample_transform(family, rng);
            let transformed = transform_duals(&case.duals, &transform, rng);
            let transformed_canon = (candidate.canonicalize)(&transformed);
            increment_status(&mut transformed_status_counts, transformed_canon.status);
            let raw_distance = (metric.distance)(&case.duals, &transformed);
            let canonicalized_distance =
                (metric.distance)(&base_canon.duals, &transformed_canon.duals);
            raw_distances.push(raw_distance);
            canonicalized_distances.push(canonicalized_distance);
            if base_canon.status == "ok" && transformed_canon.status == "ok" {
                ok_ok_canonicalized_distances.push(canonicalized_distance);
            }
            if canonicalized_distance > RESIDUAL_FAILURE_THRESHOLD {
                failures.push(FailureExample {
                    case_id: case.case_id.clone(),
                    sample_index,
                    raw_distance,
                    canonicalized_distance,
                    base_candidate_status: base_canon.status.to_string(),
                    transformed_candidate_status: transformed_canon.status.to_string(),
                });
            }
        }
    }
    let total_failures_above_threshold = failures.len();
    failures.sort_by(|left, right| {
        right
            .canonicalized_distance
            .total_cmp(&left.canonicalized_distance)
    });
    failures.truncate(8);
    FamilySummary {
        raw: summarize(&raw_distances),
        candidate_canonicalized: summarize(&canonicalized_distances),
        ok_ok_candidate_canonicalized: (!ok_ok_canonicalized_distances.is_empty())
            .then(|| summarize(&ok_ok_canonicalized_distances)),
        base_status_denominator: cases.len(),
        base_status_counts,
        transformed_status_denominator: cases.len() * samples_per_case,
        transformed_status_counts,
        residual_pair_count: canonicalized_distances.len(),
        ok_ok_pair_count: ok_ok_canonicalized_distances.len(),
        residual_failure_threshold: RESIDUAL_FAILURE_THRESHOLD,
        total_failures_above_threshold,
        largest_failures: failures,
    }
}

fn increment_status(counts: &mut std::collections::BTreeMap<String, usize>, status: &str) {
    *counts.entry(status.to_string()).or_insert(0) += 1;
}

pub fn score_candidate_metric(
    cases: &[Case],
    candidate: CandidateSpec,
    metric: MetricSpec,
    samples_per_case: usize,
    rng: &mut ChaCha8Rng,
) -> CandidateMetricSummary {
    let mut families = std::collections::BTreeMap::new();
    for family in transform_families() {
        families.insert(
            family.label().to_string(),
            score_family(cases, family, candidate, metric, samples_per_case, rng),
        );
    }
    CandidateMetricSummary {
        candidate: candidate.label.to_string(),
        metric: metric.label.to_string(),
        families,
    }
}

pub fn transform_families() -> [TransformFamily; 7] {
    [
        TransformFamily::Scale,
        TransformFamily::Translation,
        TransformFamily::FacetPermutation,
        TransformFamily::ScaleTranslationPermutation,
        TransformFamily::SymplecticBlock,
        TransformFamily::SymplecticExp,
        TransformFamily::FullGroupSample,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeSet;

    fn metric_cases() -> Vec<Case> {
        accepted_random_cases(8, 2026062804)
    }

    fn accepted_random_cases_with_facet_count(
        count: usize,
        facet_count: usize,
        seed: u64,
    ) -> Vec<Case> {
        let mut cases = Vec::with_capacity(count);
        let mut attempt = 0;
        while cases.len() < count {
            if let Ok(duals) =
                symplectic::random::generate_dual_vertices(facet_count, 0.55, 1.85, seed, attempt)
            {
                cases.push(Case {
                    case_id: format!("accepted-random-{attempt:06}-F{facet_count}"),
                    duals,
                });
            }
            attempt += 1;
            assert!(
                attempt < 200_000,
                "failed to generate {count} accepted random cases before attempt limit"
            );
        }
        cases
    }

    fn summaries() -> Vec<(TransformFamily, FamilySummary)> {
        let cases = accepted_random_cases(16, 2026062802);
        let mut rng = ChaCha8Rng::seed_from_u64(2026062803);
        let candidate = candidates::all()[0];
        let metric = metrics::all()[0];
        transform_families()
            .into_iter()
            .map(|family| {
                (
                    family,
                    score_family(&cases, family, candidate, metric, 3, &mut rng),
                )
            })
            .collect()
    }

    #[test]
    fn candidate_and_metric_registries_have_unique_nonempty_labels() {
        let mut candidate_labels = BTreeSet::new();
        for candidate in candidates::all() {
            assert!(!candidate.label.is_empty());
            assert!(
                candidate_labels.insert(candidate.label),
                "duplicate candidate label {}",
                candidate.label
            );
        }

        let mut metric_labels = BTreeSet::new();
        for metric in metrics::all() {
            assert!(!metric.label.is_empty());
            assert!(
                metric_labels.insert(metric.label),
                "duplicate metric label {}",
                metric.label
            );
        }
    }

    #[test]
    fn registered_t_candidates_return_coordinate_row_lists() {
        let cases = accepted_random_cases(4, 2026062813);
        for candidate in candidates::all() {
            for case in &cases {
                let output = (candidate.canonicalize)(&case.duals);
                assert_eq!(
                    output.duals.len(),
                    case.duals.len(),
                    "{} changed row count on {}",
                    candidate.label,
                    case.case_id
                );
            }
        }
    }

    #[test]
    fn invariant_representatives_are_not_registered_as_t_candidates() {
        let t_labels = candidates::all()
            .into_iter()
            .map(|candidate| candidate.label)
            .collect::<BTreeSet<_>>();
        for invariant in candidates::invariant_representatives() {
            assert!(
                !t_labels.contains(invariant.label),
                "{} is an invariant representative, not a T candidate",
                invariant.label
            );
        }
    }

    #[test]
    fn diagnostic_distance_is_zero_on_identical_random_rows() {
        for case in metric_cases() {
            let distance = nearest_neighbor_rms(&case.duals, &case.duals);
            assert!(
                distance.abs() < 1e-14,
                "{} identity distance {}",
                case.case_id,
                distance
            );
        }
    }

    #[test]
    fn diagnostic_distance_is_symmetric_on_random_perturbations() {
        let cases = metric_cases();
        let mut rng = ChaCha8Rng::seed_from_u64(2026062805);
        for case in cases {
            let transform = sample_transform(TransformFamily::FullGroupSample, &mut rng);
            let transformed = transform_duals(&case.duals, &transform, &mut rng);
            let forward = nearest_neighbor_rms(&case.duals, &transformed);
            let backward = nearest_neighbor_rms(&transformed, &case.duals);
            assert!(
                (forward - backward).abs() < 1e-14,
                "{} forward {} backward {}",
                case.case_id,
                forward,
                backward
            );
        }
    }

    #[test]
    fn diagnostic_distance_is_zero_after_facet_permutation() {
        let cases = metric_cases();
        let mut rng = ChaCha8Rng::seed_from_u64(2026062806);
        for case in cases {
            let transform = Transform {
                permute: true,
                ..Transform::default()
            };
            let permuted = transform_duals(&case.duals, &transform, &mut rng);
            let distance = nearest_neighbor_rms(&case.duals, &permuted);
            assert!(
                distance.abs() < 1e-14,
                "{} permutation distance {}",
                case.case_id,
                distance
            );
        }
    }

    #[test]
    fn diagnostic_distance_is_finite_nonnegative_and_detects_small_perturbation() {
        for case in metric_cases() {
            let mut perturbed = case.duals.clone();
            perturbed[0] += Vector4::new(1e-3, -2e-3, 1.5e-3, -5e-4);
            let distance = nearest_neighbor_rms(&case.duals, &perturbed);
            assert!(
                distance.is_finite() && distance >= 0.0,
                "{} bad distance {}",
                case.case_id,
                distance
            );
            assert!(
                distance > 1e-6,
                "{} failed to detect perturbation: {}",
                case.case_id,
                distance
            );
        }
    }

    #[test]
    fn diagnostic_distance_reports_incompatible_facet_counts_as_infinite() {
        let cases = metric_cases();
        let mut shortened = cases[0].duals.clone();
        shortened.pop();
        assert!(nearest_neighbor_rms(&cases[0].duals, &shortened).is_infinite());
    }

    #[test]
    fn stochastic_subgroups_score_small_and_full_group_fails() {
        let results = summaries();
        for (family, summary) in results {
            let stats = &summary.candidate_canonicalized;
            match family {
                TransformFamily::Scale | TransformFamily::FacetPermutation => {
                    assert!(stats.max < 1e-10, "{} max {}", family.label(), stats.max);
                }
                TransformFamily::Translation | TransformFamily::ScaleTranslationPermutation => {
                    assert!(
                        stats.p90 < 2e-6 && stats.max < 2e-5,
                        "{} p90 {} max {}",
                        family.label(),
                        stats.p90,
                        stats.max
                    );
                }
                TransformFamily::SymplecticBlock
                | TransformFamily::SymplecticExp
                | TransformFamily::FullGroupSample => {
                    assert!(
                        stats.median > 0.05,
                        "{} median {}",
                        family.label(),
                        stats.median
                    );
                }
            }
        }
    }

    #[test]
    fn sampled_sp4_lie_algebra_elements_satisfy_defining_equation() {
        let mut rng = ChaCha8Rng::seed_from_u64(2026062807);
        for _ in 0..32 {
            let matrix = random_sp4_lie_algebra_element(&mut rng);
            let defect = symplectic_lie_algebra_defect(&matrix);
            assert!(defect < 1e-12, "Lie algebra defect {defect}");
        }
    }

    #[test]
    fn sampled_sp4_exp_matrices_are_symplectic() {
        let mut rng = ChaCha8Rng::seed_from_u64(2026062807);
        for _ in 0..32 {
            let matrix = random_symplectic_exp(&mut rng);
            let defect = symplectic_defect(&matrix);
            assert!(defect < 1e-12, "symplectic defect {defect}");
        }
    }

    #[test]
    fn sampled_sp4_exp_matrices_are_not_restricted_to_block_subgroup() {
        let mut rng = ChaCha8Rng::seed_from_u64(2026062808);
        let mut max_cross_block_norm = 0.0_f64;
        for _ in 0..32 {
            let matrix = random_symplectic_exp(&mut rng);
            let cross_block_norm =
                matrix.fixed_view::<2, 2>(0, 2).norm() + matrix.fixed_view::<2, 2>(2, 0).norm();
            max_cross_block_norm = max_cross_block_norm.max(cross_block_norm);
        }
        assert!(
            max_cross_block_norm > 1e-3,
            "full sp4 sampler stayed block diagonal"
        );
    }

    #[test]
    fn omega_signature_candidate_scores_small_on_sampled_full_group() {
        let cases = accepted_random_cases(8, 2026062809);
        let mut rng = ChaCha8Rng::seed_from_u64(2026062810);
        let summary = score_family(
            &cases,
            TransformFamily::FullGroupSample,
            candidates::omega_signature_matrix::SPEC,
            metrics::ordered_rms::SPEC,
            2,
            &mut rng,
        );
        assert!(
            summary.candidate_canonicalized.max < 1e-5,
            "omega_signature_matrix full-group max {}",
            summary.candidate_canonicalized.max
        );
    }

    #[test]
    fn volume_one_omega_signature_candidate_scores_small_on_sampled_full_group() {
        let cases = accepted_random_cases(6, 2026062811);
        let mut rng = ChaCha8Rng::seed_from_u64(2026062812);
        let summary = score_family(
            &cases,
            TransformFamily::FullGroupSample,
            candidates::volume_one_omega_signature_matrix::SPEC,
            metrics::ordered_rms::SPEC,
            2,
            &mut rng,
        );
        assert!(
            summary.candidate_canonicalized.max < 1e-5,
            "volume_one_omega_signature_matrix full-group max {}",
            summary.candidate_canonicalized.max
        );
    }

    #[test]
    fn volume_one_omega_labeled_symplectic_frame_scores_small_on_sampled_full_group() {
        let cases = accepted_random_cases(4, 2026062814);
        let mut rng = ChaCha8Rng::seed_from_u64(2026062815);
        let summary = score_family(
            &cases,
            TransformFamily::FullGroupSample,
            candidates::volume_one_omega_labeled_symplectic_frame::SPEC,
            metrics::nearest_neighbor_rms::SPEC,
            2,
            &mut rng,
        );
        assert!(
            summary.candidate_canonicalized.max < 1e-5,
            "volume_one_omega_labeled_symplectic_frame full-group max {}",
            summary.candidate_canonicalized.max
        );
    }

    #[test]
    fn volume_one_omega_labeled_symplectic_frame_preserves_order_equivariance() {
        let cases = accepted_random_cases(4, 2026062831);
        let mut rng = ChaCha8Rng::seed_from_u64(2026062832);
        let summary = score_family(
            &cases,
            TransformFamily::FullGroupSample,
            candidates::volume_one_omega_labeled_symplectic_frame::SPEC,
            metrics::ordered_rms::SPEC,
            2,
            &mut rng,
        );
        assert_eq!(
            summary.ok_ok_pair_count, summary.residual_pair_count,
            "expected all sampled pairs to have ok status: {:?} {:?}",
            summary.base_status_counts, summary.transformed_status_counts
        );
        assert!(
            summary.candidate_canonicalized.max < 1e-5,
            "ordered full-group max {}",
            summary.candidate_canonicalized.max
        );
    }

    #[test]
    fn omega_labeled_symplectic_frame_scores_small_on_relevant_facet_counts() {
        for facet_count in [6, 10, 12] {
            let seed_offset = facet_count as u64;
            let cases =
                accepted_random_cases_with_facet_count(3, facet_count, 2026062818 + seed_offset);
            let mut rng = ChaCha8Rng::seed_from_u64(2026062828 + seed_offset);
            let summary = score_family(
                &cases,
                TransformFamily::FullGroupSample,
                candidates::volume_one_omega_labeled_symplectic_frame::SPEC,
                metrics::nearest_neighbor_rms::SPEC,
                2,
                &mut rng,
            );
            assert!(
                summary
                    .base_status_counts
                    .keys()
                    .all(|status| status == "ok"),
                "F={facet_count} base statuses {:?}",
                summary.base_status_counts
            );
            assert!(
                summary
                    .transformed_status_counts
                    .keys()
                    .all(|status| status == "ok"),
                "F={facet_count} transformed statuses {:?}",
                summary.transformed_status_counts
            );
            assert!(
                summary.candidate_canonicalized.max < 1e-5,
                "F={} full-group max {}",
                facet_count,
                summary.candidate_canonicalized.max
            );
        }
    }
}
