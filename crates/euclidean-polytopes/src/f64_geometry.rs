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
