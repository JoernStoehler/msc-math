//! Stage 2 filter: minimal smoke test from collected_poly.jsonl.
//!
//! Picks ~10 rows: one feasible per distinct m value, plus a few edge cases.
//! Purpose: verify the pipeline runs end-to-end in seconds.
//!
//! Usage:
//!   cargo run --release --bin filter_poly_smoke
//!
//! Input:  verify-numerics/collected_poly.jsonl
//! Output: verify-numerics/filtered_poly_smoke.jsonl

use serde::Deserialize;
use std::collections::HashSet;
use std::io::{BufRead, Write};

const INPUT_PATH: &str = "verify-numerics/collected_poly.jsonl";
const OUTPUT_PATH: &str = "verify-numerics/filtered_poly_smoke.jsonl";

#[derive(Deserialize)]
struct FilterFields {
    verdict: String,
    q: f64,
    m: usize,
}

fn main() {
    let file = std::fs::File::open(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", INPUT_PATH, e));
    let reader = std::io::BufReader::new(file);

    let mut out = std::fs::File::create(OUTPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", OUTPUT_PATH, e));

    let mut seen_m: HashSet<usize> = HashSet::new();
    let mut n = 0usize;

    for line in reader.lines() {
        let line = line.expect("read error");
        if line.trim().is_empty() { continue; }
        let fields: FilterFields = match serde_json::from_str(&line) {
            Ok(f) => f,
            Err(_) => continue,
        };

        // Take one feasible Q > 0 row per m value.
        if fields.verdict == "feasible" && fields.q > 1e-15 && !seen_m.contains(&fields.m) {
            seen_m.insert(fields.m);
            writeln!(out, "{}", line).expect("write error");
            n += 1;
        }

        if n >= 15 { break; }
    }

    println!("Wrote {} rows to {} (m values: {:?})", n, OUTPUT_PATH, {
        let mut v: Vec<_> = seen_m.into_iter().collect();
        v.sort();
        v
    });
}
