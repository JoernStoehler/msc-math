//! Product closure-vertex route: behavioral, exact, numerical, and timing audit.

#![recursion_limit = "256"]

use exp_dev_quadratic_program::{
    audit_product_closure_capacity_binary64, capacity_f64_only_with_policy_and_method_profiled,
    exact_binary64_dual_vertex_arrays, generated_f64_cases,
    solve_exact_capacity_for_transition_pruned_sigmas, solve_product_closure_capacity_hybrid,
    try_exact_binary64_transition_matrix_assuming_origin_interior, F64CapacityMethod,
    F64CapacityOutcome, F64ValidationPolicy,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::Zero;
use serde_json::json;
use std::time::Instant;
use symplectic::algorithms::capacity_4d::{
    check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
    check_primal_vertex_norm_bounds, exact_binary64_polytope_geometry, product_capacity,
    product_qp_minimizers, PolytopeGeometry4d,
};
use symplectic::geom::known_polytopes;

const DEFAULT_SEED: u64 = 99_599_604;

fn checked_geometry(dual_vertices: &[Vector4<f64>]) -> Result<PolytopeGeometry4d, String> {
    check_facet_count(dual_vertices.len())
        .map_err(|error| format!("production facet-count check failed: {error:?}"))?;
    check_finite_dual_vertices(dual_vertices)
        .map_err(|error| format!("production finite-coordinate check failed: {error:?}"))?;
    check_dual_vertex_norm_bounds(dual_vertices)
        .map_err(|error| format!("production dual-norm check failed: {error:?}"))?;
    let geometry = exact_binary64_polytope_geometry(dual_vertices)
        .map_err(|error| format!("production exact geometry failed: {error:?}"))?;
    check_primal_vertex_norm_bounds(&geometry)
        .map_err(|error| format!("production primal-norm check failed: {error:?}"))?;
    Ok(geometry)
}

fn main() {
    let samples = argument_usize("--samples=").unwrap_or(1);
    let repeats = argument_usize("--timing-repeats=").unwrap_or(5);
    let seed = argument_u64("--seed=").unwrap_or(DEFAULT_SEED);
    let include_generated = !std::env::args().any(|argument| argument == "--known-only");

    let mut cases = vec![
        known_case(known_polytopes::lagrangian_triangle_product()),
        known_case(known_polytopes::lagrangian_triangle_square()),
        known_case(known_polytopes::hko_pentagon()),
        square_product_case(),
        near_boundary_closure_case(),
        scale_extremes_case(),
        regular_product_case(7, 7),
        regular_product_case(8, 8),
    ];
    if include_generated {
        cases.extend(
            generated_f64_cases(samples, seed)
                .into_iter()
                .filter(|case| case.family == "generated_product_f64")
                .map(|case| AuditCase {
                    source_id: case.source_id,
                    family: case.family,
                    dual_vertices: case.dual_vertices,
                    capacity_label: None,
                }),
        );
    }

    for case in cases {
        match audit_case(&case, repeats) {
            Ok(row) => println!("{}", serde_json::to_string(&row).expect("serialize row")),
            Err(error) => println!(
                "{}",
                serde_json::to_string(&json!({
                    "source_id": case.source_id,
                    "family": case.family,
                    "status": "error",
                    "error": error,
                }))
                .expect("serialize error row")
            ),
        }
    }
}

#[derive(Clone)]
struct AuditCase {
    source_id: String,
    family: String,
    dual_vertices: Vec<Vector4<f64>>,
    capacity_label: Option<f64>,
}

fn known_case(fixture: &symplectic::geom::known_polytopes::KnownPolytope) -> AuditCase {
    AuditCase {
        source_id: fixture.name.to_string(),
        family: "known_product".to_string(),
        dual_vertices: fixture.dual_vertices_f64.clone(),
        capacity_label: Some(fixture.capacity),
    }
}

fn audit_case(case: &AuditCase, repeats: usize) -> Result<serde_json::Value, String> {
    let audit = audit_product_closure_capacity_binary64(&case.dual_vertices)
        .map_err(|error| format!("closure audit failed: {error:?}"))?;
    let geometry = checked_geometry(&case.dual_vertices)?;
    let production = product_capacity(&geometry)
        .map_err(|error| format!("production product route failed: {error:?}"))?;
    let production_minimizers = product_qp_minimizers(&geometry)
        .map_err(|error| format!("production product minimizers failed: {error:?}"))?;
    let production_capacity_exact_agrees =
        *production.capacity_exact() == audit.hybrid.capacity_exact;
    let production_winner_sigmas_agree = production_minimizers
        .candidates()
        .iter()
        .map(|winner| winner.sigma().to_vec())
        .collect::<Vec<_>>()
        == audit
            .hybrid
            .winners
            .iter()
            .map(|winner| winner.sigma.clone())
            .collect::<Vec<_>>();

    let mut hybrid_times = Vec::with_capacity(repeats);
    for _ in 0..repeats {
        let started = Instant::now();
        let repeated = solve_product_closure_capacity_hybrid(&case.dual_vertices)
            .map_err(|error| format!("hybrid repeat failed: {error:?}"))?;
        if repeated.capacity_exact != audit.hybrid.capacity_exact {
            return Err("hybrid repeat changed exact capacity".to_string());
        }
        hybrid_times.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    hybrid_times.sort_by(f64::total_cmp);
    let hybrid_median_ms = hybrid_times[hybrid_times.len() / 2];

    let old = (case.dual_vertices.len() <= 12).then(|| {
        let old_started = Instant::now();
        let (report, timing) = capacity_f64_only_with_policy_and_method_profiled(
            &case.dual_vertices,
            F64ValidationPolicy::LpOriginVertex,
            F64CapacityMethod::ProductBilliardOrHk,
        );
        let total_ms = old_started.elapsed().as_secs_f64() * 1000.0;
        let capacity = match &report.outcome {
            F64CapacityOutcome::Success { capacity, .. } => Some(*capacity),
            F64CapacityOutcome::Failure { .. } => None,
        };
        (report, timing, total_ms, capacity)
    });

    let exact_vertices = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
    let exact_transition =
        try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices)
            .map_err(|error| format!("exact transition failed: {error}"))?;
    let winners_transition_valid = audit.hybrid.winners.iter().all(|winner| {
        winner
            .sigma
            .iter()
            .zip(winner.sigma.iter().cycle().skip(1))
            .all(|(&left, &right)| exact_transition[(left, right)])
    });

    let general_exact = if case.dual_vertices.len() <= 7 {
        let started = Instant::now();
        let report = solve_exact_capacity_for_transition_pruned_sigmas(
            &exact_vertices,
            &exact_transition,
            BigRational::zero(),
        )
        .map_err(|error| format!("general exact route failed: {error:?}"))?;
        Some((
            report.capacity_exact == audit.hybrid.capacity_exact,
            started.elapsed().as_secs_f64() * 1000.0,
            report.iterations,
            report.exact_admissible_count,
        ))
    } else {
        None
    };

    let allowed_patterns = ["QPQP", "QQPQP", "QPPQP", "QQPPQP", "QQPQPP", "QPQPQP"];
    let winner_patterns_allowed = audit.hybrid.winner_type_patterns.iter().all(|pattern| {
        allowed_patterns
            .iter()
            .any(|allowed| cyclically_equivalent(pattern, allowed))
    });

    let capacity_label_relative_error = case
        .capacity_label
        .map(|label| (audit.hybrid.capacity - label).abs() / label.abs());
    let old_relative_difference = old
        .as_ref()
        .and_then(|value| value.3)
        .map(|capacity| (capacity - audit.hybrid.capacity).abs() / audit.hybrid.capacity.abs());

    Ok(json!({
        "source_id": case.source_id,
        "family": case.family,
        "status": "ok",
        "facet_count": case.dual_vertices.len(),
        "capacity": audit.hybrid.capacity,
        "capacity_exact": audit.hybrid.capacity_exact.to_string(),
        "q_max_exact": audit.hybrid.q_max_exact.to_string(),
        "capacity_label": case.capacity_label,
        "capacity_label_relative_error": capacity_label_relative_error,
        "capacity_exact_agrees": audit.capacity_exact_agrees,
        "winner_value_agrees": audit.winner_value_agrees,
        "production_capacity_exact_agrees": production_capacity_exact_agrees,
        "production_winner_sigmas_agree": production_winner_sigmas_agree,
        "winner_count": audit.hybrid.winners.len(),
        "winner_type_patterns": audit.hybrid.winner_type_patterns,
        "winner_patterns_allowed": winner_patterns_allowed,
        "winners_transition_valid": winners_transition_valid,
        "q_closure_vertices": audit.hybrid.stats.q_closure_vertices,
        "p_closure_vertices": audit.hybrid.stats.p_closure_vertices,
        "support_pairs_tested": audit.hybrid.stats.support_pairs_tested,
        "support_triples_tested": audit.hybrid.stats.support_triples_tested,
        "interval_certified_vertices": audit.hybrid.stats.interval_certified_vertices,
        "interval_certified_rejections": audit.hybrid.stats.interval_certified_rejections,
        "support_exact_fallbacks": audit.hybrid.stats.support_exact_fallbacks,
        "support_fallback_vertices": audit.hybrid.stats.support_fallback_vertices,
        "support_fallback_rejections": audit.hybrid.stats.support_fallback_rejections,
        "cyclic_orders_evaluated": audit.hybrid.stats.cyclic_orders_evaluated,
        "exact_winner_contenders": audit.hybrid.stats.exact_winner_contenders,
        "gradual_underflow_available": audit.hybrid.stats.gradual_underflow_available,
        "full_exact_fallback": audit.hybrid.stats.full_exact_fallback,
        "hybrid_median_ms": hybrid_median_ms,
        "hybrid_internal_total_ms": audit.hybrid.stats.total_ms,
        "hybrid_closure_enumeration_ms": audit.hybrid.stats.closure_enumeration_ms,
        "hybrid_objective_enumeration_ms": audit.hybrid.stats.objective_enumeration_ms,
        "hybrid_exact_winner_resolution_ms": audit.hybrid.stats.exact_winner_resolution_ms,
        "exact_all_candidates_ms": audit.exact_reference_ms,
        "old_product_total_ms": old.as_ref().map(|value| value.2),
        "old_product_candidate_ms": old.as_ref().map(|value| value.1.candidate_solve_ms),
        "old_product_kkt_ms": old.as_ref().map(|value| value.1.candidate_kkt_solve_ms),
        "old_product_sigma_count": old.as_ref().map(|value| value.0.sigma_count),
        "old_product_capacity": old.as_ref().and_then(|value| value.3),
        "old_product_relative_difference": old_relative_difference,
        "general_exact_agrees": general_exact.map(|value| value.0),
        "general_exact_ms": general_exact.map(|value| value.1),
        "general_exact_sigma_count": general_exact.map(|value| value.2),
        "general_exact_admissible_count": general_exact.map(|value| value.3),
        "compared_weights": audit.numerics.compared_weights,
        "min_positive_exact_weight": audit.numerics.min_positive_exact_weight,
        "max_weight_abs_error": audit.numerics.max_weight_abs_error,
        "max_weight_rel_error": audit.numerics.max_weight_rel_error,
        "max_weight_interval_width": audit.numerics.max_weight_interval_width,
        "max_closure_residual_inf": audit.numerics.max_closure_residual_inf,
        "weight_interval_violations": audit.numerics.weight_interval_violations,
        "compared_objectives": audit.numerics.compared_objectives,
        "min_nonzero_abs_exact_q": audit.numerics.min_nonzero_abs_exact_q,
        "max_q_abs_error": audit.numerics.max_q_abs_error,
        "max_q_rel_error": audit.numerics.max_q_rel_error,
        "max_q_error_over_qmax": audit.numerics.max_q_abs_error / (0.5 / audit.hybrid.capacity),
        "max_q_interval_width": audit.numerics.max_q_interval_width,
        "q_interval_violations": audit.numerics.q_interval_violations,
        "raw_q_sign_mismatches": audit.numerics.raw_q_sign_mismatches,
        "ternary_q_sign_mismatches": audit.numerics.q_sign_mismatches,
        "candidate_type_pattern_counts": audit.candidate_type_pattern_counts,
    }))
}

fn cyclically_equivalent(left: &str, right: &str) -> bool {
    left.len() == right.len() && format!("{right}{right}").contains(left)
}

fn square_product_case() -> AuditCase {
    AuditCase {
        source_id: "adversarial:square_product_exact_zeros".to_string(),
        family: "adversarial_product".to_string(),
        dual_vertices: vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ],
        capacity_label: None,
    }
}

fn near_boundary_closure_case() -> AuditCase {
    let eps = 2.0f64.powi(-9);
    let mut vertices = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(-1.0, eps, 0.0, 0.0),
        Vector4::new(-1.0, -eps, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, -1.0, 0.0, 0.0),
    ];
    vertices.extend(regular_triangle_factor(false, 1.0));
    AuditCase {
        source_id: "adversarial:near_boundary_closure".to_string(),
        family: "adversarial_product".to_string(),
        dual_vertices: vertices,
        capacity_label: None,
    }
}

fn scale_extremes_case() -> AuditCase {
    // Keep both dual and recovered primal coordinates comfortably inside the
    // shared 1e-3..=1e3 relevance range while retaining a large factor ratio.
    let mut vertices = regular_triangle_factor(true, 256.0);
    vertices.extend(regular_triangle_factor(false, 1.0 / 256.0));
    AuditCase {
        source_id: "adversarial:factor_scale_extremes".to_string(),
        family: "adversarial_product".to_string(),
        dual_vertices: vertices,
        capacity_label: None,
    }
}

fn regular_product_case(q_facets: usize, p_facets: usize) -> AuditCase {
    let mut vertices = regular_factor(q_facets, true, 1.0);
    vertices.extend(regular_factor(p_facets, false, 1.0));
    AuditCase {
        source_id: format!("scaling:regular_q{q_facets}_p{p_facets}"),
        family: "scaling_product".to_string(),
        dual_vertices: vertices,
        capacity_label: None,
    }
}

fn regular_triangle_factor(q_factor: bool, scale: f64) -> Vec<Vector4<f64>> {
    regular_factor(3, q_factor, scale)
}

fn regular_factor(count: usize, q_factor: bool, scale: f64) -> Vec<Vector4<f64>> {
    (0..count)
        .map(|index| {
            let angle = std::f64::consts::FRAC_PI_2
                + 2.0 * std::f64::consts::PI * index as f64 / count as f64;
            if q_factor {
                Vector4::new(scale * angle.cos(), scale * angle.sin(), 0.0, 0.0)
            } else {
                Vector4::new(0.0, 0.0, scale * angle.cos(), scale * angle.sin())
            }
        })
        .collect()
}

fn argument_usize(prefix: &str) -> Option<usize> {
    std::env::args().find_map(|argument| {
        argument
            .strip_prefix(prefix)
            .and_then(|value| value.parse().ok())
    })
}

fn argument_u64(prefix: &str) -> Option<u64> {
    std::env::args().find_map(|argument| {
        argument
            .strip_prefix(prefix)
            .and_then(|value| value.parse().ok())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use exp_dev_quadratic_program::validate_f64_polytope_input;

    #[test]
    fn scale_stress_remains_inside_shared_validation_contract() {
        let case = scale_extremes_case();
        let validation = validate_f64_polytope_input(&case.dual_vertices);
        assert!(
            validation.status.capacity_may_run(),
            "{:?}: {:?}",
            validation.status,
            validation.reasons
        );
    }
}
