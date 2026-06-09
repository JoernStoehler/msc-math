use super::row::PredictionRow;
use std::collections::BTreeMap;
use std::path::Path;

pub(crate) struct SmokeReport {
    row_count: usize,
    successful_count: usize,
    generic_success: bool,
    basepoint_summaries: Vec<BasepointSummary>,
}

impl SmokeReport {
    pub(crate) fn print(&self, output_path: &Path) {
        println!(
            "local-sys-prediction-smoke: wrote {} rows to {}",
            self.row_count,
            output_path.display()
        );
        println!("  successful rows: {}", self.successful_count);
        println!("  generic basepoint success: {}", self.generic_success);
        println!();
        self.print_basepoint_summaries();
    }

    pub(crate) fn has_required_success(&self) -> bool {
        self.successful_count > 0 && self.generic_success
    }

    pub(super) fn from_rows(rows: &[PredictionRow]) -> Self {
        Self {
            row_count: rows.len(),
            successful_count: rows.iter().filter(|row| row.status == "ok").count(),
            generic_success: rows
                .iter()
                .any(|row| row.basepoint_name.starts_with("random_f10") && row.status == "ok"),
            basepoint_summaries: summarize_by_basepoint(rows),
        }
    }

    fn print_basepoint_summaries(&self) {
        println!("basepoint summaries:");
        for summary in &self.basepoint_summaries {
            println!("  {}", summary.name);
            println!(
                "    rows: {}, ok: {}, failed: {}",
                summary.rows, summary.ok, summary.failed
            );
            println!(
                "    active_orbits: {}, candidate_orbits: {}",
                format_optional_usize(summary.active_orbit_count),
                format_optional_usize(summary.candidate_orbit_count)
            );
            println!(
                "    max_rel_error: {}",
                format_optional_f64(summary.max_rel_error)
            );
            println!(
                "    outside_base_active: {}, outside_base_candidate: {}",
                summary.outside_base_active, summary.outside_base_candidate
            );
            println!(
                "    miss_causes: {}",
                format_miss_causes(&summary.miss_causes)
            );
        }
    }
}

#[derive(Debug)]
struct BasepointSummary {
    name: String,
    rows: usize,
    ok: usize,
    failed: usize,
    active_orbit_count: Option<usize>,
    candidate_orbit_count: Option<usize>,
    max_rel_error: Option<f64>,
    outside_base_active: usize,
    outside_base_candidate: usize,
    miss_causes: BTreeMap<MissCause, usize>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MissCause {
    StableOrSeen,
    TransitionOpened,
    KktOrAdmissibilityChange,
    ActionWindowMiss,
    UnknownOrFailed,
}

impl MissCause {
    fn label(self) -> &'static str {
        match self {
            Self::StableOrSeen => "stable_or_seen",
            Self::TransitionOpened => "transition_opened",
            Self::KktOrAdmissibilityChange => "kkt_or_admissibility_change",
            Self::ActionWindowMiss => "action_window_miss",
            Self::UnknownOrFailed => "unknown_or_failed",
        }
    }
}

fn summarize_by_basepoint(rows: &[PredictionRow]) -> Vec<BasepointSummary> {
    let mut groups: BTreeMap<&str, Vec<&PredictionRow>> = BTreeMap::new();
    for row in rows {
        groups.entry(&row.basepoint_name).or_default().push(row);
    }

    groups
        .into_iter()
        .map(|(name, group)| {
            let rows = group.len();
            let ok = group.iter().filter(|row| row.status == "ok").count();
            let failed = rows - ok;
            let active_orbit_count = stable_usize(group.iter().map(|row| row.active_orbit_count));
            let candidate_orbit_count =
                stable_usize(group.iter().map(|row| row.base_candidate_orbit_count));
            let max_rel_error = group
                .iter()
                .filter_map(|row| row.rel_prediction_error)
                .max_by(f64::total_cmp);
            let outside_base_active = group
                .iter()
                .filter(|row| row.target_best_sigma_in_base_active_set == Some(false))
                .count();
            let outside_base_candidate = group
                .iter()
                .filter(|row| row.target_best_sigma_in_base_candidate_window == Some(false))
                .count();
            let mut miss_causes = BTreeMap::new();
            for row in group {
                *miss_causes.entry(classify_miss_cause(row)).or_default() += 1;
            }

            BasepointSummary {
                name: name.to_string(),
                rows,
                ok,
                failed,
                active_orbit_count,
                candidate_orbit_count,
                max_rel_error,
                outside_base_active,
                outside_base_candidate,
                miss_causes,
            }
        })
        .collect()
}

fn classify_miss_cause(row: &PredictionRow) -> MissCause {
    if row.status != "ok" {
        return MissCause::UnknownOrFailed;
    }
    if row.target_best_sigma_in_base_active_set == Some(true)
        || row.target_best_sigma_in_base_candidate_window == Some(true)
    {
        return MissCause::StableOrSeen;
    }
    if row
        .target_best_sigma_transitions_opened
        .as_ref()
        .is_some_and(|opened| !opened.is_empty())
    {
        return MissCause::TransitionOpened;
    }
    if row.target_best_sigma_base_transition_allowed != Some(true) {
        return MissCause::UnknownOrFailed;
    }
    let Some(status) = &row.target_best_sigma_base_solve_status else {
        return MissCause::UnknownOrFailed;
    };
    if !status.contains("AdmissibleF64") && !status.contains("AdmissibleExact") {
        return MissCause::KktOrAdmissibilityChange;
    }
    if row.target_best_sigma_base_action_gap.is_some() {
        return MissCause::ActionWindowMiss;
    }
    MissCause::UnknownOrFailed
}

fn stable_usize(values: impl Iterator<Item = usize>) -> Option<usize> {
    let mut values = values;
    let first = values.next()?;
    values.all(|value| value == first).then_some(first)
}

fn format_optional_usize(value: Option<usize>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "mixed".to_string())
}

fn format_optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.6e}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_miss_causes(counts: &BTreeMap<MissCause, usize>) -> String {
    [
        MissCause::StableOrSeen,
        MissCause::TransitionOpened,
        MissCause::KktOrAdmissibilityChange,
        MissCause::ActionWindowMiss,
        MissCause::UnknownOrFailed,
    ]
    .into_iter()
    .filter_map(|cause| {
        counts
            .get(&cause)
            .map(|count| format!("{}={count}", cause.label()))
    })
    .collect::<Vec<_>>()
    .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row_for_classification() -> PredictionRow {
        PredictionRow {
            basepoint_name: "test".to_string(),
            facet_count: 10,
            direction_label: "direction".to_string(),
            step: 1e-4,
            status: "ok".to_string(),
            sys0: 1.0,
            predicted_sys: 1.0,
            recomputed_sys: Some(1.0),
            abs_prediction_error: Some(0.0),
            rel_prediction_error: Some(0.0),
            base_best_sigma: vec![0, 1, 2],
            target_best_sigma: Some(vec![0, 1, 2]),
            target_best_sigma_in_base_active_set: Some(false),
            target_best_sigma_in_base_candidate_window: Some(false),
            target_best_sigma_base_transition_allowed: Some(true),
            target_best_sigma_base_solve_status: Some("ok:AdmissibleF64".to_string()),
            target_best_sigma_base_action_gap: Some(0.1),
            target_best_sigma_transitions_opened: Some(Vec::new()),
            active_orbit_count: 1,
            base_candidate_orbit_count: 1,
            base_candidate_action_gap: 1e-2,
            active_action_spread: 0.0,
            active_min_beta_margin: 0.1,
            active_max_q_error_bound: 0.0,
        }
    }

    #[test]
    fn classify_stable_or_seen() {
        let mut row = row_for_classification();
        row.target_best_sigma_in_base_active_set = Some(true);
        assert_eq!(classify_miss_cause(&row), MissCause::StableOrSeen);

        row.target_best_sigma_in_base_active_set = Some(false);
        row.target_best_sigma_in_base_candidate_window = Some(true);
        assert_eq!(classify_miss_cause(&row), MissCause::StableOrSeen);
    }

    #[test]
    fn classify_transition_opened() {
        let mut row = row_for_classification();
        row.target_best_sigma_transitions_opened = Some(vec![[0, 1]]);
        assert_eq!(classify_miss_cause(&row), MissCause::TransitionOpened);
    }

    #[test]
    fn classify_kkt_or_admissibility_change() {
        let mut row = row_for_classification();
        row.target_best_sigma_base_solve_status = Some("Inadmissible".to_string());
        assert_eq!(
            classify_miss_cause(&row),
            MissCause::KktOrAdmissibilityChange
        );
    }

    #[test]
    fn classify_action_window_miss() {
        let row = row_for_classification();
        assert_eq!(classify_miss_cause(&row), MissCause::ActionWindowMiss);
    }

    #[test]
    fn classify_unknown_or_failed() {
        let mut row = row_for_classification();
        row.status = "target_capacity_failed".to_string();
        assert_eq!(classify_miss_cause(&row), MissCause::UnknownOrFailed);

        let mut row = row_for_classification();
        row.target_best_sigma_base_solve_status = None;
        assert_eq!(classify_miss_cause(&row), MissCause::UnknownOrFailed);
    }
}
