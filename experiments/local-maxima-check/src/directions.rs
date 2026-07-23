use nalgebra::{DVector, Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};

const ORTHONORMAL_TOLERANCE: f64 = 2.0e-10;

#[derive(Debug)]
pub(crate) struct QuotientBasis {
    pub(crate) orbit_basis: Vec<DVector<f64>>,
    pub(crate) slice_basis: Vec<DVector<f64>>,
    pub(crate) orbit_generator_count: usize,
    pub(crate) max_orbit_orthonormal_error: f64,
    pub(crate) max_slice_orthonormal_error: f64,
    pub(crate) max_cross_inner_product: f64,
}

pub(crate) fn quotient_basis(duals: &[Vector4<f64>]) -> QuotientBasis {
    let generators = symmetry_generators(duals);
    let orbit_basis = orthonormalize(&generators, 1.0e-11);
    let ambient_dimension = duals.len() * 4;
    let mut slice_basis = Vec::with_capacity(ambient_dimension - orbit_basis.len());
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
        orbit_basis,
        slice_basis,
        orbit_generator_count: generators.len(),
    };
    validate_quotient_basis(&result, duals.len());
    result
}

pub(crate) fn random_quotient_directions(
    quotient: &QuotientBasis,
    count: usize,
    seed: u64,
) -> Vec<DVector<f64>> {
    let ambient_dimension = quotient
        .orbit_basis
        .first()
        .or_else(|| quotient.slice_basis.first())
        .expect("nonempty ambient basis")
        .len();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut result = Vec::with_capacity(count);
    while result.len() < count {
        let mut vector =
            DVector::from_fn(ambient_dimension, |_, _| StandardNormal.sample(&mut rng));
        project_away(&mut vector, &quotient.orbit_basis);
        let norm = vector.norm();
        if norm > 1.0e-10 {
            result.push(vector / norm);
        }
    }
    result
}

pub(crate) fn perturb_linearly(
    base: &[Vector4<f64>],
    direction: &DVector<f64>,
    absolute_radius: f64,
) -> Vec<Vector4<f64>> {
    let direction = unflatten_vector(direction);
    base.iter()
        .zip(direction)
        .map(|(a, d)| a + absolute_radius * d)
        .collect()
}

pub(crate) fn orientation_perturbation(
    base: &[Vector4<f64>],
    angle: f64,
    phi: f64,
) -> Vec<Vector4<f64>> {
    let (k1, k2) = orientation_generators();
    let generator = phi.cos() * k1 + phi.sin() * k2;
    // k1 and k2 anticommute and square to -I, hence so does every unit
    // combination. This is the exact matrix exponential.
    let map = angle.cos() * Matrix4::identity() + angle.sin() * generator;
    base.iter().map(|a| map * a).collect()
}

pub(crate) fn orientation_generators() -> (Matrix4<f64>, Matrix4<f64>) {
    #[rustfmt::skip]
    let k1 = Matrix4::new(
         0.0, -1.0,  0.0,  0.0,
         1.0,  0.0,  0.0,  0.0,
         0.0,  0.0,  0.0,  1.0,
         0.0,  0.0, -1.0,  0.0,
    );
    #[rustfmt::skip]
    let k2 = Matrix4::new(
         0.0,  0.0,  0.0, -1.0,
         0.0,  0.0,  1.0,  0.0,
         0.0, -1.0,  0.0,  0.0,
         1.0,  0.0,  0.0,  0.0,
    );
    (k1, k2)
}

pub(crate) fn flatten_vectors(vectors: &[Vector4<f64>]) -> DVector<f64> {
    DVector::from_iterator(
        vectors.len() * 4,
        vectors.iter().flat_map(|v| v.iter().copied()),
    )
}

pub(crate) fn l2_norm(vectors: &[Vector4<f64>]) -> f64 {
    flatten_vectors(vectors).norm()
}

fn symmetry_generators(duals: &[Vector4<f64>]) -> Vec<DVector<f64>> {
    let mut generators = Vec::with_capacity(15);
    // Translation y: a_i -> a_i/(1 + <a_i,y>), hence da_i=-<a_i,y>a_i.
    for coordinate in 0..4 {
        generators.push(flatten_vectors(
            &duals.iter().map(|a| -a[coordinate] * a).collect::<Vec<_>>(),
        ));
    }
    // Positive scaling rho: a_i -> rho^{-1} a_i.
    generators.push(flatten_vectors(
        &duals.iter().map(|a| -a).collect::<Vec<_>>(),
    ));
    for x in sp4_basis() {
        generators.push(flatten_vectors(
            &duals.iter().map(|a| -x.transpose() * a).collect::<Vec<_>>(),
        ));
    }
    generators
}

fn sp4_basis() -> Vec<Matrix4<f64>> {
    let mut result = Vec::with_capacity(10);
    for row in 0..2 {
        for col in 0..2 {
            let mut x = Matrix4::zeros();
            x[(row, col)] = 1.0;
            x[(2 + col, 2 + row)] = -1.0;
            result.push(x);
        }
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut x = Matrix4::zeros();
        x[(row, 2 + col)] = 1.0;
        x[(col, 2 + row)] = 1.0;
        result.push(x);
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut x = Matrix4::zeros();
        x[(2 + row, col)] = 1.0;
        x[(2 + col, row)] = 1.0;
        result.push(x);
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
    let mut result: f64 = 0.0;
    for (i, left) in basis.iter().enumerate() {
        for (j, right) in basis.iter().enumerate() {
            result = result.max((left.dot(right) - f64::from(i == j)).abs());
        }
    }
    result
}

fn max_cross_inner_product(left: &[DVector<f64>], right: &[DVector<f64>]) -> f64 {
    left.iter()
        .flat_map(|u| right.iter().map(move |v| u.dot(v).abs()))
        .fold(0.0, f64::max)
}

fn validate_quotient_basis(quotient: &QuotientBasis, facet_count: usize) {
    assert_eq!(quotient.orbit_generator_count, 15);
    assert_eq!(
        quotient.orbit_basis.len() + quotient.slice_basis.len(),
        4 * facet_count
    );
    assert!(quotient.max_orbit_orthonormal_error <= ORTHONORMAL_TOLERANCE);
    assert!(quotient.max_slice_orthonormal_error <= ORTHONORMAL_TOLERANCE);
    assert!(quotient.max_cross_inner_product <= ORTHONORMAL_TOLERANCE);
}

fn unflatten_vector(vector: &DVector<f64>) -> Vec<Vector4<f64>> {
    vector
        .as_slice()
        .chunks_exact(4)
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use symplectic::geom::symplectic_form::j4;

    #[test]
    fn orientation_generators_are_the_so4_mod_u2_tangent() {
        let j = j4();
        let (k1, k2) = orientation_generators();
        for k in [k1, k2] {
            assert!((k.transpose() + k).norm() <= 1.0e-15);
            assert!((k * j + j * k).norm() <= 1.0e-15);
            assert!((k * k + Matrix4::identity()).norm() <= 1.0e-15);
        }
        assert!((k1 * k2 + k2 * k1).norm() <= 1.0e-15);
    }
}
