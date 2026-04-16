//! Shared helpers for gradient validation experiments.
//!
//! Instrument development: validates that analytical gradients (library derivatives.rs)
//! match finite-difference approximations across polytope classes and edge cases.

use nalgebra::Vector4;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use symplectic::{ehz_capacity, Polytope4D};

/// Shared strict beta-threshold for certified-orbit enumeration in the gradient package.
///
/// This matches `kkt::EPS_MARGIN_TRUE`.
pub const EPS_BETA_CERTIFIED: f64 = 1e-9;

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

pub fn ehz_capacity_safe(polytope: &Polytope4D) -> Option<symplectic::EhzResult> {
    ehz_capacity(polytope)
}

pub fn solve_kkt_safe(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult> {
    solve_kkt_for(polytope, perm).feasible()
}

/// Enumerate all certified orbits for a polytope (strict: beta > EPS, Q > EPS).
/// Returns `(action, sigma, kkt_result)` sorted by action ascending.
pub fn enumerate_all_orbits(polytope: &Polytope4D) -> Vec<(f64, Vec<usize>, KktResult)> {
    let f = polytope.facet_count();
    let mut orbits = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(kkt) = solve_kkt_safe(polytope, perm) {
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
