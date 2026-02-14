/// Lagrangian product sweep: compute sys for families of Lagrangian products.
///
/// Three subcommands:
/// - `pentagon-sweep`: Fine θ sweep for pentagon × R(θ)pentagon
/// - `polygon-grid`:   All regular (n,m) pairs with n+m ≤ max_facets
/// - `random-products`: Random polygon Lagrangian products
///
/// Output: JSONL rows, one per polytope.
use geom::lagrangian_product::lagrangian_product;
use geom::polygon::{polygon_area, random_polygon_2d, regular_polygon_2d, rotate_polygon_2d};
use geom::volume::volume;
use hk2017::ehz_capacity_pruned;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

/// Output row for a Lagrangian product sweep point.
#[derive(Debug, Serialize)]
struct SweepRow {
    family: String,
    /// Number of sides of q-factor polygon
    n1: usize,
    /// Number of sides of p-factor polygon
    n2: usize,
    /// Rotation angle of p-factor (degrees)
    angle_deg: f64,
    facet_count: usize,
    volume: f64,
    capacity: f64,
    sys: f64,
    time_capacity_ms: f64,
    area_q: f64,
    area_p: f64,
}

// ---- Configuration constants ----

/// Angular resolution for pentagon sweep: 360 steps = 0.25° over [0°, 90°].
const PENTAGON_SWEEP_RESOLUTION: usize = 360;

/// Maximum total facet count for polygon grid pairs.
const MAX_FACETS: usize = 12;

/// Number of random Lagrangian products to generate.
const RANDOM_PRODUCT_COUNT: usize = 500;

/// RNG seed for reproducible random product generation.
const RANDOM_SEED: u64 = 42;

/// Minimum support function value for random polygon heights.
const RANDOM_H_MIN: f64 = 0.3;

/// Maximum support function value for random polygon heights.
const RANDOM_H_MAX: f64 = 2.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: lagrangian_sweep <subcommand> <output_path>");
        eprintln!("  subcommands: pentagon-sweep, polygon-grid, random-products, all");
        std::process::exit(1);
    }

    let subcommand = &args[1];
    let output_path = PathBuf::from(&args[2]);

    let file = File::create(&output_path).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    match subcommand.as_str() {
        "pentagon-sweep" => cmd_pentagon_sweep(&mut writer),
        "polygon-grid" => cmd_polygon_grid(&mut writer),
        "random-products" => cmd_random_products(&mut writer),
        "all" => {
            cmd_pentagon_sweep(&mut writer);
            cmd_polygon_grid(&mut writer);
            cmd_random_products(&mut writer);
        }
        other => {
            eprintln!("Unknown subcommand: {other}");
            std::process::exit(1);
        }
    }

    writer.flush().expect("flush output");
    eprintln!("Done. Output: {}", output_path.display());
}

/// Pentagon × R(θ)Pentagon fine sweep.
fn cmd_pentagon_sweep(writer: &mut BufWriter<File>) {
    let resolution = PENTAGON_SWEEP_RESOLUTION;
    eprintln!("Pentagon sweep: {resolution} angles in [0°, 90°]");

    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let area_q = polygon_area(&qn, &qh).unwrap();
    let area_p = polygon_area(&pn_base, &ph_base).unwrap();

    for i in 0..=resolution {
        let angle_deg = 90.0 * (i as f64) / (resolution as f64);
        let theta = angle_deg.to_radians();

        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("pentagon product construction failed");

        let vol = volume(&polytope).expect("volume computation failed");

        let start = Instant::now();
        let cap_result = ehz_capacity_pruned(&polytope).expect("capacity computation failed");
        let cap_time = start.elapsed();

        let cap = cap_result.capacity;
        let sys = cap * cap / (2.0 * vol);

        let row = SweepRow {
            family: "pentagon_sweep".to_string(),
            n1: 5,
            n2: 5,
            angle_deg,
            facet_count: 10,
            volume: vol,
            capacity: cap,
            sys,
            time_capacity_ms: cap_time.as_secs_f64() * 1000.0,
            area_q,
            area_p,
        };
        let line = serde_json::to_string(&row).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        if i % 36 == 0 {
            eprintln!(
                "  {i}/{resolution}: θ={angle_deg:.2}° sys={sys:.6} cap={cap:.6} ({:.0}ms)",
                cap_time.as_secs_f64() * 1000.0
            );
        }
    }
    eprintln!("Pentagon sweep done ({} points)", resolution + 1);
}

/// All regular (n, m) pairs with n + m ≤ max_facets.
fn cmd_polygon_grid(writer: &mut BufWriter<File>) {
    let max_facets = MAX_FACETS;
    eprintln!("Polygon grid: all (n,m) pairs with n+m ≤ {max_facets}");

    for n1 in 3..=6 {
        for n2 in n1..=6 {
            if n1 + n2 > max_facets {
                continue;
            }

            // Adaptive resolution based on facet count (= computational cost)
            let total_facets = n1 + n2;
            let resolution = match total_facets {
                0..=8 => 360,
                9 => 200,
                10 => 200,
                11 => 50,
                _ => 30,
            };

            // Sweep fundamental domain [0, π/lcm(n1,n2)]
            let fund_domain_deg = 180.0 / lcm(n1, n2) as f64;

            eprintln!(
                "  ({n1},{n2}): F={total_facets}, {resolution} angles in [0°, {fund_domain_deg:.1}°]"
            );

            let (qn, qh) = regular_polygon_2d(n1, 1.0);
            let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
            let area_q = polygon_area(&qn, &qh).unwrap();
            let area_p = polygon_area(&pn_base, &ph_base).unwrap();

            for i in 0..=resolution {
                let angle_deg = fund_domain_deg * (i as f64) / (resolution as f64);
                let theta = angle_deg.to_radians();

                let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
                let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
                    .expect("product construction failed");

                let vol = volume(&polytope).expect("volume computation failed");

                let start = Instant::now();
                let cap_result =
                    ehz_capacity_pruned(&polytope).expect("capacity computation failed");
                let cap_time = start.elapsed();

                let cap = cap_result.capacity;
                let sys = cap * cap / (2.0 * vol);

                let row = SweepRow {
                    family: "polygon_grid".to_string(),
                    n1,
                    n2,
                    angle_deg,
                    facet_count: total_facets,
                    volume: vol,
                    capacity: cap,
                    sys,
                    time_capacity_ms: cap_time.as_secs_f64() * 1000.0,
                    area_q,
                    area_p,
                };
                let line = serde_json::to_string(&row).expect("serialize");
                writeln!(writer, "{line}").expect("write");
            }
        }
    }
    eprintln!("Polygon grid done");
}

/// Random polygon Lagrangian products.
fn cmd_random_products(writer: &mut BufWriter<File>) {
    let count = RANDOM_PRODUCT_COUNT;
    let seed = RANDOM_SEED;
    let h_min = RANDOM_H_MIN;
    let h_max = RANDOM_H_MAX;

    eprintln!("Random products: {count} samples, seed={seed}");

    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let mut generated = 0;
    let mut attempts = 0;
    while generated < count {
        attempts += 1;
        if attempts > count * 10 {
            eprintln!("  WARNING: too many failed attempts ({attempts}), stopping at {generated}");
            break;
        }

        // Random polygon sizes: 3-6 sides each, total ≤ 12 facets
        let n1 = 3 + (rng.gen::<u32>() % 4) as usize; // 3..=6
        let n2 = 3 + (rng.gen::<u32>() % 4) as usize;
        if n1 + n2 > 12 {
            continue;
        }

        let (qn, qh) = random_polygon_2d(n1, h_min, h_max, &mut rng);
        let (pn, ph) = random_polygon_2d(n2, h_min, h_max, &mut rng);

        // Try to build the Lagrangian product — may fail if polygon is degenerate
        let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let vol = match volume(&polytope) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let start = Instant::now();
        let cap_result = match ehz_capacity_pruned(&polytope) {
            Some(r) => r,
            None => continue,
        };
        let cap_time = start.elapsed();

        let cap = cap_result.capacity;
        let sys = cap * cap / (2.0 * vol);

        let area_q = polygon_area(&qn, &qh)
            .expect("area computation must succeed for valid Lagrangian product factors");
        let area_p = polygon_area(&pn, &ph)
            .expect("area computation must succeed for valid Lagrangian product factors");

        let row = SweepRow {
            family: "random_product".to_string(),
            n1,
            n2,
            angle_deg: f64::NAN, // not applicable for random polygons
            facet_count: n1 + n2,
            volume: vol,
            capacity: cap,
            sys,
            time_capacity_ms: cap_time.as_secs_f64() * 1000.0,
            area_q,
            area_p,
        };
        let line = serde_json::to_string(&row).expect("serialize");
        writeln!(writer, "{line}").expect("write");

        generated += 1;
        if generated % 50 == 0 {
            eprintln!("  {generated}/{count} (attempts: {attempts})");
        }
    }
    eprintln!("Random products done: {generated} (from {attempts} attempts)");
}

/// Least common multiple.
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Greatest common divisor (Euclidean algorithm).
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
