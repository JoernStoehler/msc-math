//! Shared helpers for combinatorial-cells experiments.
//!
//! Module architecture:
//! - shared database naming helpers for experiment-local row schemas
//! - shared boundary kernel for dual-vertex step-bound detection
//! - shared polytope reconstruction at `a + t d`
//!
//! Experiments studying the local geometry of combinatorial cells in dual-vertex space:
//! cell widths, boundary characterization, convexity, gradient behavior at boundaries.

use nalgebra::{Matrix4, Vector4};
use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::database::{PolytopeRecord, Source};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE,
};

/// Shared "all valid orbit" summary used by several combinatorial-cells binaries.
///
/// This stays experiment-local because these binaries care about the total valid-orbit
/// count and the best/second-best action gap, which are not yet part of the
/// library's near-minimum collector surface.
#[derive(Debug, Clone)]
pub struct InstrumentedCapacitySummary {
    pub capacity: f64,
    pub best_permutation: Vec<usize>,
    pub n_valid_orbits: usize,
    /// `action_second_best - action_best`. `f64::INFINITY` if there is only one orbit.
    pub orbit_gap: f64,
}

/// Enumerate all valid HK2017 orbits, then return the best action/permutation plus
/// the total valid-orbit count and the best/second-best action gap.
pub fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedCapacitySummary> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);
    let mut orbits: Vec<(f64, Vec<usize>)> = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_feasible_cycle(perm, &adj) {
                    return;
                }

                if let KktOutcome::Feasible(kkt_result) = solve_kkt_for(polytope, perm) {
                    let q_val = kkt_result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = kkt_result
                        .beta
                        .iter()
                        .copied()
                        .fold(f64::INFINITY, f64::min);
                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push((0.5 / q_val, perm.to_vec()));
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.0.total_cmp(&b.0));

    let best_action = orbits[0].0;
    let best_permutation = orbits[0].1.clone();
    let n_valid_orbits = orbits.len();
    let orbit_gap = if orbits.len() >= 2 {
        orbits[1].0 - orbits[0].0
    } else {
        f64::INFINITY
    };

    Some(InstrumentedCapacitySummary {
        capacity: best_action,
        best_permutation,
        n_valid_orbits,
        orbit_gap,
    })
}

/// Classifies the next combinatorial boundary encountered along a direction.
#[derive(Debug, Clone)]
pub enum EventType {
    /// A vertex's slack with respect to a non-incident facet reaches zero.
    IncidenceFlip {
        vertex_index: usize,
        new_facet: usize,
    },
    /// sign(omega_0(a_i, a_j)) changes for ridge-adjacent facets i, j.
    OmegaFlip { facet_i: usize, facet_j: usize },
    /// |a_k + t*d_k| -> 0 (dual vertex degenerates).
    DualVertexDegen { facet: usize },
    /// t_max was capped at the caller-provided maximum.
    Unbounded,
}

impl EventType {
    /// Stable snake_case label used in JSONL output.
    pub fn name(&self) -> &'static str {
        match self {
            EventType::IncidenceFlip { .. } => "incidence_flip",
            EventType::OmegaFlip { .. } => "omega_flip",
            EventType::DualVertexDegen { .. } => "dual_vertex_degen",
            EventType::Unbounded => "unbounded",
        }
    }
}

/// Result of the enriched step-bound computation.
#[derive(Debug, Clone)]
pub struct BoundaryEvent {
    pub t_max: f64,
    pub event: EventType,
}

/// Derive a human-readable name from a database record's `Source`.
pub fn name_from_record(record: &PolytopeRecord, index: usize) -> String {
    match &record.source {
        Some(Source::Random {
            facet_count_target,
            attempt,
            ..
        }) => {
            format!("random_F{facet_count_target}_a{attempt}")
        }
        Some(Source::LagrangianProduct { n1, n2, .. }) => {
            format!("product_{n1}x{n2}_{index}")
        }
        Some(Source::Known { name }) => name.clone(),
        None => format!("polytope_{index}"),
    }
}

/// Derive a source dataset string from a database record's `Source`.
pub fn source_dataset_from_record(record: &PolytopeRecord) -> String {
    match &record.source {
        Some(Source::Random { .. }) => "random-sample".to_string(),
        Some(Source::LagrangianProduct { .. }) => "random-product-sample".to_string(),
        Some(Source::Known { .. }) => "known".to_string(),
        None => "unknown".to_string(),
    }
}

/// Construct a polytope at `a'_k = a_k + t*d_k`.
pub fn construct_at_t(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<Polytope4D> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();
    Polytope4D::from_f64(new_duals).ok()
}

/// Compute the first boundary event along a direction in dual-vertex space.
/// [lem:step-bound-incidence] incidence flip detection.
/// [lem:step-bound-omega] omega_0 sign-flip detection for ridge-adjacent facets.
///
/// The caller supplies the numerical thresholds so experiment-local epsilon policy
/// stays local to the binary that is reporting the row.
pub fn compute_step_bound_detailed(
    polytope: &Polytope4D,
    direction: &[Vector4<f64>],
    eps_numerical_zero: f64,
    max_step_size: f64,
) -> BoundaryEvent {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut best = BoundaryEvent {
        t_max: f64::INFINITY,
        event: EventType::Unbounded,
    };

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let det_facets = vertex_facets;
            let a_mat = Matrix4::from_rows(&[
                duals[det_facets[0]].transpose(),
                duals[det_facets[1]].transpose(),
                duals[det_facets[2]].transpose(),
                duals[det_facets[3]].transpose(),
            ]);

            let a_inv = match a_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                direction[det_facets[0]].dot(v),
                direction[det_facets[1]].dot(v),
                direction[det_facets[2]].dot(v),
                direction[det_facets[3]].dot(v),
            );

            let dv_dt = -(a_inv * rhs);

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - duals[j].dot(v);
                let rate = -direction[j].dot(v) - duals[j].dot(&dv_dt);
                if rate < -eps_numerical_zero {
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
            let max_d = direction.iter().map(|dk| dk.norm()).fold(0.0f64, f64::max);
            for (j, a_j) in duals.iter().enumerate() {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - a_j.dot(v);
                let max_rate = max_d * v.norm() + a_j.norm() * max_d * v.norm();
                if max_rate > eps_numerical_zero {
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

    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let c = omega0(&duals[i], &duals[j]);
        let b = omega0(&direction[i], &duals[j]) + omega0(&duals[i], &direction[j]);
        let a_coeff = omega0(&direction[i], &direction[j]);

        let roots = if a_coeff.abs() > eps_numerical_zero {
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
        } else if b.abs() > eps_numerical_zero {
            vec![-c / b]
        } else {
            vec![]
        };

        for t_flip in roots {
            if t_flip > eps_numerical_zero && t_flip < best.t_max {
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

    for k in 0..f {
        let a_coeff = direction[k].norm_squared();
        let b = 2.0 * duals[k].dot(&direction[k]);
        let c = duals[k].norm_squared();
        let disc = b * b - 4.0 * a_coeff * c;
        if disc >= 0.0 && a_coeff > eps_numerical_zero {
            let sqrt_disc = disc.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t_crit = (-b + sign * sqrt_disc) / (2.0 * a_coeff);
                if t_crit > eps_numerical_zero && t_crit < best.t_max {
                    best = BoundaryEvent {
                        t_max: t_crit,
                        event: EventType::DualVertexDegen { facet: k },
                    };
                }
            }
        }
    }

    if best.t_max > max_step_size {
        best = BoundaryEvent {
            t_max: max_step_size,
            event: EventType::Unbounded,
        };
    }

    best
}
