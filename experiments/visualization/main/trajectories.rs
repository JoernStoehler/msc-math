//! Orbit recovery and trajectory generation for visualization export.

use crate::models::{v4_to_array, VizSegment, VizTrajectory};
use crate::orbit_collection::{collect_all_orbits, CollectedOrbit};
use euclidean_polytopes::facet_vertices_from_vertex_facet_incidence;
use nalgebra::{DMatrix, DVector, Vector4};
use symplectic::geom::known_polytopes::KnownPolytope;
use symplectic::geom::reeb_trajectory::{reeb_direction, ReebSegment, ReebTrajectory};
use symplectic::omega0;

/// Maximum number of orbits to export per polytope.
const MAX_ORBITS: usize = 20;

/// Displacement magnitude for displaced orbit visualization.
const DISPLACEMENT_EPS: f64 = 0.02;

/// Max facet count for orbit computation.
const MAX_FACETS_FOR_ORBIT: usize = 12;

const EPS_FACET_INCIDENCE: f64 = symplectic::constants::EPS_FACET_INCIDENCE;
const EPS_DENOM: f64 = 1e-10;

struct RecoveredOrbit {
    breakpoints: Vec<Vector4<f64>>,
    max_violation: f64,
    action: f64,
    closure_error: f64,
}

fn max_violation_for(
    base_point: &Vector4<f64>,
    displacements: &[Vector4<f64>],
    dual_vertices: &[Vector4<f64>],
) -> f64 {
    displacements
        .iter()
        .flat_map(|v| {
            let p = base_point + v;
            dual_vertices.iter().map(move |a| a.dot(&p) - 1.0)
        })
        .fold(f64::NEG_INFINITY, f64::max)
}

fn optimize_in_null_space(
    b0: Vector4<f64>,
    null_vecs: &[Vector4<f64>],
    displacements: &[Vector4<f64>],
    dual_vertices: &[Vector4<f64>],
) -> Vector4<f64> {
    if null_vecs.is_empty() {
        return b0;
    }

    let mut alphas = vec![0.0_f64; null_vecs.len()];
    for _ in 0..20 {
        for dim in 0..null_vecs.len() {
            let candidate = |a: f64| -> Vector4<f64> {
                let mut b = b0;
                for (i, d) in null_vecs.iter().enumerate() {
                    let ai = if i == dim { a } else { alphas[i] };
                    b += ai * d;
                }
                b
            };

            let mut lo = -100.0_f64;
            let mut hi = 100.0_f64;
            for _ in 0..100 {
                let m1 = lo + (hi - lo) / 3.0;
                let m2 = hi - (hi - lo) / 3.0;
                let v1 = max_violation_for(&candidate(m1), displacements, dual_vertices);
                let v2 = max_violation_for(&candidate(m2), displacements, dual_vertices);
                if v1 < v2 {
                    hi = m2;
                } else {
                    lo = m1;
                }
            }

            alphas[dim] = (lo + hi) / 2.0;
        }
    }

    let mut b = b0;
    for (i, d) in null_vecs.iter().enumerate() {
        b += alphas[i] * d;
    }
    b
}

fn recover_sigma_beta_action(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
    beta: &[f64],
    action: f64,
) -> Option<RecoveredOrbit> {
    if sigma.len() != beta.len() || !action.is_finite() || action <= 0.0 {
        return None;
    }
    if sigma.iter().any(|&facet| facet >= dual_vertices.len()) {
        return None;
    }
    if beta.iter().any(|&entry| !entry.is_finite() || entry <= 0.0) {
        return None;
    }

    let m = sigma.len();
    let dwell_times: Vec<f64> = (0..m).map(|k| action * beta[k]).collect();
    let reeb_vectors: Vec<Vector4<f64>> = sigma
        .iter()
        .map(|&facet| reeb_direction(&dual_vertices[facet]) * 2.0)
        .collect();

    let mut displacements = Vec::with_capacity(m + 1);
    displacements.push(Vector4::zeros());
    for k in 0..m {
        displacements.push(displacements[k] + dwell_times[k] * reeb_vectors[k]);
    }

    let active: Vec<usize> = (0..m).filter(|&k| dwell_times[k] > 0.0).collect();
    if active.is_empty() {
        return None;
    }

    let rows = active.len().max(4);
    let mut mat = DMatrix::<f64>::zeros(rows, 4);
    let mut rhs = DVector::<f64>::zeros(rows);
    for (row, &k) in active.iter().enumerate() {
        let a = &dual_vertices[sigma[k]];
        for col in 0..4 {
            mat[(row, col)] = a[col];
        }
        rhs[row] = 1.0 - a.dot(&displacements[k]);
    }

    let svd = mat.svd(true, true);
    let tol = 1e-10 * svd.singular_values[0].max(1.0);
    let rank = svd.singular_values.iter().filter(|&&s| s > tol).count();
    let solution_dim = 4 - rank;

    let b_vec = svd.solve(&rhs, tol).ok()?;
    let mut base_point = Vector4::new(b_vec[0], b_vec[1], b_vec[2], b_vec[3]);

    if solution_dim > 0 {
        if let Some(v_mat) = &svd.v_t {
            let null_vecs: Vec<Vector4<f64>> = (rank..4)
                .map(|i| Vector4::new(v_mat[(i, 0)], v_mat[(i, 1)], v_mat[(i, 2)], v_mat[(i, 3)]))
                .collect();
            base_point =
                optimize_in_null_space(base_point, &null_vecs, &displacements, dual_vertices);
        }
    }

    let breakpoints: Vec<Vector4<f64>> = (0..=m).map(|k| base_point + displacements[k]).collect();
    let max_violation = breakpoints
        .iter()
        .flat_map(|p| dual_vertices.iter().map(move |a| a.dot(p) - 1.0))
        .fold(f64::NEG_INFINITY, f64::max);
    let closure_error = (breakpoints[m] - breakpoints[0]).norm();

    let mut action_sum = 0.0;
    for i in 1..m {
        for j in 0..i {
            action_sum +=
                dwell_times[j] * dwell_times[i] * omega0(&reeb_vectors[j], &reeb_vectors[i]);
        }
    }

    Some(RecoveredOrbit {
        breakpoints,
        max_violation,
        action: action_sum / 2.0,
        closure_error,
    })
}

fn simulate_with_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    start_point: Vector4<f64>,
    start_facet: usize,
    max_segments: usize,
    closure_tol: f64,
) -> ReebTrajectory {
    let n_facets = dual_vertices.len();
    let mut segments = Vec::new();
    let mut current_point = start_point;
    let mut current_facet = start_facet;
    let mut closed = false;

    for _ in 0..max_segments {
        for _ in 0..n_facets {
            let r = reeb_direction(&dual_vertices[current_facet]);
            let mut best_immediate = None;
            let mut best_denom = 0.0_f64;
            for (fj, a_j) in dual_vertices.iter().enumerate() {
                if fj == current_facet {
                    continue;
                }
                let residual = a_j.dot(&current_point) - 1.0;
                let denom = a_j.dot(&r);
                if residual.abs() < EPS_FACET_INCIDENCE && denom > EPS_DENOM && denom > best_denom {
                    best_denom = denom;
                    best_immediate = Some(fj);
                }
            }
            if let Some(fj) = best_immediate {
                current_facet = fj;
            } else {
                break;
            }
        }

        let reeb = reeb_direction(&dual_vertices[current_facet]);
        let mut best_t = f64::INFINITY;
        let mut next_facet = current_facet;

        for (fj, a_j) in dual_vertices.iter().enumerate() {
            if fj == current_facet {
                continue;
            }
            let denom = a_j.dot(&reeb);
            if denom.abs() < EPS_DENOM {
                continue;
            }
            let t = (1.0 - a_j.dot(&current_point)) / denom;
            if t > EPS_FACET_INCIDENCE && t < best_t {
                best_t = t;
                next_facet = fj;
            }
        }

        if best_t == f64::INFINITY || next_facet == current_facet {
            break;
        }

        let end_point = current_point + best_t * reeb;
        let valid = dual_vertices
            .iter()
            .all(|a_k| a_k.dot(&end_point) - 1.0 <= EPS_FACET_INCIDENCE * 100.0);
        if !valid {
            break;
        }

        segments.push(ReebSegment {
            start: current_point,
            end: end_point,
            facet: current_facet,
        });

        if segments.len() >= 2 && (end_point - start_point).norm() < closure_tol {
            closed = true;
            break;
        }

        current_point = end_point;
        current_facet = next_facet;
    }

    ReebTrajectory { segments, closed }
}

/// Recover a Reeb orbit and convert it to a visualization trajectory.
fn orbit_to_viz_trajectory(
    polytope: &KnownPolytope,
    orbit: &CollectedOrbit,
    label: String,
) -> Option<VizTrajectory> {
    let recovery = recover_sigma_beta_action(
        &polytope.dual_vertices_f64,
        &orbit.permutation,
        &orbit.beta,
        orbit.action,
    )?;

    if recovery.closure_error > 1e-6 {
        eprintln!(
            "  WARN orbit {}: closure_error={:.2e} (too large, skipping)",
            label, recovery.closure_error
        );
        return None;
    }
    if recovery.max_violation > 1e-4 {
        eprintln!(
            "  WARN orbit {}: max_violation={:.2e} (too large, skipping)",
            label, recovery.max_violation
        );
        return None;
    }

    let sigma = &orbit.permutation;
    let segments: Vec<VizSegment> = (0..sigma.len())
        .map(|k| VizSegment {
            start: v4_to_array(&recovery.breakpoints[k]),
            end: v4_to_array(&recovery.breakpoints[k + 1]),
            facet: sigma[k],
        })
        .collect();

    Some(VizTrajectory {
        label,
        start_facet: sigma[0],
        closed: true,
        segments,
    })
}

/// Compute an orthonormal basis for the tangent space of the starting two-face.
fn two_face_displacement_directions(
    dual_vertices: &[Vector4<f64>],
    first_facet: usize,
    last_facet: usize,
) -> Vec<Vector4<f64>> {
    let n0 = dual_vertices[first_facet].normalize();
    let n1 = dual_vertices[last_facet].normalize();

    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];

    let mut basis: Vec<Vector4<f64>> = Vec::new();
    for e in &candidates {
        let mut v: Vector4<f64> = e - n0.dot(e) * n0;
        v -= n1.dot(&v) * n1;
        for b in &basis {
            v -= b.dot(&v) * b;
        }
        let norm = v.norm();
        if norm > 1e-8 {
            basis.push(v / norm);
            if basis.len() == 2 {
                break;
            }
        }
    }
    basis
}

/// Generate displaced trajectories by perturbing the base point of an orbit.
fn generate_displaced_trajectories(
    polytope: &KnownPolytope,
    orbit: &CollectedOrbit,
) -> Vec<VizTrajectory> {
    let recovery = match recover_sigma_beta_action(
        &polytope.dual_vertices_f64,
        &orbit.permutation,
        &orbit.beta,
        orbit.action,
    ) {
        Some(r) => r,
        None => return vec![],
    };

    let perm = &orbit.permutation;
    let start_facet = perm[0];
    let last_facet = perm[perm.len() - 1];
    let directions =
        two_face_displacement_directions(&polytope.dual_vertices_f64, start_facet, last_facet);
    eprintln!(
        "  Two-face F_{} ∩ F_{}: {} displacement direction(s)",
        start_facet,
        last_facet,
        directions.len()
    );

    let max_segments = orbit.permutation.len();
    let mut trajectories = Vec::new();
    for (i, disp) in directions.iter().enumerate() {
        let mut displaced_start = recovery.breakpoints[0] + DISPLACEMENT_EPS * disp;
        let mut traj = simulate_with_dual_vertices(
            &polytope.dual_vertices_f64,
            displaced_start,
            start_facet,
            max_segments,
            1e-6,
        );
        if traj.segments.is_empty() {
            displaced_start = recovery.breakpoints[0] - DISPLACEMENT_EPS * disp;
            traj = simulate_with_dual_vertices(
                &polytope.dual_vertices_f64,
                displaced_start,
                start_facet,
                max_segments,
                1e-6,
            );
        }
        if traj.segments.is_empty() {
            eprintln!(
                "  displaced v{}: simulation returned 0 segments in both directions",
                i + 1
            );
            continue;
        }

        trajectories.push(VizTrajectory {
            label: format!("displaced v{} (ε={})", i + 1, DISPLACEMENT_EPS),
            start_facet,
            closed: traj.closed,
            segments: traj
                .segments
                .iter()
                .map(|s| VizSegment {
                    start: v4_to_array(&s.start),
                    end: v4_to_array(&s.end),
                    facet: s.facet,
                })
                .collect(),
        });
    }

    trajectories
}

/// Fallback: generate a single forward-simulated trajectory.
fn generate_placeholder_trajectory(polytope: &KnownPolytope) -> Vec<VizTrajectory> {
    let vertices = &polytope.vertices_f64;
    let facet_vertices =
        facet_vertices_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);

    for fi in 0..polytope.facet_count() {
        if facet_vertices[fi].is_empty() {
            continue;
        }
        let centroid = facet_vertices[fi]
            .iter()
            .map(|&vertex_index| vertices[vertex_index])
            .sum::<Vector4<f64>>()
            / facet_vertices[fi].len() as f64;
        let traj =
            simulate_with_dual_vertices(&polytope.dual_vertices_f64, centroid, fi, 100, 1e-6);

        if !traj.segments.is_empty() {
            return vec![VizTrajectory {
                label: "placeholder trajectory".to_string(),
                start_facet: fi,
                closed: traj.closed,
                segments: traj
                    .segments
                    .iter()
                    .map(|s| VizSegment {
                        start: v4_to_array(&s.start),
                        end: v4_to_array(&s.end),
                        facet: s.facet,
                    })
                    .collect(),
            }];
        }
    }

    vec![]
}

/// Generate all trajectories for a polytope and return them with the computed capacity.
pub(crate) fn generate_trajectories(polytope: &KnownPolytope) -> (Vec<VizTrajectory>, Option<f64>) {
    if polytope.facet_count() > MAX_FACETS_FOR_ORBIT {
        eprintln!(
            "  Skipping orbit computation (F={}, too many facets). Using placeholder.",
            polytope.facet_count()
        );
        return (generate_placeholder_trajectory(polytope), None);
    }

    let all_orbits = collect_all_orbits(polytope);
    if all_orbits.is_empty() {
        eprintln!("  No valid orbits found. Using placeholder.");
        return (generate_placeholder_trajectory(polytope), None);
    }

    let min_action = all_orbits[0].action;
    eprintln!(
        "  Found {} orbits (min action = {:.6}, max action = {:.6})",
        all_orbits.len(),
        min_action,
        all_orbits.last().unwrap().action
    );

    let mut trajectories = Vec::new();
    let mut min_action_count = 0usize;
    for (i, orbit) in all_orbits.iter().enumerate() {
        if trajectories.len() >= MAX_ORBITS {
            eprintln!(
                "  Capped at {} orbits (skipped {})",
                MAX_ORBITS,
                all_orbits.len() - MAX_ORBITS
            );
            break;
        }

        let is_min = (orbit.action - min_action).abs() < 1e-8;
        let label = if is_min {
            min_action_count += 1;
            if min_action_count == 1 {
                format!("min-action orbit (c={:.4})", orbit.action)
            } else {
                format!(
                    "min-action orbit #{} (c={:.4})",
                    min_action_count, orbit.action
                )
            }
        } else {
            format!("orbit #{} (action={:.4})", i + 1, orbit.action)
        };

        if let Some(traj) = orbit_to_viz_trajectory(polytope, orbit, label) {
            if is_min && min_action_count == 1 {
                trajectories.extend(generate_displaced_trajectories(polytope, orbit));
            }
            trajectories.push(traj);
        }
    }

    if trajectories.is_empty() {
        eprintln!("  Orbit recovery failed for all orbits. Using placeholder.");
        return (generate_placeholder_trajectory(polytope), None);
    }

    (trajectories, Some(min_action))
}
