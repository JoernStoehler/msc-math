//! Phase 2 and 3 of the HKO second-order experiment: curve probes and random curvature checks.

use crate::{EPSILON_GRID, EPSILON_RANDOM, N_RANDOM_DIRECTIONS, RANDOM_SEED};
use exp_hko_local_maximum::euclidean_volume_f64;
use nalgebra::Vector4;
use rand::Rng as _;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::ehz_capacity;
use symplectic::geom::polytope::Polytope4D;

#[derive(Debug, Serialize)]
struct CurveRow {
    direction_index: usize,
    epsilon: f64,
    sys: f64,
    capacity: f64,
    volume: f64,
    delta_sys: f64,
    time_ms: f64,
}

/// Sample a random unit vector in the flat subspace.
fn random_flat_direction(flat_basis: &[Vec<f64>], rng: &mut ChaCha8Rng) -> (Vec<f64>, Vec<f64>) {
    let dim = flat_basis[0].len();
    let coeffs: Vec<f64> = (0..flat_basis.len())
        .map(|_| rng.gen_range(-1.0..1.0))
        .collect();
    let norm_coeffs: f64 = coeffs.iter().map(|c| c * c).sum::<f64>().sqrt();
    let normalized_coeffs: Vec<f64> = coeffs.iter().map(|c| c / norm_coeffs).collect();

    let mut direction = vec![0.0; dim];
    for (i, &normalized_c) in normalized_coeffs.iter().enumerate() {
        for (j, value) in direction.iter_mut().enumerate() {
            *value += normalized_c * flat_basis[i][j];
        }
    }
    (direction, normalized_coeffs)
}

pub(crate) fn curvature_at_epsilon(
    polytope: &Polytope4D,
    direction: &[f64],
    eps: f64,
    sys_base: f64,
) -> Option<f64> {
    let duals = polytope.dual_vertices_f64();
    let facet_count = polytope.facet_count();

    let eval = |sign: f64| -> Option<f64> {
        let e = sign * eps;
        let perturbed: Vec<Vector4<f64>> = (0..facet_count)
            .map(|k| {
                let d_k = Vector4::new(
                    direction[4 * k],
                    direction[4 * k + 1],
                    direction[4 * k + 2],
                    direction[4 * k + 3],
                );
                duals[k] + e * d_k
            })
            .collect();
        let poly = Polytope4D::from_f64(perturbed).ok()?;
        let cap = ehz_capacity(&poly).ok()?.capacity();
        let vol = euclidean_volume_f64(poly.vertices(), poly.incidence());
        if vol <= 0.0 {
            return None;
        }
        Some(cap * cap / (2.0 * vol))
    };

    let sys_plus = eval(1.0)?;
    let sys_minus = eval(-1.0)?;
    Some((sys_plus + sys_minus - 2.0 * sys_base) / (eps * eps))
}

pub(crate) fn run_phase2(
    polytope: &Polytope4D,
    sys_base: f64,
    flat_directions: &[Vec<f64>],
    writer: &mut BufWriter<File>,
) {
    let duals = polytope.dual_vertices_f64();
    let facet_count = polytope.facet_count();

    println!(
        "\n  Evaluating {} directions × {} ε values (×2 for ±) = {} capacity evaluations",
        flat_directions.len(),
        EPSILON_GRID.len(),
        flat_directions.len() * EPSILON_GRID.len() * 2,
    );

    for (dir_idx, direction) in flat_directions.iter().enumerate() {
        let t_dir = Instant::now();
        let mut n_ok = 0;
        let mut n_fail = 0;

        for &eps_abs in EPSILON_GRID {
            for &sign in &[1.0, -1.0] {
                let eps = sign * eps_abs;
                let t_eval = Instant::now();

                let perturbed: Vec<Vector4<f64>> = (0..facet_count)
                    .map(|k| {
                        let d_k = Vector4::new(
                            direction[4 * k],
                            direction[4 * k + 1],
                            direction[4 * k + 2],
                            direction[4 * k + 3],
                        );
                        duals[k] + eps * d_k
                    })
                    .collect();

                let perturbed_poly = match Polytope4D::from_f64(perturbed) {
                    Ok(p) => p,
                    Err(_) => {
                        n_fail += 1;
                        continue;
                    }
                };

                let cap = match ehz_capacity(&perturbed_poly) {
                    Ok(r) => r.capacity(),
                    Err(_) => {
                        n_fail += 1;
                        continue;
                    }
                };

                let vol =
                    euclidean_volume_f64(perturbed_poly.vertices(), perturbed_poly.incidence());
                if vol <= 0.0 {
                    n_fail += 1;
                    continue;
                }

                let sys_val = cap * cap / (2.0 * vol);
                let row = CurveRow {
                    direction_index: dir_idx,
                    epsilon: eps,
                    sys: sys_val,
                    capacity: cap,
                    volume: vol,
                    delta_sys: sys_val - sys_base,
                    time_ms: t_eval.elapsed().as_secs_f64() * 1000.0,
                };
                serde_json::to_writer(&mut *writer, &row).expect("write curve row");
                writeln!(writer).expect("newline");
                n_ok += 1;
            }
        }

        println!(
            "  Direction {dir_idx}: {n_ok} ok, {n_fail} failed, {:.1}s",
            t_dir.elapsed().as_secs_f64()
        );
    }
}

#[derive(Debug, Serialize)]
struct RandomDirectionRow {
    direction_index: usize,
    curvature: f64,
    curvatures_by_eps: Vec<f64>,
    flat_basis_coefficients: Vec<f64>,
    time_ms: f64,
}

pub(crate) fn run_phase3(
    polytope: &Polytope4D,
    sys_base: f64,
    flat_directions: &[Vec<f64>],
    writer: &mut BufWriter<File>,
) {
    let flat_dim = flat_directions.len();
    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED);

    println!(
        "\n  Sampling {} random directions in {}D flat subspace, {} ε values each",
        N_RANDOM_DIRECTIONS,
        flat_dim,
        EPSILON_RANDOM.len(),
    );

    let mut n_negative = 0;
    let mut n_ambiguous = 0;
    let mut n_positive = 0;
    let mut worst_curvature = f64::NEG_INFINITY;

    for dir_idx in 0..N_RANDOM_DIRECTIONS {
        let t_dir = Instant::now();
        let (direction, normalized_coeffs) = random_flat_direction(flat_directions, &mut rng);

        let mut curvatures: Vec<f64> = Vec::new();
        for &eps in EPSILON_RANDOM {
            if let Some(curv) = curvature_at_epsilon(polytope, &direction, eps, sys_base) {
                curvatures.push(curv);
            }
        }

        let median = if curvatures.is_empty() {
            f64::NAN
        } else {
            let mut sorted = curvatures.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted[sorted.len() / 2]
        };

        if median < -1e-6 {
            n_negative += 1;
        } else if median > 1e-6 {
            n_positive += 1;
        } else {
            n_ambiguous += 1;
        }
        if median > worst_curvature {
            worst_curvature = median;
        }

        let row = RandomDirectionRow {
            direction_index: dir_idx,
            curvature: median,
            curvatures_by_eps: curvatures,
            flat_basis_coefficients: normalized_coeffs,
            time_ms: t_dir.elapsed().as_secs_f64() * 1000.0,
        };
        serde_json::to_writer(&mut *writer, &row).expect("write random row");
        writeln!(writer).expect("newline");

        if dir_idx % 20 == 19 {
            println!(
                "  {}/{}: {} negative, {} ambiguous, {} positive, worst={:.4e}",
                dir_idx + 1,
                N_RANDOM_DIRECTIONS,
                n_negative,
                n_ambiguous,
                n_positive,
                worst_curvature
            );
        }
    }

    println!("\n  Summary: {n_negative} negative, {n_ambiguous} ambiguous, {n_positive} positive");
    println!("  Worst (most positive) curvature: {worst_curvature:.4e}");
    if n_positive == 0 {
        println!(
            "  → No positive curvature found among {} random directions",
            N_RANDOM_DIRECTIONS
        );
    } else {
        println!(
            "  → WARNING: {} directions with positive curvature!",
            n_positive
        );
    }
}
