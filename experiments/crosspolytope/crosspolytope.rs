//! Compute EHZ capacity of the 4D crosspolytope (hyperoctahedron).
//!
//! Goal: Fill the placeholder capacity in `crates/src/geom/known_polytopes.rs`.
//! Input: Crosspolytope from `known_polytopes::crosspolytope()` (16 facets).
//! Output: `experiments/crosspolytope/crosspolytope.jsonl`
//!
//! The crosspolytope has 16 facets, making this a multi-hour computation.
//! Run in release mode:
//!   cd experiments/ && timeout 14h cargo run --release --bin crosspolytope

use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::known_polytopes;
use symplectic::{ehz_capacity, volume};

#[derive(Debug, Serialize)]
struct CrosspolytopeResult {
    name: String,
    facet_count: usize,
    volume: f64,
    capacity: f64,
    capacity_uncertain: f64,
    numerical_gap: f64,
    sys: f64,
    iterations: u64,
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    best_beta: Vec<f64>,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

fn main() {
    let t0 = Instant::now();

    let kp = known_polytopes::crosspolytope();
    let polytope = &kp.polytope;
    println!("Crosspolytope: {} facets", polytope.facet_count());

    // Volume
    let start_vol = Instant::now();
    let vol = volume(polytope).expect("volume computation failed");
    let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;
    println!("Volume: {vol:.10} ({time_volume_ms:.1} ms)");

    // EHZ capacity (pruned HK2017)
    println!("\nComputing EHZ capacity (pruned)... this may take hours.");
    let start_cap = Instant::now();
    let result = ehz_capacity(polytope).expect("capacity computation returned None");
    let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

    let cap = result.capacity;
    let sys = cap * cap / (2.0 * vol);

    println!("\n=== Results ===");
    println!("Capacity (certified):  {:.10}", result.capacity);
    println!("Capacity (uncertain):  {:.10}", result.capacity_uncertain);
    println!("Numerical gap:         {:.2e}", result.numerical_gap());
    println!("Volume:                {vol:.10}");
    println!("Systolic ratio:        {sys:.10}");
    println!("Iterations:            {}", result.iterations);
    println!("Time (capacity):       {:.1} s", time_capacity_ms / 1000.0);
    println!("Best subset (facets):  {:?}", result.best_subset);
    println!("Best permutation:      {:?}", result.best_permutation);
    println!("Best beta:             {:?}", result.best_beta);
    println!(
        "Viterbo conjecture:    {}",
        if sys <= 1.0 { "SATISFIED (sys <= 1)" } else { "VIOLATED (sys > 1)" }
    );

    // Write JSONL
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("crosspolytope/crosspolytope.jsonl");
    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let row = CrosspolytopeResult {
        name: "crosspolytope_4d".to_string(),
        facet_count: polytope.facet_count(),
        volume: vol,
        capacity: result.capacity,
        capacity_uncertain: result.capacity_uncertain,
        numerical_gap: result.numerical_gap(),
        sys,
        iterations: result.iterations,
        best_subset: result.best_subset,
        best_permutation: result.best_permutation,
        best_beta: result.best_beta,
        time_volume_ms,
        time_capacity_ms,
    };

    let line = serde_json::to_string(&row).expect("serialize row");
    writeln!(writer, "{line}").expect("write line");
    writer.flush().expect("flush output");

    println!("\nWrote results to {}", output_path.display());
    println!("Total time: {:.1} s", t0.elapsed().as_secs_f64());
}
