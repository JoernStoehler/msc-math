use crate::{round_product_blocks, ProductBlock, ProductRoundingStatus};
use nalgebra::{Vector2, Vector4};
use symplectic::classify_facets_from_dual_vertices;

const FACTOR_DET_TOLERANCE: f64 = 1e-12;
const FACTOR_FEASIBILITY_TOLERANCE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct ProductFacetRedundancy {
    pub original_index: usize,
    pub block: ProductBlock,
    pub factor_index: usize,
    pub delta: f64,
}

#[derive(Clone, Debug)]
pub struct ProductSimplificationReport {
    pub status: ProductSimplificationStatus,
    pub simplified_dual_vertices: Vec<Vector4<f64>>,
    pub kept_original_indices: Vec<usize>,
    pub removed_facets: Vec<ProductFacetRedundancy>,
    /// Intended set-level bound:
    /// P_original <= P_simplified <= (1 + delta_bound) P_original.
    /// See formal label `rem:product-simplification-experiment-contract`.
    pub delta_bound: f64,
    pub capacity_ratio_upper: f64,
    pub volume_ratio_upper: f64,
    pub sys_ratio_lower: f64,
    pub sys_ratio_upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductSimplificationStatus {
    NotAttempted,
    Simplified,
    NoNearRedundantFacets,
    NotBlockProduct,
}

impl ProductSimplificationStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Simplified => "simplified",
            Self::NoNearRedundantFacets => "no_near_redundant_facets",
            Self::NotBlockProduct => "not_block_product",
        }
    }
}

pub fn remove_nearly_redundant_product_facets(
    dual_vertices: &[Vector4<f64>],
    max_delta: f64,
) -> ProductSimplificationReport {
    let rounded = round_product_blocks(dual_vertices);
    if rounded.status != ProductRoundingStatus::Rounded {
        return ProductSimplificationReport::not_block_product(dual_vertices);
    }
    let dual_vertices = rounded.rounded_dual_vertices;
    let Ok(classification) = classify_facets_from_dual_vertices(&dual_vertices) else {
        return ProductSimplificationReport::not_block_product(&dual_vertices);
    };

    let q_facets = factor_facets(&dual_vertices, &classification.q_indices, ProductBlock::Q);
    let p_facets = factor_facets(&dual_vertices, &classification.p_indices, ProductBlock::P);
    let mut removed_facets = near_redundant_factor_facets(&q_facets, max_delta);
    removed_facets.extend(near_redundant_factor_facets(&p_facets, max_delta));
    removed_facets.sort_by_key(|facet| facet.original_index);

    if removed_facets.is_empty() {
        return ProductSimplificationReport::unchanged(
            ProductSimplificationStatus::NoNearRedundantFacets,
            dual_vertices,
        );
    }

    let mut remove = vec![false; dual_vertices.len()];
    let mut delta_bound = 0.0f64;
    for facet in &removed_facets {
        remove[facet.original_index] = true;
        delta_bound = delta_bound.max(facet.delta);
    }
    let mut simplified_dual_vertices = Vec::new();
    let mut kept_original_indices = Vec::new();
    for (idx, vertex) in dual_vertices.iter().enumerate() {
        if !remove[idx] {
            simplified_dual_vertices.push(*vertex);
            kept_original_indices.push(idx);
        }
    }
    let distortion = distortion_from_delta_bound(delta_bound);

    ProductSimplificationReport {
        status: ProductSimplificationStatus::Simplified,
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

impl ProductSimplificationReport {
    pub fn not_attempted(dual_vertices: &[Vector4<f64>]) -> Self {
        Self::unchanged(
            ProductSimplificationStatus::NotAttempted,
            dual_vertices.to_vec(),
        )
    }

    fn not_block_product(dual_vertices: &[Vector4<f64>]) -> Self {
        Self::unchanged(
            ProductSimplificationStatus::NotBlockProduct,
            dual_vertices.to_vec(),
        )
    }

    fn unchanged(status: ProductSimplificationStatus, dual_vertices: Vec<Vector4<f64>>) -> Self {
        let kept_original_indices = (0..dual_vertices.len()).collect();
        let distortion = distortion_from_delta_bound(0.0);
        Self {
            status,
            simplified_dual_vertices: dual_vertices,
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
struct ProductSimplificationDistortion {
    capacity_ratio_upper: f64,
    volume_ratio_upper: f64,
    sys_ratio_lower: f64,
    sys_ratio_upper: f64,
}

fn distortion_from_delta_bound(delta_bound: f64) -> ProductSimplificationDistortion {
    let scale = 1.0 + delta_bound;
    ProductSimplificationDistortion {
        capacity_ratio_upper: scale.powi(2),
        volume_ratio_upper: scale.powi(4),
        sys_ratio_lower: scale.powi(-4),
        sys_ratio_upper: scale.powi(4),
    }
}

#[derive(Clone, Debug)]
struct FactorFacet {
    original_index: usize,
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
                block,
                normal,
            }
        })
        .collect()
}

fn near_redundant_factor_facets(
    facets: &[FactorFacet],
    max_delta: f64,
) -> Vec<ProductFacetRedundancy> {
    let candidates = facets
        .iter()
        .enumerate()
        .filter_map(|(factor_index, _facet)| {
            let delta = removed_set_delta(facets, &[factor_index])?;
            (0.0 <= delta && delta <= max_delta).then_some(factor_index)
        })
        .collect::<Vec<_>>();
    let Some(selected) = selected_removal_subset(facets, &candidates, max_delta) else {
        return Vec::new();
    };
    selected
        .factor_indices
        .into_iter()
        .map(|factor_index| {
            let facet = &facets[factor_index];
            ProductFacetRedundancy {
                original_index: facet.original_index,
                block: facet.block,
                factor_index,
                delta: selected.delta,
            }
        })
        .collect()
}

struct SelectedRemovalSubset {
    factor_indices: Vec<usize>,
    delta: f64,
}

fn selected_removal_subset(
    facets: &[FactorFacet],
    candidates: &[usize],
    max_delta: f64,
) -> Option<SelectedRemovalSubset> {
    let mut best: Option<SelectedRemovalSubset> = None;
    for mask in 1usize..(1usize << candidates.len()) {
        let factor_indices = candidates
            .iter()
            .enumerate()
            .filter_map(|(pos, &idx)| ((mask & (1usize << pos)) != 0).then_some(idx))
            .collect::<Vec<_>>();
        let Some(delta) = removed_set_delta(facets, &factor_indices) else {
            continue;
        };
        if !(0.0 <= delta && delta <= max_delta) {
            continue;
        }
        let replace = best.as_ref().is_none_or(|current| {
            factor_indices.len() > current.factor_indices.len()
                || (factor_indices.len() == current.factor_indices.len() && delta < current.delta)
        });
        if replace {
            best = Some(SelectedRemovalSubset {
                factor_indices,
                delta,
            });
        }
    }
    best
}

fn removed_set_delta(facets: &[FactorFacet], removed_factor_indices: &[usize]) -> Option<f64> {
    let remaining = (0..facets.len())
        .filter(|idx| !removed_factor_indices.contains(idx))
        .collect::<Vec<_>>();
    if remaining.len() < 3 {
        return None;
    }

    let mut max_value: Option<f64> = None;
    for (pos, &first_idx) in remaining.iter().enumerate() {
        for &second_idx in &remaining[pos + 1..] {
            let Some(vertex) = factor_vertex(facets[first_idx].normal, facets[second_idx].normal)
            else {
                continue;
            };
            if remaining
                .iter()
                .all(|&idx| facets[idx].normal.dot(&vertex) <= 1.0 + FACTOR_FEASIBILITY_TOLERANCE)
            {
                for &removed_idx in removed_factor_indices {
                    let value = facets[removed_idx].normal.dot(&vertex) - 1.0;
                    max_value = Some(max_value.map_or(value, |current| current.max(value)));
                }
            }
        }
    }
    max_value
}

fn factor_vertex(first: Vector2<f64>, second: Vector2<f64>) -> Option<Vector2<f64>> {
    let det = first[0] * second[1] - first[1] * second[0];
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
    fn product_simplification_removes_tiny_corner_facet_with_bound() {
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
        let report = remove_nearly_redundant_product_facets(&dual_vertices, 2e-8);
        assert_eq!(report.status, ProductSimplificationStatus::Simplified);
        assert_eq!(report.removed_facets.len(), 1);
        assert!(report.removed_facets[0].delta <= 2e-8);
        assert_eq!(report.simplified_dual_vertices.len(), 7);
        let scale = 1.0 + report.delta_bound;
        assert_eq!(report.capacity_ratio_upper, scale.powi(2));
        assert_eq!(report.volume_ratio_upper, scale.powi(4));
        assert_eq!(report.sys_ratio_lower, scale.powi(-4));
        assert_eq!(report.sys_ratio_upper, scale.powi(4));
    }

    #[test]
    fn product_simplification_reports_non_products() {
        let dual_vertices = vec![Vector4::new(1.0, 0.0, 0.1, 0.0)];
        let report = remove_nearly_redundant_product_facets(&dual_vertices, 1e-8);
        assert_eq!(report.status, ProductSimplificationStatus::NotBlockProduct);
        assert_eq!(report.simplified_dual_vertices, dual_vertices);
        assert_eq!(report.delta_bound, 0.0);
        assert_eq!(report.capacity_ratio_upper, 1.0);
        assert_eq!(report.volume_ratio_upper, 1.0);
        assert_eq!(report.sys_ratio_lower, 1.0);
        assert_eq!(report.sys_ratio_upper, 1.0);
    }
}
