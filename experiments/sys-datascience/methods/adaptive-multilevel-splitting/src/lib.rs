use equal_budget_product_search::chart::{
    iid_base_candidate_attempt, ProductCandidate, ProductChart,
};
use exp_sys_landscape::{
    dual_vertices_rational_strings, polytope_key, ExpensiveComputationCache,
    SysLandscapePolytopeCache,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

pub const ADAPTIVE_BUDGET: usize = 48;
pub const IID_BUDGET: usize = 16;
pub const PACKET_VERSION: &str = "ams-readiness-smoke-v1";
pub const MASTER_SEED: u64 = 202607150101;

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
            && self.abort_wall_time_seconds == 600
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
            return Err("config changes a frozen policy rule or quotients factor exchange".into());
        }
        Ok(())
    }

    pub fn identity(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("config serializes");
        format!("{:x}", Sha256::digest(bytes))
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
    let material = format!(
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
    );
    let digest = format!("{:x}", Sha256::digest(material.as_bytes()));
    format!("amsv1-{}", &digest[..24])
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
    Failure,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
}

pub trait Oracle {
    fn compute(
        &mut self,
        exact_geometry_key: &str,
        polytope: Option<&SysLandscapePolytopeCache>,
    ) -> Option<Observation>;
}

#[derive(Default)]
pub struct ProductionOracle {
    cache: ExpensiveComputationCache,
}

impl Oracle for ProductionOracle {
    fn compute(
        &mut self,
        _exact_geometry_key: &str,
        polytope: Option<&SysLandscapePolytopeCache>,
    ) -> Option<Observation> {
        let polytope = polytope?;
        let result = self.cache.compute(polytope)?;
        Some(Observation {
            capacity: result.capacity.min_action,
            volume: result.vol,
            sys: result.sys,
        })
    }
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
    fn compute(
        &mut self,
        exact_geometry_key: &str,
        _polytope: Option<&SysLandscapePolytopeCache>,
    ) -> Option<Observation> {
        self.calls += 1;
        let digest = blake3::hash(exact_geometry_key.as_bytes());
        let fraction =
            u64::from_le_bytes(digest.as_bytes()[..8].try_into().unwrap()) as f64 / u64::MAX as f64;
        let sys = if self.force_first_hit && self.calls == 1 {
            1.01
        } else {
            0.72 + 0.2 * fraction
        };
        let capacity = 1.0 + fraction;
        Some(Observation {
            capacity,
            volume: capacity * capacity / (2.0 * sys),
            sys,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetRow {
    pub candidate_id: String,
    pub identity: CandidateIdentity,
    pub arm: Arm,
    pub attempt_index: usize,
    pub exact_geometry_key: String,
    pub geometry_identity: String,
    pub cache_status: CacheStatus,
    pub evaluation_status: EvaluationStatus,
    pub capacity: Option<f64>,
    pub volume: Option<f64>,
    pub sys: Option<f64>,
    pub parent_candidate_id: Option<String>,
    pub root_candidate_id: String,
    pub level_threshold: Option<f64>,
    pub product_chart: ProductChart,
    pub wall_time_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheRow {
    pub arm: Arm,
    pub exact_geometry_key: String,
    pub geometry_identity: String,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructionRejectionRow {
    pub candidate_id: String,
    pub identity: CandidateIdentity,
    pub arm: Arm,
    pub reason: String,
    pub parent_candidate_id: Option<String>,
    pub root_candidate_id: Option<String>,
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
    pub wall_time_ms: f64,
    pub complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopEvent {
    pub event: String,
    pub arm: Arm,
    pub candidate_id: String,
    pub exact_geometry_key: String,
    pub sys: f64,
    pub action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub git_revision: String,
    pub source_tree_clean: bool,
    pub executable_sha256: String,
    pub cargo_lock_sha256: String,
    pub production_target: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub artifact_kind: String,
    pub config_identity: String,
    pub exact_config: Config,
    pub source: SourceIdentity,
    pub adaptive_budget: usize,
    pub iid_budget: usize,
    pub target_probability_estimate: Option<f64>,
    pub factor_exchange_quotiented: bool,
}

pub struct ArtifactSink {
    directory: PathBuf,
}

impl ArtifactSink {
    pub fn create(directory: &Path, manifest: &Manifest) -> Result<Self, String> {
        if directory.exists() {
            return Err(format!("artifact directory already exists: {directory:?}"));
        }
        fs::create_dir_all(directory).map_err(|e| format!("create {directory:?}: {e}"))?;
        write_json(directory.join("manifest.json"), manifest)?;
        for name in [
            "target-evaluations.jsonl",
            "cache.jsonl",
            "construction-rejections.jsonl",
            "mutation-transitions.jsonl",
            "levels.jsonl",
            "arm-runs.jsonl",
        ] {
            File::create(directory.join(name)).map_err(|e| format!("create {name}: {e}"))?;
        }
        Ok(Self {
            directory: directory.to_owned(),
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
    facet_count: usize,
    production: Option<ProductCandidate>,
}

impl ConstructedCandidate {
    fn production(candidate: ProductCandidate) -> Result<Self, String> {
        let chart = ProductChart::from_polytope(&candidate.polytope)
            .map_err(|_| "valid product did not encode into product chart".to_owned())?;
        let exact_geometry = dual_vertices_rational_strings(&candidate.polytope);
        Ok(Self {
            chart,
            exact_geometry_key: polytope_key(&candidate.polytope),
            facet_count: candidate.polytope.facet_count(),
            exact_geometry,
            production: Some(candidate),
        })
    }

    fn bounded_synthetic(chart: ProductChart) -> Result<Self, String> {
        let factors = chart
            .reconstruct_factors()
            .map_err(|error| format!("synthetic chart reconstruction failed: {error:?}"))?;
        let mut exact_geometry = Vec::with_capacity(10);
        for (normal, height) in factors.q_normals.iter().zip(&factors.q_heights) {
            exact_geometry.push([
                decimal_rational(normal[0] / height),
                decimal_rational(normal[1] / height),
                "0".into(),
                "0".into(),
            ]);
        }
        for (normal, height) in factors.p_normals.iter().zip(&factors.p_heights) {
            exact_geometry.push([
                "0".into(),
                "0".into(),
                decimal_rational(normal[0] / height),
                decimal_rational(normal[1] / height),
            ]);
        }
        Ok(Self {
            chart,
            exact_geometry_key: exact_key(&exact_geometry),
            exact_geometry,
            facet_count: 10,
            production: None,
        })
    }
}

fn decimal_rational(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };
    format!("{value:.17}")
}

fn exact_key(vertices: &[[String; 4]]) -> String {
    vertices
        .iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("|")
}

struct Evaluated {
    candidate_id: String,
    observation: Option<Observation>,
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
        parent_candidate_id: Option<String>,
        root_candidate_id: String,
        level_threshold: Option<f64>,
        sink: &ArtifactSink,
    ) -> Result<Evaluated, String> {
        // The charge intentionally happens before cache lookup.
        self.attempts += 1;
        let attempt_index = self.attempts;
        let candidate_id = candidate_id(&identity);
        let key = candidate.exact_geometry_key.clone();
        let exact_geometry = candidate.exact_geometry.clone();
        let geometry_identity = geometry_identity(&exact_geometry);
        let chart = candidate.chart.clone();
        let started = Instant::now();
        let (cache_status, observation) = if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            (CacheStatus::Hit, Some(cached.clone()))
        } else {
            match oracle.compute(
                &key,
                candidate
                    .production
                    .as_ref()
                    .map(|candidate| &candidate.polytope),
            ) {
                Some(observation) => {
                    validate_observation(&observation)?;
                    self.misses += 1;
                    self.cache.insert(key.clone(), observation.clone());
                    sink.append(
                        "cache.jsonl",
                        &CacheRow {
                            arm: self.arm,
                            exact_geometry_key: key.clone(),
                            geometry_identity: geometry_identity.clone(),
                            dual_vertices_rational: exact_geometry,
                            facet_count: candidate.facet_count,
                            capacity: observation.capacity,
                            volume: observation.volume,
                            sys: observation.sys,
                        },
                    )?;
                    (CacheStatus::Miss, Some(observation))
                }
                None => {
                    self.failures += 1;
                    (CacheStatus::FailedMiss, None)
                }
            }
        };
        let row = TargetRow {
            candidate_id: candidate_id.clone(),
            identity,
            arm: self.arm,
            attempt_index,
            exact_geometry_key: key.clone(),
            geometry_identity,
            cache_status,
            evaluation_status: if observation.is_some() {
                EvaluationStatus::Success
            } else {
                EvaluationStatus::Failure
            },
            capacity: observation.as_ref().map(|value| value.capacity),
            volume: observation.as_ref().map(|value| value.volume),
            sys: observation.as_ref().map(|value| value.sys),
            parent_candidate_id,
            root_candidate_id,
            level_threshold,
            product_chart: chart,
            wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
        };
        sink.append("target-evaluations.jsonl", &row)?;
        Ok(Evaluated {
            candidate_id,
            observation,
            exact_geometry_key: key,
        })
    }

    fn row(&self, started: Instant, complete: bool) -> ArmRunRow {
        ArmRunRow {
            arm: self.arm,
            target_attempts: self.attempts,
            construction_rejections: self.construction_rejections,
            cache_misses: self.misses,
            cache_hits: self.hits,
            failed_misses: self.failures,
            distinct_successful_keys: self.cache.len(),
            wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
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
    {
        return Err("oracle returned an invalid observation".into());
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
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone)]
struct Particle {
    chart: ProductChart,
    candidate_id: String,
    root_candidate_id: String,
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

/// Target-free bounded smoke. It changes only the initial base source to
/// near-regular valid charts; both arms still use the production driver,
/// construction, mutation, charging, cache, genealogy, and stop paths.
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
    let config_identity = config.identity();
    let adaptive_started = Instant::now();
    let mut adaptive = Evaluator::new(Arm::Adaptive);
    let mut particles = Vec::with_capacity(config.initial_particles);

    for base_index in 0..config.initial_particles {
        let (identity, candidate, chart) = construct_base(
            config,
            &config_identity,
            source_revision,
            Arm::Adaptive,
            base_index,
            &mut adaptive,
            sink,
            base_source,
        )?;
        let root = candidate_id(&identity);
        let evaluated = adaptive.evaluate(
            adaptive_oracle,
            identity,
            &candidate,
            None,
            root.clone(),
            None,
            sink,
        )?;
        enforce_wall_time(config, overall_started)?;
        let Some(ref observation) = evaluated.observation else {
            sink.append("arm-runs.jsonl", &adaptive.row(adaptive_started, false))?;
            return Err("adaptive initial target evaluation failed; smoke is incomplete".into());
        };
        if let Some(stop) = hit_event(Arm::Adaptive, &evaluated, observation.sys) {
            sink.stop(&stop)?;
            sink.append("arm-runs.jsonl", &adaptive.row(adaptive_started, false))?;
            return Ok(RunOutcome {
                stopped: Some(stop),
                adaptive_attempts: adaptive.attempts,
                iid_attempts: 0,
            });
        }
        particles.push(Particle {
            chart,
            candidate_id: evaluated.candidate_id,
            root_candidate_id: root,
            sys: observation.sys,
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
        let mut assignment_rng = seeded_rng(&format!(
            "clone-assignment\n{}\n{}\n{}\n",
            config_identity, config.master_seed, level
        ));
        let assignments: Vec<usize> = (0..config.clones_per_level)
            .map(|_| assignment_rng.gen_range(0..survivors.len()))
            .collect();
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
            },
        )?;

        let mut clones = Vec::with_capacity(config.clones_per_level);
        for (clone_index, &parent_index) in assignments.iter().enumerate() {
            let mut state = survivors[parent_index].clone();
            for mutation_step in 0..config.mutation_steps_per_clone {
                let before = state.candidate_id.clone();
                let (identity, candidate, proposal_chart) = construct_mutation(
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
                )?;
                let evaluated = adaptive.evaluate(
                    adaptive_oracle,
                    identity,
                    &candidate,
                    Some(before.clone()),
                    state.root_candidate_id.clone(),
                    Some(threshold),
                    sink,
                )?;
                enforce_wall_time(config, overall_started)?;
                let proposal_sys = evaluated.observation.as_ref().map(|value| value.sys);
                let accepted = proposal_sys.is_some_and(|sys| sys >= threshold);
                if accepted {
                    state.chart = proposal_chart;
                    state.candidate_id = evaluated.candidate_id.clone();
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
                if let Some(observation) = evaluated.observation.as_ref() {
                    if let Some(stop) = hit_event(Arm::Adaptive, &evaluated, observation.sys) {
                        sink.stop(&stop)?;
                        sink.append("arm-runs.jsonl", &adaptive.row(adaptive_started, false))?;
                        return Ok(RunOutcome {
                            stopped: Some(stop),
                            adaptive_attempts: adaptive.attempts,
                            iid_attempts: 0,
                        });
                    }
                }
            }
            clones.push(state);
        }
        particles = survivors.into_iter().chain(clones).collect();
    }
    if adaptive.attempts != ADAPTIVE_BUDGET {
        return Err(format!(
            "adaptive accounting closed at {}, expected {ADAPTIVE_BUDGET}",
            adaptive.attempts
        ));
    }
    sink.append("arm-runs.jsonl", &adaptive.row(adaptive_started, true))?;

    let iid_started = Instant::now();
    let mut iid = Evaluator::new(Arm::Iid);
    for base_index in 0..config.iid_requests {
        let (identity, candidate, _) = construct_base(
            config,
            &config_identity,
            source_revision,
            Arm::Iid,
            base_index,
            &mut iid,
            sink,
            base_source,
        )?;
        let root = candidate_id(&identity);
        let evaluated = iid.evaluate(iid_oracle, identity, &candidate, None, root, None, sink)?;
        enforce_wall_time(config, overall_started)?;
        if let Some(observation) = evaluated.observation.as_ref() {
            if let Some(stop) = hit_event(Arm::Iid, &evaluated, observation.sys) {
                sink.stop(&stop)?;
                sink.append("arm-runs.jsonl", &iid.row(iid_started, false))?;
                return Ok(RunOutcome {
                    stopped: Some(stop),
                    adaptive_attempts: adaptive.attempts,
                    iid_attempts: iid.attempts,
                });
            }
        }
    }
    if iid.attempts != IID_BUDGET {
        return Err(format!(
            "IID accounting closed at {}, expected {IID_BUDGET}",
            iid.attempts
        ));
    }
    sink.append("arm-runs.jsonl", &iid.row(iid_started, true))?;
    Ok(RunOutcome {
        stopped: None,
        adaptive_attempts: adaptive.attempts,
        iid_attempts: iid.attempts,
    })
}

fn enforce_wall_time(config: &Config, started: Instant) -> Result<(), String> {
    if started.elapsed().as_secs() > config.abort_wall_time_seconds {
        return Err(format!(
            "readiness smoke exceeded frozen {}-second abort gate",
            config.abort_wall_time_seconds
        ));
    }
    Ok(())
}

fn hit_event(arm: Arm, evaluated: &Evaluated, sys: f64) -> Option<StopEvent> {
    (sys > 1.0).then(|| StopEvent {
        event: "sys_gt_one_flush_and_stop".into(),
        arm,
        candidate_id: evaluated.candidate_id.clone(),
        exact_geometry_key: evaluated.exact_geometry_key.clone(),
        sys,
        action: "artifacts_flushed_stop_unrelated_search_independent_validation_required".into(),
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
                    },
                )?;
            }
        }
    }
    Err(format!(
        "{arm:?} base {base_index} exhausted construction retry cap"
    ))
}

fn bounded_synthetic_chart(
    arm_seed: u64,
    base_index: usize,
    construction_attempt: usize,
) -> ProductChart {
    let mut rng = seeded_rng(&format!(
        "bounded-synthetic-base\n{arm_seed}\n{base_index}\n{construction_attempt}\n"
    ));
    let mut coordinates = [0.0; 17];
    for value in &mut coordinates[..16] {
        *value = rng.gen_range(-0.03..0.03);
    }
    coordinates[16] = rng.gen_range(0.0..std::f64::consts::TAU);
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
) -> Result<(CandidateIdentity, ConstructedCandidate, ProductChart), String> {
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
        let chart = mutate_chart(config, &state.chart, &identity);
        let construction = match base_source {
            BaseSource::ProductionIid => chart
                .reconstruct_candidate()
                .map_err(|reason| format!("{reason:?}"))
                .and_then(ConstructedCandidate::production),
            BaseSource::BoundedSynthetic => ConstructedCandidate::bounded_synthetic(chart.clone()),
        };
        match construction {
            Ok(candidate) => {
                let canonical_chart = candidate.chart.clone();
                return Ok((identity, candidate, canonical_chart));
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
                    },
                )?;
            }
        }
    }
    Err(format!(
        "adaptive level {level} clone {clone_index} step {mutation_step} exhausted construction retry cap"
    ))
}

fn mutate_chart(
    config: &Config,
    state: &ProductChart,
    identity: &CandidateIdentity,
) -> ProductChart {
    let seed = serde_json::to_vec(identity).expect("identity serializes");
    let mut rng = ChaCha8Rng::from_seed(blake3::derive_key("ams-chart-mutation-v1", &seed));
    let gap = Normal::new(0.0, config.gap_logit_scale).expect("validated positive scale");
    let radius =
        Normal::new(0.0, config.centered_log_radius_scale).expect("validated positive scale");
    let phase = Normal::new(0.0, config.phase_scale).expect("validated positive scale");
    let mut coordinates = state.continuous_coordinates();
    for value in &mut coordinates[..4] {
        *value += gap.sample(&mut rng);
    }
    for value in &mut coordinates[4..8] {
        *value += radius.sample(&mut rng);
    }
    for value in &mut coordinates[8..12] {
        *value += gap.sample(&mut rng);
    }
    for value in &mut coordinates[12..16] {
        *value += radius.sample(&mut rng);
    }
    coordinates[16] += phase.sample(&mut rng);
    ProductChart::from_continuous_coordinates(coordinates, false)
}

fn seeded_rng(material: &str) -> ChaCha8Rng {
    ChaCha8Rng::from_seed(blake3::derive_key(
        "ams-deterministic-stream-v1",
        material.as_bytes(),
    ))
}

fn derived_u64(material: &str) -> u64 {
    let hash = blake3::hash(material.as_bytes());
    u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap())
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
            config_identity: config.identity(),
            exact_config: config.clone(),
            source: SourceIdentity {
                git_revision: "test".into(),
                source_tree_clean: false,
                executable_sha256: "test".into(),
                cargo_lock_sha256: "test".into(),
                production_target: false,
            },
            adaptive_budget: ADAPTIVE_BUDGET,
            iid_budget: IID_BUDGET,
            target_probability_estimate: None,
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
    fn candidate_identity_is_deterministic_and_field_sensitive() {
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
        let mut changed = identity.clone();
        changed.construction_attempt = 1;
        assert_ne!(candidate_id(&identity), candidate_id(&changed));
        changed = identity.clone();
        changed.arm = Arm::Iid;
        assert_ne!(candidate_id(&identity), candidate_id(&changed));
    }

    #[test]
    fn budget_is_charged_before_cache_lookup() {
        let config = config();
        let (_dir, sink) = sink(&config);
        let config_identity = config.identity();
        let mut evaluator = Evaluator::new(Arm::Adaptive);
        let (identity, candidate, _) = construct_base(
            &config,
            &config_identity,
            "test",
            Arm::Adaptive,
            0,
            &mut evaluator,
            &sink,
            BaseSource::ProductionIid,
        )
        .unwrap();
        let root = candidate_id(&identity);
        let mut oracle = SyntheticOracle::new(false);
        evaluator
            .evaluate(
                &mut oracle,
                identity.clone(),
                &candidate,
                None,
                root.clone(),
                None,
                &sink,
            )
            .unwrap();
        evaluator
            .evaluate(&mut oracle, identity, &candidate, None, root, None, &sink)
            .unwrap();
        assert_eq!(evaluator.attempts, 2);
        assert_eq!(evaluator.misses, 1);
        assert_eq!(evaluator.hits, 1);
        assert_eq!(oracle.calls(), 1);
    }

    #[test]
    fn construction_rejection_is_uncharged_and_retained() {
        let config = config();
        let (_dir, sink) = sink(&config);
        let mut evaluator = Evaluator::new(Arm::Adaptive);
        let invalid_chart = ProductChart {
            q_gap_logits: [1000.0, -1000.0, -1000.0, -1000.0],
            q_centered_log_radii: [0.0; 5],
            p_gap_logits: [0.0; 4],
            p_centered_log_radii: [0.0; 5],
            relative_phase: 0.0,
            near_tie: false,
        };
        let rejection = ConstructedCandidate::bounded_synthetic(invalid_chart)
            .err()
            .expect("invalid chart is rejected");
        let identity = CandidateIdentity {
            packet_version: config.packet_version.clone(),
            config_identity: config.identity(),
            source_revision: "test".into(),
            parent_candidate_id: Some("parent".into()),
            master_seed: config.master_seed,
            replicate: 0,
            arm: Arm::Adaptive,
            level: Some(0),
            clone_index: Some(0),
            mutation_step: Some(0),
            base_index: None,
            construction_attempt: 0,
        };
        evaluator.construction_rejections += 1;
        sink.append(
            "construction-rejections.jsonl",
            &ConstructionRejectionRow {
                candidate_id: candidate_id(&identity),
                identity,
                arm: Arm::Adaptive,
                reason: rejection,
                parent_candidate_id: Some("parent".into()),
                root_candidate_id: Some("root".into()),
            },
        )
        .unwrap();
        assert_eq!(evaluator.attempts, 0);
        assert_eq!(evaluator.construction_rejections, 1);
    }

    #[test]
    fn synthetic_driver_closes_budget_and_genealogy() {
        let config = config();
        let (dir, sink) = sink(&config);
        let mut adaptive = SyntheticOracle::new(false);
        let mut iid = SyntheticOracle::new(false);
        let outcome =
            run_synthetic_packet(&config, "test", &mut adaptive, &mut iid, &sink).unwrap();
        assert!(outcome.stopped.is_none());
        assert_eq!(outcome.adaptive_attempts, ADAPTIVE_BUDGET);
        assert_eq!(outcome.iid_attempts, IID_BUDGET);
        let transitions =
            fs::read_to_string(dir.path().join("artifacts/mutation-transitions.jsonl")).unwrap();
        assert_eq!(transitions.lines().count(), 32);
        for line in transitions.lines() {
            let row: MutationTransitionRow = serde_json::from_str(line).unwrap();
            if row.accepted {
                assert_eq!(row.state_after_candidate_id, row.proposal_candidate_id);
                assert!(row.proposal_sys.unwrap() >= row.frozen_threshold);
            } else {
                assert_eq!(row.state_after_candidate_id, row.state_before_candidate_id);
            }
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
        assert!(dir.path().join("artifacts/stop-event.json").is_file());
        let targets =
            fs::read_to_string(dir.path().join("artifacts/target-evaluations.jsonl")).unwrap();
        assert_eq!(targets.lines().count(), 1);
    }
}
