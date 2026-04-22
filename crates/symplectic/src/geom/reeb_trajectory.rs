//! Piecewise-linear Reeb flow simulation on a polytope boundary.
//!
//! For a polytope K = {x : a_i^T x <= 1}, the Reeb vector on facet F_i is
//! R_i = 2 J_0 a_i. The direction J_0 a_i is automatically tangent to F_i
//! since omega_0(a_i, a_i) = 0.
//!
//! For trajectory visualization we only need the *direction* J_0 a_i (the
//! factor 2 rescales time but does not change the trajectory shape).
//!
//! A trajectory starts at a point on a facet, follows the Reeb direction until
//! hitting a ridge (shared 2-face with a neighboring facet), then switches to
//! the new facet's Reeb direction. This produces a piecewise-linear curve on
//! the boundary of K.
//!
//! Mathematical correspondence: [def:reeb-vector-field], [lem:piecewise-linear-reeb]

use crate::constants::EPS_FACET_INCIDENCE;
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

/// Default maximum number of linear segments before stopping.
///
/// All current call sites use 20-50 segments. 50 is generous enough for
/// orbits on polytopes with up to ~16 facets.
pub const DEFAULT_MAX_SEGMENTS: usize = 50;

/// Default closure tolerance: trajectory is "closed" when the endpoint
/// returns within this distance of the start point.
///
/// All current call sites use 1e-6.
pub const DEFAULT_CLOSURE_TOL: f64 = 1e-6;

/// Tolerance for ray-facet intersection denominator: a_j . R_i.
///
/// When the Reeb direction is nearly parallel to a facet (dot product near
/// zero), the intersection time t = distance / dot becomes numerically
/// unstable. Genuine transitions have dot products well above this threshold.
const EPS_DENOM: f64 = 1e-10;

/// A single linear segment of a Reeb trajectory, lying on one facet.
#[derive(Clone, Debug)]
pub struct ReebSegment {
    /// Starting point in R^4 (on the facet boundary).
    pub start: Vector4<f64>,

    /// Ending point in R^4 (on the ridge between this facet and the next).
    pub end: Vector4<f64>,

    /// Index of the facet this segment lies on.
    pub facet: usize,
}

/// A piecewise-linear Reeb trajectory on the boundary of a polytope.
///
/// Each segment follows the Reeb direction J_0 a_i on its facet. Consecutive
/// segments share endpoints (continuity at ridges).
#[derive(Clone, Debug)]
pub struct ReebTrajectory {
    /// Ordered list of linear segments.
    pub segments: Vec<ReebSegment>,

    /// Whether the trajectory returned near its starting point
    /// (within the closure tolerance).
    pub closed: bool,
}

/// Compute the Reeb flow direction on a facet: J_0 a.
///
/// The full Reeb vector field is R = 2 J_0 a, but for trajectory
/// simulation we only need the direction J_0 a (the factor 2 rescales
/// the time parametrization without changing the trajectory shape).
///
/// In coordinates (q_1, q_2, p_1, p_2) with J_0 = [[0, -I_2], [I_2, 0]]:
///   J_0 (a, b, c, d) = (-c, -d, a, b)
///
/// [def:reeb-vector-field]: Reeb direction on facet F_i is J_0 a_i (tangent to F_i).
pub fn reeb_direction(dual_vertex: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(
        -dual_vertex[2],
        -dual_vertex[3],
        dual_vertex[0],
        dual_vertex[1],
    )
}

/// Forward-simulate a Reeb trajectory with default parameters.
///
/// Equivalent to `simulate_with(polytope, start_point, start_facet,
/// DEFAULT_MAX_SEGMENTS, DEFAULT_CLOSURE_TOL)`.
///
/// See [`simulate_with`] for full documentation.
pub fn simulate(
    polytope: &Polytope4D,
    start_point: Vector4<f64>,
    start_facet: usize,
) -> ReebTrajectory {
    simulate_with(
        polytope,
        start_point,
        start_facet,
        DEFAULT_MAX_SEGMENTS,
        DEFAULT_CLOSURE_TOL,
    )
}

/// Forward-simulate a Reeb trajectory with explicit parameters.
///
/// Starting at `start_point` on `start_facet`, follows the Reeb direction
/// J_0 a_i on each facet until hitting a neighboring facet, then switches.
/// Stops after `max_segments` segments or when returning within `closure_tol`
/// of the starting point.
///
/// # Algorithm
///
/// On facet F_i with Reeb direction d_i = J_0 a_i, the trajectory is:
///   x(t) = x_0 + t * d_i
///
/// The trajectory exits F_i when it hits a neighboring facet F_j:
///   a_j . (x_0 + t * d_i) = 1
///   t = (1 - a_j . x_0) / (a_j . d_i)
///
/// Take the smallest positive t among all facets j != i.
///
/// **Ridge-point handling:** When the current point sits on the boundary
/// of another facet F_j (a_j . x ~ 1) and the Reeb direction pushes
/// through it (a_j . d > 0), the trajectory cannot proceed on the current
/// facet. It immediately transitions to F_j (zero-length segment).
///
/// [lem:piecewise-linear-reeb]: Reeb trajectories on polytope boundaries are piecewise linear.
pub fn simulate_with(
    polytope: &Polytope4D,
    start_point: Vector4<f64>,
    start_facet: usize,
    max_segments: usize,
    closure_tol: f64,
) -> ReebTrajectory {
    let duals = polytope.dual_vertices_f64();
    let n_facets = duals.len();

    let mut segments = Vec::new();
    let mut current_point = start_point;
    let mut current_facet = start_facet;
    let mut closed = false;

    for _ in 0..max_segments {
        // Phase 1: Handle immediate transitions at ridge points.
        //
        // If we're on the boundary of facet F_j (a_j . x ~ 1) and the Reeb
        // direction pushes through (a_j . d > 0), we must transition to F_j
        // before proceeding. Safety bound: at most n_facets transitions.
        for _ in 0..n_facets {
            let r = reeb_direction(&duals[current_facet]);
            let mut best_immediate = None;
            let mut best_denom = 0.0_f64;
            for (fj, a_j) in duals.iter().enumerate() {
                if fj == current_facet {
                    continue;
                }
                let residual = a_j.dot(&current_point) - 1.0;
                let denom = a_j.dot(&r);
                // On the boundary of fj AND Reeb pushes through it
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

        // Phase 2: Find the exit — smallest positive t among all other facets.
        let reeb = reeb_direction(&duals[current_facet]);
        let mut best_t = f64::INFINITY;
        let mut next_facet = current_facet;

        for (fj, a_j) in duals.iter().enumerate() {
            if fj == current_facet {
                continue;
            }
            let denom = a_j.dot(&reeb);
            if denom.abs() < EPS_DENOM {
                continue; // Reeb direction nearly parallel to this facet
            }
            let t = (1.0 - a_j.dot(&current_point)) / denom;
            if t > EPS_FACET_INCIDENCE && t < best_t {
                best_t = t;
                next_facet = fj;
            }
        }

        if best_t == f64::INFINITY || next_facet == current_facet {
            break; // No valid transition found
        }

        let end_point = current_point + best_t * reeb;

        // Validate: end point must satisfy all half-space constraints (within tolerance).
        let mut valid = true;
        for a_k in duals {
            let violation = a_k.dot(&end_point) - 1.0;
            if violation > EPS_FACET_INCIDENCE * 100.0 {
                valid = false;
                break;
            }
        }
        if !valid {
            break;
        }

        segments.push(ReebSegment {
            start: current_point,
            end: end_point,
            facet: current_facet,
        });

        // Check for closure: trajectory returned near start point.
        if segments.len() >= 2 && (end_point - start_point).norm() < closure_tol {
            closed = true;
            break;
        }

        current_point = end_point;
        current_facet = next_facet;
    }

    ReebTrajectory { segments, closed }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::skeleton::Skeleton;
    use crate::geom::symplectic_form::j4;

    // Tests for reeb_trajectory: piecewise-linear Reeb flow simulation.
    //
    // Proposition: The simulated trajectory follows the Reeb direction J_0 a_i
    // on each facet, transitions at ridges, and stays inside the polytope.
    // Reference: [def:reeb-vector-field], [lem:piecewise-linear-reeb]
    //
    // Strategy: fixture-based on known polytopes (simplex, hypercube, cross-polytope,
    // Lagrangian products). Tests verify direction, continuity, containment.

    /// J_0 (a, b, c, d) = (-c, -d, a, b) for standard basis vectors.
    #[test]
    fn reeb_direction_axis_aligned() {
        let cases = [
            (
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
            ),
            (
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ),
            (
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(-1.0, 0.0, 0.0, 0.0),
            ),
            (
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                Vector4::new(0.0, -1.0, 0.0, 0.0),
            ),
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

    /// J_0 a . a = omega_0(a, a) = 0 (antisymmetry of the symplectic form).
    #[test]
    fn reeb_direction_tangent_to_facet() {
        let duals = [
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.5, 0.5, 0.5, 0.5),
            Vector4::new(0.8, -0.3, 0.2, 0.5),
        ];
        for a in &duals {
            let r = reeb_direction(a);
            let dot = r.dot(a);
            assert!(
                dot.abs() < 1e-12,
                "Reeb direction not tangent: a={a:?}, R.a={dot}"
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
        let residual = (kp.polytope.dual_vertices_f64()[0].dot(&start) - 1.0).abs();
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
        let duals = kp.polytope.dual_vertices_f64();
        for seg in &traj.segments {
            let a = &duals[seg.facet];
            let start_res = (a.dot(&seg.start) - 1.0).abs();
            let end_res = (a.dot(&seg.end) - 1.0).abs();
            assert!(
                start_res < 1e-7,
                "start not on facet {}: {start_res}",
                seg.facet
            );
            assert!(end_res < 1e-7, "end not on facet {}: {end_res}", seg.facet);
        }
    }

    /// Each segment direction is parallel to J_0 a_i (the Reeb direction on its facet).
    #[test]
    fn segments_follow_reeb_direction() {
        let kp = known_polytopes::hypercube();
        let skel = Skeleton::compute(&kp.polytope);
        let start = skel.facet_centroid(&kp.polytope, 0);
        let traj = simulate_with(&kp.polytope, start, 0, 20, 1e-6);

        let duals = kp.polytope.dual_vertices_f64();
        for seg in &traj.segments {
            let direction = seg.end - seg.start;
            if direction.norm() < 1e-12 {
                continue; // zero-length immediate transition
            }
            let expected = reeb_direction(&duals[seg.facet]);
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
            let duals = kp.polytope.dual_vertices_f64();
            let skel = Skeleton::compute(&kp.polytope);

            for facet in 0..duals.len() {
                let start = skel.facet_centroid(&kp.polytope, facet);
                let traj = simulate_with(&kp.polytope, start, facet, 100, 1e-6);

                for (si, seg) in traj.segments.iter().enumerate() {
                    for (label, pt) in [("start", &seg.start), ("end", &seg.end)] {
                        for (fk, ak) in duals.iter().enumerate() {
                            let violation = ak.dot(pt) - 1.0;
                            assert!(
                                violation < 1e-6,
                                "{name} facet {facet} seg {si} {label}: \
                                 violates facet {fk} by {violation:.2e}"
                            );
                        }
                    }

                    // Segment start/end lies on claimed facet's hyperplane.
                    let a_f = &duals[seg.facet];
                    let res_start = (a_f.dot(&seg.start) - 1.0).abs();
                    let res_end = (a_f.dot(&seg.end) - 1.0).abs();
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
        let kp = known_polytopes::hypercube();
        let skel = Skeleton::compute(&kp.polytope);
        let start = skel.facet_centroid(&kp.polytope, 0);

        let traj_default = simulate(&kp.polytope, start, 0);
        let traj_explicit = simulate_with(
            &kp.polytope,
            start,
            0,
            DEFAULT_MAX_SEGMENTS,
            DEFAULT_CLOSURE_TOL,
        );

        assert_eq!(traj_default.segments.len(), traj_explicit.segments.len());
        assert_eq!(traj_default.closed, traj_explicit.closed);

        for (a, b) in traj_default
            .segments
            .iter()
            .zip(traj_explicit.segments.iter())
        {
            assert_eq!(a.facet, b.facet);
            assert!((a.start - b.start).norm() < 1e-14);
            assert!((a.end - b.end).norm() < 1e-14);
        }
    }
}
