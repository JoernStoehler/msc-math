//! Irredundancy checks for facet constraints.
//!
//! Mathematical correspondence: [lem:irredundancy].

use std::collections::BTreeSet;

use num_rational::BigRational;

use super::exact_linalg::rank_over_q;
use super::ConstructionError;

/// Compute affine rank of rational points in R^4.
pub(super) fn affine_rank_rational(points: &[[BigRational; 4]]) -> usize {
    if points.len() <= 1 {
        return 0;
    }

    let base = &points[0];
    let centered: Vec<[BigRational; 4]> = points[1..]
        .iter()
        .map(|p| std::array::from_fn(|i| &p[i] - &base[i]))
        .collect();

    rank_over_q(&centered)
}

/// Check irredundancy: every facet has incident vertices of affine rank >= 3.
pub(super) fn check_irredundancy_f64_first(
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
