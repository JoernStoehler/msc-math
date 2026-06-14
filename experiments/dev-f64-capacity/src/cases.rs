use nalgebra::Vector4;

#[derive(Clone, Debug)]
pub struct ScanCase {
    pub family: String,
    pub source_id: String,
    pub input_source: String,
    pub generated_attempt: Option<u64>,
    pub generator_seed: Option<u64>,
    pub requested_facet_count: Option<usize>,
    pub dual_vertices: Vec<Vector4<f64>>,
    pub audit_capacity_label: Option<f64>,
    pub artifact_capacity_label: Option<f64>,
    pub audit_sigma_label: Option<Vec<usize>>,
}

pub fn array_vertices_to_vectors(data: &[[f64; 4]]) -> Vec<Vector4<f64>> {
    data.iter()
        .map(|row| Vector4::new(row[0], row[1], row[2], row[3]))
        .collect()
}
