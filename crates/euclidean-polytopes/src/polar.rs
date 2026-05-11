use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Vector4};

use crate::linalg::{combinations4, dot4_exact, solve4_exact};
use crate::predicates::origin_in_interior_of_conv_exact;

/// Exact vertices of a normalized polar polytope and their input-facet incidence.
#[derive(Clone, Debug, PartialEq)]
pub struct PolarVerticesExact<T: ExactScalar + 'static> {
    pub vertices: Vec<Vector4<T>>,
    pub vertex_facet_incidence: DMatrix<bool>,
}

/// Enumerate vertices of `{ y in R^4 : <v_i, y> <= 1 }` exactly.
///
/// Checked precondition: `0 in int conv(vertices)`. This condition makes the
/// normalized polar full-dimensional and bounded. The input points do not have
/// to be non-redundant; redundant points add redundant inequalities and do not
/// change the returned exact vertex set.
///
/// Panics when the origin-interior contract is violated.
pub fn polar_vertices_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
) -> PolarVerticesExact<T> {
    assert!(
        origin_in_interior_of_conv_exact(vertices),
        "polar_vertices_exact requires 0 in int conv(vertices)"
    );

    let one = T::one();
    let rhs = Vector4::new(one.clone(), one.clone(), one.clone(), one.clone());
    let vertices_f64 = f64_prefilter_vertices(vertices);
    let mut polar_vertices = Vec::new();

    for tuple in combinations4(vertices.len()) {
        if let Some(vertices_f64) = &vertices_f64 {
            if f64_prefilter_rejects(vertices_f64, &tuple) {
                continue;
            }
        }

        let rows = tuple.map(|idx| vertices[idx].clone());
        let Some(candidate) = solve4_exact(&rows, &rhs) else {
            continue;
        };

        if vertices
            .iter()
            .all(|vertex| dot4_exact(vertex, &candidate) <= one)
            && !polar_vertices.iter().any(|known| known == &candidate)
        {
            polar_vertices.push(candidate);
        }
    }

    assert!(
        !polar_vertices.is_empty(),
        "origin-interior polar input produced no exact vertices"
    );

    let vertex_facet_incidence =
        DMatrix::from_fn(polar_vertices.len(), vertices.len(), |row, col| {
            dot4_exact(&vertices[col], &polar_vertices[row]) == one
        });

    PolarVerticesExact {
        vertices: polar_vertices,
        vertex_facet_incidence,
    }
}

fn f64_prefilter_vertices<T: ExactScalar>(vertices: &[Vector4<T>]) -> Option<Vec<[f64; 4]>> {
    vertices
        .iter()
        .map(|vertex| {
            let coordinates = std::array::from_fn(|coordinate| vertex[coordinate].round_to_f64());
            if coordinates.iter().all(|coordinate| coordinate.is_finite()) {
                Some(coordinates)
            } else {
                None
            }
        })
        .collect()
}

fn f64_prefilter_rejects(vertices_f64: &[[f64; 4]], tuple: &[usize; 4]) -> bool {
    use nalgebra::{Matrix4, Vector4};

    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const CONDITIONING_SAFETY_FACTOR: f64 = 1e4;

    let matrix = Matrix4::new(
        vertices_f64[tuple[0]][0],
        vertices_f64[tuple[0]][1],
        vertices_f64[tuple[0]][2],
        vertices_f64[tuple[0]][3],
        vertices_f64[tuple[1]][0],
        vertices_f64[tuple[1]][1],
        vertices_f64[tuple[1]][2],
        vertices_f64[tuple[1]][3],
        vertices_f64[tuple[2]][0],
        vertices_f64[tuple[2]][1],
        vertices_f64[tuple[2]][2],
        vertices_f64[tuple[2]][3],
        vertices_f64[tuple[3]][0],
        vertices_f64[tuple[3]][1],
        vertices_f64[tuple[3]][2],
        vertices_f64[tuple[3]][3],
    );

    let svd = matrix.svd(true, true);
    let singular_values = &svd.singular_values;
    let sigma_min = singular_values[0]
        .min(singular_values[1])
        .min(singular_values[2])
        .min(singular_values[3]);
    let sigma_max = singular_values[0]
        .max(singular_values[1])
        .max(singular_values[2])
        .max(singular_values[3]);

    if sigma_min == 0.0 {
        return false;
    }

    let condition_number_bound = sigma_max / sigma_min;
    if EPS_MACH * condition_number_bound > 0.25 {
        return false;
    }

    let rhs = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let candidate = match svd.solve(&rhs, 0.0) {
        Ok(candidate) => candidate,
        Err(_) => return false,
    };

    if candidate.iter().any(|&coordinate| !coordinate.is_finite()) {
        return false;
    }

    let candidate_norm = candidate.norm();

    for (point_index, point) in vertices_f64.iter().enumerate() {
        if tuple.contains(&point_index) {
            continue;
        }

        let dot = point[0] * candidate[0]
            + point[1] * candidate[1]
            + point[2] * candidate[2]
            + point[3] * candidate[3];
        let point_norm =
            (point[0] * point[0] + point[1] * point[1] + point[2] * point[2] + point[3] * point[3])
                .sqrt();
        let absolute_error_bound = CONDITIONING_SAFETY_FACTOR
            * condition_number_bound
            * EPS_MACH
            * candidate_norm
            * point_norm;

        if !dot.is_finite() || !absolute_error_bound.is_finite() {
            return false;
        }

        if dot > 1.0 + absolute_error_bound {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::f64_prefilter_rejects;

    #[test]
    fn f64_prefilter_rejects_clearly_outside_candidate() {
        let vertices = vec![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 2.0, 2.0, 2.0],
        ];

        assert!(f64_prefilter_rejects(&vertices, &[0, 1, 2, 3]));
    }

    #[test]
    fn f64_prefilter_keeps_plausible_candidate_for_exact_resolution() {
        let vertices = vec![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [-0.25, -0.25, -0.25, -0.25],
        ];

        assert!(!f64_prefilter_rejects(&vertices, &[0, 1, 2, 3]));
    }
}
