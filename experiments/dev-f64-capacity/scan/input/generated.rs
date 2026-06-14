use exp_dev_f64_capacity::{generated_f64_cases, ScanCase};

pub(crate) fn generated_cases(samples_per_facet: usize, seed: u64) -> Vec<ScanCase> {
    generated_f64_cases(samples_per_facet, seed)
}
