use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use symplectic::kkt::saddle_point_solver::{
    solve_kkt_for_dual_vertices, KktOutcome, KktResult, EPS_Q_POSITIVE,
};

const EPS_MARGIN_TRUE: f64 = 1e-9;
const EPS_MARGIN_FALSE: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum F64OrbitAdmissibility {
    Admissible,
    Indeterminate,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct F64OrbitKktData {
    pub(crate) sigma: Vec<usize>,
    pub(crate) beta: Vec<f64>,
    pub(crate) beta_margin: f64,
    pub(crate) action: f64,
    pub(crate) action_lower: f64,
    pub(crate) action_upper: f64,
    pub(crate) q: f64,
    pub(crate) q_error_bound: f64,
    pub(crate) mu: Option<[f64; 4]>,
    pub(crate) xi: Option<f64>,
    pub(crate) admissibility: F64OrbitAdmissibility,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum F64OrbitSolveError {
    Inadmissible,
    NumericalFailure,
}

enum MarginVerdict {
    True,
    False,
    Indeterminate,
}

pub(crate) fn solve_f64_orbit_sigma_saddle_point(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<F64OrbitKktData, F64OrbitSolveError> {
    match solve_kkt_for_dual_vertices(dual_vertices, sigma) {
        KktOutcome::Feasible(kkt) => orbit_from_saddle_point_result(sigma, kkt),
        KktOutcome::Infeasible => Err(F64OrbitSolveError::Inadmissible),
        KktOutcome::SingularMatrix
        | KktOutcome::TypeCViolation
        | KktOutcome::ConstraintViolation => Err(F64OrbitSolveError::NumericalFailure),
    }
}

fn orbit_from_saddle_point_result(
    sigma: &[usize],
    result: KktResult,
) -> Result<F64OrbitKktData, F64OrbitSolveError> {
    if result.q_corrected <= EPS_Q_POSITIVE {
        return Err(F64OrbitSolveError::Inadmissible);
    }

    let beta_margin = result.beta.iter().copied().fold(f64::INFINITY, f64::min);
    let admissibility = match classify_margin(beta_margin) {
        MarginVerdict::True => F64OrbitAdmissibility::Admissible,
        MarginVerdict::Indeterminate => F64OrbitAdmissibility::Indeterminate,
        MarginVerdict::False => return Err(F64OrbitSolveError::Inadmissible),
    };
    let (action_lower, action_upper) =
        action_bounds_from_q(result.q_corrected, result.q_error_bound);

    let mu: [f64; 4] = result
        .mu
        .as_slice()
        .try_into()
        .map_err(|_| F64OrbitSolveError::NumericalFailure)?;

    Ok(F64OrbitKktData {
        sigma: sigma.to_vec(),
        beta: result.beta,
        beta_margin,
        action: 0.5 / result.q_corrected,
        action_lower,
        action_upper,
        q: result.q_corrected,
        q_error_bound: result.q_error_bound,
        mu: Some(mu),
        xi: Some(result.xi),
        admissibility,
    })
}

fn classify_margin(margin: f64) -> MarginVerdict {
    if margin > EPS_MARGIN_TRUE {
        MarginVerdict::True
    } else if margin < -EPS_MARGIN_FALSE {
        MarginVerdict::False
    } else {
        MarginVerdict::Indeterminate
    }
}

fn action_bounds_from_q(q: f64, q_error_bound: f64) -> (f64, f64) {
    let q_upper = q + q_error_bound;
    let action_lower = 0.5 / q_upper;
    let q_lower = q - q_error_bound;
    let action_upper = if q_lower > EPS_Q_POSITIVE {
        0.5 / q_lower
    } else {
        f64::INFINITY
    };
    (action_lower, action_upper)
}
