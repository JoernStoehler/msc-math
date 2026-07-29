use nalgebra::{DVector, Matrix4, Vector4};

#[derive(Clone, Debug)]
pub struct QuotientBasis {
    pub orbit_basis: Vec<DVector<f64>>,
    pub slice_basis: Vec<DVector<f64>>,
    pub orbit_generator_count: usize,
    pub max_orbit_orthonormal_error: f64,
    pub max_slice_orthonormal_error: f64,
    pub max_cross_inner_product: f64,
}

pub fn flatten(vectors: &[Vector4<f64>]) -> Vec<f64> {
    vectors
        .iter()
        .flat_map(|x| [x[0], x[1], x[2], x[3]])
        .collect()
}

pub fn unflatten(flat: &[f64]) -> Result<Vec<Vector4<f64>>, String> {
    if flat.len() % 4 != 0 {
        return Err(format!(
            "dual coordinate count {} is not divisible by four",
            flat.len()
        ));
    }
    Ok(flat
        .chunks_exact(4)
        .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
        .collect())
}

pub fn l2_norm(vectors: &[Vector4<f64>]) -> f64 {
    flatten(vectors).iter().map(|x| x * x).sum::<f64>().sqrt()
}

pub fn displacement_l2(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).norm_squared())
        .sum::<f64>()
        .sqrt()
}

pub fn add_flat_direction(
    base: &[Vector4<f64>],
    direction: &DVector<f64>,
    scale: f64,
) -> Vec<Vector4<f64>> {
    let mut flat = flatten(base);
    for (entry, delta) in flat.iter_mut().zip(direction.iter()) {
        *entry += scale * delta;
    }
    unflatten(&flat).expect("a quotient direction has the ambient dimension")
}

pub fn quotient_basis(duals: &[Vector4<f64>]) -> Result<QuotientBasis, String> {
    let generators = symmetry_generators(duals);
    let orbit_basis = orthonormalize(&generators, 1.0e-11);
    let ambient_dimension = duals.len() * 4;
    let mut slice_basis = Vec::with_capacity(ambient_dimension.saturating_sub(orbit_basis.len()));
    for coordinate in 0..ambient_dimension {
        let mut candidate = DVector::zeros(ambient_dimension);
        candidate[coordinate] = 1.0;
        project_away(&mut candidate, &orbit_basis);
        project_away(&mut candidate, &slice_basis);
        project_away(&mut candidate, &orbit_basis);
        project_away(&mut candidate, &slice_basis);
        let norm = candidate.norm();
        if norm > 1.0e-10 {
            slice_basis.push(candidate / norm);
        }
    }
    let result = QuotientBasis {
        max_orbit_orthonormal_error: max_orthonormal_error(&orbit_basis),
        max_slice_orthonormal_error: max_orthonormal_error(&slice_basis),
        max_cross_inner_product: max_cross_inner_product(&orbit_basis, &slice_basis),
        orbit_generator_count: generators.len(),
        orbit_basis,
        slice_basis,
    };
    let tolerance = 1.0e-8;
    if result.orbit_generator_count != 15
        || result.orbit_basis.len() + result.slice_basis.len() != ambient_dimension
        || result.max_orbit_orthonormal_error > tolerance
        || result.max_slice_orthonormal_error > tolerance
        || result.max_cross_inner_product > tolerance
    {
        return Err(format!(
            "invalid quotient basis: orbit generators {}, ranks {}+{} != {}, errors ({:.3e}, {:.3e}, {:.3e})",
            result.orbit_generator_count,
            result.orbit_basis.len(),
            result.slice_basis.len(),
            ambient_dimension,
            result.max_orbit_orthonormal_error,
            result.max_slice_orthonormal_error,
            result.max_cross_inner_product
        ));
    }
    Ok(result)
}

fn symmetry_generators(duals: &[Vector4<f64>]) -> Vec<DVector<f64>> {
    let mut generators = Vec::with_capacity(15);
    for coordinate in 0..4 {
        generators.push(DVector::from_vec(flatten(
            &duals.iter().map(|a| -a[coordinate] * a).collect::<Vec<_>>(),
        )));
    }
    generators.push(DVector::from_vec(flatten(
        &duals.iter().map(|a| -a).collect::<Vec<_>>(),
    )));
    for generator in sp4_basis() {
        generators.push(DVector::from_vec(flatten(
            &duals
                .iter()
                .map(|a| -generator.transpose() * a)
                .collect::<Vec<_>>(),
        )));
    }
    generators
}

fn sp4_basis() -> Vec<Matrix4<f64>> {
    let mut result = Vec::with_capacity(10);
    for row in 0..2 {
        for col in 0..2 {
            let mut generator = Matrix4::zeros();
            generator[(row, col)] = 1.0;
            generator[(2 + col, 2 + row)] = -1.0;
            result.push(generator);
        }
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut generator = Matrix4::zeros();
        generator[(row, 2 + col)] = 1.0;
        generator[(col, 2 + row)] = 1.0;
        result.push(generator);
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut generator = Matrix4::zeros();
        generator[(2 + row, col)] = 1.0;
        generator[(2 + col, row)] = 1.0;
        result.push(generator);
    }
    result
}

fn orthonormalize(vectors: &[DVector<f64>], tolerance: f64) -> Vec<DVector<f64>> {
    let mut basis = Vec::new();
    for source in vectors {
        let mut candidate = source.clone();
        project_away(&mut candidate, &basis);
        project_away(&mut candidate, &basis);
        let norm = candidate.norm();
        if norm > tolerance * source.norm().max(1.0) {
            basis.push(candidate / norm);
        }
    }
    basis
}

fn project_away(vector: &mut DVector<f64>, basis: &[DVector<f64>]) {
    for axis in basis {
        *vector -= axis * axis.dot(vector);
    }
}

fn max_orthonormal_error(basis: &[DVector<f64>]) -> f64 {
    let mut maximum: f64 = 0.0;
    for (i, left) in basis.iter().enumerate() {
        for (j, right) in basis.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            maximum = maximum.max((left.dot(right) - expected).abs());
        }
    }
    maximum
}

fn max_cross_inner_product(left: &[DVector<f64>], right: &[DVector<f64>]) -> f64 {
    left.iter()
        .flat_map(|a| right.iter().map(move |b| a.dot(b).abs()))
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_f6_has_nine_dimensional_slice() {
        let duals = vec![
            Vector4::new(0.8, 0.1, 0.2, -0.3),
            Vector4::new(-0.4, 0.9, -0.1, 0.2),
            Vector4::new(0.1, -0.5, 0.8, 0.3),
            Vector4::new(-0.2, -0.3, -0.4, 0.9),
            Vector4::new(0.3, 0.2, -0.8, -0.5),
            Vector4::new(-0.7, -0.4, 0.2, -0.1),
        ];
        let quotient = quotient_basis(&duals).unwrap();
        assert_eq!(quotient.orbit_basis.len(), 15);
        assert_eq!(quotient.slice_basis.len(), 9);
    }
}
