mod generated;

use exp_dev_f64_capacity::{load_retained_artifact_cases, ScanCase};

#[derive(Clone, Copy, Debug)]
pub(crate) enum InputSource {
    All,
    Generated,
    Artifacts,
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
        ));
    }
    if matches!(
        options.input_source,
        InputSource::All | InputSource::Artifacts
    ) {
        cases.extend(load_retained_artifact_cases(options.max_rows_per_family));
    }
    if !options.family_filter.is_empty() {
        cases.retain(|case| options.family_filter.contains(&case.family));
    }
    if !options.source_id_filter.is_empty() {
        cases.retain(|case| options.source_id_filter.contains(&case.source_id));
    }
    cases
}
