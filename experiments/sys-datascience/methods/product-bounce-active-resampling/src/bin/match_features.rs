//! Target-free volume-free ridge summaries for retained 5x5 base matching.

#[allow(dead_code)]
#[path = "../../../../prepare/features_face_symplectic.rs"]
mod features_face_symplectic;
#[allow(dead_code)]
#[path = "../../../../prepare/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::two_faces_from_vertex_facet_incidence;
use exp_sys_landscape::SysLandscapePolytopeCache;
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

const RAW_SHA256: &str = "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736";

#[derive(Deserialize)]
struct RawRow {
    name: String,
    k: usize,
    m: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
}

#[derive(Serialize)]
struct FeatureRow {
    schema: &'static str,
    name: String,
    ridge_symp_area_normalized_entropy: f64,
    ridge_symp_area_max_share: f64,
    ordered_two_face_count: usize,
    ordering_failure_count: usize,
}

fn sha256(path: &Path) -> String {
    let mut hasher = Sha256::new();
    let mut reader = BufReader::new(File::open(path).expect("open hash input"));
    std::io::copy(&mut reader, &mut hasher).expect("hash input");
    format!("{:x}", hasher.finalize())
}

fn rational(value: &str) -> BigRational {
    let (numerator, denominator) = value.split_once('/').expect("rational contains /");
    BigRational::new(
        numerator.parse().expect("rational numerator"),
        denominator.parse().expect("rational denominator"),
    )
}

fn rational_rows(rows: Vec<[String; 4]>) -> Vec<[BigRational; 4]> {
    rows.into_iter()
        .map(|row| row.map(|x| rational(&x)))
        .collect()
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let mut input = None;
    let mut output = None;
    let mut i = 1;
    while i < argv.len() {
        let value = || {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{} requires a value", argv[i]))
        };
        match argv[i].as_str() {
            "--input" => input = Some(PathBuf::from(value())),
            "--output" => output = Some(PathBuf::from(value())),
            other => panic!("unknown argument: {other}"),
        }
        i += 2;
    }
    let input = input.expect("--input is required");
    let output = output.expect("--output is required");
    assert_eq!(sha256(&input), RAW_SHA256, "raw identity mismatch");
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).expect("create output parent");
    }
    let mut writer = BufWriter::new(File::create(&output).expect("create output"));
    let mut count = 0usize;
    for line in BufReader::new(File::open(&input).expect("open input")).lines() {
        let raw: RawRow = serde_json::from_str(&line.expect("read input")).expect("parse input");
        if (raw.k, raw.m) != (5, 5) {
            continue;
        }
        let poly = SysLandscapePolytopeCache::from_rational_parts(
            rational_rows(raw.dual_vertices_rational),
            rational_rows(raw.vertices_rational),
        )
        .expect("retained rational geometry reconstructs");
        let two_faces = two_faces_from_vertex_facet_incidence(&poly.vertex_facet_incidence);
        let fields = features_face_symplectic::compute_face_symplectic_fields(
            &two_faces,
            &poly.vertices_f64,
            &poly.vertex_facet_incidence,
            1.0,
        );
        let row = FeatureRow {
            schema: "product-bounce-active-resampling/match-features/v1",
            name: raw.name,
            ridge_symp_area_normalized_entropy: fields.ridge_symp_area_normalized_entropy,
            ridge_symp_area_max_share: fields.ridge_symp_area_max_share,
            ordered_two_face_count: fields.ridge_symp_area_ordered_face_count,
            ordering_failure_count: fields.ridge_symp_area_ordering_failure_count,
        };
        writeln!(writer, "{}", serde_json::to_string(&row).unwrap()).unwrap();
        count += 1;
    }
    writer.flush().expect("flush output");
    assert_eq!(count, 1_024, "retained 5x5 row count changed");
    eprintln!(
        "wrote {count} target-free feature rows to {}",
        output.display()
    );
}
