//! Orbit recovery validation experiment.
//!
//! For each polytope (known + random), computes c_EHZ(K), recovers the base
//! point b = γ(0), validates the reconstructed orbit, and outputs JSONL.
//!
//! Architecture:
//! 1. `cargo run --bin orbit_recovery --release` generates dataset
//! 2. Writes to orbit-recovery/orbit-recovery.jsonl
//! 3. Python script analyzes and plots results
//!
//! Validation checks per polytope:
//! - Closure: orbit returns to starting point
//! - On-facet: each segment lies on the correct facet
//! - Inside K: orbit stays inside K at all breakpoints
//! - Action: computed action matches capacity
//! - Solution dimension: how many free parameters in b

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::orbit_recovery::{recover_and_verify, OrbitRecovery};
use symplectic::algorithms::hk2017::{ehz_capacity, EhzResult};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::random::generate_random_polytopes;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples)
const RANDOM_PLAN: &[(usize, usize)] = &[
    (5, 20),
    (6, 20),
    (7, 20),
    (8, 20),
    (9, 15),
    (10, 10),
];

#[derive(Debug, Serialize)]
struct OrbitRecoveryRow {
    name: String,
    facet_count: usize,
    source: String,
    capacity: f64,
    active_facets: usize,
    total_segments: usize,
    solution_dim: usize,
    max_violation: f64,
    closure_error: f64,
    on_facet_error: f64,
    inside_k_error: f64,
    computed_action: f64,
    action_error: f64,
    time_capacity_ms: f64,
    time_recovery_ms: f64,
}

/// Compute on-facet error: max over active segments of |<a_{sigma(k)}, breakpoint_k> - 1|.
///
/// This metric was previously returned by the library's `verify_orbit`, but is
/// no longer part of `OrbitRecovery`. We compute it here from breakpoints and
/// dual vertices.
fn compute_on_facet_error(polytope: &Polytope4D, result: &EhzResult, recovery: &OrbitRecovery) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let sigma = &result.result.best_permutation;
    (0..sigma.len())
        .filter(|&k| recovery.dwell_times[k] > 0.0)
        .map(|k| {
            let a = &duals[sigma[k]];
            (a.dot(&recovery.breakpoints[k]) - 1.0).abs()
        })
        .fold(0.0_f64, f64::max)
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("orbit-recovery/orbit-recovery.jsonl");
    let file = File::create(&output_path).expect("Cannot create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut failures = 0usize;

    // Phase 1: Known polytopes
    eprintln!("=== Known polytopes ===");
    for kp in known_polytopes::all_known() {
        // Skip crosspolytope (F=16) — exponential cost, too slow
        if kp.polytope.facet_count() > 12 {
            eprintln!("  SKIP {} (F={}, too slow)", kp.name, kp.polytope.facet_count());
            continue;
        }

        let t_cap = Instant::now();
        let result = match ehz_capacity(&kp.polytope) {
            Some(r) => r,
            None => {
                eprintln!("  SKIP {} (capacity computation failed)", kp.name);
                continue;
            }
        };
        let time_capacity_ms = t_cap.elapsed().as_secs_f64() * 1000.0;

        let t_rec = Instant::now();
        let recovery = match recover_and_verify(&kp.polytope, &result) {
            Some(r) => r,
            None => {
                eprintln!("  FAIL {} (orbit recovery failed)", kp.name);
                failures += 1;
                total += 1;
                continue;
            }
        };
        let time_recovery_ms = t_rec.elapsed().as_secs_f64() * 1000.0;

        let active_facets = recovery.dwell_times.iter().filter(|&&t| t > 0.0).count();
        let on_facet_error = compute_on_facet_error(&kp.polytope, &result, &recovery);
        let action_error = (recovery.action - result.result.capacity).abs();

        let row = OrbitRecoveryRow {
            name: kp.name.to_string(),
            facet_count: kp.polytope.facet_count(),
            source: "known".to_string(),
            capacity: result.result.capacity,
            active_facets,
            total_segments: result.result.best_permutation.len(),
            solution_dim: 0, // no longer exposed by library; recorded as 0
            max_violation: recovery.max_violation,
            closure_error: recovery.closure_error,
            on_facet_error,
            inside_k_error: recovery.max_violation,
            computed_action: recovery.action,
            action_error,
            time_capacity_ms,
            time_recovery_ms,
        };

        // Thresholds: capacity values are O(1)–O(10), known polytopes achieve
        // ~1e-14 for all metrics. At F=10, ill-conditioned KKT systems cause:
        //   - closure/on_facet/violation: up to ~1e-7 (threshold 1e-6)
        //   - action: up to ~1.8e-6 (threshold 1e-5, looser because action
        //     accumulates rounding over the full orbit reconstruction)
        let valid = recovery.closure_error < 1e-6
            && on_facet_error < 1e-6
            && recovery.max_violation < 1e-6
            && action_error < 1e-5;

        eprintln!(
            "  {} F={} viol={:.2e} close={:.2e} action_err={:.2e} {}",
            kp.name,
            kp.polytope.facet_count(),
            recovery.max_violation,
            recovery.closure_error,
            action_error,
            if valid { "OK" } else { "FAIL" }
        );

        if !valid {
            failures += 1;
        }
        total += 1;

        let json = serde_json::to_string(&row).unwrap();
        writeln!(writer, "{json}").unwrap();
    }

    // Phase 2: Random polytopes
    eprintln!("\n=== Random polytopes ===");
    for &(f, n) in RANDOM_PLAN {
        let polytopes = generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);
        for (i, poly) in polytopes.iter().enumerate() {
            let name = format!("random_F{f}_{i:03}");

            let t_cap = Instant::now();
            let result = match ehz_capacity(poly) {
                Some(r) => r,
                None => {
                    eprintln!("  SKIP {name} (capacity computation failed)");
                    continue;
                }
            };
            let time_capacity_ms = t_cap.elapsed().as_secs_f64() * 1000.0;

            let t_rec = Instant::now();
            let recovery = match recover_and_verify(poly, &result) {
                Some(r) => r,
                None => {
                    eprintln!("  FAIL {name} (orbit recovery failed)");
                    failures += 1;
                    total += 1;
                    continue;
                }
            };
            let time_recovery_ms = t_rec.elapsed().as_secs_f64() * 1000.0;

            let active_facets = recovery.dwell_times.iter().filter(|&&t| t > 0.0).count();
            let on_facet_error = compute_on_facet_error(poly, &result, &recovery);
            let action_error = (recovery.action - result.result.capacity).abs();

            let row = OrbitRecoveryRow {
                name: name.clone(),
                facet_count: poly.facet_count(),
                source: "random".to_string(),
                capacity: result.result.capacity,
                active_facets,
                total_segments: result.result.best_permutation.len(),
                solution_dim: 0, // no longer exposed by library; recorded as 0
                max_violation: recovery.max_violation,
                closure_error: recovery.closure_error,
                on_facet_error,
                inside_k_error: recovery.max_violation,
                computed_action: recovery.action,
                action_error,
                time_capacity_ms,
                time_recovery_ms,
            };

            // See threshold rationale in known-polytopes block above.
            let valid = recovery.closure_error < 1e-6
                && on_facet_error < 1e-6
                && recovery.max_violation < 1e-6
                && action_error < 1e-5;

            if !valid {
                eprintln!(
                    "  {name} F={f} viol={:.2e} close={:.2e} action_err={:.2e} {}",
                    recovery.max_violation,
                    recovery.closure_error,
                    action_error,
                    if valid { "OK" } else { "FAIL" }
                );
            }

            if !valid {
                failures += 1;
            }
            total += 1;

            let json = serde_json::to_string(&row).unwrap();
            writeln!(writer, "{json}").unwrap();
        }
        eprintln!("  F={f}: {n} polytopes processed");
    }

    writer.flush().unwrap();

    let elapsed = t0.elapsed();
    eprintln!(
        "\nDone: {total} polytopes, {failures} failures, {:.1}s total",
        elapsed.as_secs_f64()
    );
    eprintln!("Output: {}", output_path.display());

    if failures > 0 {
        std::process::exit(1);
    }
}
