use crate::geometry::f64_vertex_scan_report;
use crate::{ProductBlock, ProductSimplificationReport, ProductSimplificationStatus};
use nalgebra::Vector4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacetSimplificationPolicy {
    None,
    ProductNearRedundant,
    GenericSingleBand,
}

impl FacetSimplificationPolicy {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ProductNearRedundant => "product_near_redundant",
            Self::GenericSingleBand => "generic_single_band",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacetSimplificationStatus {
    NotAttempted,
    Simplified,
    NoNearRedundantFacets,
    NotBlockProduct,
    IndeterminateGeometry,
}

impl FacetSimplificationStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Simplified => "simplified",
            Self::NoNearRedundantFacets => "no_near_redundant_facets",
            Self::NotBlockProduct => "not_block_product",
            Self::IndeterminateGeometry => "indeterminate_geometry",
        }
    }
}

#[derive(Clone, Debug)]
pub enum FacetSimplificationRemoval {
    Product {
        original_index: usize,
        block: ProductBlock,
        factor_index: usize,
        delta: f64,
    },
    GenericSingleBand {
        original_index: usize,
        guard_original_index: usize,
        guard_floor: f64,
        delta: f64,
    },
}

impl FacetSimplificationRemoval {
    pub fn original_index(&self) -> usize {
        match self {
            Self::Product { original_index, .. } => *original_index,
            Self::GenericSingleBand { original_index, .. } => *original_index,
        }
    }

    pub fn delta(&self) -> f64 {
        match self {
            Self::Product { delta, .. } => *delta,
            Self::GenericSingleBand { delta, .. } => *delta,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FacetSimplificationReport {
    pub policy: FacetSimplificationPolicy,
    pub status: FacetSimplificationStatus,
    pub simplified_dual_vertices: Vec<Vector4<f64>>,
    pub kept_original_indices: Vec<usize>,
    pub removed_facets: Vec<FacetSimplificationRemoval>,
    /// Intended set-level bound:
    /// P_original <= P_simplified <= (1 + delta_bound) P_original.
    pub delta_bound: f64,
    pub capacity_ratio_upper: f64,
    pub volume_ratio_upper: f64,
    pub sys_ratio_lower: f64,
    pub sys_ratio_upper: f64,
}

impl FacetSimplificationReport {
    pub fn not_attempted(dual_vertices: &[Vector4<f64>]) -> Self {
        Self::unchanged(
            FacetSimplificationPolicy::None,
            FacetSimplificationStatus::NotAttempted,
            dual_vertices.to_vec(),
        )
    }

    pub fn from_product(report: ProductSimplificationReport) -> Self {
        let status = match report.status {
            ProductSimplificationStatus::NotAttempted => FacetSimplificationStatus::NotAttempted,
            ProductSimplificationStatus::Simplified => FacetSimplificationStatus::Simplified,
            ProductSimplificationStatus::NoNearRedundantFacets => {
                FacetSimplificationStatus::NoNearRedundantFacets
            }
            ProductSimplificationStatus::NotBlockProduct => {
                FacetSimplificationStatus::NotBlockProduct
            }
        };
        let removed_facets = report
            .removed_facets
            .into_iter()
            .map(|facet| FacetSimplificationRemoval::Product {
                original_index: facet.original_index,
                block: facet.block,
                factor_index: facet.factor_index,
                delta: facet.delta,
            })
            .collect();
        Self {
            policy: FacetSimplificationPolicy::ProductNearRedundant,
            status,
            simplified_dual_vertices: report.simplified_dual_vertices,
            kept_original_indices: report.kept_original_indices,
            removed_facets,
            delta_bound: report.delta_bound,
            capacity_ratio_upper: report.capacity_ratio_upper,
            volume_ratio_upper: report.volume_ratio_upper,
            sys_ratio_lower: report.sys_ratio_lower,
            sys_ratio_upper: report.sys_ratio_upper,
        }
    }

    fn unchanged(
        policy: FacetSimplificationPolicy,
        status: FacetSimplificationStatus,
        dual_vertices: Vec<Vector4<f64>>,
    ) -> Self {
        let distortion = distortion_from_delta_bound(0.0);
        Self {
            policy,
            status,
            kept_original_indices: (0..dual_vertices.len()).collect(),
            simplified_dual_vertices: dual_vertices,
            removed_facets: Vec::new(),
            delta_bound: 0.0,
            capacity_ratio_upper: distortion.capacity_ratio_upper,
            volume_ratio_upper: distortion.volume_ratio_upper,
            sys_ratio_lower: distortion.sys_ratio_lower,
            sys_ratio_upper: distortion.sys_ratio_upper,
        }
    }
}

#[derive(Clone, Debug)]
struct CandidateRemoval {
    current_index: usize,
    guard_current_index: usize,
    guard_floor: f64,
    delta: f64,
}

pub fn remove_nearly_redundant_facets_single_band(
    dual_vertices: &[Vector4<f64>],
    max_delta: f64,
) -> FacetSimplificationReport {
    if !max_delta.is_finite() || max_delta < 0.0 {
        return FacetSimplificationReport::unchanged(
            FacetSimplificationPolicy::GenericSingleBand,
            FacetSimplificationStatus::IndeterminateGeometry,
            dual_vertices.to_vec(),
        );
    }

    let mut current_vertices = dual_vertices.to_vec();
    let mut current_to_original = (0..dual_vertices.len()).collect::<Vec<_>>();
    let mut removed_facets = Vec::new();
    let mut scale_bound = 1.0f64;

    loop {
        if current_vertices.len() <= 5 {
            break;
        }
        let Ok(vertex_scan) = f64_vertex_scan_report(&current_vertices) else {
            return finish_generic_report(
                current_vertices,
                current_to_original,
                removed_facets,
                scale_bound,
                FacetSimplificationStatus::IndeterminateGeometry,
            );
        };
        if vertex_scan.has_indeterminate_geometry() {
            let status = if removed_facets.is_empty() {
                FacetSimplificationStatus::IndeterminateGeometry
            } else {
                FacetSimplificationStatus::Simplified
            };
            return finish_generic_report(
                current_vertices,
                current_to_original,
                removed_facets,
                scale_bound,
                status,
            );
        }

        let Some(candidate) =
            best_single_band_candidate(&current_vertices, &vertex_scan, max_delta)
        else {
            break;
        };

        let original_index = current_to_original[candidate.current_index];
        let guard_original_index = current_to_original[candidate.guard_current_index];
        let delta = candidate.delta.max(0.0);
        scale_bound *= 1.0 + delta;
        removed_facets.push(FacetSimplificationRemoval::GenericSingleBand {
            original_index,
            guard_original_index,
            guard_floor: candidate.guard_floor,
            delta,
        });
        current_vertices.remove(candidate.current_index);
        current_to_original.remove(candidate.current_index);
    }

    let status = if removed_facets.is_empty() {
        FacetSimplificationStatus::NoNearRedundantFacets
    } else {
        FacetSimplificationStatus::Simplified
    };
    finish_generic_report(
        current_vertices,
        current_to_original,
        removed_facets,
        scale_bound,
        status,
    )
}

fn best_single_band_candidate(
    dual_vertices: &[Vector4<f64>],
    vertex_scan: &crate::geometry::F64VertexScanReport,
    max_delta: f64,
) -> Option<CandidateRemoval> {
    // Sufficient condition from formal/product-simplification-bounds.tex:
    // if one retained guard facet stays above r on every vertex of the removed
    // facet, then removing that facet expands the polytope by at most 1/r.
    let min_guard_floor = 1.0 / (1.0 + max_delta);
    let mut best: Option<CandidateRemoval> = None;
    for facet in 0..dual_vertices.len() {
        let facet_vertices = vertex_scan
            .vertices
            .iter()
            .filter(|vertex| vertex.definite_incident.contains(&facet))
            .collect::<Vec<_>>();
        if facet_vertices.is_empty() {
            continue;
        }
        let mut best_guard_floor = f64::NEG_INFINITY;
        let mut best_guard = None;
        for guard in 0..dual_vertices.len() {
            if guard == facet {
                continue;
            }
            let guard_floor = facet_vertices
                .iter()
                .map(|vertex| dual_vertices[guard].dot(&vertex.point))
                .fold(f64::INFINITY, f64::min);
            if guard_floor > best_guard_floor {
                best_guard_floor = guard_floor;
                best_guard = Some(guard);
            }
        }
        let guard_current_index = best_guard?;
        if !(best_guard_floor.is_finite() && best_guard_floor > 0.0) {
            continue;
        }
        if best_guard_floor < min_guard_floor {
            continue;
        }
        let delta = if best_guard_floor >= 1.0 {
            0.0
        } else {
            1.0 / best_guard_floor - 1.0
        };
        if delta > max_delta {
            continue;
        }
        let candidate = CandidateRemoval {
            current_index: facet,
            guard_current_index,
            guard_floor: best_guard_floor,
            delta,
        };
        let replace = best.as_ref().is_none_or(|current| {
            candidate.delta < current.delta
                || (candidate.delta == current.delta
                    && candidate.current_index < current.current_index)
        });
        if replace {
            best = Some(candidate);
        }
    }
    best
}

fn finish_generic_report(
    simplified_dual_vertices: Vec<Vector4<f64>>,
    kept_original_indices: Vec<usize>,
    removed_facets: Vec<FacetSimplificationRemoval>,
    scale_bound: f64,
    status: FacetSimplificationStatus,
) -> FacetSimplificationReport {
    let delta_bound = (scale_bound - 1.0).max(0.0);
    let distortion = distortion_from_delta_bound(delta_bound);
    FacetSimplificationReport {
        policy: FacetSimplificationPolicy::GenericSingleBand,
        status,
        simplified_dual_vertices,
        kept_original_indices,
        removed_facets,
        delta_bound,
        capacity_ratio_upper: distortion.capacity_ratio_upper,
        volume_ratio_upper: distortion.volume_ratio_upper,
        sys_ratio_lower: distortion.sys_ratio_lower,
        sys_ratio_upper: distortion.sys_ratio_upper,
    }
}

#[derive(Clone, Copy, Debug)]
struct SimplificationDistortion {
    capacity_ratio_upper: f64,
    volume_ratio_upper: f64,
    sys_ratio_lower: f64,
    sys_ratio_upper: f64,
}

fn distortion_from_delta_bound(delta_bound: f64) -> SimplificationDistortion {
    let scale = 1.0 + delta_bound;
    SimplificationDistortion {
        capacity_ratio_upper: scale.powi(2),
        volume_ratio_upper: scale.powi(4),
        sys_ratio_lower: scale.powi(-4),
        sys_ratio_upper: scale.powi(4),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_single_band_removes_near_redundant_cube_facet() {
        let eps = 1e-8;
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
            Vector4::new(1.0, eps, 0.0, 0.0),
        ];

        let report = remove_nearly_redundant_facets_single_band(&dual_vertices, 2e-8);

        assert_eq!(report.status, FacetSimplificationStatus::Simplified);
        assert_eq!(report.removed_facets.len(), 1);
        assert!([0, 8].contains(&report.removed_facets[0].original_index()));
        assert!(report.delta_bound <= 2e-8);
        assert_eq!(report.simplified_dual_vertices.len(), 8);
    }

    #[test]
    fn generic_single_band_leaves_generic_simplex_unchanged() {
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(-1.0, -1.0, -1.0, -1.0),
        ];

        let report = remove_nearly_redundant_facets_single_band(&dual_vertices, 1e-8);

        assert_eq!(
            report.status,
            FacetSimplificationStatus::NoNearRedundantFacets
        );
        assert!(report.removed_facets.is_empty());
        assert_eq!(report.simplified_dual_vertices, dual_vertices);
    }
}
