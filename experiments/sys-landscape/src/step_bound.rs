//! Step-bound event detection for dual-vertex ascent directions.

use euclidean_polytopes::{
    two_faces_from_vertex_facet_incidence, vertex_facets_from_vertex_facet_incidence,
};
use nalgebra::{Matrix4, Vector4};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic_form::omega0;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
/// Used both by `compute_step_bound_detailed` for "unbounded" classification and by
/// the ascent binaries' overshoot guard (`if t_max < MAX_STEP_SIZE`). Must be a single
/// source of truth -- if tuned, both semantics must agree.
pub const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate. Used in step bounds.
/// If changed: values much larger risk missing real boundaries; much smaller risks
/// false positives from floating-point noise.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Classification of a combinatorial boundary event.
#[derive(Debug, Clone)]
pub enum EventType {
    /// A vertex's slack with respect to a non-incident facet reaches zero.
    IncidenceFlip {
        vertex_index: usize,
        new_facet: usize,
    },
    /// sign(omega_0(a_i, a_j)) changes for facets adjacent along a two-face.
    OmegaFlip { facet_i: usize, facet_j: usize },
    /// |a_k + t*d_k| -> 0 (dual vertex degenerates).
    DualVertexDegen { facet: usize },
    /// t_max was capped at MAX_STEP_SIZE (no real boundary found).
    Unbounded,
}

/// First boundary event along a direction in dual-vertex space.
#[derive(Debug, Clone)]
pub struct BoundaryEvent {
    /// Step size at which the boundary is crossed.
    pub t_max: f64,
    /// What type of boundary event occurs.
    pub event: EventType,
}

/// Compute the first boundary event along a direction in dual-vertex space.
/// [lem:step-bound-incidence] incidence flip detection, [lem:step-bound-omega] omega_0 flip detection
///
/// For step a'_k(t) = a_k + t*d_k, the combinatorial type changes when:
/// 1. **Incidence flip:** a vertex's slack w.r.t. a non-incident facet reaches zero.
/// 2. **omega_0 flip:** sign(omega_0(a_i, a_j)) changes for two-face-adjacent facets.
/// 3. **Dual vertex degeneration:** |a_k + t*d_k| -> 0.
///
/// Source: `research/combinatorial-cells.md`.
/// Local combinatorial-cells probes showed that omega_0 sign flips make up a
/// substantial share of first boundary events, so the old
/// `compute_step_bound_a` missed an important failure mode.
pub fn compute_step_bound_detailed(
    polytope: &Polytope4D,
    direction: &[Vector4<f64>],
) -> BoundaryEvent {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let incidence = polytope.incidence();
    let vertex_facets_by_vertex = vertex_facets_from_vertex_facet_incidence(incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(incidence);

    let mut best = BoundaryEvent {
        t_max: f64::INFINITY,
        event: EventType::Unbounded,
    };

    // --- Vertex-facet incidence checks ---
    for (vi, vertex_facets) in vertex_facets_by_vertex.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let a_mat = Matrix4::from_rows(&[
                duals[vertex_facets[0]].transpose(),
                duals[vertex_facets[1]].transpose(),
                duals[vertex_facets[2]].transpose(),
                duals[vertex_facets[3]].transpose(),
            ]);

            let a_inv = match a_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                direction[vertex_facets[0]].dot(v),
                direction[vertex_facets[1]].dot(v),
                direction[vertex_facets[2]].dot(v),
                direction[vertex_facets[3]].dot(v),
            );

            let dv_dt = -(a_inv * rhs);

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - duals[j].dot(v);
                let rate = -direction[j].dot(v) - duals[j].dot(&dv_dt);
                if rate < -EPS_NUMERICAL_ZERO {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < best.t_max {
                        best = BoundaryEvent {
                            t_max: t_crit,
                            event: EventType::IncidenceFlip {
                                vertex_index: vi,
                                new_facet: j,
                            },
                        };
                    }
                }
            }
        } else {
            // Non-simple vertex (>4 incident facets). Conservative bound.
            let max_d = direction.iter().map(|dk| dk.norm()).fold(0.0f64, f64::max);
            for (j, a_j) in duals.iter().enumerate() {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - a_j.dot(v);
                let max_rate = max_d * v.norm() + a_j.norm() * max_d * v.norm();
                if max_rate > EPS_NUMERICAL_ZERO {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < best.t_max {
                        best = BoundaryEvent {
                            t_max: t_crit,
                            event: EventType::IncidenceFlip {
                                vertex_index: vi,
                                new_facet: j,
                            },
                        };
                    }
                }
            }
        }
    }

    // --- omega_0 sign preservation for adjacent two-face facet pairs ---
    for two_face in &two_faces {
        let i = two_face.facets[0];
        let j = two_face.facets[1];
        let c = omega0(&duals[i], &duals[j]);
        let b = omega0(&direction[i], &duals[j]) + omega0(&duals[i], &direction[j]);
        let a_coeff = omega0(&direction[i], &direction[j]);

        let roots = if a_coeff.abs() > EPS_NUMERICAL_ZERO {
            let disc = b * b - 4.0 * a_coeff * c;
            if disc < 0.0 {
                vec![]
            } else {
                let sqrt_disc = disc.sqrt();
                vec![
                    (-b - sqrt_disc) / (2.0 * a_coeff),
                    (-b + sqrt_disc) / (2.0 * a_coeff),
                ]
            }
        } else if b.abs() > EPS_NUMERICAL_ZERO {
            vec![-c / b]
        } else {
            vec![]
        };

        for t_flip in roots {
            if t_flip > EPS_NUMERICAL_ZERO && t_flip < best.t_max {
                best = BoundaryEvent {
                    t_max: t_flip,
                    event: EventType::OmegaFlip {
                        facet_i: i,
                        facet_j: j,
                    },
                };
            }
        }
    }

    // --- Dual vertex degeneration: |a_k + t*d_k| -> 0 ---
    for k in 0..f {
        let a_coeff = direction[k].norm_squared();
        let b = 2.0 * duals[k].dot(&direction[k]);
        let c = duals[k].norm_squared();
        let disc = b * b - 4.0 * a_coeff * c;
        if disc >= 0.0 && a_coeff > EPS_NUMERICAL_ZERO {
            let sqrt_disc = disc.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t_crit = (-b + sign * sqrt_disc) / (2.0 * a_coeff);
                if t_crit > EPS_NUMERICAL_ZERO && t_crit < best.t_max {
                    best = BoundaryEvent {
                        t_max: t_crit,
                        event: EventType::DualVertexDegen { facet: k },
                    };
                }
            }
        }
    }

    if best.t_max > MAX_STEP_SIZE {
        best = BoundaryEvent {
            t_max: MAX_STEP_SIZE,
            event: EventType::Unbounded,
        };
    }

    best
}

/// Convenience wrapper: returns just the step bound (t_max) without event details.
///
/// Drop-in replacement for the old `compute_step_bound_a`. Callers that only need
/// the scalar step bound use this; callers that need to classify boundary events
/// use `compute_step_bound_detailed` directly.
pub fn compute_step_bound(polytope: &Polytope4D, direction: &[Vector4<f64>]) -> f64 {
    compute_step_bound_detailed(polytope, direction).t_max
}
