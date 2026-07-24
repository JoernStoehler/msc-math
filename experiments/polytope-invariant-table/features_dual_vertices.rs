//! Dual-vertex decoding and inspectability columns.

use nalgebra::Vector4;
use num_rational::BigRational;

use crate::load_caches::LoadedPolytopeRow;

use super::features_helpers::{parse_rational, rational_to_f64};

pub struct DualVertexFields {
    pub dual_vertex_count: usize,
    pub dual_vertices_f64: Vec<[f64; 4]>,
}

fn dual_vertices_big(row: &LoadedPolytopeRow) -> Vec<[BigRational; 4]> {
    row.dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| parse_rational(&vertex[i])))
        .collect()
}

pub fn raw_dual_vertices_f64(row: &LoadedPolytopeRow) -> Vec<Vector4<f64>> {
    let rationals = dual_vertices_big(row);
    rationals
        .iter()
        .map(|vertex| std::array::from_fn::<_, 4, _>(|i| rational_to_f64(&vertex[i])))
        .map(|vertex| Vector4::from_row_slice(&vertex))
        .collect()
}

pub fn dual_vertex_fields(dual_vertices: &[Vector4<f64>]) -> DualVertexFields {
    let arrays = dual_vertices
        .iter()
        .map(|vertex| std::array::from_fn(|i| vertex[i]))
        .collect::<Vec<_>>();
    DualVertexFields {
        dual_vertex_count: arrays.len(),
        dual_vertices_f64: arrays,
    }
}
