//! Perturbations of the HK-O pentagon counterexample (10 facets).
//!
//! Architecture:
//! 1. `cargo run --bin pentagon_perturb --release` generates dataset
//! 2. Writes to pentagon-perturb/pentagon-perturb.jsonl
//! 3. Python script analyzes and plots
//!
//! Dataset design:
//! - Start from the HK-O 2024 pentagon product (10 facets)
//! - Apply small random perturbations to normals and heights
//! - 100 perturbed samples + 1 unperturbed baseline
//! - HK2017 pruned algorithm only

use symplectic::known_polytopes;
use symplectic::Polytope4D;
use symplectic::volume;
use symplectic::ehz_capacity;
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
const EPS_NORMALS: f64 = 0.01;
const EPS_HEIGHTS: f64 = 0.01;
const MAX_ATTEMPTS: usize = 2000;

#[derive(Debug, Serialize)]
struct PentagonPerturbRow {
    name: String,
    sample_index: usize,
    is_base: bool,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    delta_normals: Vec<[f64; 4]>,
    delta_heights: Vec<f64>,
    eps_normals: f64,
    eps_heights: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

struct PerturbedPolytope {
    polytope: Polytope4D,
    normals: Vec<Vector4<f64>>,
    heights: Vec<f64>,
    delta_normals: Vec<Vector4<f64>>,
    delta_heights: Vec<f64>,
}

fn jitter_normals(
    base: &[Vector4<f64>],
    rng: &mut ChaCha8Rng,
    eps: f64,
) -> (Vec<Vector4<f64>>, Vec<Vector4<f64>>) {
    let mut normals = Vec::with_capacity(base.len());
    let mut deltas = Vec::with_capacity(base.len());

    for n in base.iter() {
        let delta_raw = Vector4::new(
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
            rng.gen_range(-eps..=eps),
        );
        let mut candidate = n + delta_raw;
        let norm = candidate.norm();
        if norm == 0.0 {
            candidate = *n;
        } else {
            candidate /= norm;
        }
        let delta = candidate - n;
        normals.push(candidate);
        deltas.push(delta);
    }

    (normals, deltas)
}

fn jitter_heights(base: &[f64], rng: &mut ChaCha8Rng, eps: f64) -> (Vec<f64>, Vec<f64>) {
    let mut heights = Vec::with_capacity(base.len());
    let mut deltas = Vec::with_capacity(base.len());

    for h in base.iter() {
        let delta = rng.gen_range(-eps..=eps);
        let candidate = h + delta;
        heights.push(candidate);
        deltas.push(delta);
    }

    (heights, deltas)
}

fn try_perturb(
    base_normals: &[Vector4<f64>],
    base_heights: &[f64],
    rng: &mut ChaCha8Rng,
) -> Option<PerturbedPolytope> {
    let (normals, delta_normals) = jitter_normals(base_normals, rng, EPS_NORMALS);
    let (heights, delta_heights) = jitter_heights(base_heights, rng, EPS_HEIGHTS);

    if heights.iter().any(|h| *h <= 0.0) {
        return None;
    }

    let polytope = Polytope4D::new(normals.clone(), heights.clone()).ok()?;

    Some(PerturbedPolytope {
        polytope,
        normals,
        heights,
        delta_normals,
        delta_heights,
    })
}

fn main() {
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pentagon-perturb/pentagon-perturb.jsonl");

    println!("Generating HK-O pentagon perturbation dataset...\n");
    println!(
        "Perturbation eps: normals={EPS_NORMALS:.4}, heights={EPS_HEIGHTS:.4}"
    );

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let base = known_polytopes::hko_pentagon();
    let base_polytope = base.polytope;
    let base_normals = base_polytope.normals().to_vec();
    let base_heights = base_polytope.heights().to_vec();

    let start_vol = Instant::now();
    let base_vol = volume(&base_polytope).expect("volume computation failed");
    let base_time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

    let start_cap = Instant::now();
    let base_result = ehz_capacity(&base_polytope).expect("capacity computation failed");
    let base_time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

    let base_sys = base_result.capacity * base_result.capacity / (2.0 * base_vol);

    let base_row = PentagonPerturbRow {
        name: "hko_pentagon_base".to_string(),
        sample_index: 0,
        is_base: true,
        normals: base_normals
            .iter()
            .map(|n| [n[0], n[1], n[2], n[3]])
            .collect(),
        heights: base_heights.clone(),
        delta_normals: vec![[0.0; 4]; base_heights.len()],
        delta_heights: vec![0.0; base_heights.len()],
        eps_normals: EPS_NORMALS,
        eps_heights: EPS_HEIGHTS,
        volume: base_vol,
        capacity: base_result.capacity,
        sys: base_sys,
        iterations: base_result.iterations,
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

        let perturbed = match try_perturb(&base_normals, &base_heights, &mut rng) {
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

        let cap = result.capacity;
        let sys = cap * cap / (2.0 * vol);

        let row = PentagonPerturbRow {
            name: format!("hko_pentagon_perturbed_{accepted}"),
            sample_index: accepted + 1,
            is_base: false,
            normals: perturbed
                .normals
                .iter()
                .map(|n| [n[0], n[1], n[2], n[3]])
                .collect(),
            heights: perturbed.heights.clone(),
            delta_normals: perturbed
                .delta_normals
                .iter()
                .map(|n| [n[0], n[1], n[2], n[3]])
                .collect(),
            delta_heights: perturbed.delta_heights.clone(),
            eps_normals: EPS_NORMALS,
            eps_heights: EPS_HEIGHTS,
            volume: vol,
            capacity: cap,
            sys,
            iterations: result.iterations,
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
