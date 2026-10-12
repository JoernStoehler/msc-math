use exp_dev_quadratic_program::{exact_binary64_dual_vertex_arrays, generated_f64_cases};
use nalgebra::Vector4;
use std::hint::black_box;
use std::time::{Duration, Instant};
use symplectic::exact::{solve_orbit_sigma_exact, solve_orbit_sigma_exact_rational};

const SEED: u64 = 99_599_604;
const SIGMA: [usize; 5] = [0, 1, 3, 2, 4];
const ROUNDS: usize = 21;

fn median(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn main() {
    let case = generated_f64_cases(1, SEED)
        .into_iter()
        .find(|case| case.family == "generated_random_f64" && case.dual_vertices.len() == 5)
        .expect("standard generated F5 case");
    let exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices)
        .into_iter()
        .map(|value| {
            Vector4::new(
                value[0].clone(),
                value[1].clone(),
                value[2].clone(),
                value[3].clone(),
            )
        })
        .collect::<Vec<_>>();

    let fast = solve_orbit_sigma_exact_rational(&exact, &SIGMA);
    let generic = solve_orbit_sigma_exact(&exact, &SIGMA);
    assert_eq!(fast, generic);
    assert!(fast.is_some(), "retained F5 word must be exactly feasible");

    black_box(solve_orbit_sigma_exact_rational(&exact, &SIGMA));
    black_box(solve_orbit_sigma_exact(&exact, &SIGMA));
    let fast_samples = (0..ROUNDS)
        .map(|_| {
            let started = Instant::now();
            black_box(solve_orbit_sigma_exact_rational(&exact, &SIGMA));
            started.elapsed()
        })
        .collect();
    let generic_samples = (0..ROUNDS)
        .map(|_| {
            let started = Instant::now();
            black_box(solve_orbit_sigma_exact(&exact, &SIGMA));
            started.elapsed()
        })
        .collect();
    let fast_ms = median(fast_samples).as_secs_f64() * 1e3;
    let generic_ms = median(generic_samples).as_secs_f64() * 1e3;

    println!("comparison.cohort={}", case.source_id);
    println!("comparison.sigma={SIGMA:?}");
    println!("comparison.rounds={ROUNDS}");
    println!("comparison.exact_outputs_equal=true");
    println!("fast.id=exact_sigma_fraction_free_with_generic_fallback");
    println!("fast.production=true");
    println!("fast.median_ms={fast_ms:.6}");
    println!("generic.id=exact_sigma_generic_rank_kernel");
    println!("generic.production=false");
    println!("generic.median_ms={generic_ms:.6}");
    println!("comparison.speedup={:.6}", generic_ms / fast_ms);
}
