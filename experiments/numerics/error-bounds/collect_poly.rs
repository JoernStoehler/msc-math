//! Stage 1 (natural): Enumerate polytope σ-nodes → collected_poly.jsonl.
//!
//! For each polytope in the input JSONL, enumerates all subsets S and cyclic
//! permutations σ, assembles (H, C, d), runs the f64 saddle-point solver,
//! and saves input matrices + raw solver output (β, λ, etc.).
//!
//! Usage:
//!   cargo run -p dev-numerical-analysis --release --bin num-collect-poly -- --polytopes /tmp/all_polytopes.jsonl [--max-facets 8]
//! Input Artifacts: None (reads the --polytopes JSONL path passed on the CLI).
//! Output Artifacts: experiments/numerics/error-bounds/collected_poly.jsonl

use nalgebra::{DMatrix, DVector, Vector4};
use serde::Deserialize;
use std::io::BufRead;

#[path = "collect_common.rs"]
mod common;

use common::{solve_and_record, write_jsonl, print_summary, InputRow, P};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::omega0;

#[derive(Deserialize)]
struct PolytopeInput {
    dual_vertices: Vec<[f64; 4]>,
    #[serde(default)]
    facet_count: Option<usize>,
}

fn generate_natural(polytopes_path: &str, max_facets: usize) -> Vec<InputRow> {
    let file = std::fs::File::open(polytopes_path)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", polytopes_path, e));
    let reader = std::io::BufReader::new(file);

    let mut rows = Vec::new();
    let mut instance_counter = 0usize;

    for (poly_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|e| panic!("Read error at line {}: {}", poly_idx, e));
        if line.trim().is_empty() {
            continue;
        }
        let poly_row: PolytopeInput = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("JSON parse error at line {}: {}", poly_idx, e));

        let f = poly_row.dual_vertices.len();
        if f > max_facets {
            continue;
        }

        let dual_verts: Vec<Vector4<f64>> = poly_row
            .dual_vertices
            .iter()
            .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
            .collect();

        for m in 2..=f {
            for subset in combinations(f, m) {
                for_each_cyclic_permutation(&subset, &mut |perm| {
                    let mut h = DMatrix::zeros(m, m);
                    for i in 0..m {
                        for j in (i + 1)..m {
                            let val = omega0(&dual_verts[perm[i]], &dual_verts[perm[j]]);
                            h[(i, j)] = val;
                            h[(j, i)] = val;
                        }
                    }

                    let mut c = DMatrix::zeros(P, m);
                    for (col, &facet_idx) in perm.iter().enumerate() {
                        for d in 0..4 {
                            c[(d, col)] = dual_verts[facet_idx][d];
                        }
                        c[(4, col)] = 1.0;
                    }

                    let mut d = DVector::zeros(P);
                    d[P - 1] = 1.0;

                    rows.push(solve_and_record(
                        "polytope_sigma_node",
                        instance_counter,
                        "poly",
                        &h, &c, &d,
                        Some(poly_idx),
                        Some(perm.to_vec()),
                        Some(f),
                    ));
                    instance_counter += 1;
                });
            }
        }

        if (poly_idx + 1) % 10 == 0 {
            println!(
                "  Processed {} polytopes, {} σ-nodes so far",
                poly_idx + 1, instance_counter
            );
        }
    }

    rows
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut polytopes_path = None;
    let mut max_facets = usize::MAX;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--polytopes" => {
                i += 1;
                polytopes_path = Some(args[i].clone());
            }
            "--max-facets" => {
                i += 1;
                max_facets = args[i].parse().expect("--max-facets must be a number");
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    let polytopes_path = polytopes_path.unwrap_or_else(|| {
        eprintln!("Usage: {} --polytopes <path> [--max-facets N]", args[0]);
        std::process::exit(1);
    });

    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("error-bounds/collected_poly.jsonl");
    let rows = generate_natural(&polytopes_path, max_facets);
    write_jsonl(&rows, output_path.to_str().expect("utf-8 output path"));
    print_summary(&rows, "Natural (polytope)");
}
