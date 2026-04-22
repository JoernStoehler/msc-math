use num_rational::BigRational;
use num_traits::ToPrimitive;

pub struct DualVertexFields {
    pub dual_vertex_count: usize,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub dual_vertices_flat_f64: Vec<f64>,
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
}

pub fn compute(dual_vertices_rational: &[[BigRational; 4]]) -> DualVertexFields {
    let dual_vertices_f64 = dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| rational_to_f64(&vertex[i])))
        .collect::<Vec<_>>();
    let dual_vertices_flat_f64 = dual_vertices_f64
        .iter()
        .flat_map(|vertex| vertex.iter().copied())
        .collect::<Vec<_>>();
    DualVertexFields {
        dual_vertex_count: dual_vertices_f64.len(),
        dual_vertices_f64,
        dual_vertices_flat_f64,
    }
}
