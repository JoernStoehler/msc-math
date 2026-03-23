//! Generate random polytope seeds for gradient search.
//!
//! Appends random polytopes to seeds.jsonl. Run multiple times to add more.
//! Each polytope is identified by a unique seed_id.
//!
//! Usage: cargo run --bin generate_seeds --release
//! Output: gradient-search/seeds.jsonl (append mode)

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use symplectic::random::generate_random_polytopes;

/// Date-based seed for reproducibility. Different from other experiments' seeds
/// to avoid correlation with existing datasets.
const BASE_SEED: u64 = 2026_03_23_00;
/// Height range centered around 1.0 with ±20% spread. Same as gradient-descent
/// and random-sweep experiments. Narrower → nearly spherical (boring), wider →
/// highly elongated (more degenerate KKT solutions).
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_seeds). Production seed counts.
/// F=7-8: cheap (~1ms/eval), many seeds for breadth.
/// F=9-10: expensive (~20ms/eval), fewer seeds but where highest sys values live.
/// Adjust and re-run generate_seeds to expand the dataset.
const PLAN: &[(usize, usize)] = &[
    (7, 100),
    (8, 100),
    (9, 50),
    (10, 20),
];

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedRow {
    pub seed_id: u64,
    pub facet_count: usize,
    pub normals: Vec<[f64; 4]>,
    pub heights: Vec<f64>,
}

fn main() {
    let out_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("gradient-search/seeds.jsonl");
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    // Read existing seed_ids to avoid duplicates
    let existing: HashSet<u64> = if out_path.exists() {
        let f = std::fs::File::open(&out_path).unwrap();
        BufReader::new(f)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| serde_json::from_str::<SeedRow>(&l).ok())
            .map(|r| r.seed_id)
            .collect()
    } else {
        HashSet::new()
    };
    eprintln!("{} existing seeds", existing.len());

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .unwrap_or_else(|e| panic!("cannot open {}: {e}", out_path.display()));
    let mut writer = BufWriter::new(file);

    let mut seed_id = 0u64;
    let mut added = 0usize;

    for &(f, n) in PLAN {
        let mut rng = ChaCha8Rng::seed_from_u64(BASE_SEED.wrapping_add(seed_id));
        let polytopes = generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);

        for p in &polytopes {
            if !existing.contains(&seed_id) {
                let row = SeedRow {
                    seed_id,
                    facet_count: f,
                    normals: p.normals_f64().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
                    heights: p.heights_f64(),
                };
                writeln!(writer, "{}", serde_json::to_string(&row).unwrap()).unwrap();
                added += 1;
            }
            seed_id += 1;
        }
        eprintln!("F={f}: generated {n} polytopes (seed_id {}-{})", seed_id - n as u64, seed_id - 1);
    }

    writer.flush().unwrap();
    eprintln!("Added {added} new seeds to {}", out_path.display());
}
