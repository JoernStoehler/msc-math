//! Tests for reeb_trajectory: piecewise-linear Reeb flow simulation.
//!
//! Proposition: The simulated trajectory follows the Reeb direction J_0 n_i
//! on each facet, transitions at ridges, and stays inside the polytope.
//! Reference: [def:reeb-vector-field], [lem:piecewise-linear-reeb]
//!
//! Strategy: fixture-based on known polytopes (simplex, hypercube, cross-polytope,
//! Lagrangian products). Tests verify direction, continuity, containment.

use crate::geom::known_polytopes;
use crate::geom::reeb_trajectory::{reeb_direction, simulate, simulate_with};
use crate::geom::skeleton::Skeleton;
use crate::geom::symplectic_form::j4;
use nalgebra::Vector4;

/// J_0 (a, b, c, d) = (-c, -d, a, b) for standard basis vectors.
#[test]
fn reeb_direction_axis_aligned() {
    let cases = [
        (Vector4::new(1.0, 0.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 1.0, 0.0)),
        (Vector4::new(0.0, 1.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 0.0, 1.0)),
        (Vector4::new(0.0, 0.0, 1.0, 0.0), Vector4::new(-1.0, 0.0, 0.0, 0.0)),
        (Vector4::new(0.0, 0.0, 0.0, 1.0), Vector4::new(0.0, -1.0, 0.0, 0.0)),
    ];
    for (n, expected) in &cases {
        let r = reeb_direction(n);
        assert!(
            (r - expected).norm() < 1e-12,
            "reeb_direction({n:?}) = {r:?}, expected {expected:?}"
        );
    }
}

/// reeb_direction(n) agrees with J_0 * n via matrix multiplication.
#[test]
fn reeb_direction_matches_j4_matrix() {
    let j0 = j4();
    let normals = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.3, 0.4, 0.5, 0.6),
        Vector4::new(-0.7, 0.1, 0.2, -0.3),
    ];
    for n in &normals {
        let r = reeb_direction(n);
        let expected = j0 * n;
        assert!(
            (r - expected).norm() < 1e-12,
            "reeb_direction({n:?}) = {r:?}, expected {expected:?}"
        );
    }
}

/// J_0 n . n = omega_0(n, n) = 0 (antisymmetry of the symplectic form).
#[test]
fn reeb_direction_tangent_to_facet() {
    let normals = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.5, 0.5, 0.5, 0.5).normalize(),
        Vector4::new(0.8, -0.3, 0.2, 0.5).normalize(),
    ];
    for n in &normals {
        let r = reeb_direction(n);
        let dot = r.dot(n);
        assert!(
            dot.abs() < 1e-12,
            "Reeb direction not tangent: n={n:?}, R.n={dot}"
        );
    }
}

/// On the hypercube, a trajectory from facet 0 should visit multiple facets.
#[test]
fn hypercube_trajectory_visits_multiple_facets() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let start = skel.facet_centroid(&kp.polytope, 0);

    // Verify start is on facet 0.
    let residual = (kp.polytope.normals_f64()[0].dot(&start) - kp.polytope.heights_f64()[0]).abs();
    assert!(residual < 1e-7, "start not on facet 0: residual {residual}");

    let traj = simulate(&kp.polytope, start, 0);

    assert!(
        traj.segments.len() >= 2,
        "expected multiple segments, got {}",
        traj.segments.len()
    );

    // Consecutive segments are continuous: end[i] = start[i+1].
    for i in 0..traj.segments.len() - 1 {
        let gap = (traj.segments[i].end - traj.segments[i + 1].start).norm();
        assert!(gap < 1e-10, "segment gap at {i}: {gap}");
    }

    // Each segment lies on its claimed facet.
    let normals = kp.polytope.normals_f64();
    let heights = kp.polytope.heights_f64();
    for seg in &traj.segments {
        let n = &normals[seg.facet];
        let h = heights[seg.facet];
        let start_res = (n.dot(&seg.start) - h).abs();
        let end_res = (n.dot(&seg.end) - h).abs();
        assert!(start_res < 1e-7, "start not on facet {}: {start_res}", seg.facet);
        assert!(end_res < 1e-7, "end not on facet {}: {end_res}", seg.facet);
    }
}

/// Each segment direction is parallel to J_0 n_i (the Reeb direction on its facet).
#[test]
fn segments_follow_reeb_direction() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let start = skel.facet_centroid(&kp.polytope, 0);
    let traj = simulate_with(&kp.polytope, start, 0, 20, 1e-6);

    let normals = kp.polytope.normals_f64();
    for seg in &traj.segments {
        let direction = seg.end - seg.start;
        if direction.norm() < 1e-12 {
            continue; // zero-length immediate transition
        }
        let expected = reeb_direction(&normals[seg.facet]);
        let cos_angle = direction.normalize().dot(&expected.normalize());
        assert!(
            (cos_angle - 1.0).abs() < 1e-8,
            "segment on facet {} not in Reeb direction: cos_angle={cos_angle}",
            seg.facet,
        );
    }
}

/// The simplex produces at least one segment.
#[test]
fn simplex_trajectory_produces_segments() {
    let kp = known_polytopes::simplex();
    let skel = Skeleton::compute(&kp.polytope);
    let start = skel.facet_centroid(&kp.polytope, 0);
    let traj = simulate(&kp.polytope, start, 0);

    assert!(
        !traj.segments.is_empty(),
        "simplex trajectory should have segments"
    );
}

/// All segment endpoints satisfy every halfspace constraint of the polytope.
/// Tests across multiple polytopes and starting facets.
#[test]
fn trajectory_stays_inside_polytope() {
    let polytopes: Vec<(&str, _)> = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
        ("crosspolytope", known_polytopes::crosspolytope()),
        ("hko_pentagon", known_polytopes::hko_pentagon()),
        (
            "lagrangian_triangle_product",
            known_polytopes::lagrangian_triangle_product(),
        ),
        (
            "symplectic_triangle_product",
            known_polytopes::symplectic_triangle_product(),
        ),
    ];

    for (name, kp) in &polytopes {
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();
        let skel = Skeleton::compute(&kp.polytope);

        for facet in 0..normals.len() {
            let start = skel.facet_centroid(&kp.polytope, facet);
            let traj = simulate_with(&kp.polytope, start, facet, 100, 1e-6);

            for (si, seg) in traj.segments.iter().enumerate() {
                for (label, pt) in [("start", &seg.start), ("end", &seg.end)] {
                    for (fk, (nk, hk)) in normals.iter().zip(heights.iter()).enumerate() {
                        let violation = nk.dot(pt) - hk;
                        assert!(
                            violation < 1e-6,
                            "{name} facet {facet} seg {si} {label}: \
                             violates facet {fk} by {violation:.2e}"
                        );
                    }
                }

                // Segment start/end lies on claimed facet's hyperplane.
                let n_f = &normals[seg.facet];
                let h_f = heights[seg.facet];
                let res_start = (n_f.dot(&seg.start) - h_f).abs();
                let res_end = (n_f.dot(&seg.end) - h_f).abs();
                assert!(
                    res_start < 1e-6,
                    "{name} facet {facet} seg {si}: start not on facet {}, res {res_start:.2e}",
                    seg.facet
                );
                assert!(
                    res_end < 1e-6,
                    "{name} facet {facet} seg {si}: end not on facet {}, res {res_end:.2e}",
                    seg.facet
                );
            }
        }
    }
}

/// simulate() with defaults produces the same result as simulate_with()
/// using the default constants.
#[test]
fn simulate_defaults_match_explicit() {
    use crate::geom::reeb_trajectory::{DEFAULT_CLOSURE_TOL, DEFAULT_MAX_SEGMENTS};

    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let start = skel.facet_centroid(&kp.polytope, 0);

    let traj_default = simulate(&kp.polytope, start, 0);
    let traj_explicit =
        simulate_with(&kp.polytope, start, 0, DEFAULT_MAX_SEGMENTS, DEFAULT_CLOSURE_TOL);

    assert_eq!(traj_default.segments.len(), traj_explicit.segments.len());
    assert_eq!(traj_default.closed, traj_explicit.closed);

    for (a, b) in traj_default.segments.iter().zip(traj_explicit.segments.iter()) {
        assert_eq!(a.facet, b.facet);
        assert!((a.start - b.start).norm() < 1e-14);
        assert!((a.end - b.end).norm() < 1e-14);
    }
}
