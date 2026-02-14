/// Forward-simulation of Reeb trajectories on a polytope boundary.
///
/// On facet Fᵢ with outward unit normal nᵢ, the Reeb vector is J₀nᵢ.
/// This is automatically tangent to Fᵢ since ω₀(nᵢ, nᵢ) = 0,
/// i.e. ⟨J₀nᵢ, nᵢ⟩ = ω₀(nᵢ, nᵢ) = 0.
///
/// A trajectory starts at a point on a facet, follows the Reeb direction
/// until hitting a ridge (shared 2-face with a neighboring facet), then
/// switches to the new facet's Reeb vector. This produces a piecewise-linear
/// curve on ∂K.
use crate::polytope::Polytope4D;
use crate::skeleton::Skeleton;
use nalgebra::Vector4;

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

/// Compute the Reeb vector on facet with normal `n`: R = J₀ n.
///
/// In coordinates (q₁, q₂, p₁, p₂) with J₀ = \[\[0, -I₂\], \[I₂, 0\]\]:
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
/// On facet Fᵢ with Reeb vector Rᵢ = J₀nᵢ:
///   x(t) = x₀ + t · Rᵢ
///
/// The trajectory exits Fᵢ when it hits a neighboring facet Fⱼ:
///   nⱼ · (x₀ + t · Rᵢ) = hⱼ
///   t = (hⱼ - nⱼ · x₀) / (nⱼ · Rᵢ)
///
/// Take the smallest positive t among all neighbors of Fᵢ (via ridges).
pub fn simulate(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    start_point: Vector4<f64>,
    start_facet: usize,
    max_segments: usize,
    closure_tol: f64,
) -> ReebTrajectory {
    let normals = polytope.normals();
    let heights = polytope.heights();

    let mut segments = Vec::new();
    let mut current_point = start_point;
    let mut current_facet = start_facet;
    let mut closed = false;

    for _ in 0..max_segments {
        let reeb = reeb_vector(&normals[current_facet]);

        // Find neighbor facets sharing a ridge with current_facet
        let neighbors: Vec<usize> = skeleton
            .ridges
            .iter()
            .filter_map(|r| {
                if r.facets[0] == current_facet {
                    Some(r.facets[1])
                } else if r.facets[1] == current_facet {
                    Some(r.facets[0])
                } else {
                    None
                }
            })
            .collect();

        // Find smallest positive t where nⱼ · (x + t·R) = hⱼ
        let mut best_t = f64::INFINITY;
        let mut next_facet = current_facet;

        for &fj in &neighbors {
            let denom = normals[fj].dot(&reeb);
            if denom.abs() < 1e-15 {
                continue; // Reeb parallel to this facet plane
            }
            let t = (heights[fj] - normals[fj].dot(&current_point)) / denom;
            if t > 1e-12 && t < best_t {
                best_t = t;
                next_facet = fj;
            }
        }

        if best_t == f64::INFINITY || next_facet == current_facet {
            break; // No valid transition
        }

        let end_point = current_point + best_t * reeb;
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
