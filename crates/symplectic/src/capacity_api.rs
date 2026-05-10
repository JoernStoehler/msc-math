//! Final capacity/orbit API surface under construction.
//!
//! This module holds the new dual-vertex based capacity API while the old
//! `Polytope4D`-anchored root wrappers are being migrated.

use crate::algorithms::orbit_search::solve_sigma_stream;
use crate::algorithms::{OrbitAdmissibility, OrbitGuaranteeMode, OrbitSearchError};
use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredicateVerdict {
    True,
    False,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct F64Interval {
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct F64Orbit {
    pub sigma: Vec<usize>,
    pub beta: Vec<f64>,
    pub q: f64,
    pub q_error_bound: f64,
    pub mu: [f64; 4],
    pub xi: f64,
    pub action: F64Interval,
    pub admissible: PredicateVerdict,
}

#[derive(Clone, Debug, PartialEq)]
pub struct F64CapacityResult {
    pub min_action: F64Interval,
    pub orbits: Vec<F64Orbit>,
}

impl F64CapacityResult {
    pub fn capacity(&self) -> Option<f64> {
        self.best_orbit().map(|orbit| orbit.action.upper)
    }

    pub fn best_orbit(&self) -> Option<&F64Orbit> {
        self.orbits
            .iter()
            .filter(|orbit| orbit.admissible == PredicateVerdict::True)
            .min_by(|left, right| {
                left.action
                    .upper
                    .total_cmp(&right.action.upper)
                    .then_with(|| left.action.lower.total_cmp(&right.action.lower))
                    .then_with(|| left.sigma.cmp(&right.sigma))
            })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CapacityError {
    Geometry(ConstructionError),
    OrbitSearch(OrbitSearchError),
}

impl From<ConstructionError> for CapacityError {
    fn from(err: ConstructionError) -> Self {
        Self::Geometry(err)
    }
}

impl From<OrbitSearchError> for CapacityError {
    fn from(err: OrbitSearchError) -> Self {
        Self::OrbitSearch(err)
    }
}

fn polytope_from_dual_vertices_f64(
    dual_vertices: &[[f64; 4]],
) -> Result<Polytope4D, CapacityError> {
    Polytope4D::from_f64(
        dual_vertices
            .iter()
            .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
            .collect(),
    )
    .map_err(CapacityError::Geometry)
}

fn verdict_from_admissibility(admissibility: OrbitAdmissibility) -> PredicateVerdict {
    match admissibility {
        OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => {
            PredicateVerdict::True
        }
        OrbitAdmissibility::IndeterminateF64 => PredicateVerdict::Indeterminate,
    }
}

fn f64_orbit_from_current_orbit(
    orbit: crate::algorithms::OrbitKktData,
) -> Result<F64Orbit, CapacityError> {
    Ok(F64Orbit {
        sigma: orbit.sigma,
        beta: orbit.beta,
        q: orbit.q,
        q_error_bound: orbit.q_error_bound,
        mu: orbit.mu.ok_or(CapacityError::OrbitSearch(
            OrbitSearchError::NumericalFailure,
        ))?,
        xi: orbit.xi.ok_or(CapacityError::OrbitSearch(
            OrbitSearchError::NumericalFailure,
        ))?,
        action: F64Interval {
            lower: orbit.action_lower,
            upper: orbit.action_upper,
        },
        admissible: verdict_from_admissibility(orbit.admissibility),
    })
}

fn f64_capacity_result_from_current_result(
    result: crate::algorithms::OrbitSearchResult,
) -> Result<F64CapacityResult, CapacityError> {
    let orbits = result
        .orbits
        .into_iter()
        .map(f64_orbit_from_current_orbit)
        .collect::<Result<Vec<_>, _>>()?;

    let min_action_lower = orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissible,
                PredicateVerdict::True | PredicateVerdict::Indeterminate
            )
        })
        .map(|orbit| orbit.action.lower)
        .min_by(|left, right| left.total_cmp(right))
        .ok_or(CapacityError::OrbitSearch(
            OrbitSearchError::NoAdmissibleOrbit,
        ))?;

    let min_action_upper = orbits
        .iter()
        .filter(|orbit| orbit.admissible == PredicateVerdict::True)
        .map(|orbit| orbit.action.upper)
        .min_by(|left, right| left.total_cmp(right))
        .ok_or(CapacityError::OrbitSearch(
            OrbitSearchError::NoAdmissibleOrbit,
        ))?;

    Ok(F64CapacityResult {
        min_action: F64Interval {
            lower: min_action_lower,
            upper: min_action_upper,
        },
        orbits,
    })
}

pub fn capacity_hk2017_unpruned_f64(
    dual_vertices: &[[f64; 4]],
    action_gap: f64,
) -> Result<F64CapacityResult, CapacityError> {
    if !action_gap.is_finite() || action_gap < 0.0 {
        return Err(CapacityError::OrbitSearch(OrbitSearchError::InvalidGap));
    }

    let polytope = polytope_from_dual_vertices_f64(dual_vertices)?;
    let (orbits, iterations) = solve_sigma_stream(
        &polytope,
        crate::algorithms::OrbitSolveBackend::SaddlePoint,
        |visit| {
            let facet_count = polytope.facet_count();
            crate::algorithms::hk2017::for_each_sigma_unpruned_facet_count(facet_count, visit)
        },
    )?;
    let result = crate::algorithms::orbit_search::aggregate_orbits(
        &polytope,
        orbits,
        iterations,
        action_gap,
        OrbitGuaranteeMode::MinimaSafe,
    )?;
    f64_capacity_result_from_current_result(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn orbit(sigma: &[usize], lower: f64, upper: f64, admissible: PredicateVerdict) -> F64Orbit {
        F64Orbit {
            sigma: sigma.to_vec(),
            beta: vec![1.0 / sigma.len() as f64; sigma.len()],
            q: 1.0,
            q_error_bound: 0.0,
            mu: [0.0; 4],
            xi: 0.0,
            action: F64Interval { lower, upper },
            admissible,
        }
    }

    #[test]
    fn result_capacity_uses_best_true_upper_bound() {
        let result = F64CapacityResult {
            min_action: F64Interval {
                lower: 0.9,
                upper: 1.4,
            },
            orbits: vec![
                orbit(&[0, 1], 0.9, 10.0, PredicateVerdict::Indeterminate),
                orbit(&[0, 2], 1.2, 1.4, PredicateVerdict::True),
                orbit(&[1, 2], 1.1, 1.6, PredicateVerdict::True),
            ],
        };

        let best = result.best_orbit().expect("true orbit");
        assert_eq!(best.sigma, vec![0, 2]);
        assert_eq!(result.capacity(), Some(1.4));
    }

    #[test]
    fn result_without_true_orbit_has_no_scalar_capacity() {
        let result = F64CapacityResult {
            min_action: F64Interval {
                lower: 0.9,
                upper: f64::INFINITY,
            },
            orbits: vec![orbit(&[0, 1], 0.9, 10.0, PredicateVerdict::Indeterminate)],
        };

        assert!(result.best_orbit().is_none());
        assert_eq!(result.capacity(), None);
    }
}
