use equal_budget_product_search::chart::{
    iid_base_candidate_attempt, ProductCandidate, ProductChart,
};
use exp_sys_landscape::{dual_vertices_rational_strings, polytope_key};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::f64::consts::TAU;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const ADAPTIVE_BUDGET: usize = 48;
pub const IID_BUDGET: usize = 16;
pub const PACKET_VERSION: &str = "ams-readiness-smoke-v1";
pub const MASTER_SEED: u64 = 202607150101;
pub const MUTATION_KERNEL: &str = "non_invariant_threshold_only_gaussian";
pub const GENERATION_SCHEDULE: &str = "sha256_counter_box_muller_v1";
pub const STOP_ACTION: &str =
    "artifacts_flushed_stop_unrelated_search_independent_validation_required";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub packet_version: String,
    pub master_seed: u64,
    pub replicate: usize,
    pub initial_particles: usize,
    pub levels: usize,
    pub survivors_per_level: usize,
    pub clones_per_level: usize,
    pub mutation_steps_per_clone: usize,
    pub iid_requests: usize,
    pub construction_retry_cap: usize,
    pub abort_wall_time_seconds: u64,
    pub gap_logit_scale: f64,
    pub centered_log_radius_scale: f64,
    pub phase_scale: f64,
    pub tie_rule: String,
    pub clone_assignment: String,
    pub acceptance_rule: String,
    pub factor_exchange_quotiented: bool,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| format!("read config {path:?}: {e}"))?;
        let config: Self =
            serde_json::from_slice(&bytes).map_err(|e| format!("parse config {path:?}: {e}"))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        let fixed = self.packet_version == PACKET_VERSION
            && self.master_seed == MASTER_SEED
            && self.replicate == 0
            && self.initial_particles == 16
            && self.levels == 2
            && self.survivors_per_level == 8
            && self.clones_per_level == 8
            && self.mutation_steps_per_clone == 2
            && self.iid_requests == 16
            && self.construction_retry_cap == 64
            && self.abort_wall_time_seconds == 900
            && self.gap_logit_scale == 0.08
            && self.centered_log_radius_scale == 0.04
            && self.phase_scale == 0.08
            && self.initial_particles
                + self.levels * self.clones_per_level * self.mutation_steps_per_clone
                == ADAPTIVE_BUDGET;
        if !fixed {
            return Err("config differs from the fully frozen readiness smoke".into());
        }
        if self.tie_rule != "sys_desc_candidate_id_asc"
            || self.clone_assignment != "seeded_uniform_with_replacement"
            || self.acceptance_rule != "successful_sys_at_least_frozen_level_threshold"
            || self.factor_exchange_quotiented
        {
            return Err("config changes a frozen policy or claim boundary".into());
        }
        Ok(())
    }

    pub fn identity(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("config serializes");
        sha256_bytes(&bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    Adaptive,
    Iid,
}

impl Arm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Adaptive => "adaptive",
            Self::Iid => "iid",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CandidateIdentity {
    pub packet_version: String,
    pub config_identity: String,
    pub source_revision: String,
    pub parent_candidate_id: Option<String>,
    pub master_seed: u64,
    pub replicate: usize,
    pub arm: Arm,
    pub level: Option<usize>,
    pub clone_index: Option<usize>,
    pub mutation_step: Option<usize>,
    pub base_index: Option<usize>,
    pub construction_attempt: usize,
}

pub fn candidate_id(identity: &CandidateIdentity) -> String {
    let digest = sha256_bytes(candidate_material(identity).as_bytes());
    format!("amsv1-{}", &digest[..24])
}

fn candidate_material(identity: &CandidateIdentity) -> String {
    format!(
        "packet={}\nconfig={}\nsource={}\nparent={}\nseed={}\nreplicate={}\narm={}\nlevel={}\nclone={}\nstep={}\nbase={}\nconstruction={}\n",
        identity.packet_version,
        identity.config_identity,
        identity.source_revision,
        identity.parent_candidate_id.as_deref().unwrap_or("none"),
        identity.master_seed,
        identity.replicate,
        identity.arm.as_str(),
        option_usize(identity.level),
        option_usize(identity.clone_index),
        option_usize(identity.mutation_step),
        option_usize(identity.base_index),
        identity.construction_attempt,
    )
}

fn option_usize(value: Option<usize>) -> String {
    value.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheStatus {
    Miss,
    Hit,
    FailedMiss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationStatus {
    Success,
    TargetUnavailable,
    InvalidOutput,
    ChildFailure,
    Timeout,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetDiagnostics {
    pub iterations: u64,
    pub returned_orbit_count: usize,
    pub action_lower: f64,
    pub action_upper: f64,
    pub exact_admissible_count: usize,
    pub indeterminate_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub diagnostics: TargetDiagnostics,
    /// Full `OrbitSearchResult` for production. Synthetic target-free rows use
    /// `None` and retain an explicit synthetic audit kind instead.
    pub capacity_result: Option<Value>,
    pub audit_kind: String,
}

#[derive(Clone, Debug)]
pub enum OracleOutcome {
    Success(Observation),
    Failure {
        status: EvaluationStatus,
        reason: String,
    },
}

pub struct OracleRequest<'a> {
    pub exact_geometry_key: &'a str,
    pub dual_vertices_f64: &'a [[f64; 4]],
}

pub trait Oracle {
    fn compute(&mut self, request: OracleRequest<'_>, timeout: Duration) -> OracleOutcome;
}

#[derive(Clone, Debug)]
pub struct SyntheticOracle {
    calls: usize,
    force_first_hit: bool,
}

impl SyntheticOracle {
    pub fn new(force_first_hit: bool) -> Self {
        Self {
            calls: 0,
            force_first_hit,
        }
    }

    pub fn calls(&self) -> usize {
        self.calls
    }
}

impl Oracle for SyntheticOracle {
    fn compute(&mut self, request: OracleRequest<'_>, _timeout: Duration) -> OracleOutcome {
        self.calls += 1;
        let digest = Sha256::digest(request.exact_geometry_key.as_bytes());
        let fraction = u64::from_be_bytes(digest[..8].try_into().expect("eight digest bytes"))
            as f64
            / u64::MAX as f64;
        let sys = if self.force_first_hit && self.calls == 1 {
            1.01
        } else {
            0.72 + 0.2 * fraction
        };
        let capacity = 1.0 + fraction;
        OracleOutcome::Success(synthetic_observation(capacity, sys))
    }
}

pub fn synthetic_observation(capacity: f64, sys: f64) -> Observation {
    Observation {
        capacity,
        volume: capacity * capacity / (2.0 * sys),
        sys,
        diagnostics: TargetDiagnostics {
            iterations: 0,
            returned_orbit_count: 0,
            action_lower: capacity,
            action_upper: capacity,
            exact_admissible_count: 0,
            indeterminate_count: 0,
        },
        capacity_result: None,
        audit_kind: "synthetic_formula_fixture".into(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetRow {
    pub global_request_index: usize,
    pub candidate_id: String,
    pub identity: CandidateIdentity,
    pub arm: Arm,
    pub attempt_index: usize,
    pub exact_geometry_key: String,
    pub geometry_identity: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub facet_count: usize,
    pub cache_status: CacheStatus,
    pub evaluation_status: EvaluationStatus,
    pub failure_reason: Option<String>,
    pub capacity: Option<f64>,
    pub volume: Option<f64>,
    pub sys: Option<f64>,
    pub diagnostics: Option<TargetDiagnostics>,
    pub audit_kind: Option<String>,
    pub parent_candidate_id: Option<String>,
    pub root_candidate_id: String,
    pub level_threshold: Option<f64>,
    pub raw_proposed_chart: Option<ProductChart>,
    pub product_chart: ProductChart,
    pub started_monotonic_ms: f64,
    pub wall_time_ms: f64,
    pub cumulative_monotonic_ms: f64,
}

/// Durable charge record synchronized before a cache lookup or target child can
/// be exposed. A normally finalized packet has exactly one matching target row
/// for every ledger row; an externally interrupted packet may end with one
/// unmatched ledger row because evaluation is sequential.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChargedRequestRow {
    pub global_request_index: usize,
    pub candidate_id: String,
    pub identity: CandidateIdentity,
    pub arm: Arm,
    pub attempt_index: usize,
    pub exact_geometry_key: String,
    pub geometry_identity: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub facet_count: usize,
    pub parent_candidate_id: Option<String>,
    pub root_candidate_id: String,
    pub level_threshold: Option<f64>,
    pub raw_proposed_chart: Option<ProductChart>,
    pub product_chart: ProductChart,
    pub charged_monotonic_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheRow {
    pub arm: Arm,
    pub exact_geometry_key: String,
    pub geometry_identity: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub facet_count: usize,
    pub product_chart: ProductChart,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub diagnostics: TargetDiagnostics,
    pub capacity_result: Option<Value>,
    pub audit_kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructionRejectionRow {
    pub candidate_id: String,
    pub identity: CandidateIdentity,
    pub arm: Arm,
    pub reason: String,
    pub parent_candidate_id: Option<String>,
    pub root_candidate_id: Option<String>,
    pub raw_proposed_chart: Option<ProductChart>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MutationTransitionRow {
    pub level: usize,
    pub clone_index: usize,
    pub mutation_step: usize,
    pub frozen_threshold: f64,
    pub state_before_candidate_id: String,
    pub proposal_candidate_id: String,
    pub proposal_sys: Option<f64>,
    pub accepted: bool,
    pub state_after_candidate_id: String,
    pub root_candidate_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelRow {
    pub level: usize,
    pub frozen_threshold: f64,
    pub survivor_candidate_ids: Vec<String>,
    pub survivor_root_candidate_ids: Vec<String>,
    pub clone_parent_candidate_ids: Vec<String>,
    pub post_level_population_candidate_ids: Vec<String>,
    pub post_level_population_geometry_keys: Vec<String>,
    pub post_level_distinct_geometry_keys: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmRunRow {
    pub arm: Arm,
    pub target_attempts: usize,
    pub construction_rejections: usize,
    pub cache_misses: usize,
    pub cache_hits: usize,
    pub failed_misses: usize,
    pub distinct_successful_keys: usize,
    pub started_monotonic_ms: f64,
    pub wall_time_ms: f64,
    pub cumulative_monotonic_ms: f64,
    pub complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopEvent {
    pub event: String,
    pub global_request_index: usize,
    pub arm: Arm,
    pub candidate_id: String,
    pub exact_geometry_key: String,
    pub sys: f64,
    pub action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub git_revision: String,
    pub reviewed_revision: Option<String>,
    pub source_tree_clean: bool,
    pub executable_sha256: String,
    pub cargo_lock_sha256: String,
    pub production_target: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub artifact_kind: String,
    pub run_id: String,
    pub start_unix_ms: u128,
    pub launch_process_id: u32,
    pub artifact_directory: String,
    pub config_identity: String,
    pub exact_config: Config,
    pub source: SourceIdentity,
    pub adaptive_budget: usize,
    pub iid_budget: usize,
    pub target_probability_estimate: Option<f64>,
    pub tail_probability_supported: bool,
    pub mutation_kernel: String,
    pub generation_schedule: String,
    pub factor_exchange_quotiented: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalErrorKind {
    FailedTarget,
    ConstructionExhaustion,
    WallTermination,
    PostLevelDiversityGate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalErrorEvidence {
    pub kind: TerminalErrorKind,
    pub arm: Arm,
    pub global_request_index: Option<usize>,
    pub candidate_id: Option<String>,
    pub evaluation_status: Option<EvaluationStatus>,
    pub failure_reason: Option<String>,
    pub next_schedule_identity: Option<CandidateIdentity>,
    pub level: Option<usize>,
    pub observed_distinct_geometry_keys: Option<usize>,
    pub required_distinct_geometry_keys: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunStatus {
    pub run_id: String,
    pub disposition: String,
    pub error: Option<String>,
    pub terminal_error: Option<TerminalErrorEvidence>,
    pub end_unix_ms: u128,
    pub total_monotonic_wall_time_ms: f64,
    pub adaptive_charged_requests: usize,
    pub iid_charged_requests: usize,
    pub total_charged_requests: usize,
    pub artifact_sha256: BTreeMap<String, String>,
}

pub struct ArtifactSink {
    directory: PathBuf,
    run_id: String,
}

impl ArtifactSink {
    pub fn create(directory: &Path, manifest: &Manifest) -> Result<Self, String> {
        if directory.exists() {
            return Err(format!("artifact directory already exists: {directory:?}"));
        }
        fs::create_dir_all(directory).map_err(|e| format!("create {directory:?}: {e}"))?;
        write_json(directory.join("manifest.json"), manifest)?;
        for name in artifact_jsonl_files() {
            File::create(directory.join(name)).map_err(|e| format!("create {name}: {e}"))?;
        }
        Ok(Self {
            directory: directory.to_owned(),
            run_id: manifest.run_id.clone(),
        })
    }

    pub fn append<T: Serialize>(&self, name: &str, row: &T) -> Result<(), String> {
        let path = self.directory.join(name);
        let file = OpenOptions::new()
            .append(true)
            .open(&path)
            .map_err(|e| format!("open {path:?}: {e}"))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, row).map_err(|e| e.to_string())?;
        writer.write_all(b"\n").map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
        writer
            .get_ref()
            .sync_all()
            .map_err(|e| format!("sync {path:?}: {e}"))
    }

    pub fn stop(&self, event: &StopEvent) -> Result<(), String> {
        write_json(self.directory.join("stop-event.json"), event)
    }

    pub fn finalize(
        &self,
        disposition: &str,
        error: Option<String>,
        terminal_error: Option<TerminalErrorEvidence>,
        started: Instant,
    ) -> Result<RunStatus, String> {
        let (adaptive, iid) = self.charged_counts()?;
        let mut hashes = BTreeMap::new();
        let mut files = vec!["manifest.json"];
        files.extend(artifact_jsonl_files());
        if self.directory.join("stop-event.json").exists() {
            files.push("stop-event.json");
        }
        for name in files {
            hashes.insert(name.to_owned(), file_sha256(&self.directory.join(name))?);
        }
        let elapsed = started.elapsed();
        let end_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("system clock before Unix epoch at finalization: {e}"))?
            .as_millis();
        let status = RunStatus {
            run_id: self.run_id.clone(),
            disposition: disposition.to_owned(),
            error,
            terminal_error,
            end_unix_ms,
            total_monotonic_wall_time_ms: elapsed.as_secs_f64() * 1_000.0,
            adaptive_charged_requests: adaptive,
            iid_charged_requests: iid,
            total_charged_requests: adaptive + iid,
            artifact_sha256: hashes,
        };
        write_json(self.directory.join("run-status.json"), &status)?;
        Ok(status)
    }

    pub fn terminal_error_evidence(&self, error: &str) -> Result<TerminalErrorEvidence, String> {
        if error.starts_with("failed_target:") {
            let rows = read_jsonl_rows::<TargetRow>(
                &self.directory.join("target-evaluations.jsonl"),
                "target row for terminal evidence",
            )?;
            let row = rows
                .last()
                .ok_or("failed-target terminal evidence has no target row")?;
            return Ok(TerminalErrorEvidence {
                kind: TerminalErrorKind::FailedTarget,
                arm: row.arm,
                global_request_index: Some(row.global_request_index),
                candidate_id: Some(row.candidate_id.clone()),
                evaluation_status: Some(row.evaluation_status),
                failure_reason: row.failure_reason.clone(),
                next_schedule_identity: None,
                level: None,
                observed_distinct_geometry_keys: None,
                required_distinct_geometry_keys: None,
            });
        }
        if error.starts_with("construction_exhaustion:") {
            let rows = read_jsonl_rows::<ConstructionRejectionRow>(
                &self.directory.join("construction-rejections.jsonl"),
                "construction rejection for terminal evidence",
            )?;
            let row = rows
                .last()
                .ok_or("construction-exhaustion terminal evidence has no rejection row")?;
            let mut identity = row.identity.clone();
            identity.construction_attempt = 0;
            return Ok(TerminalErrorEvidence {
                kind: TerminalErrorKind::ConstructionExhaustion,
                arm: row.arm,
                global_request_index: None,
                candidate_id: None,
                evaluation_status: None,
                failure_reason: None,
                next_schedule_identity: Some(identity),
                level: None,
                observed_distinct_geometry_keys: None,
                required_distinct_geometry_keys: None,
            });
        }
        if error.starts_with("wall_termination:") {
            let targets = read_jsonl_rows::<TargetRow>(
                &self.directory.join("target-evaluations.jsonl"),
                "target row for wall terminal evidence",
            )?;
            let arm_rows = read_jsonl_rows::<ArmRunRow>(
                &self.directory.join("arm-runs.jsonl"),
                "arm row for wall terminal evidence",
            )?;
            let arm = arm_rows
                .last()
                .map(|row| row.arm)
                .or_else(|| targets.last().map(|row| row.arm))
                .unwrap_or(Arm::Adaptive);
            return Ok(TerminalErrorEvidence {
                kind: TerminalErrorKind::WallTermination,
                arm,
                global_request_index: targets.last().map(|row| row.global_request_index),
                candidate_id: targets.last().map(|row| row.candidate_id.clone()),
                evaluation_status: None,
                failure_reason: None,
                next_schedule_identity: None,
                level: None,
                observed_distinct_geometry_keys: None,
                required_distinct_geometry_keys: None,
            });
        }
        if error.starts_with("post_level_diversity_gate:") {
            let levels = read_jsonl_rows::<LevelRow>(
                &self.directory.join("levels.jsonl"),
                "level row for diversity terminal evidence",
            )?;
            let level = levels
                .last()
                .ok_or("diversity-gate terminal evidence has no completed level row")?;
            let targets = read_jsonl_rows::<TargetRow>(
                &self.directory.join("target-evaluations.jsonl"),
                "target row for diversity terminal evidence",
            )?;
            return Ok(TerminalErrorEvidence {
                kind: TerminalErrorKind::PostLevelDiversityGate,
                arm: Arm::Adaptive,
                global_request_index: targets.last().map(|row| row.global_request_index),
                candidate_id: targets.last().map(|row| row.candidate_id.clone()),
                evaluation_status: None,
                failure_reason: None,
                next_schedule_identity: None,
                level: Some(level.level),
                observed_distinct_geometry_keys: Some(level.post_level_distinct_geometry_keys),
                required_distinct_geometry_keys: Some(8),
            });
        }
        Err(format!(
            "unstructured packet error cannot be finalized: {error}"
        ))
    }

    fn charged_counts(&self) -> Result<(usize, usize), String> {
        let text = fs::read_to_string(self.directory.join("target-evaluations.jsonl"))
            .map_err(|e| format!("read target rows for final status: {e}"))?;
        let mut adaptive = 0;
        let mut iid = 0;
        for line in text.lines() {
            let row: TargetRow = serde_json::from_str(line)
                .map_err(|e| format!("parse target row for final status: {e}"))?;
            match row.arm {
                Arm::Adaptive => adaptive += 1,
                Arm::Iid => iid += 1,
            }
        }
        Ok((adaptive, iid))
    }
}

fn read_jsonl_rows<T: for<'de> Deserialize<'de>>(
    path: &Path,
    label: &str,
) -> Result<Vec<T>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {label}: {e}"))?;
    text.lines()
        .map(|line| serde_json::from_str(line).map_err(|e| format!("parse {label}: {e}")))
        .collect()
}

pub fn artifact_jsonl_files() -> Vec<&'static str> {
    vec![
        "charged-requests.jsonl",
        "target-evaluations.jsonl",
        "cache.jsonl",
        "construction-rejections.jsonl",
        "mutation-transitions.jsonl",
        "levels.jsonl",
        "arm-runs.jsonl",
    ]
}

fn write_json(path: PathBuf, value: &impl Serialize) -> Result<(), String> {
    let file = File::create(&path).map_err(|e| format!("create {path:?}: {e}"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, value).map_err(|e| e.to_string())?;
    writer.write_all(b"\n").map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    writer
        .get_ref()
        .sync_all()
        .map_err(|e| format!("sync {path:?}: {e}"))
}

struct Evaluator {
    arm: Arm,
    attempts: usize,
    cache: HashMap<String, Observation>,
    construction_rejections: usize,
    misses: usize,
    hits: usize,
    failures: usize,
}

struct ConstructedCandidate {
    chart: ProductChart,
    exact_geometry_key: String,
    exact_geometry: Vec<[String; 4]>,
    f64_geometry: Vec<[f64; 4]>,
    facet_count: usize,
}

impl ConstructedCandidate {
    fn production(candidate: ProductCandidate) -> Result<Self, String> {
        let chart = ProductChart::from_polytope(&candidate.polytope)
            .map_err(|_| "valid product did not encode into product chart".to_owned())?;
        let exact_geometry = dual_vertices_rational_strings(&candidate.polytope);
        let f64_geometry = candidate
            .polytope
            .dual_vertices_f64
            .iter()
            .map(|vertex| [vertex[0], vertex[1], vertex[2], vertex[3]])
            .collect();
        Ok(Self {
            chart,
            exact_geometry_key: polytope_key(&candidate.polytope),
            facet_count: candidate.polytope.facet_count(),
            exact_geometry,
            f64_geometry,
        })
    }

    fn bounded_synthetic(chart: ProductChart) -> Result<Self, String> {
        let factors = chart
            .reconstruct_factors()
            .map_err(|error| format!("synthetic chart reconstruction failed: {error:?}"))?;
        let canonical_chart = ProductChart::from_factors(
            &factors.q_normals,
            &factors.q_heights,
            &factors.p_normals,
            &factors.p_heights,
        )
        .map_err(|error| format!("synthetic factors failed canonical chart encoding: {error:?}"))?;
        let mut exact_geometry = Vec::with_capacity(10);
        let mut f64_geometry = Vec::with_capacity(10);
        for (normal, height) in factors.q_normals.iter().zip(&factors.q_heights) {
            let row = [normal[0] / height, normal[1] / height, 0.0, 0.0];
            f64_geometry.push(row);
            exact_geometry.push(row.map(decimal_rational));
        }
        for (normal, height) in factors.p_normals.iter().zip(&factors.p_heights) {
            let row = [0.0, 0.0, normal[0] / height, normal[1] / height];
            f64_geometry.push(row);
            exact_geometry.push(row.map(decimal_rational));
        }
        Ok(Self {
            chart: canonical_chart,
            exact_geometry_key: exact_key(&exact_geometry),
            exact_geometry,
            f64_geometry,
            facet_count: 10,
        })
    }
}

fn decimal_rational(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };
    let decimal = format!("{value:.17}");
    let negative = decimal.starts_with('-');
    let unsigned = decimal.strip_prefix('-').unwrap_or(&decimal);
    let (whole, fractional) = unsigned
        .split_once('.')
        .expect("fixed precision decimal contains a point");
    let fractional = fractional.trim_end_matches('0');
    let denominator = 10_i128.pow(fractional.len() as u32);
    let mut numerator = whole.parse::<i128>().expect("finite fixed decimal integer") * denominator
        + if fractional.is_empty() {
            0
        } else {
            fractional
                .parse::<i128>()
                .expect("finite fixed decimal fraction")
        };
    if negative {
        numerator = -numerator;
    }
    if numerator == 0 {
        return "0/1".into();
    }
    let divisor = gcd_i128(numerator.unsigned_abs(), denominator as u128) as i128;
    let numerator = numerator / divisor;
    let denominator = denominator / divisor;
    format!("{numerator}/{denominator}")
}

fn gcd_i128(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn exact_key(vertices: &[[String; 4]]) -> String {
    vertices
        .iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("|")
}

struct Evaluated {
    global_request_index: usize,
    candidate_id: String,
    observation: Option<Observation>,
    failure_status: Option<EvaluationStatus>,
    failure_reason: Option<String>,
    exact_geometry_key: String,
}

impl Evaluator {
    fn new(arm: Arm) -> Self {
        Self {
            arm,
            attempts: 0,
            cache: HashMap::new(),
            construction_rejections: 0,
            misses: 0,
            hits: 0,
            failures: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn evaluate(
        &mut self,
        oracle: &mut dyn Oracle,
        identity: CandidateIdentity,
        candidate: &ConstructedCandidate,
        raw_proposed_chart: Option<ProductChart>,
        parent_candidate_id: Option<String>,
        root_candidate_id: String,
        level_threshold: Option<f64>,
        sink: &ArtifactSink,
        global_request_count: &mut usize,
        deadline: Instant,
        overall_started: Instant,
    ) -> Result<Evaluated, String> {
        if Instant::now() >= deadline {
            return Err(
                "wall_termination: global deadline elapsed before the next request was charged"
                    .into(),
            );
        }
        self.attempts += 1;
        *global_request_count += 1;
        let global_request_index = *global_request_count;
        let attempt_index = self.attempts;
        let candidate_id = candidate_id(&identity);
        let key = candidate.exact_geometry_key.clone();
        let exact_geometry = candidate.exact_geometry.clone();
        let geometry_identity = geometry_identity(&exact_geometry);
        let started = Instant::now();
        let started_monotonic_ms = overall_started.elapsed().as_secs_f64() * 1_000.0;
        sink.append(
            "charged-requests.jsonl",
            &ChargedRequestRow {
                global_request_index,
                candidate_id: candidate_id.clone(),
                identity: identity.clone(),
                arm: self.arm,
                attempt_index,
                exact_geometry_key: key.clone(),
                geometry_identity: geometry_identity.clone(),
                dual_vertices_rational: exact_geometry.clone(),
                dual_vertices_f64: candidate.f64_geometry.clone(),
                facet_count: candidate.facet_count,
                parent_candidate_id: parent_candidate_id.clone(),
                root_candidate_id: root_candidate_id.clone(),
                level_threshold,
                raw_proposed_chart: raw_proposed_chart.clone(),
                product_chart: candidate.chart.clone(),
                charged_monotonic_ms: overall_started.elapsed().as_secs_f64() * 1_000.0,
            },
        )?;
        let (cache_status, outcome) = if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            (CacheStatus::Hit, OracleOutcome::Success(cached.clone()))
        } else {
            let timeout = deadline.saturating_duration_since(Instant::now());
            let mut outcome = if timeout.is_zero() {
                OracleOutcome::Failure {
                    status: EvaluationStatus::Timeout,
                    reason: "global deadline elapsed before target child started".into(),
                }
            } else {
                oracle.compute(
                    OracleRequest {
                        exact_geometry_key: &key,
                        dual_vertices_f64: &candidate.f64_geometry,
                    },
                    timeout,
                )
            };
            if let OracleOutcome::Success(observation) = &outcome {
                if let Err(reason) = validate_observation(observation) {
                    outcome = OracleOutcome::Failure {
                        status: EvaluationStatus::InvalidOutput,
                        reason,
                    };
                }
            }
            match &outcome {
                OracleOutcome::Success(observation) => {
                    self.misses += 1;
                    self.cache.insert(key.clone(), observation.clone());
                    sink.append(
                        "cache.jsonl",
                        &CacheRow {
                            arm: self.arm,
                            exact_geometry_key: key.clone(),
                            geometry_identity: geometry_identity.clone(),
                            dual_vertices_rational: exact_geometry.clone(),
                            dual_vertices_f64: candidate.f64_geometry.clone(),
                            facet_count: candidate.facet_count,
                            product_chart: candidate.chart.clone(),
                            capacity: observation.capacity,
                            volume: observation.volume,
                            sys: observation.sys,
                            diagnostics: observation.diagnostics.clone(),
                            capacity_result: observation.capacity_result.clone(),
                            audit_kind: observation.audit_kind.clone(),
                        },
                    )?;
                    (CacheStatus::Miss, outcome)
                }
                OracleOutcome::Failure { .. } => {
                    self.failures += 1;
                    (CacheStatus::FailedMiss, outcome)
                }
            }
        };
        let (evaluation_status, failure_reason, observation) = match outcome {
            OracleOutcome::Success(observation) => {
                (EvaluationStatus::Success, None, Some(observation))
            }
            OracleOutcome::Failure { status, reason } => (status, Some(reason), None),
        };
        let row = TargetRow {
            global_request_index,
            candidate_id: candidate_id.clone(),
            identity,
            arm: self.arm,
            attempt_index,
            exact_geometry_key: key.clone(),
            geometry_identity,
            dual_vertices_rational: exact_geometry,
            dual_vertices_f64: candidate.f64_geometry.clone(),
            facet_count: candidate.facet_count,
            cache_status,
            evaluation_status,
            failure_reason: failure_reason.clone(),
            capacity: observation.as_ref().map(|value| value.capacity),
            volume: observation.as_ref().map(|value| value.volume),
            sys: observation.as_ref().map(|value| value.sys),
            diagnostics: observation.as_ref().map(|value| value.diagnostics.clone()),
            audit_kind: observation.as_ref().map(|value| value.audit_kind.clone()),
            parent_candidate_id,
            root_candidate_id,
            level_threshold,
            raw_proposed_chart,
            product_chart: candidate.chart.clone(),
            started_monotonic_ms,
            wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
            cumulative_monotonic_ms: overall_started.elapsed().as_secs_f64() * 1_000.0,
        };
        sink.append("target-evaluations.jsonl", &row)?;
        Ok(Evaluated {
            global_request_index,
            candidate_id,
            observation,
            failure_status: (evaluation_status != EvaluationStatus::Success)
                .then_some(evaluation_status),
            failure_reason,
            exact_geometry_key: key,
        })
    }

    fn row(&self, started: Instant, overall_started: Instant, complete: bool) -> ArmRunRow {
        ArmRunRow {
            arm: self.arm,
            target_attempts: self.attempts,
            construction_rejections: self.construction_rejections,
            cache_misses: self.misses,
            cache_hits: self.hits,
            failed_misses: self.failures,
            distinct_successful_keys: self.cache.len(),
            started_monotonic_ms: started.duration_since(overall_started).as_secs_f64() * 1_000.0,
            wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
            cumulative_monotonic_ms: overall_started.elapsed().as_secs_f64() * 1_000.0,
            complete,
        }
    }
}

fn validate_observation(value: &Observation) -> Result<(), String> {
    if !value.capacity.is_finite()
        || !value.volume.is_finite()
        || !value.sys.is_finite()
        || value.capacity <= 0.0
        || value.volume <= 0.0
        || !value.diagnostics.action_lower.is_finite()
        || !value.diagnostics.action_upper.is_finite()
        || value.diagnostics.action_lower > value.diagnostics.action_upper
    {
        return Err("oracle returned nonfinite or invalid output".into());
    }
    let expected_sys = value.capacity * value.capacity / (2.0 * value.volume);
    let tolerance = 16.0 * f64::EPSILON * value.sys.abs().max(expected_sys.abs()).max(1.0);
    if (value.sys - expected_sys).abs() > tolerance {
        return Err("oracle returned sys inconsistent with capacity and volume".into());
    }
    Ok(())
}

fn geometry_identity(vertices: &[[String; 4]]) -> String {
    let bytes = serde_json::to_vec(vertices).expect("exact geometry serializes");
    sha256_bytes(&bytes)
}

#[derive(Clone)]
struct Particle {
    chart: ProductChart,
    candidate_id: String,
    root_candidate_id: String,
    exact_geometry_key: String,
    sys: f64,
}

#[derive(Clone, Debug)]
pub struct RunOutcome {
    pub stopped: Option<StopEvent>,
    pub adaptive_attempts: usize,
    pub iid_attempts: usize,
}

pub fn run_packet(
    config: &Config,
    source_revision: &str,
    adaptive_oracle: &mut dyn Oracle,
    iid_oracle: &mut dyn Oracle,
    sink: &ArtifactSink,
) -> Result<RunOutcome, String> {
    run_packet_with_base_source(
        config,
        source_revision,
        adaptive_oracle,
        iid_oracle,
        sink,
        BaseSource::ProductionIid,
    )
}

pub fn run_synthetic_packet(
    config: &Config,
    source_revision: &str,
    adaptive_oracle: &mut dyn Oracle,
    iid_oracle: &mut dyn Oracle,
    sink: &ArtifactSink,
) -> Result<RunOutcome, String> {
    run_packet_with_base_source(
        config,
        source_revision,
        adaptive_oracle,
        iid_oracle,
        sink,
        BaseSource::BoundedSynthetic,
    )
}

#[derive(Clone, Copy)]
enum BaseSource {
    ProductionIid,
    BoundedSynthetic,
}

fn run_packet_with_base_source(
    config: &Config,
    source_revision: &str,
    adaptive_oracle: &mut dyn Oracle,
    iid_oracle: &mut dyn Oracle,
    sink: &ArtifactSink,
    base_source: BaseSource,
) -> Result<RunOutcome, String> {
    config.validate()?;
    let overall_started = Instant::now();
    let deadline = overall_started + Duration::from_secs(config.abort_wall_time_seconds);
    let config_identity = config.identity();
    let adaptive_started = Instant::now();
    let mut global_request_count = 0;
    let mut adaptive = Evaluator::new(Arm::Adaptive);
    let mut particles = Vec::with_capacity(config.initial_particles);

    for base_index in 0..config.initial_particles {
        let construction = construct_base(
            config,
            &config_identity,
            source_revision,
            Arm::Adaptive,
            base_index,
            &mut adaptive,
            sink,
            base_source,
        );
        let (identity, candidate, chart) = match construction {
            Ok(value) => value,
            Err(error) => {
                sink.append(
                    "arm-runs.jsonl",
                    &adaptive.row(adaptive_started, overall_started, false),
                )?;
                return Err(error);
            }
        };
        let root = candidate_id(&identity);
        let evaluation = adaptive.evaluate(
            adaptive_oracle,
            identity,
            &candidate,
            None,
            None,
            root.clone(),
            None,
            sink,
            &mut global_request_count,
            deadline,
            overall_started,
        );
        let evaluated = match evaluation {
            Ok(value) => value,
            Err(error) if error.starts_with("wall_termination:") => {
                sink.append(
                    "arm-runs.jsonl",
                    &adaptive.row(adaptive_started, overall_started, false),
                )?;
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        if let Some(stop) = hit_event(Arm::Adaptive, &evaluated) {
            sink.stop(&stop)?;
            sink.append(
                "arm-runs.jsonl",
                &adaptive.row(adaptive_started, overall_started, false),
            )?;
            return Ok(stopped_outcome(stop, &adaptive, None));
        }
        let sys = match require_success(&evaluated) {
            Ok(observation) => observation.sys,
            Err(error) => {
                sink.append(
                    "arm-runs.jsonl",
                    &adaptive.row(adaptive_started, overall_started, false),
                )?;
                return Err(error);
            }
        };
        if let Err(error) = enforce_wall_time(config, overall_started) {
            sink.append(
                "arm-runs.jsonl",
                &adaptive.row(adaptive_started, overall_started, false),
            )?;
            return Err(error);
        }
        particles.push(Particle {
            chart,
            candidate_id: evaluated.candidate_id,
            root_candidate_id: root,
            exact_geometry_key: evaluated.exact_geometry_key,
            sys,
        });
    }

    for level in 0..config.levels {
        particles.sort_by(|left, right| {
            right
                .sys
                .total_cmp(&left.sys)
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
        });
        let survivors = particles[..config.survivors_per_level].to_vec();
        let threshold = survivors.last().expect("fixed nonempty survivor set").sys;
        let assignments: Vec<usize> = (0..config.clones_per_level)
            .map(|clone| clone_assignment(&config_identity, config.master_seed, level, clone, 8))
            .collect();
        let mut clones = Vec::with_capacity(config.clones_per_level);
        for (clone_index, &parent_index) in assignments.iter().enumerate() {
            let mut state = survivors[parent_index].clone();
            for mutation_step in 0..config.mutation_steps_per_clone {
                let before = state.candidate_id.clone();
                let construction = construct_mutation(
                    config,
                    &config_identity,
                    source_revision,
                    level,
                    clone_index,
                    mutation_step,
                    &state,
                    &mut adaptive,
                    sink,
                    base_source,
                );
                let (identity, candidate, proposal_chart, raw_chart) = match construction {
                    Ok(value) => value,
                    Err(error) => {
                        sink.append(
                            "arm-runs.jsonl",
                            &adaptive.row(adaptive_started, overall_started, false),
                        )?;
                        return Err(error);
                    }
                };
                let evaluation = adaptive.evaluate(
                    adaptive_oracle,
                    identity,
                    &candidate,
                    Some(raw_chart),
                    Some(before.clone()),
                    state.root_candidate_id.clone(),
                    Some(threshold),
                    sink,
                    &mut global_request_count,
                    deadline,
                    overall_started,
                );
                let evaluated = match evaluation {
                    Ok(value) => value,
                    Err(error) if error.starts_with("wall_termination:") => {
                        sink.append(
                            "arm-runs.jsonl",
                            &adaptive.row(adaptive_started, overall_started, false),
                        )?;
                        return Err(error);
                    }
                    Err(error) => return Err(error),
                };
                let proposal_sys = evaluated.observation.as_ref().map(|value| value.sys);
                let accepted = proposal_sys.is_some_and(|sys| sys >= threshold);
                if accepted {
                    state.chart = proposal_chart;
                    state.candidate_id = evaluated.candidate_id.clone();
                    state.exact_geometry_key = evaluated.exact_geometry_key.clone();
                    state.sys = proposal_sys.expect("accepted proposal has sys");
                }
                sink.append(
                    "mutation-transitions.jsonl",
                    &MutationTransitionRow {
                        level,
                        clone_index,
                        mutation_step,
                        frozen_threshold: threshold,
                        state_before_candidate_id: before,
                        proposal_candidate_id: evaluated.candidate_id.clone(),
                        proposal_sys,
                        accepted,
                        state_after_candidate_id: state.candidate_id.clone(),
                        root_candidate_id: state.root_candidate_id.clone(),
                    },
                )?;
                if let Some(stop) = hit_event(Arm::Adaptive, &evaluated) {
                    sink.stop(&stop)?;
                    sink.append(
                        "arm-runs.jsonl",
                        &adaptive.row(adaptive_started, overall_started, false),
                    )?;
                    return Ok(stopped_outcome(stop, &adaptive, None));
                }
                if let Err(error) = require_success(&evaluated) {
                    sink.append(
                        "arm-runs.jsonl",
                        &adaptive.row(adaptive_started, overall_started, false),
                    )?;
                    return Err(error);
                }
                if let Err(error) = enforce_wall_time(config, overall_started) {
                    sink.append(
                        "arm-runs.jsonl",
                        &adaptive.row(adaptive_started, overall_started, false),
                    )?;
                    return Err(error);
                }
            }
            clones.push(state);
        }
        particles = survivors.iter().cloned().chain(clones).collect();
        let distinct = particles
            .iter()
            .map(|particle| particle.exact_geometry_key.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len();
        sink.append(
            "levels.jsonl",
            &LevelRow {
                level,
                frozen_threshold: threshold,
                survivor_candidate_ids: survivors
                    .iter()
                    .map(|particle| particle.candidate_id.clone())
                    .collect(),
                survivor_root_candidate_ids: survivors
                    .iter()
                    .map(|particle| particle.root_candidate_id.clone())
                    .collect(),
                clone_parent_candidate_ids: assignments
                    .iter()
                    .map(|&index| survivors[index].candidate_id.clone())
                    .collect(),
                post_level_population_candidate_ids: particles
                    .iter()
                    .map(|particle| particle.candidate_id.clone())
                    .collect(),
                post_level_population_geometry_keys: particles
                    .iter()
                    .map(|particle| particle.exact_geometry_key.clone())
                    .collect(),
                post_level_distinct_geometry_keys: distinct,
            },
        )?;
        if distinct < 8 {
            sink.append(
                "arm-runs.jsonl",
                &adaptive.row(adaptive_started, overall_started, false),
            )?;
            return Err(format!(
                "post_level_diversity_gate: level {level} retained {distinct} distinct states; required 8"
            ));
        }
    }
    if adaptive.attempts != ADAPTIVE_BUDGET {
        return Err(format!(
            "adaptive accounting closed at {}, expected {ADAPTIVE_BUDGET}",
            adaptive.attempts
        ));
    }
    sink.append(
        "arm-runs.jsonl",
        &adaptive.row(adaptive_started, overall_started, true),
    )?;

    let iid_started = Instant::now();
    let mut iid = Evaluator::new(Arm::Iid);
    for base_index in 0..config.iid_requests {
        let construction = construct_base(
            config,
            &config_identity,
            source_revision,
            Arm::Iid,
            base_index,
            &mut iid,
            sink,
            base_source,
        );
        let (identity, candidate, _) = match construction {
            Ok(value) => value,
            Err(error) => {
                sink.append(
                    "arm-runs.jsonl",
                    &iid.row(iid_started, overall_started, false),
                )?;
                return Err(error);
            }
        };
        let root = candidate_id(&identity);
        let evaluation = iid.evaluate(
            iid_oracle,
            identity,
            &candidate,
            None,
            None,
            root,
            None,
            sink,
            &mut global_request_count,
            deadline,
            overall_started,
        );
        let evaluated = match evaluation {
            Ok(value) => value,
            Err(error) if error.starts_with("wall_termination:") => {
                sink.append(
                    "arm-runs.jsonl",
                    &iid.row(iid_started, overall_started, false),
                )?;
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        if let Some(stop) = hit_event(Arm::Iid, &evaluated) {
            sink.stop(&stop)?;
            sink.append(
                "arm-runs.jsonl",
                &iid.row(iid_started, overall_started, false),
            )?;
            return Ok(stopped_outcome(stop, &adaptive, Some(&iid)));
        }
        if let Err(error) = require_success(&evaluated) {
            sink.append(
                "arm-runs.jsonl",
                &iid.row(iid_started, overall_started, false),
            )?;
            return Err(error);
        }
        if let Err(error) = enforce_wall_time(config, overall_started) {
            sink.append(
                "arm-runs.jsonl",
                &iid.row(iid_started, overall_started, false),
            )?;
            return Err(error);
        }
    }
    if iid.attempts != IID_BUDGET {
        return Err(format!(
            "IID accounting closed at {}, expected {IID_BUDGET}",
            iid.attempts
        ));
    }
    sink.append(
        "arm-runs.jsonl",
        &iid.row(iid_started, overall_started, true),
    )?;
    Ok(RunOutcome {
        stopped: None,
        adaptive_attempts: adaptive.attempts,
        iid_attempts: iid.attempts,
    })
}

fn require_success(evaluated: &Evaluated) -> Result<&Observation, String> {
    evaluated.observation.as_ref().ok_or_else(|| {
        format!(
            "failed_target: charged request {} ({:?}) failed: {}",
            evaluated.global_request_index,
            evaluated.failure_status,
            evaluated
                .failure_reason
                .as_deref()
                .unwrap_or("unknown failure")
        )
    })
}

fn stopped_outcome(stop: StopEvent, adaptive: &Evaluator, iid: Option<&Evaluator>) -> RunOutcome {
    RunOutcome {
        stopped: Some(stop),
        adaptive_attempts: adaptive.attempts,
        iid_attempts: iid.map_or(0, |value| value.attempts),
    }
}

fn enforce_wall_time(config: &Config, started: Instant) -> Result<(), String> {
    if started.elapsed() > Duration::from_secs(config.abort_wall_time_seconds) {
        return Err(format!(
            "wall_termination: readiness smoke exceeded frozen {}-second gate",
            config.abort_wall_time_seconds
        ));
    }
    Ok(())
}

fn hit_event(arm: Arm, evaluated: &Evaluated) -> Option<StopEvent> {
    let sys = evaluated.observation.as_ref()?.sys;
    (sys > 1.0).then(|| StopEvent {
        event: "sys_gt_one_flush_and_stop".into(),
        global_request_index: evaluated.global_request_index,
        arm,
        candidate_id: evaluated.candidate_id.clone(),
        exact_geometry_key: evaluated.exact_geometry_key.clone(),
        sys,
        action: STOP_ACTION.into(),
    })
}

#[allow(clippy::too_many_arguments)]
fn construct_base(
    config: &Config,
    config_identity: &str,
    source_revision: &str,
    arm: Arm,
    base_index: usize,
    evaluator: &mut Evaluator,
    sink: &ArtifactSink,
    base_source: BaseSource,
) -> Result<(CandidateIdentity, ConstructedCandidate, ProductChart), String> {
    let arm_seed = derived_u64(&format!(
        "iid-arm-seed\n{}\n{}\n{}\n",
        config.master_seed,
        config_identity,
        arm.as_str()
    ));
    for construction_attempt in 0..config.construction_retry_cap {
        let identity = CandidateIdentity {
            packet_version: config.packet_version.clone(),
            config_identity: config_identity.to_owned(),
            source_revision: source_revision.to_owned(),
            parent_candidate_id: None,
            master_seed: config.master_seed,
            replicate: config.replicate,
            arm,
            level: None,
            clone_index: None,
            mutation_step: None,
            base_index: Some(base_index),
            construction_attempt,
        };
        let construction: Result<ConstructedCandidate, String> = match base_source {
            BaseSource::ProductionIid => iid_base_candidate_attempt(
                arm_seed,
                config.replicate,
                base_index,
                construction_attempt,
            )
            .map_err(|reason| format!("{reason:?}"))
            .and_then(ConstructedCandidate::production),
            BaseSource::BoundedSynthetic => ConstructedCandidate::bounded_synthetic(
                bounded_synthetic_chart(arm_seed, base_index, construction_attempt),
            ),
        };
        match construction {
            Ok(candidate) => {
                let chart = candidate.chart.clone();
                return Ok((identity, candidate, chart));
            }
            Err(reason) => {
                evaluator.construction_rejections += 1;
                sink.append(
                    "construction-rejections.jsonl",
                    &ConstructionRejectionRow {
                        candidate_id: candidate_id(&identity),
                        identity,
                        arm,
                        reason,
                        parent_candidate_id: None,
                        root_candidate_id: None,
                        raw_proposed_chart: None,
                    },
                )?;
            }
        }
    }
    Err(format!(
        "construction_exhaustion: {arm:?} base {base_index} exhausted construction retry cap"
    ))
}

fn bounded_synthetic_chart(
    arm_seed: u64,
    base_index: usize,
    construction_attempt: usize,
) -> ProductChart {
    let mut coordinates = [0.0; 17];
    for (index, value) in coordinates[..16].iter_mut().enumerate() {
        *value = -0.03
            + 0.06
                * deterministic_unit(&format!(
                    "bounded-synthetic-base\n{arm_seed}\n{base_index}\n{construction_attempt}\n{index}\n"
                ));
    }
    coordinates[16] = TAU
        * deterministic_unit(&format!(
            "bounded-synthetic-base\n{arm_seed}\n{base_index}\n{construction_attempt}\n16\n"
        ));
    ProductChart::from_continuous_coordinates(coordinates, false)
}

#[allow(clippy::too_many_arguments)]
fn construct_mutation(
    config: &Config,
    config_identity: &str,
    source_revision: &str,
    level: usize,
    clone_index: usize,
    mutation_step: usize,
    state: &Particle,
    evaluator: &mut Evaluator,
    sink: &ArtifactSink,
    base_source: BaseSource,
) -> Result<
    (
        CandidateIdentity,
        ConstructedCandidate,
        ProductChart,
        ProductChart,
    ),
    String,
> {
    for construction_attempt in 0..config.construction_retry_cap {
        let identity = CandidateIdentity {
            packet_version: config.packet_version.clone(),
            config_identity: config_identity.to_owned(),
            source_revision: source_revision.to_owned(),
            parent_candidate_id: Some(state.candidate_id.clone()),
            master_seed: config.master_seed,
            replicate: config.replicate,
            arm: Arm::Adaptive,
            level: Some(level),
            clone_index: Some(clone_index),
            mutation_step: Some(mutation_step),
            base_index: None,
            construction_attempt,
        };
        let raw_chart = mutate_chart(config, &state.chart, &identity);
        let construction = match base_source {
            BaseSource::ProductionIid => raw_chart
                .reconstruct_candidate()
                .map_err(|reason| format!("{reason:?}"))
                .and_then(ConstructedCandidate::production),
            BaseSource::BoundedSynthetic => {
                ConstructedCandidate::bounded_synthetic(raw_chart.clone())
            }
        };
        match construction {
            Ok(candidate) => {
                let canonical_chart = candidate.chart.clone();
                return Ok((identity, candidate, canonical_chart, raw_chart));
            }
            Err(reason) => {
                evaluator.construction_rejections += 1;
                sink.append(
                    "construction-rejections.jsonl",
                    &ConstructionRejectionRow {
                        candidate_id: candidate_id(&identity),
                        identity,
                        arm: Arm::Adaptive,
                        reason,
                        parent_candidate_id: Some(state.candidate_id.clone()),
                        root_candidate_id: Some(state.root_candidate_id.clone()),
                        raw_proposed_chart: Some(raw_chart),
                    },
                )?;
            }
        }
    }
    Err(format!(
        "construction_exhaustion: adaptive level {level} clone {clone_index} step {mutation_step} exhausted construction retry cap"
    ))
}

fn mutate_chart(
    config: &Config,
    state: &ProductChart,
    identity: &CandidateIdentity,
) -> ProductChart {
    let id = candidate_id(identity);
    let mut coordinates = state.continuous_coordinates();
    for (index, value) in coordinates.iter_mut().enumerate() {
        let scale = match index {
            0..=3 | 8..=11 => config.gap_logit_scale,
            4..=7 | 12..=15 => config.centered_log_radius_scale,
            16 => config.phase_scale,
            _ => unreachable!(),
        };
        *value += scale * standard_normal(&id, index);
    }
    ProductChart::from_continuous_coordinates(coordinates, false)
}

fn clone_assignment(
    config_identity: &str,
    master_seed: u64,
    level: usize,
    clone_index: usize,
    survivor_count: usize,
) -> usize {
    let material = format!(
        "ams-clone-assignment-v1\n{config_identity}\n{master_seed}\n{level}\n{clone_index}\n"
    );
    derived_u64(&material) as usize % survivor_count
}

fn standard_normal(candidate: &str, coordinate: usize) -> f64 {
    let pair = coordinate / 2;
    let digest =
        Sha256::digest(format!("ams-mutation-gaussian-v1\n{candidate}\n{pair}\n").as_bytes());
    let u1 = unit_from_bytes(&digest[..8]);
    let u2 = unit_from_bytes(&digest[8..16]);
    let radius = (-2.0 * u1.ln()).sqrt();
    let angle = TAU * u2;
    if coordinate.is_multiple_of(2) {
        radius * angle.cos()
    } else {
        radius * angle.sin()
    }
}

fn deterministic_unit(material: &str) -> f64 {
    let digest = Sha256::digest(material.as_bytes());
    unit_from_bytes(&digest[..8])
}

fn unit_from_bytes(bytes: &[u8]) -> f64 {
    let bits = u64::from_be_bytes(bytes.try_into().expect("eight digest bytes")) >> 11;
    (bits as f64 + 0.5) / ((1_u64 << 53) as f64)
}

fn derived_u64(material: &str) -> u64 {
    let digest = Sha256::digest(material.as_bytes());
    u64::from_be_bytes(digest[..8].try_into().expect("eight digest bytes"))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn file_sha256(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path:?} for identity: {e}"))?;
    Ok(sha256_bytes(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn config() -> Config {
        Config::from_path(Path::new("resolved-config.json")).unwrap()
    }

    fn manifest(config: &Config) -> Manifest {
        Manifest {
            artifact_kind: "synthetic_target_free".into(),
            run_id: "test-run".into(),
            start_unix_ms: 1,
            launch_process_id: 1,
            artifact_directory: "test-artifacts".into(),
            config_identity: config.identity(),
            exact_config: config.clone(),
            source: SourceIdentity {
                git_revision: "test".into(),
                reviewed_revision: None,
                source_tree_clean: false,
                executable_sha256: "test".into(),
                cargo_lock_sha256: "test".into(),
                production_target: false,
            },
            adaptive_budget: ADAPTIVE_BUDGET,
            iid_budget: IID_BUDGET,
            target_probability_estimate: None,
            tail_probability_supported: false,
            mutation_kernel: MUTATION_KERNEL.into(),
            generation_schedule: GENERATION_SCHEDULE.into(),
            factor_exchange_quotiented: false,
        }
    }

    fn sink(config: &Config) -> (tempfile::TempDir, ArtifactSink) {
        let dir = tempdir().unwrap();
        let artifacts = dir.path().join("artifacts");
        let sink = ArtifactSink::create(&artifacts, &manifest(config)).unwrap();
        (dir, sink)
    }

    #[test]
    fn candidate_identity_and_sha_schedules_are_deterministic() {
        let config = config();
        let identity = CandidateIdentity {
            packet_version: config.packet_version.clone(),
            config_identity: config.identity(),
            source_revision: "test".into(),
            parent_candidate_id: Some("parent".into()),
            master_seed: config.master_seed,
            replicate: 0,
            arm: Arm::Adaptive,
            level: Some(0),
            clone_index: Some(1),
            mutation_step: Some(0),
            base_index: None,
            construction_attempt: 0,
        };
        assert_eq!(candidate_id(&identity), candidate_id(&identity));
        assert_eq!(
            standard_normal(&candidate_id(&identity), 3),
            standard_normal(&candidate_id(&identity), 3)
        );
        assert_eq!(
            clone_assignment(&config.identity(), config.master_seed, 1, 4, 8),
            clone_assignment(&config.identity(), config.master_seed, 1, 4, 8)
        );
    }

    #[test]
    fn budget_is_charged_before_cache_lookup() {
        let config = config();
        let (_dir, sink) = sink(&config);
        let mut evaluator = Evaluator::new(Arm::Adaptive);
        let (identity, candidate, _) = construct_base(
            &config,
            &config.identity(),
            "test",
            Arm::Adaptive,
            0,
            &mut evaluator,
            &sink,
            BaseSource::BoundedSynthetic,
        )
        .unwrap();
        let root = candidate_id(&identity);
        let mut oracle = SyntheticOracle::new(false);
        let mut global = 0;
        let deadline = Instant::now() + Duration::from_secs(10);
        evaluator
            .evaluate(
                &mut oracle,
                identity.clone(),
                &candidate,
                None,
                None,
                root.clone(),
                None,
                &sink,
                &mut global,
                deadline,
                Instant::now(),
            )
            .unwrap();
        evaluator
            .evaluate(
                &mut oracle,
                identity,
                &candidate,
                None,
                None,
                root,
                None,
                &sink,
                &mut global,
                deadline,
                Instant::now(),
            )
            .unwrap();
        assert_eq!(evaluator.attempts, 2);
        assert_eq!(evaluator.misses, 1);
        assert_eq!(evaluator.hits, 1);
        assert_eq!(oracle.calls(), 1);
    }

    struct InvalidOracle;
    impl Oracle for InvalidOracle {
        fn compute(&mut self, _request: OracleRequest<'_>, _timeout: Duration) -> OracleOutcome {
            OracleOutcome::Success(synthetic_observation(1.0, f64::NAN))
        }
    }

    struct UnavailableOracle;
    impl Oracle for UnavailableOracle {
        fn compute(&mut self, _request: OracleRequest<'_>, _timeout: Duration) -> OracleOutcome {
            OracleOutcome::Failure {
                status: EvaluationStatus::TargetUnavailable,
                reason: "synthetic unavailable fixture".into(),
            }
        }
    }

    #[test]
    fn invalid_oracle_output_is_charged_and_retained() {
        let config = config();
        let (dir, sink) = sink(&config);
        let mut evaluator = Evaluator::new(Arm::Adaptive);
        let (identity, candidate, _) = construct_base(
            &config,
            &config.identity(),
            "test",
            Arm::Adaptive,
            0,
            &mut evaluator,
            &sink,
            BaseSource::BoundedSynthetic,
        )
        .unwrap();
        let root = candidate_id(&identity);
        let mut oracle = InvalidOracle;
        let evaluated = evaluator
            .evaluate(
                &mut oracle,
                identity,
                &candidate,
                None,
                None,
                root,
                None,
                &sink,
                &mut 0,
                Instant::now() + Duration::from_secs(1),
                Instant::now(),
            )
            .unwrap();
        assert_eq!(
            evaluated.failure_status,
            Some(EvaluationStatus::InvalidOutput)
        );
        let text =
            fs::read_to_string(dir.path().join("artifacts/target-evaluations.jsonl")).unwrap();
        let row: TargetRow = serde_json::from_str(text.trim()).unwrap();
        assert_eq!(row.evaluation_status, EvaluationStatus::InvalidOutput);
        assert_eq!(row.cache_status, CacheStatus::FailedMiss);
        assert!(!row.failure_reason.unwrap().is_empty());
        assert_eq!(row.dual_vertices_rational.len(), 10);
    }

    #[test]
    fn explicit_oracle_failure_is_charged_and_retains_geometry_and_reason() {
        let config = config();
        let (dir, sink) = sink(&config);
        let mut evaluator = Evaluator::new(Arm::Adaptive);
        let (identity, candidate, _) = construct_base(
            &config,
            &config.identity(),
            "test",
            Arm::Adaptive,
            0,
            &mut evaluator,
            &sink,
            BaseSource::BoundedSynthetic,
        )
        .unwrap();
        let root = candidate_id(&identity);
        let mut oracle = UnavailableOracle;
        let evaluated = evaluator
            .evaluate(
                &mut oracle,
                identity,
                &candidate,
                None,
                None,
                root,
                None,
                &sink,
                &mut 0,
                Instant::now() + Duration::from_secs(1),
                Instant::now(),
            )
            .unwrap();
        assert_eq!(
            evaluated.failure_status,
            Some(EvaluationStatus::TargetUnavailable)
        );
        let text =
            fs::read_to_string(dir.path().join("artifacts/target-evaluations.jsonl")).unwrap();
        let row: TargetRow = serde_json::from_str(text.trim()).unwrap();
        assert_eq!(row.dual_vertices_rational.len(), 10);
        assert_eq!(
            row.failure_reason.as_deref(),
            Some("synthetic unavailable fixture")
        );
    }

    #[test]
    fn synthetic_driver_closes_budget_and_records_post_level_populations() {
        let config = config();
        let (dir, sink) = sink(&config);
        let mut adaptive = SyntheticOracle::new(false);
        let mut iid = SyntheticOracle::new(false);
        let outcome =
            run_synthetic_packet(&config, "test", &mut adaptive, &mut iid, &sink).unwrap();
        assert!(outcome.stopped.is_none());
        assert_eq!(outcome.adaptive_attempts, ADAPTIVE_BUDGET);
        assert_eq!(outcome.iid_attempts, IID_BUDGET);
        let levels = fs::read_to_string(dir.path().join("artifacts/levels.jsonl")).unwrap();
        assert_eq!(levels.lines().count(), 2);
        for line in levels.lines() {
            let row: LevelRow = serde_json::from_str(line).unwrap();
            assert_eq!(row.post_level_population_candidate_ids.len(), 16);
            assert!(row.post_level_distinct_geometry_keys >= 8);
        }
    }

    #[test]
    fn positive_hit_is_flushed_and_stops_all_later_calls() {
        let config = config();
        let (dir, sink) = sink(&config);
        let mut adaptive = SyntheticOracle::new(true);
        let mut iid = SyntheticOracle::new(false);
        let outcome =
            run_synthetic_packet(&config, "test", &mut adaptive, &mut iid, &sink).unwrap();
        assert!(outcome.stopped.is_some());
        assert_eq!(outcome.adaptive_attempts, 1);
        assert_eq!(outcome.iid_attempts, 0);
        assert_eq!(adaptive.calls(), 1);
        assert_eq!(iid.calls(), 0);
        let event: StopEvent = serde_json::from_slice(
            &fs::read(dir.path().join("artifacts/stop-event.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(event.global_request_index, 1);
    }

    #[test]
    fn diversity_gate_terminal_evidence_is_structured_from_final_level() {
        let config = config();
        let (_dir, sink) = sink(&config);
        sink.append(
            "levels.jsonl",
            &LevelRow {
                level: 0,
                frozen_threshold: 0.8,
                survivor_candidate_ids: vec!["s".into(); 8],
                survivor_root_candidate_ids: vec!["r".into(); 8],
                clone_parent_candidate_ids: vec!["s".into(); 8],
                post_level_population_candidate_ids: vec!["p".into(); 16],
                post_level_population_geometry_keys: vec!["k".into(); 16],
                post_level_distinct_geometry_keys: 7,
            },
        )
        .unwrap();
        let evidence = sink
            .terminal_error_evidence(
                "post_level_diversity_gate: level 0 retained 7 distinct states; required 8",
            )
            .unwrap();
        assert!(matches!(
            evidence.kind,
            TerminalErrorKind::PostLevelDiversityGate
        ));
        assert_eq!(evidence.arm, Arm::Adaptive);
        assert_eq!(evidence.level, Some(0));
        assert_eq!(evidence.observed_distinct_geometry_keys, Some(7));
        assert_eq!(evidence.required_distinct_geometry_keys, Some(8));
    }
}
