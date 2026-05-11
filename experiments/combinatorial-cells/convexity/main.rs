//! Convexity: midpoint combinatorial-type checks for cell convexity.
//!
//! Location: experiments/combinatorial-cells/convexity/main.rs
//!
//! Tests whether combinatorial-type cells in dual-vertex space are convex by sampling
//! pairs of boundary probes and checking if the midpoint has the same combinatorial type.
//!
//! First computes per-facet boundary probes (same as cell-widths) to find
//! interior points near the cell boundary, then tests midpoints of pairs of such points.
//! Checks three levels: incidence preservation, omega_0 sign preservation, transition
//! matrix preservation.
//!
//! Split from combinatorial-structure (Pass 3, with inlined Pass 1 boundary computation).
//!
//! Input Artifacts: experiments/combinatorial-cells/polytopes.jsonl (owned cache)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output Artifacts: experiments/combinatorial-cells/convexity/combinatorial-boundaries-convexity.jsonl

use euclidean_polytopes::{
    two_faces_from_vertex_facet_incidence, vertex_facets_from_vertex_facet_incidence,
};
use exp_combinatorial_cells::CellPolytopeCache;
use exp_combinatorial_cells::{
    compute_step_bound_detailed, construct_at_t, ehz_capacity_instrumented, name_from_record,
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
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::database;
use symplectic::geom::symplectic_form::omega0;

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Number of random S^3 directions per facet for boundary probing.
/// 10 directions in R^4 give reasonable coverage of S^3.
const N_FACET_DIRS: usize = 10;

/// Number of direction pairs sampled per polytope for convexity testing.
/// Mix of same-facet and cross-facet pairs.
const N_CONVEXITY_PAIRS: usize = 20;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate.
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

/// Convexity testing row.
#[derive(Debug, Serialize)]
struct ConvexityRow {
    polytope_name: String,
    facet_count: usize,
    dir1_facet: usize,
    dir1_index: usize,
    dir2_facet: usize,
    dir2_index: usize,
    t1_max: f64,
    t2_max: f64,
    midpoint_same_incidence: bool,
    midpoint_same_omega_signs: bool,
    midpoint_same_transitions: bool,
    midpoint_construction_ok: bool,
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
// Combinatorial type comparison (for convexity testing)
// ============================================================================

/// Combinatorial type signature: sorted vertex-facet incidence + omega signs.
/// Two polytopes with the same signature have the same combinatorial type.
struct CombinatorialType {
    /// Sorted list of sorted vertex-facet incidence vectors.
    vertex_facets: Vec<Vec<usize>>,
    /// Sorted list of (facet_i, facet_j, sign_positive) for two-face-adjacent pairs.
    omega_signs: Vec<(usize, usize, bool)>,
}

fn combinatorial_type(polytope: &CellPolytopeCache) -> CombinatorialType {
    let vertex_facets_by_vertex =
        vertex_facets_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);
    let duals = &polytope.dual_vertices_f64;

    let mut vf: Vec<Vec<usize>> = vertex_facets_by_vertex
        .iter()
        .map(|facets| {
            let mut f = facets.clone();
            f.sort();
            f
        })
        .collect();
    vf.sort();

    let mut omega_signs: Vec<(usize, usize, bool)> = two_faces
        .iter()
        .map(|two_face| {
            let i = two_face.facets[0].min(two_face.facets[1]);
            let j = two_face.facets[0].max(two_face.facets[1]);
            let sign = omega0(&duals[i], &duals[j]) >= 0.0;
            (i, j, sign)
        })
        .collect();
    omega_signs.sort();

    CombinatorialType {
        vertex_facets: vf,
        omega_signs,
    }
}

fn same_incidence(a: &CombinatorialType, b: &CombinatorialType) -> bool {
    a.vertex_facets == b.vertex_facets
}

fn same_omega(a: &CombinatorialType, b: &CombinatorialType) -> bool {
    a.omega_signs == b.omega_signs
}

/// Compare transition matrices:
/// facet intersection nonemptiness + omega_0 signs -> directed facet graph.
/// This is what actually determines which Reeb orbits are feasible.
fn same_transitions(base: &CellPolytopeCache, other: &CellPolytopeCache) -> bool {
    let base_facet_intersection_is_nonempty = &base.facet_intersection_is_nonempty;
    let base_omega_signs = &base.omega_signs;
    let t1 = build_transition_matrix_from_facet_intersections_and_omega(
        &base_facet_intersection_is_nonempty,
        &base_omega_signs,
    );
    let other_facet_intersection_is_nonempty = &other.facet_intersection_is_nonempty;
    let other_omega_signs = &other.omega_signs;
    let t2 = build_transition_matrix_from_facet_intersections_and_omega(
        &other_facet_intersection_is_nonempty,
        &other_omega_signs,
    );
    t1 == t2
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Convexity: midpoint combinatorial-type checks\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from owned cache (F <= {MAX_FACET_COUNT})...");

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("polytopes.jsonl");
    let db = database::load_many(&[owned_db_path.as_path()]).expect("failed to load database");

    let mut polytopes: Vec<(String, CellPolytopeCache)> = Vec::new();

    for (idx, (_, record)) in db.iter().enumerate() {
        let f = record.dual_vertices_rational.len();
        if f > MAX_FACET_COUNT {
            continue;
        }
        let p = match exp_combinatorial_cells::cache_from_record(record) {
            Some(p) => p,
            None => {
                eprintln!("  db entry {idx}: reconstruction failed");
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

    let out_dir = base_dir.join("convexity");
    let convexity_path = out_dir.join("combinatorial-boundaries-convexity.jsonl");
    let convexity_file = File::create(&convexity_path)
        .unwrap_or_else(|err| panic!("create convexity JSONL {}: {err}", convexity_path.display()));
    let mut convexity_writer = BufWriter::new(convexity_file);

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_convexity = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();
        let duals = &polytope.dual_vertices_f64;

        // =====================================================================
        // Base computation: need orbit membership for consistent output schema
        // (ConvexityRow doesn't use orbit info, but we need base to succeed
        // to ensure polytope is valid for EHZ-based experiments.)
        // =====================================================================

        let _perm = match ehz_capacity_instrumented(polytope) {
            Some(instrumented) => instrumented.best_permutation,
            None => {
                n_skipped += 1;
                continue;
            }
        };

        // Base combinatorial type (for convexity testing)
        let base_type = combinatorial_type(polytope);

        // =====================================================================
        // Inlined Pass 1: per-facet boundary probes (cheap, no EHZ)
        // These provide the interior points used for midpoint convexity tests.
        // =====================================================================

        let facet_dirs = build_facet_directions(f, &mut rng);

        struct FacetProbe {
            facet: usize,
            dir_index: usize,
            t_max: f64,
            direction: Vec<Vector4<f64>>,
        }
        let mut probes: Vec<FacetProbe> = Vec::new();

        for dir in &facet_dirs {
            let boundary =
                compute_step_bound_detailed(polytope, &dir.d, EPS_NUMERICAL_ZERO, MAX_STEP_SIZE);
            let k = dir.facet_index.unwrap();

            // Store for convexity testing (skip unbounded)
            if boundary.t_max < MAX_STEP_SIZE {
                probes.push(FacetProbe {
                    facet: k,
                    dir_index: dir.index,
                    t_max: boundary.t_max,
                    direction: dir.d.clone(),
                });
            }
        }

        // =====================================================================
        // Convexity testing
        // =====================================================================

        if probes.len() >= 2 {
            let n_pairs = N_CONVEXITY_PAIRS.min(probes.len() * (probes.len() - 1) / 2);

            // Sample pairs: mix of same-facet and cross-facet
            let mut pairs_tested = 0;
            let mut pair_idx = 0;

            // Deterministic sampling: stride through all pairs
            let total_pairs = probes.len() * (probes.len() - 1) / 2;
            let stride = if total_pairs > n_pairs {
                total_pairs / n_pairs
            } else {
                1
            };

            'pairs: for i in 0..probes.len() {
                for j in (i + 1)..probes.len() {
                    if pair_idx % stride == 0 {
                        let p1 = &probes[i];
                        let p2 = &probes[j];

                        // Interior points: 0.5 * t_max along each direction
                        let t1 = 0.5 * p1.t_max;
                        let t2 = 0.5 * p2.t_max;

                        // Midpoint direction: 0.5 * (t1*d1 + t2*d2)
                        let mid_dir: Vec<Vector4<f64>> = p1
                            .direction
                            .iter()
                            .zip(p2.direction.iter())
                            .map(|(d1, d2)| 0.5 * (t1 * d1 + t2 * d2))
                            .collect();

                        // Construct polytope at midpoint (a + mid_dir, i.e. t=1)
                        let mut construction_ok = false;
                        let mut same_incidence_val = false;
                        let mut same_omega_val = false;
                        let mut same_transitions_val = false;

                        if let Some(mid_poly) = construct_at_t(duals, &mid_dir, 1.0) {
                            construction_ok = true;
                            let mid_type = combinatorial_type(&mid_poly);
                            same_incidence_val = same_incidence(&base_type, &mid_type);
                            same_omega_val = same_omega(&base_type, &mid_type);
                            same_transitions_val = same_transitions(polytope, &mid_poly);
                        }

                        let row = ConvexityRow {
                            polytope_name: name.clone(),
                            facet_count: f,
                            dir1_facet: p1.facet,
                            dir1_index: p1.dir_index,
                            dir2_facet: p2.facet,
                            dir2_index: p2.dir_index,
                            t1_max: p1.t_max,
                            t2_max: p2.t_max,
                            midpoint_same_incidence: same_incidence_val,
                            midpoint_same_omega_signs: same_omega_val,
                            midpoint_same_transitions: same_transitions_val,
                            midpoint_construction_ok: construction_ok,
                        };

                        serde_json::to_writer(&mut convexity_writer, &row).unwrap();
                        writeln!(convexity_writer).unwrap();
                        total_convexity += 1;
                        pairs_tested += 1;

                        if pairs_tested >= n_pairs {
                            break 'pairs;
                        }
                    }
                    pair_idx += 1;
                }
            }
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

    convexity_writer.flush().unwrap();

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Convexity rows: {total_convexity}");
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}
