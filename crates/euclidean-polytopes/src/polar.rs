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
    let mut polar_vertices = Vec::new();

    for tuple in combinations4(vertices.len()) {
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
