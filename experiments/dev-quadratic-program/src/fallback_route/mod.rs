use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use symplectic::kkt::rational_solver::{solve_kkt_exact, ExactKktResult};
use symplectic::{
    CertifiedOrbitKktData, CertifiedOrbitSearchResult, CertifiedOrbitSetMode, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitKktData, OrbitSearchError, OrbitSearchResult,
};

pub use symplectic::exact::{solve_orbit_sigma_exact, ExactOrbitKktData};

/// Dev-owned fallback aggregation over f64-retained orbit candidates.
///
/// This is local to `experiments/dev-quadratic-program` so route semantics,
/// instrumentation, and consumer-shaped examples can change here before any
/// stable crate API changes. It certifies only the supplied candidate set; it
/// does not prove that f64 candidate generation retained every exact minimizer.
pub fn aggregate_orbits_with_local_exact_fallback(
    dual_vertices_exact: &[[BigRational; 4]],
    mut orbits: Vec<OrbitKktData>,
    iterations: u64,
    gap: f64,
    mode: OrbitGuaranteeMode,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    resolve_orbits_for_guarantee(dual_vertices_exact, &mut orbits, mode)?;
    trim_orbits_to_gap(&mut orbits, gap)?;
    if mode == OrbitGuaranteeMode::AllSafe {
        resolve_orbits_for_guarantee(dual_vertices_exact, &mut orbits, mode)?;
    }
    sort_orbits_by_lower_action(&mut orbits);
    summarize_orbits(orbits, iterations)
}

/// Dev-owned exact orbit-set aggregation over f64-retained candidates.
///
/// This returns exact rational capacity/minimizer data for the retained set.
/// It is not a substitute for exact-all-visited-sigma reference routes when the
/// candidate filter itself is under audit.
pub fn aggregate_certified_orbits_with_local_exact_fallback(
    dual_vertices_exact: &[[BigRational; 4]],
    mut candidates: Vec<OrbitKktData>,
    iterations: u64,
    action_gap_exact: BigRational,
    mode: CertifiedOrbitSetMode,
) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
    if candidates.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    if action_gap_exact.is_negative() {
        return Err(OrbitSearchError::InvalidGap);
    }

    sort_orbits_by_lower_action(&mut candidates);
    let mut certified: Vec<Option<CertifiedOrbitKktData>> = vec![None; candidates.len()];
    let mut rejected = vec![false; candidates.len()];
    let mut exact_resolutions = 0usize;

    let mut capacity_exact = None;
    for (idx, candidate) in candidates.iter().enumerate() {
        exact_resolutions += 1;
        match certified_orbit_from_sigma(dual_vertices_exact, &candidate.sigma) {
            Some(exact_orbit) => {
                capacity_exact = Some(exact_orbit.action_exact.clone());
                certified[idx] = Some(exact_orbit);
                break;
            }
            None => rejected[idx] = true,
        }
    }
    let mut capacity_exact = capacity_exact.ok_or(OrbitSearchError::NoAdmissibleOrbit)?;

    let resolution_gap = match mode {
        CertifiedOrbitSetMode::MinimizersOnly => BigRational::zero(),
        CertifiedOrbitSetMode::GapWindow => action_gap_exact.clone(),
    };

    loop {
        let threshold_exact = capacity_exact.clone() + resolution_gap.clone();
        let threshold_f64 = conservative_f64_upper_bound(&threshold_exact);
        let needs_resolution: Vec<usize> = candidates
            .iter()
            .enumerate()
            .filter_map(|(idx, candidate)| {
                (certified[idx].is_none()
                    && !rejected[idx]
                    && candidate.action_lower <= threshold_f64)
                    .then_some(idx)
            })
            .collect();

        if needs_resolution.is_empty() {
            break;
        }

        for idx in needs_resolution {
            exact_resolutions += 1;
            match certified_orbit_from_sigma(dual_vertices_exact, &candidates[idx].sigma) {
                Some(exact_orbit) => {
                    if exact_orbit.action_exact < capacity_exact {
                        capacity_exact = exact_orbit.action_exact.clone();
                    }
                    certified[idx] = Some(exact_orbit);
                }
                None => rejected[idx] = true,
            }
        }
    }

    let window_cutoff = capacity_exact.clone() + action_gap_exact.clone();
    let mut minimizers: Vec<CertifiedOrbitKktData> = certified
        .iter()
        .filter_map(|orbit| orbit.as_ref())
        .filter(|orbit| orbit.action_exact == capacity_exact)
        .cloned()
        .collect();
    if minimizers.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    sort_certified_orbits_by_sigma(&mut minimizers);

    let mut orbits: Vec<CertifiedOrbitKktData> = match mode {
        CertifiedOrbitSetMode::MinimizersOnly => minimizers.clone(),
        CertifiedOrbitSetMode::GapWindow => certified
            .into_iter()
            .flatten()
            .filter(|orbit| orbit.action_exact <= window_cutoff)
            .collect(),
    };
    sort_certified_orbits_by_action(&mut orbits);

    Ok(CertifiedOrbitSearchResult {
        capacity: rational_to_f64(&capacity_exact),
        capacity_exact,
        action_gap_exact,
        minimizers,
        orbits,
        iterations,
        exact_resolutions,
    })
}

fn exact_orbit_from_candidate(
    dual_vertices_exact: &[[BigRational; 4]],
    orbit: &OrbitKktData,
) -> Option<OrbitKktData> {
    let exact = solve_kkt_exact(dual_vertices_exact, &orbit.sigma)?;
    if !exact_kkt_result_satisfies_constraints(dual_vertices_exact, &orbit.sigma, &exact) {
        return None;
    }
    if !exact.q_exact.is_positive() {
        return None;
    }
    let beta: Vec<f64> = exact.beta.iter().map(rational_to_f64).collect();
    let beta_margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
    let action_exact = exact_action_from_q(&exact.q_exact);
    let action = rational_to_f64(&action_exact);

    Some(OrbitKktData {
        sigma: orbit.sigma.clone(),
        beta,
        beta_margin,
        action,
        action_lower: action,
        action_upper: action,
        q: rational_to_f64(&exact.q_exact),
        q_error_bound: 0.0,
        mu: orbit.mu,
        xi: orbit.xi,
        admissibility: OrbitAdmissibility::AdmissibleExact,
    })
}

fn certified_orbit_from_sigma(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
) -> Option<CertifiedOrbitKktData> {
    let exact = solve_kkt_exact(dual_vertices_exact, sigma)?;
    if !exact_kkt_result_satisfies_constraints(dual_vertices_exact, sigma, &exact) {
        return None;
    }
    let action_exact = exact_action_from_q(&exact.q_exact);
    let action = rational_to_f64(&action_exact);

    Some(CertifiedOrbitKktData {
        sigma: sigma.to_vec(),
        beta_exact: exact.beta,
        q_exact: exact.q_exact,
        action_exact,
        action,
    })
}

fn exact_kkt_result_satisfies_constraints(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
    exact: &ExactKktResult,
) -> bool {
    if exact.beta.len() != sigma.len()
        || !exact.beta.iter().all(|beta| beta.is_positive())
        || !exact.q_exact.is_positive()
    {
        return false;
    }

    let beta_sum = exact
        .beta
        .iter()
        .cloned()
        .fold(BigRational::zero(), |acc, beta| acc + beta);
    if beta_sum != BigRational::one() {
        return false;
    }

    (0..4).all(|d| {
        sigma
            .iter()
            .zip(exact.beta.iter())
            .map(|(&facet, beta)| beta * &dual_vertices_exact[facet][d])
            .fold(BigRational::zero(), |acc, entry| acc + entry)
            .is_zero()
    })
}

fn resolve_orbits_for_guarantee(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
    mode: OrbitGuaranteeMode,
) -> Result<(), OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    match mode {
        OrbitGuaranteeMode::BoundSafe => resolve_boundsafe(dual_vertices_exact, orbits),
        OrbitGuaranteeMode::MinimaSafe => resolve_minimasafe(dual_vertices_exact, orbits),
        OrbitGuaranteeMode::AllSafe => resolve_allsafe(dual_vertices_exact, orbits),
    }
}

fn resolve_boundsafe(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    loop {
        let lower_idx = argmin_action_lower(orbits).ok_or(OrbitSearchError::NoAdmissibleOrbit)?;
        let upper_idx = argmin_action_upper(orbits).ok_or(OrbitSearchError::NoAdmissibleOrbit)?;

        let mut needs_exact = Vec::new();
        if orbits[lower_idx].admissibility == OrbitAdmissibility::IndeterminateF64 {
            needs_exact.push(lower_idx);
        }
        if orbits[upper_idx].admissibility == OrbitAdmissibility::IndeterminateF64 {
            needs_exact.push(upper_idx);
        }

        if needs_exact.is_empty() {
            return Ok(());
        }

        resolve_indices_exact(dual_vertices_exact, orbits, needs_exact)?;
    }
}

fn resolve_minimasafe(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    loop {
        resolve_boundsafe(dual_vertices_exact, orbits)?;
        let lower = orbits
            .iter()
            .map(|orbit| orbit.action_lower)
            .fold(f64::INFINITY, f64::min);
        let upper = orbits
            .iter()
            .map(|orbit| orbit.action_upper)
            .fold(f64::INFINITY, f64::min);

        let needs_exact: Vec<usize> = orbits
            .iter()
            .enumerate()
            .filter_map(|(idx, orbit)| {
                let intersects_minimum_window =
                    orbit.action_lower <= upper && lower <= orbit.action_upper;
                (orbit.admissibility == OrbitAdmissibility::IndeterminateF64
                    && intersects_minimum_window)
                    .then_some(idx)
            })
            .collect();

        if needs_exact.is_empty() {
            return Ok(());
        }

        resolve_indices_exact(dual_vertices_exact, orbits, needs_exact)?;
    }
}

fn resolve_allsafe(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    let needs_exact: Vec<usize> = orbits
        .iter()
        .enumerate()
        .filter_map(|(idx, orbit)| {
            (orbit.admissibility == OrbitAdmissibility::IndeterminateF64).then_some(idx)
        })
        .collect();
    resolve_indices_exact(dual_vertices_exact, orbits, needs_exact)
}

fn resolve_indices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
    mut indices: Vec<usize>,
) -> Result<(), OrbitSearchError> {
    indices.sort_unstable();
    indices.dedup();

    for idx in indices.into_iter().rev() {
        match exact_orbit_from_candidate(dual_vertices_exact, &orbits[idx]) {
            Some(exact_orbit) => orbits[idx] = exact_orbit,
            None => {
                orbits.remove(idx);
            }
        }
    }

    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    Ok(())
}

fn argmin_action_lower(orbits: &[OrbitKktData]) -> Option<usize> {
    orbits
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.action_lower.total_cmp(&b.action_lower))
        .map(|(idx, _)| idx)
}

fn argmin_action_upper(orbits: &[OrbitKktData]) -> Option<usize> {
    orbits
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.action_upper.total_cmp(&b.action_upper))
        .map(|(idx, _)| idx)
}

fn sort_orbits_by_lower_action(orbits: &mut [OrbitKktData]) {
    orbits.sort_by(|a, b| {
        a.action_lower
            .total_cmp(&b.action_lower)
            .then_with(|| a.action_upper.total_cmp(&b.action_upper))
            .then_with(|| a.action.total_cmp(&b.action))
    });
}

fn trim_orbits_to_gap(orbits: &mut Vec<OrbitKktData>, gap: f64) -> Result<(), OrbitSearchError> {
    let min_action_upper = orbits
        .iter()
        .map(|orbit| orbit.action_upper)
        .fold(f64::INFINITY, f64::min);
    let cutoff = min_action_upper + gap;
    orbits.retain(|orbit| orbit.action_lower <= cutoff);
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    Ok(())
}

fn summarize_orbits(
    orbits: Vec<OrbitKktData>,
    iterations: u64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let min_action = orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| orbit.action)
        .min_by(|a, b| a.total_cmp(b))
        .ok_or(OrbitSearchError::NoAdmissibleOrbit)?;
    let min_action_lower = orbits
        .iter()
        .map(|orbit| orbit.action_lower)
        .fold(f64::INFINITY, f64::min);
    let min_action_upper = orbits
        .iter()
        .map(|orbit| orbit.action_upper)
        .fold(f64::INFINITY, f64::min);

    Ok(OrbitSearchResult {
        orbits,
        min_action,
        min_action_lower,
        min_action_upper,
        iterations,
    })
}

fn exact_action_from_q(q_exact: &BigRational) -> BigRational {
    BigRational::one() / (q_exact.clone() + q_exact.clone())
}

fn conservative_f64_upper_bound(exact: &BigRational) -> f64 {
    let mut value = rational_to_f64(exact);
    for _ in 0..8 {
        value = if value.is_finite() && value >= 0.0 {
            f64::from_bits(value.to_bits() + 1)
        } else {
            value
        };
    }
    value
}

fn sort_certified_orbits_by_action(orbits: &mut [CertifiedOrbitKktData]) {
    orbits.sort_by(|a, b| {
        a.action_exact
            .cmp(&b.action_exact)
            .then_with(|| a.sigma.cmp(&b.sigma))
    });
}

fn sort_certified_orbits_by_sigma(orbits: &mut [CertifiedOrbitKktData]) {
    orbits.sort_by(|a, b| a.sigma.cmp(&b.sigma));
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        exact_binary64_dual_vertex_arrays,
        exact_binary64_transition_matrix_assuming_origin_interior,
        generated_f64_cases_with_source_filter,
    };
    use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
    use symplectic::solve_orbit_sigma_saddle_point;

    #[test]
    fn local_boundsafe_resolves_indeterminate_candidate() {
        let (dual_vertices_exact, mut orbit) = small_exact_candidate();
        orbit.admissibility = OrbitAdmissibility::IndeterminateF64;
        orbit.action_lower = orbit.action * 0.5;
        orbit.action_upper = f64::INFINITY;

        let crate_result = symplectic::aggregate_orbits_with_dual_vertices_exact(
            &dual_vertices_exact,
            vec![orbit.clone()],
            1,
            0.0,
            OrbitGuaranteeMode::BoundSafe,
        )
        .expect("crate fallback should resolve the candidate");
        let result = aggregate_orbits_with_local_exact_fallback(
            &dual_vertices_exact,
            vec![orbit],
            1,
            0.0,
            OrbitGuaranteeMode::BoundSafe,
        )
        .expect("local fallback should resolve the candidate");

        assert_eq!(result.orbits.len(), 1);
        assert_eq!(
            result.orbits[0].admissibility,
            OrbitAdmissibility::AdmissibleExact
        );
        assert_eq!(result.orbits[0].action_lower, result.orbits[0].action_upper);
        assert_eq!(result, crate_result);
    }

    #[test]
    fn local_certified_aggregation_returns_exact_minimizer() {
        let (dual_vertices_exact, orbit) = small_exact_candidate();

        let crate_result = symplectic::aggregate_certified_orbits_with_dual_vertices_exact(
            &dual_vertices_exact,
            vec![orbit.clone()],
            1,
            BigRational::zero(),
            CertifiedOrbitSetMode::MinimizersOnly,
        )
        .expect("crate certified fallback should resolve exact minimizer");
        let result = aggregate_certified_orbits_with_local_exact_fallback(
            &dual_vertices_exact,
            vec![orbit],
            1,
            BigRational::zero(),
            CertifiedOrbitSetMode::MinimizersOnly,
        )
        .expect("local certified fallback should resolve exact minimizer");

        assert_eq!(result.minimizers.len(), 1);
        assert_eq!(result.orbits, result.minimizers);
        assert_eq!(result.exact_resolutions, 1);
        assert_eq!(result, crate_result);
    }

    fn small_exact_candidate() -> (Vec<[BigRational; 4]>, OrbitKktData) {
        let case = generated_f64_cases_with_source_filter(
            1,
            99540836,
            &["seed99540836:F5:sample0:attempt5000000008".to_string()],
        )
        .pop()
        .expect("known generated case");
        let dual_vertices_exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
        let transition =
            exact_binary64_transition_matrix_assuming_origin_interior(&dual_vertices_exact);
        let sigma = SimpleDirectedCyclesCanonical::new(&transition)
            .find(|sigma| solve_kkt_exact(&dual_vertices_exact, sigma).is_some())
            .expect("known case has an exact-admissible sigma");
        let orbit = solve_orbit_sigma_saddle_point(&case.dual_vertices, &sigma)
            .expect("same sigma should solve in f64");
        (dual_vertices_exact, orbit)
    }
}
