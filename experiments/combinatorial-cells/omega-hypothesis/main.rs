//! Omega-obstacle experiment: do near-Lagrangian 2-faces help create high systolic ratios?
//!
//! Hypothesis: small |ω₀(n_i, n_j)| between adjacent facets → high sys.
//! Mechanism: Q(β) = Σ β_i β_j ω₀(...), capacity = 1/(2·max Q), sys = c²/(2V).
//! Small ω contributions → smaller Q → larger capacity → potentially larger sys.
//!
//! Phase A (observational): For each polytope, compute ω₀ for all adjacent two-face pairs
//! and for orbit transitions. Plot min|ω| vs sys.
//!
//! Phase B (gradient): Compute ⟨∇_{n_k} sys, ∇_{n_k} ω(n_k, n_i)⟩ analytically.
//! Negative dot product → sys increases when ω decreases → hypothesis supported.
//!
//! Location: experiments/combinatorial-cells/omega-hypothesis/main.rs
//! Goal: Generate the shared combinatorial polytope cache plus omega-obstacle rows.
//! Input Artifacts: None (samples and known polytopes are generated from hardcoded plans and seeds).
//! Output Artifacts: experiments/combinatorial-cells/polytopes.jsonl,
//!         experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl
//!
//! Architecture:
//! 1. `cargo run -p exp-combinatorial-cells --bin cell-omega --release` generates dataset
//! 2. Polytopes cached in experiments/combinatorial-cells/polytopes.jsonl.
//!    When capacity + sigmas are cached, skips full EHZ (exponential) and only
//!    runs single-perm KKT solve for beta.
//! 3. Writes to omega-hypothesis/omega-obstacle.jsonl
//! 4. Python script reads JSONL, produces figures

use euclidean_polytopes::{two_faces_from_vertex_facet_incidence, TwoFace};
use exp_combinatorial_cells::euclidean_volume_f64;
use nalgebra::Vector4;
use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::database::{self, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::derivatives::{capacity_derivatives_a_from_kkt_result, volume_derivatives_a};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktResult};
use symplectic::random::generate_polytope;

// ============================================================================
// Configuration
// ============================================================================

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples) pairs for random polytope generation.
const SAMPLING_PLAN: &[(usize, usize)] =
    &[(5, 200), (6, 200), (7, 200), (8, 200), (9, 100), (10, 50)];

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct OmegaRow {
    source: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_ms: f64,

    // Orbit info
    orbit_length: usize,
    orbit_facets: Vec<usize>,
    orbit_betas: Vec<f64>,

    // Omega features — orbit transitions (physical direction, all ≥ 0)
    orbit_omegas: Vec<f64>,
    orbit_omega_min: f64,
    orbit_omega_mean: f64,

    // Omega features — all ridge-adjacent pairs
    ridge_omegas: Vec<[f64; 3]>, // [i, j, ω₀(n_i, n_j)] where i < j
    ridge_omega_abs_min: f64,
    n_ridges: usize,

    // Gradient dot products (Phase B)
    gradient_dots: Vec<GradientDot>,
}

#[derive(Debug, Serialize)]
struct GradientDot {
    facet_k: usize,
    neighbor_i: usize,
    k_on_orbit: bool,
    i_on_orbit: bool,
    omega: f64,
    dot: f64,
    grad_sys_norm: f64,
}

// ============================================================================
// Sensitivity computation
// ============================================================================

/// J₀(a,b,c,d) = (-c,-d,a,b) in (q₁,q₂,p₁,p₂) coordinates.
/// Equivalent to `symplectic::geom::symplectic_form::j4() * v` but avoids matrix allocation.
fn j0_apply(v: &Vector4<f64>) -> Vector4<f64> {
    Vector4::new(-v[2], -v[3], v[0], v[1])
}

/// Full ∇_{n_k} sys via chain rule: d(sys)/d(n_k) = (1/V)[c·dc/dn_k - sys·dV/dn_k].
#[allow(clippy::too_many_arguments)]
fn compute_d_sys_a(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    best_perm: &[usize],
    kkt_result: &KktResult,
) -> Vec<Vector4<f64>> {
    let d_vol_a = volume_derivatives_a(
        polytope.dual_vertices_f64(),
        polytope.vertices_f64(),
        polytope.incidence(),
    )
    .expect("combinatorial-cell polytope has valid finite geometry");
    let d_cap_a =
        capacity_derivatives_a_from_kkt_result(polytope.dual_vertices_f64(), best_perm, kkt_result);

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Omega features: ω₀(n_i, n_j) for adjacent two-faces and orbit transitions.
/// Uses normals (unit dual vertices) for ω₀ computation.
fn compute_omega_features(
    polytope: &Polytope4D,
    two_faces: &[TwoFace],
    orbit_facets: &[usize],
) -> (Vec<[f64; 3]>, Vec<f64>) {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();

    // Two-face omegas: for each 2-face shared by facets i, j with i < j.
    let ridge_omegas: Vec<[f64; 3]> = two_faces
        .iter()
        .map(|two_face| {
            let i = two_face.facets[0];
            let j = two_face.facets[1];
            let w = omega0(&normals[i], &normals[j]);
            [i as f64, j as f64, w]
        })
        .collect();

    // Orbit omegas: ω₀(n_{σ(k)}, n_{σ(k+1)}) for physical transition σ(k) → σ(k+1).
    let m = orbit_facets.len();
    let orbit_omegas: Vec<f64> = (0..m)
        .map(|k| {
            let from = orbit_facets[k];
            let to = orbit_facets[(k + 1) % m];
            omega0(&normals[from], &normals[to])
        })
        .collect();

    (ridge_omegas, orbit_omegas)
}

/// ∂ω₀(n_k, n_i)/∂n_k projected to T_{n_k}S³.
///
/// ω₀(u,v) = u^T M v where M = J₀^T = -J₀ (skew-symmetric).
/// So ∂ω₀(n_k, n_i)/∂n_k = -J₀ n_i, projected onto tangent space of S³ at n_k.
fn omega_gradient_on_tangent(n_k: &Vector4<f64>, n_i: &Vector4<f64>) -> Vector4<f64> {
    let neg_j0_ni = -j0_apply(n_i);
    // Project to T_{n_k}S³: remove component along n_k
    neg_j0_ni - neg_j0_ni.dot(n_k) * n_k
}

/// Gradient dot products for all adjacent two-face facet pairs.
fn compute_gradient_dots(
    polytope: &Polytope4D,
    two_faces: &[TwoFace],
    d_sys_a: &[Vector4<f64>],
    orbit_facets: &[usize],
) -> Vec<GradientDot> {
    let normals: Vec<Vector4<f64>> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| a / a.norm())
        .collect();
    let orbit_set: std::collections::HashSet<usize> = orbit_facets.iter().copied().collect();

    // Build two-face neighbor lookup: for each facet k, list of neighbors.
    let f = polytope.facet_count();
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); f];
    for two_face in two_faces {
        let i = two_face.facets[0];
        let j = two_face.facets[1];
        neighbors[i].push(j);
        neighbors[j].push(i);
    }

    let mut dots = Vec::new();
    for k in 0..f {
        let grad_sys = &d_sys_a[k];
        let grad_sys_norm = grad_sys.norm();

        for &i in &neighbors[k] {
            let grad_omega = omega_gradient_on_tangent(&normals[k], &normals[i]);
            let dot = grad_sys.dot(&grad_omega);
            let w = omega0(&normals[k], &normals[i]);

            dots.push(GradientDot {
                facet_k: k,
                neighbor_i: i,
                k_on_orbit: orbit_set.contains(&k),
                i_on_orbit: orbit_set.contains(&i),
                omega: w,
                dot,
                grad_sys_norm,
            });
        }
    }

    dots
}

// ============================================================================
// Database helpers
// ============================================================================

/// Find a cached record by Source metadata. Linear scan, negligible for <1000 records.
fn find_by_source<'a>(
    db: &'a HashMap<DualVerticesKey, PolytopeRecord>,
    source: &Source,
) -> Option<(&'a DualVerticesKey, &'a PolytopeRecord)> {
    db.iter().find(|(_, r)| r.source.as_ref() == Some(source))
}

/// Cached capacity + best permutation from a database record.
struct CachedCapacity {
    capacity: f64,
    volume: f64,
    best_perm: Vec<usize>,
}

fn cached_capacity_from_record(record: &PolytopeRecord) -> Option<CachedCapacity> {
    let capacity = record.capacity?;
    let volume = record.volume?;
    let sigmas = record.sigmas.as_ref()?;
    let best_sigma = sigmas.first()?;
    Some(CachedCapacity {
        capacity,
        volume,
        best_perm: best_sigma.perm.clone(),
    })
}

// ============================================================================
// Core processing
// ============================================================================

/// Process one polytope: compute omega features and gradient dots.
///
/// If `cached` is Some, skip full EHZ and use cached capacity + permutation.
/// Still runs single-perm KKT solve for beta (needed for gradient computation).
fn process_polytope(
    polytope: &Polytope4D,
    source: &str,
    cached: Option<&CachedCapacity>,
) -> Option<OmegaRow> {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();

    let t0 = Instant::now();

    let (vol, cap, iterations, best_perm);

    if let Some(c) = cached {
        // Database hit: skip full EHZ, use cached values
        vol = c.volume;
        cap = c.capacity;
        iterations = 0;
        best_perm = c.best_perm.clone();
    } else {
        // No cache: full EHZ computation
        vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());
        let ehz_result = exp_combinatorial_cells::capacity_auto(polytope).ok()?;
        cap = ehz_result.capacity();
        iterations = ehz_result.iterations;
        best_perm = ehz_result.best_sigma().to_vec();
    }

    let sys = cap * cap / (2.0 * vol);
    let time_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Single-perm KKT solve for beta (cheap, ~0.01ms)
    let dual_vertices = polytope.dual_vertices_f64();
    let kkt_result = solve_kkt_for_dual_vertices(dual_vertices, &best_perm).feasible()?;
    let best_beta = &kkt_result.beta;

    // Phase A: omega features
    let two_faces = two_faces_from_vertex_facet_incidence(polytope.incidence());
    let (ridge_omegas, orbit_omegas) = compute_omega_features(polytope, &two_faces, &best_perm);

    let orbit_omega_min = orbit_omegas.iter().cloned().fold(f64::INFINITY, f64::min);
    let orbit_omega_mean = if orbit_omegas.is_empty() {
        0.0
    } else {
        orbit_omegas.iter().sum::<f64>() / orbit_omegas.len() as f64
    };

    let ridge_omega_abs_min = ridge_omegas
        .iter()
        .map(|r| r[2].abs())
        .fold(f64::INFINITY, f64::min);

    // Sanity: orbit omegas should all be ≥ 0 (feasibility)
    let n_negative = orbit_omegas.iter().filter(|&&w| w < -1e-10).count();
    if n_negative > 0 {
        let worst = orbit_omegas.iter().cloned().fold(f64::INFINITY, f64::min);
        eprintln!(
            "WARNING: {}: {}/{} orbit omegas < 0 (worst: {:.6e})",
            source,
            n_negative,
            orbit_omegas.len(),
            worst
        );
    }

    // Phase B: gradient dots (using library derivative functions with dual vertex parameterization)
    let d_sys_a = compute_d_sys_a(polytope, vol, cap, sys, &best_perm, &kkt_result);
    let gradient_dots = compute_gradient_dots(polytope, &two_faces, &d_sys_a, &best_perm);

    Some(OmegaRow {
        source: source.to_string(),
        facet_count: f,
        dual_vertices: duals.iter().map(|a| [a[0], a[1], a[2], a[3]]).collect(),
        volume: vol,
        capacity: cap,
        sys,
        iterations,
        time_ms,
        orbit_length: best_perm.len(),
        orbit_facets: best_perm,
        orbit_betas: best_beta.clone(),
        orbit_omegas,
        orbit_omega_min,
        orbit_omega_mean,
        ridge_omegas,
        ridge_omega_abs_min,
        n_ridges: two_faces.len(),
        gradient_dots,
    })
}

fn main() {
    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("omega-hypothesis");
    let out_path = out_dir.join("omega-obstacle.jsonl");
    std::fs::create_dir_all(&out_dir)
        .unwrap_or_else(|err| panic!("create output directory {}: {err}", out_dir.display()));
    let file = std::fs::File::create(&out_path)
        .unwrap_or_else(|err| panic!("create output file {}: {err}", out_path.display()));
    let mut writer = BufWriter::new(file);

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("polytopes.jsonl");
    let mut db: HashMap<DualVerticesKey, PolytopeRecord> =
        database::load_many(&[owned_db_path.as_path()]).expect("failed to load database");
    eprintln!("Loaded database: {} entries", db.len());

    let mut total = 0usize;
    let mut failed = 0usize;
    let mut cache_hits = 0usize;

    eprintln!("=== Omega-obstacle experiment ===");
    eprintln!("Output: {}", out_path.display());

    // Random polytopes via generate_polytope (blake3 per-attempt seeding)
    let mut attempt: u64 = 0;
    for &(f, n) in SAMPLING_PLAN {
        let t0 = Instant::now();
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
            let (polytope, cached) = if let Some((_, record)) = find_by_source(&db, &source_tag) {
                let p = record
                    .to_polytope()
                    .expect("failed to reconstruct polytope from database");
                let c = cached_capacity_from_record(record);
                if c.is_some() {
                    hits_this_f += 1;
                }
                (p, c)
            } else {
                // Generate new polytope
                match generate_polytope(f, H_MIN, H_MAX, SEED, attempt) {
                    Ok(p) => (p, None),
                    Err(_) => {
                        attempt += 1;
                        continue; // rejection sampling
                    }
                }
            };

            let source_name = format!("random_F{}_{}", f, accepted);
            match process_polytope(&polytope, &source_name, cached.as_ref()) {
                Some(row) => {
                    serde_json::to_writer(&mut writer, &row).unwrap();
                    writeln!(writer).unwrap();

                    // Insert into database if not already there
                    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
                    if !db.contains_key(&key) {
                        let mut record = PolytopeRecord::from_polytope(&polytope);
                        record.source = Some(source_tag);
                        record = record.with_computed_fields(row.volume, 0.0, row.capacity, 0.0);
                        record = record.with_sigmas(
                            vec![SigmaAction {
                                perm: row.orbit_facets.clone(),
                                action: row.capacity,
                            }],
                            0.0,
                        );
                        db.insert(key, record);
                    }

                    total += 1;
                    if cached.is_some() {
                        cache_hits += 1;
                    }
                }
                None => {
                    eprintln!("  SKIP: {} (capacity computation failed)", source_name);
                    failed += 1;
                }
            }
            accepted += 1;
            attempt += 1;
        }
        eprintln!(
            "F={}: {} polytopes in {:.1}s ({} cache hits)",
            f,
            n,
            t0.elapsed().as_secs_f64(),
            hits_this_f
        );
    }

    // Known polytopes (HKO pentagon, simplex, hypercube)
    let known_polytopes_list: Vec<(String, Polytope4D)> = {
        let hko = known_polytopes::hko_pentagon();
        let mut list = vec![("hko_pentagon".to_string(), hko.polytope.clone())];
        for kp in &[known_polytopes::simplex(), known_polytopes::hypercube()] {
            if kp.polytope.facet_count() <= 10 {
                list.push((kp.name.to_string(), kp.polytope.clone()));
            } else {
                eprintln!(
                    "SKIP: {} (F={} > 10, too expensive for HK2017)",
                    kp.name,
                    kp.polytope.facet_count()
                );
            }
        }
        list
    };

    for (name, polytope) in &known_polytopes_list {
        let key: DualVerticesKey = polytope.dual_vertices().to_vec();
        let cached = db.get(&key).and_then(cached_capacity_from_record);

        match process_polytope(polytope, name, cached.as_ref()) {
            Some(row) => {
                serde_json::to_writer(&mut writer, &row).unwrap();
                writeln!(writer).unwrap();

                if !db.contains_key(&key) {
                    let mut record = PolytopeRecord::from_polytope(polytope);
                    record.source = Some(Source::Known { name: name.clone() });
                    record = record.with_computed_fields(row.volume, 0.0, row.capacity, 0.0);
                    record = record.with_sigmas(
                        vec![SigmaAction {
                            perm: row.orbit_facets.clone(),
                            action: row.capacity,
                        }],
                        0.0,
                    );
                    db.insert(key, record);
                }

                total += 1;
                eprintln!("{}: sys = {:.6}", name, row.sys);
            }
            None => {
                eprintln!("SKIP: {} (capacity computation returned None)", name);
                failed += 1;
            }
        }
    }

    writer.flush().unwrap();
    database::save(&owned_db_path, &db).expect("failed to save database");

    eprintln!(
        "\nDone: {} polytopes written, {} failed, {} cache hits. Database: {} entries.",
        total,
        failed,
        cache_hits,
        db.len()
    );
    eprintln!("Output: {}", out_path.display());
}
