//! Exact 4D polytope construction over ordered scalar fields.
//!
//! This is the exact-field analogue of `geom::polytope::Polytope4D`, kept on an
//! expert surface because it is intended for selected exact calculations rather
//! than the ordinary library routing path.
//!
//! TODO: add [def:...] to formal math for the exact dual-vertex polytope model.
//! TODO: add [lem:...] to formal math for the exact 4D vertex enumeration and
//! irredundancy checks implemented in this module.

use algebraic_numbers::{rank, solve_linear_system, ExactScalar, LinearSystemSolution};
use nalgebra::{DMatrix, DVector};
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

/// Exact 4D polytope with exact combinatorics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPolytope4D<F: ExactScalar + 'static> {
    dual_vertices: Vec<[F; 4]>,
    vertices: Vec<[F; 4]>,
    incidence: Vec<Vec<bool>>,
    vertex_adjacency: Vec<Vec<bool>>,
    omega_signs: Vec<Vec<i8>>,
}

impl<F: ExactScalar + 'static> ExactPolytope4D<F> {
    /// Construct from exact dual vertices `a_i` for `a_i . x <= 1`.
    pub fn new(dual_vertices: Vec<[F; 4]>) -> Result<Self, ExactPolytopeError> {
        let f = dual_vertices.len();
        if f < 5 {
            return Err(ExactPolytopeError::TooFewFacets(f));
        }
        for (idx, dual) in dual_vertices.iter().enumerate() {
            if dual.iter().all(|entry| entry.is_zero()) {
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
                            match omega0(&dual_vertices[i], &dual_vertices[j]).cmp(&F::zero()) {
                                Ordering::Less => -1,
                                Ordering::Equal => 0,
                                Ordering::Greater => 1,
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

    pub fn dual_vertices(&self) -> &[[F; 4]] {
        &self.dual_vertices
    }

    pub fn vertices(&self) -> &[[F; 4]] {
        &self.vertices
    }

    pub fn incidence(&self) -> &[Vec<bool>] {
        &self.incidence
    }

    pub fn vertex_adjacency(&self) -> &[Vec<bool>] {
        &self.vertex_adjacency
    }

    pub fn omega_signs(&self) -> &[Vec<i8>] {
        &self.omega_signs
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }
}

/// Exact 4D dot product.
pub fn dot4<F: ExactScalar>(left: &[F; 4], right: &[F; 4]) -> F {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

/// Exact standard symplectic form `omega_0`.
pub fn omega0<F: ExactScalar>(u: &[F; 4], v: &[F; 4]) -> F {
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

fn cross_product_4d<F: ExactScalar>(a: &[F; 4], b: &[F; 4], c: &[F; 4]) -> [F; 4] {
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

fn solve4<F: ExactScalar + 'static>(rows: &[[F; 4]; 4], rhs: &[F; 4]) -> Option<[F; 4]> {
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

fn affine_rank<F: ExactScalar + 'static>(points: &[[F; 4]]) -> usize {
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
    dual_vertices: &[[F; 4]],
) -> Result<(), ExactPolytopeError> {
    let f = dual_vertices.len();
    for combo in combinations4(f) {
        let normal = cross_product_4d(
            &dual_vertices[combo[1]],
            &dual_vertices[combo[2]],
            &dual_vertices[combo[3]],
        );
        for sign in [F::one(), -F::one()] {
            let candidate = std::array::from_fn(|idx| sign.clone() * normal[idx].clone());
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
    dual_vertices: &[[F; 4]],
) -> Result<(Vec<[F; 4]>, VertexDescriptors), ExactPolytopeError> {
    let mut vertices = Vec::new();
    let mut descriptors = Vec::new();

    for combo in combinations4(dual_vertices.len()) {
        let rows = combo.map(|idx| dual_vertices[idx].clone());
        let rhs = std::array::from_fn(|_| F::one());
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
    use super::{dot4, ExactPolytope4D};
    use algebraic_numbers::{Algebraic, ExactScalar, RealAlgebraicField};
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

    fn exact_simplex() -> ExactPolytope4D<BigRational> {
        let z = BigRational::zero();
        ExactPolytope4D::new(vec![
            [q(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), q(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), q(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), q(-5)],
            [q(5), q(5), q(5), q(5)],
        ])
        .expect("exact simplex")
    }

    fn exact_hko() -> ExactPolytope4D<TanPiFifthField> {
        let z = TanPiFifthField::zero();
        let one = TanPiFifthField::one();
        let t = TanPiFifthField::root();
        let t2 = t.clone() * t.clone();
        let t3 = t2.clone() * t.clone();
        let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from(4);
        let b = (TanPiFifthField::from(7) * t.clone() - t3.clone()) / TanPiFifthField::from(4);
        let sec36 = (TanPiFifthField::from(3) - t2.clone()) / TanPiFifthField::from(2);

        ExactPolytope4D::new(vec![
            [one.clone(), t.clone(), z.clone(), z.clone()],
            [-a.clone(), b.clone(), z.clone(), z.clone()],
            [-sec36.clone(), z.clone(), z.clone(), z.clone()],
            [-a.clone(), -b.clone(), z.clone(), z.clone()],
            [one.clone(), -t.clone(), z.clone(), z.clone()],
            [z.clone(), z.clone(), t.clone(), -one.clone()],
            [z.clone(), z.clone(), b.clone(), a.clone()],
            [z.clone(), z.clone(), z.clone(), sec36.clone()],
            [z.clone(), z.clone(), -b, a],
            [z.clone(), z.clone(), -t, -one],
        ])
        .expect("exact HKO")
    }

    fn assert_self_consistent<F: ExactScalar + 'static>(polytope: &ExactPolytope4D<F>) {
        let one = F::one();
        for (vertex_idx, vertex) in polytope.vertices().iter().enumerate() {
            for (facet_idx, dual) in polytope.dual_vertices().iter().enumerate() {
                let relation = dot4(vertex, dual).cmp(&one);
                if polytope.incidence()[vertex_idx][facet_idx] {
                    assert_eq!(relation, Ordering::Equal);
                } else {
                    assert_eq!(relation, Ordering::Less);
                }
            }
        }
    }

    #[test]
    fn simplex_is_self_consistent() {
        let polytope = exact_simplex();
        assert_eq!(polytope.facet_count(), 5);
        assert_eq!(polytope.vertices().len(), 5);
        assert_self_consistent(&polytope);
    }

    #[test]
    fn hko_detects_known_exact_zero_pairs() {
        let polytope = exact_hko();
        assert_eq!(polytope.facet_count(), 10);
        assert_eq!(polytope.vertices().len(), 25);
        assert_self_consistent(&polytope);
        for &(i, j) in &[(1usize, 6usize), (3usize, 8usize), (4usize, 9usize)] {
            assert_eq!(polytope.omega_signs()[i][j], 0);
            assert_eq!(polytope.omega_signs()[j][i], 0);
        }
    }
}
