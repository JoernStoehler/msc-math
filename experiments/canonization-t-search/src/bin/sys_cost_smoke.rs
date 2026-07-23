use exp_sys_landscape::{
    compute_capacity_result, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use nalgebra::Vector4;
use std::time::Instant;

fn main() {
    let case_count = std::env::args()
        .nth(1)
        .map(|value| value.parse::<usize>().expect("case count"))
        .unwrap_or(8);
    let facet_count = std::env::args()
        .nth(2)
        .map(|value| value.parse::<usize>().expect("facet count"));
    let cases = accepted_random_cases(case_count, facet_count, 2026062816);
    let mut ok = 0usize;
    let mut reconstruct_s = 0.0;
    let mut volume_s = 0.0;
    let mut capacity_s = 0.0;
    let start = Instant::now();
    for case in cases {
        let reconstruct_start = Instant::now();
        let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(case) else {
            continue;
        };
        reconstruct_s += reconstruct_start.elapsed().as_secs_f64();

        let volume_start = Instant::now();
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        volume_s += volume_start.elapsed().as_secs_f64();
        if volume <= 0.0 {
            continue;
        }

        let capacity_start = Instant::now();
        let capacity = compute_capacity_result(&polytope);
        capacity_s += capacity_start.elapsed().as_secs_f64();
        if capacity.is_some() {
            ok += 1;
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "cases={case_count} ok={ok} elapsed_s={elapsed:.6} per_ok_s={:.6} reconstruct_s={reconstruct_s:.6} volume_s={volume_s:.6} capacity_s={capacity_s:.6}",
        elapsed / ok.max(1) as f64
    );
}

fn accepted_random_cases(
    count: usize,
    facet_count: Option<usize>,
    seed: u64,
) -> Vec<Vec<Vector4<f64>>> {
    let mut cases = Vec::with_capacity(count);
    let mut attempt = 0;
    while cases.len() < count {
        let f = facet_count.unwrap_or(8 + 2 * (cases.len() % 4));
        if let Ok(duals) = symplectic::random::generate_dual_vertices(f, 0.55, 1.85, seed, attempt)
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
