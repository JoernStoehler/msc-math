use exp_dev_quadratic_program::{generated_f64_cases_with_source_filter, ScanCase};

pub(crate) fn generated_cases(
    samples_per_facet: usize,
    seed: u64,
    source_id_filter: &[String],
) -> Vec<ScanCase> {
    generated_f64_cases_with_source_filter(samples_per_facet, seed, source_id_filter)
}
