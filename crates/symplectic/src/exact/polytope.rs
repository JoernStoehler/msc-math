//! Flat exact 4D dual-vertex polytope helpers over ordered scalar fields.
//!
//! TODO: add [def:...] to formal math for the exact dual-vertex polytope model.
//! TODO: add [lem:...] to formal math for the exact 4D vertex enumeration and
//! irredundancy checks implemented in this module.

use algebraic_numbers::{rank, solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector, Vector4};
use std::cmp::Ordering;
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

/// Exact vertices of `{x : <a_i, x> <= 1}` with vertex-facet incidence.
///
/// The rows of `vertex_facet_incidence` are indexed by `vertices`; columns are
/// indexed by the input `dual_vertices`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactVerticesWithIncidence<F: ExactScalar + 'static> {
    pub vertices: Vec<Vector4<F>>,
    pub vertex_facet_incidence: DMatrix<bool>,
}

/// Enumerate exact vertices and incidence for exact dual vertices `a_i`.
///
/// Input contract:
/// - `dual_vertices` are interpreted as inequalities `<a_i, x> <= 1`;
/// - all facets are expected to be nonzero and irredundant;
/// - the resulting 4D polytope is expected to be bounded.
///
/// Checked here:
/// - at least five facets;
/// - no zero dual vertex;
/// - boundedness by exact cone search;
/// - existence of at least one vertex;
/// - every input facet has affine-rank-three incident vertices.
pub fn exact_vertices_with_incidence<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
) -> Result<ExactVerticesWithIncidence<F>, ExactPolytopeError> {
    let f = dual_vertices.len();
    if f < 5 {
        return Err(ExactPolytopeError::TooFewFacets(f));
    }
    for (idx, dual) in dual_vertices.iter().enumerate() {
        if dual.iter().all(|entry| entry.is_zero()) {
            return Err(ExactPolytopeError::ZeroDualVertex(idx));
        }
    }

    check_bounded(dual_vertices)?;
    let (vertices, descriptors) = enumerate_vertices(dual_vertices)?;
    check_irredundancy(&vertices, &descriptors, f)?;

    let vertex_facet_incidence = DMatrix::from_fn(vertices.len(), f, |row, col| {
        descriptors[row].contains(&col)
    });

    Ok(ExactVerticesWithIncidence {
        vertices,
        vertex_facet_incidence,
    })
}

/// Exact 4D dot product.
pub fn dot4<F: ExactScalar>(left: &Vector4<F>, right: &Vector4<F>) -> F {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

/// Exact standard symplectic form `omega_0`.
pub fn omega0<F: ExactScalar>(u: &Vector4<F>, v: &Vector4<F>) -> F {
    u[0].clone() * v[2].clone() - u[2].clone() * v[0].clone() + u[1].clone() * v[3].clone()
        - u[3].clone() * v[1].clone()
}

/// Exact sign matrix for the standard symplectic form on exact dual vertices.
///
/// Rows and columns are indexed by `dual_vertices`; each entry is `-1`, `0`, or
/// `1` according to the exact sign of `omega0(dual_vertices[i], dual_vertices[j])`.
pub fn omega_signs_exact<F: ExactScalar>(dual_vertices: &[Vector4<F>]) -> DMatrix<i8> {
    DMatrix::from_fn(dual_vertices.len(), dual_vertices.len(), |i, j| {
        if i == j {
            0
        } else {
            match omega0(&dual_vertices[i], &dual_vertices[j]).cmp(&F::zero()) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
    })
}

/// Exact nonempty facet-intersection matrix from vertex-facet incidence.
///
/// Rows and columns are facet indices. The diagonal is false.
pub fn facet_intersection_is_nonempty_exact(
    vertex_facet_incidence: &DMatrix<bool>,
) -> DMatrix<bool> {
    DMatrix::from_fn(
        vertex_facet_incidence.ncols(),
        vertex_facet_incidence.ncols(),
        |i, j| {
            i != j
                && (0..vertex_facet_incidence.nrows())
                    .any(|v| vertex_facet_incidence[(v, i)] && vertex_facet_incidence[(v, j)])
        },
    )
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

fn cross_product_4d<F: ExactScalar>(a: &Vector4<F>, b: &Vector4<F>, c: &Vector4<F>) -> Vector4<F> {
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
    Vector4::new(d0, d1, d2, d3)
}

fn solve4<F: ExactScalar + 'static>(
    rows: &[Vector4<F>; 4],
    rhs: &Vector4<F>,
) -> Option<Vector4<F>> {
    let matrix = DMatrix::from_fn(4, 4, |row, col| rows[row][col].clone());
    let rhs = DVector::from_fn(4, |row, _| rhs[row].clone());
    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => Some(Vector4::new(
            particular[0].clone(),
            particular[1].clone(),
            particular[2].clone(),
            particular[3].clone(),
        )),
        _ => None,
    }
}

fn affine_rank<F: ExactScalar + 'static>(points: &[Vector4<F>]) -> usize {
    if points.len() <= 1 {
        return 0;
    }
    let origin = &points[0];
    let rows: Vec<Vec<F>> = points[1..]
        .iter()
        .map(|point| {
            (0..4)
                .map(|i| point[i].clone() - origin[i].clone())
                .collect()
        })
        .collect();
    let matrix = DMatrix::from_fn(rows.len(), 4, |row, col| rows[row][col].clone());
    rank(&matrix)
}

fn check_bounded<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
) -> Result<(), ExactPolytopeError> {
    let f = dual_vertices.len();
    for combo in combinations4(f) {
        let normal = cross_product_4d(
            &dual_vertices[combo[1]],
            &dual_vertices[combo[2]],
            &dual_vertices[combo[3]],
        );
        for sign in [F::one(), -F::one()] {
            let candidate = Vector4::new(
                sign.clone() * normal[0].clone(),
                sign.clone() * normal[1].clone(),
                sign.clone() * normal[2].clone(),
                sign.clone() * normal[3].clone(),
            );
            if candidate.iter().all(|entry| entry.is_zero()) {
                continue;
            }
            if dual_vertices
                .iter()
                .all(|dual| dot4(dual, &candidate) <= F::zero())
            {
                return Err(ExactPolytopeError::Unbounded);
            }
        }
    }
    Ok(())
}

type VertexDescriptors = Vec<BTreeSet<usize>>;

fn enumerate_vertices<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
) -> Result<(Vec<Vector4<F>>, VertexDescriptors), ExactPolytopeError> {
    let mut vertices = Vec::new();
    let mut descriptors = Vec::new();

    for combo in combinations4(dual_vertices.len()) {
        let rows = combo.map(|idx| dual_vertices[idx].clone());
        let rhs = Vector4::new(F::one(), F::one(), F::one(), F::one());
        let Some(vertex) = solve4(&rows, &rhs) else {
            continue;
        };

        let mut descriptor = BTreeSet::new();
        let mut feasible = true;
        for (facet_idx, dual) in dual_vertices.iter().enumerate() {
            match dot4(&vertex, dual).cmp(&F::one()) {
                std::cmp::Ordering::Greater => {
                    feasible = false;
                    break;
                }
                std::cmp::Ordering::Equal => {
                    descriptor.insert(facet_idx);
                }
                std::cmp::Ordering::Less => {}
            }
        }

        if feasible
            && !descriptor.is_empty()
            && !descriptors.iter().any(|known| known == &descriptor)
        {
            vertices.push(vertex);
            descriptors.push(descriptor);
        }
    }

    if vertices.is_empty() {
        return Err(ExactPolytopeError::NoVertices);
    }

    Ok((vertices, descriptors))
}

fn check_irredundancy<F: ExactScalar + 'static>(
    vertices: &[Vector4<F>],
    descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> Result<(), ExactPolytopeError> {
    for facet in 0..facet_count {
        let incident: Vec<Vector4<F>> = descriptors
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
    use super::{
        dot4, exact_vertices_with_incidence, facet_intersection_is_nonempty_exact,
        omega_signs_exact,
    };
    use algebraic_numbers::{Algebraic, ExactScalar, RealAlgebraicField};
    use nalgebra::{DMatrix, Vector4};
    use num_rational::BigRational;
    use num_traits::{One, Zero};
    use std::cmp::Ordering;

    enum TanPiFifth {}

    impl RealAlgebraicField for TanPiFifth {
        fn polynomial() -> Vec<BigRational> {
            vec![q(5), q(0), q(-10), q(0), q(1)]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (q(0), q(1))
        }
    }

    type TanPiFifthField = Algebraic<TanPiFifth>;

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    fn exact_simplex_dual_vertices() -> Vec<Vector4<BigRational>> {
        let z = BigRational::zero();
        vec![
            Vector4::new(q(-5), z.clone(), z.clone(), z.clone()),
            Vector4::new(z.clone(), q(-5), z.clone(), z.clone()),
            Vector4::new(z.clone(), z.clone(), q(-5), z.clone()),
            Vector4::new(z.clone(), z.clone(), z.clone(), q(-5)),
            Vector4::new(q(5), q(5), q(5), q(5)),
        ]
    }

    fn exact_hko_dual_vertices() -> Vec<Vector4<TanPiFifthField>> {
        let z = TanPiFifthField::zero();
        let one = TanPiFifthField::one();
        let t = TanPiFifthField::root();
        let t2 = t.clone() * t.clone();
        let t3 = t2.clone() * t.clone();
        let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from(4);
        let b = (TanPiFifthField::from(7) * t.clone() - t3.clone()) / TanPiFifthField::from(4);
        let sec36 = (TanPiFifthField::from(3) - t2.clone()) / TanPiFifthField::from(2);

        vec![
            Vector4::new(one.clone(), t.clone(), z.clone(), z.clone()),
            Vector4::new(-a.clone(), b.clone(), z.clone(), z.clone()),
            Vector4::new(-sec36.clone(), z.clone(), z.clone(), z.clone()),
            Vector4::new(-a.clone(), -b.clone(), z.clone(), z.clone()),
            Vector4::new(one.clone(), -t.clone(), z.clone(), z.clone()),
            Vector4::new(z.clone(), z.clone(), t.clone(), -one.clone()),
            Vector4::new(z.clone(), z.clone(), b.clone(), a.clone()),
            Vector4::new(z.clone(), z.clone(), z.clone(), sec36.clone()),
            Vector4::new(z.clone(), z.clone(), -b, a),
            Vector4::new(z.clone(), z.clone(), -t, -one),
        ]
    }

    fn assert_self_consistent<F: ExactScalar + 'static>(
        dual_vertices: &[Vector4<F>],
        vertices: &[Vector4<F>],
        vertex_facet_incidence: &DMatrix<bool>,
    ) {
        let one = F::one();
        for (vertex_idx, vertex) in vertices.iter().enumerate() {
            for (facet_idx, dual) in dual_vertices.iter().enumerate() {
                let relation = dot4(vertex, dual).cmp(&one);
                if vertex_facet_incidence[(vertex_idx, facet_idx)] {
                    assert_eq!(relation, Ordering::Equal);
                } else {
                    assert_eq!(relation, Ordering::Less);
                }
            }
        }
    }

    #[test]
    fn simplex_is_self_consistent() {
        let dual_vertices = exact_simplex_dual_vertices();
        let exact = exact_vertices_with_incidence(&dual_vertices).expect("exact simplex");
        assert_eq!(dual_vertices.len(), 5);
        assert_eq!(exact.vertices.len(), 5);
        assert_self_consistent(
            &dual_vertices,
            &exact.vertices,
            &exact.vertex_facet_incidence,
        );
        let facet_intersections =
            facet_intersection_is_nonempty_exact(&exact.vertex_facet_incidence);
        assert_eq!(facet_intersections.nrows(), 5);
        assert_eq!(facet_intersections.ncols(), 5);
        assert!(!facet_intersections[(0, 0)]);
    }

    #[test]
    fn hko_detects_known_exact_zero_pairs() {
        let dual_vertices = exact_hko_dual_vertices();
        let exact = exact_vertices_with_incidence(&dual_vertices).expect("exact HKO");
        assert_eq!(dual_vertices.len(), 10);
        assert_eq!(exact.vertices.len(), 25);
        assert_self_consistent(
            &dual_vertices,
            &exact.vertices,
            &exact.vertex_facet_incidence,
        );
        let omega_signs = omega_signs_exact(&dual_vertices);
        for &(i, j) in &[(1usize, 6usize), (3usize, 8usize), (4usize, 9usize)] {
            assert_eq!(omega_signs[(i, j)], 0);
            assert_eq!(omega_signs[(j, i)], 0);
        }
    }
}
