mod generated;

use exp_dev_quadratic_program::{load_retained_artifact_cases_filtered, ScanCase};

#[derive(Clone, Copy, Debug)]
pub(crate) enum InputSource {
    All,
    Generated,
    Artifacts,
    EdgeFixtures,
}

#[derive(Clone, Debug)]
pub(crate) struct LoadCaseOptions {
    pub(crate) input_source: InputSource,
    pub(crate) max_rows_per_family: usize,
    pub(crate) generated_samples_per_facet: usize,
    pub(crate) generated_seed: u64,
    pub(crate) family_filter: Vec<String>,
    pub(crate) source_id_filter: Vec<String>,
}

pub(crate) fn load_cases(options: &LoadCaseOptions) -> Vec<ScanCase> {
    let mut cases = Vec::new();
    if matches!(
        options.input_source,
        InputSource::All | InputSource::Generated
    ) {
        cases.extend(generated::generated_cases(
            options.generated_samples_per_facet,
            options.generated_seed,
            &options.source_id_filter,
        ));
    }
    if matches!(
        options.input_source,
        InputSource::All | InputSource::Artifacts
    ) {
        let artifact_row_limit = if options.source_id_filter.is_empty() {
            options.max_rows_per_family
        } else {
            0
        };
        cases.extend(load_retained_artifact_cases_filtered(
            artifact_row_limit,
            &options.family_filter,
            &options.source_id_filter,
        ));
    }
    if matches!(
        options.input_source,
        InputSource::All | InputSource::EdgeFixtures
    ) {
        cases.extend(exp_dev_quadratic_program::edge_fixture_cases());
    }
    if !options.family_filter.is_empty() {
        cases.retain(|case| options.family_filter.contains(&case.family));
    }
    if !options.source_id_filter.is_empty() {
        cases.retain(|case| options.source_id_filter.contains(&case.source_id));
    }
    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hko_source_id_filter_does_not_require_artifact_jsonl_payloads() {
        let hko = exp_dev_quadratic_program::hko_case();
        let cases = load_cases(&LoadCaseOptions {
            input_source: InputSource::Artifacts,
            max_rows_per_family: 1,
            generated_samples_per_facet: 0,
            generated_seed: 0,
            family_filter: Vec::new(),
            source_id_filter: vec![hko.source_id.clone()],
        });

        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].source_id, hko.source_id);
    }

    #[test]
    fn edge_fixture_source_loads_named_edge_cases() {
        let cases = load_cases(&LoadCaseOptions {
            input_source: InputSource::EdgeFixtures,
            max_rows_per_family: 1,
            generated_samples_per_facet: 0,
            generated_seed: 0,
            family_filter: Vec::new(),
            source_id_filter: vec!["edge:duplicate_dual_vertices".to_string()],
        });

        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].source_id, "edge:duplicate_dual_vertices");
    }

    #[test]
    fn all_source_edge_filter_does_not_require_artifact_jsonl_payloads() {
        let cases = load_cases(&LoadCaseOptions {
            input_source: InputSource::All,
            max_rows_per_family: 1,
            generated_samples_per_facet: 0,
            generated_seed: 0,
            family_filter: Vec::new(),
            source_id_filter: vec!["edge:duplicate_dual_vertices".to_string()],
        });

        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].source_id, "edge:duplicate_dual_vertices");
    }
}
