/// Profiling harness for ehz_capacity_pruned.
///
/// Runs on known + random polytopes, checks correctness, reports timing.
/// Use directly for wall-clock timing, or under callgrind for instruction profiling.
///
/// Usage:
///   cargo run --release --example profile_ehz           # wall-clock timing
///   valgrind --tool=callgrind cargo run --release --example profile_ehz -- --callgrind
use geom::known_polytopes;
use geom::test_utils::random_bounded_polytope;
use hk2017::ehz_capacity_pruned;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::Instant;

fn main() {
    let callgrind_mode = std::env::args().any(|a| a == "--callgrind");

    // Known polytopes with expected capacities
    let known = [
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
        ("lag_tri_tri", known_polytopes::lagrangian_triangle_product()),
        ("sym_tri_tri", known_polytopes::symplectic_triangle_product()),
        ("lag_tri_sq", known_polytopes::lagrangian_triangle_square()),
        ("sym_tri_sq", known_polytopes::symplectic_triangle_square()),
    ];

    // In callgrind mode, skip F=10 (too slow under instrumentation)
    let random_configs: &[(usize, usize)] = if callgrind_mode {
        &[(8, 3)]
    } else {
        &[(8, 3), (10, 2)]
    };

    println!("{:<20} {:>6} {:>10} {:>12} {:>12}", "name", "facets", "iters", "time_ms", "capacity");
    println!("{}", "-".repeat(65));

    // Known polytopes: run + correctness check
    for (name, kp) in &known {
        let start = Instant::now();
        let result = ehz_capacity_pruned(&kp.polytope).expect("capacity failed");
        let elapsed = start.elapsed();

        let err = (result.capacity - kp.capacity).abs() / kp.capacity;
        assert!(
            err < 1e-6,
            "CORRECTNESS FAILURE: {name} expected {}, got {}, rel_err={err:.2e}",
            kp.capacity, result.capacity
        );

        println!(
            "{:<20} {:>6} {:>10} {:>12.3} {:>12.6}",
            name,
            kp.polytope.facet_count(),
            result.iterations,
            elapsed.as_secs_f64() * 1000.0,
            result.capacity
        );
    }

    // HK-O pentagon (F=10, takes ~2.5s)
    if !callgrind_mode {
        let kp = known_polytopes::hko_pentagon();
        let start = Instant::now();
        let result = ehz_capacity_pruned(&kp.polytope).expect("capacity failed");
        let elapsed = start.elapsed();

        let err = (result.capacity - kp.capacity).abs() / kp.capacity;
        assert!(
            err < 1e-6,
            "CORRECTNESS FAILURE: hko_pentagon expected {}, got {}, rel_err={err:.2e}",
            kp.capacity, result.capacity
        );

        println!(
            "{:<20} {:>6} {:>10} {:>12.3} {:>12.6}",
            "hko_pentagon", 10, result.iterations,
            elapsed.as_secs_f64() * 1000.0, result.capacity
        );
    }

    // Random polytopes: record baseline capacities, check determinism
    let mut rng = ChaCha8Rng::seed_from_u64(12345);

    // Cache expected capacities from first run (hardcode after first baseline)
    for &(facet_count, n_samples) in random_configs {
        for i in 0..n_samples {
            let p = random_bounded_polytope(facet_count, &mut rng);
            let start = Instant::now();
            let result = ehz_capacity_pruned(&p).expect("capacity failed");
            let elapsed = start.elapsed();

            println!(
                "{:<20} {:>6} {:>10} {:>12.3} {:>12.6}",
                format!("random_{facet_count}_{i}"),
                facet_count,
                result.iterations,
                elapsed.as_secs_f64() * 1000.0,
                result.capacity
            );
        }
    }
}
