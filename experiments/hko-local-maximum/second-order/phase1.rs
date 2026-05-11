//! Phase 1 of the HKO second-order experiment: gradients, SVD, and flat directions.

use crate::flat_polytope::HkoPolytopeCache;
use crate::{NEAR_OPTIMAL_GAP, SVD_RANK_THRESHOLD};
use exp_hko_local_maximum::ehz_capacity_instrumented;
use exp_hko_local_maximum::euclidean_volume_f64;
use nalgebra::{DMatrix, Vector4};
use serde::Serialize;
use symplectic::algorithms::OrbitKktData;
use symplectic::derivatives::{capacity_derivatives_a_from_orbit, volume_derivatives_a};

#[derive(Debug, Serialize)]
pub(crate) struct BaseRow {
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    pub(crate) sys_base: f64,
    capacity_base: f64,
    pub(crate) volume_base: f64,
    n_orbits_total: usize,
    n_near_optimal: usize,
    singular_values: Vec<f64>,
    rank: usize,
    n_flat_directions: usize,
    flat_directions: Vec<Vec<[f64; 4]>>,
    gradient_matrix: Vec<Vec<[f64; 4]>>,
    pub(crate) time_phase1_ms: f64,
}

/// Compute ∇_{a_i} sys for a single orbit, returned as Vec<Vector4>.
fn orbit_sys_gradient_a(
    polytope: &HkoPolytopeCache,
    orbit: &OrbitKktData,
    vol: f64,
    cap: f64,
    sys: f64,
    d_vol_a: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    let d_cap_a = capacity_derivatives_a_from_orbit(&polytope.dual_vertices_f64, orbit)
        .expect("second-order stores orbit payloads with closure multipliers");

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

fn flatten_gradient(grad: &[Vector4<f64>]) -> Vec<f64> {
    grad.iter().flat_map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn unflatten_to_arrays(flat: &[f64]) -> Vec<[f64; 4]> {
    flat.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]).collect()
}

pub(crate) fn run_phase1(polytope: &HkoPolytopeCache) -> (BaseRow, Vec<Vec<f64>>) {
    let facet_count = polytope.facet_count();
    let dim = facet_count * 4;

    let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    let instr = ehz_capacity_instrumented(
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .expect("no valid orbits");
    let cap = instr.capacity;
    let sys = cap * cap / (2.0 * vol);

    println!("  Base: sys={sys:.10}, c={cap:.10}, vol={vol:.10}");
    println!("  Total valid orbits: {}", instr.orbits.len());

    let best_action = instr.orbits[0].action;
    let near_optimal: Vec<&OrbitKktData> = instr
        .orbits
        .iter()
        .filter(|o| {
            let gap = (o.action - best_action) / best_action;
            gap < NEAR_OPTIMAL_GAP
        })
        .collect();
    println!(
        "  Near-optimal (gap < {NEAR_OPTIMAL_GAP:.0e}): {} orbits",
        near_optimal.len()
    );
    if let Some(worst) = near_optimal.last() {
        let gap = (worst.action - best_action) / best_action;
        println!("  Worst near-optimal gap: {gap:.2e}");
    }

    let d_vol_a = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .expect("second-order base polytope has valid finite geometry");

    let mut gradient_rows: Vec<Vec<f64>> = Vec::with_capacity(near_optimal.len());
    let mut gradient_matrix_arrays: Vec<Vec<[f64; 4]>> = Vec::with_capacity(near_optimal.len());

    for orbit in &near_optimal {
        let grad = orbit_sys_gradient_a(polytope, orbit, vol, cap, sys, &d_vol_a);
        gradient_matrix_arrays.push(grad.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect());
        gradient_rows.push(flatten_gradient(&grad));
    }

    let orbit_count = gradient_rows.len();
    let g_matrix = DMatrix::from_fn(orbit_count, dim, |i, j| gradient_rows[i][j]);

    println!("\n  Gradient matrix: {}×{}", orbit_count, dim);

    let svd = g_matrix.svd(false, true);
    let singular_values: Vec<f64> = svd.singular_values.iter().cloned().collect();

    let sigma_max = singular_values[0];
    let threshold = sigma_max * SVD_RANK_THRESHOLD;
    let rank = singular_values.iter().filter(|&&s| s > threshold).count();
    let n_flat = dim - rank;

    println!("  SVD: σ_max={sigma_max:.6e}, threshold={threshold:.6e}");
    println!("  Rank: {rank} (of {dim})");
    println!("  Flat directions: {n_flat}");
    println!(
        "  Top 10 singular values: {:?}",
        singular_values
            .iter()
            .take(10)
            .map(|s| format!("{s:.4e}"))
            .collect::<Vec<_>>()
    );
    if rank < dim {
        println!("  Singular values near rank boundary:");
        let start = rank.saturating_sub(2);
        let end = (rank + 3).min(singular_values.len());
        for (i, &s) in singular_values[start..end].iter().enumerate() {
            let idx = start + i;
            let marker = if idx == rank {
                " ← rank boundary"
            } else {
                ""
            };
            println!("    σ[{idx}] = {s:.6e}{marker}");
        }
    }

    let v_t = svd.v_t.expect("SVD v_t should exist");
    let mut flat_directions: Vec<Vec<f64>> = Vec::with_capacity(n_flat);
    let mut flat_directions_arrays: Vec<Vec<[f64; 4]>> = Vec::with_capacity(n_flat);

    for i in rank..dim {
        let row: Vec<f64> = (0..dim).map(|j| v_t[(i, j)]).collect();
        flat_directions_arrays.push(unflatten_to_arrays(&row));
        flat_directions.push(row);
    }

    let duals_raw: Vec<[f64; 4]> = polytope
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();

    let base_row = BaseRow {
        facet_count,
        dual_vertices: duals_raw,
        sys_base: sys,
        capacity_base: cap,
        volume_base: vol,
        n_orbits_total: instr.orbits.len(),
        n_near_optimal: near_optimal.len(),
        singular_values,
        rank,
        n_flat_directions: n_flat,
        flat_directions: flat_directions_arrays,
        gradient_matrix: gradient_matrix_arrays,
        time_phase1_ms: 0.0,
    };

    (base_row, flat_directions)
}
