use euclidean_polytopes::volume_from_incidence_f64;
use exp_sys_landscape::{exact_binary64_geometry_from_cache, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use std::time::Instant;
use symplectic::capacity_4d::{
    capacity, capacity_value, check_dual_vertex_norm_bounds, check_facet_count,
    check_primal_vertex_norm_bounds,
};

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
        let volume =
            volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                .expect("validated cache has positive f64 incidence volume");
        volume_s += volume_start.elapsed().as_secs_f64();
        if volume <= 0.0 {
            continue;
        }

        let capacity_start = Instant::now();
        let capacity = (|| {
            check_facet_count(polytope.facet_count()).ok()?;
            check_dual_vertex_norm_bounds(&polytope.dual_vertices_f64).ok()?;
            let geometry = exact_binary64_geometry_from_cache(&polytope).ok()?;
            check_primal_vertex_norm_bounds(&geometry).ok()?;
            let result = capacity(&geometry).ok()?;
            capacity_value(&result, 1e-10).ok()
        })();
        capacity_s += capacity_start.elapsed().as_secs_f64();
        if capacity.is_some() {
            ok += 1;
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "cases={case_count} ok={ok} elapsed_s={elapsed:.6} per_ok_s={:.6} reconstruct_s={reconstruct_s:.6} volume_s={volume_s:.6} capacity_s={capacity_s:.6} volume_method=f64-from-exact-derived-incidence-v1 capacity_method=certified-production-capacity-v1",
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
