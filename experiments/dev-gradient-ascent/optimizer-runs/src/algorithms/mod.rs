mod cma_es;
mod direct_search;
mod gap_model;
mod iid_source;
mod literal_gradient;
pub mod nonlinear_candidate_cma;
mod online_source;
mod safeguarded_gradient;

use crate::algorithm::Optimizer;
use crate::evaluator::{Evaluation, EvaluatorConfig};
use crate::manifest::AlgorithmSpec;
use crate::schema::SourcePoint;

pub fn construct(
    spec: &AlgorithmSpec,
    seed: u64,
    initial: &Evaluation,
    source_pool: &[SourcePoint],
    evaluator_config: &EvaluatorConfig,
) -> Result<Box<dyn Optimizer + Send>, String> {
    match spec {
        AlgorithmSpec::OnlineSource {
            batch_size,
            facet_count,
            height_min,
            height_max,
            ..
        } => {
            let actual_facet_count = initial.duals.len();
            if let Some(configured) = facet_count {
                if *configured != actual_facet_count {
                    return Err(format!(
                        "online source configured for F{configured} on an F{actual_facet_count} start"
                    ));
                }
            }
            Ok(Box::new(online_source::OnlineSource::new(
                seed,
                *batch_size,
                actual_facet_count,
                *height_min,
                *height_max,
                initial,
            )?))
        }
        AlgorithmSpec::IidSource { batch_size, .. } => Ok(Box::new(iid_source::IidSource::new(
            seed,
            *batch_size,
            initial,
            source_pool,
        )?)),
        AlgorithmSpec::DirectSearch {
            initial_radius,
            expansion,
            contraction,
            minimum_radius,
            ..
        } => Ok(Box::new(direct_search::DirectSearch::new(
            initial,
            *initial_radius,
            *expansion,
            *contraction,
            *minimum_radius,
        )?)),
        AlgorithmSpec::CmaEs {
            initial_sigma,
            population_size,
            minimum_sigma,
            maximum_sigma,
            scale_mode,
            ..
        } => Ok(Box::new(cma_es::CmaEs::new(
            seed,
            initial,
            *initial_sigma,
            *population_size,
            *minimum_sigma,
            *maximum_sigma,
            *scale_mode,
        )?)),
        AlgorithmSpec::LiteralGradient { rate, .. } => Ok(Box::new(
            literal_gradient::LiteralGradient::new(initial, *rate)?,
        )),
        AlgorithmSpec::SafeguardedGradient {
            schedule,
            slice_mode,
            ..
        } => Ok(Box::new(safeguarded_gradient::SafeguardedGradient::new(
            initial,
            schedule.clone(),
            *slice_mode,
        )?)),
        AlgorithmSpec::GapModel {
            candidate_window_relative,
            extension_mode,
            extension_reachability_scale,
            schedule,
            slice_mode,
            norm_mode,
            require_positive_prediction,
            ..
        } => Ok(Box::new(gap_model::GapModel::new(
            initial,
            *candidate_window_relative,
            *extension_mode,
            *extension_reachability_scale,
            schedule.clone(),
            *slice_mode,
            *norm_mode,
            *require_positive_prediction,
        )?)),
        AlgorithmSpec::NonlinearCandidateCma {
            candidate_window_relative,
            inner_generations,
            population_size,
            initial_sigma,
            minimum_sigma,
            maximum_sigma,
            ..
        } => Ok(Box::new(
            nonlinear_candidate_cma::NonlinearCandidateCma::new(
                initial,
                *candidate_window_relative,
                *inner_generations,
                *population_size,
                *initial_sigma,
                *minimum_sigma,
                *maximum_sigma,
                seed,
                evaluator_config.clone(),
            )?,
        )),
        AlgorithmSpec::NonlinearCandidateRelinearized {
            candidate_window_relative,
            beta_allowance,
            history_depth,
            acceptance,
            directional_transition,
            remember_validated_winner,
            inner_steps,
            initial_distance,
            expansion,
            contraction,
            minimum_distance,
            ..
        } => Ok(Box::new(
            nonlinear_candidate_cma::NonlinearCandidateRelinearized::new(
                initial,
                *candidate_window_relative,
                *beta_allowance,
                *history_depth,
                acceptance.clone(),
                directional_transition.clone(),
                *remember_validated_winner,
                *inner_steps,
                *initial_distance,
                *expansion,
                *contraction,
                *minimum_distance,
                evaluator_config.clone(),
            )?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::cma_es::CmaEs;
    use super::direct_search::DirectSearch;
    use crate::algorithm::{EvaluatedProposal, Optimizer};
    use crate::evaluator::Evaluation;
    use crate::quotient::flatten;
    use crate::schema::EvaluationRow;
    use nalgebra::Vector4;

    fn generic_f6() -> Vec<Vector4<f64>> {
        vec![
            Vector4::new(0.8, 0.1, 0.2, -0.3),
            Vector4::new(-0.4, 0.9, -0.1, 0.2),
            Vector4::new(0.1, -0.5, 0.8, 0.3),
            Vector4::new(-0.2, -0.3, -0.4, 0.9),
            Vector4::new(0.3, 0.2, -0.8, -0.5),
            Vector4::new(-0.7, -0.4, 0.2, -0.1),
        ]
    }

    fn evaluation(id: &str, sys: f64, duals: Vec<Vector4<f64>>) -> Evaluation {
        Evaluation {
            row: EvaluationRow {
                schema_version: 1,
                run_id: "test".to_string(),
                evaluation_id: id.to_string(),
                proposal_id: None,
                role: "test".to_string(),
                logical_call: 0,
                charged: false,
                point_key: id.to_string(),
                cache_status: "miss".to_string(),
                status: "ok".to_string(),
                geometry_route: "f64".to_string(),
                fallback_reason: None,
                usable_by_optimizer: true,
                error: None,
                facet_count: duals.len(),
                dual_flat: flatten(&duals),
                sys: Some(sys),
                capacity: Some(1.0),
                volume: Some(1.0),
                winning_sigma: Some(vec![0, 1]),
                winning_beta_margin: Some(1.0),
                orbit_count: Some(1),
                sigma_iterations: Some(1),
                geometry_indeterminate_count: 0,
                vertex_indeterminate_count: 0,
                bounded_near_singular_vertex_count: 0,
                ambiguous_vertex_incidence_count: 0,
                facet_intersection_indeterminate_count: 0,
                omega_indeterminate_count: 0,
                geometry_ms: 0.0,
                volume_ms: 0.0,
                capacity_ms: 0.0,
                total_ms: 0.0,
            },
            duals,
            physical_evaluation: true,
            context: None,
        }
    }

    #[test]
    fn direct_search_supports_incomplete_poll_and_accepts_improvement() {
        let initial = evaluation("initial", 1.0, generic_f6());
        let mut optimizer = DirectSearch::new(&initial, 0.03, 2.0, 0.5, 1e-6).unwrap();
        let proposals = optimizer.ask(3).unwrap();
        assert_eq!(proposals.len(), 3);
        let observations = proposals
            .into_iter()
            .enumerate()
            .map(|(index, proposal)| EvaluatedProposal {
                proposal_id: format!("p{index}"),
                evaluation: evaluation(
                    &format!("e{index}"),
                    if index == 1 { 1.1 } else { 0.9 },
                    proposal.duals,
                ),
            })
            .collect::<Vec<_>>();
        let outcome = optimizer.tell(&observations).unwrap();
        assert_eq!(outcome.selected, vec![(1, 1.0)]);
        assert_eq!(outcome.fields["accepted"], true);
        assert_eq!(outcome.fields["complete_poll"], false);
        assert_eq!(outcome.fields["radius_after"], 0.06);
    }

    #[test]
    fn cma_seed_reproduces_population_and_handles_partial_generation() {
        let initial = evaluation("initial", 1.0, generic_f6());
        let mut left = CmaEs::new(
            42,
            &initial,
            0.03,
            8,
            1e-6,
            1.0,
            crate::manifest::CmaScaleMode::PerCoordinate,
        )
        .unwrap();
        let mut right = CmaEs::new(
            42,
            &initial,
            0.03,
            8,
            1e-6,
            1.0,
            crate::manifest::CmaScaleMode::PerCoordinate,
        )
        .unwrap();
        let left_proposals = left.ask(5).unwrap();
        let right_proposals = right.ask(5).unwrap();
        assert_eq!(left_proposals.len(), 5);
        assert_eq!(right_proposals.len(), 5);
        for (left, right) in left_proposals.iter().zip(&right_proposals) {
            assert_eq!(flatten(&left.duals), flatten(&right.duals));
        }
        let partial = left_proposals
            .into_iter()
            .take(3)
            .enumerate()
            .map(|(index, proposal)| EvaluatedProposal {
                proposal_id: format!("partial-{index}"),
                evaluation: evaluation(
                    &format!("partial-e{index}"),
                    1.0 + index as f64 / 10.0,
                    proposal.duals,
                ),
            })
            .collect::<Vec<_>>();
        left.tell(&partial)
            .expect("CMA accepts an evaluated population prefix");
    }
}
