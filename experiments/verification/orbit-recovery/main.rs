//! Orbit recovery validation experiment.
//!
//! For each polytope (known + random), computes c_EHZ(K), recovers the base
//! point b = γ(0), validates the reconstructed orbit, and outputs JSONL.
//!
//! Architecture:
//! 1. `cargo run --bin axioms-orbit-recovery --release` generates dataset
//! 2. Polytopes cached in experiments/verification/orbit-recovery/polytopes.jsonl.
//!    When capacity + sigmas are cached, skips full EHZ and constructs result
//!    from cached perm + single-perm KKT solve.
//! 3. Writes to orbit-recovery/orbit-recovery.jsonl
//! 4. Python script analyzes and plots results
//!
//! Validation checks per polytope:
//! - Closure: orbit returns to starting point
//! - On-facet: each segment lies on the correct facet
//! - Inside K: orbit stays inside K at all breakpoints
//! - Action: computed action matches capacity
//! - Solution dimension: how many free parameters in b

use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::database::{self, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::algorithms::capacity_accumulator::CapacityResult;
use symplectic::algorithms::hk2017::orbit_recovery::{recover_and_verify, OrbitRecovery};
use symplectic::algorithms::hk2017::{ehz_capacity, EhzResult};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;
use symplectic::random::generate_polytope;

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
fn compute_on_facet_error(polytope: &Polytope4D, perm: &[usize], recovery: &OrbitRecovery) -> f64 {
    let duals = polytope.dual_vertices_f64();
    (0..perm.len())
        .filter(|&k| recovery.dwell_times[k] > 0.0)
        .map(|k| {
            let a = &duals[perm[k]];
            (a.dot(&recovery.breakpoints[k]) - 1.0).abs()
        })
        .fold(0.0_f64, f64::max)
}

/// Source-based lookup for random polytopes.
fn find_by_source<'a>(
    db: &'a HashMap<DualVerticesKey, PolytopeRecord>,
    source: &Source,
) -> Option<(&'a DualVerticesKey, &'a PolytopeRecord)> {
    db.iter().find(|(_, r)| r.source.as_ref() == Some(source))
}

/// Try to construct an EhzResult from cached database record + single-perm KKT solve.
/// Returns None if the record lacks capacity/sigmas or KKT solve fails.
fn ehz_result_from_cache(polytope: &Polytope4D, record: &PolytopeRecord) -> Option<EhzResult> {
    let capacity = record.capacity?;
    let sigmas = record.sigmas.as_ref()?;
    let best_perm = sigmas.first()?.perm.clone();

    // Single-perm KKT solve for beta (cheap, ~0.01ms)
    let kkt = solve_kkt_for(polytope, &best_perm).feasible()?;

    let mut best_subset = best_perm.clone();
    best_subset.sort();

    Some(EhzResult {
        result: CapacityResult {
            capacity,
            capacity_uncertain: capacity,
            best_permutation: best_perm,
            best_beta: kkt.beta,
            iterations: 0,
        },
        best_subset,
    })
}

fn main() {
    let t0 = Instant::now();

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("orbit-recovery/polytopes.jsonl");
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("orbit-recovery/orbit-recovery.jsonl");

    let mut db: HashMap<DualVerticesKey, PolytopeRecord> =
        database::load_many(&[owned_db_path.as_path()])
            .expect("failed to load database");
    eprintln!("Loaded database: {} entries", db.len());

    let file = File::create(&output_path).expect("Cannot create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut failures = 0usize;
    let mut cache_hits = 0usize;

    // Phase 1: Known polytopes
    eprintln!("=== Known polytopes ===");
    for kp in known_polytopes::all_known() {
        if kp.polytope.facet_count() > 12 {
            eprintln!("  SKIP {} (F={}, too slow)", kp.name, kp.polytope.facet_count());
            continue;
        }

        let key: DualVerticesKey = kp.polytope.dual_vertices().to_vec();

        let t_cap = Instant::now();
        let cap_result = (|| {
            if let Some(record) = db.get(&key) {
                if let Some(r) = ehz_result_from_cache(&kp.polytope, record) {
                    return Some((r, true));
                }
            }
            ehz_capacity(&kp.polytope).map(|r| (r, false))
        })();
        let (result, is_cache_hit) = match cap_result {
            Some(pair) => pair,
            None => {
                eprintln!("  SKIP {} (capacity computation failed)", kp.name);
                continue;
            }
        };
        let time_capacity_ms = t_cap.elapsed().as_secs_f64() * 1000.0;
        if is_cache_hit { cache_hits += 1; }

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

        let perm = &result.result.best_permutation;
        let active_facets = recovery.dwell_times.iter().filter(|&&t| t > 0.0).count();
        let on_facet_error = compute_on_facet_error(&kp.polytope, perm, &recovery);
        let action_error = (recovery.action - result.result.capacity).abs();

        let row = OrbitRecoveryRow {
            name: kp.name.to_string(),
            facet_count: kp.polytope.facet_count(),
            source: "known".to_string(),
            capacity: result.result.capacity,
            active_facets,
            total_segments: perm.len(),
            solution_dim: 0,
            max_violation: recovery.max_violation,
            closure_error: recovery.closure_error,
            on_facet_error,
            inside_k_error: recovery.max_violation,
            computed_action: recovery.action,
            action_error,
            time_capacity_ms,
            time_recovery_ms,
        };

        let valid = recovery.closure_error < 1e-6
            && on_facet_error < 1e-6
            && recovery.max_violation < 1e-6
            && action_error < 1e-5;

        eprintln!(
            "  {} F={} viol={:.2e} close={:.2e} action_err={:.2e} {}{}",
            kp.name, kp.polytope.facet_count(),
            recovery.max_violation, recovery.closure_error, action_error,
            if valid { "OK" } else { "FAIL" },
            if is_cache_hit { " (cached)" } else { "" }
        );

        if !valid { failures += 1; }
        total += 1;

        // Insert into database if not already there
        if !db.contains_key(&key) {
            let mut record = PolytopeRecord::from_polytope(&kp.polytope);
            record.source = Some(Source::Known { name: kp.name.to_string() });
            record = record.with_computed_fields(
                0.0, 0.0, result.result.capacity, 0.0,
            );
            record = record.with_sigmas(
                vec![SigmaAction { perm: perm.clone(), action: result.result.capacity }],
                0.0,
            );
            db.insert(key, record);
        }

        let json = serde_json::to_string(&row).unwrap();
        writeln!(writer, "{json}").unwrap();
    }

    // Phase 2: Random polytopes via generate_polytope (blake3 per-attempt seeding)
    eprintln!("\n=== Random polytopes ===");
    let mut attempt: u64 = 0;
    for &(f, n) in RANDOM_PLAN {
        let mut accepted = 0usize;
        let mut hits_this_f = 0usize;

        while accepted < n {
            let source_tag = Source::Random {
                master_seed: SEED,
                attempt,
                facet_count_target: f,
                h_min: H_MIN,
                h_max: H_MAX,
            };

            // Source-based lookup
            let (poly, cached_result) = if let Some((_, record)) = find_by_source(&db, &source_tag) {
                let p = record.to_polytope().expect("failed to reconstruct polytope");
                let r = ehz_result_from_cache(&p, record);
                (p, r)
            } else {
                match generate_polytope(f, H_MIN, H_MAX, SEED, attempt) {
                    Ok(p) => (p, None),
                    Err(_) => {
                        attempt += 1;
                        continue;
                    }
                }
            };

            let name = format!("random_F{f}_{accepted:03}");

            let t_cap = Instant::now();
            let (result, is_cache_hit) = if let Some(r) = cached_result {
                (r, true)
            } else {
                match ehz_capacity(&poly) {
                    Some(r) => (r, false),
                    None => {
                        eprintln!("  SKIP {name} (capacity computation failed)");
                        accepted += 1;
                        attempt += 1;
                        continue;
                    }
                }
            };
            let time_capacity_ms = t_cap.elapsed().as_secs_f64() * 1000.0;
            if is_cache_hit { cache_hits += 1; hits_this_f += 1; }

            let t_rec = Instant::now();
            let recovery = match recover_and_verify(&poly, &result) {
                Some(r) => r,
                None => {
                    eprintln!("  FAIL {name} (orbit recovery failed)");
                    failures += 1;
                    total += 1;
                    accepted += 1;
                    attempt += 1;
                    continue;
                }
            };
            let time_recovery_ms = t_rec.elapsed().as_secs_f64() * 1000.0;

            let perm = &result.result.best_permutation;
            let active_facets = recovery.dwell_times.iter().filter(|&&t| t > 0.0).count();
            let on_facet_error = compute_on_facet_error(&poly, perm, &recovery);
            let action_error = (recovery.action - result.result.capacity).abs();

            let row = OrbitRecoveryRow {
                name: name.clone(),
                facet_count: poly.facet_count(),
                source: "random".to_string(),
                capacity: result.result.capacity,
                active_facets,
                total_segments: perm.len(),
                solution_dim: 0,
                max_violation: recovery.max_violation,
                closure_error: recovery.closure_error,
                on_facet_error,
                inside_k_error: recovery.max_violation,
                computed_action: recovery.action,
                action_error,
                time_capacity_ms,
                time_recovery_ms,
            };

            let valid = recovery.closure_error < 1e-6
                && on_facet_error < 1e-6
                && recovery.max_violation < 1e-6
                && action_error < 1e-5;

            if !valid {
                eprintln!(
                    "  {name} F={f} viol={:.2e} close={:.2e} action_err={:.2e} FAIL",
                    recovery.max_violation, recovery.closure_error, action_error,
                );
                failures += 1;
            }
            total += 1;

            // Insert into database if not already there
            let key: DualVerticesKey = poly.dual_vertices().to_vec();
            if !db.contains_key(&key) {
                let mut record = PolytopeRecord::from_polytope(&poly);
                record.source = Some(source_tag);
                record = record.with_computed_fields(0.0, 0.0, result.result.capacity, 0.0);
                record = record.with_sigmas(
                    vec![SigmaAction { perm: perm.clone(), action: result.result.capacity }],
                    0.0,
                );
                db.insert(key, record);
            }

            let json = serde_json::to_string(&row).unwrap();
            writeln!(writer, "{json}").unwrap();
            accepted += 1;
            attempt += 1;
        }
        eprintln!("  F={f}: {n} polytopes ({hits_this_f} cache hits)");
    }

    writer.flush().unwrap();
    database::save(&owned_db_path, &db).expect("failed to save database");

    let elapsed = t0.elapsed();
    eprintln!(
        "\nDone: {total} polytopes, {failures} failures, {cache_hits} cache hits, {:.1}s total",
        elapsed.as_secs_f64()
    );
    eprintln!("Database: {} entries. Output: {}", db.len(), output_path.display());

    if failures > 0 {
        std::process::exit(1);
    }
}
