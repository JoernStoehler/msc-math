//! Geometric orbit validation experiment on a curated local-first target pool.
//!
//! Goal: validate minimum-action geometric orbit recovery on a bounded target set while
//! reusing cached capacity + minimum-sigma data from shared repo mirrors when
//! possible.
//!
//! Architecture:
//! 1. Build a curated target pool: literature-known polytopes, one random row
//!    per shared-cache facet-count stratum, and one lagrangian-product row per
//!    shared-cache polygon-pair stratum.
//! 2. Load shared mirrors as read-only search inputs plus an experiment-owned
//!    extension cache for locally produced rows.
//! 3. Resolve one minimum-action EHZ result per target via exact-key lookup,
//!    provenance lookup, or local computation.
//! 4. Recompute beta from the chosen best permutation, recover the orbit, and
//!    record finite numerical verification metrics.
//! 5. Write a validation JSONL and save only new locally produced cache rows to
//!    the experiment-owned extension file.

use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::algorithms::hk2017::orbit_recovery::{recover_and_verify, GeometricOrbit};
use symplectic::algorithms::{OrbitAdmissibility, OrbitKktData};
use symplectic::database::{self, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;
use symplectic::random::generate_polytope;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const GEOMETRY_TOL: f64 = 1e-6;
const ACTION_TOL: f64 = 1e-5;
const CACHE_ACTION_TOL: f64 = 1e-10;
const EXCLUDED_KNOWN_NAMES: &[&str] = &["crosspolytope"];

#[derive(Clone)]
enum TargetSpec {
    Known {
        name: String,
        family: &'static str,
        polytope: Polytope4D,
        source: Source,
    },
    RandomBySource {
        name: String,
        family: &'static str,
        facet_count: usize,
        source: Source,
    },
    CatalogByKey {
        name: String,
        family: &'static str,
        facet_count: usize,
        key: DualVerticesKey,
    },
}

#[derive(Debug, Serialize)]
struct GeometricOrbitRow {
    name: String,
    family: String,
    resolution: String,
    facet_count: usize,
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

struct ResolvedTarget {
    polytope: Polytope4D,
    orbit: OrbitKktData,
    resolution: &'static str,
    time_capacity_ms: f64,
    persist_source: Option<Source>,
}

enum RunMode {
    Smoke,
    CommitOutput,
}

struct RunPaths {
    extension_db_path: PathBuf,
    output_path: PathBuf,
    mode: RunMode,
}

fn main() {
    let t0 = Instant::now();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let run_paths = parse_run_paths(manifest_dir);
    let extension_db_path = run_paths.extension_db_path.clone();
    let output_path = run_paths.output_path.clone();
    let merged_db = load_merged_database(manifest_dir, &extension_db_path);
    let mut extension_db =
        database::load(&extension_db_path).expect("failed to load extension cache");
    let targets = build_target_pool(&merged_db);

    eprintln!(
        "Mode: {}",
        match run_paths.mode {
            RunMode::Smoke => "smoke",
            RunMode::CommitOutput => "commit-output",
        }
    );
    eprintln!("Loaded merged cache: {} entries", merged_db.len());
    eprintln!("Target pool: {} rows", targets.len());

    let file = File::create(&output_path).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut failures = 0usize;
    let mut by_resolution: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut extension_db_dirty = false;

    for target in &targets {
        let name = target_name(target).to_string();
        let family = target_family(target).to_string();

        let resolved = resolve_target(target, &merged_db);
        let ResolvedTarget {
            polytope,
            orbit,
            resolution,
            time_capacity_ms,
            persist_source,
        } = match resolved {
            Ok(value) => value,
            Err(err) => {
                eprintln!("  FAIL {name} ({err})");
                failures += 1;
                total += 1;
                continue;
            }
        };

        let t_rec = Instant::now();
        let geometric_orbit = match recover_and_verify(&polytope, &orbit) {
            Some(value) => value,
            None => {
                eprintln!("  FAIL {name} (geometric orbit recovery failed)");
                failures += 1;
                total += 1;
                continue;
            }
        };
        let time_recovery_ms = t_rec.elapsed().as_secs_f64() * 1000.0;

        let on_facet_error = compute_on_facet_error(&polytope, &orbit.sigma, &geometric_orbit);
        let action_error = (geometric_orbit.action - orbit.action).abs();
        let active_facets = geometric_orbit
            .dwell_times
            .iter()
            .filter(|&&t| t > 0.0)
            .count();

        let row = GeometricOrbitRow {
            name: name.clone(),
            family: family.clone(),
            resolution: resolution.to_string(),
            facet_count: polytope.facet_count(),
            capacity: orbit.action,
            active_facets,
            total_segments: orbit.sigma.len(),
            solution_dim: geometric_orbit.solution_dim,
            max_violation: geometric_orbit.max_violation,
            closure_error: geometric_orbit.closure_error,
            on_facet_error,
            inside_k_error: geometric_orbit.max_violation,
            computed_action: geometric_orbit.action,
            action_error,
            time_capacity_ms,
            time_recovery_ms,
        };

        let valid = row.closure_error < GEOMETRY_TOL
            && row.on_facet_error < GEOMETRY_TOL
            && row.inside_k_error < GEOMETRY_TOL
            && row.action_error < ACTION_TOL;

        eprintln!(
            "  {name} [{}|{}] F={} dim={} viol={:.2e} close={:.2e} action_err={:.2e} {}",
            family,
            resolution,
            row.facet_count,
            row.solution_dim,
            row.max_violation,
            row.closure_error,
            row.action_error,
            if valid { "OK" } else { "FAIL" },
        );

        if let Some(source) = persist_source {
            extension_db_dirty |=
                persist_extension_row(&mut extension_db, &polytope, source, &orbit);
        }

        *by_resolution.entry(resolution).or_insert(0) += 1;
        if !valid {
            failures += 1;
        }
        total += 1;

        let json = serde_json::to_string(&row).expect("serialize orbit row");
        writeln!(writer, "{json}").expect("write output row");
    }

    writer.flush().expect("flush output writer");
    if extension_db_dirty {
        database::save(&extension_db_path, &extension_db).expect("failed to save extension cache");
    }

    let elapsed = t0.elapsed();
    eprintln!("\nResolution counts:");
    for (mode, count) in by_resolution {
        eprintln!("  {mode}: {count}");
    }
    eprintln!(
        "\nDone: {total} polytopes, {failures} failures, {:.1}s total",
        elapsed.as_secs_f64()
    );
    eprintln!(
        "Extension cache: {} entries. Output: {}",
        extension_db.len(),
        output_path.display()
    );

    if failures > 0 {
        std::process::exit(1);
    }
}

fn parse_run_paths(manifest_dir: &Path) -> RunPaths {
    let mut args = env::args().skip(1);
    let mut commit_output = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--commit-output" => commit_output = true,
            "--help" | "-h" => {
                print_help_and_exit();
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_help_and_exit();
            }
        }
    }

    let orbit_dir = manifest_dir.join("orbit-recovery");
    if commit_output {
        RunPaths {
            extension_db_path: orbit_dir.join("cache-extension.jsonl"),
            output_path: orbit_dir.join("orbit-recovery.jsonl"),
            mode: RunMode::CommitOutput,
        }
    } else {
        RunPaths {
            extension_db_path: orbit_dir.join("smoke-cache-extension.jsonl"),
            output_path: orbit_dir.join("smoke-orbit-recovery.jsonl"),
            mode: RunMode::Smoke,
        }
    }
}

fn print_help_and_exit() -> ! {
    eprintln!("Usage: cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery [--commit-output]");
    eprintln!("  default: write smoke-orbit-recovery.jsonl and smoke-cache-extension.jsonl");
    eprintln!("  --commit-output: refresh orbit-recovery.jsonl and cache-extension.jsonl");
    std::process::exit(2);
}

fn shared_cache_paths(manifest_dir: &Path) -> [PathBuf; 3] {
    [
        manifest_dir.join("orbit-recovery/polytopes.jsonl"),
        manifest_dir.join("../combinatorial-cells/polytopes.jsonl"),
        manifest_dir.join("../sys-landscape/cache.jsonl"),
    ]
}

fn load_merged_database(
    manifest_dir: &Path,
    extension_db_path: &Path,
) -> HashMap<DualVerticesKey, PolytopeRecord> {
    let shared_paths = shared_cache_paths(manifest_dir);
    let merged_paths = [
        shared_paths[0].as_path(),
        shared_paths[1].as_path(),
        shared_paths[2].as_path(),
        extension_db_path,
    ];
    database::load_many(&merged_paths).expect("failed to load merged orbit-recovery cache surface")
}

fn build_target_pool(db: &HashMap<DualVerticesKey, PolytopeRecord>) -> Vec<TargetSpec> {
    let mut targets = Vec::new();
    targets.extend(build_known_targets());
    targets.extend(build_random_targets(db));
    targets.extend(build_lagrangian_product_targets(db));
    targets
}

fn build_known_targets() -> Vec<TargetSpec> {
    known_polytopes::all_known()
        .into_iter()
        .filter(|kp| !EXCLUDED_KNOWN_NAMES.contains(&kp.name))
        .filter(|kp| kp.polytope.facet_count() <= 12)
        .map(|kp| TargetSpec::Known {
            name: kp.name.to_string(),
            family: "known",
            polytope: kp.polytope.clone(),
            source: Source::Known {
                name: kp.name.to_string(),
            },
        })
        .collect()
}

fn build_random_targets(db: &HashMap<DualVerticesKey, PolytopeRecord>) -> Vec<TargetSpec> {
    let mut by_facet_count: BTreeMap<usize, Source> = BTreeMap::new();

    for record in db.values() {
        let Some(Source::Random {
            master_seed,
            attempt,
            facet_count_target,
            h_min,
            h_max,
        }) = record.source.clone()
        else {
            continue;
        };

        let source = Source::Random {
            master_seed,
            attempt,
            facet_count_target,
            h_min,
            h_max,
        };
        by_facet_count
            .entry(facet_count_target)
            .and_modify(|current| {
                let current_attempt = random_attempt(current);
                if attempt < current_attempt {
                    *current = source.clone();
                }
            })
            .or_insert(source);
    }

    by_facet_count
        .into_iter()
        .map(|(facet_count, source)| TargetSpec::RandomBySource {
            name: format!("random_F{facet_count}_seeded"),
            family: "random",
            facet_count,
            source,
        })
        .collect()
}

fn build_lagrangian_product_targets(
    db: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> Vec<TargetSpec> {
    let mut by_pair: BTreeMap<(usize, usize), (String, DualVerticesKey, usize)> = BTreeMap::new();

    for (key, record) in db {
        let Some(Source::LagrangianProduct { n1, n2, .. }) = record.source.as_ref() else {
            continue;
        };
        let pair = (*n1, *n2);
        let signature = serde_json::to_string(&record.dual_vertices_rational)
            .expect("serialize dual-vertex key signature");
        let facet_count = record.dual_vertices_rational.len();

        match by_pair.get_mut(&pair) {
            Some((best_signature, best_key, best_facet_count)) => {
                if signature < *best_signature {
                    *best_signature = signature;
                    *best_key = key.clone();
                    *best_facet_count = facet_count;
                }
            }
            None => {
                by_pair.insert(pair, (signature, key.clone(), facet_count));
            }
        }
    }

    by_pair
        .into_iter()
        .map(
            |((n1, n2), (_, key, facet_count))| TargetSpec::CatalogByKey {
                name: format!("lagrangian_product_{n1}x{n2}"),
                family: "lagrangian_product",
                facet_count,
                key,
            },
        )
        .collect()
}

fn random_attempt(source: &Source) -> u64 {
    match source {
        Source::Random { attempt, .. } => *attempt,
        _ => unreachable!("random_attempt called on non-random source"),
    }
}

fn target_name(target: &TargetSpec) -> &str {
    match target {
        TargetSpec::Known { name, .. }
        | TargetSpec::RandomBySource { name, .. }
        | TargetSpec::CatalogByKey { name, .. } => name,
    }
}

fn target_family(target: &TargetSpec) -> &'static str {
    match target {
        TargetSpec::Known { family, .. }
        | TargetSpec::RandomBySource { family, .. }
        | TargetSpec::CatalogByKey { family, .. } => family,
    }
}

fn resolve_target(
    target: &TargetSpec,
    db: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> Result<ResolvedTarget, String> {
    match target {
        TargetSpec::Known {
            polytope, source, ..
        } => resolve_known_target(polytope, source.clone(), db),
        TargetSpec::RandomBySource {
            facet_count,
            source,
            ..
        } => resolve_random_target(*facet_count, source, db),
        TargetSpec::CatalogByKey { key, .. } => resolve_catalog_key_target(key, db),
    }
}

fn resolve_known_target(
    polytope: &Polytope4D,
    source: Source,
    db: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> Result<ResolvedTarget, String> {
    let key = polytope.dual_vertices().to_vec();
    if let Some(record) = db.get(&key) {
        let t_cap = Instant::now();
        let orbit = orbit_from_cache(polytope, record)?;
        return Ok(ResolvedTarget {
            polytope: polytope.clone(),
            orbit,
            resolution: "key_hit",
            time_capacity_ms: t_cap.elapsed().as_secs_f64() * 1000.0,
            persist_source: None,
        });
    }

    compute_target_locally(polytope.clone(), source)
}

fn resolve_random_target(
    facet_count: usize,
    source: &Source,
    db: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> Result<ResolvedTarget, String> {
    if let Some((_, record)) = find_by_source(db, source) {
        let polytope = record
            .to_polytope()
            .map_err(|err| format!("failed to reconstruct cached random polytope: {err}"))?;
        let t_cap = Instant::now();
        let orbit = orbit_from_cache(&polytope, record)?;
        return Ok(ResolvedTarget {
            polytope,
            orbit,
            resolution: "source_hit",
            time_capacity_ms: t_cap.elapsed().as_secs_f64() * 1000.0,
            persist_source: None,
        });
    }

    let Source::Random {
        master_seed,
        attempt,
        h_min,
        h_max,
        ..
    } = source
    else {
        return Err("non-random source passed to random target".to_string());
    };

    let polytope = generate_polytope(facet_count, *h_min, *h_max, *master_seed, *attempt)
        .map_err(|err| format!("failed to generate random target: {err}"))?;
    compute_target_locally(polytope, source.clone())
}

fn resolve_catalog_key_target(
    key: &DualVerticesKey,
    db: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> Result<ResolvedTarget, String> {
    let record = db
        .get(key)
        .ok_or_else(|| "catalog key missing from merged cache".to_string())?;
    let polytope = record
        .to_polytope()
        .map_err(|err| format!("failed to reconstruct cached catalog polytope: {err}"))?;
    let t_cap = Instant::now();
    let orbit = orbit_from_cache(&polytope, record)?;

    Ok(ResolvedTarget {
        polytope,
        orbit,
        resolution: "key_hit",
        time_capacity_ms: t_cap.elapsed().as_secs_f64() * 1000.0,
        persist_source: None,
    })
}

fn compute_target_locally(polytope: Polytope4D, source: Source) -> Result<ResolvedTarget, String> {
    let t_cap = Instant::now();
    let result =
        ehz_capacity(&polytope).map_err(|err| format!("capacity computation failed: {err:?}"))?;
    Ok(ResolvedTarget {
        polytope,
        orbit: result.best_orbit().clone(),
        resolution: "local_compute",
        time_capacity_ms: t_cap.elapsed().as_secs_f64() * 1000.0,
        persist_source: Some(source),
    })
}

fn persist_extension_row(
    extension_db: &mut HashMap<DualVerticesKey, PolytopeRecord>,
    polytope: &Polytope4D,
    source: Source,
    orbit: &OrbitKktData,
) -> bool {
    let key = polytope.dual_vertices().to_vec();
    if extension_db.contains_key(&key) {
        return false;
    }

    let record = PolytopeRecord {
        source: Some(source),
        capacity: Some(orbit.action),
        capacity_err: Some(0.0),
        ..PolytopeRecord::from_polytope(polytope).with_sigmas(
            vec![SigmaAction {
                perm: orbit.sigma.clone(),
                action: orbit.action,
            }],
            0.0,
        )
    };
    extension_db.insert(key, record);
    true
}

fn compute_on_facet_error(
    polytope: &Polytope4D,
    perm: &[usize],
    geometric_orbit: &GeometricOrbit,
) -> f64 {
    let duals = polytope.dual_vertices_f64();
    (0..perm.len())
        .filter(|&k| geometric_orbit.dwell_times[k] > 0.0)
        .map(|k| {
            let a = &duals[perm[k]];
            (a.dot(&geometric_orbit.breakpoints[k]) - 1.0).abs()
        })
        .fold(0.0_f64, f64::max)
}

fn find_by_source<'a>(
    db: &'a HashMap<DualVerticesKey, PolytopeRecord>,
    source: &Source,
) -> Option<(&'a DualVerticesKey, &'a PolytopeRecord)> {
    db.iter()
        .find(|(_, record)| record.source.as_ref() == Some(source))
}

fn orbit_from_cache(
    polytope: &Polytope4D,
    record: &PolytopeRecord,
) -> Result<OrbitKktData, String> {
    // Cache rows store the minimum-action value and a sigma, but not the beta
    // needed by geometric orbit recovery, so this experiment must still rebuild one
    // solved orbit payload by re-solving KKT for that cached minimizer.
    let capacity = record
        .capacity
        .ok_or_else(|| "cached row missing capacity".to_string())?;
    let sigmas = record
        .sigmas
        .as_ref()
        .ok_or_else(|| "cached row missing sigma list".to_string())?;
    let best_sigma = sigmas
        .first()
        .ok_or_else(|| "cached row has empty sigma list".to_string())?;
    if (best_sigma.action - capacity).abs() > CACHE_ACTION_TOL {
        return Err(format!(
            "cached sigma/capacity mismatch: |{} - {}| > {}",
            best_sigma.action, capacity, CACHE_ACTION_TOL,
        ));
    }

    let kkt = solve_kkt_for(polytope, &best_sigma.perm)
        .feasible()
        .ok_or_else(|| "KKT solve failed for cached best permutation".to_string())?;

    let beta_margin = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
    Ok(OrbitKktData {
        sigma: best_sigma.perm.clone(),
        beta: kkt.beta,
        beta_margin,
        action: capacity,
        action_lower: capacity,
        action_upper: capacity,
        q: kkt.q_corrected,
        q_error_bound: 0.0,
        mu: Some([kkt.mu[0], kkt.mu[1], kkt.mu[2], kkt.mu[3]]),
        xi: Some(kkt.xi),
        admissibility: OrbitAdmissibility::AdmissibleF64,
    })
}
