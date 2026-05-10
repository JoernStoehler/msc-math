//! Resumable, symmetry-reduced EHZ capacity computation for the 4D crosspolytope.
//!
//! Goal: preserve and, if needed, extend the computation supporting the
//! crosspolytope capacity recorded in `known_polytopes::crosspolytope()`.
//! Input Artifacts: Crosspolytope from `known_polytopes::crosspolytope()` (16 facets).
//! Output Artifacts: `experiments/crosspolytope/main/crosspolytope.jsonl`
//!
//! Three optimizations over the library's `ehz_capacity()`:
//! 1. backtracking permutation search
//! 2. symmetry reduction
//! 3. checkpointing

mod checkpoint;
mod kkt;
mod search;

use nalgebra::Vector4;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::geom::known_polytopes;
use symplectic::geom::volume::volume_f64;

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
    symmetry_group_order: usize,
    hyperoctahedral_group_order: usize,
    search_complete_through_m: usize,
}

fn main() {
    let t0 = Instant::now();

    let kp = known_polytopes::crosspolytope();
    let polytope = &kp.polytope;
    let facet_count = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    println!("Crosspolytope: {facet_count} facets");

    let start_vol = Instant::now();
    let vol = volume_f64(polytope);
    let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;
    println!("Volume: {vol:.10} ({time_volume_ms:.1} ms)");

    let search = search::run_crosspolytope_search(polytope, &normals, &heights);
    let time_capacity_ms = search.elapsed_secs * 1000.0;

    let certified = search.best_certified;
    let uncertain_cap = search.best_uncertain.map_or(certified.0, |b| b.0);
    assert!(
        uncertain_cap <= certified.0,
        "Unexpected: uncertain capacity {:.6e} > certified {:.6e}",
        uncertain_cap,
        certified.0
    );

    let cap = certified.0;
    let best_subset = certified.1;
    let best_permutation = certified.2;
    let best_beta = certified.3;
    let sys = cap * cap / (2.0 * vol);

    println!("\n=== Results ===");
    println!("Capacity (certified):  {cap:.10}");
    println!("Capacity (uncertain):  {uncertain_cap:.10}");
    println!("Numerical gap:         {:.2e}", cap - uncertain_cap);
    println!("Volume:                {vol:.10}");
    println!("Systolic ratio:        {sys:.10}");
    println!("Iterations:            {}", search.iterations);
    println!("Time (capacity):       {:.1} s", time_capacity_ms / 1000.0);
    println!("Best subset (facets):  {:?}", best_subset);
    println!("Best permutation:      {:?}", best_permutation);
    println!("Best beta:             {:?}", best_beta);
    println!("Symmetry group order:  {}", search.symmetry_group_order);
    println!(
        "Viterbo conjecture:    {}",
        if sys <= 1.0 {
            "SATISFIED (sys <= 1)"
        } else {
            "VIOLATED (sys > 1)"
        }
    );

    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("main/crosspolytope.jsonl");
    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let row = CrosspolytopeResult {
        name: "crosspolytope_4d".to_string(),
        facet_count,
        volume: vol,
        capacity: cap,
        capacity_uncertain: uncertain_cap,
        numerical_gap: cap - uncertain_cap,
        sys,
        iterations: search.iterations,
        best_subset,
        best_permutation,
        best_beta,
        time_volume_ms,
        time_capacity_ms,
        symmetry_group_order: search.symmetry_group_order,
        hyperoctahedral_group_order: 384,
        search_complete_through_m: search.search_complete_through_m,
    };

    let line = serde_json::to_string(&row).expect("serialize row");
    writeln!(writer, "{line}").expect("write line");
    writer.flush().expect("flush output");

    println!("\nWrote results to {}", output_path.display());
    println!("Total time: {:.1} s", t0.elapsed().as_secs_f64());

    let cp_path = checkpoint::checkpoint_path();
    if cp_path.exists() {
        std::fs::remove_file(&cp_path).ok();
        println!("Removed checkpoint file.");
    }
}
