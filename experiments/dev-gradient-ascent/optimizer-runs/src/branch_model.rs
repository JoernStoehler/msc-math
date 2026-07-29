use crate::evaluator::{capacity_with_action_gap, Evaluation};
use crate::quotient::{flatten, quotient_basis, unflatten};
use clarabel::algebra::CscMatrix;
use clarabel::solver::{
    DefaultSettingsBuilder, DefaultSolver, IPSolver, NonnegativeConeT, SecondOrderConeT,
    SolverStatus,
};
use exp_sys_landscape::SysLandscapePolytopeCache;
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, DVector, Vector4};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::derivatives::{
    capacity_derivatives_a, capacity_derivatives_a_from_orbit, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::geom::symplectic_form::omega0;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, solve_orbit_sigma_saddle_point, OrbitAdmissibility,
    OrbitGuaranteeMode,
};

const RAW_EIGEN_FLOOR: f64 = 1.0e-10;
const RAW_KKT_RESIDUAL_MAX: f64 = 1.0e-7;
const RAW_Q_POSITIVE: f64 = 1.0e-12;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SliceMode {
    Ambient,
    SymmetryTransverse,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormMode {
    BoxLinf,
    EuclideanL2,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BranchExtensionMode {
    None,
    TransitionBlockedAdmissible,
}

#[derive(Clone, Debug)]
pub struct BranchModelConfig {
    pub candidate_window_relative: f64,
    pub extension_mode: BranchExtensionMode,
}

#[derive(Clone, Debug, Serialize)]
pub struct BranchModelTiming {
    pub candidate_search_ms: f64,
    pub derivative_ms: f64,
    pub extension_enumeration_ms: f64,
    pub total_ms: f64,
}

#[derive(Clone, Debug)]
pub struct LinearBranch {
    pub sigma: Vec<usize>,
    pub gap: f64,
    pub gradient: Vec<Vector4<f64>>,
    pub blocked_edges: Vec<[usize; 2]>,
    pub source: &'static str,
    pub beta_margin: f64,
    pub beta_scale: f64,
    pub requires_transition_reachability: bool,
}

#[derive(Clone, Debug)]
pub struct BranchModel {
    pub base_sys: f64,
    pub candidates: Vec<LinearBranch>,
    pub extended: Vec<LinearBranch>,
    pub timing: BranchModelTiming,
}

#[derive(Clone, Debug, Serialize)]
pub struct DirectionSolution {
    pub displacement_flat: Vec<f64>,
    pub predicted_delta: f64,
    pub predicted_winning_sigma: Vec<usize>,
    pub represented_branch_count: usize,
    pub candidate_branch_count: usize,
    pub reachable_extended_branch_count: usize,
    pub slice_mode: SliceMode,
    pub solve_ms: f64,
}

impl BranchModel {
    pub fn build_from_named_candidates(
        polytope: &SysLandscapePolytopeCache,
        volume: f64,
        sigmas: &[Vec<usize>],
    ) -> Result<Self, String> {
        let total_started = Instant::now();
        let candidate_started = Instant::now();
        let transition = build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
        let orbits = sigmas
            .iter()
            .filter(|sigma| is_feasible_cycle(sigma, &transition))
            .filter_map(|sigma| {
                let orbit =
                    solve_orbit_sigma_saddle_point(&polytope.dual_vertices_f64, sigma).ok()?;
                if orbit.admissibility == OrbitAdmissibility::AdmissibleF64 {
                    return Some(orbit);
                }
                aggregate_orbits_with_dual_vertices_exact(
                    &polytope.dual_vertices,
                    vec![orbit],
                    1,
                    0.0,
                    OrbitGuaranteeMode::AllSafe,
                )
                .ok()
                .map(|result| result.best_orbit().clone())
            })
            .collect::<Vec<_>>();
        let candidate_search_ms = candidate_started.elapsed().as_secs_f64() * 1000.0;
        let min_action = orbits
            .iter()
            .map(|orbit| orbit.action)
            .min_by(f64::total_cmp)
            .ok_or("no named candidate is admissible at the surrogate point")?;
        let base_sys = symplectic::systolic_ratio(min_action, volume);
        let derivative_started = Instant::now();
        let d_volume = volume_derivatives_a(
            &polytope.dual_vertices_f64,
            &polytope.vertices_f64,
            &polytope.vertex_facet_incidence,
        )
        .map_err(|error| format!("volume derivative failed: {error:?}"))?;
        let mut candidates = orbits
            .into_iter()
            .map(|orbit| {
                let mu = orbit
                    .mu
                    .as_ref()
                    .ok_or("named branch derivative lacks closure multiplier")?;
                let d_capacity = capacity_derivatives_a(
                    &orbit.beta,
                    orbit.q,
                    mu,
                    &orbit.sigma,
                    &polytope.dual_vertices_f64,
                );
                let gradient =
                    systolic_ratio_gradient_a(orbit.action, volume, &d_capacity, &d_volume);
                if !gradient
                    .iter()
                    .flat_map(|entry| entry.iter())
                    .all(|entry| entry.is_finite())
                {
                    return Err("nonfinite named branch derivative".to_string());
                }
                Ok(LinearBranch {
                    sigma: orbit.sigma,
                    gap: base_sys * ((orbit.action / min_action).powi(2) - 1.0),
                    gradient,
                    blocked_edges: Vec::new(),
                    source: "named_candidate",
                    beta_margin: orbit.beta_margin,
                    beta_scale: orbit
                        .beta
                        .iter()
                        .map(|value| value.abs())
                        .fold(0.0_f64, f64::max),
                    requires_transition_reachability: false,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        candidates.sort_by(|left, right| {
            left.gap
                .total_cmp(&right.gap)
                .then_with(|| left.sigma.cmp(&right.sigma))
        });
        let derivative_ms = derivative_started.elapsed().as_secs_f64() * 1000.0;
        Ok(Self {
            base_sys,
            candidates,
            extended: Vec::new(),
            timing: BranchModelTiming {
                candidate_search_ms,
                derivative_ms,
                extension_enumeration_ms: 0.0,
                total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
            },
        })
    }

    pub fn build(evaluation: &Evaluation, config: &BranchModelConfig) -> Result<Self, String> {
        let total_started = Instant::now();
        let context = evaluation
            .context
            .as_ref()
            .ok_or("evaluation lacks reusable geometry context")?;
        let base_sys = evaluation.row.sys.ok_or("evaluation lacks sys")?;
        let candidate_started = Instant::now();
        let capacity = capacity_with_action_gap(
            &context.polytope,
            context.min_action * config.candidate_window_relative,
        )
        .map_err(|error| format!("candidate branch search failed: {error:?}"))?;
        let candidate_search_ms = candidate_started.elapsed().as_secs_f64() * 1000.0;

        let derivative_started = Instant::now();
        let d_volume = volume_derivatives_a(
            &context.polytope.dual_vertices_f64,
            &context.polytope.vertices_f64,
            &context.polytope.vertex_facet_incidence,
        )
        .map_err(|error| format!("volume derivative failed: {error:?}"))?;
        let gradient_from_parts = |action: f64,
                                   sigma: &[usize],
                                   beta: &[f64],
                                   q: f64,
                                   mu: &[f64; 4]|
         -> Result<Vec<Vector4<f64>>, String> {
            let d_capacity =
                capacity_derivatives_a(beta, q, mu, sigma, &context.polytope.dual_vertices_f64);
            let value = systolic_ratio_gradient_a(action, context.volume, &d_capacity, &d_volume);
            if value
                .iter()
                .flat_map(|entry| entry.iter())
                .all(|entry| entry.is_finite())
            {
                Ok(value)
            } else {
                Err("nonfinite branch derivative".to_string())
            }
        };
        let gradient = |orbit: &symplectic::OrbitKktData| -> Result<Vec<Vector4<f64>>, String> {
            let mu = orbit
                .mu
                .as_ref()
                .ok_or("branch derivative lacks closure multiplier")?;
            gradient_from_parts(orbit.action, &orbit.sigma, &orbit.beta, orbit.q, mu)
        };
        let cutoff = context.min_action * (1.0 + config.candidate_window_relative);
        let mut candidates = capacity
            .orbits
            .iter()
            .filter(|orbit| {
                matches!(
                    orbit.admissibility,
                    OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
                ) && orbit.action <= cutoff
            })
            .map(|orbit| {
                Ok(LinearBranch {
                    sigma: orbit.sigma.clone(),
                    gap: base_sys * ((orbit.action / context.min_action).powi(2) - 1.0),
                    gradient: gradient(orbit)?,
                    blocked_edges: Vec::new(),
                    source: "candidate_window",
                    beta_margin: orbit.beta_margin,
                    beta_scale: orbit
                        .beta
                        .iter()
                        .map(|value| value.abs())
                        .fold(0.0_f64, f64::max),
                    requires_transition_reachability: false,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        candidates.sort_by(|left, right| {
            left.gap
                .total_cmp(&right.gap)
                .then_with(|| left.sigma.cmp(&right.sigma))
        });
        if candidates.is_empty() {
            return Err(
                "candidate search returned no differentiable admissible branch".to_string(),
            );
        }
        let derivative_ms = derivative_started.elapsed().as_secs_f64() * 1000.0;

        let extension_started = Instant::now();
        let transition = build_transition_matrix_from_facet_intersections_and_omega(
            &context.polytope.facet_intersection_is_nonempty,
            &context.polytope.omega_signs,
        );
        let mut extended = Vec::new();
        match config.extension_mode {
            BranchExtensionMode::None => {}
            BranchExtensionMode::TransitionBlockedAdmissible => {
                for sigma in SimpleDirectedCyclesCanonical::new(
                    &context.polytope.facet_intersection_is_nonempty,
                ) {
                    if is_feasible_cycle(&sigma, &transition) {
                        continue;
                    }
                    let blocked_edges = blocked_edges(&sigma, &transition);
                    let Ok(orbit) =
                        solve_orbit_sigma_saddle_point(&context.polytope.dual_vertices_f64, &sigma)
                    else {
                        continue;
                    };
                    if orbit.admissibility != OrbitAdmissibility::AdmissibleF64
                        || orbit.action > cutoff
                    {
                        continue;
                    }
                    let Ok(branch_gradient) = gradient(&orbit) else {
                        continue;
                    };
                    extended.push(LinearBranch {
                        sigma,
                        gap: base_sys * ((orbit.action / context.min_action).powi(2) - 1.0),
                        gradient: branch_gradient,
                        blocked_edges,
                        source: "transition_blocked_admissible",
                        beta_margin: orbit.beta_margin,
                        beta_scale: orbit
                            .beta
                            .iter()
                            .map(|value| value.abs())
                            .fold(0.0_f64, f64::max),
                        requires_transition_reachability: true,
                    });
                }
            }
        }
        extended.sort_by(|left, right| {
            left.gap
                .total_cmp(&right.gap)
                .then_with(|| left.sigma.cmp(&right.sigma))
        });
        let extension_enumeration_ms = extension_started.elapsed().as_secs_f64() * 1000.0;
        Ok(Self {
            base_sys,
            candidates,
            extended,
            timing: BranchModelTiming {
                candidate_search_ms,
                derivative_ms,
                extension_enumeration_ms,
                total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
            },
        })
    }

    pub fn primary_gradient(&self) -> &[Vector4<f64>] {
        &self.candidates[0].gradient
    }

    pub fn solve_box(
        &self,
        base_duals: &[Vector4<f64>],
        radius: f64,
        slice_mode: SliceMode,
        extension_reachability_scale: f64,
    ) -> Result<DirectionSolution, String> {
        let started = Instant::now();
        let dimension = base_duals.len() * 4;
        let absolute_distance = radius * (dimension as f64).sqrt();
        let reachable = self
            .extended
            .iter()
            .filter(|branch| {
                extended_branch_is_reachable(
                    branch,
                    base_duals,
                    absolute_distance,
                    extension_reachability_scale,
                )
            })
            .collect::<Vec<_>>();
        let branches = self
            .candidates
            .iter()
            .chain(reachable.iter().copied())
            .collect::<Vec<_>>();
        let mut variables = variables!();
        let coordinates = (0..dimension)
            .map(|_| variables.add(variable().min(-1.0).max(1.0)))
            .collect::<Vec<_>>();
        let minimum = variables.add(variable().min(f64::NEG_INFINITY));
        let mut problem = variables
            .maximise(Expression::from(minimum))
            .using(default_solver);
        for branch in &branches {
            let mut value = Expression::from(branch.gap);
            for (coefficient, coordinate) in flatten(&branch.gradient).iter().zip(&coordinates) {
                value += radius * *coefficient * *coordinate;
            }
            problem = problem.with(constraint!(value >= minimum));
        }
        if slice_mode == SliceMode::SymmetryTransverse {
            let quotient = quotient_basis(base_duals)?;
            for axis in quotient.orbit_basis {
                let mut projection = Expression::from(0.0);
                for (coefficient, coordinate) in axis.iter().zip(&coordinates) {
                    projection += *coefficient * *coordinate;
                }
                problem = problem.with(constraint!(projection == 0.0));
            }
        }
        let solution = problem
            .solve()
            .map_err(|error| format!("branch model LP failed: {error}"))?;
        let direction_flat = coordinates
            .iter()
            .map(|coordinate| solution.value(*coordinate))
            .collect::<Vec<_>>();
        if !direction_flat.iter().all(|value| value.is_finite()) {
            return Err("branch model returned a nonfinite direction".to_string());
        }
        let direction = unflatten(&direction_flat)?;
        let values = branches
            .iter()
            .map(|branch| {
                branch.gap
                    + radius
                        * branch
                            .gradient
                            .iter()
                            .zip(&direction)
                            .map(|(gradient, delta)| gradient.dot(delta))
                            .sum::<f64>()
            })
            .collect::<Vec<_>>();
        let (winner, predicted_delta) = values
            .iter()
            .enumerate()
            .min_by(|left, right| left.1.total_cmp(right.1))
            .ok_or("branch model has no represented branches")?;
        Ok(DirectionSolution {
            displacement_flat: direction_flat
                .into_iter()
                .map(|coordinate| radius * coordinate)
                .collect(),
            predicted_delta: *predicted_delta,
            predicted_winning_sigma: branches[winner].sigma.clone(),
            represented_branch_count: branches.len(),
            candidate_branch_count: self.candidates.len(),
            reachable_extended_branch_count: reachable.len(),
            slice_mode,
            solve_ms: started.elapsed().as_secs_f64() * 1000.0,
        })
    }

    pub fn solve_euclidean(
        &self,
        base_duals: &[Vector4<f64>],
        normalized_distance: f64,
        slice_mode: SliceMode,
        extension_reachability_scale: f64,
    ) -> Result<DirectionSolution, String> {
        let started = Instant::now();
        let base_norm = flatten(base_duals)
            .iter()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt();
        if !normalized_distance.is_finite()
            || normalized_distance <= 0.0
            || !base_norm.is_finite()
            || base_norm <= 0.0
        {
            return Err("invalid normalized Euclidean distance".to_string());
        }
        let absolute_distance = normalized_distance * base_norm;
        let reachable = self
            .extended
            .iter()
            .filter(|branch| {
                extended_branch_is_reachable(
                    branch,
                    base_duals,
                    absolute_distance,
                    extension_reachability_scale,
                )
            })
            .collect::<Vec<_>>();
        let branches = self
            .candidates
            .iter()
            .chain(reachable.iter().copied())
            .collect::<Vec<_>>();
        let quotient = (slice_mode == SliceMode::SymmetryTransverse)
            .then(|| quotient_basis(base_duals))
            .transpose()?;
        let gradients = branches
            .iter()
            .map(|branch| {
                let flat = DVector::from_vec(flatten(&branch.gradient));
                match &quotient {
                    Some(quotient) => quotient
                        .slice_basis
                        .iter()
                        .map(|axis| axis.dot(&flat))
                        .collect::<Vec<_>>(),
                    None => flat.as_slice().to_vec(),
                }
            })
            .collect::<Vec<_>>();
        let gaps = branches.iter().map(|branch| branch.gap).collect::<Vec<_>>();
        let (coordinates, predicted_delta, winner) =
            solve_bounded_affine_minimum(&gradients, &gaps, absolute_distance)?;
        let displacement = match quotient {
            Some(quotient) => {
                let mut ambient = DVector::zeros(base_duals.len() * 4);
                for (coordinate, axis) in coordinates.iter().zip(&quotient.slice_basis) {
                    ambient += axis * *coordinate;
                }
                ambient
            }
            None => DVector::from_vec(coordinates),
        };
        Ok(DirectionSolution {
            displacement_flat: displacement.as_slice().to_vec(),
            predicted_delta,
            predicted_winning_sigma: branches[winner].sigma.clone(),
            represented_branch_count: branches.len(),
            candidate_branch_count: self.candidates.len(),
            reachable_extended_branch_count: reachable.len(),
            slice_mode,
            solve_ms: started.elapsed().as_secs_f64() * 1000.0,
        })
    }

    pub fn predict_displacement(
        &self,
        base_duals: &[Vector4<f64>],
        displacement: &DVector<f64>,
        extension_reachability_scale: f64,
    ) -> Result<(f64, Vec<usize>, usize), String> {
        if displacement.len() != base_duals.len() * 4 {
            return Err("prediction displacement has wrong dimension".to_string());
        }
        let absolute_distance = displacement.norm();
        let reachable = self
            .extended
            .iter()
            .filter(|branch| {
                extended_branch_is_reachable(
                    branch,
                    base_duals,
                    absolute_distance,
                    extension_reachability_scale,
                )
            })
            .collect::<Vec<_>>();
        let (branch, value) = self
            .candidates
            .iter()
            .chain(reachable.iter().copied())
            .map(|branch| {
                let value = branch.gap
                    + flatten(&branch.gradient)
                        .iter()
                        .zip(displacement.iter())
                        .map(|(gradient, delta)| gradient * delta)
                        .sum::<f64>();
                (branch, value)
            })
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .ok_or("branch model has no represented branches")?;
        Ok((value, branch.sigma.clone(), reachable.len()))
    }
}

fn l2_norm_flat(values: &[f64]) -> f64 {
    values.iter().map(|value| value * value).sum::<f64>().sqrt()
}

fn extended_branch_is_reachable(
    branch: &LinearBranch,
    base_duals: &[Vector4<f64>],
    absolute_distance: f64,
    reachability_scale: f64,
) -> bool {
    if !absolute_distance.is_finite()
        || absolute_distance < 0.0
        || !reachability_scale.is_finite()
        || reachability_scale < 0.0
    {
        return false;
    }
    if branch.requires_transition_reachability
        && !branch.blocked_edges.iter().all(|&[from, to]| {
            let a_from = &base_duals[from];
            let a_to = &base_duals[to];
            let omega = omega0(a_from, a_to);
            let max_linear_change = 2.0 * absolute_distance * (a_from.norm() + a_to.norm());
            let max_quadratic_change = 4.0 * absolute_distance * absolute_distance;
            omega >= 0.0
                || -omega <= reachability_scale * (max_linear_change + max_quadratic_change)
        })
    {
        return false;
    }
    let gradient_norm = l2_norm_flat(&flatten(&branch.gradient));
    branch.gap <= reachability_scale * gradient_norm * absolute_distance
}

/// Unrestricted f64 KKT solution for one named sigma.
///
/// This is a diagnostic branch germ, not an admissible orbit: callers must
/// separately check transition feasibility and beta. Failure means that this
/// heuristic eigensolve did not meet its residual/positivity contract.
#[derive(Clone, Debug)]
pub struct RawSysext {
    pub beta: Vec<f64>,
    pub beta_margin: f64,
    pub action: f64,
    pub q: f64,
    pub mu: [f64; 4],
}

#[derive(Clone, Debug)]
pub struct RawBetaDirectional {
    pub base: RawSysext,
    pub beta_directional: Vec<f64>,
    pub differentiated_residual: f64,
    pub retained_rank: usize,
    pub smallest_retained_eigenvalue_abs: f64,
    pub retained_condition_number: f64,
}

struct RawKktState {
    raw: RawSysext,
    kkt: DMatrix<f64>,
    solution: DVector<f64>,
    eigenvalues: DVector<f64>,
    eigenvectors: DMatrix<f64>,
}

pub fn solve_raw_sysext_kkt(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<RawSysext, String> {
    Ok(solve_raw_sysext_kkt_state(dual_vertices, sigma)?.raw)
}

/// Differentiate the unrestricted f64 KKT germ along a supplied displacement.
///
/// For `M(a) x(a) = b`, this solves
///
/// `M(a) dx = -dM(a) x`
///
/// with the same eigendecomposition and numerical rank threshold as the base
/// raw solve. This is a local diagnostic for a fixed-rank KKT branch. It is not
/// a production beta predicate: rank changes and near-null directions can make
/// the derivative discontinuous or nonunique.
pub fn solve_raw_beta_directional(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
    displacement: &[Vector4<f64>],
) -> Result<RawBetaDirectional, String> {
    if displacement.len() != dual_vertices.len() {
        return Err(format!(
            "raw beta displacement has {} vertices, expected {}",
            displacement.len(),
            dual_vertices.len()
        ));
    }
    let state = solve_raw_sysext_kkt_state(dual_vertices, sigma)?;
    let beta_count = sigma.len();
    let size = beta_count + 5;
    let mut differentiated_matrix = DMatrix::zeros(size, size);
    for i in 0..beta_count {
        let facet_i = sigma[i];
        for j in (i + 1)..beta_count {
            let facet_j = sigma[j];
            let value = omega0(&displacement[facet_i], &dual_vertices[facet_j])
                + omega0(&dual_vertices[facet_i], &displacement[facet_j]);
            differentiated_matrix[(i, j)] = value;
            differentiated_matrix[(j, i)] = value;
        }
        for coordinate in 0..4 {
            let value = displacement[facet_i][coordinate];
            differentiated_matrix[(i, beta_count + coordinate)] = value;
            differentiated_matrix[(beta_count + coordinate, i)] = value;
        }
    }
    let differentiated_rhs = -&differentiated_matrix * &state.solution;
    let differentiated_solution =
        apply_raw_pseudoinverse(&state.eigenvalues, &state.eigenvectors, &differentiated_rhs);
    let differentiated_residual =
        (&state.kkt * &differentiated_solution + differentiated_matrix * &state.solution).norm();
    if differentiated_residual > RAW_KKT_RESIDUAL_MAX {
        return Err(format!(
            "raw beta differentiated residual is too large: {differentiated_residual:.3e}"
        ));
    }
    let retained_eigenvalues = state
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .filter(|value| *value > RAW_EIGEN_FLOOR)
        .collect::<Vec<_>>();
    let smallest_retained_eigenvalue_abs = retained_eigenvalues
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let largest_retained_eigenvalue_abs =
        retained_eigenvalues.iter().copied().fold(0.0_f64, f64::max);
    Ok(RawBetaDirectional {
        base: state.raw,
        beta_directional: differentiated_solution
            .rows(0, beta_count)
            .iter()
            .copied()
            .collect(),
        differentiated_residual,
        retained_rank: retained_eigenvalues.len(),
        smallest_retained_eigenvalue_abs,
        retained_condition_number: largest_retained_eigenvalue_abs
            / smallest_retained_eigenvalue_abs,
    })
}

fn solve_raw_sysext_kkt_state(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<RawKktState, String> {
    let (kkt, rhs) = symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices(
        dual_vertices,
        sigma,
    );
    let beta_count = rhs.len() - 5;
    let size = rhs.len();
    let eigen = kkt.clone().symmetric_eigen();
    let maximum_eigenvalue = eigen
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    if maximum_eigenvalue < RAW_EIGEN_FLOOR {
        return Err("raw sysext matrix is singular".to_string());
    }
    let solution = apply_raw_pseudoinverse(&eigen.eigenvalues, &eigen.eigenvectors, &rhs);
    let residual = &kkt * &solution - rhs;
    if residual.norm() > RAW_KKT_RESIDUAL_MAX {
        return Err("raw sysext residual is too large".to_string());
    }
    let residual_multiplier_dot = (beta_count..beta_count + 4)
        .map(|index| residual[index] * solution[index])
        .sum::<f64>();
    let correction = residual_multiplier_dot + residual[beta_count + 4] * solution[beta_count + 4];
    let beta = solution
        .rows(0, beta_count)
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let mut q = 0.0;
    for left in 0..beta_count {
        for right in 0..beta_count {
            q += beta[left] * kkt[(left, right)] * beta[right];
        }
    }
    q = 0.5 * q + correction;
    if !q.is_finite() || q <= RAW_Q_POSITIVE {
        return Err("raw sysext q is nonpositive".to_string());
    }
    let mu = <[f64; 4]>::try_from(
        (beta_count..beta_count + 4)
            .map(|index| solution[index])
            .collect::<Vec<_>>(),
    )
    .map_err(|_| "raw sysext closure multiplier has wrong dimension")?;
    Ok(RawKktState {
        raw: RawSysext {
            beta_margin: beta.iter().copied().fold(f64::INFINITY, f64::min),
            beta,
            action: 0.5 / q,
            q,
            mu,
        },
        kkt,
        solution,
        eigenvalues: eigen.eigenvalues,
        eigenvectors: eigen.eigenvectors,
    })
}

fn apply_raw_pseudoinverse(
    eigenvalues: &DVector<f64>,
    eigenvectors: &DMatrix<f64>,
    rhs: &DVector<f64>,
) -> DVector<f64> {
    let size = rhs.len();
    let mut solution = DVector::zeros(size);
    for index in 0..size {
        if eigenvalues[index].abs() > RAW_EIGEN_FLOOR {
            let coefficient = eigenvectors.column(index).dot(rhs) / eigenvalues[index];
            for row in 0..size {
                solution[row] += coefficient * eigenvectors[(row, index)];
            }
        }
    }
    solution
}

fn blocked_edges(sigma: &[usize], transition: &DMatrix<bool>) -> Vec<[usize; 2]> {
    sigma
        .iter()
        .copied()
        .zip(sigma.iter().copied().cycle().skip(1))
        .take(sigma.len())
        .filter(|&(from, to)| !transition[(from, to)])
        .map(|(from, to)| [from, to])
        .collect()
}

fn solve_bounded_affine_minimum(
    gradients: &[Vec<f64>],
    gaps: &[f64],
    radius: f64,
) -> Result<(Vec<f64>, f64, usize), String> {
    if gradients.is_empty() || gradients.len() != gaps.len() {
        return Err("invalid Euclidean branch arrays".to_string());
    }
    let dimension = gradients[0].len();
    if gradients.iter().any(|gradient| {
        gradient.len() != dimension || gradient.iter().any(|value| !value.is_finite())
    }) || gaps.iter().any(|gap| !gap.is_finite())
    {
        return Err("invalid Euclidean model coefficients".to_string());
    }
    let variable_count = dimension + 1;
    let quadratic = CscMatrix::from(&vec![vec![0.0; variable_count]; variable_count]);
    let mut objective = vec![0.0; variable_count];
    objective[dimension] = -1.0;
    let mut constraint_rows = gradients
        .iter()
        .map(|gradient| {
            let mut row = gradient.iter().map(|value| -*value).collect::<Vec<_>>();
            row.push(1.0);
            row
        })
        .collect::<Vec<_>>();
    constraint_rows.push(vec![0.0; variable_count]);
    for coordinate in 0..dimension {
        let mut row = vec![0.0; variable_count];
        row[coordinate] = -1.0;
        constraint_rows.push(row);
    }
    let constraints = CscMatrix::from(&constraint_rows);
    let mut rhs = gaps.to_vec();
    rhs.push(radius);
    rhs.extend(std::iter::repeat(0.0).take(dimension));
    let cones = [
        NonnegativeConeT(gradients.len()),
        SecondOrderConeT(dimension + 1),
    ];
    let settings = DefaultSettingsBuilder::default()
        .verbose(false)
        .tol_gap_abs(1.0e-10)
        .tol_gap_rel(1.0e-10)
        .tol_feas(1.0e-10)
        .build()
        .map_err(|error| format!("Euclidean model settings failed: {error:?}"))?;
    let mut solver =
        DefaultSolver::new(&quadratic, &objective, &constraints, &rhs, &cones, settings)
            .map_err(|error| format!("Euclidean model setup failed: {error:?}"))?;
    solver.solve();
    if !matches!(
        solver.solution.status,
        SolverStatus::Solved | SolverStatus::AlmostSolved
    ) {
        return Err(format!(
            "Euclidean model solver failed: {:?}",
            solver.solution.status
        ));
    }
    let coordinates = solver.solution.x[..dimension].to_vec();
    if coordinates.iter().any(|value| !value.is_finite()) {
        return Err("Euclidean model returned nonfinite coordinates".to_string());
    }
    let values = gradients
        .iter()
        .zip(gaps)
        .map(|(gradient, gap)| {
            *gap + gradient
                .iter()
                .zip(&coordinates)
                .map(|(left, right)| left * right)
                .sum::<f64>()
        })
        .collect::<Vec<_>>();
    let (winner, predicted_delta) = values
        .iter()
        .enumerate()
        .min_by(|left, right| left.1.total_cmp(right.1))
        .ok_or("Euclidean model has no winner")?;
    Ok((coordinates, *predicted_delta, winner))
}

pub fn normalized_primary_direction(
    model: &BranchModel,
    base_duals: &[Vector4<f64>],
    slice_mode: SliceMode,
) -> Result<DVector<f64>, String> {
    let mut direction = DVector::from_vec(flatten(model.primary_gradient()));
    if slice_mode == SliceMode::SymmetryTransverse {
        for axis in quotient_basis(base_duals)?.orbit_basis {
            direction -= &axis * axis.dot(&direction);
        }
    }
    let norm = direction.norm();
    if !norm.is_finite() || norm <= 1.0e-14 {
        return Err("primary branch direction is zero after projection".to_string());
    }
    Ok(direction / norm)
}

pub fn winning_gradient(evaluation: &Evaluation) -> Result<Vec<Vector4<f64>>, String> {
    let context = evaluation
        .context
        .as_ref()
        .ok_or("evaluation lacks reusable geometry context")?;
    let d_volume = volume_derivatives_a(
        &context.polytope.dual_vertices_f64,
        &context.polytope.vertices_f64,
        &context.polytope.vertex_facet_incidence,
    )
    .map_err(|error| format!("volume derivative failed: {error:?}"))?;
    let d_capacity = capacity_derivatives_a_from_orbit(
        &context.polytope.dual_vertices_f64,
        &context.winning_orbit,
    )
    .map_err(|error| format!("branch derivative failed: {error:?}"))?;
    let gradient = systolic_ratio_gradient_a(
        context.winning_orbit.action,
        context.volume,
        &d_capacity,
        &d_volume,
    );
    if gradient
        .iter()
        .flat_map(|entry| entry.iter())
        .all(|entry| entry.is_finite())
    {
        Ok(gradient)
    } else {
        Err("nonfinite winning-branch gradient".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_bounded_affine_minimum, solve_raw_beta_directional, solve_raw_sysext_kkt};
    use nalgebra::Vector4;
    use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
    use symplectic::{known_polytopes, solve_pruned_hk2017_candidates};

    #[test]
    fn euclidean_model_can_choose_an_interior_finite_step() {
        // Around a=1, (1-|a+da|) - (1-|a|) = min(-da, 2+da).
        let (coordinate, predicted, winner) =
            solve_bounded_affine_minimum(&[vec![-1.0], vec![1.0]], &[0.0, 2.0], 2.0)
                .expect("one-dimensional model solves");
        assert!((coordinate[0] + 1.0).abs() < 1.0e-8);
        assert!((predicted - 1.0).abs() < 1.0e-8);
        assert!(winner < 2);
    }

    #[test]
    fn euclidean_model_uses_boundary_before_interior_optimum() {
        let (coordinate, predicted, _) =
            solve_bounded_affine_minimum(&[vec![-1.0], vec![1.0]], &[0.0, 2.0], 0.5)
                .expect("one-dimensional model solves");
        assert!((coordinate[0] + 0.5).abs() < 1.0e-8);
        assert!((predicted - 0.5).abs() < 1.0e-8);
    }

    #[test]
    fn copied_raw_solver_agrees_with_public_solver_on_regular_branch() {
        let fixture = known_polytopes::simplex();
        let transition = build_transition_matrix_from_facet_intersections_and_omega(
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        );
        let (orbits, _) = solve_pruned_hk2017_candidates(&fixture.dual_vertices_f64, &transition)
            .expect("simplex candidate search succeeds");
        let ordinary = &orbits[0];
        let raw = solve_raw_sysext_kkt(&fixture.dual_vertices_f64, &ordinary.sigma)
            .expect("raw solve succeeds on ordinary branch");
        assert!((raw.action - ordinary.action).abs() < 1.0e-7);
        assert!((raw.q - ordinary.q).abs() < 1.0e-7);
        assert_eq!(raw.beta.len(), ordinary.beta.len());
        for (raw_beta, ordinary_beta) in raw.beta.iter().zip(&ordinary.beta) {
            assert!((raw_beta - ordinary_beta).abs() < 1.0e-7);
        }
    }

    #[test]
    fn raw_beta_directional_matches_centered_difference_on_regular_branch() {
        let fixture = known_polytopes::simplex();
        let transition = build_transition_matrix_from_facet_intersections_and_omega(
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        );
        let (orbits, _) = solve_pruned_hk2017_candidates(&fixture.dual_vertices_f64, &transition)
            .expect("simplex candidate search succeeds");
        let sigma = &orbits[0].sigma;
        let displacement = fixture
            .dual_vertices_f64
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let scale = (index + 1) as f64;
                Vector4::new(0.03 * scale, -0.02 / scale, 0.01, -0.015)
            })
            .collect::<Vec<_>>();
        let directional =
            solve_raw_beta_directional(&fixture.dual_vertices_f64, sigma, &displacement)
                .expect("directional beta solve succeeds");
        let step = 1.0e-6;
        let plus = fixture
            .dual_vertices_f64
            .iter()
            .zip(&displacement)
            .map(|(base, direction)| base + step * direction)
            .collect::<Vec<_>>();
        let minus = fixture
            .dual_vertices_f64
            .iter()
            .zip(&displacement)
            .map(|(base, direction)| base - step * direction)
            .collect::<Vec<_>>();
        let plus_beta = solve_raw_sysext_kkt(&plus, sigma)
            .expect("positive finite-difference solve succeeds")
            .beta;
        let minus_beta = solve_raw_sysext_kkt(&minus, sigma)
            .expect("negative finite-difference solve succeeds")
            .beta;
        assert!(directional.differentiated_residual < 1.0e-10);
        assert!(directional.beta_directional.iter().sum::<f64>().abs() < 1.0e-10);
        for ((plus, minus), predicted) in plus_beta
            .iter()
            .zip(&minus_beta)
            .zip(&directional.beta_directional)
        {
            let finite_difference = (plus - minus) / (2.0 * step);
            assert!((finite_difference - predicted).abs() < 1.0e-7);
        }
    }
}
