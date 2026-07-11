//! Orbit recovery and trajectory generation for visualization export.

use crate::models::{v4_to_array, VizSegment, VizTrajectory};
use crate::orbit_collection::{collect_all_orbits, CollectedOrbit};
use euclidean_polytopes::facet_vertices_from_vertex_facet_incidence;
use nalgebra::{DMatrix, Vector4};
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify_sigma_beta_action;
use symplectic::geom::reeb_trajectory;

/// Maximum number of orbits to export per polytope.
const MAX_ORBITS: usize = 20;

/// Displacement magnitude for displaced orbit visualization.
const DISPLACEMENT_EPS: f64 = 0.02;

/// Max facet count for orbit computation.
const MAX_FACETS_FOR_ORBIT: usize = 12;

/// Tolerance for checking that simulated segments remain on their declared
/// facet and inside all half-spaces `a_i · x <= 1`.
const EPS_TRAJECTORY_BOUNDARY: f64 = 1e-6;

/// Recover a Reeb orbit and convert it to a visualization trajectory.
fn orbit_to_viz_trajectory(
    dual_vertices: &[Vector4<f64>],
    orbit: &CollectedOrbit,
    label: String,
) -> Option<VizTrajectory> {
    let recovery = recover_and_verify_sigma_beta_action(
        dual_vertices,
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
    let n1_raw = dual_vertices[last_facet].normalize();
    let n1_orthogonal = n1_raw - n0.dot(&n1_raw) * n0;
    let normal_basis = if n1_orthogonal.norm() > 1e-8 {
        vec![n0, n1_orthogonal.normalize()]
    } else {
        vec![n0]
    };

    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];

    let mut basis: Vec<Vector4<f64>> = Vec::new();
    for e in &candidates {
        let mut v = *e;
        for normal in &normal_basis {
            v -= normal.dot(&v) * normal;
        }
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

fn segments_satisfy_boundary(
    dual_vertices: &[Vector4<f64>],
    segments: &[reeb_trajectory::ReebSegment],
) -> bool {
    segments.iter().all(|segment| {
        let Some(facet_normal) = dual_vertices.get(segment.facet) else {
            return false;
        };
        [&segment.start, &segment.end].into_iter().all(|point| {
            (facet_normal.dot(point) - 1.0).abs() <= EPS_TRAJECTORY_BOUNDARY
                && dual_vertices
                    .iter()
                    .all(|normal| normal.dot(point) <= 1.0 + EPS_TRAJECTORY_BOUNDARY)
        })
    })
}

/// Generate displaced trajectories by perturbing the base point of an orbit.
fn generate_displaced_trajectories(
    dual_vertices: &[Vector4<f64>],
    orbit: &CollectedOrbit,
) -> Vec<VizTrajectory> {
    let recovery = match recover_and_verify_sigma_beta_action(
        dual_vertices,
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
    let directions = two_face_displacement_directions(dual_vertices, start_facet, last_facet);
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
        let mut traj = reeb_trajectory::simulate_with(
            dual_vertices,
            displaced_start,
            start_facet,
            max_segments,
            1e-6,
        );
        if traj.segments.is_empty() {
            displaced_start = recovery.breakpoints[0] - DISPLACEMENT_EPS * disp;
            traj = reeb_trajectory::simulate_with(
                dual_vertices,
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
        if !segments_satisfy_boundary(dual_vertices, &traj.segments) {
            eprintln!(
                "  displaced v{}: trajectory failed facet-boundary check",
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
fn generate_placeholder_trajectory(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
    vertex_facet_incidence: &DMatrix<bool>,
) -> Vec<VizTrajectory> {
    let facet_vertices = facet_vertices_from_vertex_facet_incidence(vertex_facet_incidence);

    for fi in 0..dual_vertices.len() {
        if facet_vertices[fi].is_empty() {
            continue;
        }
        let centroid = facet_vertices[fi]
            .iter()
            .map(|&vertex_index| vertices[vertex_index])
            .sum::<Vector4<f64>>()
            / facet_vertices[fi].len() as f64;
        let traj = reeb_trajectory::simulate_with(dual_vertices, centroid, fi, 100, 1e-6);

        if !traj.segments.is_empty() {
            if !segments_satisfy_boundary(dual_vertices, &traj.segments) {
                eprintln!("  placeholder candidate on facet {fi} failed boundary check");
                continue;
            }
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
pub(crate) fn generate_trajectories(
    dual_vertices: &[Vector4<f64>],
    vertices: &[Vector4<f64>],
    vertex_facet_incidence: &DMatrix<bool>,
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> (Vec<VizTrajectory>, Option<f64>) {
    if dual_vertices.len() > MAX_FACETS_FOR_ORBIT {
        eprintln!(
            "  Skipping orbit computation (F={}, too many facets). Using placeholder.",
            dual_vertices.len()
        );
        return (
            generate_placeholder_trajectory(dual_vertices, vertices, vertex_facet_incidence),
            None,
        );
    }

    let all_orbits = collect_all_orbits(dual_vertices, facet_intersection_is_nonempty, omega_signs);
    if all_orbits.is_empty() {
        eprintln!("  No valid orbits found. Using placeholder.");
        return (
            generate_placeholder_trajectory(dual_vertices, vertices, vertex_facet_incidence),
            None,
        );
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

        match orbit_to_viz_trajectory(dual_vertices, orbit, label.clone()) {
            Some(traj) => {
                eprintln!(
                    "  {} → {} segments, facets {:?}",
                    label,
                    traj.segments.len(),
                    orbit.permutation
                );
                trajectories.push(traj);
            }
            None => {
                eprintln!("  {} → recovery failed, skipping", label);
            }
        }
    }

    if let Some(first_orbit) = all_orbits.first() {
        let displaced = generate_displaced_trajectories(dual_vertices, first_orbit);
        for d in displaced {
            eprintln!(
                "  {} → {} segments, closed={}",
                d.label,
                d.segments.len(),
                d.closed
            );
            trajectories.push(d);
        }
    }

    if trajectories.is_empty() {
        eprintln!("  All orbit recoveries failed. Using placeholder.");
        return (
            generate_placeholder_trajectory(dual_vertices, vertices, vertex_facet_incidence),
            Some(min_action),
        );
    }

    (trajectories, Some(min_action))
}

#[cfg(test)]
mod tests {
    use super::two_face_displacement_directions;
    use nalgebra::Vector4;

    #[test]
    fn displacement_directions_are_tangent_to_both_nonorthogonal_facets() {
        let normals = [
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(1.0, 1.0, 0.0, 0.0),
        ];
        let directions = two_face_displacement_directions(&normals, 0, 1);

        assert_eq!(directions.len(), 2);
        for direction in directions {
            assert!(normals[0].dot(&direction).abs() < 1e-12);
            assert!(normals[1].dot(&direction).abs() < 1e-12);
        }
    }
}
