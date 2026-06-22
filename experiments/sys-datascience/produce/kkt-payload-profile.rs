//! Profile known-sigma derivative recomputation against full capacity search.
//!
//! This answers whether a computed-polytope cache row that stores the best sigma
//! can cheaply reconstruct ascent derivative inputs without storing full KKT
//! payloads.

use exp_sys_landscape::{
    capacity_auto, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;
use symplectic::derivatives::{
    capacity_derivatives_a_from_kkt_result, systolic_ratio_gradient_a, volume_derivatives_a,
};
use symplectic::kkt::saddle_point_solver::solve_kkt_for_dual_vertices;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const FACETS: &[usize] = &[5, 6, 7, 8, 9, 10, 11, 12];

#[derive(Serialize)]
struct ProfileRow {
    name: String,
    facet_count: usize,
    attempt: u64,
    capacity: f64,
    volume: f64,
    sys: f64,
    full_capacity_ms: f64,
    known_sigma_derivative_ms: f64,
    known_sigma_kkt_ms: f64,
    known_sigma_volume_derivative_ms: f64,
    known_sigma_gradient_ms: f64,
}

struct Args {
    samples_per_f: usize,
    out: Option<PathBuf>,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut samples_per_f = 1usize;
    let mut out = None;

    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .unwrap_or_else(|| panic!("{flag} requires a value"));
        match flag {
            "--samples-per-f" => {
                samples_per_f = value.parse().expect("--samples-per-f must be usize");
                i += 2;
            }
            "--out" => {
                out = Some(PathBuf::from(value));
                i += 2;
            }
            "--help" | "-h" => {
                print_help(
                    argv.first()
                        .map(String::as_str)
                        .unwrap_or("sys-kkt-payload-profile"),
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args { samples_per_f, out }
}

fn print_help(program: &str) {
    println!(
        "\
Profile known-sigma derivative recomputation.

Usage:
  {program} [--samples-per-f <n>] [--out <jsonl>]
"
    );
}

fn write_rows(rows: &[ProfileRow], out: Option<PathBuf>) {
    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).expect("create output parent");
                }
            }
            let mut file = std::fs::File::create(&path)
                .unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
            for row in rows {
                serde_json::to_writer(&mut file, row).expect("serialize profile row");
                use std::io::Write;
                writeln!(file).expect("write newline");
            }
        }
        None => {
            for row in rows {
                println!(
                    "{}",
                    serde_json::to_string(row).expect("serialize profile row")
                );
            }
        }
    }
}

fn profile_polytope(name: String, facet_count: usize, attempt: u64) -> Option<ProfileRow> {
    let polytope =
        SysLandscapePolytopeCache::generate_random(facet_count, H_MIN, H_MAX, SEED, attempt)?;

    let full_capacity_started = Instant::now();
    let capacity_result = capacity_auto(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .ok()?;
    let full_capacity_ms = full_capacity_started.elapsed().as_secs_f64() * 1000.0;

    let capacity = capacity_result.min_action;
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if volume <= 0.0 {
        return None;
    }
    let sys = symplectic::systolic_ratio(capacity, volume);
    let sigma = capacity_result.best_sigma().to_vec();

    let derivative_started = Instant::now();
    let kkt_started = Instant::now();
    let kkt = solve_kkt_for_dual_vertices(&polytope.dual_vertices_f64, &sigma).feasible()?;
    let known_sigma_kkt_ms = kkt_started.elapsed().as_secs_f64() * 1000.0;

    let volume_started = Instant::now();
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .ok()?;
    let known_sigma_volume_derivative_ms = volume_started.elapsed().as_secs_f64() * 1000.0;

    let gradient_started = Instant::now();
    let d_capacity_da =
        capacity_derivatives_a_from_kkt_result(&polytope.dual_vertices_f64, &sigma, &kkt);
    let _d_sys_da = systolic_ratio_gradient_a(capacity, volume, &d_capacity_da, &d_volume_da);
    let known_sigma_gradient_ms = gradient_started.elapsed().as_secs_f64() * 1000.0;
    let known_sigma_derivative_ms = derivative_started.elapsed().as_secs_f64() * 1000.0;

    Some(ProfileRow {
        name,
        facet_count,
        attempt,
        capacity,
        volume,
        sys,
        full_capacity_ms,
        known_sigma_derivative_ms,
        known_sigma_kkt_ms,
        known_sigma_volume_derivative_ms,
        known_sigma_gradient_ms,
    })
}

fn main() {
    let args = parse_args();
    let mut rows = Vec::new();
    for &facet_count in FACETS {
        let mut accepted = 0usize;
        let mut attempt = 0u64;
        while accepted < args.samples_per_f {
            if let Some(row) = profile_polytope(
                format!("profile_F{facet_count}_{accepted}"),
                facet_count,
                attempt,
            ) {
                rows.push(row);
                accepted += 1;
            }
            attempt += 1;
        }
    }
    write_rows(&rows, args.out);
}
