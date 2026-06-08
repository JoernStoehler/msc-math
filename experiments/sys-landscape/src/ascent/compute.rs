use crate::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use num_rational::BigRational;
use symplectic::algorithms::billiard::facet_classification::FacetClassification;
use symplectic::database::OrbitScalars;
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::{systolic_ratio, OrbitAdmissibility, OrbitKktData, OrbitSearchResult};

/// Numerical zero threshold for gradient checks.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Relative tie tolerance for admissible orbit actions in the scalar capacity
/// minimum. This keeps the nonsmooth direction model aligned with
/// `OrbitSearchResult::capacity()`, which already ignores indeterminate
/// candidates.
const ACTIVE_ORBIT_RTOL: f64 = 1e-9;

/// Shared local state for one ascent iteration.
///
/// This packages the active-orbit capacity result together with the smooth
/// volume term. It does not choose a single orbit branch.
#[derive(Clone, Debug)]
pub struct ActiveSysState {
    pub capacity: OrbitSearchResult,
    pub vol: f64,
    pub sys: f64,
}

#[derive(Clone, Debug)]
pub struct SysComputation {
    pub capacity: OrbitSearchResult,
    pub vol: f64,
    pub sys: f64,
}

/// Mode-specific projection for the ascent direction.
#[derive(Clone, Copy, Debug)]
pub enum AscentMode<'a> {
    General,
    LagrangianProduct {
        classification: &'a FacetClassification,
    },
}

/// Compute the active-orbit local state for one polytope.
pub fn compute_active_sys_state(polytope: &SysLandscapePolytopeCache) -> Option<ActiveSysState> {
    let capacity = compute_capacity_result(polytope)?;
    let vol =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let sys = systolic_ratio(capacity.capacity(), vol);
    sys.is_finite()
        .then_some(ActiveSysState { capacity, vol, sys })
}

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) from a cached capacity result.
///
/// `capacity` must come from the same `polytope`.
pub fn compute_sys_from_capacity(
    polytope: &SysLandscapePolytopeCache,
    capacity: &OrbitSearchResult,
) -> Option<f64> {
    let vol =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let cap = capacity.capacity();
    let sys = systolic_ratio(cap, vol);
    sys.is_finite().then_some(sys)
}

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) for a polytope using HK2017.
pub fn compute_sys(polytope: &SysLandscapePolytopeCache) -> Option<f64> {
    Some(compute_sys_computation(polytope)?.sys)
}

/// Compute sys and keep the capacity payload for producer audit rows.
pub fn compute_sys_computation(polytope: &SysLandscapePolytopeCache) -> Option<SysComputation> {
    let vol =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let capacity = compute_capacity_result(polytope)?;
    let cap = capacity.capacity();
    let sys = systolic_ratio(cap, vol);
    sys.is_finite()
        .then(|| SysComputation { capacity, vol, sys })
}

/// Compute the active-orbit capacity result.
pub fn compute_capacity_result(polytope: &SysLandscapePolytopeCache) -> Option<OrbitSearchResult> {
    crate::capacity_auto(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .ok()
}

pub fn orbit_scalars_from_result(result: &OrbitSearchResult) -> OrbitScalars {
    let best = result.best_orbit();
    OrbitScalars {
        iterations: result.iterations,
        returned_orbit_count: result.orbits.len(),
        best_beta_margin: best.beta_margin,
        best_q_error_bound: best.q_error_bound,
        best_has_mu: best.mu.is_some(),
        best_has_xi: best.xi.is_some(),
        best_is_admissible_exact: matches!(best.admissibility, OrbitAdmissibility::AdmissibleExact),
        best_is_indeterminate_f64: matches!(
            best.admissibility,
            OrbitAdmissibility::IndeterminateF64
        ),
    }
}

fn flatten_gradient(grad: &[Vector4<f64>]) -> Vec<f64> {
    grad.iter()
        .flat_map(|vk| [vk[0], vk[1], vk[2], vk[3]])
        .collect()
}

fn unflatten_direction(flat: &[f64]) -> Vec<Vector4<f64>> {
    flat.chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn coordinate_bounds(flat_idx: usize, mode: AscentMode<'_>) -> (f64, f64) {
    let facet = flat_idx / 4;
    let component = flat_idx % 4;

    match mode {
        AscentMode::General => (-1.0, 1.0),
        AscentMode::LagrangianProduct { classification } => {
            let q_forbidden = classification.q_indices.contains(&facet) && component >= 2;
            let p_forbidden = classification.p_indices.contains(&facet) && component < 2;
            if q_forbidden || p_forbidden {
                (0.0, 0.0)
            } else {
                (-1.0, 1.0)
            }
        }
    }
}

pub(crate) fn maximin_subgradient_direction(
    subdiff: &[Vec<Vector4<f64>>],
    facet_count: usize,
    mode: AscentMode<'_>,
) -> Option<Vec<Vector4<f64>>> {
    let dim = facet_count * 4;
    let mut vars = variables!();
    let direction_vars: Vec<_> = (0..dim)
        .map(|flat_idx| {
            let (min, max) = coordinate_bounds(flat_idx, mode);
            vars.add(variable().min(min).max(max))
        })
        .collect();
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let mut model = vars.maximise(Expression::from(t_var)).using(default_solver);
    for grad in subdiff {
        let flat_grad = flatten_gradient(grad);
        let mut lhs = Expression::from(0.0);
        for (coeff, var) in flat_grad.iter().zip(&direction_vars) {
            if *coeff != 0.0 {
                lhs += *coeff * *var;
            }
        }
        model = model.with(constraint!(lhs >= t_var));
    }

    let solution = model.solve().ok()?;
    let flat_direction: Vec<f64> = direction_vars
        .iter()
        .map(|var| solution.value(*var))
        .collect();
    let direction = unflatten_direction(&flat_direction);
    let predicted = clarke_directional_derivative_a(subdiff, &direction).ok()?;

    (predicted > EPS_NUMERICAL_ZERO).then_some(direction)
}

pub(crate) fn admissible_active_orbits(result: &OrbitSearchResult) -> Vec<&OrbitKktData> {
    let tol = ACTIVE_ORBIT_RTOL * result.min_action.abs().max(1.0);
    let active: Vec<&OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| (orbit.action - result.min_action).abs() <= tol)
        .collect();

    if active.is_empty() {
        vec![result.best_orbit()]
    } else {
        active
    }
}

/// Build the ascent direction for a single polytope state.
///
/// With a single active orbit, this reduces to that branch gradient. At
/// switching points, it solves a maximin LP for a feasible direction `d`
/// satisfying `max_d min_i <∇sys_i, d>` under box bounds on the ambient
/// coordinates.
pub fn ascent_direction(
    polytope: &SysLandscapePolytopeCache,
    state: &ActiveSysState,
    mode: AscentMode<'_>,
) -> Option<Vec<Vector4<f64>>> {
    let active_orbits: Vec<OrbitKktData> = admissible_active_orbits(&state.capacity)
        .into_iter()
        .cloned()
        .collect();
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .ok()?;
    let d_capacity_da =
        capacity_subgradients_a(&polytope.dual_vertices_f64, &active_orbits).ok()?;
    let subdiff: Vec<Vec<Vector4<f64>>> = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(
                state.capacity.capacity(),
                state.vol,
                capacity_gradient,
                &d_volume_da,
            )
        })
        .collect();
    match subdiff.as_slice() {
        [] => None,
        [single] => {
            let mut direction = single.clone();
            if let AscentMode::LagrangianProduct { classification } = mode {
                classification.mask_dual_direction_in_place(&mut direction);
            }
            Some(direction)
        }
        _ => maximin_subgradient_direction(&subdiff, polytope.facet_count(), mode),
    }
}

/// Try a step in dual-vertex space: a_k(t) = a_k + t * d_k.
pub fn apply_dual_step(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(SysLandscapePolytopeCache, f64)> {
    let (polytope, computation) = apply_dual_step_with_computation(duals, direction, t)?;
    Some((polytope, computation.sys))
}

/// Try a step and keep the capacity payload for producer audit rows.
pub fn apply_dual_step_with_computation(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(SysLandscapePolytopeCache, SysComputation)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(new_duals)?;
    let computation = compute_sys_computation(&polytope)?;
    Some((polytope, computation))
}

pub fn rational_vec4_to_strings(data: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
        .collect()
}

pub fn dual_vertices_rational_strings(polytope: &SysLandscapePolytopeCache) -> Vec<[String; 4]> {
    rational_vec4_to_strings(&polytope.dual_vertices)
}
