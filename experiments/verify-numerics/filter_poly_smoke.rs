//! Stage 2 filter: diverse smoke-test sample from collected_poly.jsonl.
//!
//! Selects ~1500 rows covering different polytopes, m values, margin ranges.
//! Strategy: per polytope, take the max-Q σ-node, the min-margin feasible
//! σ-node, and a random sample of others.
//!
//! Usage:
//!   cargo run --release --bin filter_poly_smoke
//!
//! Input:  verify-numerics/collected_poly.jsonl
//! Output: verify-numerics/filtered_poly_smoke.jsonl

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, Write};

const INPUT_PATH: &str = "verify-numerics/collected_poly.jsonl";
const OUTPUT_PATH: &str = "verify-numerics/filtered_poly_smoke.jsonl";

/// Max rows per polytope (the interesting ones + random sample).
/// 307 polytopes × 5 = ~1500 rows → ~2.5 min exact solve.
const MAX_PER_POLYTOPE: usize = 5;

/// Row from collected_poly.jsonl — only the fields needed for filtering.
/// Extra fields are preserved via raw JSON pass-through.
#[derive(Deserialize)]
struct FilterFields {
    verdict: String,
    q: f64,
    margin: f64,
    sigma_min_c: f64,
    polytope_id: Option<usize>,
    m: usize,
}

/// Track per-polytope stats for selection.
struct PolytopeInfo {
    /// All row indices for this polytope.
    row_indices: Vec<usize>,
    /// Index of max-Q feasible row (if any).
    max_q_idx: Option<usize>,
    max_q: f64,
    /// Index of min-margin feasible row (if any).
    min_margin_idx: Option<usize>,
    min_margin: f64,
}

fn main() {
    // Pass 1: read all rows, build per-polytope index.
    let file = std::fs::File::open(INPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot open {}: {}", INPUT_PATH, e));
    let reader = std::io::BufReader::new(file);

    let mut raw_lines: Vec<String> = Vec::new();
    let mut polytope_map: HashMap<usize, PolytopeInfo> = HashMap::new();
    let mut no_polytope_indices: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("read error");
        if line.trim().is_empty() { continue; }
        let idx = raw_lines.len();

        let fields: FilterFields = match serde_json::from_str(&line) {
            Ok(f) => f,
            Err(_) => { raw_lines.push(line); continue; }
        };

        let poly_id = fields.polytope_id.unwrap_or(usize::MAX);
        if poly_id == usize::MAX {
            no_polytope_indices.push(idx);
        } else {
            let info = polytope_map.entry(poly_id).or_insert_with(|| PolytopeInfo {
                row_indices: Vec::new(),
                max_q_idx: None,
                max_q: f64::NEG_INFINITY,
                min_margin_idx: None,
                min_margin: f64::INFINITY,
            });
            info.row_indices.push(idx);

            if fields.verdict == "feasible" && fields.q > info.max_q {
                info.max_q = fields.q;
                info.max_q_idx = Some(idx);
            }
            if fields.verdict == "feasible" && fields.margin.is_finite() && fields.margin < info.min_margin {
                info.min_margin = fields.margin;
                info.min_margin_idx = Some(idx);
            }
        }

        raw_lines.push(line);
    }

    println!("Read {} rows from {}", raw_lines.len(), INPUT_PATH);
    println!("  {} polytopes, {} rows without polytope_id",
        polytope_map.len(), no_polytope_indices.len());

    // Pass 2: select rows.
    let mut selected: Vec<usize> = Vec::new();

    for (_poly_id, info) in &polytope_map {
        let mut picked: Vec<usize> = Vec::new();

        // Always include max-Q and min-margin rows.
        if let Some(idx) = info.max_q_idx {
            picked.push(idx);
        }
        if let Some(idx) = info.min_margin_idx {
            if !picked.contains(&idx) {
                picked.push(idx);
            }
        }

        // Fill up to MAX_PER_POLYTOPE with uniform step sampling.
        let remaining = MAX_PER_POLYTOPE.saturating_sub(picked.len());
        if remaining > 0 && info.row_indices.len() > picked.len() {
            let candidates: Vec<usize> = info.row_indices.iter()
                .copied()
                .filter(|idx| !picked.contains(idx))
                .collect();
            let step = candidates.len().max(1) / remaining.min(candidates.len()).max(1);
            for (i, &idx) in candidates.iter().enumerate() {
                if i % step.max(1) == 0 && picked.len() < MAX_PER_POLYTOPE {
                    picked.push(idx);
                }
            }
        }

        selected.extend(picked);
    }

    selected.sort();
    selected.dedup();

    println!("Selected {} rows", selected.len());

    // Write output.
    let mut out = std::fs::File::create(OUTPUT_PATH)
        .unwrap_or_else(|e| panic!("Cannot create {}: {}", OUTPUT_PATH, e));

    for &idx in &selected {
        writeln!(out, "{}", raw_lines[idx]).expect("write error");
    }

    println!("Wrote {} rows to {}", selected.len(), OUTPUT_PATH);
}
