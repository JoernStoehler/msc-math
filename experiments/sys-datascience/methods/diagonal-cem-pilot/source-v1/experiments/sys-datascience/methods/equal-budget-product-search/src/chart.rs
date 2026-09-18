//! The fixed `5 x 5` Lagrangian-product chart used by the S0 CEM arm.
//!
//! The chart represents the ten *dual* vertices, rather than a particular
//! half-space presentation.  It removes the simultaneous rotation, overall
//! scaling, and reciprocal q/p scaling gauges.  It deliberately does not
//! quotient factor exchange.

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};

use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::Vector2;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use symplectic::geom::polygon::random_polygon_2d;

/// The number of facets in each factor in the frozen S0 bucket.
pub const FACTOR_FACETS: usize = 5;
pub const IID_HEIGHT_MIN: f64 = 0.8;
pub const IID_HEIGHT_MAX: f64 = 1.2;

/// A diagnostic tolerance only: exact `f64` lexicographic order chooses the
/// origin, while this marks nearby competing cyclic origins in artifacts.
pub const CYCLIC_NEAR_TIE_TOLERANCE: f64 = 1.0e-10;

/// A canonical 17-dimensional chart, with redundant fifth centered log radius
/// retained in the serialized form required by the packet schema.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductChart {
    pub q_gap_logits: [f64; 4],
    pub q_centered_log_radii: [f64; 5],
    pub p_gap_logits: [f64; 4],
    pub p_centered_log_radii: [f64; 5],
    pub relative_phase: f64,
    pub near_tie: bool,
}

impl ProductChart {
    /// Recover the chart directly from an already validated product polytope.
    /// The dual radius determines the equivalent unit-normal H-presentation
    /// with support height `1 / radius`.
    pub fn from_polytope(polytope: &SysLandscapePolytopeCache) -> Result<Self, ChartCodecError> {
        let mut q_normals = Vec::new();
        let mut q_heights = Vec::new();
        let mut p_normals = Vec::new();
        let mut p_heights = Vec::new();
        for (index, dual) in polytope.dual_vertices_f64.iter().enumerate() {
            let q_norm = dual.fixed_rows::<2>(0).norm();
            let p_norm = dual.fixed_rows::<2>(2).norm();
            if q_norm > 0.0 && p_norm == 0.0 {
                q_normals.push(Vector2::new(dual[0] / q_norm, dual[1] / q_norm));
                q_heights.push(1.0 / q_norm);
            } else if p_norm > 0.0 && q_norm == 0.0 {
                p_normals.push(Vector2::new(dual[2] / p_norm, dual[3] / p_norm));
                p_heights.push(1.0 / p_norm);
            } else {
                return Err(ChartCodecError::NonFiniteInput {
                    factor: "mixed",
                    index,
                });
            }
        }
        Self::from_factors(&q_normals, &q_heights, &p_normals, &p_heights)
    }

    /// The independent coordinates used by the diagonal CEM distribution.
    /// The last log radius in each factor is recovered from the zero-sum
    /// condition when decoding.
    pub fn continuous_coordinates(&self) -> [f64; 17] {
        let mut coordinates = [0.0; 17];
        coordinates[..4].copy_from_slice(&self.q_gap_logits);
        coordinates[4..8].copy_from_slice(&self.q_centered_log_radii[..4]);
        coordinates[8..12].copy_from_slice(&self.p_gap_logits);
        coordinates[12..16].copy_from_slice(&self.p_centered_log_radii[..4]);
        coordinates[16] = self.relative_phase;
        coordinates
    }

    /// Decode independent CEM coordinates.  The supplied phase is wrapped to
    /// `[0, 2 pi)` and the fifth radius coordinate enforces zero mean exactly.
    pub fn from_continuous_coordinates(coordinates: [f64; 17], near_tie: bool) -> Self {
        let q_last = -coordinates[4..8].iter().sum::<f64>();
        let p_last = -coordinates[12..16].iter().sum::<f64>();
        Self {
            q_gap_logits: coordinates[..4].try_into().expect("fixed q logits"),
            q_centered_log_radii: [
                coordinates[4],
                coordinates[5],
                coordinates[6],
                coordinates[7],
                q_last,
            ],
            p_gap_logits: coordinates[8..12].try_into().expect("fixed p logits"),
            p_centered_log_radii: [
                coordinates[12],
                coordinates[13],
                coordinates[14],
                coordinates[15],
                p_last,
            ],
            relative_phase: wrap_phase(coordinates[16]),
            near_tie,
        }
    }

    /// Encode a valid two-factor H-presentation after canonical cyclic
    /// relabeling.  Normals need not be unit length: the represented dual
    /// radius is `||normal|| / height`.
    pub fn from_factors(
        q_normals: &[Vector2<f64>],
        q_heights: &[f64],
        p_normals: &[Vector2<f64>],
        p_heights: &[f64],
    ) -> Result<Self, ChartCodecError> {
        let q = FactorData::from_h_rep("q", q_normals, q_heights)?;
        let p = FactorData::from_h_rep("p", p_normals, p_heights)?;
        let q = q.canonicalize();
        let p = p.canonicalize();
        Ok(Self {
            q_gap_logits: gap_logits(&q.gaps),
            q_centered_log_radii: q.centered_log_radii,
            p_gap_logits: gap_logits(&p.gaps),
            p_centered_log_radii: p.centered_log_radii,
            relative_phase: wrap_phase(p.origin_angle - q.origin_angle),
            near_tie: q.near_tie || p.near_tie,
        })
    }

    pub fn reconstruct_factors(&self) -> Result<ProductFactors, ConstructionRejection> {
        let q = reconstruct_factor("q", &self.q_gap_logits, &self.q_centered_log_radii, 0.0)?;
        let p = reconstruct_factor(
            "p",
            &self.p_gap_logits,
            &self.p_centered_log_radii,
            self.relative_phase,
        )?;
        Ok(ProductFactors {
            q_normals: q.0,
            q_heights: q.1,
            p_normals: p.0,
            p_heights: p.1,
        })
    }

    /// Reconstruct and run the ordinary product constructor.  It never repairs
    /// an invalid CEM proposal, so rejection remains observable to the caller.
    pub fn reconstruct_candidate(&self) -> Result<ProductCandidate, ConstructionRejection> {
        let factors = self.reconstruct_factors()?;
        let polytope = SysLandscapePolytopeCache::from_lagrangian_product(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .ok_or(ConstructionRejection::PolytopeConstructorRejected)?;
        Ok(ProductCandidate { factors, polytope })
    }
}

/// A product H-presentation.  Its normal order is the chart's canonical
/// cyclic order, beginning at q angle zero and p relative phase.
#[derive(Clone, Debug)]
pub struct ProductFactors {
    pub q_normals: Vec<Vector2<f64>>,
    pub q_heights: Vec<f64>,
    pub p_normals: Vec<Vector2<f64>>,
    pub p_heights: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct ProductCandidate {
    pub factors: ProductFactors,
    pub polytope: SysLandscapePolytopeCache,
}

/// Why an uncharged proposal could not be constructed.  CEM must retain this
/// reason/count instead of projecting the proposal into the valid set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstructionRejection {
    NonFiniteCoordinate,
    NonFiniteRadius,
    NonPositiveRadius,
    GapAtLeastPi { factor: &'static str, index: usize },
    PolytopeConstructorRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChartCodecError {
    WrongFacetCount {
        factor: &'static str,
        normals: usize,
        heights: usize,
    },
    NonFiniteInput {
        factor: &'static str,
        index: usize,
    },
    NonPositiveHeight {
        factor: &'static str,
        index: usize,
    },
    ZeroNormal {
        factor: &'static str,
        index: usize,
    },
}

/// Wrap an angle to the canonical half-open phase interval.
pub fn wrap_phase(phase: f64) -> f64 {
    if !phase.is_finite() {
        return phase;
    }
    phase.rem_euclid(TAU)
}

/// The common pre-target IID stream for all arms in one replicate.  `arm` is
/// intentionally absent from the seed material so `0..63` agrees with CEM
/// generation zero and successive local starts begin at index zero.
pub fn iid_base_candidate(
    master_seed: u64,
    replicate: usize,
    base_index: usize,
) -> Result<ProductCandidate, ConstructionRejection> {
    iid_base_candidate_attempt(master_seed, replicate, base_index, 0)
}

/// One deterministic construction attempt of a common IID base candidate.
/// Callers that need to retry an uncharged construction rejection must increase
/// `construction_attempt` and preserve that number in the candidate identity.
pub fn iid_base_candidate_attempt(
    master_seed: u64,
    replicate: usize,
    base_index: usize,
    construction_attempt: usize,
) -> Result<ProductCandidate, ConstructionRejection> {
    let mut material = [0u8; 32];
    material[..8].copy_from_slice(&master_seed.to_le_bytes());
    material[8..16].copy_from_slice(&(replicate as u64).to_le_bytes());
    material[16..24].copy_from_slice(&(base_index as u64).to_le_bytes());
    material[24..].copy_from_slice(&(construction_attempt as u64).to_le_bytes());
    let seed = blake3::derive_key("s0-iid-product-base-stream-v1", &material);
    let mut rng = ChaCha8Rng::from_seed(seed);
    let (q_normals, q_heights) =
        random_polygon_2d(FACTOR_FACETS, IID_HEIGHT_MIN, IID_HEIGHT_MAX, &mut rng);
    let (p_normals, p_heights) =
        random_polygon_2d(FACTOR_FACETS, IID_HEIGHT_MIN, IID_HEIGHT_MAX, &mut rng);
    let factors = ProductFactors {
        q_normals,
        q_heights,
        p_normals,
        p_heights,
    };
    let polytope = SysLandscapePolytopeCache::from_lagrangian_product(
        &factors.q_normals,
        &factors.q_heights,
        &factors.p_normals,
        &factors.p_heights,
    )
    .ok_or(ConstructionRejection::PolytopeConstructorRejected)?;
    Ok(ProductCandidate { factors, polytope })
}

#[derive(Clone, Debug)]
struct FactorData {
    angles: [f64; FACTOR_FACETS],
    centered_log_radii: [f64; FACTOR_FACETS],
}

#[derive(Clone, Debug)]
struct CanonicalFactor {
    gaps: [f64; FACTOR_FACETS],
    centered_log_radii: [f64; FACTOR_FACETS],
    origin_angle: f64,
    near_tie: bool,
}

impl FactorData {
    fn from_h_rep(
        factor: &'static str,
        normals: &[Vector2<f64>],
        heights: &[f64],
    ) -> Result<Self, ChartCodecError> {
        if normals.len() != FACTOR_FACETS || heights.len() != FACTOR_FACETS {
            return Err(ChartCodecError::WrongFacetCount {
                factor,
                normals: normals.len(),
                heights: heights.len(),
            });
        }
        let mut entries = Vec::with_capacity(FACTOR_FACETS);
        for (index, (normal, height)) in normals.iter().zip(heights).enumerate() {
            if !normal[0].is_finite() || !normal[1].is_finite() || !height.is_finite() {
                return Err(ChartCodecError::NonFiniteInput { factor, index });
            }
            if *height <= 0.0 {
                return Err(ChartCodecError::NonPositiveHeight { factor, index });
            }
            let norm = normal.norm();
            if norm == 0.0 {
                return Err(ChartCodecError::ZeroNormal { factor, index });
            }
            entries.push((wrap_phase(normal[1].atan2(normal[0])), (norm / height).ln()));
        }
        entries.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mean = entries
            .iter()
            .map(|(_, log_radius)| log_radius)
            .sum::<f64>()
            / FACTOR_FACETS as f64;
        Ok(Self {
            angles: std::array::from_fn(|i| entries[i].0),
            centered_log_radii: std::array::from_fn(|i| entries[i].1 - mean),
        })
    }

    fn canonicalize(&self) -> CanonicalFactor {
        let gaps = outgoing_gaps(&self.angles);
        let best = (0..FACTOR_FACETS)
            .min_by(|&left, &right| compare_rotations(&gaps, &self.centered_log_radii, left, right))
            .expect("five cyclic rotations");
        let near_tie = (0..FACTOR_FACETS)
            .filter(|&other| other != best)
            .any(|other| rotations_near(&gaps, &self.centered_log_radii, best, other));
        CanonicalFactor {
            gaps: std::array::from_fn(|i| gaps[(best + i) % FACTOR_FACETS]),
            centered_log_radii: std::array::from_fn(|i| {
                self.centered_log_radii[(best + i) % FACTOR_FACETS]
            }),
            origin_angle: self.angles[best],
            near_tie,
        }
    }
}

fn outgoing_gaps(angles: &[f64; FACTOR_FACETS]) -> [f64; FACTOR_FACETS] {
    std::array::from_fn(|i| wrap_phase(angles[(i + 1) % FACTOR_FACETS] - angles[i]))
}

/// Lexicographic order of `(outgoing gap, centered log radius)` pairs.
fn compare_rotations(
    gaps: &[f64; FACTOR_FACETS],
    log_radii: &[f64; FACTOR_FACETS],
    left: usize,
    right: usize,
) -> Ordering {
    for offset in 0..FACTOR_FACETS {
        let left_index = (left + offset) % FACTOR_FACETS;
        let right_index = (right + offset) % FACTOR_FACETS;
        let gap_order = gaps[left_index].total_cmp(&gaps[right_index]);
        if gap_order != Ordering::Equal {
            return gap_order;
        }
        let radius_order = log_radii[left_index].total_cmp(&log_radii[right_index]);
        if radius_order != Ordering::Equal {
            return radius_order;
        }
    }
    Ordering::Equal
}

fn rotations_near(
    gaps: &[f64; FACTOR_FACETS],
    log_radii: &[f64; FACTOR_FACETS],
    left: usize,
    right: usize,
) -> bool {
    for offset in 0..FACTOR_FACETS {
        let left_index = (left + offset) % FACTOR_FACETS;
        let right_index = (right + offset) % FACTOR_FACETS;
        if gaps[left_index] != gaps[right_index] {
            return (gaps[left_index] - gaps[right_index]).abs() <= CYCLIC_NEAR_TIE_TOLERANCE;
        }
        if log_radii[left_index] != log_radii[right_index] {
            return (log_radii[left_index] - log_radii[right_index]).abs()
                <= CYCLIC_NEAR_TIE_TOLERANCE;
        }
        // An exactly tied leading pair is a discontinuity stratum even if a
        // later pair currently resolves the exact lexicographic order: an
        // arbitrarily small perturbation of this pair can reverse the origin.
        if offset == 0 {
            return true;
        }
    }
    true
}

fn gap_logits(gaps: &[f64; FACTOR_FACETS]) -> [f64; 4] {
    std::array::from_fn(|i| (gaps[i] / gaps[4]).ln())
}

fn reconstruct_factor(
    factor: &'static str,
    logits: &[f64; 4],
    centered_log_radii: &[f64; 5],
    origin_angle: f64,
) -> Result<(Vec<Vector2<f64>>, Vec<f64>), ConstructionRejection> {
    if !origin_angle.is_finite() || logits.iter().any(|x| !x.is_finite()) {
        return Err(ConstructionRejection::NonFiniteCoordinate);
    }
    if centered_log_radii.iter().any(|x| !x.is_finite()) {
        return Err(ConstructionRejection::NonFiniteRadius);
    }
    let gaps = softmax_gaps(logits);
    for (index, gap) in gaps.iter().enumerate() {
        if *gap >= PI {
            return Err(ConstructionRejection::GapAtLeastPi { factor, index });
        }
    }
    let mut angle = wrap_phase(origin_angle);
    let mut normals = Vec::with_capacity(FACTOR_FACETS);
    let mut heights = Vec::with_capacity(FACTOR_FACETS);
    for (index, log_radius) in centered_log_radii.iter().enumerate() {
        let radius = log_radius.exp();
        if !radius.is_finite() {
            return Err(ConstructionRejection::NonFiniteRadius);
        }
        if radius <= 0.0 {
            return Err(ConstructionRejection::NonPositiveRadius);
        }
        normals.push(Vector2::new(angle.cos(), angle.sin()));
        heights.push(radius.recip());
        angle = wrap_phase(angle + gaps[index]);
    }
    Ok((normals, heights))
}

fn softmax_gaps(logits: &[f64; 4]) -> [f64; FACTOR_FACETS] {
    let max = logits.iter().copied().fold(0.0_f64, f64::max);
    let weights = std::array::from_fn::<_, FACTOR_FACETS, _>(|i| {
        if i == 4 {
            (-max).exp()
        } else {
            (logits[i] - max).exp()
        }
    });
    let sum = weights.iter().sum::<f64>();
    std::array::from_fn(|i| TAU * weights[i] / sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 2.0e-11;

    fn assert_close(left: f64, right: f64) {
        assert!(
            (left - right).abs() <= EPS,
            "left={left:.17e}, right={right:.17e}, delta={:.17e}",
            (left - right).abs()
        );
    }

    fn assert_chart_close(left: &ProductChart, right: &ProductChart) {
        for (left, right) in left
            .continuous_coordinates()
            .iter()
            .zip(right.continuous_coordinates())
        {
            assert_close(*left, right);
        }
        assert_eq!(left.near_tie, right.near_tie);
    }

    fn base_factors() -> ProductFactors {
        (0..128)
            .find_map(|attempt| iid_base_candidate_attempt(202607110001, 1, 7, attempt).ok())
            .expect("one deterministic IID retry constructs")
            .factors
    }

    fn rotate(normals: &[Vector2<f64>], phase: f64) -> Vec<Vector2<f64>> {
        normals
            .iter()
            .map(|normal| {
                Vector2::new(
                    phase.cos() * normal[0] - phase.sin() * normal[1],
                    phase.sin() * normal[0] + phase.cos() * normal[1],
                )
            })
            .collect()
    }

    fn cycle<T: Clone>(values: &[T], shift: usize) -> Vec<T> {
        (0..values.len())
            .map(|i| values[(i + shift) % values.len()].clone())
            .collect()
    }

    #[test]
    fn chart_round_trip_reconstructs_canonical_dual_geometry() {
        let factors = base_factors();
        let chart = ProductChart::from_factors(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .expect("chart encodes generated product");
        let rebuilt = chart.reconstruct_candidate().expect("chart reconstructs");
        let rebuilt_chart = ProductChart::from_factors(
            &rebuilt.factors.q_normals,
            &rebuilt.factors.q_heights,
            &rebuilt.factors.p_normals,
            &rebuilt.factors.p_heights,
        )
        .expect("reconstructed factors encode");
        assert_chart_close(&chart, &rebuilt_chart);
        assert_eq!(rebuilt.polytope.dual_vertices.len(), 10);
    }

    #[test]
    fn chart_removes_simultaneous_rotation_gauge() {
        let factors = base_factors();
        let original = ProductChart::from_factors(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .unwrap();
        let rotated = ProductChart::from_factors(
            &rotate(&factors.q_normals, 0.731),
            &factors.q_heights,
            &rotate(&factors.p_normals, 0.731),
            &factors.p_heights,
        )
        .unwrap();
        assert_chart_close(&original, &rotated);
    }

    #[test]
    fn chart_removes_overall_and_reciprocal_q_p_scaling_gauges() {
        let factors = base_factors();
        let original = ProductChart::from_factors(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .unwrap();
        let overall = 1.73;
        let reciprocal = 0.61;
        let q_heights: Vec<f64> = factors
            .q_heights
            .iter()
            .map(|height| height * overall * reciprocal)
            .collect();
        let p_heights: Vec<f64> = factors
            .p_heights
            .iter()
            .map(|height| height * overall / reciprocal)
            .collect();
        let scaled = ProductChart::from_factors(
            &factors.q_normals,
            &q_heights,
            &factors.p_normals,
            &p_heights,
        )
        .unwrap();
        assert_chart_close(&original, &scaled);
    }

    #[test]
    fn chart_removes_within_factor_cyclic_relabeling() {
        let factors = base_factors();
        let original = ProductChart::from_factors(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .unwrap();
        let relabeled = ProductChart::from_factors(
            &cycle(&factors.q_normals, 2),
            &cycle(&factors.q_heights, 2),
            &cycle(&factors.p_normals, 4),
            &cycle(&factors.p_heights, 4),
        )
        .unwrap();
        assert_chart_close(&original, &relabeled);
    }

    #[test]
    fn tie_stratum_is_recorded_and_has_a_cyclic_chart_discontinuity() {
        let make_factor = |epsilon: f64| {
            // The first two facets have exactly equal outgoing gaps.  They
            // become the two competing lexicographic origins; all other
            // equal-gap origins have a larger radius coordinate.
            let angles: Vec<f64> = vec![0.0, 1.0, 2.0, 4.0, 5.0];
            let radii = [epsilon, -epsilon, 0.2, 0.4, 0.6];
            let normals = angles
                .iter()
                .map(|angle| Vector2::new(angle.cos(), angle.sin()))
                .collect::<Vec<_>>();
            let heights: Vec<f64> = radii
                .iter()
                .map(|radius: &f64| radius.exp().recip())
                .collect();
            (normals, heights)
        };
        let (q_tie, qh_tie) = make_factor(0.0);
        let (p, ph) = make_factor(0.17);
        let tie = ProductChart::from_factors(&q_tie, &qh_tie, &p, &ph).unwrap();
        let (q_left, qh_left) = make_factor(-1.0e-12);
        let (q_right, qh_right) = make_factor(1.0e-12);
        let left = ProductChart::from_factors(&q_left, &qh_left, &p, &ph).unwrap();
        let right = ProductChart::from_factors(&q_right, &qh_right, &p, &ph).unwrap();
        assert!(tie.near_tie, "the exact tie is an artifact-visible stratum");
        assert!(left.near_tie && right.near_tie);
        assert!(
            (left.q_centered_log_radii[2] - right.q_centered_log_radii[2]).abs() > 0.1,
            "crossing the tie selects a different cyclic origin"
        );
    }

    #[test]
    fn invalid_chart_has_deterministic_uncharged_rejection_reason() {
        let chart = ProductChart {
            q_gap_logits: [1000.0, -1000.0, -1000.0, -1000.0],
            q_centered_log_radii: [0.0; 5],
            p_gap_logits: [0.0; 4],
            p_centered_log_radii: [0.0; 5],
            relative_phase: 0.0,
            near_tie: false,
        };
        assert!(matches!(
            chart.reconstruct_candidate(),
            Err(ConstructionRejection::GapAtLeastPi {
                factor: "q",
                index: 0,
            })
        ));
    }

    #[test]
    fn iid_base_stream_is_deterministic_and_common_by_index() {
        let attempt = (0..128)
            .find(|attempt| iid_base_candidate_attempt(202607110002, 2, 63, *attempt).is_ok())
            .expect("one deterministic IID retry constructs");
        let first = iid_base_candidate_attempt(202607110002, 2, 63, attempt).unwrap();
        let second = iid_base_candidate_attempt(202607110002, 2, 63, attempt).unwrap();
        let first_chart = ProductChart::from_factors(
            &first.factors.q_normals,
            &first.factors.q_heights,
            &first.factors.p_normals,
            &first.factors.p_heights,
        )
        .unwrap();
        let second_chart = ProductChart::from_factors(
            &second.factors.q_normals,
            &second.factors.q_heights,
            &second.factors.p_normals,
            &second.factors.p_heights,
        )
        .unwrap();
        assert_chart_close(&first_chart, &second_chart);
    }

    #[test]
    fn iid_retry_attempt_is_deterministic_and_changes_the_stream() {
        let mut valid_attempts = (0..128)
            .filter(|attempt| iid_base_candidate_attempt(202607110003, 0, 11, *attempt).is_ok());
        let first_attempt = valid_attempts.next().expect("first valid retry");
        let retry_attempt = valid_attempts.next().expect("second valid retry");
        let first = iid_base_candidate_attempt(202607110003, 0, 11, first_attempt).unwrap();
        let repeat = iid_base_candidate_attempt(202607110003, 0, 11, first_attempt).unwrap();
        let retry = iid_base_candidate_attempt(202607110003, 0, 11, retry_attempt).unwrap();
        let chart = |candidate: &ProductCandidate| {
            ProductChart::from_factors(
                &candidate.factors.q_normals,
                &candidate.factors.q_heights,
                &candidate.factors.p_normals,
                &candidate.factors.p_heights,
            )
            .unwrap()
        };
        let first_chart = chart(&first);
        assert_chart_close(&first_chart, &chart(&repeat));
        assert_ne!(
            first_chart.continuous_coordinates(),
            chart(&retry).continuous_coordinates(),
            "retry attempt is an independent deterministic draw"
        );
    }
}
