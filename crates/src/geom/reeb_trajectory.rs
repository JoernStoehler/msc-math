//! Piecewise-linear Reeb flow simulation on a polytope boundary.
//!
//! On facet F_i with outward unit normal n_i and height h_i, the Reeb vector
//! field is R_i = (2/h_i) J_0 n_i. The direction J_0 n_i is automatically
//! tangent to F_i since omega_0(n_i, n_i) = 0.
//!
//! For trajectory visualization we only need the *direction* J_0 n_i (the
//! factor 2/h_i rescales time but does not change the trajectory shape).
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

/// Tolerance for ray-facet intersection denominator: n_j . R_i.
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
/// Each segment follows the Reeb direction J_0 n_i on its facet. Consecutive
/// segments share endpoints (continuity at ridges).
#[derive(Clone, Debug)]
pub struct ReebTrajectory {
    /// Ordered list of linear segments.
    pub segments: Vec<ReebSegment>,

    /// Whether the trajectory returned near its starting point
    /// (within the closure tolerance).
    pub closed: bool,
}

/// Compute the Reeb flow direction on a facet: J_0 n.
///
/// The full Reeb vector field is R = (2/h) J_0 n, but for trajectory
/// simulation we only need the direction J_0 n (the 2/h factor rescales
/// the time parametrization without changing the trajectory shape).
///
/// In coordinates (q_1, q_2, p_1, p_2) with J_0 = [[0, -I_2], [I_2, 0]]:
///   J_0 (a, b, c, d) = (-c, -d, a, b)
///
/// [def:reeb-vector-field]
pub fn reeb_direction(normal: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-normal[2], -normal[3], normal[0], normal[1])
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
/// J_0 n_i on each facet until hitting a neighboring facet, then switches.
/// Stops after `max_segments` segments or when returning within `closure_tol`
/// of the starting point.
///
/// # Algorithm
///
/// On facet F_i with Reeb direction d_i = J_0 n_i, the trajectory is:
///   x(t) = x_0 + t * d_i
///
/// The trajectory exits F_i when it hits a neighboring facet F_j:
///   n_j . (x_0 + t * d_i) = h_j
///   t = (h_j - n_j . x_0) / (n_j . d_i)
///
/// Take the smallest positive t among all facets j != i.
///
/// **Ridge-point handling:** When the current point sits on the boundary
/// of another facet F_j (n_j . x ~ h_j) and the Reeb direction pushes
/// through it (n_j . d > 0), the trajectory cannot proceed on the current
/// facet. It immediately transitions to F_j (zero-length segment).
///
/// [lem:piecewise-linear-reeb]
pub fn simulate_with(
    polytope: &Polytope4D,
    start_point: Vector4<f64>,
    start_facet: usize,
    max_segments: usize,
    closure_tol: f64,
) -> ReebTrajectory {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let n_facets = normals.len();

    let mut segments = Vec::new();
    let mut current_point = start_point;
    let mut current_facet = start_facet;
    let mut closed = false;

    for _ in 0..max_segments {
        // Phase 1: Handle immediate transitions at ridge points.
        //
        // If we're on the boundary of facet F_j (n_j . x ~ h_j) and the Reeb
        // direction pushes through (n_j . d > 0), we must transition to F_j
        // before proceeding. Safety bound: at most n_facets transitions.
        for _ in 0..n_facets {
            let r = reeb_direction(&normals[current_facet]);
            let mut best_immediate = None;
            let mut best_denom = 0.0_f64;
            for fj in 0..n_facets {
                if fj == current_facet {
                    continue;
                }
                let residual = normals[fj].dot(&current_point) - heights[fj];
                let denom = normals[fj].dot(&r);
                // On the boundary of fj AND Reeb pushes through it
                if residual.abs() < EPS_FACET_INCIDENCE && denom > EPS_DENOM {
                    if denom > best_denom {
                        best_denom = denom;
                        best_immediate = Some(fj);
                    }
                }
            }
            if let Some(fj) = best_immediate {
                current_facet = fj;
            } else {
                break;
            }
        }

        // Phase 2: Find the exit — smallest positive t among all other facets.
        let reeb = reeb_direction(&normals[current_facet]);
        let mut best_t = f64::INFINITY;
        let mut next_facet = current_facet;

        for fj in 0..n_facets {
            if fj == current_facet {
                continue;
            }
            let denom = normals[fj].dot(&reeb);
            if denom.abs() < EPS_DENOM {
                continue; // Reeb direction nearly parallel to this facet
            }
            let t = (heights[fj] - normals[fj].dot(&current_point)) / denom;
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
        for fk in 0..n_facets {
            let violation = normals[fk].dot(&end_point) - heights[fk];
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
