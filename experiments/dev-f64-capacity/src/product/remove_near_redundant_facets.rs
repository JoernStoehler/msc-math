use super::{round_blocks, ProductBlock, ProductRoundingStatus};
use nalgebra::{Vector2, Vector4};
use symplectic::classify_facets_from_dual_vertices;

const FACTOR_DET_TOLERANCE: f64 = 1e-12;
const FACTOR_FEASIBILITY_TOLERANCE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct ProductFacetRemoval {
    pub original_index: usize,
    pub block: ProductBlock,
    pub factor_index: usize,
    pub delta: f64,
}

#[derive(Clone, Debug)]
pub struct ProductFacetRemovalReport {
    pub status: ProductFacetRemovalStatus,
    pub vertices_after_removal: Vec<Vector4<f64>>,
    pub kept_original_indices: Vec<usize>,
    pub removed_facets: Vec<ProductFacetRemoval>,
    /// Intended set-level bound:
    /// P_original <= P_after <= (1 + delta_bound) P_original.
    /// See formal label `rem:near-redundant-facet-removal-experiment-contract`.
    pub delta_bound: f64,
    pub capacity_ratio_upper: f64,
    pub volume_ratio_upper: f64,
    pub sys_ratio_lower: f64,
    pub sys_ratio_upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductFacetRemovalStatus {
    NotAttempted,
    Removed,
    NoNearRedundantFacets,
    NotBlockProduct,
    InvalidDelta,
}

impl ProductFacetRemovalStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Removed => "removed",
            Self::NoNearRedundantFacets => "no_near_redundant_facets",
            Self::NotBlockProduct => "not_block_product",
            Self::InvalidDelta => "invalid_delta",
        }
    }
}

pub fn remove_near_redundant_facets(
    dual_vertices: &[Vector4<f64>],
    max_delta: f64,
) -> ProductFacetRemovalReport {
    if !max_delta.is_finite() || max_delta < 0.0 {
        return ProductFacetRemovalReport::unchanged(
            ProductFacetRemovalStatus::InvalidDelta,
            dual_vertices.to_vec(),
        );
    }

    let rounded = round_blocks(dual_vertices);
    if rounded.status != ProductRoundingStatus::Rounded {
        return ProductFacetRemovalReport::not_block_product(dual_vertices);
    }
    let dual_vertices = rounded.rounded_dual_vertices;
    let Ok(classification) = classify_facets_from_dual_vertices(&dual_vertices) else {
        return ProductFacetRemovalReport::not_block_product(&dual_vertices);
    };

    let q_facets = factor_facets(&dual_vertices, &classification.q_indices, ProductBlock::Q);
    let p_facets = factor_facets(&dual_vertices, &classification.p_indices, ProductBlock::P);
    let q_plan = near_redundant_factor_facets(&q_facets, max_delta);
    let p_plan = near_redundant_factor_facets(&p_facets, max_delta);
    let mut removed_facets = q_plan.removed_facets;
    removed_facets.extend(p_plan.removed_facets);
    removed_facets.sort_by_key(|facet| facet.original_index);

    if removed_facets.is_empty() {
        return ProductFacetRemovalReport::unchanged(
            ProductFacetRemovalStatus::NoNearRedundantFacets,
            dual_vertices,
        );
    }

    let mut remove = vec![false; dual_vertices.len()];
    for facet in &removed_facets {
        remove[facet.original_index] = true;
    }
    let mut vertices_after_removal = Vec::new();
    let mut kept_original_indices = Vec::new();
    for (idx, vertex) in dual_vertices.iter().enumerate() {
        if !remove[idx] {
            vertices_after_removal.push(*vertex);
            kept_original_indices.push(idx);
        }
    }
    let delta_bound = q_plan.delta_bound.max(p_plan.delta_bound);
    let distortion = distortion_from_delta_bound(delta_bound);

    ProductFacetRemovalReport {
        status: ProductFacetRemovalStatus::Removed,
        vertices_after_removal,
        kept_original_indices,
        removed_facets,
        delta_bound,
        capacity_ratio_upper: distortion.capacity_ratio_upper,
        volume_ratio_upper: distortion.volume_ratio_upper,
        sys_ratio_lower: distortion.sys_ratio_lower,
        sys_ratio_upper: distortion.sys_ratio_upper,
    }
}

impl ProductFacetRemovalReport {
    pub fn not_attempted(dual_vertices: &[Vector4<f64>]) -> Self {
        Self::unchanged(
            ProductFacetRemovalStatus::NotAttempted,
            dual_vertices.to_vec(),
        )
    }

    fn not_block_product(dual_vertices: &[Vector4<f64>]) -> Self {
        Self::unchanged(
            ProductFacetRemovalStatus::NotBlockProduct,
            dual_vertices.to_vec(),
        )
    }

    fn unchanged(status: ProductFacetRemovalStatus, dual_vertices: Vec<Vector4<f64>>) -> Self {
        let kept_original_indices = (0..dual_vertices.len()).collect();
        let distortion = distortion_from_delta_bound(0.0);
        Self {
            status,
            vertices_after_removal: dual_vertices,
            kept_original_indices,
            removed_facets: Vec::new(),
            delta_bound: 0.0,
            capacity_ratio_upper: distortion.capacity_ratio_upper,
            volume_ratio_upper: distortion.volume_ratio_upper,
            sys_ratio_lower: distortion.sys_ratio_lower,
            sys_ratio_upper: distortion.sys_ratio_upper,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ProductFacetRemovalDistortion {
    capacity_ratio_upper: f64,
    volume_ratio_upper: f64,
    sys_ratio_lower: f64,
    sys_ratio_upper: f64,
}

fn distortion_from_delta_bound(delta_bound: f64) -> ProductFacetRemovalDistortion {
    let scale = 1.0 + delta_bound;
    ProductFacetRemovalDistortion {
        capacity_ratio_upper: scale.powi(2),
        volume_ratio_upper: scale.powi(4),
        sys_ratio_lower: scale.powi(-4),
        sys_ratio_upper: scale.powi(4),
    }
}

#[derive(Clone, Debug)]
struct FactorFacet {
    original_index: usize,
    factor_index: usize,
    block: ProductBlock,
    normal: Vector2<f64>,
}

fn factor_facets(
    dual_vertices: &[Vector4<f64>],
    original_indices: &[usize],
    block: ProductBlock,
) -> Vec<FactorFacet> {
    original_indices
        .iter()
        .enumerate()
        .map(|(factor_index, &original_index)| {
            let vertex = dual_vertices[original_index];
            let normal = match block {
                ProductBlock::Q => Vector2::new(vertex[0], vertex[1]),
                ProductBlock::P => Vector2::new(vertex[2], vertex[3]),
            };
            FactorFacet {
                original_index,
                factor_index,
                block,
                normal,
            }
        })
        .collect()
}

struct FactorRemovalPlan {
    removed_facets: Vec<ProductFacetRemoval>,
    delta_bound: f64,
}

fn near_redundant_factor_facets(facets: &[FactorFacet], max_delta: f64) -> FactorRemovalPlan {
    let mut current = facets.to_vec();
    current.sort_by(|left, right| {
        left.normal[1]
            .atan2(left.normal[0])
            .total_cmp(&right.normal[1].atan2(right.normal[0]))
            .then_with(|| left.original_index.cmp(&right.original_index))
    });

    let mut removed_facets = Vec::new();
    let mut scale_bound = 1.0f64;
    loop {
        if current.len() <= 3 {
            break;
        }
        let remaining_delta_budget = (1.0 + max_delta) / scale_bound - 1.0;
        if remaining_delta_budget < 0.0 {
            break;
        }
        let Some(candidate) = best_cyclic_removal_candidate(&current, remaining_delta_budget)
        else {
            break;
        };
        let facet = current.remove(candidate.sorted_index);
        scale_bound *= 1.0 + candidate.delta;
        removed_facets.push(ProductFacetRemoval {
            original_index: facet.original_index,
            block: facet.block,
            factor_index: facet.factor_index,
            delta: candidate.delta,
        });
    }

    FactorRemovalPlan {
        removed_facets,
        delta_bound: (scale_bound - 1.0).max(0.0),
    }
}

#[derive(Clone, Copy, Debug)]
struct CyclicRemovalCandidate {
    sorted_index: usize,
    delta: f64,
}

fn best_cyclic_removal_candidate(
    facets: &[FactorFacet],
    max_delta: f64,
) -> Option<CyclicRemovalCandidate> {
    let mut best: Option<CyclicRemovalCandidate> = None;
    for sorted_index in 0..facets.len() {
        let Some(delta) = cyclic_single_removal_delta(facets, sorted_index) else {
            continue;
        };
        if delta > max_delta {
            continue;
        }
        let candidate = CyclicRemovalCandidate {
            sorted_index,
            delta,
        };
        let replace = best.as_ref().is_none_or(|current| {
            candidate.delta < current.delta
                || (candidate.delta == current.delta
                    && facets[candidate.sorted_index].original_index
                        < facets[current.sorted_index].original_index)
        });
        if replace {
            best = Some(candidate);
        }
    }
    best
}

fn cyclic_single_removal_delta(facets: &[FactorFacet], sorted_index: usize) -> Option<f64> {
    let n = facets.len();
    if n <= 3 {
        return None;
    }
    // For one removed facet in a 2D cyclic H-representation, the only new
    // candidate vertex is the intersection of its two cyclic neighbors.
    let previous_index = (sorted_index + n - 1) % n;
    let next_index = (sorted_index + 1) % n;
    let vertex = factor_vertex(facets[previous_index].normal, facets[next_index].normal)?;
    if !facets
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx != sorted_index)
        .all(|(_, facet)| facet.normal.dot(&vertex) <= 1.0 + FACTOR_FEASIBILITY_TOLERANCE)
    {
        return None;
    }
    Some((facets[sorted_index].normal.dot(&vertex) - 1.0).max(0.0))
}

fn factor_vertex(first: Vector2<f64>, second: Vector2<f64>) -> Option<Vector2<f64>> {
    let det = first[0] * second[1] - first[1] * second[0];
    // Small denominators are rejected. Once this guard passes, the remaining
    // arithmetic is a 2D solve and dot products, so f64 roundoff is negligible
    // at the experiment scales compared with the chosen removal budget.
    if det.abs() <= FACTOR_DET_TOLERANCE {
        return None;
    }
    Some(Vector2::new(
        (second[1] - first[1]) / det,
        (first[0] - second[0]) / det,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_facet_removal_removes_tiny_corner_facet_with_bound() {
        let eps = 1e-8;
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(1.0, eps, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, -1.0),
        ];
        let report = remove_near_redundant_facets(&dual_vertices, 2e-8);
        assert_eq!(report.status, ProductFacetRemovalStatus::Removed);
        assert_eq!(report.removed_facets.len(), 1);
        assert!(report.removed_facets[0].delta <= 2e-8);
        assert_eq!(report.vertices_after_removal.len(), 7);
        let scale = 1.0 + report.delta_bound;
        assert_eq!(report.capacity_ratio_upper, scale.powi(2));
        assert_eq!(report.volume_ratio_upper, scale.powi(4));
        assert_eq!(report.sys_ratio_lower, scale.powi(-4));
        assert_eq!(report.sys_ratio_upper, scale.powi(4));
    }

    #[test]
    fn product_facet_removal_can_remove_multiple_factor_facets_sequentially() {
        let eps = 1e-8;
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(1.0, eps, 0.0, 0.0),
            Vector4::new(eps, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, -1.0),
        ];

        let report = remove_near_redundant_facets(&dual_vertices, 3e-8);

        assert_eq!(report.status, ProductFacetRemovalStatus::Removed);
        assert_eq!(report.removed_facets.len(), 2);
        assert_eq!(report.removed_facets[0].block, ProductBlock::Q);
        assert_eq!(report.removed_facets[1].block, ProductBlock::Q);
        let removed_original_indices = report
            .removed_facets
            .iter()
            .map(|facet| facet.original_index)
            .collect::<Vec<_>>();
        assert!(report
            .removed_facets
            .iter()
            .all(|facet| [0, 1, 2, 3].contains(&facet.original_index)));
        assert_eq!(
            removed_original_indices
                .iter()
                .filter(|idx| [0, 1].contains(idx))
                .count(),
            1
        );
        assert_eq!(
            removed_original_indices
                .iter()
                .filter(|idx| [2, 3].contains(idx))
                .count(),
            1
        );
        assert!(report.delta_bound <= 3e-8);
        assert_eq!(report.vertices_after_removal.len(), 7);
    }

    #[test]
    fn product_facet_removal_leaves_valid_product_without_near_redundancy_unchanged() {
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, -1.0),
        ];

        let report = remove_near_redundant_facets(&dual_vertices, 1e-8);

        assert_eq!(
            report.status,
            ProductFacetRemovalStatus::NoNearRedundantFacets
        );
        assert_eq!(report.vertices_after_removal, dual_vertices);
        assert_eq!(report.removed_facets.len(), 0);
        assert_eq!(report.delta_bound, 0.0);
        assert_eq!(report.capacity_ratio_upper, 1.0);
        assert_eq!(report.volume_ratio_upper, 1.0);
        assert_eq!(report.sys_ratio_lower, 1.0);
        assert_eq!(report.sys_ratio_upper, 1.0);
    }

    #[test]
    fn product_facet_removal_reports_invalid_delta() {
        let dual_vertices = vec![Vector4::new(1.0, 0.0, 0.0, 0.0)];
        let report = remove_near_redundant_facets(&dual_vertices, f64::NAN);

        assert_eq!(report.status, ProductFacetRemovalStatus::InvalidDelta);
        assert_eq!(report.vertices_after_removal, dual_vertices);
        assert_eq!(report.delta_bound, 0.0);
    }

    #[test]
    fn product_facet_removal_reports_non_products() {
        let dual_vertices = vec![Vector4::new(1.0, 0.0, 0.1, 0.0)];
        let report = remove_near_redundant_facets(&dual_vertices, 1e-8);
        assert_eq!(report.status, ProductFacetRemovalStatus::NotBlockProduct);
        assert_eq!(report.vertices_after_removal, dual_vertices);
        assert_eq!(report.delta_bound, 0.0);
        assert_eq!(report.capacity_ratio_upper, 1.0);
        assert_eq!(report.volume_ratio_upper, 1.0);
        assert_eq!(report.sys_ratio_lower, 1.0);
        assert_eq!(report.sys_ratio_upper, 1.0);
    }
}
