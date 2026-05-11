//! Algebraic 4D polytope construction over experiment-owned reporting scalars.
//!
//! This stays experiment-scoped and field-generic: enough for exact HKO-family
//! geometry and rational controls, not a reusable geometry API.
//!
//! TODO: add [def:...] to formal math for the exact dual-vertex polytope model.
//! TODO: add [lem:...] to formal math for the exact 4D vertex enumeration and
//! irredundancy checks implemented in this module.

use super::field::{sign_of, ExactSign, ExperimentScalar};
use algebraic_numbers::{rank, solve_linear_system, LinearSystemSolution};
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeSet;

/// Errors from algebraic polytope construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlgebraicPolytopeError {
    TooFewFacets(usize),
    ZeroDualVertex(usize),
    Unbounded,
    NoVertices,
    RedundantFacet(usize),
}

/// Construct exact primal vertices and pairwise fields from dual vertices
/// `a_i` for `a_i . x <= 1`.
pub fn exact_polytope_fields<F: ExperimentScalar + 'static>(
    dual_vertices: &[[F; 4]],
) -> Result<(Vec<[F; 4]>, Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<i8>>), AlgebraicPolytopeError> {
    let f = dual_vertices.len();
    if f < 5 {
        return Err(AlgebraicPolytopeError::TooFewFacets(f));
    }
    for (idx, dual) in dual_vertices.iter().enumerate() {
        if dual.iter().all(|entry| entry.is_zero()) {
            return Err(AlgebraicPolytopeError::ZeroDualVertex(idx));
        }
    }

    check_bounded(dual_vertices)?;
    let (vertices, descriptors) = enumerate_vertices(dual_vertices)?;
    check_irredundancy(&vertices, &descriptors, f)?;

    let incidence: Vec<Vec<bool>> = descriptors
        .iter()
        .map(|incident| (0..f).map(|facet| incident.contains(&facet)).collect())
        .collect();

    let vertex_count = vertices.len();
    let facet_intersection_is_nonempty: Vec<Vec<bool>> = (0..f)
        .map(|i| {
            (0..f)
                .map(|j| i != j && (0..vertex_count).any(|v| incidence[v][i] && incidence[v][j]))
                .collect()
        })
        .collect();

    let omega_signs: Vec<Vec<i8>> = (0..f)
        .map(|i| {
            (0..f)
                .map(|j| {
                    if i == j {
                        0
                    } else {
                        match sign_of(&omega0(&dual_vertices[i], &dual_vertices[j])) {
                            ExactSign::Negative => -1,
                            ExactSign::Zero => 0,
                            ExactSign::Positive => 1,
                        }
                    }
                })
                .collect()
        })
        .collect();

    Ok((
        vertices,
        incidence,
        facet_intersection_is_nonempty,
        omega_signs,
    ))
}

/// Exact 4D dot product.
pub fn dot4<F: ExperimentScalar>(left: &[F; 4], right: &[F; 4]) -> F {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

/// Exact standard symplectic form `omega_0`.
pub fn omega0<F: ExperimentScalar>(u: &[F; 4], v: &[F; 4]) -> F {
    u[0].clone() * v[2].clone() - u[2].clone() * v[0].clone() + u[1].clone() * v[3].clone()
        - u[3].clone() * v[1].clone()
}

fn combinations4(n: usize) -> Vec<[usize; 4]> {
    let mut result = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    result.push([i, j, k, l]);
                }
            }
        }
    }
    result
}

fn cross_product_4d<F: ExperimentScalar>(a: &[F; 4], b: &[F; 4], c: &[F; 4]) -> [F; 4] {
    let bc_01 = b[0].clone() * c[1].clone() - b[1].clone() * c[0].clone();
    let bc_02 = b[0].clone() * c[2].clone() - b[2].clone() * c[0].clone();
    let bc_03 = b[0].clone() * c[3].clone() - b[3].clone() * c[0].clone();
    let bc_12 = b[1].clone() * c[2].clone() - b[2].clone() * c[1].clone();
    let bc_13 = b[1].clone() * c[3].clone() - b[3].clone() * c[1].clone();
    let bc_23 = b[2].clone() * c[3].clone() - b[3].clone() * c[2].clone();

    let d0 =
        a[1].clone() * bc_23.clone() - a[2].clone() * bc_13.clone() + a[3].clone() * bc_12.clone();
    let d1 = -(a[0].clone() * bc_23.clone() - a[2].clone() * bc_03.clone()
        + a[3].clone() * bc_02.clone());
    let d2 = a[0].clone() * bc_13 - a[1].clone() * bc_03.clone() + a[3].clone() * bc_01.clone();
    let d3 = -(a[0].clone() * bc_12 - a[1].clone() * bc_02 + a[2].clone() * bc_01);
    [d0, d1, d2, d3]
}

fn rank_row_vectors<F: ExperimentScalar + 'static>(rows: &[Vec<F>], ncols: usize) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let matrix = DMatrix::from_fn(rows.len(), ncols, |row, col| rows[row][col].clone());
    rank(&matrix)
}

fn solve4<F: ExperimentScalar + 'static>(rows: &[[F; 4]; 4], rhs: &[F; 4]) -> Option<[F; 4]> {
    let matrix = DMatrix::from_fn(4, 4, |row, col| rows[row][col].clone());
    let rhs = DVector::from_fn(4, |row, _| rhs[row].clone());
    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => Some(std::array::from_fn(|idx| particular[idx].clone())),
        _ => None,
    }
}

fn affine_rank<F: ExperimentScalar + 'static>(points: &[[F; 4]]) -> usize {
    if points.len() <= 1 {
        return 0;
    }
    let origin = &points[0];
    let centered: Vec<Vec<F>> = points[1..]
        .iter()
        .map(|point| {
            (0..4)
                .map(|i| point[i].clone() - origin[i].clone())
                .collect()
        })
        .collect();
    rank_row_vectors(&centered, 4)
}

fn check_bounded<F: ExperimentScalar + 'static>(
    dual_vertices: &[[F; 4]],
) -> Result<(), AlgebraicPolytopeError> {
    let rows: Vec<Vec<F>> = dual_vertices
        .iter()
        .map(|dual| (0..4).map(|i| dual[i].clone()).collect())
        .collect();
    if rank_row_vectors(&rows, 4) < 4 {
        return Err(AlgebraicPolytopeError::Unbounded);
    }

    let f = dual_vertices.len();
    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let normal =
                    cross_product_4d(&dual_vertices[i], &dual_vertices[j], &dual_vertices[k]);
                if normal.iter().all(|entry| entry.is_zero()) {
                    continue;
                }
                let has_pos = (0..f)
                    .filter(|&idx| idx != i && idx != j && idx != k)
                    .any(|idx| dot4(&dual_vertices[idx], &normal) > F::zero());
                let has_neg = (0..f)
                    .filter(|&idx| idx != i && idx != j && idx != k)
                    .any(|idx| dot4(&dual_vertices[idx], &normal) < F::zero());
                if !has_pos || !has_neg {
                    return Err(AlgebraicPolytopeError::Unbounded);
                }
            }
        }
    }

    Ok(())
}

fn enumerate_vertices<F: ExperimentScalar + 'static>(
    dual_vertices: &[[F; 4]],
) -> Result<(Vec<[F; 4]>, Vec<BTreeSet<usize>>), AlgebraicPolytopeError> {
    let f = dual_vertices.len();
    let rhs = [F::one(), F::one(), F::one(), F::one()];

    let mut vertices = Vec::new();
    let mut descriptors = Vec::new();

    for subset in combinations4(f) {
        let system = [
            dual_vertices[subset[0]].clone(),
            dual_vertices[subset[1]].clone(),
            dual_vertices[subset[2]].clone(),
            dual_vertices[subset[3]].clone(),
        ];
        let Some(candidate) = solve4(&system, &rhs) else {
            continue;
        };

        let mut incident = BTreeSet::from(subset);
        let mut valid = true;
        for (facet_idx, dual) in dual_vertices.iter().enumerate() {
            if subset.contains(&facet_idx) {
                continue;
            }
            let gap = dot4(dual, &candidate) - F::one();
            match sign_of(&gap) {
                ExactSign::Positive => {
                    valid = false;
                    break;
                }
                ExactSign::Zero => {
                    incident.insert(facet_idx);
                }
                ExactSign::Negative => {}
            }
        }
        if !valid {
            continue;
        }

        let already_seen = vertices
            .iter()
            .any(|existing: &[F; 4]| (0..4).all(|idx| existing[idx] == candidate[idx]));
        if already_seen {
            continue;
        }

        vertices.push(candidate);
        descriptors.push(incident);
    }

    if vertices.is_empty() {
        return Err(AlgebraicPolytopeError::NoVertices);
    }

    Ok((vertices, descriptors))
}

fn check_irredundancy<F: ExperimentScalar + 'static>(
    vertices: &[[F; 4]],
    descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> Result<(), AlgebraicPolytopeError> {
    for facet in 0..facet_count {
        let incident: Vec<[F; 4]> = descriptors
            .iter()
            .enumerate()
            .filter(|(_, descriptor)| descriptor.contains(&facet))
            .map(|(idx, _)| vertices[idx].clone())
            .collect();

        if incident.is_empty() || affine_rank(&incident) < 3 {
            return Err(AlgebraicPolytopeError::RedundantFacet(facet));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::dot4;
    use crate::algebraic::field::ExperimentScalar;
    use crate::algebraic::fixtures::{exact_hko_pentagon, exact_hypercube, exact_simplex};
    use nalgebra::DMatrix;
    use std::cmp::Ordering;
    use symplectic::geom::known_polytopes;

    fn normalized_incidence_rows(rows: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let mut rows = rows.to_vec();
        rows.sort();
        rows
    }

    fn matrix_rows<T: Copy>(matrix: &DMatrix<T>) -> Vec<Vec<T>> {
        (0..matrix.nrows())
            .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
            .collect()
    }

    fn same_incidence(lhs: &[Vec<bool>], rhs: &DMatrix<bool>) -> bool {
        lhs.len() == rhs.nrows()
            && normalized_incidence_rows(lhs) == normalized_incidence_rows(&matrix_rows(rhs))
    }

    fn same_bool_matrix(lhs: &[Vec<bool>], rhs: &DMatrix<bool>) -> bool {
        lhs.len() == rhs.nrows()
            && lhs.iter().enumerate().all(|(row_idx, row)| {
                row.len() == rhs.ncols()
                    && row
                        .iter()
                        .enumerate()
                        .all(|(col_idx, val)| *val == rhs[(row_idx, col_idx)])
            })
    }

    fn assert_exact_polytope_self_consistent<F: ExperimentScalar>(
        dual_vertices: &[[F; 4]],
        vertices: &[[F; 4]],
        incidence: &[Vec<bool>],
        facet_intersection_is_nonempty: &[Vec<bool>],
        omega_signs: &[Vec<i8>],
    ) {
        let one = F::one();

        for (vertex_idx, vertex) in vertices.iter().enumerate() {
            let mut incident_count = 0usize;
            for (facet_idx, dual) in dual_vertices.iter().enumerate() {
                let value = dot4(vertex, dual);
                let relation = value.cmp(&one);
                if incidence[vertex_idx][facet_idx] {
                    incident_count += 1;
                    assert_eq!(
                        relation,
                        Ordering::Equal,
                        "incident facet {facet_idx} should satisfy a_i.x = 1 at vertex {vertex_idx}"
                    );
                } else {
                    assert_eq!(
                        relation,
                        Ordering::Less,
                        "non-incident facet {facet_idx} should satisfy a_i.x < 1 at vertex {vertex_idx}"
                    );
                }
            }
            assert!(
                incident_count >= 4,
                "every stored 4D vertex should saturate at least four facets"
            );
        }

        let facet_count = dual_vertices.len();
        let vertex_count = vertices.len();
        for row in 0..facet_count {
            assert_eq!(omega_signs[row][row], 0, "omega diagonal should vanish");
            for col in 0..facet_count {
                let expected_adjacency = row != col
                    && (0..vertex_count)
                        .any(|vertex| incidence[vertex][row] && incidence[vertex][col]);
                assert_eq!(
                    facet_intersection_is_nonempty[row][col], expected_adjacency,
                    "facet-pair nonempty intersection should agree with shared exact vertices"
                );
                assert_eq!(
                    omega_signs[row][col], -omega_signs[col][row],
                    "omega sign matrix should be antisymmetric"
                );
            }
        }
    }

    #[test]
    fn simplex_geometry_matches_rational_control() {
        let dual_vertices = exact_simplex();
        let (vertices, incidence, facet_intersection_is_nonempty, omega_signs) =
            super::exact_polytope_fields(&dual_vertices).expect("exact simplex");
        let library = known_polytopes::simplex();

        assert_eq!(dual_vertices.len(), library.dual_vertices.len());
        assert_eq!(vertices.len(), library.vertices.len());
        assert!(same_incidence(&incidence, &library.incidence));
        assert_exact_polytope_self_consistent(
            &dual_vertices,
            &vertices,
            &incidence,
            &facet_intersection_is_nonempty,
            &omega_signs,
        );
    }

    #[test]
    fn hypercube_geometry_matches_rational_control() {
        let dual_vertices = exact_hypercube();
        let (vertices, incidence, facet_intersection_is_nonempty, omega_signs) =
            super::exact_polytope_fields(&dual_vertices).expect("exact hypercube");
        let library = known_polytopes::hypercube();

        assert_eq!(vertices.len(), library.vertices.len());
        assert!(same_incidence(&incidence, &library.incidence));
        assert_exact_polytope_self_consistent(
            &dual_vertices,
            &vertices,
            &incidence,
            &facet_intersection_is_nonempty,
            &omega_signs,
        );
    }

    #[test]
    fn hko_exact_geometry_is_self_consistent_and_detects_known_zero_pairs() {
        let dual_vertices = exact_hko_pentagon();
        let (vertices, incidence, facet_intersection_is_nonempty, omega_signs) =
            super::exact_polytope_fields(&dual_vertices).expect("exact hko");

        assert_eq!(dual_vertices.len(), 10);
        assert_eq!(vertices.len(), 25);
        assert_exact_polytope_self_consistent(
            &dual_vertices,
            &vertices,
            &incidence,
            &facet_intersection_is_nonempty,
            &omega_signs,
        );

        for &(i, j) in &[(1usize, 6usize), (3usize, 8usize), (4usize, 9usize)] {
            assert_eq!(omega_signs[i][j], 0, "known exact HKO zero pair ({i}, {j})");
            assert_eq!(omega_signs[j][i], 0, "known exact HKO zero pair ({j}, {i})");
        }
    }

    #[test]
    fn hko_vs_dyadic_combinatorics_stays_diagnostic_only() {
        let dual_vertices = exact_hko_pentagon();
        let (vertices, _, _, _) = super::exact_polytope_fields(&dual_vertices).expect("exact hko");
        let library = known_polytopes::hko_pentagon();

        assert_eq!(dual_vertices.len(), library.dual_vertices.len());
        assert_eq!(vertices.len(), library.vertices.len());
    }
}
