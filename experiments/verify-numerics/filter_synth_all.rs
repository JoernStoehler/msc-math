//! Stage 2 filter: pass all synthetic data through.
//!
//! The synthetic dataset is small (~4300 rows) and designed to cover
//! specific edge cases. No filtering needed — just copy.
//!
//! Usage:
//!   cargo run --release --bin filter_synth_all
//!
//! Input:  verify-numerics/collected_synth.jsonl
//! Output: verify-numerics/filtered_synth_all.jsonl

use std::io::{BufRead, Write};

const INPUT_PATH: &str = "verify-numerics/collected_synth.jsonl";
const OUTPUT_PATH: &str = "verify-numerics/filtered_synth_all.jsonl";

fn main() {
    let file = std::fs::File::open(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", INPUT_PATH, e));
    let reader = std::io::BufReader::new(file);

    let mut out = std::fs::File::create(OUTPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", OUTPUT_PATH, e));

    let mut n = 0usize;
    for line in reader.lines() {
        let line = line.expect("read error");
        if line.trim().is_empty() { continue; }
        writeln!(out, "{}", line).expect("write error");
        n += 1;
    }

    println!("Copied {} rows from {} to {}", n, INPUT_PATH, OUTPUT_PATH);
}
