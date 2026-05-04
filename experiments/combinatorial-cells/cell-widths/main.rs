//! Cell Widths: per-facet cell width measurement in dual-vertex space.
//!
//! Location: experiments/combinatorial-cells/cell-widths/main.rs
//!
//! For polytopes K = {x : a_k . x <= 1}, the combinatorial type (vertex-facet incidence,
//! omega_0 sign pattern) is constant within open regions of dual-vertex space R^{4F}. This
//! experiment measures the cell width per facet by probing random S^3 directions in each
//! facet's R^4 subspace. No EHZ capacity computation needed -- only boundary detection.
//!
//! For each facet k, probe N_FACET_DIRS random S^3 directions in R^4_k.
//! Measures cell width per facet, anisotropy, orbit-facet narrowness.
//!
//! Split from combinatorial-structure (Pass 1).
//!
//! Input Artifacts: experiments/combinatorial-cells/polytopes.jsonl (owned cache)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output Artifacts: experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl

use exp_combinatorial_cells::{
    compute_step_bound_detailed, ehz_capacity_instrumented, name_from_record,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::database;
use symplectic::geom::polytope::Polytope4D;

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Number of random S^3 directions per facet for cell profiling.
/// 10 directions in R^4 give reasonable coverage of S^3.
const N_FACET_DIRS: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
/// If changed: only affects the "unbounded" classification, not actual boundary detection.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate. Used in step bounds and gradient checks.
/// If changed: values much larger risk missing real boundaries; much smaller risks
/// false positives from floating-point noise.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Random seed for reproducibility.
const SEED: u64 = 42;

// ============================================================================
// Direction types
// ============================================================================

/// A probe direction in dual-vertex space R^{4F}.
#[derive(Debug, Clone)]
struct Direction {
    /// Type label for the JSONL output.
    dir_type: String,
    /// Index within the type.
    index: usize,
    /// Which facet this direction perturbs (None for global/dense directions).
    facet_index: Option<usize>,
    /// Direction vector: one Vector4 per facet. Step: a'_k(t) = a_k + t*d[k].
    d: Vec<Vector4<f64>>,
}

// ============================================================================
// Output schema
// ============================================================================

/// Per-facet cell profiling row.
#[derive(Debug, Serialize)]
struct ProfilingRow {
    polytope_name: String,
    facet_count: usize,
    facet_index: usize,
    facet_in_orbit: bool,
    direction_index: usize,
    t_max: f64,
    event_type: String,
}

// ============================================================================
// Direction construction
// ============================================================================

/// Build per-facet directions: N_FACET_DIRS random S^3 directions per facet.
/// Each direction is nonzero only in R^4_k for facet k.
fn build_facet_directions(f: usize, rng: &mut ChaCha8Rng) -> Vec<Direction> {
    let mut dirs = Vec::with_capacity(f * N_FACET_DIRS);
    for k in 0..f {
        for i in 0..N_FACET_DIRS {
            let raw: Vector4<f64> = Vector4::new(
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
            );
            let norm = raw.norm();
            if norm > EPS_NUMERICAL_ZERO {
                let mut d = vec![Vector4::zeros(); f];
                d[k] = raw / norm;
                dirs.push(Direction {
                    dir_type: "facet".to_string(),
                    index: i,
                    facet_index: Some(k),
                    d,
                });
            }
        }
    }
    dirs
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Profiling: per-facet cell width measurement\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from owned cache (F <= {MAX_FACET_COUNT})...");

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("polytopes.jsonl");
    let db = database::load_many(&[owned_db_path.as_path()]).expect("failed to load database");

    let mut polytopes: Vec<(String, Polytope4D)> = Vec::new();

    for (idx, (_, record)) in db.iter().enumerate() {
        let f = record.dual_vertices_rational.len();
        if f > MAX_FACET_COUNT {
            continue;
        }
        let p = match record.to_polytope() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  db entry {idx}: reconstruction failed: {e}");
                continue;
            }
        };
        let name = name_from_record(record, idx);
        polytopes.push((name, p));
    }

    let n_polytopes = polytopes.len();
    println!("  {n_polytopes} polytopes loaded from database (F <= {MAX_FACET_COUNT})\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes in database. Run sys-random-sample and sys-random-product-sample first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Open output file
    // =========================================================================

    let out_dir = base_dir.join("cell-widths");
    let profiling_path = out_dir.join("combinatorial-boundaries-profiling.jsonl");
    let profiling_file = File::create(&profiling_path)
        .unwrap_or_else(|err| panic!("create profiling JSONL {}: {err}", profiling_path.display()));
    let mut profiling_writer = BufWriter::new(profiling_file);

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_profiling = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();

        // =====================================================================
        // Base computation: instrumented EHZ for orbit membership
        // =====================================================================

        let (perm,) = match ehz_capacity_instrumented(polytope) {
            Some(instrumented) => (instrumented.best_permutation,),
            None => {
                n_skipped += 1;
                continue;
            }
        };

        // Which facets are in the optimal orbit?
        let orbit_facets: Vec<bool> = (0..f).map(|k| perm.contains(&k)).collect();

        // =====================================================================
        // Per-facet cell profiling (cheap, no EHZ)
        // =====================================================================

        let facet_dirs = build_facet_directions(f, &mut rng);

        for dir in &facet_dirs {
            let boundary =
                compute_step_bound_detailed(polytope, &dir.d, EPS_NUMERICAL_ZERO, MAX_STEP_SIZE);
            let k = dir.facet_index.unwrap();

            let row = ProfilingRow {
                polytope_name: name.clone(),
                facet_count: f,
                facet_index: k,
                facet_in_orbit: orbit_facets[k],
                direction_index: dir.index,
                t_max: boundary.t_max,
                event_type: boundary.event.name().to_string(),
            };

            serde_json::to_writer(&mut profiling_writer, &row).unwrap();
            writeln!(profiling_writer).unwrap();
            total_profiling += 1;
        }

        // =====================================================================
        // Progress reporting
        // =====================================================================

        let elapsed = t_poly.elapsed().as_secs_f64();
        if (idx + 1) % 10 == 0 || idx + 1 == n_polytopes {
            println!(
                "  [{}/{}] {}: F={}, {:.1}s",
                idx + 1,
                n_polytopes,
                name,
                f,
                elapsed
            );
        }
    }

    // =========================================================================
    // Flush and report
    // =========================================================================

    profiling_writer.flush().unwrap();

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Profiling rows: {total_profiling}");
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}
