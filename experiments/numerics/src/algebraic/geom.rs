//! Exact 4D polytope construction over experiment-owned ordered fields.
//!
//! This mirrors the high-level role of `library::geom::polytope::Polytope4D`
//! but stays experiment-scoped and field-generic. It is intentionally small:
//! enough for exact HKO-family geometry and rational controls, not a drop-in
//! library replacement.
//!
//! TODO: add [def:...] to formal math for the exact dual-vertex polytope model.
//! TODO: add [lem:...] to formal math for the exact 4D vertex enumeration and
//! irredundancy checks implemented in this module.

use super::field::{ExactOrderedField, ExactSign};
use std::collections::BTreeSet;

/// Errors from exact polytope construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExactPolytopeError {
    TooFewFacets(usize),
    ZeroDualVertex(usize),
    Unbounded,
    NoVertices,
    RedundantFacet(usize),
}

/// Experiment-owned exact 4D polytope with exact combinatorics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPolytope4D<F: ExactOrderedField> {
    dual_vertices: Vec<[F; 4]>,
    vertices: Vec<[F; 4]>,
    incidence: Vec<Vec<bool>>,
    vertex_adjacency: Vec<Vec<bool>>,
    omega_signs: Vec<Vec<i8>>,
}

impl<F: ExactOrderedField> ExactPolytope4D<F> {
    /// Construct from exact dual vertices `a_i` for `a_i . x <= 1`.
    pub fn new(dual_vertices: Vec<[F; 4]>) -> Result<Self, ExactPolytopeError> {
        let f = dual_vertices.len();
        if f < 5 {
            return Err(ExactPolytopeError::TooFewFacets(f));
        }
        for (idx, dual) in dual_vertices.iter().enumerate() {
            if dual.iter().all(ExactOrderedField::is_zero) {
                return Err(ExactPolytopeError::ZeroDualVertex(idx));
            }
        }

        check_bounded(&dual_vertices)?;
        let (vertices, descriptors) = enumerate_vertices(&dual_vertices)?;
        check_irredundancy(&vertices, &descriptors, f)?;

        let incidence: Vec<Vec<bool>> = descriptors
            .iter()
            .map(|incident| (0..f).map(|facet| incident.contains(&facet)).collect())
            .collect();

        let vertex_count = vertices.len();
        let vertex_adjacency: Vec<Vec<bool>> = (0..f)
            .map(|i| {
                (0..f)
                    .map(|j| {
                        i != j && (0..vertex_count).any(|v| incidence[v][i] && incidence[v][j])
                    })
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
                            match omega0(&dual_vertices[i], &dual_vertices[j]).sign() {
                                ExactSign::Negative => -1,
                                ExactSign::Zero => 0,
                                ExactSign::Positive => 1,
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            dual_vertices,
            vertices,
            incidence,
            vertex_adjacency,
            omega_signs,
        })
    }

    /// Dual vertices in exact coordinates.
    pub fn dual_vertices(&self) -> &[[F; 4]] {
        &self.dual_vertices
    }

    /// Primal vertices in exact coordinates.
    pub fn vertices(&self) -> &[[F; 4]] {
        &self.vertices
    }

    /// Exact vertex-facet incidence.
    pub fn incidence(&self) -> &[Vec<bool>] {
        &self.incidence
    }

    /// Exact facet adjacency via shared vertices.
    pub fn vertex_adjacency(&self) -> &[Vec<bool>] {
        &self.vertex_adjacency
    }

    /// Exact sign of `omega_0(a_i, a_j)`.
    pub fn omega_signs(&self) -> &[Vec<i8>] {
        &self.omega_signs
    }

    /// Facet count.
    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }

    /// Best-effort `f64` dual-vertex approximation for comparisons/reporting.
    pub fn dual_vertices_f64(&self) -> Vec<[f64; 4]> {
        self.dual_vertices
            .iter()
            .map(|dual| std::array::from_fn(|i| dual[i].to_f64()))
            .collect()
    }

    /// Best-effort `f64` primal-vertex approximation for comparisons/reporting.
    pub fn vertices_f64(&self) -> Vec<[f64; 4]> {
        self.vertices
            .iter()
            .map(|vertex| std::array::from_fn(|i| vertex[i].to_f64()))
            .collect()
    }
}

/// Exact 4D dot product.
pub fn dot4<F: ExactOrderedField>(left: &[F; 4], right: &[F; 4]) -> F {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

/// Exact standard symplectic form `omega_0`.
pub fn omega0<F: ExactOrderedField>(u: &[F; 4], v: &[F; 4]) -> F {
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

fn cross_product_4d<F: ExactOrderedField>(a: &[F; 4], b: &[F; 4], c: &[F; 4]) -> [F; 4] {
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

fn rank_rows<F: ExactOrderedField>(rows: &[Vec<F>], ncols: usize) -> usize {
    if rows.is_empty() {
        return 0;
    }

    let mut mat = rows.to_vec();
    let m = mat.len();
    let mut rank = 0usize;

    for col in 0..ncols {
        let Some(pivot_row) = (rank..m).find(|&row| !mat[row][col].is_zero()) else {
            continue;
        };
        mat.swap(rank, pivot_row);
        let pivot = mat[rank][col].clone();

        for row in 0..m {
            if row == rank || mat[row][col].is_zero() {
                continue;
            }
            let factor = mat[row][col].clone() / pivot.clone();
            for j in col..ncols {
                let correction = factor.clone() * mat[rank][j].clone();
                mat[row][j] = mat[row][j].clone() - correction;
            }
        }
        rank += 1;
    }

    rank
}

fn solve4<F: ExactOrderedField>(rows: &[[F; 4]; 4], rhs: &[F; 4]) -> Option<[F; 4]> {
    let mut aug: Vec<Vec<F>> = (0..4)
        .map(|row| {
            let mut line: Vec<F> = (0..4).map(|col| rows[row][col].clone()).collect();
            line.push(rhs[row].clone());
            line
        })
        .collect();

    for col in 0..4 {
        let pivot_row = (col..4).find(|&row| !aug[row][col].is_zero())?;
        aug.swap(col, pivot_row);
        let pivot = aug[col][col].clone();
        for row in (col + 1)..4 {
            if aug[row][col].is_zero() {
                continue;
            }
            let factor = aug[row][col].clone() / pivot.clone();
            for j in col..=4 {
                let correction = aug[col][j].clone() * factor.clone();
                aug[row][j] = aug[row][j].clone() - correction;
            }
        }
    }

    let mut solution = [F::zero(), F::zero(), F::zero(), F::zero()];
    for row in (0..4).rev() {
        let mut rhs_val = aug[row][4].clone();
        for col in (row + 1)..4 {
            rhs_val = rhs_val - aug[row][col].clone() * solution[col].clone();
        }
        if aug[row][row].is_zero() {
            return None;
        }
        solution[row] = rhs_val / aug[row][row].clone();
    }
    Some(solution)
}

fn affine_rank<F: ExactOrderedField>(points: &[[F; 4]]) -> usize {
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
    rank_rows(&centered, 4)
}

fn check_bounded<F: ExactOrderedField>(dual_vertices: &[[F; 4]]) -> Result<(), ExactPolytopeError> {
    let rows: Vec<Vec<F>> = dual_vertices
        .iter()
        .map(|dual| (0..4).map(|i| dual[i].clone()).collect())
        .collect();
    if rank_rows(&rows, 4) < 4 {
        return Err(ExactPolytopeError::Unbounded);
    }

    let f = dual_vertices.len();
    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let normal =
                    cross_product_4d(&dual_vertices[i], &dual_vertices[j], &dual_vertices[k]);
                if normal.iter().all(ExactOrderedField::is_zero) {
                    continue;
                }
                let has_pos = (0..f)
                    .filter(|&idx| idx != i && idx != j && idx != k)
                    .any(|idx| dot4(&dual_vertices[idx], &normal).is_positive());
                let has_neg = (0..f)
                    .filter(|&idx| idx != i && idx != j && idx != k)
                    .any(|idx| dot4(&dual_vertices[idx], &normal).is_negative());
                if !has_pos || !has_neg {
                    return Err(ExactPolytopeError::Unbounded);
                }
            }
        }
    }

    Ok(())
}

fn enumerate_vertices<F: ExactOrderedField>(
    dual_vertices: &[[F; 4]],
) -> Result<(Vec<[F; 4]>, Vec<BTreeSet<usize>>), ExactPolytopeError> {
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
            match gap.sign() {
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
        return Err(ExactPolytopeError::NoVertices);
    }

    Ok((vertices, descriptors))
}

fn check_irredundancy<F: ExactOrderedField>(
    vertices: &[[F; 4]],
    descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> Result<(), ExactPolytopeError> {
    for facet in 0..facet_count {
        let incident: Vec<[F; 4]> = descriptors
            .iter()
            .enumerate()
            .filter(|(_, descriptor)| descriptor.contains(&facet))
            .map(|(idx, _)| vertices[idx].clone())
            .collect();

        if incident.is_empty() || affine_rank(&incident) < 3 {
            return Err(ExactPolytopeError::RedundantFacet(facet));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::dot4;
    use crate::algebraic::field::{cmp_field, ExactOrderedField};
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

    fn assert_exact_polytope_self_consistent<F: ExactOrderedField>(
        polytope: &super::ExactPolytope4D<F>,
    ) {
        let one = F::one();

        for (vertex_idx, vertex) in polytope.vertices().iter().enumerate() {
            let mut incident_count = 0usize;
            for (facet_idx, dual) in polytope.dual_vertices().iter().enumerate() {
                let value = dot4(vertex, dual);
                let relation = cmp_field(&value, &one);
                if polytope.incidence()[vertex_idx][facet_idx] {
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

        for row in 0..polytope.facet_count() {
            assert_eq!(
                polytope.omega_signs()[row][row],
                0,
                "omega diagonal should vanish"
            );
            for col in 0..polytope.facet_count() {
                let expected_adjacency = row != col
                    && (0..polytope.vertices().len()).any(|vertex| {
                        polytope.incidence()[vertex][row] && polytope.incidence()[vertex][col]
                    });
                assert_eq!(
                    polytope.vertex_adjacency()[row][col],
                    expected_adjacency,
                    "facet adjacency should agree with shared exact vertices"
                );
                assert_eq!(
                    polytope.omega_signs()[row][col],
                    -polytope.omega_signs()[col][row],
                    "omega sign matrix should be antisymmetric"
                );
            }
        }
    }

    #[test]
    fn simplex_geometry_matches_rational_control() {
        let exact = exact_simplex().expect("exact simplex");
        let library = known_polytopes::simplex();

        assert_eq!(exact.facet_count(), library.polytope.facet_count());
        assert_eq!(exact.vertices().len(), library.polytope.vertices().len());
        assert!(same_incidence(
            exact.incidence(),
            library.polytope.incidence()
        ));
        assert_exact_polytope_self_consistent(&exact);
    }

    #[test]
    fn hypercube_geometry_matches_rational_control() {
        let exact = exact_hypercube().expect("exact hypercube");
        let library = known_polytopes::hypercube();

        assert_eq!(exact.vertices().len(), library.polytope.vertices().len());
        assert!(same_incidence(
            exact.incidence(),
            library.polytope.incidence()
        ));
        assert_exact_polytope_self_consistent(&exact);
    }

    #[test]
    fn hko_exact_geometry_is_self_consistent_and_detects_known_zero_pairs() {
        let exact = exact_hko_pentagon().expect("exact hko");

        assert_eq!(exact.facet_count(), 10);
        assert_eq!(exact.vertices().len(), 25);
        assert_exact_polytope_self_consistent(&exact);

        for &(i, j) in &[(1usize, 6usize), (3usize, 8usize), (4usize, 9usize)] {
            assert_eq!(
                exact.omega_signs()[i][j],
                0,
                "known exact HKO zero pair ({i}, {j})"
            );
            assert_eq!(
                exact.omega_signs()[j][i],
                0,
                "known exact HKO zero pair ({j}, {i})"
            );
        }
    }

    #[test]
    fn hko_vs_dyadic_combinatorics_stays_diagnostic_only() {
        let exact = exact_hko_pentagon().expect("exact hko");
        let library = known_polytopes::hko_pentagon();

        assert_eq!(exact.facet_count(), library.polytope.facet_count());
        assert_eq!(exact.vertices().len(), library.polytope.vertices().len());
    }
}
