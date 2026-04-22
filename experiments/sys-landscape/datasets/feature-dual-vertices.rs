//! Compute a datascience-facing dual-vertex feature table keyed by `poly_id`.
//!
//! Goal: expose cached exact dual vertices as floating-point feature columns for
//! downstream analysis, while keeping exact coordinates in the raw cache layer.
//! Input Artifacts:
//!   - experiments/sys-landscape/datasets/normalized/ under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::features::{
    deserialize_vec4_rational, parse_standard_feature_args, read_jsonl, write_jsonl,
};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PolytopeInputRow {
    poly_id: String,
    #[serde(deserialize_with = "deserialize_vec4_rational")]
    dual_vertices_rational: Vec<[BigRational; 4]>,
    facet_count: usize,
}

#[derive(Debug, Serialize)]
struct DualVerticesFeatureRow {
    poly_id: String,
    facet_count: usize,
    dual_vertex_count: usize,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_flat_f64: Vec<f64>,
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
}

fn enrich_row(row: PolytopeInputRow) -> DualVerticesFeatureRow {
    let dual_vertices_f64 = row
        .dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| rational_to_f64(&vertex[i])))
        .collect::<Vec<_>>();
    let dual_vertices_flat_f64 = dual_vertices_f64
        .iter()
        .flat_map(|vertex| vertex.iter().copied())
        .collect::<Vec<_>>();
    DualVerticesFeatureRow {
        poly_id: row.poly_id,
        facet_count: row.facet_count,
        dual_vertex_count: dual_vertices_f64.len(),
        dual_vertices_f64,
        dual_vertices_flat_f64,
    }
}

fn main() {
    let args = parse_standard_feature_args("dual-vertices");
    let mut rows = read_jsonl::<PolytopeInputRow>(&args.normalized_dir.join("polytopes.jsonl"))
        .into_iter()
        .map(enrich_row)
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} dual-vertex rows to {}", rows.len(), args.out.display());
}
