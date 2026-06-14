use exp_dev_f64_capacity::ScanRow;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Summary {
    families: Vec<FamilySummaryRow>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct FamilySummaryRow {
    family: String,
    validation_policy: String,
    capacity_method: String,
    rows: usize,
    accepted_decisive: usize,
    accepted_ambiguous: usize,
    validation_rejected: usize,
    validation_fallback_required: usize,
    capacity_run_rows: usize,
    capacity_not_run_rows: usize,
    success: usize,
    failure: usize,
    success_rate: f64,
    clean: usize,
    degenerate_value_agrees: usize,
    fallback_required: usize,
    indeterminate_overlap: usize,
    exact_audit_statuses: String,
    product_rounding_statuses: String,
    near_redundant_facet_removal_policies: String,
    near_redundant_facet_removal_statuses: String,
    origin_lp_statuses: String,
    max_product_rounding_minor_over_major: Option<f64>,
    max_product_rounding_abs_change: Option<f64>,
    max_removed_facet_count: Option<usize>,
    max_near_redundant_facet_removal_delta_bound: Option<f64>,
    max_capacity_ratio_upper_bound: Option<f64>,
    max_volume_ratio_upper_bound: Option<f64>,
    min_sys_ratio_lower_bound: Option<f64>,
    max_sys_ratio_upper_bound: Option<f64>,
    preprocessed_f64_vs_original_artifact_within_bound: String,
    preprocessed_audit_vs_original_artifact_within_bound: String,
    max_preprocessed_f64_vs_original_artifact_rel_error: Option<f64>,
    max_preprocessed_audit_vs_original_artifact_rel_error: Option<f64>,
    max_preprocessed_f64_vs_preprocessed_audit_rel_error: Option<f64>,
    median_exact_audit_time_ms: Option<f64>,
    max_exact_audit_time_ms: Option<f64>,
    median_origin_lp_max_min_lambda: Option<f64>,
    max_origin_lp_max_abs_residual: Option<f64>,
    max_facets_without_definite_vertex: Option<usize>,
    max_facets_without_possible_vertex: Option<usize>,
    median_validation_bundle_time_ms: Option<f64>,
    max_validation_bundle_time_ms: Option<f64>,
    median_capacity_bundle_time_ms: Option<f64>,
    max_capacity_bundle_time_ms: Option<f64>,
    median_rel_error: Option<f64>,
    max_rel_error: Option<f64>,
    max_abs_error: Option<f64>,
    median_gap: Option<f64>,
    min_gap: Option<f64>,
    median_vertex_indeterminate: Option<usize>,
    max_vertex_indeterminate: Option<usize>,
    median_near_singular_vertex: Option<usize>,
    max_near_singular_vertex: Option<usize>,
    median_bounded_near_singular_vertex: Option<usize>,
    max_bounded_near_singular_vertex: Option<usize>,
    median_ambiguous_vertex_incidence: Option<usize>,
    max_ambiguous_vertex_incidence: Option<usize>,
    median_facet_intersection_indeterminate: Option<usize>,
    max_facet_intersection_indeterminate: Option<usize>,
    median_omega_indeterminate: Option<usize>,
    max_omega_indeterminate: Option<usize>,
    median_kkt_indeterminate: Option<usize>,
    max_kkt_indeterminate: Option<usize>,
    validation_reasons: String,
    exact_audit_reasons: String,
    failure_reasons: String,
    trust_reasons: String,
}

#[derive(Default)]
struct FamilySummary {
    rows: usize,
    accepted_decisive: usize,
    accepted_ambiguous: usize,
    validation_rejected: usize,
    validation_fallback_required: usize,
    capacity_run_rows: usize,
    capacity_not_run_rows: usize,
    success: usize,
    failure: usize,
    clean: usize,
    degenerate_value_agrees: usize,
    fallback_required: usize,
    indeterminate_overlap: usize,
    exact_audit_times_ms: Vec<f64>,
    origin_lp_max_min_lambdas: Vec<f64>,
    origin_lp_max_abs_residuals: Vec<f64>,
    facets_without_definite_vertex_counts: Vec<usize>,
    facets_without_possible_vertex_counts: Vec<usize>,
    validation_bundle_times_ms: Vec<f64>,
    capacity_bundle_times_ms: Vec<f64>,
    abs_errors: Vec<f64>,
    rel_errors: Vec<f64>,
    gaps: Vec<f64>,
    vertex_indeterminate_counts: Vec<usize>,
    near_singular_vertex_counts: Vec<usize>,
    bounded_near_singular_vertex_counts: Vec<usize>,
    ambiguous_vertex_incidence_counts: Vec<usize>,
    facet_intersection_indeterminate_counts: Vec<usize>,
    omega_indeterminate_counts: Vec<usize>,
    kkt_indeterminate_counts: Vec<usize>,
    exact_audit_statuses: BTreeMap<String, usize>,
    product_rounding_statuses: BTreeMap<String, usize>,
    near_redundant_facet_removal_policies: BTreeMap<String, usize>,
    near_redundant_facet_removal_statuses: BTreeMap<String, usize>,
    product_rounding_minor_over_major: Vec<f64>,
    product_rounding_abs_changes: Vec<f64>,
    removed_facet_counts: Vec<usize>,
    near_redundant_facet_removal_delta_bounds: Vec<f64>,
    capacity_ratio_upper_bounds: Vec<f64>,
    volume_ratio_upper_bounds: Vec<f64>,
    sys_ratio_lower_bounds: Vec<f64>,
    sys_ratio_upper_bounds: Vec<f64>,
    preprocessed_f64_vs_original_artifact_within_bound: BTreeMap<String, usize>,
    preprocessed_audit_vs_original_artifact_within_bound: BTreeMap<String, usize>,
    preprocessed_f64_vs_original_artifact_rel_errors: Vec<f64>,
    preprocessed_audit_vs_original_artifact_rel_errors: Vec<f64>,
    preprocessed_f64_vs_preprocessed_audit_rel_errors: Vec<f64>,
    origin_lp_statuses: BTreeMap<String, usize>,
    validation_reasons: BTreeMap<String, usize>,
    exact_audit_reasons: BTreeMap<String, usize>,
    failure_reasons: BTreeMap<String, usize>,
    trust_reasons: BTreeMap<String, usize>,
}

pub(crate) fn summarize(rows: impl IntoIterator<Item = ScanRow>) -> Summary {
    let mut by_family: BTreeMap<(String, String, String), FamilySummary> = BTreeMap::new();
    for row in rows {
        by_family
            .entry((
                row.family.clone(),
                row.validation_policy.clone(),
                row.capacity_method.clone(),
            ))
            .or_default()
            .record(row);
    }

    Summary {
        families: by_family
            .into_iter()
            .map(
                |((family, validation_policy, capacity_method), mut summary)| {
                    summary.prepare_quantiles();
                    summary.into_row(family, validation_policy, capacity_method)
                },
            )
            .collect(),
    }
}

pub(crate) fn print_summary(summary: &Summary) {
    println!(
        "family,validation_policy,capacity_method,rows,accepted_decisive,accepted_ambiguous,validation_rejected,validation_fallback_required,capacity_run_rows,capacity_not_run_rows,success,failure,success_rate,clean,degenerate_value_agrees,fallback_required,indeterminate_overlap,exact_audit_statuses,product_rounding_statuses,near_redundant_facet_removal_policies,near_redundant_facet_removal_statuses,origin_lp_statuses,max_product_rounding_minor_over_major,max_product_rounding_abs_change,max_removed_facet_count,max_near_redundant_facet_removal_delta_bound,max_capacity_ratio_upper_bound,max_volume_ratio_upper_bound,min_sys_ratio_lower_bound,max_sys_ratio_upper_bound,preprocessed_f64_vs_original_artifact_within_bound,preprocessed_audit_vs_original_artifact_within_bound,max_preprocessed_f64_vs_original_artifact_rel_error,max_preprocessed_audit_vs_original_artifact_rel_error,max_preprocessed_f64_vs_preprocessed_audit_rel_error,median_exact_audit_time_ms,max_exact_audit_time_ms,median_origin_lp_max_min_lambda,max_origin_lp_max_abs_residual,max_facets_without_definite_vertex,max_facets_without_possible_vertex,median_validation_bundle_time_ms,max_validation_bundle_time_ms,median_capacity_bundle_time_ms,max_capacity_bundle_time_ms,median_rel_error,max_rel_error,max_abs_error,median_gap,min_gap,median_vertex_indeterminate,max_vertex_indeterminate,median_near_singular_vertex,max_near_singular_vertex,median_bounded_near_singular_vertex,max_bounded_near_singular_vertex,median_ambiguous_vertex_incidence,max_ambiguous_vertex_incidence,median_facet_intersection_indeterminate,max_facet_intersection_indeterminate,median_omega_indeterminate,max_omega_indeterminate,median_kkt_indeterminate,max_kkt_indeterminate,validation_reasons,exact_audit_reasons,failure_reasons,trust_reasons"
    );
    for row in &summary.families {
        println!("{}", row.csv_line());
    }
}

pub(crate) fn write_json_summary(path: &Path, summary: &Summary) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create JSON summary directory");
        }
    }
    let file = File::create(path).expect("create JSON summary");
    serde_json::to_writer_pretty(BufWriter::new(file), summary).expect("write JSON summary");
}

impl FamilySummary {
    fn record(&mut self, row: ScanRow) {
        self.rows += 1;
        match row.validation_status.as_str() {
            "accepted_decisive" => self.accepted_decisive += 1,
            "accepted_ambiguous" => self.accepted_ambiguous += 1,
            "rejected" => self.validation_rejected += 1,
            "fallback_required" => self.validation_fallback_required += 1,
            other => {
                *self
                    .validation_reasons
                    .entry(format!("unknown_validation_status:{other}"))
                    .or_default() += 1
            }
        }
        for reason in &row.validation_reasons {
            *self.validation_reasons.entry(reason.clone()).or_default() += 1;
        }
        if row.outcome == "success" {
            self.success += 1;
        } else {
            self.failure += 1;
            let reason = row.failure_reason.unwrap_or_else(|| "unknown".to_string());
            *self.failure_reasons.entry(reason).or_insert(0) += 1;
        }
        match row.trust_class.as_str() {
            "clean" => self.clean += 1,
            "degenerate_value_agrees" => self.degenerate_value_agrees += 1,
            "fallback_required" => self.fallback_required += 1,
            other => {
                *self
                    .trust_reasons
                    .entry(format!("unknown_trust_class:{other}"))
                    .or_default() += 1
            }
        }
        for reason in row.trust_reasons {
            *self.trust_reasons.entry(reason).or_default() += 1;
        }
        if row.indeterminate_overlaps_best_interval {
            self.indeterminate_overlap += 1;
        }
        *self
            .exact_audit_statuses
            .entry(row.exact_audit_status.clone())
            .or_default() += 1;
        *self
            .product_rounding_statuses
            .entry(row.product_rounding_status.clone())
            .or_default() += 1;
        *self
            .near_redundant_facet_removal_policies
            .entry(row.near_redundant_facet_removal_policy.clone())
            .or_default() += 1;
        *self
            .near_redundant_facet_removal_statuses
            .entry(row.near_redundant_facet_removal_status.clone())
            .or_default() += 1;
        if let Some(value) = row.product_rounding_max_minor_over_major {
            self.product_rounding_minor_over_major.push(value);
        }
        if let Some(value) = row.product_rounding_max_abs_change {
            self.product_rounding_abs_changes.push(value);
        }
        self.removed_facet_counts.push(row.removed_facet_count);
        if let Some(value) = row.near_redundant_facet_removal_delta_bound {
            self.near_redundant_facet_removal_delta_bounds.push(value);
        }
        if let Some(value) = row.capacity_ratio_upper_bound {
            self.capacity_ratio_upper_bounds.push(value);
        }
        if let Some(value) = row.volume_ratio_upper_bound {
            self.volume_ratio_upper_bounds.push(value);
        }
        if let Some(value) = row.sys_ratio_lower_bound {
            self.sys_ratio_lower_bounds.push(value);
        }
        if let Some(value) = row.sys_ratio_upper_bound {
            self.sys_ratio_upper_bounds.push(value);
        }
        *self
            .preprocessed_f64_vs_original_artifact_within_bound
            .entry(option_bool_label(
                row.preprocessed_f64_vs_original_artifact_within_bound,
            ))
            .or_default() += 1;
        *self
            .preprocessed_audit_vs_original_artifact_within_bound
            .entry(option_bool_label(
                row.preprocessed_audit_vs_original_artifact_within_bound,
            ))
            .or_default() += 1;
        if let Some(value) = row.preprocessed_f64_vs_original_artifact_rel_error {
            self.preprocessed_f64_vs_original_artifact_rel_errors
                .push(value);
        }
        if let Some(value) = row.preprocessed_audit_vs_original_artifact_rel_error {
            self.preprocessed_audit_vs_original_artifact_rel_errors
                .push(value);
        }
        if let Some(value) = row.preprocessed_f64_vs_preprocessed_audit_rel_error {
            self.preprocessed_f64_vs_preprocessed_audit_rel_errors
                .push(value);
        }
        *self
            .origin_lp_statuses
            .entry(row.origin_lp_status.clone())
            .or_default() += 1;
        if let Some(value) = row.origin_lp_max_min_lambda {
            self.origin_lp_max_min_lambdas.push(value);
        }
        if let Some(value) = row.origin_lp_max_abs_residual {
            self.origin_lp_max_abs_residuals.push(value);
        }
        self.facets_without_definite_vertex_counts
            .push(row.facets_without_definite_vertex_count);
        self.facets_without_possible_vertex_counts
            .push(row.facets_without_possible_vertex_count);
        for reason in &row.exact_audit_reasons {
            *self.exact_audit_reasons.entry(reason.clone()).or_default() += 1;
        }
        if row.exact_audit_status != "not_requested" {
            self.exact_audit_times_ms.push(row.exact_audit_time_ms);
        }
        self.validation_bundle_times_ms.push(row.validation_time_ms);
        if row.outcome != "not_run" {
            self.capacity_run_rows += 1;
            self.capacity_bundle_times_ms.push(row.f64_time_ms);
        } else {
            self.capacity_not_run_rows += 1;
        }
        if let Some(err) = row.abs_action_error {
            self.abs_errors.push(err);
        }
        if let Some(err) = row.rel_action_error {
            self.rel_errors.push(err);
        }
        if let Some(gap) = row.min_action_gap {
            self.gaps.push(gap);
        }
        self.vertex_indeterminate_counts
            .push(row.vertex_indeterminate_count);
        self.near_singular_vertex_counts
            .push(row.near_singular_vertex_count);
        self.bounded_near_singular_vertex_counts
            .push(row.bounded_near_singular_vertex_count);
        self.ambiguous_vertex_incidence_counts
            .push(row.ambiguous_vertex_incidence_count);
        self.facet_intersection_indeterminate_counts
            .push(row.facet_intersection_indeterminate_count);
        self.omega_indeterminate_counts
            .push(row.omega_indeterminate_count);
        self.kkt_indeterminate_counts
            .push(row.indeterminate_f64_count);
    }

    fn prepare_quantiles(&mut self) {
        self.capacity_bundle_times_ms.sort_by(f64::total_cmp);
        self.exact_audit_times_ms.sort_by(f64::total_cmp);
        self.product_rounding_minor_over_major
            .sort_by(f64::total_cmp);
        self.product_rounding_abs_changes.sort_by(f64::total_cmp);
        self.removed_facet_counts.sort_unstable();
        self.near_redundant_facet_removal_delta_bounds
            .sort_by(f64::total_cmp);
        self.capacity_ratio_upper_bounds.sort_by(f64::total_cmp);
        self.volume_ratio_upper_bounds.sort_by(f64::total_cmp);
        self.sys_ratio_lower_bounds.sort_by(f64::total_cmp);
        self.sys_ratio_upper_bounds.sort_by(f64::total_cmp);
        self.preprocessed_f64_vs_original_artifact_rel_errors
            .sort_by(f64::total_cmp);
        self.preprocessed_audit_vs_original_artifact_rel_errors
            .sort_by(f64::total_cmp);
        self.preprocessed_f64_vs_preprocessed_audit_rel_errors
            .sort_by(f64::total_cmp);
        self.origin_lp_max_min_lambdas.sort_by(f64::total_cmp);
        self.origin_lp_max_abs_residuals.sort_by(f64::total_cmp);
        self.facets_without_definite_vertex_counts.sort_unstable();
        self.facets_without_possible_vertex_counts.sort_unstable();
        self.validation_bundle_times_ms.sort_by(f64::total_cmp);
        self.abs_errors.sort_by(f64::total_cmp);
        self.rel_errors.sort_by(f64::total_cmp);
        self.gaps.sort_by(f64::total_cmp);
        self.vertex_indeterminate_counts.sort_unstable();
        self.near_singular_vertex_counts.sort_unstable();
        self.bounded_near_singular_vertex_counts.sort_unstable();
        self.ambiguous_vertex_incidence_counts.sort_unstable();
        self.facet_intersection_indeterminate_counts.sort_unstable();
        self.omega_indeterminate_counts.sort_unstable();
        self.kkt_indeterminate_counts.sort_unstable();
    }

    fn into_row(
        self,
        family: String,
        validation_policy: String,
        capacity_method: String,
    ) -> FamilySummaryRow {
        FamilySummaryRow {
            family,
            validation_policy,
            capacity_method,
            rows: self.rows,
            accepted_decisive: self.accepted_decisive,
            accepted_ambiguous: self.accepted_ambiguous,
            validation_rejected: self.validation_rejected,
            validation_fallback_required: self.validation_fallback_required,
            capacity_run_rows: self.capacity_run_rows,
            capacity_not_run_rows: self.capacity_not_run_rows,
            success: self.success,
            failure: self.failure,
            success_rate: ratio(self.success, self.rows),
            clean: self.clean,
            degenerate_value_agrees: self.degenerate_value_agrees,
            fallback_required: self.fallback_required,
            indeterminate_overlap: self.indeterminate_overlap,
            exact_audit_statuses: format_counts(&self.exact_audit_statuses),
            product_rounding_statuses: format_counts(&self.product_rounding_statuses),
            near_redundant_facet_removal_policies: format_counts(
                &self.near_redundant_facet_removal_policies,
            ),
            near_redundant_facet_removal_statuses: format_counts(
                &self.near_redundant_facet_removal_statuses,
            ),
            origin_lp_statuses: format_counts(&self.origin_lp_statuses),
            max_product_rounding_minor_over_major: self
                .product_rounding_minor_over_major
                .last()
                .copied(),
            max_product_rounding_abs_change: self.product_rounding_abs_changes.last().copied(),
            max_removed_facet_count: self.removed_facet_counts.last().copied(),
            max_near_redundant_facet_removal_delta_bound: self
                .near_redundant_facet_removal_delta_bounds
                .last()
                .copied(),
            max_capacity_ratio_upper_bound: self.capacity_ratio_upper_bounds.last().copied(),
            max_volume_ratio_upper_bound: self.volume_ratio_upper_bounds.last().copied(),
            min_sys_ratio_lower_bound: self.sys_ratio_lower_bounds.first().copied(),
            max_sys_ratio_upper_bound: self.sys_ratio_upper_bounds.last().copied(),
            preprocessed_f64_vs_original_artifact_within_bound: format_counts(
                &self.preprocessed_f64_vs_original_artifact_within_bound,
            ),
            preprocessed_audit_vs_original_artifact_within_bound: format_counts(
                &self.preprocessed_audit_vs_original_artifact_within_bound,
            ),
            max_preprocessed_f64_vs_original_artifact_rel_error: self
                .preprocessed_f64_vs_original_artifact_rel_errors
                .last()
                .copied(),
            max_preprocessed_audit_vs_original_artifact_rel_error: self
                .preprocessed_audit_vs_original_artifact_rel_errors
                .last()
                .copied(),
            max_preprocessed_f64_vs_preprocessed_audit_rel_error: self
                .preprocessed_f64_vs_preprocessed_audit_rel_errors
                .last()
                .copied(),
            median_exact_audit_time_ms: median(&self.exact_audit_times_ms),
            max_exact_audit_time_ms: self.exact_audit_times_ms.last().copied(),
            median_origin_lp_max_min_lambda: median(&self.origin_lp_max_min_lambdas),
            max_origin_lp_max_abs_residual: self.origin_lp_max_abs_residuals.last().copied(),
            max_facets_without_definite_vertex: self
                .facets_without_definite_vertex_counts
                .last()
                .copied(),
            max_facets_without_possible_vertex: self
                .facets_without_possible_vertex_counts
                .last()
                .copied(),
            median_validation_bundle_time_ms: median(&self.validation_bundle_times_ms),
            max_validation_bundle_time_ms: self.validation_bundle_times_ms.last().copied(),
            median_capacity_bundle_time_ms: median(&self.capacity_bundle_times_ms),
            max_capacity_bundle_time_ms: self.capacity_bundle_times_ms.last().copied(),
            median_rel_error: median(&self.rel_errors),
            max_rel_error: self.rel_errors.last().copied(),
            max_abs_error: self.abs_errors.last().copied(),
            median_gap: median(&self.gaps),
            min_gap: self.gaps.first().copied(),
            median_vertex_indeterminate: median_usize(&self.vertex_indeterminate_counts),
            max_vertex_indeterminate: self.vertex_indeterminate_counts.last().copied(),
            median_near_singular_vertex: median_usize(&self.near_singular_vertex_counts),
            max_near_singular_vertex: self.near_singular_vertex_counts.last().copied(),
            median_bounded_near_singular_vertex: median_usize(
                &self.bounded_near_singular_vertex_counts,
            ),
            max_bounded_near_singular_vertex: self
                .bounded_near_singular_vertex_counts
                .last()
                .copied(),
            median_ambiguous_vertex_incidence: median_usize(
                &self.ambiguous_vertex_incidence_counts,
            ),
            max_ambiguous_vertex_incidence: self.ambiguous_vertex_incidence_counts.last().copied(),
            median_facet_intersection_indeterminate: median_usize(
                &self.facet_intersection_indeterminate_counts,
            ),
            max_facet_intersection_indeterminate: self
                .facet_intersection_indeterminate_counts
                .last()
                .copied(),
            median_omega_indeterminate: median_usize(&self.omega_indeterminate_counts),
            max_omega_indeterminate: self.omega_indeterminate_counts.last().copied(),
            median_kkt_indeterminate: median_usize(&self.kkt_indeterminate_counts),
            max_kkt_indeterminate: self.kkt_indeterminate_counts.last().copied(),
            validation_reasons: format_counts(&self.validation_reasons),
            exact_audit_reasons: format_counts(&self.exact_audit_reasons),
            failure_reasons: format_counts(&self.failure_reasons),
            trust_reasons: format_counts(&self.trust_reasons),
        }
    }
}

impl FamilySummaryRow {
    fn csv_line(&self) -> String {
        vec![
            self.family.clone(),
            self.validation_policy.clone(),
            self.capacity_method.clone(),
            self.rows.to_string(),
            self.accepted_decisive.to_string(),
            self.accepted_ambiguous.to_string(),
            self.validation_rejected.to_string(),
            self.validation_fallback_required.to_string(),
            self.capacity_run_rows.to_string(),
            self.capacity_not_run_rows.to_string(),
            self.success.to_string(),
            self.failure.to_string(),
            format!("{:.6}", self.success_rate),
            self.clean.to_string(),
            self.degenerate_value_agrees.to_string(),
            self.fallback_required.to_string(),
            self.indeterminate_overlap.to_string(),
            self.exact_audit_statuses.clone(),
            self.product_rounding_statuses.clone(),
            self.near_redundant_facet_removal_policies.clone(),
            self.near_redundant_facet_removal_statuses.clone(),
            self.origin_lp_statuses.clone(),
            format_option(self.max_product_rounding_minor_over_major),
            format_option(self.max_product_rounding_abs_change),
            format_usize_option(self.max_removed_facet_count),
            format_option(self.max_near_redundant_facet_removal_delta_bound),
            format_option(self.max_capacity_ratio_upper_bound),
            format_option(self.max_volume_ratio_upper_bound),
            format_option(self.min_sys_ratio_lower_bound),
            format_option(self.max_sys_ratio_upper_bound),
            self.preprocessed_f64_vs_original_artifact_within_bound
                .clone(),
            self.preprocessed_audit_vs_original_artifact_within_bound
                .clone(),
            format_option(self.max_preprocessed_f64_vs_original_artifact_rel_error),
            format_option(self.max_preprocessed_audit_vs_original_artifact_rel_error),
            format_option(self.max_preprocessed_f64_vs_preprocessed_audit_rel_error),
            format_option(self.median_exact_audit_time_ms),
            format_option(self.max_exact_audit_time_ms),
            format_option(self.median_origin_lp_max_min_lambda),
            format_option(self.max_origin_lp_max_abs_residual),
            format_usize_option(self.max_facets_without_definite_vertex),
            format_usize_option(self.max_facets_without_possible_vertex),
            format_option(self.median_validation_bundle_time_ms),
            format_option(self.max_validation_bundle_time_ms),
            format_option(self.median_capacity_bundle_time_ms),
            format_option(self.max_capacity_bundle_time_ms),
            format_option(self.median_rel_error),
            format_option(self.max_rel_error),
            format_option(self.max_abs_error),
            format_option(self.median_gap),
            format_option(self.min_gap),
            format_usize_option(self.median_vertex_indeterminate),
            format_usize_option(self.max_vertex_indeterminate),
            format_usize_option(self.median_near_singular_vertex),
            format_usize_option(self.max_near_singular_vertex),
            format_usize_option(self.median_bounded_near_singular_vertex),
            format_usize_option(self.max_bounded_near_singular_vertex),
            format_usize_option(self.median_ambiguous_vertex_incidence),
            format_usize_option(self.max_ambiguous_vertex_incidence),
            format_usize_option(self.median_facet_intersection_indeterminate),
            format_usize_option(self.max_facet_intersection_indeterminate),
            format_usize_option(self.median_omega_indeterminate),
            format_usize_option(self.max_omega_indeterminate),
            format_usize_option(self.median_kkt_indeterminate),
            format_usize_option(self.max_kkt_indeterminate),
            self.validation_reasons.clone(),
            self.exact_audit_reasons.clone(),
            self.failure_reasons.clone(),
            self.trust_reasons.clone(),
        ]
        .join(",")
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values[values.len() / 2])
    }
}

fn format_option(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.12e}"))
        .unwrap_or_default()
}

fn median_usize(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        None
    } else {
        Some(values[values.len() / 2])
    }
}

fn format_usize_option(value: Option<usize>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn format_counts(counts: &BTreeMap<String, usize>) -> String {
    counts
        .iter()
        .map(|(reason, count)| format!("{reason}:{count}"))
        .collect::<Vec<_>>()
        .join("|")
}

fn option_bool_label(value: Option<bool>) -> String {
    match value {
        Some(true) => "true".to_string(),
        Some(false) => "false".to_string(),
        None => "unavailable".to_string(),
    }
}
