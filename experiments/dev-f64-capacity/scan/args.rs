use std::path::PathBuf;

use crate::input::InputSource;
use exp_dev_f64_capacity::{
    F64CapacityMethod, F64ValidationPolicy, NearRedundantFacetRemovalPolicy,
};

#[derive(Clone, Debug)]
pub(crate) struct Args {
    pub(crate) output: PathBuf,
    pub(crate) max_rows_per_family: usize,
    pub(crate) input_source: InputSource,
    pub(crate) generated_samples_per_facet: usize,
    pub(crate) generated_seed: u64,
    pub(crate) audit_generated: AuditGenerated,
    pub(crate) audit_preprocessed: AuditPreprocessed,
    pub(crate) max_audit_rows: usize,
    pub(crate) validation_policy: F64ValidationPolicy,
    pub(crate) capacity_method: F64CapacityMethod,
    pub(crate) near_redundant_facet_removal: NearRedundantFacetRemovalPolicy,
    pub(crate) near_redundant_facet_removal_delta: f64,
    pub(crate) family_filter: Vec<String>,
    pub(crate) source_id_filter: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AuditGenerated {
    None,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AuditPreprocessed {
    None,
    All,
}

pub(crate) fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut output = PathBuf::from("/tmp/f64-capacity-scan.jsonl");
    let mut max_rows_per_family = 8usize;
    let mut input_source = InputSource::All;
    let mut generated_samples_per_facet = 4usize;
    let mut generated_seed = 0x5eed_f64_u64;
    let mut audit_generated = AuditGenerated::None;
    let mut audit_preprocessed = AuditPreprocessed::None;
    let mut max_audit_rows = 0usize;
    let mut validation_policy = F64ValidationPolicy::LpOriginVertex;
    let mut capacity_method = F64CapacityMethod::ProductBilliardOrHk;
    let mut near_redundant_facet_removal = NearRedundantFacetRemovalPolicy::None;
    let mut near_redundant_facet_removal_delta = 1e-8f64;
    let mut family_filter = Vec::new();
    let mut source_id_filter = Vec::new();
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--output" => {
                output = PathBuf::from(value(&argv, i, "--output"));
                i += 2;
            }
            "--max-rows-per-family" => {
                max_rows_per_family = value(&argv, i, "--max-rows-per-family")
                    .parse()
                    .expect("--max-rows-per-family must be a non-negative integer");
                i += 2;
            }
            "--input-source" => {
                input_source = match value(&argv, i, "--input-source") {
                    "all" => InputSource::All,
                    "generated" => InputSource::Generated,
                    "artifacts" => InputSource::Artifacts,
                    "edge-fixtures" => InputSource::EdgeFixtures,
                    other => {
                        panic!("--input-source must be all, generated, artifacts, or edge-fixtures, got {other}")
                    }
                };
                i += 2;
            }
            "--generated-samples-per-facet" => {
                generated_samples_per_facet = value(&argv, i, "--generated-samples-per-facet")
                    .parse()
                    .expect("--generated-samples-per-facet must be a non-negative integer");
                i += 2;
            }
            "--generated-seed" => {
                generated_seed = value(&argv, i, "--generated-seed")
                    .parse()
                    .expect("--generated-seed must be a u64");
                i += 2;
            }
            "--audit-generated" => {
                audit_generated = match value(&argv, i, "--audit-generated") {
                    "none" => AuditGenerated::None,
                    "all" => AuditGenerated::All,
                    other => panic!("--audit-generated must be none or all, got {other}"),
                };
                i += 2;
            }
            "--audit-preprocessed" => {
                audit_preprocessed = match value(&argv, i, "--audit-preprocessed") {
                    "none" => AuditPreprocessed::None,
                    "all" => AuditPreprocessed::All,
                    other => panic!("--audit-preprocessed must be none or all, got {other}"),
                };
                i += 2;
            }
            "--max-audit-rows" => {
                max_audit_rows = value(&argv, i, "--max-audit-rows")
                    .parse()
                    .expect("--max-audit-rows must be a non-negative integer");
                i += 2;
            }
            "--validation-policy" => {
                validation_policy = match value(&argv, i, "--validation-policy") {
                    "strict" => F64ValidationPolicy::Strict,
                    "lp_origin_vertex" => F64ValidationPolicy::LpOriginVertex,
                    "lp" => F64ValidationPolicy::Lp,
                    other => panic!(
                        "--validation-policy must be strict, lp_origin_vertex, or lp, got {other}"
                    ),
                };
                i += 2;
            }
            "--capacity-method" => {
                capacity_method = match value(&argv, i, "--capacity-method") {
                    "transition_pruned_hk" => F64CapacityMethod::TransitionPrunedHk,
                    "product_billiard_or_hk" => F64CapacityMethod::ProductBilliardOrHk,
                    other => panic!(
                        "--capacity-method must be transition_pruned_hk or product_billiard_or_hk, got {other}"
                    ),
                };
                i += 2;
            }
            "--near-redundant-facet-removal" => {
                near_redundant_facet_removal = parse_near_redundant_facet_removal(
                    value(&argv, i, "--near-redundant-facet-removal"),
                    "--near-redundant-facet-removal",
                );
                i += 2;
            }
            "--near-redundant-facet-removal-delta" => {
                near_redundant_facet_removal_delta = parse_removal_delta(
                    value(&argv, i, "--near-redundant-facet-removal-delta"),
                    "--near-redundant-facet-removal-delta",
                );
                i += 2;
            }
            "--family-filter" => {
                family_filter.extend(
                    value(&argv, i, "--family-filter")
                        .split(',')
                        .filter(|family| !family.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--source-id-filter" => {
                source_id_filter.extend(
                    value(&argv, i, "--source-id-filter")
                        .split(',')
                        .filter(|source_id| !source_id.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        output,
        max_rows_per_family,
        input_source,
        generated_samples_per_facet,
        generated_seed,
        audit_generated,
        audit_preprocessed,
        max_audit_rows,
        validation_policy,
        capacity_method,
        near_redundant_facet_removal,
        near_redundant_facet_removal_delta,
        family_filter,
        source_id_filter,
    }
}

fn parse_near_redundant_facet_removal(value: &str, flag: &str) -> NearRedundantFacetRemovalPolicy {
    match value {
        "none" => NearRedundantFacetRemovalPolicy::None,
        "product" => NearRedundantFacetRemovalPolicy::Product,
        "generic" => NearRedundantFacetRemovalPolicy::Generic,
        other => panic!("{flag} must be none, product, or generic, got {other}"),
    }
}

fn parse_removal_delta(value: &str, flag: &str) -> f64 {
    let delta: f64 = value
        .parse()
        .unwrap_or_else(|_| panic!("{flag} must be a finite non-negative f64"));
    assert!(
        delta.is_finite() && delta >= 0.0,
        "{flag} must be finite and non-negative"
    );
    delta
}

fn value<'a>(argv: &'a [String], i: usize, flag: &str) -> &'a str {
    argv.get(i + 1)
        .map(|s| s.as_str())
        .unwrap_or_else(|| panic!("{flag} requires a value"))
}

fn print_help() {
    println!(
        "Usage: f64-capacity-scan [--output PATH] [--input-source all|generated|artifacts|edge-fixtures] \\\n         [--max-rows-per-family N] [--generated-samples-per-facet N] [--generated-seed U64]\n\
         [--audit-generated none|all] [--audit-preprocessed none|all] [--max-audit-rows N]\n\
         [--validation-policy strict|lp_origin_vertex|lp]\n\
         [--capacity-method transition_pruned_hk|product_billiard_or_hk]\n\
         [--near-redundant-facet-removal none|product|generic]\n\
         [--near-redundant-facet-removal-delta F64]\n\
         [--family-filter FAMILY[,FAMILY...]] [--source-id-filter SOURCE_ID[,SOURCE_ID...]]\n\
         N=0 scans every row in each retained artifact. Generated rows are deterministic attempts.\n\
         --family-filter keeps only the named emitted families after case loading.\n\
         --source-id-filter keeps only exact source id matches after case loading.\n\
         --max-audit-rows 0 means no audit row cap."
    );
}
