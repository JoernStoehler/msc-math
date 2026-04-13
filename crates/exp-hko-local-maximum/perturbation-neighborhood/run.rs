//! Perturbations of the HK-O pentagon counterexample (10 facets).
//!
//! The binary generates N random dual-vertex perturbations of the HK-O pentagon
//! at a given eps magnitude, computes sys for each, and writes one row per sample
//! plus one unperturbed baseline row to an output .jsonl path.
//!
//! CLI (all optional; defaults match the original hardcoded run):
//! - `--n <count>`   number of perturbed samples               (default: 100)
//! - `--eps <f64>`   perturbation magnitude per component      (default: 0.01)
//! - `--seed <u64>`  base RNG seed                             (default: 41)
//! - `--out <path>`  output .jsonl path                        (default: pentagon-perturb.jsonl in CWD)
//!
//! Row identity across eps buckets is `(eps, name)` — `name` alone is not unique
//! between files generated at different eps. Analysis code must group by eps first.

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
use std::path::PathBuf;
use std::time::Instant;

const DEFAULT_SEED: u64 = 41;
const DEFAULT_N_SAMPLES: usize = 100;
const DEFAULT_EPS: f64 = 0.01;
/// Expected attempts per accepted sample is ~1 (rejection rate is negligible
/// at eps <= 0.1 for the HK-O pentagon). 20 gives headroom for larger eps.
const MAX_ATTEMPTS_PER_SAMPLE: usize = 20;

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
    eps: f64,
) -> Option<PerturbedPolytope> {
    let (dual_vertices, delta_dual_vertices) = jitter_dual_vertices(base_duals, rng, eps);

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

struct Args {
    n: usize,
    eps: f64,
    seed: u64,
    out: PathBuf,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut n = DEFAULT_N_SAMPLES;
    let mut eps = DEFAULT_EPS;
    let mut seed = DEFAULT_SEED;
    let mut out: Option<PathBuf> = None;

    let mut i = 1;
    while i < argv.len() {
        let arg = argv[i].as_str();
        let need_value = |flag: &str| -> &str {
            argv.get(i + 1)
                .map(|s| s.as_str())
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match arg {
            "--n" => {
                n = need_value("--n").parse().expect("--n must be a non-negative integer");
                i += 2;
            }
            "--eps" => {
                eps = need_value("--eps").parse().expect("--eps must be a finite f64");
                assert!(eps.is_finite() && eps > 0.0, "--eps must be positive and finite");
                i += 2;
            }
            "--seed" => {
                seed = need_value("--seed").parse().expect("--seed must be a u64");
                i += 2;
            }
            "--out" => {
                out = Some(PathBuf::from(need_value("--out")));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        n,
        eps,
        seed,
        out: out.unwrap_or_else(|| PathBuf::from("pentagon-perturb.jsonl")),
    }
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);

    if let Some(parent) = args.out.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output directory");
        }
    }

    println!("HK-O pentagon perturbation dataset");
    println!("  n:    {}", args.n);
    println!("  eps:  {:.6}", args.eps);
    println!("  seed: {}", args.seed);
    println!("  out:  {}", args.out.display());

    let file = File::create(&args.out).expect("failed to create output file");
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
        eps: args.eps,
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
    let mut total_attempts = 0usize;
    let max_total_attempts = args.n.saturating_mul(MAX_ATTEMPTS_PER_SAMPLE).max(2000);
    while accepted < args.n {
        if total_attempts >= max_total_attempts {
            panic!(
                "failed to generate {} valid perturbations after {max_total_attempts} attempts",
                args.n
            );
        }
        total_attempts += 1;

        let perturbed = match try_perturb(&base_duals, &mut rng, args.eps) {
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
            eps: args.eps,
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
    println!(
        "Wrote {} entries ({} perturbed + 1 base) to {}",
        args.n + 1,
        args.n,
        args.out.display()
    );
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}
