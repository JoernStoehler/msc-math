//! Lagrangian products of rotated regular polygon pairs.
//!
//! Goal: Sweep rotated regular polygon pairs and record systolic-ratio curves
//! for fine and coarse angle grids.
//! Input Artifacts: None (constructs all polygon pairs internally).
//! Output Artifacts: experiments/sys-landscape/rotated-regular-products/lagrangian-products-5x5.jsonl
//!         experiments/sys-landscape/rotated-regular-products/lagrangian-products-7x7.jsonl
//!         experiments/sys-landscape/rotated-regular-products/lagrangian-products-{3x3,3x4,3x5,3x6,4x4,4x5,4x6,5x5,5x6,6x6}-6deg.jsonl
//!
//! Architecture:
//! 1. `cargo run -p exp-sys-landscape --release --bin sys-rotated-regular-products`
//!    generates fine sweeps (pentagon 5x5, heptagon 7x7) and coarse polygon pair sweeps.
//! 2. Writes to rotated-regular-products/lagrangian-products-5x5.jsonl,
//!    rotated-regular-products/lagrangian-products-7x7.jsonl, and
//!    rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl
//!
//! Capacity algorithm: explicit billiard. These outputs intentionally keep the
//! specialized algorithm because the JSONL rows report billiard-native
//! `iterations` and `bounces`, which the root `symplectic::ehz_capacity`
//! wrapper does not expose.
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma;
use symplectic::ehz_capacity_billiard;
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
use symplectic::geom::volume::volume;

const PENTAGON_START_DEG: f64 = 0.0;
const PENTAGON_END_DEG: f64 = 36.0;
const PENTAGON_STEP_DEG: f64 = 1.0;

// 6° steps give at least 6 sample points even on the smallest fundamental domain
// (lcm=6 pairs like (3,6) have domain [0°,30°], yielding 6 angles).
const PAIR_STEP_DEG: f64 = 6.0;

const PAIRS: &[(usize, usize)] = &[
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (4, 4),
    (4, 5),
    (4, 6),
    (5, 5),
    (5, 6),
    (6, 6),
];

#[derive(Debug, Serialize)]
struct SweepRow {
    family: String,
    n1: usize,
    n2: usize,
    angle_deg: f64,
    facet_count: usize,
    volume: f64,
    capacity: f64,
    sys: f64,
    time_capacity_ms: f64,
    area_q: f64,
    area_p: f64,
    // These fields justify the explicit billiard call sites in this file.
    iterations: u64,
    bounces: usize,
}

fn main() {
    generate_pentagon_5x5();
    generate_heptagon_7x7();
    generate_polygon_pairs();
}

fn experiment_output_path(file_name: &str) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rotated-regular-products")
        .join(file_name)
}

/// Heptagon x heptagon fine sweep over [0, 180/7] degrees.
/// Fundamental domain from [lem:rotation-fundamental-domain].
fn generate_heptagon_7x7() {
    let n = 7;
    let end_deg = 180.0 / n as f64;
    // 26 steps over 25.71° ≈ 0.99° per step, comparable to the 5x5 sweep (1°/step).
    let num_steps: usize = 26;
    let step_deg = end_deg / num_steps as f64;

    let output_path = experiment_output_path("lagrangian-products-7x7.jsonl");
    let file = File::create(&output_path).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    eprintln!(
        "Heptagon 7x7 sweep: {} angles in [0.0°, {end_deg:.2}°], step={step_deg:.3}°",
        num_steps + 1
    );

    let (qn, qh) = regular_polygon_2d(n, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(n, 1.0);
    let area_q = polygon_area(&qn, &qh).expect("area_q");
    let area_p = polygon_area(&pn_base, &ph_base).expect("area_p");

    for i in 0..=num_steps {
        let angle_deg = step_deg * (i as f64);
        let theta = angle_deg.to_radians();

        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope =
            lagrangian_product(&qn, &qh, &pn, &ph).expect("heptagon product construction failed");

        let vol = volume(&polytope);

        let start = Instant::now();
        let result =
            ehz_capacity_billiard(&polytope).expect("billiard should accept Lagrangian product");
        let time_ms = start.elapsed().as_secs_f64() * 1000.0;

        let cap = result.capacity();
        let sys = cap * cap / (2.0 * vol); // [def:systolic-ratio]: sys = c_EHZ^2 / (2 vol)
        let Some(bounces) = bounce_count_from_sigma(&polytope, result.best_sigma())
            .expect("billiard classification failed")
        else {
            continue;
        };

        let row = SweepRow {
            family: "heptagon_7x7_sweep".to_string(),
            n1: n,
            n2: n,
            angle_deg,
            facet_count: 2 * n,
            volume: vol,
            capacity: cap,
            sys,
            time_capacity_ms: time_ms,
            area_q,
            area_p,
            iterations: result.iterations,
            bounces,
        };
        let line = serde_json::to_string(&row).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        if i % 5 == 0 {
            eprintln!(
                "  {i}/{num_steps}: theta={angle_deg:.2} deg sys={sys:.6} cap={cap:.6} ({time_ms:.0}ms)"
            );
        }
    }

    writer.flush().expect("flush output");
    eprintln!("Done. Output: {}", output_path.display());
}

/// Pentagon x pentagon fine sweep over [0, 36] degrees.
/// Fundamental domain from [lem:rotation-fundamental-domain]: 180°/lcm(5,5) = 36°.
fn generate_pentagon_5x5() {
    let output_path = experiment_output_path("lagrangian-products-5x5.jsonl");
    let file = File::create(&output_path).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    let steps = ((PENTAGON_END_DEG - PENTAGON_START_DEG) / PENTAGON_STEP_DEG).round() as usize;
    eprintln!(
        "Pentagon 5x5 sweep: {} angles in [{PENTAGON_START_DEG:.1}°, {PENTAGON_END_DEG:.1}°]",
        steps + 1
    );

    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let area_q = polygon_area(&qn, &qh).expect("area_q");
    let area_p = polygon_area(&pn_base, &ph_base).expect("area_p");

    for i in 0..=steps {
        let angle_deg = PENTAGON_START_DEG + PENTAGON_STEP_DEG * (i as f64);
        let theta = angle_deg.to_radians();

        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope =
            lagrangian_product(&qn, &qh, &pn, &ph).expect("pentagon product construction failed");

        let vol = volume(&polytope);

        let start = Instant::now();
        let result =
            ehz_capacity_billiard(&polytope).expect("billiard should accept Lagrangian product");
        let time_ms = start.elapsed().as_secs_f64() * 1000.0;

        let cap = result.capacity();
        let sys = cap * cap / (2.0 * vol); // [def:systolic-ratio]: sys = c_EHZ^2 / (2 vol)
        let Some(bounces) = bounce_count_from_sigma(&polytope, result.best_sigma())
            .expect("billiard classification failed")
        else {
            continue;
        };

        let row = SweepRow {
            family: "pentagon_5x5_sweep".to_string(),
            n1: 5,
            n2: 5,
            angle_deg,
            facet_count: 10,
            volume: vol,
            capacity: cap,
            sys,
            time_capacity_ms: time_ms,
            area_q,
            area_p,
            iterations: result.iterations,
            bounces,
        };
        let line = serde_json::to_string(&row).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        if i % 6 == 0 {
            eprintln!(
                "  {i}/{steps}: theta={angle_deg:.1} deg sys={sys:.6} cap={cap:.6} ({time_ms:.0}ms)"
            );
        }
    }

    writer.flush().expect("flush output");
    eprintln!("Done. Output: {}", output_path.display());
}

fn generate_polygon_pairs() {
    for &(n1, n2) in PAIRS {
        let output_path =
            experiment_output_path(&format!("lagrangian-products-{}x{}-6deg.jsonl", n1, n2));
        let file = File::create(&output_path).expect("cannot create output file");
        let mut writer = BufWriter::new(file);

        let end_deg = 180.0 / lcm(n1, n2) as f64;
        let angles = sweep_angles(0.0, end_deg, PAIR_STEP_DEG);

        eprintln!(
            "Polygon pair ({n1},{n2}): {} angles in [0.0°, {end_deg:.1}°]",
            angles.len()
        );

        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
        let area_q = polygon_area(&qn, &qh).expect("area_q");
        let area_p = polygon_area(&pn_base, &ph_base).expect("area_p");

        for (i, angle_deg) in angles.iter().enumerate() {
            let theta = angle_deg.to_radians();
            let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                .expect("polygon product construction failed");

            let vol = volume(&polytope);

            let start = Instant::now();
            let result = ehz_capacity_billiard(&polytope)
                .expect("billiard should accept Lagrangian product");
            let time_ms = start.elapsed().as_secs_f64() * 1000.0;

            let cap = result.capacity();
            let sys = cap * cap / (2.0 * vol); // [def:systolic-ratio]: sys = c_EHZ^2 / (2 vol)
            let Some(bounces) = bounce_count_from_sigma(&polytope, result.best_sigma())
                .expect("billiard classification failed")
            else {
                continue;
            };

            let row = SweepRow {
                family: "polygon_pair".to_string(),
                n1,
                n2,
                angle_deg: *angle_deg,
                facet_count: n1 + n2,
                volume: vol,
                capacity: cap,
                sys,
                time_capacity_ms: time_ms,
                area_q,
                area_p,
                iterations: result.iterations,
                bounces,
            };
            let line = serde_json::to_string(&row).expect("serialize");
            writeln!(writer, "{line}").expect("write");

            if i % 5 == 0 {
                eprintln!(
                    "  {i}/{}: theta={angle_deg:.1} deg sys={sys:.6} cap={cap:.6} ({time_ms:.0}ms)",
                    angles.len() - 1
                );
            }
        }

        writer.flush().expect("flush output");
        eprintln!("Done. Output: {}", output_path.display());
    }
}

fn sweep_angles(start_deg: f64, end_deg: f64, step_deg: f64) -> Vec<f64> {
    // 1e-9 is a floating-point snap tolerance: angles within this tolerance
    // of a grid point or of end_deg are considered exact. This is much smaller
    // than any step_deg used in practice (minimum 1°), so false snaps are impossible.
    const SNAP_TOL: f64 = 1e-9;
    let mut angles = Vec::new();
    let mut angle = start_deg;
    while angle <= end_deg + SNAP_TOL {
        angles.push(angle);
        angle += step_deg;
    }
    if (angles.last().unwrap_or(&start_deg) - end_deg).abs() > SNAP_TOL {
        angles.push(end_deg);
    }
    angles
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/*
Legacy experiment (commented out for now):
- Pentagon sweep over [0, 90] at 0.25-degree resolution
- Polygon grid across (n, m) pairs with adaptive resolution
- Random Lagrangian products
- Optional HK2017 cross-validation

See git history for the full multi-family sweep implementation.
*/
