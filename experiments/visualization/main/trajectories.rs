//! Orbit recovery and trajectory generation for visualization export.

use crate::models::{v4_to_array, VizSegment, VizTrajectory};
use crate::orbit_collection::{collect_all_orbits, CollectedOrbit};
use nalgebra::Vector4;
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify_sigma_beta_action;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::reeb_trajectory;
use symplectic::geom::skeleton::Skeleton;

/// Maximum number of orbits to export per polytope.
const MAX_ORBITS: usize = 20;

/// Displacement magnitude for displaced orbit visualization.
const DISPLACEMENT_EPS: f64 = 0.02;

/// Max facet count for orbit computation.
const MAX_FACETS_FOR_ORBIT: usize = 12;

/// Recover a Reeb orbit and convert it to a visualization trajectory.
fn orbit_to_viz_trajectory(
    polytope: &Polytope4D,
    orbit: &CollectedOrbit,
    label: String,
) -> Option<VizTrajectory> {
    let recovery = recover_and_verify_sigma_beta_action(
        polytope,
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

/// Compute an orthonormal basis for the tangent space of the starting ridge.
fn ridge_displacement_directions(
    polytope: &Polytope4D,
    first_facet: usize,
    last_facet: usize,
) -> Vec<Vector4<f64>> {
    let duals = polytope.dual_vertices_f64();
    let n0 = duals[first_facet].normalize();
    let n1 = duals[last_facet].normalize();

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
    polytope: &Polytope4D,
    orbit: &CollectedOrbit,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    let recovery = match recover_and_verify_sigma_beta_action(
        polytope,
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
    let directions = ridge_displacement_directions(polytope, start_facet, last_facet);
    eprintln!(
        "  Ridge F_{} ∩ F_{}: {} displacement direction(s)",
        start_facet,
        last_facet,
        directions.len()
    );

    let _ = skeleton;

    let max_segments = orbit.permutation.len();
    let mut trajectories = Vec::new();
    for (i, disp) in directions.iter().enumerate() {
        let mut displaced_start = recovery.breakpoints[0] + DISPLACEMENT_EPS * disp;
        let mut traj = reeb_trajectory::simulate_with(
            polytope,
            displaced_start,
            start_facet,
            max_segments,
            1e-6,
        );
        if traj.segments.is_empty() {
            displaced_start = recovery.breakpoints[0] - DISPLACEMENT_EPS * disp;
            traj = reeb_trajectory::simulate_with(
                polytope,
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
fn generate_placeholder_trajectory(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    for fi in 0..polytope.facet_count() {
        let centroid = skeleton.facet_centroid(polytope, fi);
        let traj = reeb_trajectory::simulate_with(polytope, centroid, fi, 100, 1e-6);

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
pub(crate) fn generate_trajectories(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
) -> (Vec<VizTrajectory>, Option<f64>) {
    if polytope.facet_count() > MAX_FACETS_FOR_ORBIT {
        eprintln!(
            "  Skipping orbit computation (F={}, too many facets). Using placeholder.",
            polytope.facet_count()
        );
        return (generate_placeholder_trajectory(polytope, skeleton), None);
    }

    let all_orbits = collect_all_orbits(polytope);
    if all_orbits.is_empty() {
        eprintln!("  No valid orbits found. Using placeholder.");
        return (generate_placeholder_trajectory(polytope, skeleton), None);
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

        match orbit_to_viz_trajectory(polytope, orbit, label.clone()) {
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
        let displaced = generate_displaced_trajectories(polytope, first_orbit, skeleton);
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
            generate_placeholder_trajectory(polytope, skeleton),
            Some(min_action),
        );
    }

    (trajectories, Some(min_action))
}
