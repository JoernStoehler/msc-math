//! Perturbations of the HK-O pentagon counterexample (10 facets).
//!
//! Architecture:
//! 1. `cargo run -p exp-hko-local-maximum --release --bin hko-perturbation` generates dataset
//! 2. Writes to perturbation-neighborhood/pentagon-perturb.jsonl
//! 3. Python script analyzes and plots
//!
//! Dataset design:
//! - Start from the HK-O 2024 pentagon product (10 facets)
//! - Apply small random perturbations to dual vertices a_i directly
//! - 100 perturbed samples + 1 unperturbed baseline
//! - HK2017 pruned algorithm only

// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;
use symplectic::algorithms::hk2017::ehz_capacity;
use nalgebra::Vector4;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

const SEED: u64 = 41;
const N_SAMPLES: usize = 100;
const EPS: f64 = 0.01;
const MAX_ATTEMPTS: usize = 2000;

#[derive(Debug, Serialize)]
struct PentagonPerturbRow {
    name: String,
    sample_index: usize,
    is_base: bool,
    dual_vertices: Vec<[f64; 4]>,
    delta_dual_vertices: Vec<[f64; 4]>,
    eps: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

struct PerturbedPolytope {
    polytope: Polytope4D,
    dual_vertices: Vec<Vector4<f64>>,
    delta_dual_vertices: Vec<Vector4<f64>>,
}

/// Add a small random perturbation to each dual vertex a_i.
///
/// Returns (perturbed_vertices, deltas) where delta_i = perturbed_i - base_i.
fn jitter_dual_vertices(
    base: &[Vector4<f64>],
    rng: &mut ChaCha8Rng,
    eps: f64,
) -> (Vec<Vector4<f64>>, Vec<Vector4<f64>>) {
    let mut vertices = Vec::with_capacity(base.len());
    let mut deltas = Vec::with_capacity(base.len());

    for a in base.iter() {
        let delta = Vector4::new(
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
        );
        let candidate = a + delta;
        vertices.push(candidate);
        deltas.push(delta);
    }

    (vertices, deltas)
}

fn try_perturb(
    base_duals: &[Vector4<f64>],
    rng: &mut ChaCha8Rng,
) -> Option<PerturbedPolytope> {
    let (dual_vertices, delta_dual_vertices) = jitter_dual_vertices(base_duals, rng, EPS);

    let polytope = Polytope4D::from_f64(dual_vertices.clone()).ok()?;

    Some(PerturbedPolytope {
        polytope,
        dual_vertices,
        delta_dual_vertices,
    })
}

fn v4_to_array(v: &Vector4<f64>) -> [f64; 4] {
    [v[0], v[1], v[2], v[3]]
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pentagon-perturb/pentagon-perturb.jsonl");

    println!("Generating HK-O pentagon perturbation dataset...\n");
    println!("Perturbation eps: {EPS:.4}");

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let base = known_polytopes::hko_pentagon();
    let base_polytope = &base.polytope;
    let base_duals: Vec<Vector4<f64>> = base_polytope.dual_vertices_f64().to_vec();
    let n_facets = base_duals.len();

    let start_vol = Instant::now();
    let base_vol = volume(base_polytope).expect("volume computation failed");
    let base_time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

    let start_cap = Instant::now();
    let base_result = ehz_capacity(base_polytope).expect("capacity computation failed");
    let base_time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

    let base_sys = base_result.result.capacity * base_result.result.capacity / (2.0 * base_vol);

    let base_row = PentagonPerturbRow {
        name: "hko_pentagon_base".to_string(),
        sample_index: 0,
        is_base: true,
        dual_vertices: base_duals.iter().map(v4_to_array).collect(),
        delta_dual_vertices: vec![[0.0; 4]; n_facets],
        eps: EPS,
        volume: base_vol,
        capacity: base_result.result.capacity,
        sys: base_sys,
        iterations: base_result.result.iterations,
        time_volume_ms: base_time_volume_ms,
        time_capacity_ms: base_time_capacity_ms,
    };

    let line = serde_json::to_string(&base_row).expect("serialize row");
    writeln!(writer, "{line}").expect("write line");

    let mut accepted = 0usize;
    let mut attempts = 0usize;
    while accepted < N_SAMPLES {
        if attempts >= MAX_ATTEMPTS {
            panic!(
                "failed to generate {N_SAMPLES} valid perturbations after {MAX_ATTEMPTS} attempts"
            );
        }
        attempts += 1;

        let perturbed = match try_perturb(&base_duals, &mut rng) {
            Some(p) => p,
            None => continue,
        };

        let start_vol = Instant::now();
        let vol = volume(&perturbed.polytope).expect("volume computation failed");
        let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

        let start_cap = Instant::now();
        let result = ehz_capacity(&perturbed.polytope)
            .expect("capacity computation failed");
        let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

        let cap = result.result.capacity;
        let sys = cap * cap / (2.0 * vol);

        let row = PentagonPerturbRow {
            name: format!("hko_pentagon_perturbed_{accepted}"),
            sample_index: accepted + 1,
            is_base: false,
            dual_vertices: perturbed.dual_vertices.iter().map(v4_to_array).collect(),
            delta_dual_vertices: perturbed.delta_dual_vertices.iter().map(v4_to_array).collect(),
            eps: EPS,
            volume: vol,
            capacity: cap,
            sys,
            iterations: result.result.iterations,
            time_volume_ms,
            time_capacity_ms,
        };

        let line = serde_json::to_string(&row).expect("serialize row");
        writeln!(writer, "{line}").expect("write line");
        accepted += 1;
    }

    writer.flush().expect("flush output");
    println!("\nWrote {} entries to {}", N_SAMPLES + 1, output_path.display());
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
