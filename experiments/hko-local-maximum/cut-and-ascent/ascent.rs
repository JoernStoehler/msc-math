//! Local ascent/search helpers for the cut-and-ascent HKO experiment.
//!
//! These stay experiment-local because they preserve the current experiment's
//! search policy rather than defining a durable library API.

use crate::{
    CONVERGENCE_THRESHOLD, EPS, MAX_ESCAPE_ROUNDS, MAX_ITERATIONS, MAX_STEP_SIZE, N_WIGGLES,
    OVERSHOOT_MULTIPLIERS, STEP_FRACTIONS, WIGGLE_STRENGTH,
};
use exp_hko_local_maximum::euclidean_volume_f64;
use nalgebra::{Matrix4, Vector4};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use std::time::Instant;
use symplectic::derivatives::{capacity_derivatives_a_from_kkt_result, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::saddle_point_solver::solve_kkt_for_dual_vertices;

/// Compute the first boundary event along a direction in dual-vertex space.
///
/// [lem:step-bound-incidence] incidence flip detection,
/// [lem:step-bound-omega] omega_0 flip detection.
fn compute_step_bound(polytope: &Polytope4D, direction: &[Vector4<f64>]) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let facet_count = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
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

            for j in 0..facet_count {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - duals[j].dot(v);
                let rate = -direction[j].dot(v) - duals[j].dot(&dv_dt);
                if rate < -EPS {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
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
                if max_rate > EPS {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
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

        let roots = if a_coeff.abs() > EPS {
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
        } else if b.abs() > EPS {
            vec![-c / b]
        } else {
            vec![]
        };

        for t_flip in roots {
            if t_flip > EPS && t_flip < t_max {
                t_max = t_flip;
            }
        }
    }

    for k in 0..facet_count {
        let a_coeff = direction[k].norm_squared();
        let b = 2.0 * duals[k].dot(&direction[k]);
        let c = duals[k].norm_squared();
        let disc = b * b - 4.0 * a_coeff * c;
        if disc >= 0.0 && a_coeff > EPS {
            let sqrt_disc = disc.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t_crit = (-b + sign * sqrt_disc) / (2.0 * a_coeff);
                if t_crit > EPS && t_crit < t_max {
                    t_max = t_crit;
                }
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

pub(crate) fn compute_sys(polytope: &Polytope4D) -> Option<f64> {
    let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());
    if vol <= 0.0 {
        return None;
    }
    let cap = compute_capacity(polytope)?;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some(sys)
}

fn try_step_a(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = Polytope4D::from_f64(new_duals).ok()?;
    let sys = compute_sys(&polytope)?;
    Some((polytope, sys))
}

fn compute_capacity(polytope: &Polytope4D) -> Option<f64> {
    symplectic::ehz_capacity(polytope)
        .ok()
        .map(|r| r.capacity())
}

fn compute_capacity_result(polytope: &Polytope4D) -> Option<(f64, Vec<usize>)> {
    let r = symplectic::ehz_capacity(polytope).ok()?;
    Some((r.capacity(), r.best_sigma().to_vec()))
}

/// Single gradient ascent phase: iterate until convergence or budget.
// TODO: add [lem:sys-sensitivity] to formal math (see gradient-correctness experiment)
fn gradient_ascent_phase(
    start: &Polytope4D,
    t0: Instant,
    budget: f64,
) -> Option<(Polytope4D, f64, usize)> {
    let mut current = Polytope4D::from_f64(start.dual_vertices_f64().to_vec()).ok()?;
    let mut current_sys = compute_sys(&current)?;
    let mut n_iters = 0usize;

    for iter in 0..MAX_ITERATIONS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }

        let (cap, best_perm) = compute_capacity_result(&current)?;
        let dual_vertices = current.dual_vertices_f64();
        let kkt = solve_kkt_for_dual_vertices(dual_vertices, &best_perm).feasible()?;
        let vol = euclidean_volume_f64(current.vertices(), current.incidence());
        if vol <= 0.0 {
            return None;
        }
        let sys = cap * cap / (2.0 * vol);
        let duals = current.dual_vertices_f64();

        let d_vol_a = volume_derivatives_a(&current);
        let d_cap_a = capacity_derivatives_a_from_kkt_result(&current, &best_perm, &kkt);
        let d_sys_a: Vec<Vector4<f64>> = d_vol_a
            .iter()
            .zip(d_cap_a.iter())
            .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
            .collect();

        let gradient_norm = d_sys_a.iter().map(|d| d.norm_squared()).sum::<f64>().sqrt();
        if gradient_norm < EPS {
            break;
        }

        let t_max = compute_step_bound(&current, &d_sys_a);
        if t_max <= 0.0 {
            break;
        }

        let mut best: Option<(Polytope4D, f64)> = None;

        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys));
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys));
                    }
                }
            }
        }

        match best {
            Some((new_polytope, new_sys)) => {
                let delta = new_sys - sys;
                current = new_polytope;
                current_sys = new_sys;
                n_iters = iter + 1;
                if delta < CONVERGENCE_THRESHOLD {
                    break;
                }
            }
            None => break,
        }
    }

    Some((current, current_sys, n_iters))
}

fn wiggle(polytope: &Polytope4D, rng: &mut ChaCha8Rng) -> Option<Polytope4D> {
    let duals = polytope.dual_vertices_f64();
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .map(|a| {
            a.map(|c| {
                let noise: f64 = StandardNormal.sample(rng);
                c * (1.0 + WIGGLE_STRENGTH * noise)
            })
        })
        .collect();
    Polytope4D::from_f64(new_duals).ok()
}

pub(crate) struct AscentResult {
    pub(crate) final_polytope: Polytope4D,
    pub(crate) final_sys: f64,
    pub(crate) n_iters: usize,
    pub(crate) n_phases: usize,
}

pub(crate) fn full_ascent(
    start: &Polytope4D,
    rng: &mut ChaCha8Rng,
    budget: f64,
) -> Option<AscentResult> {
    let t0 = Instant::now();

    let (mut best_polytope, mut best_sys, mut total_iters) =
        gradient_ascent_phase(start, t0, budget)?;
    let mut n_phases = 1usize;

    for _ in 0..MAX_ESCAPE_ROUNDS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }
        let mut escaped = false;
        for _ in 0..N_WIGGLES {
            if t0.elapsed().as_secs_f64() > budget {
                break;
            }
            if let Some(wiggled) = wiggle(&best_polytope, rng) {
                if let Some((p, s, iters)) = gradient_ascent_phase(&wiggled, t0, budget) {
                    n_phases += 1;
                    total_iters += iters;
                    if s > best_sys + CONVERGENCE_THRESHOLD {
                        best_sys = s;
                        best_polytope = p;
                        escaped = true;
                        break;
                    }
                }
            }
        }
        if !escaped {
            break;
        }
    }

    Some(AscentResult {
        final_polytope: best_polytope,
        final_sys: best_sys,
        n_iters: total_iters,
        n_phases,
    })
}
