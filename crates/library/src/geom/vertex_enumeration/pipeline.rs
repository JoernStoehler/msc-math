//! Pipeline orchestration and output assembly.

use num_rational::BigRational;
use num_traits::Zero;
use std::collections::BTreeSet;

use crate::geom::polytope::ConstructionError;

use super::boundedness::{check_bounded_f64_first, integer_scale_dual_vertices};
use super::enumerate::enumerate_vertices_int;
use super::linear_algebra::affine_rank_rational;

fn check_irredundancy_f64_first(
    vertices: &[[BigRational; 4]],
    vertex_descriptors: &[BTreeSet<usize>],
    facet_count: usize,
) -> Result<(), ConstructionError> {
    use crate::geom::rational_arithmetic::rational_to_f64;

    for i in 0..facet_count {
        let incident_indices: Vec<usize> = vertex_descriptors
            .iter()
            .enumerate()
            .filter(|(_, vd)| vd.contains(&i))
            .map(|(idx, _)| idx)
            .collect();

        if incident_indices.is_empty() {
            return Err(ConstructionError::RedundantFacet(i));
        }

        if incident_indices.len() >= 4 {
            let inc_f64: Vec<[f64; 4]> = incident_indices
                .iter()
                .map(|&idx| std::array::from_fn(|c| rational_to_f64(&vertices[idx][c])))
                .collect();
            const EPS_RANK_F64: f64 = 1e-10;
            let mut rank_ok = false;
            'outer: for base_idx in 0..inc_f64.len() {
                let base = &inc_f64[base_idx];
                let others: Vec<usize> = (0..inc_f64.len()).filter(|&j| j != base_idx).collect();
                for a in 0..others.len() {
                    for b in (a + 1)..others.len() {
                        for c in (b + 1)..others.len() {
                            let rows: [[f64; 4]; 3] = [
                                std::array::from_fn(|d| inc_f64[others[a]][d] - base[d]),
                                std::array::from_fn(|d| inc_f64[others[b]][d] - base[d]),
                                std::array::from_fn(|d| inc_f64[others[c]][d] - base[d]),
                            ];
                            for skip_col in 0..4 {
                                let cols: Vec<usize> = (0..4).filter(|&d| d != skip_col).collect();
                                let det = rows[0][cols[0]]
                                    * (rows[1][cols[1]] * rows[2][cols[2]]
                                        - rows[1][cols[2]] * rows[2][cols[1]])
                                    - rows[0][cols[1]]
                                        * (rows[1][cols[0]] * rows[2][cols[2]]
                                            - rows[1][cols[2]] * rows[2][cols[0]])
                                    + rows[0][cols[2]]
                                        * (rows[1][cols[0]] * rows[2][cols[1]]
                                            - rows[1][cols[1]] * rows[2][cols[0]]);
                                if det.abs() > EPS_RANK_F64 {
                                    rank_ok = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
            if rank_ok {
                continue;
            }
        }

        let incident: Vec<[BigRational; 4]> = incident_indices
            .iter()
            .map(|&idx| vertices[idx].clone())
            .collect();
        if affine_rank_rational(&incident) < 3 {
            return Err(ConstructionError::RedundantFacet(i));
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
pub(in crate::geom) fn construct_rational_pipeline(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<[BigRational; 4]>, Vec<BTreeSet<usize>>), ConstructionError> {
    let f = dual_vertices.len();

    // Stage 1: input normalization / validation.
    if f < 5 {
        return Err(ConstructionError::TooFewFacets(f));
    }
    for (i, y) in dual_vertices.iter().enumerate() {
        if y.iter().all(|c| c.is_zero()) {
            return Err(ConstructionError::ZeroDualVertex(i));
        }
    }

    // Stage 2: exact integer normalization for downstream checks.
    let (int_dual_vertices, common_denom) = integer_scale_dual_vertices(dual_vertices);

    // Stage 3: prefilter + exact candidate enumeration.
    let (vertex_descriptors, vertices) =
        enumerate_vertices_int(dual_vertices, &int_dual_vertices, &common_denom)?;

    // Stage 4: irredundancy/boundedness checks.
    check_irredundancy_f64_first(&vertices, &vertex_descriptors, f)?;
    check_bounded_f64_first(dual_vertices, &int_dual_vertices)?;

    // Stage 5: output assembly.

    Ok((vertices, vertex_descriptors))
}
