//! Symmetry reduction and transition-pruned permutation DFS for the crosspolytope search.

use crate::checkpoint::{load_checkpoint, save_checkpoint, CandidateSer, Checkpoint};
use crate::kkt::{solve_kkt, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use nalgebra::{DMatrix, Matrix4, Vector4};
use std::collections::HashSet;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::combinations;
use symplectic::geom::polytope::Polytope4D;

/// Search subset sizes `m = 2..=13`.
///
/// The full crosspolytope has 16 facets, but earlier runs already found the
/// best action at `m = 4`; `m = 14..16` was left unattempted because the
/// search cost grows sharply for little expected benefit.
const MAX_SUBSET_SIZE: usize = 13;

pub(crate) type Candidate = (f64, Vec<usize>, Vec<usize>, Vec<f64>);

pub(crate) struct SearchResult {
    pub(crate) iterations: u64,
    pub(crate) best_certified: Candidate,
    pub(crate) best_uncertain: Option<Candidate>,
    pub(crate) elapsed_secs: f64,
    pub(crate) symmetry_group_order: usize,
    pub(crate) search_complete_through_m: usize,
}

fn for_each_transition_allowed_cyclic_permutation(
    elements: &[usize],
    transition_is_allowed: &DMatrix<bool>,
    callback: &mut impl FnMut(&[usize]),
) {
    let m = elements.len();
    if m == 0 {
        return;
    }
    if m == 1 {
        callback(elements);
        return;
    }

    let first = elements[0];
    let rest: Vec<usize> = elements[1..].to_vec();
    let mut perm = Vec::with_capacity(m);
    perm.push(first);
    let mut used = vec![false; rest.len()];

    dfs_transition_allowed(
        &rest,
        &mut used,
        transition_is_allowed,
        first,
        &mut perm,
        m,
        callback,
    );
}

fn dfs_transition_allowed(
    candidates: &[usize],
    used: &mut [bool],
    transition_is_allowed: &DMatrix<bool>,
    first: usize,
    perm: &mut Vec<usize>,
    total: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    let prev = *perm.last().unwrap();

    if perm.len() == total {
        if transition_is_allowed[(prev, first)] {
            callback(perm);
        }
        return;
    }

    for (i, &elem) in candidates.iter().enumerate() {
        if used[i] || !transition_is_allowed[(prev, elem)] {
            continue;
        }
        used[i] = true;
        perm.push(elem);
        dfs_transition_allowed(
            candidates,
            used,
            transition_is_allowed,
            first,
            perm,
            total,
            callback,
        );
        perm.pop();
        used[i] = false;
    }
}

fn all_permutations_4() -> Vec<[usize; 4]> {
    let mut result = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            if b == a {
                continue;
            }
            for c in 0..4 {
                if c == a || c == b {
                    continue;
                }
                let d = 6 - a - b - c;
                result.push([a, b, c, d]);
            }
        }
    }
    result
}

fn compute_symplectic_hyperoctahedral(normals: &[Vector4<f64>]) -> Vec<[usize; 16]> {
    let j0 = Matrix4::new(
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
    );

    let mut facet_perms = Vec::new();
    for perm in all_permutations_4() {
        for sign_bits in 0..16u32 {
            let signs: [f64; 4] = [
                if sign_bits & 8 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 4 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 2 != 0 { -1.0 } else { 1.0 },
                if sign_bits & 1 != 0 { -1.0 } else { 1.0 },
            ];

            let mut m = Matrix4::zeros();
            for j in 0..4 {
                m[(perm[j], j)] = signs[j];
            }

            if (m.transpose() * j0 * m - j0).norm() > 1e-10 {
                continue;
            }

            let mut facet_perm = [0usize; 16];
            for (i, n) in normals.iter().enumerate() {
                let mn = m * n;
                let j = normals
                    .iter()
                    .position(|n2| (mn - n2).norm() < 1e-10)
                    .expect("transformed normal not found");
                facet_perm[i] = j;
            }
            facet_perms.push(facet_perm);
        }
    }

    facet_perms
}

fn canonical_subset(subset: &[usize], group: &[[usize; 16]]) -> Vec<usize> {
    let mut canonical = subset.to_vec();
    canonical.sort();

    for facet_perm in group {
        let mut transformed: Vec<usize> = subset.iter().map(|&i| facet_perm[i]).collect();
        transformed.sort();
        if transformed < canonical {
            canonical = transformed;
        }
    }

    canonical
}

pub(crate) fn run_crosspolytope_search(
    polytope: &Polytope4D,
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> SearchResult {
    let facet_count = polytope.facet_count();

    println!("\nComputing symplectic symmetry group...");
    let group = compute_symplectic_hyperoctahedral(normals);
    println!("Hyperoctahedral group order: 384");
    println!("Symplectic subgroup order:   {}", group.len());
    println!("Expected (by hand):          32");
    if group.len() != 32 {
        println!("WARNING: Rust computation disagrees with analytical result!");
    }

    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    );
    let avg_out_degree: f64 = (0..facet_count)
        .map(|i| {
            (0..facet_count)
                .filter(|&j| transition_is_allowed[(i, j)] && i != j)
                .count() as f64
        })
        .sum::<f64>()
        / facet_count as f64;
    println!(
        "\nDirected transition matrix: avg out-degree = {avg_out_degree:.1} (of {} possible)",
        facet_count - 1
    );

    let checkpoint = load_checkpoint();
    let start_m: usize;
    let mut iterations: u64;
    let mut best_certified: Option<Candidate>;
    let mut best_uncertain: Option<Candidate>;
    let prior_elapsed: f64;

    if let Some(cp) = &checkpoint {
        start_m = cp.completed_m + 1;
        iterations = cp.iterations;
        prior_elapsed = cp.elapsed_secs;
        best_certified = cp.best_certified.as_ref().map(|c| {
            (
                c.action,
                c.subset.clone(),
                c.permutation.clone(),
                c.beta.clone(),
            )
        });
        best_uncertain = cp.best_uncertain.as_ref().map(|c| {
            (
                c.action,
                c.subset.clone(),
                c.permutation.clone(),
                c.beta.clone(),
            )
        });
        println!(
            "\nResuming from checkpoint: m={} done, {} iterations, {:.1}s prior",
            cp.completed_m, iterations, prior_elapsed
        );
    } else {
        start_m = 2;
        iterations = 0;
        prior_elapsed = 0.0;
        best_certified = None;
        best_uncertain = None;
        println!("\nNo checkpoint found, starting from scratch.");
    }

    println!("\n=== Computing capacity ===\n");
    let cap_start = Instant::now();

    let max_m = MAX_SUBSET_SIZE.min(facet_count);
    for m in start_m..=max_m {
        let m_start = Instant::now();
        let all_subsets = combinations(facet_count, m);
        let total_subsets = all_subsets.len();

        let mut seen = HashSet::new();
        let mut canonical_subsets = Vec::new();
        for subset in &all_subsets {
            let canon = canonical_subset(subset, &group);
            if seen.insert(canon.clone()) {
                canonical_subsets.push(canon);
            }
        }

        let canonical_count = canonical_subsets.len();
        let reduction = if canonical_count > 0 {
            total_subsets as f64 / canonical_count as f64
        } else {
            0.0
        };
        print!(
            "m={m:2}: C({facet_count},{m})={total_subsets:6} → {canonical_count:5} canonical ({reduction:.1}x)  ",
        );

        let mut m_iterations = 0u64;
        let mut m_kkt_solutions = 0u64;

        for subset in &canonical_subsets {
            for_each_transition_allowed_cyclic_permutation(
                subset,
                &transition_is_allowed,
                &mut |perm| {
                    iterations += 1;
                    m_iterations += 1;

                    if let Some((beta, q_val)) = solve_kkt(normals, heights, perm) {
                        if q_val <= EPS_Q_POSITIVE {
                            return;
                        }
                        m_kkt_solutions += 1;
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
                                best_uncertain =
                                    Some((action, subset.clone(), perm.to_vec(), beta));
                            }
                        }
                    }
                },
            );
        }

        let m_elapsed = m_start.elapsed().as_secs_f64();
        println!(
            "transition_perms={m_iterations:8}, kkt_solutions={m_kkt_solutions:6}, {m_elapsed:.2}s"
        );

        let total_elapsed = prior_elapsed + cap_start.elapsed().as_secs_f64();
        let cp = Checkpoint {
            completed_m: m,
            iterations,
            elapsed_secs: total_elapsed,
            best_certified: best_certified.as_ref().map(|c| CandidateSer {
                action: c.0,
                subset: c.1.clone(),
                permutation: c.2.clone(),
                beta: c.3.clone(),
            }),
            best_uncertain: best_uncertain.as_ref().map(|c| CandidateSer {
                action: c.0,
                subset: c.1.clone(),
                permutation: c.2.clone(),
                beta: c.3.clone(),
            }),
        };
        save_checkpoint(&cp);
    }

    SearchResult {
        iterations,
        best_certified: best_certified
            .unwrap_or_else(|| panic!("no certified (S,σ) found through subset size m = {max_m}")),
        best_uncertain,
        elapsed_secs: prior_elapsed + cap_start.elapsed().as_secs_f64(),
        symmetry_group_order: group.len(),
        search_complete_through_m: max_m,
    }
}
