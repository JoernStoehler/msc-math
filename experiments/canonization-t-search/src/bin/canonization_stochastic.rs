use exp_canonization_t_search::{
    accepted_random_cases, candidates, metrics, score_candidate_metric, RESIDUAL_FAILURE_THRESHOLD,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize)]
struct StochasticReport {
    schema_version: u32,
    implementation: &'static str,
    package: &'static str,
    profile: &'static str,
    command: Vec<String>,
    git_rev: Option<String>,
    git_tree_state: &'static str,
    seed_cases: u64,
    seed_transforms: u64,
    case_count: usize,
    samples_per_case: usize,
    case_source: &'static str,
    case_facet_counts: BTreeMap<usize, usize>,
    residual_failure_threshold: f64,
    metric_note: &'static str,
    transform_family_notes: BTreeMap<&'static str, &'static str>,
    candidates: Vec<&'static str>,
    metrics: Vec<&'static str>,
    candidate_metric_results: Vec<exp_canonization_t_search::CandidateMetricSummary>,
}

fn main() {
    let mut out = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out.push("artifacts/stochastic-rust-summary.json");
    let mut case_count = 24;
    let mut samples_per_case = 4;
    let mut candidate_filter: Option<String> = None;
    let mut metric_filter: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = PathBuf::from(args.next().expect("--out needs a path")),
            "--cases" => {
                case_count = args
                    .next()
                    .expect("--cases needs a value")
                    .parse()
                    .expect("parse --cases")
            }
            "--samples-per-case" => {
                samples_per_case = args
                    .next()
                    .expect("--samples-per-case needs a value")
                    .parse()
                    .expect("parse --samples-per-case")
            }
            "--candidate" => {
                candidate_filter = Some(args.next().expect("--candidate needs a label"))
            }
            "--metric" => metric_filter = Some(args.next().expect("--metric needs a label")),
            other => panic!("unknown argument {other}"),
        }
    }

    let command = std::env::args().collect::<Vec<_>>();
    let cases = accepted_random_cases(case_count, 2026062802);
    let case_facet_counts = facet_counts(&cases);
    let mut rng = ChaCha8Rng::seed_from_u64(2026062803);
    let candidates = candidates::all()
        .into_iter()
        .filter(|candidate| {
            candidate_filter
                .as_deref()
                .map(|label| candidate.label == label)
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    let metrics = metrics::all()
        .into_iter()
        .filter(|metric| {
            metric_filter
                .as_deref()
                .map(|label| metric.label == label)
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    assert!(
        !candidates.is_empty(),
        "candidate filter matched no registered candidates"
    );
    assert!(
        !metrics.is_empty(),
        "metric filter matched no registered metrics"
    );
    let mut candidate_metric_results = Vec::new();
    for candidate in &candidates {
        for metric in &metrics {
            candidate_metric_results.push(score_candidate_metric(
                &cases,
                *candidate,
                *metric,
                samples_per_case,
                &mut rng,
            ));
        }
    }

    let report = StochasticReport {
        schema_version: 2,
        implementation: "rust",
        package: "exp-canonization-t-search",
        profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
        command,
        git_rev: git_rev(),
        git_tree_state: git_tree_state(),
        seed_cases: 2026062802,
        seed_transforms: 2026062803,
        case_count,
        samples_per_case,
        case_source: "symplectic::random::generate_dual_vertices accepted rows",
        case_facet_counts,
        residual_failure_threshold: RESIDUAL_FAILURE_THRESHOLD,
        metric_note:
            "registered metrics are diagnostics, not automatically proved mathematical metrics",
        transform_family_notes: transform_family_notes(),
        candidates: candidates.iter().map(|candidate| candidate.label).collect(),
        metrics: metrics.iter().map(|metric| metric.label).collect(),
        candidate_metric_results,
    };

    fs::create_dir_all(out.parent().expect("output parent")).expect("create output parent");
    fs::write(
        &out,
        serde_json::to_string_pretty(&report).expect("serialize report") + "\n",
    )
    .expect("write report");
    println!("wrote {}", out.display());
}

fn facet_counts(cases: &[exp_canonization_t_search::Case]) -> BTreeMap<usize, usize> {
    let mut counts = BTreeMap::new();
    for case in cases {
        *counts.entry(case.duals.len()).or_insert(0) += 1;
    }
    counts
}

fn transform_family_notes() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("scale", "positive scalar sampled from an exponential range"),
        (
            "translation",
            "interior translation direction with bounded radius",
        ),
        ("facet_permutation", "uniform shuffle of the facet row list"),
        (
            "scale_translation_permutation",
            "composition of scale, translation, and facet permutation",
        ),
        (
            "sampled_block_symplectic",
            "block subgroup diag(A,A^{-T}); useful subgroup test, not full Sp(4)",
        ),
        (
            "sampled_sp4_exp",
            "full-dimensional local Sp(4) exponential sample exp(JH), H symmetric; not Haar-like",
        ),
        (
            "sampled_full_group",
            "composition of scale, translation, facet permutation, and sampled_sp4_exp",
        ),
    ])
}

fn git_rev() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_tree_state() -> &'static str {
    let Ok(output) = Command::new("git")
        .args(["status", "--short", "--untracked-files=normal"])
        .output()
    else {
        return "unknown";
    };
    if !output.status.success() {
        return "unknown";
    }
    if output.stdout.is_empty() {
        "clean"
    } else {
        "dirty"
    }
}
