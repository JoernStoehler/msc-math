//! Shared helpers for gradient validation experiments.
//!
//! Module architecture:
//! - shared random-direction sampling
//! - shared polytope analysis for the first-order gradient harness
//! - shared perturbed-evaluation and JSONL row writing
//! - small smoke-run helpers used by the numerics binaries
//!
//! Instrument development: validates that analytical gradients (library derivatives.rs)
//! match finite-difference approximations across polytope classes and edge cases.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{
    capacity_derivatives_a_from_kkt_result, directional_derivative_a, volume_derivatives_a,
};
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for_dual_vertices, KktResult, EPS_Q_POSITIVE,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, BilliardError, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSearchResult,
};

mod flat_polytope;

use flat_polytope::GradientPolytopeCache;

/// Shared strict beta-threshold for certified-orbit enumeration in the gradient package.
///
/// This matches `kkt::EPS_MARGIN_TRUE`.
pub const EPS_BETA_CERTIFIED: f64 = 1e-9;

/// Perturbation sizes for the first-order prediction test.
pub const T_VALUES: &[f64] = &[
    1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7,
];

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

pub fn capacity_pruned_hk2017(
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            facet_intersection_is_nonempty,
            omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

pub fn capacity_billiard(
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, BilliardError> {
    let classification = classify_facets_from_dual_vertices(dual_vertices_f64)?;
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            facet_intersection_is_nonempty,
            omega_signs,
        );
    let (orbits, iterations) = solve_billiard_candidates(
        dual_vertices_f64,
        &classification.q_indices,
        &classification.p_indices,
        facet_intersection_is_nonempty,
        &transition_is_allowed,
    )
    .map_err(BilliardError::OrbitSearch)?;
    aggregate_orbits_with_dual_vertices_exact(
        dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(BilliardError::OrbitSearch)
}

pub fn capacity_auto(
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if classify_facets_from_dual_vertices(dual_vertices_f64).is_ok() {
        return capacity_billiard(
            dual_vertices,
            dual_vertices_f64,
            facet_intersection_is_nonempty,
            omega_signs,
        )
        .map_err(|err| match err {
            BilliardError::OrbitSearch(err) => err,
            BilliardError::NotLagrangianProduct { .. } | BilliardError::TooFewFacets { .. } => {
                unreachable!("classification was checked immediately before billiard routing")
            }
        });
    }

    capacity_pruned_hk2017(
        dual_vertices,
        dual_vertices_f64,
        facet_intersection_is_nonempty,
        omega_signs,
    )
}

/// Shared row schema for the first-order gradient harness.
#[derive(Debug, serde::Serialize)]
pub struct PredictionRow {
    pub phase: String,
    pub polytope_id: String,
    pub facet_count: usize,
    pub polytope_class: String,

    pub target: String,
    pub dir_idx: usize,
    pub t: f64,

    pub f_base: f64,
    pub f_perturbed: f64,
    pub grad_dot_d: f64,
    pub predicted_change: f64,
    pub actual_change: f64,
    pub residual: f64,
    pub residual_over_t: f64,

    pub log_t: f64,
    pub log_residual: f64,

    pub action_gap: Option<f64>,
    pub barely_cutting_delta: Option<f64>,
    pub min_facet_volume: Option<f64>,

    pub time_ms: f64,
}

/// Polytope with precomputed base values and KKT solution.
pub struct PolytopeInfo {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    pub vertices_f64: Vec<Vector4<f64>>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub cap: f64,
    pub vol: f64,
    pub sys: f64,
    pub best_perm: Vec<usize>,
    pub kkt: KktResult,
}

/// Sample a random unit vector in `R^{4F}`.
pub fn random_direction(f: usize, rng: &mut ChaCha8Rng) -> Vec<Vector4<f64>> {
    let mut dir: Vec<Vector4<f64>> = (0..f)
        .map(|_| {
            Vector4::new(
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
            )
        })
        .collect();
    let norm = dir.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
    if norm > 1e-10 {
        for v in &mut dir {
            *v /= norm;
        }
    }
    dir
}

pub fn ehz_capacity_safe(
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Option<OrbitSearchResult> {
    capacity_auto(
        dual_vertices,
        dual_vertices_f64,
        facet_intersection_is_nonempty,
        omega_signs,
    )
    .ok()
}

pub fn solve_kkt_safe(dual_vertices_f64: &[Vector4<f64>], perm: &[usize]) -> Option<KktResult> {
    solve_kkt_for_dual_vertices(dual_vertices_f64, perm).feasible()
}

/// Compute dsys/da_k via quotient rule: sys = c^2/(2*vol).
/// dsys/da_k = (c*dc/da_k - sys*dvol/da_k) / vol.
/// [cor:sys-derivative] quotient-rule derivative of the systolic ratio.
/// In formal/capacity-derivatives.tex.
fn sys_derivatives_a(
    d_cap: &[Vector4<f64>],
    d_vol: &[Vector4<f64>],
    cap: f64,
    vol: f64,
    sys: f64,
) -> Vec<Vector4<f64>> {
    d_vol
        .iter()
        .zip(d_cap.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Compute capacity, volume, sys, and KKT for a polytope's best orbit.
pub fn analyze_polytope(
    dual_vertices: &[[BigRational; 4]],
    vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    vertices_f64: &[Vector4<f64>],
    vertex_facet_incidence: &DMatrix<bool>,
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Option<PolytopeInfo> {
    let ehz = ehz_capacity_safe(
        dual_vertices,
        dual_vertices_f64,
        facet_intersection_is_nonempty,
        omega_signs,
    )?;
    let cap = ehz.capacity();
    let vol = euclidean_volume_f64(vertices, vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    let best_perm = ehz.best_sigma().to_vec();
    let kkt = solve_kkt_safe(dual_vertices_f64, &best_perm)?;
    Some(PolytopeInfo {
        dual_vertices: dual_vertices.to_vec(),
        vertices: vertices.to_vec(),
        dual_vertices_f64: dual_vertices_f64.to_vec(),
        vertices_f64: vertices_f64.to_vec(),
        vertex_facet_incidence: vertex_facet_incidence.clone(),
        cap,
        vol,
        sys,
        best_perm,
        kkt,
    })
}

/// Values of capacity, volume, and sys at a perturbed point `a + t*d`.
struct PerturbedValues {
    capacity: Option<f64>,
    volume: Option<f64>,
    sys: Option<f64>,
}

/// Compute cap, vol, sys at perturbed dual vertices `a + t*d`.
///
/// Capacity uses the flat dual-vertex KKT solver with the base orbit on the
/// perturbed polytope.
fn compute_perturbed(
    base_duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
    base_perm: &[usize],
) -> PerturbedValues {
    let perturbed: Vec<Vector4<f64>> = base_duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();

    let polytope = match GradientPolytopeCache::from_f64(perturbed) {
        Some(p) => p,
        None => {
            return PerturbedValues {
                capacity: None,
                volume: None,
                sys: None,
            }
        }
    };

    let cap = solve_kkt_safe(&polytope.dual_vertices_f64, base_perm)
        .filter(|kkt| kkt.q_corrected > EPS_Q_POSITIVE && kkt.beta.iter().all(|&b| b > 0.0))
        .map(|kkt| 0.5 / kkt.q_corrected);

    let vol = {
        let v = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
        (v > 0.0).then_some(v)
    };

    let sys = match (cap, vol) {
        (Some(c), Some(v)) => Some(c * c / (2.0 * v)),
        _ => None,
    };

    PerturbedValues {
        capacity: cap,
        volume: vol,
        sys,
    }
}

/// Run the first-order prediction test for all three targets on one polytope.
pub fn first_order_test(
    info: &PolytopeInfo,
    phase: &str,
    polytope_id: &str,
    polytope_class: &str,
    n_dirs: usize,
    rng: &mut ChaCha8Rng,
    action_gap: Option<f64>,
    barely_cutting_delta: Option<f64>,
    min_facet_volume: Option<f64>,
) -> Vec<PredictionRow> {
    let duals = &info.dual_vertices_f64;
    let f = duals.len();

    // Analytical gradients for all three targets.
    let g_cap = capacity_derivatives_a_from_kkt_result(duals, &info.best_perm, &info.kkt);
    let g_vol = volume_derivatives_a(duals, &info.vertices_f64, &info.vertex_facet_incidence)
        .expect("gradient validation polytope has valid finite geometry");
    let g_sys = sys_derivatives_a(&g_cap, &g_vol, info.cap, info.vol, info.sys);

    let targets: [(&str, f64, &[Vector4<f64>]); 3] = [
        ("capacity", info.cap, &g_cap),
        ("volume", info.vol, &g_vol),
        ("sys", info.sys, &g_sys),
    ];

    let mut rows = Vec::new();

    for dir_idx in 0..n_dirs {
        let direction = random_direction(f, rng);
        let gd: Vec<f64> = targets
            .iter()
            .map(|(_, _, g)| directional_derivative_a(g, &direction))
            .collect();

        for &t in T_VALUES {
            let t0 = std::time::Instant::now();
            let perturbed = compute_perturbed(duals, &direction, t, &info.best_perm);
            let elapsed = t0.elapsed().as_secs_f64() * 1000.0;

            let f_perturbed = [perturbed.capacity, perturbed.volume, perturbed.sys];

            for (i, &(target_name, f_base, _)) in targets.iter().enumerate() {
                if let Some(f_pert) = f_perturbed[i] {
                    let actual = f_pert - f_base;
                    let predicted = t * gd[i];
                    let residual = (actual - predicted).abs();
                    let rot = residual / t.abs();
                    let log_residual = residual.max(1e-300).log10();

                    rows.push(PredictionRow {
                        phase: phase.to_string(),
                        polytope_id: polytope_id.to_string(),
                        facet_count: f,
                        polytope_class: polytope_class.to_string(),
                        target: target_name.to_string(),
                        dir_idx,
                        t,
                        f_base,
                        f_perturbed: f_pert,
                        grad_dot_d: gd[i],
                        predicted_change: predicted,
                        actual_change: actual,
                        residual,
                        residual_over_t: rot,
                        log_t: t.abs().log10(),
                        log_residual,
                        action_gap,
                        barely_cutting_delta,
                        min_facet_volume,
                        time_ms: elapsed,
                    });
                }
            }
        }
    }

    rows
}

/// Write first-order rows to a JSONL writer.
pub fn write_rows(writer: &mut std::io::BufWriter<std::fs::File>, rows: &[PredictionRow]) {
    for row in rows {
        let json = serde_json::to_string(row).expect("serialize row");
        use std::io::Write as _;
        writeln!(writer, "{}", json).expect("write row");
    }
}

/// Detect `--smoke` in the process arguments.
pub fn smoke_mode() -> bool {
    std::env::args().skip(1).any(|arg| arg == "--smoke")
}

/// Create a temporary smoke-output directory with a label prefix.
pub fn smoke_output_dir(label: &str) -> String {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{label}-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create smoke output dir");
    dir.to_string_lossy().into_owned()
}

/// Enumerate all certified orbits for a polytope (strict: beta > EPS, Q > EPS).
/// Returns `(action, sigma, kkt_result)` sorted by action ascending.
pub fn enumerate_all_orbits(
    dual_vertices_f64: &[Vector4<f64>],
) -> Vec<(f64, Vec<usize>, KktResult)> {
    let f = dual_vertices_f64.len();
    let mut orbits = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(kkt) = solve_kkt_safe(dual_vertices_f64, perm) {
                    let min_beta = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
                    if min_beta > EPS_BETA_CERTIFIED && kkt.q_corrected > EPS_Q_POSITIVE {
                        let action = 0.5 / kkt.q_corrected;
                        orbits.push((action, perm.to_vec(), kkt));
                    }
                }
            });
        }
    }

    orbits.sort_by(|a, b| a.0.total_cmp(&b.0));
    orbits
}
