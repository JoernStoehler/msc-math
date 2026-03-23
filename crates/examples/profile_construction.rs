//! TEMPORARY profiling harness for Polytope4D::new() construction phases.
//!
//! Measures wall-clock time in each phase of the construction pipeline and
//! counts combinatorial statistics (subsets tested, prefilter rejections, etc.).
//!
//! Usage: cd crates && cargo run --example profile_construction --release
//!
//! Remove this file and the associated profiling code in vertex_enumeration.rs
//! and polytope.rs when profiling is complete.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};

use symplectic::geom::{profile_construction_phases, ConstructionProfile};

/// Same seed and height range as benchmarks (crates/benches/profiling.rs).
const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

/// Facet counts to profile. Includes F=12 which is too slow for criterion
/// but fine for a single-shot measurement.
const FACET_COUNTS: &[usize] = &[5, 7, 8, 9, 10, 11, 12];

/// Maximum attempts to find a valid polytope at each facet count.
const MAX_ATTEMPTS: usize = 500;

fn main() {
    eprintln!("Polytope4D::new() construction phase profiler");
    eprintln!("seed={SEED}, h_min={H_MIN}, h_max={H_MAX}");
    eprintln!();

    let mut profiles: Vec<ConstructionProfile> = Vec::new();

    for &f in FACET_COUNTS {
        eprint!("F={f:>2}: sampling... ");
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);

        // Sample random halfspaces until we find one that constructs successfully.
        // (Same rejection-sampling approach as random.rs.)
        let mut found = false;
        for attempt in 0..MAX_ATTEMPTS {
            let halfspaces = sample_halfspaces(f, &mut rng);
            match profile_construction_phases(halfspaces) {
                Ok(profile) => {
                    eprintln!("OK (attempt {})", attempt + 1);
                    profiles.push(profile);
                    found = true;
                    break;
                }
                Err(_) => continue,
            }
        }
        if !found {
            eprintln!("FAILED after {MAX_ATTEMPTS} attempts, skipping");
        }
    }

    eprintln!();
    print_timing_table(&profiles);
    eprintln!();
    print_combinatorial_table(&profiles);
}

/// Sample random halfspaces a_i = n_i / h_i (same method as random.rs).
fn sample_halfspaces(facet_count: usize, rng: &mut ChaCha8Rng) -> Vec<Vector4<f64>> {
    let h_dist = Uniform::new(H_MIN, H_MAX);

    let normals: Vec<Vector4<f64>> = (0..facet_count)
        .map(|_| {
            loop {
                let x: f64 = StandardNormal.sample(rng);
                let y: f64 = StandardNormal.sample(rng);
                let z: f64 = StandardNormal.sample(rng);
                let w: f64 = StandardNormal.sample(rng);
                let v = Vector4::new(x, y, z, w);
                let norm = v.norm();
                if norm > 1e-10 {
                    break v / norm;
                }
            }
        })
        .collect();

    let heights: Vec<f64> = (0..facet_count).map(|_| h_dist.sample(rng)).collect();

    normals
        .iter()
        .zip(heights.iter())
        .map(|(n, &h)| n / h)
        .collect()
}

/// Format nanoseconds as human-readable duration.
fn fmt_ns(ns: u128) -> String {
    if ns < 1_000 {
        format!("{ns}ns")
    } else if ns < 1_000_000 {
        format!("{:.1}us", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:.2}ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:.3}s", ns as f64 / 1_000_000_000.0)
    }
}

/// Print the timing breakdown table.
fn print_timing_table(profiles: &[ConstructionProfile]) {
    println!("=== TIMING BREAKDOWN (wall-clock) ===");
    println!();
    println!(
        "{:>4} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10}",
        "F", "f64_valid", "f64→rat", "bounded_Q", "enum_vert", "irredund", "assemble", "TOTAL"
    );
    println!("{}", "-".repeat(95));

    for p in profiles {
        let total = p.f64_validation_ns
            + p.f64_to_rational_ns
            + p.check_bounded_ns
            + p.enumerate_vertices_ns
            + p.irredundancy_ns
            + p.assemble_ns;

        println!(
            "{:>4} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10}",
            p.facet_count,
            fmt_ns(p.f64_validation_ns),
            fmt_ns(p.f64_to_rational_ns),
            fmt_ns(p.check_bounded_ns),
            fmt_ns(p.enumerate_vertices_ns),
            fmt_ns(p.irredundancy_ns),
            fmt_ns(p.assemble_ns),
            fmt_ns(total),
        );
    }

    // Also print percentage breakdown
    println!();
    println!("=== PERCENTAGE BREAKDOWN ===");
    println!();
    println!(
        "{:>4} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9}",
        "F", "f64_valid", "f64→rat", "bounded_Q", "enum_vert", "irredund", "assemble"
    );
    println!("{}", "-".repeat(75));

    for p in profiles {
        let total = (p.f64_validation_ns
            + p.f64_to_rational_ns
            + p.check_bounded_ns
            + p.enumerate_vertices_ns
            + p.irredundancy_ns
            + p.assemble_ns) as f64;

        if total == 0.0 {
            continue;
        }

        println!(
            "{:>4} | {:>8.1}% | {:>8.1}% | {:>8.1}% | {:>8.1}% | {:>8.1}% | {:>8.1}%",
            p.facet_count,
            p.f64_validation_ns as f64 / total * 100.0,
            p.f64_to_rational_ns as f64 / total * 100.0,
            p.check_bounded_ns as f64 / total * 100.0,
            p.enumerate_vertices_ns as f64 / total * 100.0,
            p.irredundancy_ns as f64 / total * 100.0,
            p.assemble_ns as f64 / total * 100.0,
        );
    }
}

/// Print the combinatorial statistics table.
fn print_combinatorial_table(profiles: &[ConstructionProfile]) {
    println!("=== COMBINATORIAL STATISTICS ===");
    println!();
    println!(
        "{:>4} | {:>8} | {:>10} | {:>8} | {:>10} | {:>8} | {:>10}",
        "F", "C(F,4)", "prefilter", "det=0", "outside_K", "vertices", "prefilter%"
    );
    println!("{}", "-".repeat(75));

    for p in profiles {
        // total = prefilter_rejected + det_zero + (rational path results)
        // rational path: subsets_yielding_vertex + outside_K_or_dup
        let rational_subsets = p.total_subsets - p.prefilter_rejected;
        // of those: det_zero + (yielding_vertex + outside) = rational_subsets
        let outside_or_dup = rational_subsets - p.det_zero - p.subsets_yielding_vertex;

        let prefilter_pct = if p.total_subsets > 0 {
            p.prefilter_rejected as f64 / p.total_subsets as f64 * 100.0
        } else {
            0.0
        };

        println!(
            "{:>4} | {:>8} | {:>10} | {:>8} | {:>10} | {:>8} | {:>9.1}%",
            p.facet_count,
            p.total_subsets,
            p.prefilter_rejected,
            p.det_zero,
            outside_or_dup,
            p.total_vertices,
            prefilter_pct,
        );
    }
}
