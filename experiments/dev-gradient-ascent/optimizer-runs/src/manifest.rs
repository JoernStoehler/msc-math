use crate::branch_model::{BranchExtensionMode, NormMode, SliceMode};
use crate::evaluator::{EvaluatorConfig, GeometryMode};
use crate::quotient::flatten;
use crate::schedule::DistanceScheduleSpec;
use crate::schema::SourcePoint;
use nalgebra::Vector4;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub study_id: String,
    #[serde(default)]
    pub dataset_role: DatasetRole,
    pub source: PathBuf,
    pub facet_count: Option<usize>,
    #[serde(default)]
    pub facet_counts: Vec<usize>,
    pub starts: StartSelection,
    /// Hard safety cap on charged full-`sys` evaluations per run.
    pub budget: usize,
    /// Optional primary budget for equal-compute comparisons.
    ///
    /// The runner checks this before starting another proposal. A proposal
    /// already started may overshoot, and the overshoot is recorded.
    #[serde(default)]
    pub compute_budget_ms: Option<f64>,
    /// Optional early-stop threshold for the best fully evaluated `sys`.
    #[serde(default)]
    pub stop_sys_threshold: Option<f64>,
    #[serde(default)]
    pub charge_initial: bool,
    pub master_seed: u64,
    #[serde(default = "default_parallelism")]
    pub parallelism: usize,
    #[serde(default)]
    pub evaluator: EvaluatorConfig,
    pub algorithms: Vec<AlgorithmSpec>,
    #[serde(default)]
    pub checkpoints: Vec<usize>,
    pub probe_start_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetRole {
    #[default]
    Development,
    Tuning,
    HeldOut,
}

fn default_parallelism() -> usize {
    1
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StartSelection {
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(default)]
    pub prefixes: Vec<String>,
    #[serde(default)]
    pub offset_per_prefix: usize,
    pub per_prefix: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CmaScaleMode {
    #[default]
    PerCoordinate,
    NormalizedRmsDistance,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CandidateAcceptancePolicy {
    #[default]
    AnyUsable,
    ImprovingOnly,
    BoundedIncumbentDrawdown {
        max_relative_drawdown: f64,
        #[serde(default)]
        return_to_incumbent_on_rejection: bool,
    },
}

impl CandidateAcceptancePolicy {
    fn validate(&self, id: &str) -> Result<(), String> {
        if let Self::BoundedIncumbentDrawdown {
            max_relative_drawdown,
            ..
        } = self
        {
            if !max_relative_drawdown.is_finite()
                || *max_relative_drawdown < 0.0
                || *max_relative_drawdown >= 1.0
            {
                return Err(format!("{id}: invalid maximum incumbent drawdown"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DirectionalTransitionPolicy {
    #[default]
    None,
    Unfiltered,
    AnchorActionWindow {
        relative_window: f64,
    },
    UnfilteredAboveDistance {
        minimum_normalized_distance: f64,
    },
}

impl DirectionalTransitionPolicy {
    fn validate(&self, id: &str) -> Result<(), String> {
        if let Self::AnchorActionWindow { relative_window } = self {
            if !relative_window.is_finite() || *relative_window < 0.0 {
                return Err(format!(
                    "{id}: directional transition action window must be finite and nonnegative"
                ));
            }
        }
        if let Self::UnfilteredAboveDistance {
            minimum_normalized_distance,
        } = self
        {
            if !minimum_normalized_distance.is_finite() || *minimum_normalized_distance <= 0.0 {
                return Err(format!(
                    "{id}: directional transition distance threshold must be positive and finite"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AlgorithmSpec {
    OnlineSource {
        id: String,
        batch_size: usize,
        #[serde(default)]
        facet_count: Option<usize>,
        height_min: f64,
        height_max: f64,
    },
    IidSource {
        id: String,
        batch_size: usize,
    },
    DirectSearch {
        id: String,
        initial_radius: f64,
        expansion: f64,
        contraction: f64,
        minimum_radius: f64,
    },
    CmaEs {
        id: String,
        initial_sigma: f64,
        population_size: usize,
        minimum_sigma: f64,
        maximum_sigma: f64,
        #[serde(default)]
        scale_mode: CmaScaleMode,
    },
    LiteralGradient {
        id: String,
        rate: f64,
    },
    SafeguardedGradient {
        id: String,
        schedule: DistanceScheduleSpec,
        slice_mode: SliceMode,
    },
    GapModel {
        id: String,
        candidate_window_relative: f64,
        extension_mode: BranchExtensionMode,
        extension_reachability_scale: f64,
        schedule: DistanceScheduleSpec,
        slice_mode: SliceMode,
        norm_mode: NormMode,
        require_positive_prediction: bool,
    },
    NonlinearCandidateCma {
        id: String,
        candidate_window_relative: f64,
        inner_generations: usize,
        population_size: usize,
        initial_sigma: f64,
        minimum_sigma: f64,
        maximum_sigma: f64,
    },
    NonlinearCandidateRelinearized {
        id: String,
        candidate_window_relative: f64,
        #[serde(default)]
        beta_allowance: Option<f64>,
        #[serde(default = "default_history_depth")]
        history_depth: usize,
        #[serde(default)]
        acceptance: CandidateAcceptancePolicy,
        #[serde(default)]
        directional_transition: DirectionalTransitionPolicy,
        #[serde(default)]
        remember_validated_winner: bool,
        inner_steps: usize,
        initial_distance: f64,
        expansion: f64,
        contraction: f64,
        minimum_distance: f64,
    },
}

impl AlgorithmSpec {
    pub fn id(&self) -> &str {
        match self {
            Self::OnlineSource { id, .. }
            | Self::IidSource { id, .. }
            | Self::DirectSearch { id, .. }
            | Self::CmaEs { id, .. }
            | Self::LiteralGradient { id, .. }
            | Self::SafeguardedGradient { id, .. }
            | Self::GapModel { id, .. }
            | Self::NonlinearCandidateCma { id, .. }
            | Self::NonlinearCandidateRelinearized { id, .. } => id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::OnlineSource { .. } => "online_source",
            Self::IidSource { .. } => "iid_source",
            Self::DirectSearch { .. } => "direct_search",
            Self::CmaEs { .. } => "cma_es",
            Self::LiteralGradient { .. } => "literal_gradient",
            Self::SafeguardedGradient { .. } => "safeguarded_gradient",
            Self::GapModel {
                extension_mode: BranchExtensionMode::None,
                ..
            } => "candidate_window_gap_model",
            Self::GapModel {
                extension_mode: BranchExtensionMode::TransitionBlockedAdmissible,
                ..
            } => "transition_blocked_gap_model",
            Self::NonlinearCandidateCma { .. } => "nonlinear_candidate_cma",
            Self::NonlinearCandidateRelinearized { .. } => "nonlinear_candidate_relinearized",
        }
    }

    fn seed_group(&self) -> &str {
        match self {
            Self::OnlineSource { .. } => "online_source",
            Self::IidSource { .. } => "iid_source",
            Self::CmaEs { .. } => "cma_es",
            Self::NonlinearCandidateCma { .. } => "nonlinear_candidate_cma",
            Self::NonlinearCandidateRelinearized { .. } => "nonlinear_candidate_relinearized",
            _ => self.id(),
        }
    }

    fn validate(&self) -> Result<(), String> {
        match self {
            Self::OnlineSource {
                id,
                batch_size,
                facet_count,
                height_min,
                height_max,
            } => {
                validate_id(id)?;
                if *batch_size == 0 {
                    return Err(format!("{id}: batch_size must be positive"));
                }
                if facet_count.is_some_and(|count| count < 5) {
                    return Err(format!("{id}: facet_count must be at least five when set"));
                }
                if !height_min.is_finite()
                    || !height_max.is_finite()
                    || *height_min <= 0.0
                    || *height_min >= *height_max
                {
                    return Err(format!("{id}: invalid height interval"));
                }
            }
            Self::IidSource { id, batch_size } => {
                validate_id(id)?;
                if *batch_size == 0 {
                    return Err(format!("{id}: batch_size must be positive"));
                }
            }
            Self::DirectSearch {
                id,
                initial_radius,
                expansion,
                contraction,
                minimum_radius,
            } => {
                validate_id(id)?;
                if !initial_radius.is_finite() || *initial_radius <= 0.0 {
                    return Err(format!("{id}: initial_radius must be positive and finite"));
                }
                if !expansion.is_finite() || *expansion <= 1.0 {
                    return Err(format!("{id}: expansion must exceed one"));
                }
                if !contraction.is_finite() || !(0.0..1.0).contains(contraction) {
                    return Err(format!("{id}: contraction must lie in (0,1)"));
                }
                if !minimum_radius.is_finite()
                    || *minimum_radius <= 0.0
                    || *minimum_radius >= *initial_radius
                {
                    return Err(format!(
                        "{id}: minimum_radius must be positive and below initial_radius"
                    ));
                }
            }
            Self::CmaEs {
                id,
                initial_sigma,
                population_size,
                minimum_sigma,
                maximum_sigma,
                ..
            } => {
                validate_id(id)?;
                if !initial_sigma.is_finite() || *initial_sigma <= 0.0 {
                    return Err(format!("{id}: initial_sigma must be positive and finite"));
                }
                if *population_size < 4 {
                    return Err(format!("{id}: population_size must be at least four"));
                }
                if !minimum_sigma.is_finite()
                    || !maximum_sigma.is_finite()
                    || *minimum_sigma <= 0.0
                    || *minimum_sigma >= *initial_sigma
                    || *maximum_sigma <= *initial_sigma
                {
                    return Err(format!("{id}: invalid sigma bounds"));
                }
            }
            Self::LiteralGradient { id, rate } => {
                validate_id(id)?;
                if !rate.is_finite() || *rate <= 0.0 {
                    return Err(format!("{id}: rate must be positive and finite"));
                }
            }
            Self::SafeguardedGradient {
                id,
                schedule,
                slice_mode: _,
            }
            | Self::GapModel { id, schedule, .. } => {
                validate_id(id)?;
                schedule.validate(id)?;
                if let Self::GapModel {
                    candidate_window_relative,
                    extension_reachability_scale,
                    ..
                } = self
                {
                    if !candidate_window_relative.is_finite()
                        || *candidate_window_relative < 0.0
                        || !extension_reachability_scale.is_finite()
                        || *extension_reachability_scale < 0.0
                    {
                        return Err(format!("{id}: invalid gap-model search parameters"));
                    }
                }
            }
            Self::NonlinearCandidateCma {
                id,
                candidate_window_relative,
                inner_generations,
                population_size,
                initial_sigma,
                minimum_sigma,
                maximum_sigma,
            } => {
                validate_id(id)?;
                if !candidate_window_relative.is_finite() || *candidate_window_relative < 0.0 {
                    return Err(format!("{id}: invalid candidate window"));
                }
                if *inner_generations == 0 {
                    return Err(format!("{id}: inner_generations must be positive"));
                }
                if *population_size < 4 {
                    return Err(format!("{id}: population_size must be at least four"));
                }
                if !initial_sigma.is_finite()
                    || !minimum_sigma.is_finite()
                    || !maximum_sigma.is_finite()
                    || *minimum_sigma <= 0.0
                    || *minimum_sigma >= *initial_sigma
                    || *maximum_sigma <= *initial_sigma
                {
                    return Err(format!("{id}: invalid sigma bounds"));
                }
            }
            Self::NonlinearCandidateRelinearized {
                id,
                candidate_window_relative,
                beta_allowance,
                history_depth,
                acceptance,
                directional_transition,
                remember_validated_winner: _,
                inner_steps,
                initial_distance,
                expansion,
                contraction,
                minimum_distance,
            } => {
                validate_id(id)?;
                if !candidate_window_relative.is_finite() || *candidate_window_relative < 0.0 {
                    return Err(format!("{id}: invalid candidate window"));
                }
                if beta_allowance.is_some_and(|value| !value.is_finite() || value < 0.0) {
                    return Err(format!("{id}: invalid beta allowance"));
                }
                if *history_depth == 0 {
                    return Err(format!("{id}: history_depth must be positive"));
                }
                acceptance.validate(id)?;
                directional_transition.validate(id)?;
                if *inner_steps == 0 {
                    return Err(format!("{id}: inner_steps must be positive"));
                }
                if !initial_distance.is_finite()
                    || !minimum_distance.is_finite()
                    || *minimum_distance <= 0.0
                    || *minimum_distance >= *initial_distance
                    || !expansion.is_finite()
                    || *expansion <= 1.0
                    || !contraction.is_finite()
                    || !(0.0..1.0).contains(contraction)
                {
                    return Err(format!("{id}: invalid distance policy"));
                }
            }
        }
        Ok(())
    }
}

fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
    {
        return Err(format!("invalid algorithm id {id:?}"));
    }
    Ok(())
}

fn default_history_depth() -> usize {
    1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolvedPlan {
    pub schema_version: u32,
    pub study_id: String,
    #[serde(default)]
    pub dataset_role: DatasetRole,
    pub source: String,
    pub source_pool_count: usize,
    pub source_pool_hash: String,
    pub facet_counts: Vec<usize>,
    pub starts: Vec<SourcePoint>,
    pub budget: usize,
    pub compute_budget_ms: Option<f64>,
    pub stop_sys_threshold: Option<f64>,
    pub charge_initial: bool,
    pub master_seed: u64,
    pub parallelism: usize,
    pub evaluator: EvaluatorConfig,
    pub algorithms: Vec<AlgorithmSpec>,
    pub checkpoints: Vec<usize>,
    pub probe_start_count: Option<usize>,
    pub runs: Vec<ResolvedRun>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolvedRun {
    pub run_id: String,
    pub start_id: String,
    pub algorithm_id: String,
    pub algorithm_kind: String,
    pub seed: u64,
}

#[derive(Deserialize)]
struct SourceRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    sys: Option<f64>,
}

pub fn load_and_resolve(manifest_path: &Path) -> Result<(ResolvedPlan, Vec<SourcePoint>), String> {
    let manifest_file = File::open(manifest_path)
        .map_err(|error| format!("open manifest {}: {error}", manifest_path.display()))?;
    let manifest: Manifest = serde_json::from_reader(manifest_file)
        .map_err(|error| format!("parse manifest {}: {error}", manifest_path.display()))?;
    validate_manifest(&manifest)?;
    let source_path = resolve_source_path(manifest_path, &manifest.source);
    let facet_counts = resolved_facet_counts(&manifest)?;
    let source_pool = load_source(&source_path, &facet_counts)?;
    let starts = resolve_starts(&manifest.starts, &source_pool)?;
    for facet_count in &facet_counts {
        if !starts.iter().any(|start| start.facet_count == *facet_count) {
            return Err(format!("start selection contains no F{facet_count} points"));
        }
    }
    if manifest
        .probe_start_count
        .is_some_and(|count| count == 0 || count > starts.len())
    {
        return Err(format!(
            "probe_start_count must lie in 1..={} for the resolved starts",
            starts.len()
        ));
    }
    let source_pool_hash = hash_json(&source_pool)?;
    let mut runs = Vec::with_capacity(starts.len() * manifest.algorithms.len());
    for start in &starts {
        for algorithm in &manifest.algorithms {
            let seed = derive_seed(manifest.master_seed, &start.name, algorithm.seed_group());
            runs.push(ResolvedRun {
                run_id: format!("{}--{}", sanitize(&start.name), sanitize(algorithm.id())),
                start_id: start.name.clone(),
                algorithm_id: algorithm.id().to_string(),
                algorithm_kind: algorithm.kind().to_string(),
                seed,
            });
        }
    }
    let mut execution_rng = ChaCha8Rng::seed_from_u64(manifest.master_seed ^ 0x72bf_9d31_05a4_c8e7);
    runs.shuffle(&mut execution_rng);
    Ok((
        ResolvedPlan {
            schema_version: manifest.schema_version,
            study_id: manifest.study_id,
            dataset_role: manifest.dataset_role,
            source: source_path.display().to_string(),
            source_pool_count: source_pool.len(),
            source_pool_hash,
            facet_counts,
            starts,
            budget: manifest.budget,
            compute_budget_ms: manifest.compute_budget_ms,
            stop_sys_threshold: manifest.stop_sys_threshold,
            charge_initial: manifest.charge_initial,
            master_seed: manifest.master_seed,
            parallelism: manifest.parallelism,
            evaluator: manifest.evaluator,
            algorithms: manifest.algorithms,
            checkpoints: manifest.checkpoints,
            probe_start_count: manifest.probe_start_count,
            runs,
        },
        source_pool,
    ))
}

fn validate_manifest(manifest: &Manifest) -> Result<(), String> {
    if manifest.schema_version != 1 {
        return Err(format!(
            "unsupported manifest schema version {}",
            manifest.schema_version
        ));
    }
    if manifest.study_id.is_empty() {
        return Err("study_id must not be empty".to_string());
    }
    resolved_facet_counts(manifest)?;
    if manifest.budget == 0 {
        return Err("budget must be positive".to_string());
    }
    if manifest
        .compute_budget_ms
        .is_some_and(|value| !value.is_finite() || value <= 0.0)
    {
        return Err("compute_budget_ms must be positive and finite".to_string());
    }
    if manifest
        .stop_sys_threshold
        .is_some_and(|value| !value.is_finite())
    {
        return Err("stop_sys_threshold must be finite".to_string());
    }
    if manifest.parallelism == 0 {
        return Err("parallelism must be positive".to_string());
    }
    if manifest.evaluator.geometry_mode == GeometryMode::F64 {
        return Err(
            "geometry_mode=f64 belongs to the archived heuristic evaluator; \
             the clean runner requires geometry_mode=exact"
                .to_string(),
        );
    }
    if manifest.algorithms.is_empty() {
        return Err("at least one algorithm is required".to_string());
    }
    let mut ids = HashSet::new();
    for algorithm in &manifest.algorithms {
        algorithm.validate()?;
        if !ids.insert(algorithm.id()) {
            return Err(format!("duplicate algorithm id {}", algorithm.id()));
        }
    }
    if manifest.starts.ids.is_empty() && manifest.starts.prefixes.is_empty() {
        return Err("starts must contain ids or prefixes".to_string());
    }
    if manifest
        .checkpoints
        .iter()
        .any(|checkpoint| *checkpoint == 0 || *checkpoint > manifest.budget)
    {
        return Err("checkpoints must lie in 1..=budget".to_string());
    }
    Ok(())
}

fn resolve_source_path(manifest_path: &Path, configured: &Path) -> PathBuf {
    if configured.is_absolute() || configured.exists() {
        configured.to_path_buf()
    } else {
        manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(configured)
    }
}

fn resolved_facet_counts(manifest: &Manifest) -> Result<Vec<usize>, String> {
    let mut counts = manifest.facet_counts.clone();
    if let Some(count) = manifest.facet_count {
        counts.push(count);
    }
    counts.sort_unstable();
    counts.dedup();
    if counts.is_empty() || counts.iter().any(|count| *count < 5) {
        return Err("facet_count/facet_counts must specify values at least five".to_string());
    }
    Ok(counts)
}

fn load_source(path: &Path, facet_counts: &[usize]) -> Result<Vec<SourcePoint>, String> {
    let file =
        File::open(path).map_err(|error| format!("open source {}: {error}", path.display()))?;
    let mut result = Vec::new();
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line
            .map_err(|error| format!("read {} line {}: {error}", path.display(), line_index + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let row: SourceRow = serde_json::from_str(&line).map_err(|error| {
            format!("parse {} line {}: {error}", path.display(), line_index + 1)
        })?;
        if !facet_counts.contains(&row.facet_count) {
            continue;
        }
        if row.dual_vertices.len() != row.facet_count
            || row
                .dual_vertices
                .iter()
                .flatten()
                .any(|coordinate| !coordinate.is_finite())
        {
            return Err(format!("malformed source point {}", row.name));
        }
        let duals = row
            .dual_vertices
            .iter()
            .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
            .collect::<Vec<_>>();
        result.push(SourcePoint {
            name: row.name,
            facet_count: row.facet_count,
            dual_flat: flatten(&duals),
            source_sys: row.sys,
        });
    }
    if result.is_empty() {
        return Err(format!(
            "source {} has no rows for facet counts {:?}",
            path.display(),
            facet_counts
        ));
    }
    let unique = result
        .iter()
        .map(|point| &point.name)
        .collect::<HashSet<_>>();
    if unique.len() != result.len() {
        return Err(format!("source {} has duplicate names", path.display()));
    }
    Ok(result)
}

fn resolve_starts(
    selection: &StartSelection,
    source_pool: &[SourcePoint],
) -> Result<Vec<SourcePoint>, String> {
    let by_name = source_pool
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<HashMap<_, _>>();
    let mut selected = Vec::new();
    for id in &selection.ids {
        selected.push(
            by_name
                .get(id.as_str())
                .ok_or_else(|| format!("start {id} is absent from source"))?
                .to_owned()
                .clone(),
        );
    }
    for prefix in &selection.prefixes {
        let mut matches = source_pool
            .iter()
            .filter(|point| point.name.starts_with(prefix))
            .collect::<Vec<_>>();
        matches.sort_by(|left, right| {
            natural_suffix_key(&left.name).cmp(&natural_suffix_key(&right.name))
        });
        let available = matches.len().saturating_sub(selection.offset_per_prefix);
        let take = selection.per_prefix.unwrap_or(available);
        if available < take {
            return Err(format!(
                "prefix {prefix:?} has only {available} rows after offset, need {take}"
            ));
        }
        selected.extend(
            matches
                .into_iter()
                .skip(selection.offset_per_prefix)
                .take(take)
                .cloned(),
        );
    }
    let mut names = HashSet::new();
    selected.retain(|point| names.insert(point.name.clone()));
    if selected.is_empty() {
        return Err("start selection resolved to no points".to_string());
    }
    Ok(selected)
}

fn natural_suffix_key(name: &str) -> (String, u64) {
    let split = name.rfind('_').unwrap_or(name.len());
    let suffix = name
        .get(split + 1..)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(u64::MAX);
    (name[..split].to_string(), suffix)
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn derive_seed(master: u64, start: &str, algorithm: &str) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"sys-optimizer-study-run-seed-v1");
    hasher.update(&master.to_le_bytes());
    hasher.update(start.as_bytes());
    hasher.update(&[0]);
    hasher.update(algorithm.as_bytes());
    let bytes = hasher.finalize();
    u64::from_le_bytes(bytes.as_bytes()[..8].try_into().expect("eight-byte slice"))
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{AlgorithmSpec, CandidateAcceptancePolicy};

    #[test]
    fn omitted_candidate_acceptance_preserves_legacy_any_usable_behavior() {
        let policy: CandidateAcceptancePolicy =
            serde_json::from_str(r#"{"kind":"any_usable"}"#).expect("valid policy");
        assert!(matches!(policy, CandidateAcceptancePolicy::AnyUsable));
        assert!(matches!(
            CandidateAcceptancePolicy::default(),
            CandidateAcceptancePolicy::AnyUsable
        ));
    }

    #[test]
    fn cma_variants_share_random_stream_group() {
        let left = AlgorithmSpec::CmaEs {
            id: "left".to_string(),
            initial_sigma: 0.01,
            population_size: 8,
            minimum_sigma: 1.0e-6,
            maximum_sigma: 1.0,
            scale_mode: super::CmaScaleMode::NormalizedRmsDistance,
        };
        let right = AlgorithmSpec::CmaEs {
            id: "right".to_string(),
            initial_sigma: 0.1,
            population_size: 16,
            minimum_sigma: 1.0e-6,
            maximum_sigma: 1.0,
            scale_mode: super::CmaScaleMode::NormalizedRmsDistance,
        };
        assert_eq!(left.seed_group(), right.seed_group());
    }
}
