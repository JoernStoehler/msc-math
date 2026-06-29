use crate::panel_io::load_jsonl;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct LocalProbeRow {
    poly_id: String,
    step: f64,
    status: String,
    decomposition_total_prediction_error: Option<f64>,
    decomposition_fixed_sigma_linearization_error: Option<f64>,
    decomposition_inside_window_selection_error: Option<f64>,
    decomposition_window_miss_error: Option<f64>,
    decomposition_capacity_linearization_error: Option<f64>,
    decomposition_volume_linearization_error: Option<f64>,
    decomposition_capacity_volume_interaction_error: Option<f64>,
    decomposition_linearization_error: Option<f64>,
    decomposition_sigma_set_error: Option<f64>,
    target_best_sigma_in_base_candidate_window: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct BetaScanRow {
    poly_id: String,
    facet_count: usize,
    status: String,
    raw_ok_branches: usize,
    min_abs_beta_margin: Option<f64>,
    near_abs_1e_6: usize,
    near_abs_1e_5: usize,
    near_abs_1e_4: usize,
    near_abs_1e_3: usize,
    near_abs_1e_2: usize,
}

#[derive(Serialize)]
pub(crate) struct PredictionHighlight {
    pub(crate) facet_count: usize,
    pub(crate) step: f64,
    pub(crate) rows: usize,
    pub(crate) ok_rows: usize,
    pub(crate) failure_rows: usize,
    pub(crate) median_abs_total_error: Option<f64>,
    pub(crate) max_abs_total_error: Option<f64>,
    pub(crate) median_abs_fixed_sigma_linearization_error: Option<f64>,
    pub(crate) max_abs_fixed_sigma_linearization_error: Option<f64>,
    pub(crate) median_abs_inside_window_selection_error: Option<f64>,
    pub(crate) max_abs_inside_window_selection_error: Option<f64>,
    pub(crate) median_abs_window_miss_error: Option<f64>,
    pub(crate) max_abs_window_miss_error: Option<f64>,
    pub(crate) median_abs_capacity_linearization_error: Option<f64>,
    pub(crate) max_abs_capacity_linearization_error: Option<f64>,
    pub(crate) median_abs_volume_linearization_error: Option<f64>,
    pub(crate) max_abs_volume_linearization_error: Option<f64>,
    pub(crate) median_abs_capacity_volume_interaction_error: Option<f64>,
    pub(crate) max_abs_capacity_volume_interaction_error: Option<f64>,
    pub(crate) median_abs_linearization_error: Option<f64>,
    pub(crate) max_abs_linearization_error: Option<f64>,
    pub(crate) median_abs_sigma_set_error: Option<f64>,
    pub(crate) max_abs_sigma_set_error: Option<f64>,
    pub(crate) target_best_not_in_base_window: usize,
}

#[derive(Serialize)]
pub(crate) struct BetaFacetSummary {
    pub(crate) facet_count: usize,
    pub(crate) path: String,
    pub(crate) rows: usize,
    pub(crate) ok_rows: usize,
    pub(crate) near_abs_1e_6: usize,
    pub(crate) near_abs_1e_5: usize,
    pub(crate) near_abs_1e_4: usize,
    pub(crate) near_abs_1e_3: usize,
    pub(crate) near_abs_1e_2: usize,
    pub(crate) closest_examples: Vec<BetaClosestExample>,
}

#[derive(Serialize)]
pub(crate) struct BetaClosestExample {
    poly_id: String,
    facet_count: usize,
    min_abs_beta_margin: Option<f64>,
    raw_ok_branches: usize,
    near_abs_1e_5: usize,
}

pub(crate) fn summarize_prediction_probe(
    probe_path: &Path,
    poly_id_to_facet: &BTreeMap<String, usize>,
) -> Vec<PredictionHighlight> {
    let mut groups: BTreeMap<(usize, String), Vec<LocalProbeRow>> = BTreeMap::new();
    for row in load_jsonl::<LocalProbeRow>(probe_path) {
        let facet_count = *poly_id_to_facet
            .get(&row.poly_id)
            .unwrap_or_else(|| panic!("probe row for unknown poly_id {}", row.poly_id));
        groups
            .entry((facet_count, step_key(row.step)))
            .or_default()
            .push(row);
    }

    groups
        .into_iter()
        .map(|((facet_count, step), rows)| prediction_highlight(facet_count, &step, &rows))
        .collect()
}

pub(crate) fn summarize_beta_scan(facet_count: usize, path: &Path) -> BetaFacetSummary {
    let rows = load_jsonl::<BetaScanRow>(path);
    let ok_rows = rows
        .iter()
        .filter(|row| row.status == "ok")
        .collect::<Vec<_>>();
    let mut closest = ok_rows.clone();
    closest.sort_by(|a, b| {
        margin_abs_or_inf(a.min_abs_beta_margin)
            .total_cmp(&margin_abs_or_inf(b.min_abs_beta_margin))
    });

    BetaFacetSummary {
        facet_count,
        path: path.display().to_string(),
        rows: rows.len(),
        ok_rows: ok_rows.len(),
        near_abs_1e_6: ok_rows.iter().map(|row| row.near_abs_1e_6).sum(),
        near_abs_1e_5: ok_rows.iter().map(|row| row.near_abs_1e_5).sum(),
        near_abs_1e_4: ok_rows.iter().map(|row| row.near_abs_1e_4).sum(),
        near_abs_1e_3: ok_rows.iter().map(|row| row.near_abs_1e_3).sum(),
        near_abs_1e_2: ok_rows.iter().map(|row| row.near_abs_1e_2).sum(),
        closest_examples: closest
            .into_iter()
            .take(5)
            .map(|row| BetaClosestExample {
                poly_id: row.poly_id.clone(),
                facet_count: row.facet_count,
                min_abs_beta_margin: row.min_abs_beta_margin,
                raw_ok_branches: row.raw_ok_branches,
                near_abs_1e_5: row.near_abs_1e_5,
            })
            .collect(),
    }
}

fn prediction_highlight(
    facet_count: usize,
    step_key: &str,
    rows: &[LocalProbeRow],
) -> PredictionHighlight {
    let ok_rows = rows.iter().filter(|row| row.status == "ok").count();
    let total_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| row.decomposition_total_prediction_error.map(f64::abs))
        .collect::<Vec<_>>();
    let fixed_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| {
            row.decomposition_fixed_sigma_linearization_error
                .map(f64::abs)
        })
        .collect::<Vec<_>>();
    let inside_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| {
            row.decomposition_inside_window_selection_error
                .map(f64::abs)
        })
        .collect::<Vec<_>>();
    let window_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| {
            row.decomposition_window_miss_error
                .or(row.decomposition_sigma_set_error)
                .map(f64::abs)
        })
        .collect::<Vec<_>>();
    let capacity_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| row.decomposition_capacity_linearization_error.map(f64::abs))
        .collect::<Vec<_>>();
    let volume_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| row.decomposition_volume_linearization_error.map(f64::abs))
        .collect::<Vec<_>>();
    let interaction_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| {
            row.decomposition_capacity_volume_interaction_error
                .map(f64::abs)
        })
        .collect::<Vec<_>>();
    let linear_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| row.decomposition_linearization_error.map(f64::abs))
        .collect::<Vec<_>>();
    let sigma_abs = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter_map(|row| row.decomposition_sigma_set_error.map(f64::abs))
        .collect::<Vec<_>>();
    let target_best_not_in_base_window = rows
        .iter()
        .filter(|row| row.status == "ok")
        .filter(|row| row.target_best_sigma_in_base_candidate_window == Some(false))
        .count();

    PredictionHighlight {
        facet_count,
        step: step_key.parse().expect("step key must be f64"),
        rows: rows.len(),
        ok_rows,
        failure_rows: rows.len() - ok_rows,
        median_abs_total_error: percentile(total_abs.clone(), 0.5),
        max_abs_total_error: percentile(total_abs, 1.0),
        median_abs_fixed_sigma_linearization_error: percentile(fixed_abs.clone(), 0.5),
        max_abs_fixed_sigma_linearization_error: percentile(fixed_abs, 1.0),
        median_abs_inside_window_selection_error: percentile(inside_abs.clone(), 0.5),
        max_abs_inside_window_selection_error: percentile(inside_abs, 1.0),
        median_abs_window_miss_error: percentile(window_abs.clone(), 0.5),
        max_abs_window_miss_error: percentile(window_abs, 1.0),
        median_abs_capacity_linearization_error: percentile(capacity_abs.clone(), 0.5),
        max_abs_capacity_linearization_error: percentile(capacity_abs, 1.0),
        median_abs_volume_linearization_error: percentile(volume_abs.clone(), 0.5),
        max_abs_volume_linearization_error: percentile(volume_abs, 1.0),
        median_abs_capacity_volume_interaction_error: percentile(interaction_abs.clone(), 0.5),
        max_abs_capacity_volume_interaction_error: percentile(interaction_abs, 1.0),
        median_abs_linearization_error: percentile(linear_abs.clone(), 0.5),
        max_abs_linearization_error: percentile(linear_abs, 1.0),
        median_abs_sigma_set_error: percentile(sigma_abs.clone(), 0.5),
        max_abs_sigma_set_error: percentile(sigma_abs, 1.0),
        target_best_not_in_base_window,
    }
}

fn margin_abs_or_inf(value: Option<f64>) -> f64 {
    value.map(f64::abs).unwrap_or(f64::INFINITY)
}

fn percentile(mut values: Vec<f64>, p: f64) -> Option<f64> {
    values.retain(|value| value.is_finite());
    values.sort_by(|a, b| a.total_cmp(b));
    match values.len() {
        0 => None,
        1 => Some(values[0]),
        len => {
            let pos = (len - 1) as f64 * p;
            let lo = pos.floor() as usize;
            let hi = pos.ceil() as usize;
            if lo == hi {
                Some(values[lo])
            } else {
                Some(values[lo] * (hi as f64 - pos) + values[hi] * (pos - lo as f64))
            }
        }
    }
}

fn step_key(step: f64) -> String {
    format!("{step:.17e}")
}
