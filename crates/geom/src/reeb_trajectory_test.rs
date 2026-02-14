use super::*;
use crate::known_polytopes;
use crate::skeleton::Skeleton;
use crate::symplectic::j4;

#[test]
fn reeb_vector_axis_aligned() {
    // J₀ (1,0,0,0) = (0,0,1,0) in coords (q₁,q₂,p₁,p₂)
    // J₀ [[0,-I],[I,0]]: (a,b,c,d) -> (-c,-d,a,b)
    let n = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let r = reeb_vector(&n);
    assert!((r - Vector4::new(0.0, 0.0, 1.0, 0.0)).norm() < 1e-12);

    let n = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let r = reeb_vector(&n);
    assert!((r - Vector4::new(0.0, 0.0, 0.0, 1.0)).norm() < 1e-12);

    let n = Vector4::new(0.0, 0.0, 1.0, 0.0);
    let r = reeb_vector(&n);
    assert!((r - Vector4::new(-1.0, 0.0, 0.0, 0.0)).norm() < 1e-12);

    let n = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let r = reeb_vector(&n);
    assert!((r - Vector4::new(0.0, -1.0, 0.0, 0.0)).norm() < 1e-12);
}

#[test]
fn reeb_vector_matches_j4_matrix() {
    // Verify reeb_vector(n) == J₀ * n for a few normals
    let j0 = j4();
    let normals = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.3, 0.4, 0.5, 0.6),
        Vector4::new(-0.7, 0.1, 0.2, -0.3),
    ];
    for n in &normals {
        let r = reeb_vector(n);
        let expected = j0 * n;
        assert!(
            (r - expected).norm() < 1e-12,
            "reeb_vector({n:?}) = {r:?}, expected {expected:?}"
        );
    }
}

#[test]
fn reeb_vector_tangent_to_facet() {
    // J₀n · n = ω₀(n, n) = 0 (antisymmetry)
    let normals = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.5, 0.5, 0.5, 0.5).normalize(),
        Vector4::new(0.8, -0.3, 0.2, 0.5).normalize(),
    ];
    for n in &normals {
        let r = reeb_vector(n);
        let dot = r.dot(n);
        assert!(
            dot.abs() < 1e-12,
            "Reeb vector not tangent: n={n:?}, R·n={dot}"
        );
    }
}

#[test]
fn hypercube_trajectory_visits_multiple_facets() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);

    // Start from centroid of facet 0 (n = (1,0,0,0), the x₁=1 facet)
    let start = facet_centroid(&kp.polytope, &skel, 0);

    // Verify start is on facet 0: n₀·start ≈ h₀
    let residual = (kp.polytope.normals()[0].dot(&start) - kp.polytope.heights()[0]).abs();
    assert!(residual < 1e-7, "start not on facet 0: residual {residual}");

    let traj = simulate(&kp.polytope, start, 0, 50, 1e-6);

    assert!(
        traj.segments.len() >= 2,
        "trajectory should visit multiple facets, got {} segments",
        traj.segments.len()
    );

    // Check that segments are continuous: end of segment i = start of segment i+1
    for i in 0..traj.segments.len() - 1 {
        let gap = (traj.segments[i].end - traj.segments[i + 1].start).norm();
        assert!(gap < 1e-10, "segment gap at {i}: {gap}");
    }

    // Check that each segment stays on its facet
    for seg in &traj.segments {
        let n = &kp.polytope.normals()[seg.facet];
        let h = kp.polytope.heights()[seg.facet];
        let start_res = (n.dot(&seg.start) - h).abs();
        let end_res = (n.dot(&seg.end) - h).abs();
        assert!(
            start_res < 1e-7,
            "segment start not on facet {}: residual {start_res}",
            seg.facet
        );
        assert!(
            end_res < 1e-7,
            "segment end not on facet {}: residual {end_res}",
            seg.facet
        );
    }
}

#[test]
fn trajectory_segments_are_in_reeb_direction() {
    let kp = known_polytopes::hypercube();
    let skel = Skeleton::compute(&kp.polytope);
    let start = facet_centroid(&kp.polytope, &skel, 0);
    let traj = simulate(&kp.polytope, start, 0, 20, 1e-6);

    for seg in &traj.segments {
        let direction = seg.end - seg.start;
        let expected_dir = reeb_vector(&kp.polytope.normals()[seg.facet]);

        // direction should be parallel to expected_dir (positive scalar multiple)
        if direction.norm() < 1e-12 {
            continue;
        }
        let normalized = direction.normalize();
        let expected_normalized = expected_dir.normalize();
        let cross_norm = (normalized - expected_normalized).norm();
        assert!(
            cross_norm < 1e-8,
            "segment on facet {} not in Reeb direction: diff={cross_norm}",
            seg.facet,
        );
    }
}

#[test]
fn simplex_trajectory_produces_segments() {
    let kp = known_polytopes::simplex();
    let skel = Skeleton::compute(&kp.polytope);
    let start = facet_centroid(&kp.polytope, &skel, 0);
    let traj = simulate(&kp.polytope, start, 0, 50, 1e-6);

    assert!(
        !traj.segments.is_empty(),
        "simplex trajectory should have segments"
    );
}

/// Assert that every segment endpoint lies inside the polytope (all half-spaces).
/// This catches the bug where trajectories escape through unchecked facets.
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
        let normals = kp.polytope.normals();
        let heights = kp.polytope.heights();
        let skel = Skeleton::compute(&kp.polytope);

        for facet in 0..normals.len() {
            let start = facet_centroid(&kp.polytope, &skel, facet);
            let traj = simulate(&kp.polytope, start, facet, 100, 1e-6);

            for (si, seg) in traj.segments.iter().enumerate() {
                // Segment endpoints must satisfy all half-space constraints
                for (label, pt) in [("start", &seg.start), ("end", &seg.end)] {
                    for (fk, (nk, hk)) in normals.iter().zip(heights.iter()).enumerate() {
                        let violation = nk.dot(pt) - hk;
                        assert!(
                            violation < 1e-6,
                            "{name} facet {facet} traj seg {si} {label}: \
                             violates facet {fk} by {violation:.2e}, \
                             point={pt:?}"
                        );
                    }
                }

                // Segment start/end must lie on claimed facet's hyperplane
                let n_f = &normals[seg.facet];
                let h_f = heights[seg.facet];
                let res_start = (n_f.dot(&seg.start) - h_f).abs();
                let res_end = (n_f.dot(&seg.end) - h_f).abs();
                assert!(
                    res_start < 1e-6,
                    "{name} facet {facet} traj seg {si}: start not on facet {}, residual {res_start:.2e}",
                    seg.facet
                );
                assert!(
                    res_end < 1e-6,
                    "{name} facet {facet} traj seg {si}: end not on facet {}, residual {res_end:.2e}",
                    seg.facet
                );
            }
        }
    }
}
