//! Dual-vertex decoding and flattened coordinate feature columns.

use nalgebra::Vector4;
use num_rational::BigRational;

use crate::load_caches::LoadedPolytopeRow;

use super::features_helpers::{parse_rational, rational_to_f64};

pub struct DualVertexFields {
    pub dual_vertex_count: usize,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub dual_vertices_flat_f64: Vec<f64>,
}

fn dual_vertices_big(row: &LoadedPolytopeRow) -> Vec<[BigRational; 4]> {
    row.dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| parse_rational(&vertex[i])))
        .collect()
}

pub fn dual_vertices_f64(row: &LoadedPolytopeRow) -> (Vec<Vector4<f64>>, DualVertexFields) {
    let rationals = dual_vertices_big(row);
    let arrays = rationals
        .iter()
        .map(|vertex| std::array::from_fn(|i| rational_to_f64(&vertex[i])))
        .collect::<Vec<_>>();
    let vectors = arrays
        .iter()
        .map(|vertex| Vector4::from_row_slice(vertex))
        .collect::<Vec<_>>();
    let flat = arrays
        .iter()
        .flat_map(|vertex| vertex.iter().copied())
        .collect::<Vec<_>>();
    (
        vectors,
        DualVertexFields {
            dual_vertex_count: arrays.len(),
            dual_vertices_f64: arrays,
            dual_vertices_flat_f64: flat,
        },
    )
}
