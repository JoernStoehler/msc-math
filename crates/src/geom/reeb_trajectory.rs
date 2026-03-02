/// Forward-simulation of Reeb trajectories on a polytope boundary.
///
/// On facet Fᵢ with outward unit normal nᵢ and height hᵢ, the Reeb vector
/// field is R_i = (2/h_i) J₀ n_i. The direction J₀ n_i is automatically
/// tangent to Fᵢ since ω₀(nᵢ, nᵢ) = 0, i.e. ⟨J₀nᵢ, nᵢ⟩ = 0.
///
/// For visualization purposes, we only need the *direction* J₀ n_i
/// (the factor 2/h_i rescales time but does not change the trajectory shape).
///
/// A trajectory starts at a point on a facet, follows the Reeb direction
/// until hitting a ridge (shared 2-face with a neighboring facet), then
/// switches to the new facet's Reeb direction. This produces a piecewise-linear
/// curve on ∂K.
use crate::constants::EPS_FACET_INCIDENCE;
use crate::geom::polytope::Polytope4D;
use crate::geom::skeleton::Skeleton;
use nalgebra::Vector4;

/// Tolerance for ray-facet intersection denominator: ⟨Reeb direction, normal⟩.
///
/// When the Reeb direction is nearly parallel to a facet (dot product near zero),
/// the intersection time t = distance / dot becomes numerically unstable.
/// **Why 1e-10:** The Reeb direction and normals are both O(1), so genuine
/// "pushing through" transitions have dot products well above 1e-10. Near-parallel
/// facets (dot < 1e-10) are skipped to avoid spurious intersections.
const EPS_DENOM: f64 = 1e-10;

/// A single linear segment of a Reeb trajectory, lying on one facet.
#[derive(Clone, Debug)]
pub struct ReebSegment {
    /// Starting point in R⁴ (on facet boundary).
    pub start: Vector4<f64>,
    /// Ending point in R⁴ (on ridge between this facet and next).
    pub end: Vector4<f64>,
    /// Facet index this segment lies on.
    pub facet: usize,
}

/// A piecewise-linear Reeb trajectory on the boundary of a polytope.
#[derive(Clone, Debug)]
pub struct ReebTrajectory {
    pub segments: Vec<ReebSegment>,
    /// Whether the trajectory returned near its starting point.
    pub closed: bool,
}

/// Compute the Reeb flow direction on a facet: J₀ n.
///
/// The full Reeb vector field on facet with normal n and height h is
/// R = (2/h) J₀ n, but for trajectory visualization we only need the
/// direction J₀ n (the factor 2/h rescales time parametrization).
///
/// In coordinates (q₁, q₂, p₁, p₂) with J₀ = [[0, -I₂], [I₂, 0]]:
///   J₀ (a, b, c, d) = (-c, -d, a, b)
pub fn reeb_vector(normal: &Vector4<f64>) -> Vector4<f64> {
    // Direct computation avoids matrix multiply:
    // J₀ n = (-n[2], -n[3], n[0], n[1])
    Vector4::new(-normal[2], -normal[3], normal[0], normal[1])
}

/// Forward-simulate a Reeb trajectory.
///
/// Starting at `start_point` on `start_facet`, follows the Reeb direction
/// J₀nᵢ on each facet until hitting a neighboring facet, then switches.
///
/// Stops after `max_segments` segments or when returning within
/// `closure_tol` of the starting point.
///
/// # Algorithm
///
/// On facet Fᵢ with Reeb direction dᵢ = J₀nᵢ:
///   x(t) = x₀ + t · dᵢ
///
/// The trajectory exits Fᵢ when it hits a neighboring facet Fⱼ:
///   nⱼ · (x₀ + t · Rᵢ) = hⱼ
///   t = (hⱼ - nⱼ · x₀) / (nⱼ · Rᵢ)
///
/// Take the smallest positive t among all facets j ≠ i.
///
/// **Ridge-point handling:** When the current point sits on the boundary
/// of another facet Fⱼ (nⱼ·x ≈ hⱼ) and the Reeb direction pushes
/// through it (nⱼ·R > 0), the trajectory cannot proceed on the current
/// facet. It immediately transitions to Fⱼ (zero-length segment).
pub fn simulate(
    polytope: &Polytope4D,
    start_point: Vector4<f64>,
    start_facet: usize,
    max_segments: usize,
    closure_tol: f64,
) -> ReebTrajectory {
    let normals = polytope.normals();
    let heights = polytope.heights();
    let n_facets = normals.len();

    let mut segments = Vec::new();
    let mut current_point = start_point;
    let mut current_facet = start_facet;
    let mut closed = false;

    for _ in 0..max_segments {
        let reeb = reeb_vector(&normals[current_facet]);

        // Phase 1: Check for immediate transition.
        // If we're on the boundary of facet Fⱼ (nⱼ·x ≈ hⱼ) and the Reeb
        // direction pushes us through (nⱼ·R > 0), we must transition to Fⱼ
        // before proceeding.  This happens at ridges where the Reeb flow
        // on the current facet would immediately leave the polytope.
        let mut did_immediate = false;
        // Safety bound: at most n_facets immediate transitions (prevents infinite loop)
        for _ in 0..n_facets {
            let r = reeb_vector(&normals[current_facet]);
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
                    // Pick the facet where the Reeb pushes through most strongly
                    if denom > best_denom {
                        best_denom = denom;
                        best_immediate = Some(fj);
                    }
                }
            }
            if let Some(fj) = best_immediate {
                current_facet = fj;
                did_immediate = true;
            } else {
                break;
            }
        }
        // After immediate transitions, recompute the Reeb vector
        let reeb = if did_immediate {
            reeb_vector(&normals[current_facet])
        } else {
            reeb
        };

        // Phase 2: Find the exit — smallest positive t among ALL other facets.
        let mut best_t = f64::INFINITY;
        let mut next_facet = current_facet;

        for fj in 0..n_facets {
            if fj == current_facet {
                continue;
            }
            let denom = normals[fj].dot(&reeb);
            if denom.abs() < EPS_DENOM {
                continue; // Reeb parallel to this facet plane
            }
            let t = (heights[fj] - normals[fj].dot(&current_point)) / denom;
            if t > EPS_FACET_INCIDENCE && t < best_t {
                best_t = t;
                next_facet = fj;
            }
        }

        if best_t == f64::INFINITY || next_facet == current_facet {
            break; // No valid transition
        }

        let end_point = current_point + best_t * reeb;

        // Validate: end point must satisfy all half-space constraints
        // (within tolerance). If not, clamp to the polytope boundary.
        let mut valid = true;
        for fk in 0..n_facets {
            let violation = normals[fk].dot(&end_point) - heights[fk];
            if violation > EPS_FACET_INCIDENCE * 100.0 {
                // Gross violation — something went wrong
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

        // Check for closure
        if segments.len() >= 2 && (end_point - start_point).norm() < closure_tol {
            closed = true;
            break;
        }

        current_point = end_point;
        current_facet = next_facet;
    }

    ReebTrajectory { segments, closed }
}

/// Compute the centroid of a facet's vertices (useful as a starting point).
pub fn facet_centroid(polytope: &Polytope4D, skeleton: &Skeleton, facet: usize) -> Vector4<f64> {
    let vertices = polytope.vertices();
    let facet_verts: Vec<usize> = skeleton
        .vertex_facets
        .iter()
        .enumerate()
        .filter_map(|(vi, facets)| facets.contains(&facet).then_some(vi))
        .collect();

    if facet_verts.is_empty() {
        return Vector4::zeros();
    }

    facet_verts
        .iter()
        .map(|&vi| vertices[vi])
        .sum::<Vector4<f64>>()
        / facet_verts.len() as f64
}

#[cfg(test)]
#[path = "reeb_trajectory_test.rs"]
mod reeb_trajectory_test;
