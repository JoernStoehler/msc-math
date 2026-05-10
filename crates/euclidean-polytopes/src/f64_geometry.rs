/// Recoverable validation errors for approximate geometry APIs.
#[derive(Clone, Debug, PartialEq)]
pub enum F64GeometryError {
    NonFiniteCoordinate {
        vector_role: &'static str,
        vector_index: usize,
        coordinate_index: usize,
        value: f64,
    },
}

pub(crate) fn validate_finite_vectors4(
    vector_role: &'static str,
    vectors: &[nalgebra::Vector4<f64>],
) -> Result<(), F64GeometryError> {
    for (vector_index, vector) in vectors.iter().enumerate() {
        for coordinate_index in 0..4 {
            let value = vector[coordinate_index];
            if !value.is_finite() {
                return Err(F64GeometryError::NonFiniteCoordinate {
                    vector_role,
                    vector_index,
                    coordinate_index,
                    value,
                });
            }
        }
    }

    Ok(())
}

pub(crate) fn signed_gap_abs_error_bound(
    facet: &nalgebra::Vector4<f64>,
    vertex: &nalgebra::Vector4<f64>,
) -> f64 {
    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const ERROR_SCALE: f64 = 1.0e4;

    ERROR_SCALE * EPS_MACH * (facet.norm() * vertex.norm() + facet.dot(vertex).abs() + 1.0)
}
