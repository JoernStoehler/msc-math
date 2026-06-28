use exp_dev_canonization_t_search::candidates::volume_one_omega_labeled_symplectic_frame;
use nalgebra::Vector4;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let case_count = std::env::args()
        .nth(1)
        .map(|value| value.parse::<usize>().expect("case count"))
        .unwrap_or(64);
    let facet_count = std::env::args()
        .nth(2)
        .map(|value| value.parse::<usize>().expect("facet count"))
        .unwrap_or(10);
    let repeat_count = std::env::args()
        .nth(3)
        .map(|value| value.parse::<usize>().expect("repeat count"))
        .unwrap_or(16);
    let cases = accepted_random_cases(case_count, facet_count, 2026062817);
    let mut ok = 0usize;
    let mut checksum = 0.0_f64;
    let start = Instant::now();
    for _ in 0..repeat_count {
        for case in &cases {
            let output = volume_one_omega_labeled_symplectic_frame::canonicalize(black_box(case));
            if output.status == "ok" {
                ok += 1;
            }
            checksum += output
                .duals
                .iter()
                .flat_map(|dual| dual.iter())
                .map(|value| value.abs())
                .sum::<f64>();
            black_box(&output.duals);
        }
    }
    black_box(checksum);
    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "cases={case_count} facets={facet_count} repeats={repeat_count} calls={} ok={ok} elapsed_s={elapsed:.6} per_ok_s={:.9} checksum={checksum:.6}",
        case_count * repeat_count,
        elapsed / ok.max(1) as f64
    );
}

fn accepted_random_cases(count: usize, facet_count: usize, seed: u64) -> Vec<Vec<Vector4<f64>>> {
    let mut cases = Vec::with_capacity(count);
    let mut attempt = 0;
    while cases.len() < count {
        if let Ok(duals) =
            symplectic::random::generate_dual_vertices(facet_count, 0.55, 1.85, seed, attempt)
        {
            cases.push(duals);
        }
        attempt += 1;
        assert!(
            attempt < 200_000,
            "failed to generate {count} accepted random cases before attempt limit"
        );
    }
    cases
}
