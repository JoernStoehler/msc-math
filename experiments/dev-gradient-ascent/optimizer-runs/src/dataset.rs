use crate::schema::{EvaluationRow, ProposalRow, RoundRow, RunRow};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Dataset {
    pub runs: Vec<RunRow>,
    pub rounds: Vec<RoundRow>,
    pub proposals: Vec<ProposalRow>,
    pub evaluations: Vec<EvaluationRow>,
}

impl Dataset {
    pub fn load(directory: &Path) -> Result<Self, String> {
        let dataset = Self {
            runs: load_jsonl(&directory.join("runs.jsonl"))?,
            rounds: load_jsonl(&directory.join("rounds.jsonl"))?,
            proposals: load_jsonl(&directory.join("proposals.jsonl"))?,
            evaluations: load_jsonl(&directory.join("evaluations.jsonl"))?,
        };
        dataset.validate_links()?;
        Ok(dataset)
    }

    pub fn evaluations_by_id(&self) -> HashMap<&str, &EvaluationRow> {
        self.evaluations
            .iter()
            .map(|row| (row.evaluation_id.as_str(), row))
            .collect()
    }

    pub fn best_at_or_before(&self, run_id: &str, logical_call: usize) -> Option<&EvaluationRow> {
        self.evaluations
            .iter()
            .filter(|row| {
                row.run_id == run_id
                    && row.logical_call <= logical_call
                    && row.usable_by_optimizer
                    && row.sys.is_some()
            })
            .max_by(|left, right| {
                left.sys
                    .expect("filtered")
                    .total_cmp(&right.sys.expect("filtered"))
                    .then_with(|| right.logical_call.cmp(&left.logical_call))
                    .then_with(|| right.evaluation_id.cmp(&left.evaluation_id))
            })
    }

    fn validate_links(&self) -> Result<(), String> {
        let evaluations = self.evaluations_by_id();
        let runs = self
            .runs
            .iter()
            .map(|row| row.run_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        for run in &self.runs {
            if !evaluations.contains_key(run.initial_evaluation_id.as_str())
                || !evaluations.contains_key(run.best_evaluation_id.as_str())
            {
                return Err(format!("run {} references absent evaluations", run.run_id));
            }
        }
        for evaluation in &self.evaluations {
            if !runs.contains(evaluation.run_id.as_str()) {
                return Err(format!(
                    "evaluation {} references absent run {}",
                    evaluation.evaluation_id, evaluation.run_id
                ));
            }
        }
        for proposal in &self.proposals {
            if !evaluations.contains_key(proposal.evaluation_id.as_str()) {
                return Err(format!(
                    "proposal {} references absent evaluation {}",
                    proposal.proposal_id, proposal.evaluation_id
                ));
            }
        }
        Ok(())
    }
}

fn load_jsonl<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>, String> {
    let file = File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    BufReader::new(file)
        .lines()
        .enumerate()
        .filter_map(|(index, line)| match line {
            Ok(line) if line.trim().is_empty() => None,
            result => Some((index, result)),
        })
        .map(|(index, line)| {
            let line = line
                .map_err(|error| format!("read {} line {}: {error}", path.display(), index + 1))?;
            serde_json::from_str(&line)
                .map_err(|error| format!("parse {} line {}: {error}", path.display(), index + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::AlgorithmStateRow;
    use std::fs;

    fn round_value() -> serde_json::Value {
        serde_json::json!({
            "schema_version": 1,
            "run_id": "run",
            "round_id": "run--r00000",
            "round_index": 0,
            "charged_calls_before": 0,
            "charged_calls_after": 1,
            "charged_compute_ms_before": 0.0,
            "charged_compute_ms_after": 1.0,
            "best_evaluation_id_before": "run--e000000",
            "best_evaluation_id_after": "run--e000001",
            "best_sys_before": 0.1,
            "best_sys_after": 0.2,
            "algorithm_state_before": {
                "kind": "evaluated_point",
                "evaluation_id": "run--e000000"
            },
            "algorithm_state_after": {
                "kind": "evaluated_population",
                "evaluation_ids": ["run--e000001"]
            },
            "geometric_reference_kind": null,
            "geometric_reference_dual_flat": null,
            "ask_ms": 0.1,
            "tell_ms": 0.2,
            "proposal_ids": [],
            "selected": [],
            "stop_reason": null,
            "algorithm_fields": {}
        })
    }

    fn load_round(value: serde_json::Value) -> Result<Vec<RoundRow>, String> {
        let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = directory.path().join("rounds.jsonl");
        fs::write(
            path,
            serde_json::to_string(&value).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        load_jsonl(&directory.path().join("rounds.jsonl"))
    }

    #[test]
    fn current_round_preserves_recorded_algorithm_states() {
        let rows = load_round(round_value()).expect("current round should load");
        assert_eq!(
            rows[0].algorithm_state_before,
            AlgorithmStateRow::EvaluatedPoint {
                evaluation_id: "run--e000000".to_string()
            }
        );
        assert_eq!(
            rows[0].algorithm_state_after,
            AlgorithmStateRow::EvaluatedPopulation {
                evaluation_ids: vec!["run--e000001".to_string()]
            }
        );
    }

    #[test]
    fn present_invalid_algorithm_state_remains_rejected() {
        let mut value = round_value();
        value["algorithm_state_before"] = serde_json::json!({"kind": "unknown"});

        assert!(load_round(value).is_err());
    }

    #[test]
    fn best_at_checkpoint_uses_only_reached_calls() {
        let directory = tempfile::tempdir().unwrap();
        fs::write(
            directory.path().join("runs.jsonl"),
            r#"{"schema_version":1,"run_id":"r","start_id":"s","algorithm_id":"a","algorithm_kind":"k","seed":1,"budget":2,"charge_initial":false,"initial_evaluation_id":"e0","initial_sys":1.0,"best_evaluation_id":"e2","best_sys":3.0,"final_algorithm_state":{"kind":"evaluated_point","evaluation_id":"e2"},"charged_calls":2,"physical_evaluations":3,"invalid_evaluations":0,"indeterminate_evaluations":0,"exact_fallback_evaluations":0,"rounds":2,"stop_reason":"budget_exhausted","wall_ms":1.0}
"#,
        )
        .unwrap();
        fs::write(directory.path().join("rounds.jsonl"), "").unwrap();
        fs::write(directory.path().join("proposals.jsonl"), "").unwrap();
        let row = |id: &str, call: usize, sys: f64| {
            format!(
                "{{\"schema_version\":1,\"run_id\":\"r\",\"evaluation_id\":\"{id}\",\"proposal_id\":null,\"role\":\"x\",\"logical_call\":{call},\"charged\":true,\"point_key\":\"{id}\",\"cache_status\":\"miss\",\"status\":\"ok\",\"geometry_route\":\"f64\",\"fallback_reason\":null,\"usable_by_optimizer\":true,\"error\":null,\"facet_count\":6,\"dual_flat\":[],\"sys\":{sys},\"capacity\":1.0,\"volume\":1.0,\"winning_sigma\":[0],\"winning_beta_margin\":1.0,\"orbit_count\":1,\"sigma_iterations\":1,\"geometry_indeterminate_count\":0,\"vertex_indeterminate_count\":0,\"bounded_near_singular_vertex_count\":0,\"ambiguous_vertex_incidence_count\":0,\"facet_intersection_indeterminate_count\":0,\"omega_indeterminate_count\":0,\"geometry_ms\":0.0,\"volume_ms\":0.0,\"capacity_ms\":0.0,\"total_ms\":0.0}}\n"
            )
        };
        fs::write(
            directory.path().join("evaluations.jsonl"),
            row("e0", 0, 1.0) + &row("e1", 1, 2.0) + &row("e2", 2, 3.0),
        )
        .unwrap();
        let dataset = Dataset::load(directory.path()).unwrap();
        assert_eq!(
            dataset.best_at_or_before("r", 1).unwrap().evaluation_id,
            "e1"
        );
    }

    #[test]
    fn present_valid_run_state_is_preserved() {
        let value = serde_json::json!({
            "schema_version": 1, "run_id": "r", "start_id": "s",
            "algorithm_id": "a", "algorithm_kind": "k", "seed": 1,
            "budget": 2, "charge_initial": false,
            "initial_evaluation_id": "e0", "initial_sys": 1.0,
            "best_evaluation_id": "e0", "best_sys": 1.0,
            "final_algorithm_state": {"kind":"evaluated_point", "evaluation_id":"e0"},
            "charged_calls": 0, "physical_evaluations": 1,
            "invalid_evaluations": 0, "indeterminate_evaluations": 0,
            "exact_fallback_evaluations": 0, "rounds": 0,
            "stop_reason": "budget_exhausted", "wall_ms": 1.0
        });
        let rows = load_run(value).expect("valid run state should load");
        assert_eq!(
            rows[0].final_algorithm_state,
            AlgorithmStateRow::EvaluatedPoint {
                evaluation_id: "e0".into()
            }
        );
    }

    #[test]
    fn present_malformed_run_state_is_rejected() {
        let value = serde_json::json!({
            "schema_version": 1, "run_id": "r", "start_id": "s",
            "algorithm_id": "a", "algorithm_kind": "k", "seed": 1,
            "budget": 2, "charge_initial": false,
            "initial_evaluation_id": "e0", "initial_sys": 1.0,
            "best_evaluation_id": "e0", "best_sys": 1.0,
            "final_algorithm_state": {"kind":"unknown"},
            "charged_calls": 0, "physical_evaluations": 1,
            "invalid_evaluations": 0, "indeterminate_evaluations": 0,
            "exact_fallback_evaluations": 0, "rounds": 0,
            "stop_reason": "budget_exhausted", "wall_ms": 1.0
        });
        assert!(load_run(value).is_err());
    }

    fn load_run(value: serde_json::Value) -> Result<Vec<RunRow>, String> {
        let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
        let path = directory.path().join("runs.jsonl");
        fs::write(
            path,
            serde_json::to_string(&value).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        load_jsonl(&directory.path().join("runs.jsonl"))
    }
}
