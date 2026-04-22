use crate::kkt::{
    omega0, solve_kkt_full, EPS_BETA_POSITIVE, EPS_DIRECTED, EPS_FACET_INCIDENCE, EPS_Q_POSITIVE,
};
use crate::models::{AblationCapacityResult, AblationResult, Variant};
use nalgebra::{DMatrix, Vector4};
use symplectic::ehz_capacity_unpruned;
use symplectic::geom::polytope::Polytope4D;

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.clone());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

fn for_each_cyclic_permutation(elements: &[usize], callback: &mut impl FnMut(&[usize])) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }
    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    heap_perms_buf(&mut buf, 1, k, callback);
}

fn heap_perms_buf(buf: &mut [usize], offset: usize, k: usize, callback: &mut impl FnMut(&[usize])) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_perms_buf(buf, offset, k - 1, callback);
    for i in 0..k - 1 {
        if k.is_multiple_of(2) {
            buf.swap(offset + i, offset + k - 1);
        } else {
            buf.swap(offset, offset + k - 1);
        }
        heap_perms_buf(buf, offset, k - 1, callback);
    }
}

fn dmatrix_to_vec(adj: &DMatrix<bool>) -> Vec<Vec<bool>> {
    let f = adj.nrows();
    (0..f)
        .map(|i| (0..f).map(|j| adj[(i, j)]).collect())
        .collect()
}

fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

fn build_directed_adjacency(
    vertex_adj: &DMatrix<bool>,
    normals: &[Vector4<f64>],
) -> Vec<Vec<bool>> {
    let f = normals.len();
    let mut dir_adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            if i == j {
                continue;
            }
            if vertex_adj[(i, j)] {
                dir_adj[i][j] = omega0(&normals[i], &normals[j]) >= -EPS_DIRECTED;
            }
        }
    }
    dir_adj
}

type Candidate = (f64, Vec<usize>, Vec<usize>, Vec<f64>);

fn ehz_capacity_unpruned_with(
    polytope: &Polytope4D,
    adj: &[Vec<bool>],
    solver: fn(&[Vector4<f64>], &[f64], &[usize]) -> Option<(Vec<f64>, f64)>,
) -> Option<AblationResult> {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();

    let mut best_certified: Option<Candidate> = None;
    let mut best_uncertain: Option<Candidate> = None;
    let mut iterations: u64 = 0;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_adjacent_cycle(perm, adj) {
                    return;
                }
                iterations += 1;

                if let Some((beta, q_val)) = solver(&normals, &heights, perm) {
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                    let action = 0.5 / q_val;

                    if beta_min > EPS_BETA_POSITIVE {
                        let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_certified =
                                Some((action, subset.clone(), perm.to_vec(), beta.clone()));
                        }
                    }

                    if beta_min > -EPS_BETA_POSITIVE {
                        let update = best_uncertain.as_ref().is_none_or(|b| action < b.0);
                        if update {
                            best_uncertain = Some((action, subset.clone(), perm.to_vec(), beta));
                        }
                    }
                }
            });
        }
    }

    let certified = best_certified?;
    let uncertain_cap = best_uncertain.map_or(certified.0, |b| b.0);
    Some(AblationResult {
        result: AblationCapacityResult {
            capacity: certified.0,
            capacity_uncertain: uncertain_cap,
            best_permutation: certified.2,
            best_beta: certified.3,
            iterations,
        },
        best_subset: certified.1,
    })
}

fn ehz_capacity_unpruned_a0(polytope: &Polytope4D) -> Option<AblationResult> {
    ehz_capacity_unpruned(polytope)
        .ok()
        .map(|result| AblationResult {
            result: AblationCapacityResult {
                capacity: result.capacity(),
                capacity_uncertain: result
                    .orbits
                    .iter()
                    .map(|orbit| orbit.action)
                    .fold(f64::INFINITY, f64::min),
                best_permutation: result.best_sigma().to_vec(),
                best_beta: result.best_beta().to_vec(),
                iterations: result.iterations,
            },
            best_subset: result.best_subset(),
        })
}

fn ehz_capacity_unpruned_a1(polytope: &Polytope4D) -> Option<AblationResult> {
    let vertex_adj = dmatrix_to_vec(polytope.vertex_adjacency());
    ehz_capacity_unpruned_with(polytope, &vertex_adj, solve_kkt_full)
}

fn ehz_capacity_unpruned_a2(polytope: &Polytope4D) -> Option<AblationResult> {
    let vertex_adj = polytope.vertex_adjacency();
    let normals: Vec<Vector4<f64>> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| a / a.norm())
        .collect();
    let dir_adj = build_directed_adjacency(vertex_adj, &normals);
    ehz_capacity_unpruned_with(polytope, &dir_adj, solve_kkt_full)
}

fn fourier_motzkin_2d_feasible(constraints: &[(f64, f64, f64)]) -> bool {
    let eps = 1e-15;

    let mut upper_t: Vec<(f64, f64)> = Vec::new();
    let mut lower_t: Vec<(f64, f64)> = Vec::new();
    let mut s_bounds: Vec<(f64, f64)> = Vec::new();

    for &(a, b, c) in constraints {
        if b.abs() < eps {
            s_bounds.push((a, c));
        } else if b > 0.0 {
            upper_t.push((-a / b, c / b));
        } else {
            lower_t.push((-a / b, c / b));
        }
    }

    for &(sl, il) in &lower_t {
        for &(su, iu) in &upper_t {
            s_bounds.push((sl - su, iu - il));
        }
    }

    let mut s_lo = f64::NEG_INFINITY;
    let mut s_hi = f64::INFINITY;

    for &(coeff, rhs) in &s_bounds {
        if coeff.abs() < eps {
            if rhs < -eps {
                return false;
            }
        } else if coeff > 0.0 {
            s_hi = s_hi.min(rhs / coeff);
        } else {
            s_lo = s_lo.max(rhs / coeff);
        }
    }

    s_lo <= s_hi + eps
}

fn is_physical_transition_feasible(
    normals: &[Vector4<f64>],
    heights: &[f64],
    src: usize,
    dst: usize,
) -> bool {
    let f = normals.len();

    let blocking: Vec<bool> = (0..f)
        .map(|k| {
            if k == src || k == dst {
                return false;
            }
            let omega_src_k = omega0(&normals[src], &normals[k]);
            let omega_dst_k = omega0(&normals[dst], &normals[k]);
            omega_src_k < 0.0 || omega_dst_k > 0.0
        })
        .collect();

    if !blocking.iter().any(|&b| b) {
        return true;
    }

    let n_src = &normals[src];
    let n_dst = &normals[dst];

    let a_mat = DMatrix::from_row_slice(
        2,
        4,
        &[
            n_src[0], n_src[1], n_src[2], n_src[3], n_dst[0], n_dst[1], n_dst[2], n_dst[3],
        ],
    );
    let b_vec = nalgebra::DVector::from_row_slice(&[heights[src], heights[dst]]);
    let aat = &a_mat * a_mat.transpose();
    let aat_inv = match aat.try_inverse() {
        Some(inv) => inv,
        None => return false,
    };
    let lambda = &aat_inv * &b_vec;
    let x0_dv = a_mat.transpose() * lambda;
    let x0 = Vector4::new(x0_dv[0], x0_dv[1], x0_dv[2], x0_dv[3]);

    let ata = a_mat.transpose() * &a_mat;
    let eigen = ata.symmetric_eigen();
    let mut null_vecs: Vec<Vector4<f64>> = Vec::new();
    for col in 0..4 {
        if eigen.eigenvalues[col].abs() < 1e-10 {
            null_vecs.push(Vector4::new(
                eigen.eigenvectors[(0, col)],
                eigen.eigenvectors[(1, col)],
                eigen.eigenvectors[(2, col)],
                eigen.eigenvectors[(3, col)],
            ));
        }
    }
    if null_vecs.len() < 2 {
        return false;
    }
    let u1 = null_vecs[0];
    let u2 = null_vecs[1];

    let delta = EPS_FACET_INCIDENCE;
    let mut constraints: Vec<(f64, f64, f64)> = Vec::new();
    for k in 0..f {
        if k == src || k == dst {
            continue;
        }
        let a_k = normals[k].dot(&u1);
        let b_k = normals[k].dot(&u2);
        let slack = heights[k] - normals[k].dot(&x0);
        let c_k = if blocking[k] { slack - delta } else { slack };
        constraints.push((a_k, b_k, c_k));
    }

    fourier_motzkin_2d_feasible(&constraints)
}

fn build_a3_adjacency(
    a2_adj: &[Vec<bool>],
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Vec<Vec<bool>> {
    let f = normals.len();
    let mut a3_adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            if i == j || !a2_adj[i][j] {
                continue;
            }
            a3_adj[i][j] = is_physical_transition_feasible(normals, heights, i, j);
        }
    }
    a3_adj
}

fn ehz_capacity_unpruned_a3(polytope: &Polytope4D) -> Option<AblationResult> {
    let vertex_adj = polytope.vertex_adjacency();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let a2_adj = build_directed_adjacency(vertex_adj, &normals);
    let a3_adj = build_a3_adjacency(&a2_adj, &normals, &heights);
    ehz_capacity_unpruned_with(polytope, &a3_adj, solve_kkt_full)
}

pub const VARIANTS: &[Variant] = &[
    Variant {
        name: "a0_unpruned",
        run: ehz_capacity_unpruned_a0,
    },
    Variant {
        name: "a1_vertex_adj",
        run: ehz_capacity_unpruned_a1,
    },
    Variant {
        name: "a2_omega_directed",
        run: ehz_capacity_unpruned_a2,
    },
    Variant {
        name: "a3_reeb_feasible",
        run: ehz_capacity_unpruned_a3,
    },
];
