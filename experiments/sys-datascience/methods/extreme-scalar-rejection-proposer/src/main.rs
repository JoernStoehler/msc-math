#[path = "../../../prepare/features_face_symplectic.rs"]
mod features_face_symplectic;
#[allow(dead_code)]
#[path = "../../../prepare/features_helpers.rs"]
mod features_helpers;
#[allow(dead_code)]
#[path = "../../../prepare/features_skeleton.rs"]
mod features_skeleton;

use euclidean_polytopes::{
    edges_from_vertex_facet_incidence, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    two_faces_from_vertex_facet_incidence, vertex_facets_from_vertex_facet_incidence,
};
use exp_sys_landscape::{capacity_billiard, poly_id_from_dual_vertices, SysLandscapePolytopeCache};
use nalgebra::{DMatrix, Matrix4, SymmetricEigen, Vector2, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::geom::polygon::{polygon_area, random_polygon_2d};
use symplectic::{classify_facets_from_dual_vertices, systolic_ratio};

const PRODUCT_PAIRS: &[(usize, usize)] = &[
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (4, 4),
    (4, 5),
    (4, 6),
    (5, 5),
    (5, 6),
    (6, 6),
];
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const DEFAULT_SELECTION_FEATURE: ScalarFeature = ScalarFeature::RidgeSympAreaSumOverVolumeSqrt;
const DEFAULT_SELECTION_DIRECTION: SelectionDirection = SelectionDirection::Low;
const DEFAULT_CHUNK_ROWS: usize = 1000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    All,
    Geometry,
    Features,
    Selection,
    Sys,
    Reports,
}

impl Stage {
    fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Geometry => "geometry",
            Self::Features => "features",
            Self::Selection => "selection",
            Self::Sys => "sys",
            Self::Reports => "reports",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectionDirection {
    Low,
    High,
}

impl SelectionDirection {
    fn parse(value: &str) -> Self {
        match value {
            "low" => Self::Low,
            "high" => Self::High,
            other => panic!("unknown selection direction {other}; expected low|high"),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::High => "high",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScalarFeature {
    VertexCovarianceRho,
    FacetCount,
    VertexCount,
    EdgeCount,
    RidgeCount,
    Volume,
    SimpleVertexFraction,
    EdgeDensity,
    VertexIncidentFacetsMean,
    VertexIncidentFacetsStd,
    VertexIncidentFacetsMin,
    VertexIncidentFacetsMax,
    VertexDegreeMean,
    VertexDegreeStd,
    VertexDegreeMin,
    VertexDegreeMax,
    RidgeSizeMean,
    RidgeSizeStd,
    RidgeSizeMin,
    RidgeSizeMax,
    FacetVertexCountMean,
    FacetVertexCountStd,
    FacetVertexCountMin,
    FacetVertexCountMax,
    FacetNeighborCountMean,
    FacetNeighborCountStd,
    FacetNeighborCountMin,
    FacetNeighborCountMax,
    RidgeSympAreaOrderedFraction,
    RidgeSympAreaMeanOverVolumeSqrt,
    RidgeSympAreaStdOverVolumeSqrt,
    RidgeSympAreaMinOverVolumeSqrt,
    RidgeSympAreaMaxOverVolumeSqrt,
    RidgeSympAreaQ25OverVolumeSqrt,
    RidgeSympAreaMedianOverVolumeSqrt,
    RidgeSympAreaQ75OverVolumeSqrt,
    RidgeSympAreaQ90OverVolumeSqrt,
    RidgeSympAreaQ95OverVolumeSqrt,
    RidgeSympAreaSumOverVolumeSqrt,
    RidgeSympAreaMaxShare,
    RidgeSympAreaTop3Share,
    RidgeSympAreaLe1em3OverVolumeSqrtFraction,
    RidgeSympAreaLe1em2OverVolumeSqrtFraction,
    RidgeSympAreaLe1em1OverVolumeSqrtFraction,
    RidgeSympAreaEntropy,
    RidgeSympAreaEffectiveFaceCount,
    RidgeSympAreaNormalizedEntropy,
}

const SCALAR_FEATURES: &[ScalarFeature] = &[
    ScalarFeature::VertexCovarianceRho,
    ScalarFeature::FacetCount,
    ScalarFeature::VertexCount,
    ScalarFeature::EdgeCount,
    ScalarFeature::RidgeCount,
    ScalarFeature::Volume,
    ScalarFeature::SimpleVertexFraction,
    ScalarFeature::EdgeDensity,
    ScalarFeature::VertexIncidentFacetsMean,
    ScalarFeature::VertexIncidentFacetsStd,
    ScalarFeature::VertexIncidentFacetsMin,
    ScalarFeature::VertexIncidentFacetsMax,
    ScalarFeature::VertexDegreeMean,
    ScalarFeature::VertexDegreeStd,
    ScalarFeature::VertexDegreeMin,
    ScalarFeature::VertexDegreeMax,
    ScalarFeature::RidgeSizeMean,
    ScalarFeature::RidgeSizeStd,
    ScalarFeature::RidgeSizeMin,
    ScalarFeature::RidgeSizeMax,
    ScalarFeature::FacetVertexCountMean,
    ScalarFeature::FacetVertexCountStd,
    ScalarFeature::FacetVertexCountMin,
    ScalarFeature::FacetVertexCountMax,
    ScalarFeature::FacetNeighborCountMean,
    ScalarFeature::FacetNeighborCountStd,
    ScalarFeature::FacetNeighborCountMin,
    ScalarFeature::FacetNeighborCountMax,
    ScalarFeature::RidgeSympAreaOrderedFraction,
    ScalarFeature::RidgeSympAreaMeanOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaStdOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaMinOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaMaxOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaQ25OverVolumeSqrt,
    ScalarFeature::RidgeSympAreaMedianOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaQ75OverVolumeSqrt,
    ScalarFeature::RidgeSympAreaQ90OverVolumeSqrt,
    ScalarFeature::RidgeSympAreaQ95OverVolumeSqrt,
    ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
    ScalarFeature::RidgeSympAreaMaxShare,
    ScalarFeature::RidgeSympAreaTop3Share,
    ScalarFeature::RidgeSympAreaLe1em3OverVolumeSqrtFraction,
    ScalarFeature::RidgeSympAreaLe1em2OverVolumeSqrtFraction,
    ScalarFeature::RidgeSympAreaLe1em1OverVolumeSqrtFraction,
    ScalarFeature::RidgeSympAreaEntropy,
    ScalarFeature::RidgeSympAreaEffectiveFaceCount,
    ScalarFeature::RidgeSympAreaNormalizedEntropy,
];

impl ScalarFeature {
    fn parse(value: &str) -> Self {
        for feature in SCALAR_FEATURES {
            if feature.name() == value {
                return *feature;
            }
        }
        panic!(
            "unknown selection feature {value}; allowed features: {}",
            allowed_scalar_feature_names()
        );
    }

    fn name(self) -> &'static str {
        match self {
            Self::VertexCovarianceRho => "vertex_covariance_rho",
            Self::FacetCount => "facet_count",
            Self::VertexCount => "vertex_count",
            Self::EdgeCount => "edge_count",
            Self::RidgeCount => "ridge_count",
            Self::Volume => "volume",
            Self::SimpleVertexFraction => "simple_vertex_fraction",
            Self::EdgeDensity => "edge_density",
            Self::VertexIncidentFacetsMean => "vertex_incident_facets_mean",
            Self::VertexIncidentFacetsStd => "vertex_incident_facets_std",
            Self::VertexIncidentFacetsMin => "vertex_incident_facets_min",
            Self::VertexIncidentFacetsMax => "vertex_incident_facets_max",
            Self::VertexDegreeMean => "vertex_degree_mean",
            Self::VertexDegreeStd => "vertex_degree_std",
            Self::VertexDegreeMin => "vertex_degree_min",
            Self::VertexDegreeMax => "vertex_degree_max",
            Self::RidgeSizeMean => "ridge_size_mean",
            Self::RidgeSizeStd => "ridge_size_std",
            Self::RidgeSizeMin => "ridge_size_min",
            Self::RidgeSizeMax => "ridge_size_max",
            Self::FacetVertexCountMean => "facet_vertex_count_mean",
            Self::FacetVertexCountStd => "facet_vertex_count_std",
            Self::FacetVertexCountMin => "facet_vertex_count_min",
            Self::FacetVertexCountMax => "facet_vertex_count_max",
            Self::FacetNeighborCountMean => "facet_neighbor_count_mean",
            Self::FacetNeighborCountStd => "facet_neighbor_count_std",
            Self::FacetNeighborCountMin => "facet_neighbor_count_min",
            Self::FacetNeighborCountMax => "facet_neighbor_count_max",
            Self::RidgeSympAreaOrderedFraction => "ridge_symp_area_ordered_fraction",
            Self::RidgeSympAreaMeanOverVolumeSqrt => "ridge_symp_area_mean_over_volume_sqrt",
            Self::RidgeSympAreaStdOverVolumeSqrt => "ridge_symp_area_std_over_volume_sqrt",
            Self::RidgeSympAreaMinOverVolumeSqrt => "ridge_symp_area_min_over_volume_sqrt",
            Self::RidgeSympAreaMaxOverVolumeSqrt => "ridge_symp_area_max_over_volume_sqrt",
            Self::RidgeSympAreaQ25OverVolumeSqrt => "ridge_symp_area_q25_over_volume_sqrt",
            Self::RidgeSympAreaMedianOverVolumeSqrt => "ridge_symp_area_median_over_volume_sqrt",
            Self::RidgeSympAreaQ75OverVolumeSqrt => "ridge_symp_area_q75_over_volume_sqrt",
            Self::RidgeSympAreaQ90OverVolumeSqrt => "ridge_symp_area_q90_over_volume_sqrt",
            Self::RidgeSympAreaQ95OverVolumeSqrt => "ridge_symp_area_q95_over_volume_sqrt",
            Self::RidgeSympAreaSumOverVolumeSqrt => "ridge_symp_area_sum_over_volume_sqrt",
            Self::RidgeSympAreaMaxShare => "ridge_symp_area_max_share",
            Self::RidgeSympAreaTop3Share => "ridge_symp_area_top3_share",
            Self::RidgeSympAreaLe1em3OverVolumeSqrtFraction => {
                "ridge_symp_area_le_1em3_over_volume_sqrt_fraction"
            }
            Self::RidgeSympAreaLe1em2OverVolumeSqrtFraction => {
                "ridge_symp_area_le_1em2_over_volume_sqrt_fraction"
            }
            Self::RidgeSympAreaLe1em1OverVolumeSqrtFraction => {
                "ridge_symp_area_le_1em1_over_volume_sqrt_fraction"
            }
            Self::RidgeSympAreaEntropy => "ridge_symp_area_entropy",
            Self::RidgeSympAreaEffectiveFaceCount => "ridge_symp_area_effective_face_count",
            Self::RidgeSympAreaNormalizedEntropy => "ridge_symp_area_normalized_entropy",
        }
    }

    fn value(self, row: &CandidateFeatureRow) -> f64 {
        match self {
            Self::VertexCovarianceRho => row.vertex_covariance_rho.unwrap_or(f64::NAN),
            Self::FacetCount => row.facet_count as f64,
            Self::VertexCount => row.vertex_count as f64,
            Self::EdgeCount => row.edge_count as f64,
            Self::RidgeCount => row.ridge_count as f64,
            Self::Volume => row.volume,
            Self::SimpleVertexFraction => row.simple_vertex_fraction,
            Self::EdgeDensity => row.edge_density,
            Self::VertexIncidentFacetsMean => row.vertex_incident_facets_mean,
            Self::VertexIncidentFacetsStd => row.vertex_incident_facets_std,
            Self::VertexIncidentFacetsMin => row.vertex_incident_facets_min,
            Self::VertexIncidentFacetsMax => row.vertex_incident_facets_max,
            Self::VertexDegreeMean => row.vertex_degree_mean,
            Self::VertexDegreeStd => row.vertex_degree_std,
            Self::VertexDegreeMin => row.vertex_degree_min,
            Self::VertexDegreeMax => row.vertex_degree_max,
            Self::RidgeSizeMean => row.ridge_size_mean,
            Self::RidgeSizeStd => row.ridge_size_std,
            Self::RidgeSizeMin => row.ridge_size_min,
            Self::RidgeSizeMax => row.ridge_size_max,
            Self::FacetVertexCountMean => row.facet_vertex_count_mean,
            Self::FacetVertexCountStd => row.facet_vertex_count_std,
            Self::FacetVertexCountMin => row.facet_vertex_count_min,
            Self::FacetVertexCountMax => row.facet_vertex_count_max,
            Self::FacetNeighborCountMean => row.facet_neighbor_count_mean,
            Self::FacetNeighborCountStd => row.facet_neighbor_count_std,
            Self::FacetNeighborCountMin => row.facet_neighbor_count_min,
            Self::FacetNeighborCountMax => row.facet_neighbor_count_max,
            Self::RidgeSympAreaOrderedFraction => row.ridge_symp_area_ordered_fraction,
            Self::RidgeSympAreaMeanOverVolumeSqrt => row.ridge_symp_area_mean_over_volume_sqrt,
            Self::RidgeSympAreaStdOverVolumeSqrt => row.ridge_symp_area_std_over_volume_sqrt,
            Self::RidgeSympAreaMinOverVolumeSqrt => row.ridge_symp_area_min_over_volume_sqrt,
            Self::RidgeSympAreaMaxOverVolumeSqrt => row.ridge_symp_area_max_over_volume_sqrt,
            Self::RidgeSympAreaQ25OverVolumeSqrt => row.ridge_symp_area_q25_over_volume_sqrt,
            Self::RidgeSympAreaMedianOverVolumeSqrt => row.ridge_symp_area_median_over_volume_sqrt,
            Self::RidgeSympAreaQ75OverVolumeSqrt => row.ridge_symp_area_q75_over_volume_sqrt,
            Self::RidgeSympAreaQ90OverVolumeSqrt => row.ridge_symp_area_q90_over_volume_sqrt,
            Self::RidgeSympAreaQ95OverVolumeSqrt => row.ridge_symp_area_q95_over_volume_sqrt,
            Self::RidgeSympAreaSumOverVolumeSqrt => row.ridge_symp_area_sum_over_volume_sqrt,
            Self::RidgeSympAreaMaxShare => row.ridge_symp_area_max_share,
            Self::RidgeSympAreaTop3Share => row.ridge_symp_area_top3_share,
            Self::RidgeSympAreaLe1em3OverVolumeSqrtFraction => {
                row.ridge_symp_area_le_1em3_over_volume_sqrt_fraction
            }
            Self::RidgeSympAreaLe1em2OverVolumeSqrtFraction => {
                row.ridge_symp_area_le_1em2_over_volume_sqrt_fraction
            }
            Self::RidgeSympAreaLe1em1OverVolumeSqrtFraction => {
                row.ridge_symp_area_le_1em1_over_volume_sqrt_fraction
            }
            Self::RidgeSympAreaEntropy => row.ridge_symp_area_entropy,
            Self::RidgeSympAreaEffectiveFaceCount => row.ridge_symp_area_effective_face_count,
            Self::RidgeSympAreaNormalizedEntropy => row.ridge_symp_area_normalized_entropy,
        }
    }
}

fn allowed_scalar_feature_names() -> String {
    SCALAR_FEATURES
        .iter()
        .map(|feature| feature.name())
        .collect::<Vec<_>>()
        .join(",")
}

fn default_selection_feature_string() -> String {
    DEFAULT_SELECTION_FEATURE.name().to_string()
}

fn default_selection_direction_string() -> String {
    DEFAULT_SELECTION_DIRECTION.as_str().to_string()
}

fn default_baseline_policy_string() -> String {
    "per_selection_matched".to_string()
}

fn available_scalar_feature_names() -> Vec<String> {
    SCALAR_FEATURES
        .iter()
        .map(|feature| feature.name().to_string())
        .collect()
}

fn run_metadata(baseline_policy: &str) -> RunMetadata {
    RunMetadata {
        geometry_generator: GeometryGeneratorMetadata {
            schema: "sys-datascience.extreme-scalar-rejection-proposer.candidate-geometry.v2"
                .to_string(),
            generator: "random_lagrangian_product_from_2d_h_representations".to_string(),
            h_min: H_MIN,
            h_max: H_MAX,
            product_pairs: PRODUCT_PAIRS.iter().map(|&(k, m)| [k, m]).collect(),
            candidate_id_key:
                "random-product:seed{seed}:{k}x{m}:h0p8_1p2:sample{sample_index}".to_string(),
        },
        feature_schema: FeatureSchemaMetadata {
            schema: "sys-datascience.extreme-scalar-rejection-proposer.candidate-feature.v3"
                .to_string(),
            available_scalar_features: available_scalar_feature_names(),
            source_cache_fields: vec![
                "vertices".to_string(),
                "vertex_facet_incidence".to_string(),
                "volume".to_string(),
            ],
        },
        selection_semantics:
            "selection reads the frozen candidate-feature-table.jsonl and writes selected-candidates-before-sys.jsonl plus selection-plan.json"
                .to_string(),
        baseline_policy: baseline_policy.to_string(),
        sys_cache: SysCacheMetadata {
            schema: "sys-datascience.extreme-scalar-rejection-proposer.evaluated-target.v2"
                .to_string(),
            reuse_key: "candidate_id; poly_id is recorded for human compatibility checks".to_string(),
            reuse_contract: "sys-evaluation-cache.jsonl is append/resume by candidate_id and is intentionally reusable across selection configs that name the same candidate ids".to_string(),
        },
        cache_lifecycle:
            "artifact-producing stages overwrite their own files except sys, which appends missing candidate ids; cleanup is manual with rm -rf <out-dir>"
                .to_string(),
        report_semantics:
            "reports read frozen selection-plan.json and selected-candidates-before-sys.jsonl instead of recomputing selection semantics from current CLI/config arguments"
                .to_string(),
    }
}

#[derive(Clone, Debug)]
struct SelectionRule {
    feature: ScalarFeature,
    direction: SelectionDirection,
    global_top: Vec<usize>,
    per_bucket_top: Vec<usize>,
    percentile_cutoffs: Vec<f64>,
}

impl SelectionRule {
    fn single(
        feature: ScalarFeature,
        direction: SelectionDirection,
        global_top: &[usize],
        per_bucket_top: &[usize],
        percentile_cutoffs: &[f64],
    ) -> Self {
        Self {
            feature,
            direction,
            global_top: global_top.to_vec(),
            per_bucket_top: per_bucket_top.to_vec(),
            percentile_cutoffs: percentile_cutoffs.to_vec(),
        }
    }

    fn label(&self, use_legacy_default_alias: bool) -> String {
        if use_legacy_default_alias
            && self.feature == DEFAULT_SELECTION_FEATURE
            && self.direction == DEFAULT_SELECTION_DIRECTION
        {
            return "low_sum".to_string();
        }
        format!("{}_{}", self.direction.as_str(), self.feature.name())
    }

    fn value(&self, row: &CandidateFeatureRow) -> f64 {
        self.feature.value(row)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SelectionRuleValue {
    selection_feature: String,
    selection_direction: String,
    selection_feature_value: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SelectionRuleReport {
    selection_feature: String,
    selection_direction: String,
    global_top: Vec<usize>,
    per_bucket_top: Vec<usize>,
    percentile_cutoffs: Vec<f64>,
}

#[derive(Clone, Debug)]
struct PerBucketCascade {
    primary_feature: ScalarFeature,
    primary_direction: SelectionDirection,
    primary_fraction: f64,
    secondary_feature: ScalarFeature,
    secondary_direction: SelectionDirection,
    secondary_fraction: f64,
    emit_stage_1_comparator: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PerBucketCascadeReport {
    scope: String,
    rounding: String,
    primary_feature: String,
    primary_direction: String,
    primary_fraction: f64,
    secondary_feature: String,
    secondary_direction: String,
    secondary_fraction: f64,
    emit_stage_1_comparator: bool,
}

impl PerBucketCascade {
    fn report(&self) -> PerBucketCascadeReport {
        PerBucketCascadeReport {
            scope: "actual_bucket_id".to_string(),
            rounding: "ceil_min_one".to_string(),
            primary_feature: self.primary_feature.name().to_string(),
            primary_direction: self.primary_direction.as_str().to_string(),
            primary_fraction: self.primary_fraction,
            secondary_feature: self.secondary_feature.name().to_string(),
            secondary_direction: self.secondary_direction.as_str().to_string(),
            secondary_fraction: self.secondary_fraction,
            emit_stage_1_comparator: self.emit_stage_1_comparator,
        }
    }

    fn rules(&self) -> Vec<SelectionRule> {
        vec![
            SelectionRule::single(self.primary_feature, self.primary_direction, &[], &[], &[]),
            SelectionRule::single(
                self.secondary_feature,
                self.secondary_direction,
                &[],
                &[],
                &[],
            ),
        ]
    }
}

#[derive(Clone)]
struct Args {
    config_path: Option<PathBuf>,
    stage: Stage,
    out_dir: PathBuf,
    seed: u64,
    candidates_per_bucket: usize,
    limit_total: Option<usize>,
    baseline_replicates: usize,
    global_top: Vec<usize>,
    per_bucket_top: Vec<usize>,
    percentile_cutoffs: Vec<f64>,
    selection_feature: ScalarFeature,
    selection_direction: SelectionDirection,
    rule_set: String,
    baseline_policy: String,
    selection_rules: Vec<SelectionRule>,
    per_bucket_cascade: Option<PerBucketCascade>,
    frozen_covariance_validation: Option<FrozenCovarianceValidation>,
    use_legacy_default_rule_alias: bool,
    jobs: usize,
    chunk_rows: usize,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RunConfigFile {
    stage: Option<String>,
    out_dir: Option<PathBuf>,
    seed: Option<u64>,
    candidates_per_bucket: Option<usize>,
    limit_total: Option<usize>,
    baseline_policy: Option<String>,
    baseline_replicates: Option<usize>,
    selection: Option<SelectionConfigFile>,
    jobs: Option<usize>,
    chunk_rows: Option<usize>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionConfigFile {
    rule_set: Option<String>,
    selection_feature: Option<String>,
    selection_direction: Option<String>,
    selection_rules: Option<Vec<SelectionRuleConfigFile>>,
    global_top: Option<Vec<usize>>,
    per_bucket_top: Option<Vec<usize>>,
    percentile_cutoffs: Option<Vec<f64>>,
    per_bucket_cascade: Option<PerBucketCascadeConfigFile>,
    frozen_covariance_validation: Option<FrozenCovarianceValidationConfigFile>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenCovarianceValidationConfigFile {
    rho_fraction: f64,
    ridge_primary_fraction: f64,
    ridge_secondary_fraction: f64,
    control_count_per_bucket: usize,
    control_seed: u64,
}

#[derive(Clone, Debug)]
struct FrozenCovarianceValidation {
    rho_fraction: f64,
    ridge_primary_fraction: f64,
    ridge_secondary_fraction: f64,
    control_count_per_bucket: usize,
    control_seed: u64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct FrozenCovarianceValidationReport {
    scope: String,
    rounding: String,
    rho_feature: String,
    rho_direction: String,
    rho_fraction: f64,
    ridge_primary_feature: String,
    ridge_primary_direction: String,
    ridge_primary_fraction: f64,
    ridge_secondary_feature: String,
    ridge_secondary_direction: String,
    ridge_secondary_fraction: f64,
    control_count_per_bucket: usize,
    control_seed: u64,
    control_exclusion: String,
}

impl FrozenCovarianceValidation {
    fn report(&self) -> FrozenCovarianceValidationReport {
        FrozenCovarianceValidationReport {
            scope: "actual_bucket_id_within_one_producer_seed".to_string(),
            rounding: "ceil_min_one".to_string(),
            rho_feature: "vertex_covariance_rho".to_string(),
            rho_direction: "low".to_string(),
            rho_fraction: self.rho_fraction,
            ridge_primary_feature: "ridge_symp_area_sum_over_volume_sqrt".to_string(),
            ridge_primary_direction: "low".to_string(),
            ridge_primary_fraction: self.ridge_primary_fraction,
            ridge_secondary_feature: "ridge_symp_area_max_share".to_string(),
            ridge_secondary_direction: "low".to_string(),
            ridge_secondary_fraction: self.ridge_secondary_fraction,
            control_count_per_bucket: self.control_count_per_bucket,
            control_seed: self.control_seed,
            control_exclusion: "rho_ridge_union".to_string(),
        }
    }

    fn rules(&self) -> Vec<SelectionRule> {
        vec![
            SelectionRule::single(
                ScalarFeature::VertexCovarianceRho,
                SelectionDirection::Low,
                &[],
                &[],
                &[],
            ),
            SelectionRule::single(
                ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
                SelectionDirection::Low,
                &[],
                &[],
                &[],
            ),
            SelectionRule::single(
                ScalarFeature::RidgeSympAreaMaxShare,
                SelectionDirection::Low,
                &[],
                &[],
                &[],
            ),
        ]
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct PerBucketCascadeConfigFile {
    primary_feature: String,
    primary_direction: String,
    primary_fraction: f64,
    secondary_feature: String,
    secondary_direction: String,
    secondary_fraction: f64,
    emit_stage_1_comparator: bool,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionRuleConfigFile {
    feature: String,
    direction: String,
}

#[derive(Default)]
struct CliOverrides {
    config_path: Option<PathBuf>,
    stage: Option<Stage>,
    out_dir: Option<PathBuf>,
    seed: Option<u64>,
    candidates_per_bucket: Option<usize>,
    limit_total: Option<Option<usize>>,
    baseline_replicates: Option<usize>,
    global_top: Option<Vec<usize>>,
    per_bucket_top: Option<Vec<usize>>,
    percentile_cutoffs: Option<Vec<f64>>,
    selection_feature: Option<ScalarFeature>,
    selection_direction: Option<SelectionDirection>,
    rule_set: Option<String>,
    explicit_selection_rules: Option<Vec<(ScalarFeature, SelectionDirection)>>,
    jobs: Option<usize>,
    chunk_rows: Option<usize>,
}

#[derive(Clone, Serialize)]
struct ResolvedRunConfig {
    schema: String,
    config_path: Option<String>,
    stage: String,
    out_dir: String,
    seed: u64,
    h_min: f64,
    h_max: f64,
    product_pairs: Vec<[usize; 2]>,
    candidates_per_bucket: usize,
    limit_total: Option<usize>,
    baseline_policy: String,
    baseline_replicates: usize,
    selection: ResolvedSelectionConfig,
    jobs: usize,
    chunk_rows: usize,
    metadata: RunMetadata,
}

#[derive(Clone, Serialize)]
struct ResolvedSelectionConfig {
    rule_set: String,
    selection_feature: String,
    selection_direction: String,
    global_top: Vec<usize>,
    per_bucket_top: Vec<usize>,
    percentile_cutoffs: Vec<f64>,
    rules: Vec<SelectionRuleReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_bucket_cascade: Option<PerBucketCascadeReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frozen_covariance_validation: Option<FrozenCovarianceValidationReport>,
}

#[derive(Clone, Serialize, Deserialize)]
struct GeometryGeneratorMetadata {
    schema: String,
    generator: String,
    h_min: f64,
    h_max: f64,
    product_pairs: Vec<[usize; 2]>,
    candidate_id_key: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct FeatureSchemaMetadata {
    schema: String,
    available_scalar_features: Vec<String>,
    source_cache_fields: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SysCacheMetadata {
    schema: String,
    reuse_key: String,
    reuse_contract: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct RunMetadata {
    geometry_generator: GeometryGeneratorMetadata,
    feature_schema: FeatureSchemaMetadata,
    selection_semantics: String,
    baseline_policy: String,
    sys_cache: SysCacheMetadata,
    cache_lifecycle: String,
    report_semantics: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct ProductSource {
    producer: String,
    k: usize,
    m: usize,
    h_min: f64,
    h_max: f64,
    seed: u64,
    sample_index: usize,
    attempt: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct CandidateGeometryRow {
    schema: String,
    candidate_id: String,
    name: String,
    poly_id: String,
    producer: String,
    source: ProductSource,
    bucket_id: String,
    seed: u64,
    sample_index: usize,
    attempt: u64,
    h_min: f64,
    h_max: f64,
    product_k: usize,
    product_m: usize,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    vertices: Vec<[f64; 4]>,
    vertex_facet_incidence: Vec<Vec<bool>>,
    vertex_count: usize,
    edge_count: usize,
    ridge_count: usize,
    is_simple: bool,
    simple_vertex_fraction: f64,
    validation_status: String,
    validation_marker: String,
    time_geometry_ms: f64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct CandidateFeatureRow {
    schema: String,
    candidate_id: String,
    name: String,
    poly_id: String,
    producer: String,
    source: ProductSource,
    bucket_id: String,
    seed: u64,
    sample_index: usize,
    attempt: u64,
    h_min: f64,
    h_max: f64,
    product_k: usize,
    product_m: usize,
    facet_count: usize,
    volume: f64,
    vertex_count: usize,
    edge_count: usize,
    ridge_count: usize,
    is_simple: bool,
    simple_vertex_fraction: f64,
    #[serde(default)]
    vertex_covariance_rho: Option<f64>,
    #[serde(default)]
    vertex_covariance_nu1: Option<f64>,
    #[serde(default)]
    vertex_covariance_nu2: Option<f64>,
    #[serde(default)]
    vertex_covariance_condition: Option<f64>,
    #[serde(default)]
    vertex_covariance_ordinary_eigenvalue_min: Option<f64>,
    #[serde(default)]
    vertex_covariance_ordinary_eigenvalue_max: Option<f64>,
    #[serde(default)]
    vertex_covariance_distinct_vertex_count: usize,
    #[serde(default)]
    vertex_covariance_expected_vertex_count: usize,
    #[serde(default)]
    vertex_covariance_status: String,
    edge_density: f64,
    vertex_incident_facets_mean: f64,
    vertex_incident_facets_std: f64,
    vertex_incident_facets_min: f64,
    vertex_incident_facets_max: f64,
    vertex_degree_mean: f64,
    vertex_degree_std: f64,
    vertex_degree_min: f64,
    vertex_degree_max: f64,
    ridge_size_mean: f64,
    ridge_size_std: f64,
    ridge_size_min: f64,
    ridge_size_max: f64,
    facet_vertex_count_mean: f64,
    facet_vertex_count_std: f64,
    facet_vertex_count_min: f64,
    facet_vertex_count_max: f64,
    facet_neighbor_count_mean: f64,
    facet_neighbor_count_std: f64,
    facet_neighbor_count_min: f64,
    facet_neighbor_count_max: f64,
    ridge_symp_area_ordered_face_count: usize,
    ridge_symp_area_ordering_failure_count: usize,
    ridge_symp_area_ordered_fraction: f64,
    ridge_symp_area_mean_over_volume_sqrt: f64,
    ridge_symp_area_std_over_volume_sqrt: f64,
    ridge_symp_area_min_over_volume_sqrt: f64,
    ridge_symp_area_max_over_volume_sqrt: f64,
    ridge_symp_area_q25_over_volume_sqrt: f64,
    ridge_symp_area_median_over_volume_sqrt: f64,
    ridge_symp_area_q75_over_volume_sqrt: f64,
    ridge_symp_area_q90_over_volume_sqrt: f64,
    ridge_symp_area_q95_over_volume_sqrt: f64,
    ridge_symp_area_sum_over_volume_sqrt: f64,
    ridge_symp_area_max_share: f64,
    ridge_symp_area_top3_share: f64,
    ridge_symp_area_le_1em3_over_volume_sqrt_fraction: f64,
    ridge_symp_area_le_1em2_over_volume_sqrt_fraction: f64,
    ridge_symp_area_le_1em1_over_volume_sqrt_fraction: f64,
    ridge_symp_area_entropy: f64,
    ridge_symp_area_effective_face_count: f64,
    ridge_symp_area_normalized_entropy: f64,
    time_feature_ms: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct PreTargetSelectionRow {
    schema: String,
    candidate_id: String,
    name: String,
    poly_id: String,
    producer: String,
    source: ProductSource,
    bucket_id: String,
    #[serde(default = "default_selection_feature_string")]
    selection_feature: String,
    #[serde(default = "default_selection_direction_string")]
    selection_direction: String,
    selection_feature_value: f64,
    #[serde(default)]
    selection_rule_values: Vec<SelectionRuleValue>,
    selection_ids: Vec<String>,
    baseline_ids: Vec<String>,
    evaluation_roles: Vec<String>,
    stage_order: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct EvaluatedRow {
    schema: String,
    candidate_id: String,
    name: String,
    poly_id: String,
    bucket_id: String,
    product_k: usize,
    product_m: usize,
    sample_index: usize,
    attempt: u64,
    evaluation_roles: Vec<String>,
    selection_ids: Vec<String>,
    baseline_ids: Vec<String>,
    #[serde(default = "default_selection_feature_string")]
    selection_feature: String,
    #[serde(default = "default_selection_direction_string")]
    selection_direction: String,
    selection_feature_value: f64,
    #[serde(default)]
    selection_rule_values: Vec<SelectionRuleValue>,
    capacity: f64,
    sys: f64,
    bounces: usize,
    time_capacity_ms: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct SelectionPlanEntry {
    selection_id: String,
    selection_kind: String,
    selection_feature: String,
    selection_direction: String,
    requested_budget: String,
    selected_rows: usize,
}

#[derive(Serialize, Deserialize)]
struct SelectionPlanReport {
    schema: String,
    #[serde(default)]
    metadata: Option<RunMetadata>,
    seed: u64,
    candidates_per_bucket: usize,
    limit_total: Option<usize>,
    feature_rows: usize,
    #[serde(default = "default_baseline_policy_string")]
    baseline_policy: String,
    baseline_replicates: usize,
    rule_set: String,
    rules: Vec<SelectionRuleReport>,
    #[serde(default)]
    per_bucket_cascade: Option<PerBucketCascadeReport>,
    #[serde(default)]
    frozen_covariance_validation: Option<FrozenCovarianceValidationReport>,
    selection_feature: String,
    selection_direction: String,
    global_top: Vec<usize>,
    per_bucket_top: Vec<usize>,
    percentile_cutoffs: Vec<f64>,
    selection_sets: usize,
    selected_rows_sum_over_sets: usize,
    unique_selected_rows: usize,
    unique_baseline_rows: usize,
    unique_selected_or_baseline_rows: usize,
    selections: Vec<SelectionPlanEntry>,
}

#[derive(Clone, Serialize)]
struct SelectionSummaryRow {
    selection_id: String,
    selection_kind: String,
    requested_budget: String,
    selected_rows: usize,
    selected_max_sys: f64,
    selected_mean_sys: f64,
    baseline_replicates: usize,
    baseline_rows_total: usize,
    baseline_p90_sys: f64,
    baseline_p95_sys: f64,
    selected_rows_above_baseline_p95: usize,
    baseline_mean_sys: f64,
    improvement_vs_baseline_mean_sys: f64,
    selected_bucket_counts: String,
}

#[derive(Serialize)]
struct EvaluationReport {
    schema: String,
    question: String,
    metadata: RunMetadata,
    architecture_status: String,
    stage_interface: String,
    deterministic_output_status: String,
    feature_cache_usage_status: String,
    sys_cache_status: String,
    seed: u64,
    h_min: f64,
    h_max: f64,
    candidates_per_bucket: usize,
    limit_total: Option<usize>,
    product_buckets: usize,
    jobs: usize,
    chunk_rows: usize,
    candidate_geometry_rows: usize,
    feature_rows: usize,
    pre_target_selection_rows: usize,
    evaluated_rows_for_current_selection: usize,
    sys_evaluation_cache_rows_total: usize,
    baseline_policy: String,
    baseline_replicates: usize,
    rule_set: String,
    selection_rules: Vec<SelectionRuleReport>,
    selection_sets: usize,
    selected_rows_sum_over_sets: usize,
    unique_selected_rows: usize,
    unique_baseline_rows: usize,
    unique_selected_or_baseline_rows: usize,
    selection_feature: String,
    selection_direction: String,
    max_sys_evaluated_current_selection: f64,
    max_sys_selected: f64,
    missing_sys_rows_for_current_selection: usize,
    candidate_geometry_cache_blake3: String,
    candidate_feature_table_blake3: String,
    selected_candidates_before_sys_blake3: String,
    sys_evaluation_cache_blake3: String,
    selection_summary_blake3: String,
    selection_plan_blake3: String,
    resolved_run_config_blake3: String,
    pipeline_summary_status: String,
}

#[derive(Clone)]
struct SelectionSet {
    id: String,
    kind: String,
    feature: ScalarFeature,
    direction: SelectionDirection,
    requested_budget: String,
    indices: Vec<usize>,
    require_disjoint_baseline: bool,
}

struct ProductGeometry {
    dual_vertices: Vec<Vector4<f64>>,
    vertices: Vec<Vector4<f64>>,
    incidence: DMatrix<bool>,
    volume: f64,
    attempt: u64,
}

fn parse_cli_overrides() -> CliOverrides {
    let argv = std::env::args().collect::<Vec<_>>();
    let mut overrides = CliOverrides::default();
    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = || {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match flag {
            "--config" => {
                overrides.config_path = Some(PathBuf::from(value()));
                i += 2;
            }
            "--stage" => {
                overrides.stage = Some(parse_stage(value()));
                i += 2;
            }
            "--out-dir" => {
                overrides.out_dir = Some(PathBuf::from(value()));
                i += 2;
            }
            "--seed" => {
                overrides.seed = Some(value().parse().expect("--seed must be a u64"));
                i += 2;
            }
            "--candidates-per-bucket" => {
                let parsed = value()
                    .parse()
                    .expect("--candidates-per-bucket must be positive");
                assert!(parsed > 0);
                overrides.candidates_per_bucket = Some(parsed);
                i += 2;
            }
            "--limit-total" => {
                let parsed = value().parse().expect("--limit-total must be positive");
                assert!(parsed > 0);
                overrides.limit_total = Some(Some(parsed));
                i += 2;
            }
            "--baseline-replicates" => {
                let parsed = value()
                    .parse()
                    .expect("--baseline-replicates must be positive");
                assert!(parsed > 0);
                overrides.baseline_replicates = Some(parsed);
                i += 2;
            }
            "--global-top" => {
                overrides.global_top = Some(parse_usize_csv(value()));
                i += 2;
            }
            "--per-bucket-top" => {
                overrides.per_bucket_top = Some(parse_usize_csv(value()));
                i += 2;
            }
            "--percentile-cutoffs" => {
                overrides.percentile_cutoffs = Some(parse_f64_csv(value()));
                i += 2;
            }
            "--selection-feature" => {
                overrides.selection_feature = Some(ScalarFeature::parse(value()));
                i += 2;
            }
            "--selection-direction" => {
                overrides.selection_direction = Some(SelectionDirection::parse(value()));
                i += 2;
            }
            "--selection-rules" => {
                overrides.explicit_selection_rules = Some(parse_selection_rules(value()));
                overrides.rule_set = Some("custom".to_string());
                i += 2;
            }
            "--rule-set" => {
                overrides.rule_set = Some(value().to_string());
                i += 2;
            }
            "--jobs" => {
                let parsed = value().parse().expect("--jobs must be positive");
                assert!(parsed > 0);
                overrides.jobs = Some(parsed);
                i += 2;
            }
            "--chunk-rows" => {
                let parsed = value().parse().expect("--chunk-rows must be positive");
                assert!(parsed > 0);
                overrides.chunk_rows = Some(parsed);
                i += 2;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: extreme-scalar-rejection-proposer [--config path/to/run.json] [--out-dir <dir>] [--stage all|geometry|features|selection|sys|reports] [--seed 271828] [--candidates-per-bucket 3] [--limit-total N] [--baseline-replicates 1] [--global-top 1,5] [--per-bucket-top 1] [--percentile-cutoffs 0.10] [--selection-feature ridge_symp_area_sum_over_volume_sqrt] [--selection-direction low|high] [--selection-rules feature:low,feature:high] [--rule-set single|promising-scalars] [--jobs N] [--chunk-rows 1000]\nAllowed selection features: {}",
                    allowed_scalar_feature_names()
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    overrides
}

fn read_config_file(path: &Path) -> RunConfigFile {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("open config {}: {err}", path.display()));
    serde_json::from_reader(BufReader::new(file))
        .unwrap_or_else(|err| panic!("parse config {}: {err}", path.display()))
}

fn validate_positive_usize(value: usize, field: &str) -> usize {
    assert!(value > 0, "{field} must be positive");
    value
}

fn validate_usize_vec(values: Vec<usize>, field: &str) -> Vec<usize> {
    assert!(
        values.iter().all(|value| *value > 0),
        "{field} entries must be positive"
    );
    values
}

fn validate_percentile_vec(values: Vec<f64>) -> Vec<f64> {
    assert!(
        values.iter().all(|value| *value > 0.0 && *value <= 1.0),
        "percentile_cutoffs entries must be in (0, 1]"
    );
    values
}

fn validate_fraction(value: f64, field: &str) -> f64 {
    assert!(
        value.is_finite() && value > 0.0 && value <= 1.0,
        "{field} must be finite and in (0, 1]"
    );
    value
}

fn validate_cascade_conflicts(selection: &SelectionConfigFile) {
    if selection.per_bucket_cascade.is_some() || selection.frozen_covariance_validation.is_some() {
        assert!(
            !(selection.per_bucket_cascade.is_some()
                && selection.frozen_covariance_validation.is_some())
                &&
            selection.rule_set.is_none()
                && selection.selection_feature.is_none()
                && selection.selection_direction.is_none()
                && selection.selection_rules.is_none()
                && selection.global_top.is_none()
                && selection.per_bucket_top.is_none()
                && selection.percentile_cutoffs.is_none(),
            "special multi-stage selection cannot be combined with another special mode or ordinary scalar-rule selection fields"
        );
    }
}

fn parse_frozen_covariance_validation(
    config: FrozenCovarianceValidationConfigFile,
) -> FrozenCovarianceValidation {
    FrozenCovarianceValidation {
        rho_fraction: validate_fraction(
            config.rho_fraction,
            "selection.frozen_covariance_validation.rho_fraction",
        ),
        ridge_primary_fraction: validate_fraction(
            config.ridge_primary_fraction,
            "selection.frozen_covariance_validation.ridge_primary_fraction",
        ),
        ridge_secondary_fraction: validate_fraction(
            config.ridge_secondary_fraction,
            "selection.frozen_covariance_validation.ridge_secondary_fraction",
        ),
        control_count_per_bucket: validate_positive_usize(
            config.control_count_per_bucket,
            "selection.frozen_covariance_validation.control_count_per_bucket",
        ),
        control_seed: config.control_seed,
    }
}

fn parse_per_bucket_cascade(config: PerBucketCascadeConfigFile) -> PerBucketCascade {
    PerBucketCascade {
        primary_feature: ScalarFeature::parse(&config.primary_feature),
        primary_direction: SelectionDirection::parse(&config.primary_direction),
        primary_fraction: validate_fraction(
            config.primary_fraction,
            "selection.per_bucket_cascade.primary_fraction",
        ),
        secondary_feature: ScalarFeature::parse(&config.secondary_feature),
        secondary_direction: SelectionDirection::parse(&config.secondary_direction),
        secondary_fraction: validate_fraction(
            config.secondary_fraction,
            "selection.per_bucket_cascade.secondary_fraction",
        ),
        emit_stage_1_comparator: config.emit_stage_1_comparator,
    }
}

fn parse_config_selection_rules(
    rules: Vec<SelectionRuleConfigFile>,
) -> Vec<(ScalarFeature, SelectionDirection)> {
    assert!(
        !rules.is_empty(),
        "selection.selection_rules must name at least one rule"
    );
    rules
        .into_iter()
        .map(|rule| {
            (
                ScalarFeature::parse(&rule.feature),
                SelectionDirection::parse(&rule.direction),
            )
        })
        .collect()
}

fn parse_args() -> Args {
    let overrides = parse_cli_overrides();
    let cli_has_scalar_selection_override = overrides.global_top.is_some()
        || overrides.per_bucket_top.is_some()
        || overrides.percentile_cutoffs.is_some()
        || overrides.selection_feature.is_some()
        || overrides.selection_direction.is_some()
        || overrides.rule_set.is_some()
        || overrides.explicit_selection_rules.is_some();
    let config = overrides.config_path.as_deref().map(read_config_file);

    let mut stage = Stage::All;
    let mut out_dir = None;
    let mut seed = 271828_u64;
    let mut candidates_per_bucket = 3_usize;
    let mut limit_total = None;
    let mut baseline_policy = default_baseline_policy_string();
    let mut baseline_replicates = 1_usize;
    let mut global_top = vec![1usize, 5];
    let mut per_bucket_top = vec![1usize];
    let mut percentile_cutoffs = vec![0.10_f64];
    let mut selection_feature = DEFAULT_SELECTION_FEATURE;
    let mut selection_direction = DEFAULT_SELECTION_DIRECTION;
    let mut rule_set = "single".to_string();
    let mut explicit_selection_rules = None::<Vec<(ScalarFeature, SelectionDirection)>>;
    let mut per_bucket_cascade = None::<PerBucketCascade>;
    let mut frozen_covariance_validation = None::<FrozenCovarianceValidation>;
    let mut jobs = std::thread::available_parallelism().map_or(1, usize::from);
    let mut chunk_rows = DEFAULT_CHUNK_ROWS;

    if let Some(config) = config {
        if let Some(value) = config.stage {
            stage = parse_stage(&value);
        }
        if let Some(value) = config.out_dir {
            out_dir = Some(value);
        }
        if let Some(value) = config.seed {
            seed = value;
        }
        if let Some(value) = config.candidates_per_bucket {
            candidates_per_bucket = validate_positive_usize(value, "candidates_per_bucket");
        }
        if let Some(value) = config.limit_total {
            limit_total = Some(validate_positive_usize(value, "limit_total"));
        }
        if let Some(value) = config.baseline_policy {
            baseline_policy = value;
        }
        if let Some(value) = config.baseline_replicates {
            baseline_replicates = validate_positive_usize(value, "baseline_replicates");
        }
        if let Some(selection) = config.selection {
            validate_cascade_conflicts(&selection);
            if let Some(value) = selection.per_bucket_cascade {
                per_bucket_cascade = Some(parse_per_bucket_cascade(value));
                rule_set = "per-bucket-cascade".to_string();
                let cascade = per_bucket_cascade.as_ref().expect("cascade was set");
                selection_feature = cascade.primary_feature;
                selection_direction = cascade.primary_direction;
            }
            if let Some(value) = selection.frozen_covariance_validation {
                frozen_covariance_validation = Some(parse_frozen_covariance_validation(value));
                rule_set = "frozen-covariance-validation".to_string();
                selection_feature = ScalarFeature::VertexCovarianceRho;
                selection_direction = SelectionDirection::Low;
            }
            if let Some(value) = selection.rule_set {
                rule_set = value;
            }
            if let Some(value) = selection.selection_feature {
                selection_feature = ScalarFeature::parse(&value);
            }
            if let Some(value) = selection.selection_direction {
                selection_direction = SelectionDirection::parse(&value);
            }
            if let Some(value) = selection.selection_rules {
                explicit_selection_rules = Some(parse_config_selection_rules(value));
                rule_set = "custom".to_string();
            }
            if let Some(value) = selection.global_top {
                global_top = validate_usize_vec(value, "selection.global_top");
            }
            if let Some(value) = selection.per_bucket_top {
                per_bucket_top = validate_usize_vec(value, "selection.per_bucket_top");
            }
            if let Some(value) = selection.percentile_cutoffs {
                percentile_cutoffs = validate_percentile_vec(value);
            }
        }
        if let Some(value) = config.jobs {
            jobs = validate_positive_usize(value, "jobs");
        }
        if let Some(value) = config.chunk_rows {
            chunk_rows = validate_positive_usize(value, "chunk_rows");
        }
    }

    if let Some(value) = overrides.stage {
        stage = value;
    }
    if let Some(value) = overrides.out_dir {
        out_dir = Some(value);
    }
    if let Some(value) = overrides.seed {
        seed = value;
    }
    if let Some(value) = overrides.candidates_per_bucket {
        candidates_per_bucket = value;
    }
    if let Some(value) = overrides.limit_total {
        limit_total = value;
    }
    if let Some(value) = overrides.baseline_replicates {
        baseline_replicates = value;
    }
    if let Some(value) = overrides.global_top {
        global_top = value;
    }
    if let Some(value) = overrides.per_bucket_top {
        per_bucket_top = value;
    }
    if let Some(value) = overrides.percentile_cutoffs {
        percentile_cutoffs = value;
    }
    if let Some(value) = overrides.selection_feature {
        selection_feature = value;
    }
    if let Some(value) = overrides.selection_direction {
        selection_direction = value;
    }
    if let Some(value) = overrides.rule_set {
        rule_set = value;
    }
    if let Some(value) = overrides.explicit_selection_rules {
        explicit_selection_rules = Some(value);
        rule_set = "custom".to_string();
    }
    if let Some(value) = overrides.jobs {
        jobs = value;
    }
    if let Some(value) = overrides.chunk_rows {
        chunk_rows = value;
    }

    assert_eq!(
        baseline_policy, "per_selection_matched",
        "only baseline_policy=per_selection_matched is implemented"
    );
    assert!(
        (per_bucket_cascade.is_none() && frozen_covariance_validation.is_none())
            || !cli_has_scalar_selection_override,
        "special multi-stage selection cannot be combined with ordinary scalar-rule CLI overrides"
    );
    let selection_rules = if let Some(frozen) = &frozen_covariance_validation {
        frozen.rules()
    } else if let Some(cascade) = &per_bucket_cascade {
        cascade.rules()
    } else {
        build_selection_rules(
            &rule_set,
            explicit_selection_rules,
            selection_feature,
            selection_direction,
            &global_top,
            &per_bucket_top,
            &percentile_cutoffs,
        )
    };
    let use_legacy_default_rule_alias = rule_set == "single"
        && selection_rules.len() == 1
        && selection_rules[0].feature == DEFAULT_SELECTION_FEATURE
        && selection_rules[0].direction == DEFAULT_SELECTION_DIRECTION;
    Args {
        config_path: overrides.config_path,
        stage,
        out_dir: out_dir.expect("--out-dir is required"),
        seed,
        candidates_per_bucket,
        limit_total,
        baseline_replicates,
        global_top,
        per_bucket_top,
        percentile_cutoffs,
        selection_feature,
        selection_direction,
        rule_set,
        baseline_policy,
        selection_rules,
        per_bucket_cascade,
        frozen_covariance_validation,
        use_legacy_default_rule_alias,
        jobs,
        chunk_rows,
    }
}

fn parse_selection_rules(value: &str) -> Vec<(ScalarFeature, SelectionDirection)> {
    let rules = value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            let pieces = part.trim().split(':').collect::<Vec<_>>();
            assert_eq!(
                pieces.len(),
                2,
                "--selection-rules entries must have form feature:low or feature:high"
            );
            (
                ScalarFeature::parse(pieces[0].trim()),
                SelectionDirection::parse(pieces[1].trim()),
            )
        })
        .collect::<Vec<_>>();
    assert!(
        !rules.is_empty(),
        "--selection-rules must name at least one rule"
    );
    rules
}

fn promising_scalar_rule_pairs() -> Vec<(ScalarFeature, SelectionDirection)> {
    vec![
        (
            ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (
            ScalarFeature::RidgeSympAreaMaxOverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (
            ScalarFeature::RidgeSympAreaStdOverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (
            ScalarFeature::RidgeSympAreaQ95OverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (
            ScalarFeature::RidgeSympAreaQ90OverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (
            ScalarFeature::RidgeSympAreaMeanOverVolumeSqrt,
            SelectionDirection::Low,
        ),
        (ScalarFeature::RidgeCount, SelectionDirection::High),
        (ScalarFeature::EdgeCount, SelectionDirection::High),
        (ScalarFeature::VertexCount, SelectionDirection::High),
        (ScalarFeature::FacetCount, SelectionDirection::High),
    ]
}

fn build_selection_rules(
    rule_set: &str,
    explicit_selection_rules: Option<Vec<(ScalarFeature, SelectionDirection)>>,
    selection_feature: ScalarFeature,
    selection_direction: SelectionDirection,
    global_top: &[usize],
    per_bucket_top: &[usize],
    percentile_cutoffs: &[f64],
) -> Vec<SelectionRule> {
    let pairs = if let Some(rules) = explicit_selection_rules {
        rules
    } else {
        match rule_set {
            "single" => vec![(selection_feature, selection_direction)],
            "promising-scalars" => promising_scalar_rule_pairs(),
            other => panic!("unknown rule set {other}; expected single|promising-scalars"),
        }
    };
    pairs
        .into_iter()
        .map(|(feature, direction)| {
            SelectionRule::single(
                feature,
                direction,
                global_top,
                per_bucket_top,
                percentile_cutoffs,
            )
        })
        .collect()
}

fn parse_stage(value: &str) -> Stage {
    match value {
        "all" => Stage::All,
        "geometry" | "geometry_cache" => Stage::Geometry,
        "features" | "feature_table" => Stage::Features,
        "selection" | "pre_sys_selection" | "pre_target_selection" => Stage::Selection,
        "sys" | "sys_evaluation" => Stage::Sys,
        "reports" | "report" => Stage::Reports,
        other => panic!("unknown stage {other}"),
    }
}

fn parse_usize_csv(value: &str) -> Vec<usize> {
    if value.trim().eq_ignore_ascii_case("none") {
        return Vec::new();
    }
    value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.trim().parse().expect("usize csv entry"))
        .collect()
}

fn parse_f64_csv(value: &str) -> Vec<f64> {
    if value.trim().eq_ignore_ascii_case("none") {
        return Vec::new();
    }
    value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            let parsed = part.trim().parse::<f64>().expect("f64 csv entry");
            assert!(
                parsed > 0.0 && parsed <= 1.0,
                "percentile cutoffs must be in (0, 1]"
            );
            parsed
        })
        .collect()
}

fn product_seed(
    seed: u64,
    k: usize,
    m: usize,
    h_min: f64,
    h_max: f64,
    sample_index: usize,
    attempt: u64,
) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(&(k as u64).to_le_bytes());
    material.extend_from_slice(&(m as u64).to_le_bytes());
    material.extend_from_slice(&h_min.to_le_bytes());
    material.extend_from_slice(&h_max.to_le_bytes());
    material.extend_from_slice(&(sample_index as u64).to_le_bytes());
    material.extend_from_slice(&attempt.to_le_bytes());
    blake3::derive_key("datascience-feature-first-random-product", &material)
}

fn polygon_vertices_from_h_rep(
    normals: &[Vector2<f64>],
    heights: &[f64],
) -> Option<Vec<Vector2<f64>>> {
    if normals.len() < 3 || normals.len() != heights.len() {
        return None;
    }
    let n = normals.len();
    let mut vertices = Vec::with_capacity(n);
    for i in 0..n {
        let j = (i + 1) % n;
        let ni = &normals[i];
        let nj = &normals[j];
        let det = ni[0] * nj[1] - ni[1] * nj[0];
        if det.abs() < 1e-12 {
            return None;
        }
        let x = (heights[i] * nj[1] - heights[j] * ni[1]) / det;
        let y = (ni[0] * heights[j] - nj[0] * heights[i]) / det;
        vertices.push(Vector2::new(x, y));
    }
    for vertex in &vertices {
        for (normal, height) in normals.iter().zip(heights) {
            let slack = normal.dot(vertex) - height;
            if slack > 1e-9 * height.abs().max(1.0) {
                return None;
            }
        }
    }
    Some(vertices)
}

fn lagrangian_product_dual_vertices(
    q_normals: &[Vector2<f64>],
    q_heights: &[f64],
    p_normals: &[Vector2<f64>],
    p_heights: &[f64],
) -> Vec<Vector4<f64>> {
    let mut dual_vertices = Vec::with_capacity(q_normals.len() + p_normals.len());
    for (normal, height) in q_normals.iter().zip(q_heights) {
        dual_vertices.push(Vector4::new(
            normal[0] / height,
            normal[1] / height,
            0.0,
            0.0,
        ));
    }
    for (normal, height) in p_normals.iter().zip(p_heights) {
        dual_vertices.push(Vector4::new(
            0.0,
            0.0,
            normal[0] / height,
            normal[1] / height,
        ));
    }
    dual_vertices
}

fn lagrangian_product_vertices_and_incidence(
    q_vertices: &[Vector2<f64>],
    p_vertices: &[Vector2<f64>],
) -> (Vec<Vector4<f64>>, DMatrix<bool>) {
    let k = q_vertices.len();
    let m = p_vertices.len();
    let mut vertices = Vec::with_capacity(k * m);
    let mut incidence = DMatrix::from_element(k * m, k + m, false);
    for (q_index, q_vertex) in q_vertices.iter().enumerate() {
        for (p_index, p_vertex) in p_vertices.iter().enumerate() {
            let row = q_index * m + p_index;
            vertices.push(Vector4::new(
                q_vertex[0],
                q_vertex[1],
                p_vertex[0],
                p_vertex[1],
            ));
            incidence[(row, q_index)] = true;
            incidence[(row, (q_index + 1) % k)] = true;
            incidence[(row, k + p_index)] = true;
            incidence[(row, k + ((p_index + 1) % m))] = true;
        }
    }
    (vertices, incidence)
}

fn generate_product_geometry(
    seed: u64,
    k: usize,
    m: usize,
    sample_index: usize,
) -> Option<ProductGeometry> {
    for attempt in 0.. {
        let mut rng = ChaCha8Rng::from_seed(product_seed(
            seed,
            k,
            m,
            H_MIN,
            H_MAX,
            sample_index,
            attempt,
        ));
        let (qn, qh) = random_polygon_2d(k, H_MIN, H_MAX, &mut rng);
        let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
        let Some(q_area) = polygon_area(&qn, &qh) else {
            continue;
        };
        let Some(p_area) = polygon_area(&pn, &ph) else {
            continue;
        };
        let Some(q_vertices) = polygon_vertices_from_h_rep(&qn, &qh) else {
            continue;
        };
        let Some(p_vertices) = polygon_vertices_from_h_rep(&pn, &ph) else {
            continue;
        };
        let dual_vertices = lagrangian_product_dual_vertices(&qn, &qh, &pn, &ph);
        let (vertices, incidence) =
            lagrangian_product_vertices_and_incidence(&q_vertices, &p_vertices);
        return Some(ProductGeometry {
            dual_vertices,
            vertices,
            incidence,
            volume: q_area * p_area,
            attempt,
        });
    }
    None
}

fn h_range_label(h_min: f64, h_max: f64) -> String {
    format!("{:.1}_{:.1}", h_min, h_max).replace('.', "p")
}

fn candidate_id(seed: u64, k: usize, m: usize, sample_index: usize) -> String {
    format!(
        "random-product:seed{seed}:{k}x{m}:h{}:sample{sample_index}",
        h_range_label(H_MIN, H_MAX)
    )
}

fn bucket_id(k: usize, m: usize) -> String {
    format!("random-product:{k}x{m}:h{}", h_range_label(H_MIN, H_MAX))
}

fn vectors_to_arrays(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn arrays_to_vectors(arrays: &[[f64; 4]]) -> Vec<Vector4<f64>> {
    arrays
        .iter()
        .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
        .collect()
}

fn incidence_to_rows(incidence: &DMatrix<bool>) -> Vec<Vec<bool>> {
    (0..incidence.nrows())
        .map(|row| {
            (0..incidence.ncols())
                .map(|col| incidence[(row, col)])
                .collect()
        })
        .collect()
}

fn rows_to_incidence(rows: &[Vec<bool>]) -> DMatrix<bool> {
    let row_count = rows.len();
    let col_count = rows.first().map_or(0, Vec::len);
    assert!(
        rows.iter().all(|row| row.len() == col_count),
        "all incidence rows must have the same width"
    );
    DMatrix::from_fn(row_count, col_count, |row, col| rows[row][col])
}

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| {
            let centered = value - mean;
            centered * centered
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().reduce(f64::min).unwrap_or(0.0);
    let max = values.iter().copied().reduce(f64::max).unwrap_or(0.0);
    (mean, variance.sqrt(), min, max)
}

fn compute_skeleton_fields_from_cached_parts(
    facet_count: usize,
    vertex_count: usize,
    incidence: &DMatrix<bool>,
    vertex_facets: &[Vec<usize>],
    edges: &[[usize; 2]],
    two_faces: &[euclidean_polytopes::TwoFace],
) -> features_skeleton::SkeletonFields {
    let edge_count = edges.len();
    let ridge_count = two_faces.len();
    let vertex_incident_facets = vertex_facets
        .iter()
        .map(|facets| facets.len() as f64)
        .collect::<Vec<_>>();
    let simple_vertices = vertex_incident_facets
        .iter()
        .filter(|&&count| (count - 4.0).abs() < f64::EPSILON)
        .count();
    let simple_vertex_fraction = if vertex_count == 0 {
        0.0
    } else {
        simple_vertices as f64 / vertex_count as f64
    };
    let mut vertex_degrees = vec![0usize; vertex_count];
    for edge in edges {
        vertex_degrees[edge[0]] += 1;
        vertex_degrees[edge[1]] += 1;
    }
    let vertex_degrees = vertex_degrees
        .into_iter()
        .map(|degree| degree as f64)
        .collect::<Vec<_>>();
    let ridge_sizes = two_faces
        .iter()
        .map(|two_face| two_face.vertices.len() as f64)
        .collect::<Vec<_>>();
    let mut facet_vertex_counts = vec![0usize; facet_count];
    for facets in vertex_facets {
        for &facet in facets {
            facet_vertex_counts[facet] += 1;
        }
    }
    let facet_vertex_counts = facet_vertex_counts
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let facet_intersection_is_nonempty =
        facet_intersection_is_nonempty_from_vertex_facet_incidence(incidence);
    let facet_neighbor_counts = (0..facet_count)
        .map(|facet| {
            (0..facet_count)
                .filter(|&other| facet_intersection_is_nonempty[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();
    let edge_density = if vertex_count >= 2 {
        (2.0 * edge_count as f64) / ((vertex_count * (vertex_count - 1)) as f64)
    } else {
        0.0
    };
    let (
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
    ) = stats_or_zero(&vertex_incident_facets);
    let (vertex_degree_mean, vertex_degree_std, vertex_degree_min, vertex_degree_max) =
        stats_or_zero(&vertex_degrees);
    let (ridge_size_mean, ridge_size_std, ridge_size_min, ridge_size_max) =
        stats_or_zero(&ridge_sizes);
    let (
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
    ) = stats_or_zero(&facet_vertex_counts);
    let (
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    ) = stats_or_zero(&facet_neighbor_counts);

    features_skeleton::SkeletonFields {
        vertex_count,
        edge_count,
        ridge_count,
        is_simple: simple_vertices == vertex_count,
        simple_vertex_fraction,
        edge_density,
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
        vertex_degree_mean,
        vertex_degree_std,
        vertex_degree_min,
        vertex_degree_max,
        ridge_size_mean,
        ridge_size_std,
        ridge_size_min,
        ridge_size_max,
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    }
}

fn build_geometry_row(seed: u64, k: usize, m: usize, sample_index: usize) -> CandidateGeometryRow {
    let start = std::time::Instant::now();
    let geometry = generate_product_geometry(seed, k, m, sample_index)
        .unwrap_or_else(|| panic!("failed random-product candidate {k}x{m} sample {sample_index}"));
    assert!(
        geometry.volume.is_finite() && geometry.volume > 0.0,
        "volume must be positive"
    );
    let vertex_facets = vertex_facets_from_vertex_facet_incidence(&geometry.incidence);
    let edges = edges_from_vertex_facet_incidence(&geometry.incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(&geometry.incidence);
    let skeleton = compute_skeleton_fields_from_cached_parts(
        geometry.dual_vertices.len(),
        geometry.vertices.len(),
        &geometry.incidence,
        &vertex_facets,
        &edges,
        &two_faces,
    );
    let candidate_id = candidate_id(seed, k, m, sample_index);
    CandidateGeometryRow {
        schema: "sys-datascience.extreme-scalar-rejection-proposer.candidate-geometry.v2"
            .to_string(),
        candidate_id: candidate_id.clone(),
        name: format!("random_product_{k}x{m}_h0p8_1p2_{sample_index}"),
        poly_id: poly_id_from_dual_vertices(&geometry.dual_vertices),
        producer: "random-product".to_string(),
        source: ProductSource {
            producer: "random-product".to_string(),
            k,
            m,
            h_min: H_MIN,
            h_max: H_MAX,
            seed,
            sample_index,
            attempt: geometry.attempt,
        },
        bucket_id: bucket_id(k, m),
        seed,
        sample_index,
        attempt: geometry.attempt,
        h_min: H_MIN,
        h_max: H_MAX,
        product_k: k,
        product_m: m,
        facet_count: geometry.dual_vertices.len(),
        dual_vertices: vectors_to_arrays(&geometry.dual_vertices),
        volume: geometry.volume,
        vertices: vectors_to_arrays(&geometry.vertices),
        vertex_facet_incidence: incidence_to_rows(&geometry.incidence),
        vertex_count: skeleton.vertex_count,
        edge_count: skeleton.edge_count,
        ridge_count: skeleton.ridge_count,
        is_simple: skeleton.is_simple,
        simple_vertex_fraction: skeleton.simple_vertex_fraction,
        validation_status: "trusted_generated_geometry".to_string(),
        validation_marker:
            "direct_lagrangian_product_geometry_from_2d_h_reps; volume_is_product_of_2d_polygon_areas; selected_candidates_are_reconstructed_by_SysLandscapePolytopeCache_in_sys_stage".to_string(),
        time_geometry_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

const VERTEX_COVARIANCE_CONDITION_LIMIT: f64 = 1.0e10;

#[derive(Clone, Debug)]
struct VertexCovarianceDiagnostics {
    rho: Option<f64>,
    nu1: Option<f64>,
    nu2: Option<f64>,
    condition: Option<f64>,
    ordinary_eigenvalue_min: Option<f64>,
    ordinary_eigenvalue_max: Option<f64>,
    distinct_vertex_count: usize,
    expected_vertex_count: usize,
    status: String,
}

fn canonical_distinct_vertices(vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    let mut keyed = BTreeMap::<[u64; 4], Vector4<f64>>::new();
    for vertex in vertices {
        let key = std::array::from_fn(|index| {
            if vertex[index] == 0.0 {
                0.0_f64.to_bits()
            } else {
                vertex[index].to_bits()
            }
        });
        keyed.entry(key).or_insert_with(|| *vertex);
    }
    keyed.into_values().collect()
}

/// Computes the population covariance of the canonical distinct primal vertices
/// and its two four-dimensional Williamson eigenvalues.  For positive definite
/// `C`, the squared Williamson eigenvalues are the roots of
/// `t^2 - s t + det(C)`, where `s = -tr((J C)^2)/2`, in project coordinate
/// order `(q1,q2,p1,p2)`.  The result is explicitly ineligible when ordinary
/// eigenvalue conditioning or the invariant calculation is numerically unsafe.
fn vertex_covariance_diagnostics(
    vertices: &[Vector4<f64>],
    expected_vertex_count: usize,
) -> VertexCovarianceDiagnostics {
    let vertices = canonical_distinct_vertices(vertices);
    let count = vertices.len();
    let mut result = VertexCovarianceDiagnostics {
        rho: None,
        nu1: None,
        nu2: None,
        condition: None,
        ordinary_eigenvalue_min: None,
        ordinary_eigenvalue_max: None,
        distinct_vertex_count: count,
        expected_vertex_count,
        status: String::new(),
    };
    if count != expected_vertex_count {
        result.status = "unexpected_distinct_vertex_count".to_string();
        return result;
    }
    if count < 2
        || vertices
            .iter()
            .any(|vertex| !vertex.iter().all(|x| x.is_finite()))
    {
        result.status = "insufficient_or_nonfinite_vertices".to_string();
        return result;
    }
    let mean = vertices
        .iter()
        .fold(Vector4::zeros(), |sum, vertex| sum + vertex)
        / count as f64;
    let covariance = vertices.iter().fold(Matrix4::zeros(), |sum, vertex| {
        let centered = vertex - mean;
        sum + centered * centered.transpose()
    }) / count as f64;
    if !covariance.iter().all(|x| x.is_finite()) {
        result.status = "nonfinite_covariance".to_string();
        return result;
    }
    let ordinary = SymmetricEigen::new(covariance);
    let lambda_min = ordinary
        .eigenvalues
        .iter()
        .copied()
        .reduce(f64::min)
        .unwrap();
    let lambda_max = ordinary
        .eigenvalues
        .iter()
        .copied()
        .reduce(f64::max)
        .unwrap();
    result.ordinary_eigenvalue_min = Some(lambda_min);
    result.ordinary_eigenvalue_max = Some(lambda_max);
    if !(lambda_min.is_finite() && lambda_max.is_finite() && lambda_min > 0.0) {
        result.status = "covariance_not_positive_definite".to_string();
        return result;
    }
    let condition = lambda_max / lambda_min;
    result.condition = Some(condition);
    if !condition.is_finite() || condition > VERTEX_COVARIANCE_CONDITION_LIMIT {
        result.status = "ordinary_condition_exceeds_limit".to_string();
        return result;
    }
    let j = Matrix4::new(
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
    );
    let jc = j * covariance;
    let sum_nu_squared = -0.5 * (jc * jc).trace();
    let product_nu_squared = covariance.determinant();
    let raw_discriminant = sum_nu_squared * sum_nu_squared - 4.0 * product_nu_squared;
    let discriminant_scale = (sum_nu_squared * sum_nu_squared)
        .abs()
        .max((4.0 * product_nu_squared).abs())
        .max(1.0);
    if !sum_nu_squared.is_finite()
        || !product_nu_squared.is_finite()
        || sum_nu_squared <= 0.0
        || product_nu_squared <= 0.0
        || raw_discriminant < -1.0e-12 * discriminant_scale
    {
        result.status = "unstable_williamson_invariants".to_string();
        return result;
    }
    let discriminant = raw_discriminant.max(0.0).sqrt();
    let nu2_squared = 0.5 * (sum_nu_squared + discriminant);
    // Recover the smaller root from the product to avoid cancellation when the
    // two Williamson eigenvalues are far apart.
    let nu1_squared = product_nu_squared / nu2_squared;
    if !(nu1_squared.is_finite()
        && nu2_squared.is_finite()
        && nu1_squared > 0.0
        && nu2_squared >= nu1_squared)
    {
        result.status = "unstable_williamson_roots".to_string();
        return result;
    }
    let nu1 = nu1_squared.sqrt();
    let nu2 = nu2_squared.sqrt();
    let rho = nu2 / nu1;
    if !rho.is_finite() {
        result.status = "nonfinite_rho".to_string();
        return result;
    }
    result.rho = Some(rho);
    result.nu1 = Some(nu1);
    result.nu2 = Some(nu2);
    result.status = "eligible".to_string();
    result
}

fn feature_row_from_geometry(row: &CandidateGeometryRow) -> CandidateFeatureRow {
    let start = std::time::Instant::now();
    let incidence = rows_to_incidence(&row.vertex_facet_incidence);
    let vertices = arrays_to_vectors(&row.vertices);
    let vertex_facets = vertex_facets_from_vertex_facet_incidence(&incidence);
    let edges = edges_from_vertex_facet_incidence(&incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(&incidence);
    let skeleton = compute_skeleton_fields_from_cached_parts(
        row.facet_count,
        row.vertices.len(),
        &incidence,
        &vertex_facets,
        &edges,
        &two_faces,
    );
    let face = features_face_symplectic::compute_face_symplectic_fields(
        &two_faces,
        &vertices,
        &incidence,
        row.volume.sqrt(),
    );
    let covariance =
        vertex_covariance_diagnostics(&vertices, row.product_k.saturating_mul(row.product_m));
    CandidateFeatureRow {
        schema: "sys-datascience.extreme-scalar-rejection-proposer.candidate-feature.v3"
            .to_string(),
        candidate_id: row.candidate_id.clone(),
        name: row.name.clone(),
        poly_id: row.poly_id.clone(),
        producer: row.producer.clone(),
        source: row.source.clone(),
        bucket_id: row.bucket_id.clone(),
        seed: row.seed,
        sample_index: row.sample_index,
        attempt: row.attempt,
        h_min: row.h_min,
        h_max: row.h_max,
        product_k: row.product_k,
        product_m: row.product_m,
        facet_count: row.facet_count,
        volume: row.volume,
        vertex_count: skeleton.vertex_count,
        edge_count: skeleton.edge_count,
        ridge_count: skeleton.ridge_count,
        is_simple: skeleton.is_simple,
        simple_vertex_fraction: skeleton.simple_vertex_fraction,
        vertex_covariance_rho: covariance.rho,
        vertex_covariance_nu1: covariance.nu1,
        vertex_covariance_nu2: covariance.nu2,
        vertex_covariance_condition: covariance.condition,
        vertex_covariance_ordinary_eigenvalue_min: covariance.ordinary_eigenvalue_min,
        vertex_covariance_ordinary_eigenvalue_max: covariance.ordinary_eigenvalue_max,
        vertex_covariance_distinct_vertex_count: covariance.distinct_vertex_count,
        vertex_covariance_expected_vertex_count: covariance.expected_vertex_count,
        vertex_covariance_status: covariance.status,
        edge_density: skeleton.edge_density,
        vertex_incident_facets_mean: skeleton.vertex_incident_facets_mean,
        vertex_incident_facets_std: skeleton.vertex_incident_facets_std,
        vertex_incident_facets_min: skeleton.vertex_incident_facets_min,
        vertex_incident_facets_max: skeleton.vertex_incident_facets_max,
        vertex_degree_mean: skeleton.vertex_degree_mean,
        vertex_degree_std: skeleton.vertex_degree_std,
        vertex_degree_min: skeleton.vertex_degree_min,
        vertex_degree_max: skeleton.vertex_degree_max,
        ridge_size_mean: skeleton.ridge_size_mean,
        ridge_size_std: skeleton.ridge_size_std,
        ridge_size_min: skeleton.ridge_size_min,
        ridge_size_max: skeleton.ridge_size_max,
        facet_vertex_count_mean: skeleton.facet_vertex_count_mean,
        facet_vertex_count_std: skeleton.facet_vertex_count_std,
        facet_vertex_count_min: skeleton.facet_vertex_count_min,
        facet_vertex_count_max: skeleton.facet_vertex_count_max,
        facet_neighbor_count_mean: skeleton.facet_neighbor_count_mean,
        facet_neighbor_count_std: skeleton.facet_neighbor_count_std,
        facet_neighbor_count_min: skeleton.facet_neighbor_count_min,
        facet_neighbor_count_max: skeleton.facet_neighbor_count_max,
        ridge_symp_area_ordered_face_count: face.ridge_symp_area_ordered_face_count,
        ridge_symp_area_ordering_failure_count: face.ridge_symp_area_ordering_failure_count,
        ridge_symp_area_ordered_fraction: face.ridge_symp_area_ordered_fraction,
        ridge_symp_area_mean_over_volume_sqrt: face.ridge_symp_area_mean / row.volume.sqrt(),
        ridge_symp_area_std_over_volume_sqrt: face.ridge_symp_area_std / row.volume.sqrt(),
        ridge_symp_area_min_over_volume_sqrt: face.ridge_symp_area_min / row.volume.sqrt(),
        ridge_symp_area_max_over_volume_sqrt: face.ridge_symp_area_max / row.volume.sqrt(),
        ridge_symp_area_q25_over_volume_sqrt: face.ridge_symp_area_q25 / row.volume.sqrt(),
        ridge_symp_area_median_over_volume_sqrt: face.ridge_symp_area_median / row.volume.sqrt(),
        ridge_symp_area_q75_over_volume_sqrt: face.ridge_symp_area_q75 / row.volume.sqrt(),
        ridge_symp_area_q90_over_volume_sqrt: face.ridge_symp_area_q90 / row.volume.sqrt(),
        ridge_symp_area_q95_over_volume_sqrt: face.ridge_symp_area_q95 / row.volume.sqrt(),
        ridge_symp_area_sum_over_volume_sqrt: face.ridge_symp_area_sum / row.volume.sqrt(),
        ridge_symp_area_max_share: face.ridge_symp_area_max_share,
        ridge_symp_area_top3_share: face.ridge_symp_area_top3_share,
        ridge_symp_area_le_1em3_over_volume_sqrt_fraction: face
            .ridge_symp_area_le_1em3_over_volume_sqrt_fraction,
        ridge_symp_area_le_1em2_over_volume_sqrt_fraction: face
            .ridge_symp_area_le_1em2_over_volume_sqrt_fraction,
        ridge_symp_area_le_1em1_over_volume_sqrt_fraction: face
            .ridge_symp_area_le_1em1_over_volume_sqrt_fraction,
        ridge_symp_area_entropy: face.ridge_symp_area_entropy,
        ridge_symp_area_effective_face_count: face.ridge_symp_area_effective_face_count,
        ridge_symp_area_normalized_entropy: face.ridge_symp_area_normalized_entropy,
        time_feature_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

fn sort_selection_indices(
    indices: &mut [usize],
    features: &[CandidateFeatureRow],
    rule: &SelectionRule,
) {
    indices.sort_by(|&left, &right| {
        let value_order = match rule.direction {
            SelectionDirection::Low => rule
                .value(&features[left])
                .total_cmp(&rule.value(&features[right])),
            SelectionDirection::High => rule
                .value(&features[right])
                .total_cmp(&rule.value(&features[left])),
        };
        value_order.then_with(|| {
            features[left]
                .candidate_id
                .cmp(&features[right].candidate_id)
        })
    });
}

fn fraction_count(row_count: usize, fraction: f64) -> usize {
    if row_count == 0 {
        0
    } else {
        ((fraction * row_count as f64).ceil() as usize)
            .max(1)
            .min(row_count)
    }
}

fn cascade_fraction_label(fraction: f64) -> String {
    format!("{fraction:.6}").replace('.', "p")
}

fn per_bucket_cascade_selection_sets(
    features: &[CandidateFeatureRow],
    cascade: &PerBucketCascade,
) -> Vec<SelectionSet> {
    let primary_rule = SelectionRule::single(
        cascade.primary_feature,
        cascade.primary_direction,
        &[],
        &[],
        &[],
    );
    let secondary_rule = SelectionRule::single(
        cascade.secondary_feature,
        cascade.secondary_direction,
        &[],
        &[],
        &[],
    );
    let mut buckets = BTreeMap::<String, Vec<usize>>::new();
    for (index, row) in features.iter().enumerate() {
        buckets
            .entry(row.bucket_id.clone())
            .or_default()
            .push(index);
    }

    let mut stage_1 = Vec::new();
    let mut cascade_indices = Vec::new();
    for mut bucket in buckets.into_values() {
        sort_selection_indices(&mut bucket, features, &primary_rule);
        bucket.truncate(fraction_count(bucket.len(), cascade.primary_fraction));
        stage_1.extend(bucket.iter().copied());
        sort_selection_indices(&mut bucket, features, &secondary_rule);
        bucket.truncate(fraction_count(bucket.len(), cascade.secondary_fraction));
        cascade_indices.extend(bucket);
    }

    let primary_label = format!(
        "{}_{}",
        cascade.primary_direction.as_str(),
        cascade.primary_feature.name()
    );
    let secondary_label = format!(
        "{}_{}",
        cascade.secondary_direction.as_str(),
        cascade.secondary_feature.name()
    );
    let primary_fraction_label = cascade_fraction_label(cascade.primary_fraction);
    let secondary_fraction_label = cascade_fraction_label(cascade.secondary_fraction);
    let mut sets = Vec::new();
    if cascade.emit_stage_1_comparator {
        sets.push(SelectionSet {
            id: format!("per_bucket_{primary_label}_fraction_{primary_fraction_label}"),
            kind: "per_bucket_cascade_stage_1_comparator".to_string(),
            feature: cascade.primary_feature,
            direction: cascade.primary_direction,
            requested_budget: format!(
                "per_bucket_fraction={:.6};rounding=ceil_min_one",
                cascade.primary_fraction
            ),
            indices: stage_1,
            require_disjoint_baseline: true,
        });
    }
    sets.push(SelectionSet {
        id: format!(
            "per_bucket_{primary_label}_fraction_{primary_fraction_label}_then_{secondary_label}_fraction_{secondary_fraction_label}"
        ),
        kind: "per_bucket_two_stage_cascade".to_string(),
        feature: cascade.primary_feature,
        direction: cascade.primary_direction,
        requested_budget: format!(
            "primary_per_bucket_fraction={:.6};secondary_within_primary_fraction={:.6};rounding=ceil_min_one",
            cascade.primary_fraction, cascade.secondary_fraction
        ),
        indices: cascade_indices,
        require_disjoint_baseline: true,
    });
    sets
}

fn frozen_control_hash(control_seed: u64, candidate_id: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"frozen-canonical-vertex-covariance-control-v1");
    hasher.update(&control_seed.to_le_bytes());
    hasher.update(candidate_id.as_bytes());
    *hasher.finalize().as_bytes()
}

fn frozen_covariance_validation_selection_sets(
    features: &[CandidateFeatureRow],
    design: &FrozenCovarianceValidation,
) -> Vec<SelectionSet> {
    let rho_rule = SelectionRule::single(
        ScalarFeature::VertexCovarianceRho,
        SelectionDirection::Low,
        &[],
        &[],
        &[],
    );
    let ridge_primary_rule = SelectionRule::single(
        ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
        SelectionDirection::Low,
        &[],
        &[],
        &[],
    );
    let ridge_secondary_rule = SelectionRule::single(
        ScalarFeature::RidgeSympAreaMaxShare,
        SelectionDirection::Low,
        &[],
        &[],
        &[],
    );
    let mut buckets = BTreeMap::<String, Vec<usize>>::new();
    for (index, row) in features.iter().enumerate() {
        if row.vertex_covariance_status == "eligible" && row.vertex_covariance_rho.is_some() {
            buckets
                .entry(row.bucket_id.clone())
                .or_default()
                .push(index);
        }
    }
    assert_eq!(
        buckets.len(),
        PRODUCT_PAIRS.len(),
        "all product buckets must have eligible rows"
    );

    let mut rho_indices = Vec::new();
    let mut ridge_indices = Vec::new();
    let mut by_bucket = Vec::new();
    for (bucket_id, mut eligible) in buckets {
        let mut rho_bucket = eligible.clone();
        sort_selection_indices(&mut rho_bucket, features, &rho_rule);
        rho_bucket.truncate(fraction_count(rho_bucket.len(), design.rho_fraction));

        sort_selection_indices(&mut eligible, features, &ridge_primary_rule);
        eligible.truncate(fraction_count(
            eligible.len(),
            design.ridge_primary_fraction,
        ));
        sort_selection_indices(&mut eligible, features, &ridge_secondary_rule);
        eligible.truncate(fraction_count(
            eligible.len(),
            design.ridge_secondary_fraction,
        ));

        rho_indices.extend(rho_bucket.iter().copied());
        ridge_indices.extend(eligible.iter().copied());
        by_bucket.push((bucket_id, rho_bucket, eligible));
    }
    let arm_union = rho_indices
        .iter()
        .chain(&ridge_indices)
        .copied()
        .collect::<BTreeSet<_>>();
    let mut control_indices = Vec::new();
    for (bucket_id, _, _) in &by_bucket {
        let mut pool = (0..features.len())
            .filter(|index| {
                features[*index].bucket_id == *bucket_id
                    && features[*index].vertex_covariance_status == "eligible"
                    && !arm_union.contains(index)
            })
            .collect::<Vec<_>>();
        pool.sort_by(|&left, &right| {
            frozen_control_hash(design.control_seed, &features[left].candidate_id)
                .cmp(&frozen_control_hash(
                    design.control_seed,
                    &features[right].candidate_id,
                ))
                .then_with(|| {
                    features[left]
                        .candidate_id
                        .cmp(&features[right].candidate_id)
                })
        });
        assert!(
            pool.len() >= design.control_count_per_bucket,
            "bucket {bucket_id} has insufficient disjoint control rows"
        );
        control_indices.extend(pool.into_iter().take(design.control_count_per_bucket));
    }
    assert!(control_indices
        .iter()
        .all(|index| !arm_union.contains(index)));
    vec![
        SelectionSet {
            id: "frozen_low_vertex_covariance_rho_bottom_0p005".to_string(),
            kind: "frozen_covariance_rho_arm".to_string(),
            feature: ScalarFeature::VertexCovarianceRho,
            direction: SelectionDirection::Low,
            requested_budget: format!("per_bucket_fraction={:.6};rounding=ceil_min_one", design.rho_fraction),
            indices: rho_indices,
            require_disjoint_baseline: true,
        },
        SelectionSet {
            id: "frozen_ridge_bottom_0p01_then_bottom_0p5".to_string(),
            kind: "frozen_ridge_comparator_arm".to_string(),
            feature: ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
            direction: SelectionDirection::Low,
            requested_budget: format!(
                "primary_per_bucket_fraction={:.6};secondary_within_primary_fraction={:.6};rounding=ceil_min_one",
                design.ridge_primary_fraction, design.ridge_secondary_fraction
            ),
            indices: ridge_indices,
            require_disjoint_baseline: true,
        },
        SelectionSet {
            id: "frozen_shared_disjoint_control_25_per_bucket".to_string(),
            kind: "frozen_shared_disjoint_control".to_string(),
            feature: ScalarFeature::VertexCovarianceRho,
            direction: SelectionDirection::Low,
            requested_budget: format!("count_per_bucket={};control_seed={}", design.control_count_per_bucket, design.control_seed),
            indices: control_indices,
            require_disjoint_baseline: true,
        },
    ]
}

fn selection_sets(features: &[CandidateFeatureRow], args: &Args) -> Vec<SelectionSet> {
    if let Some(design) = &args.frozen_covariance_validation {
        return frozen_covariance_validation_selection_sets(features, design);
    }
    if let Some(cascade) = &args.per_bucket_cascade {
        return per_bucket_cascade_selection_sets(features, cascade);
    }
    let mut sets = Vec::new();
    for rule in &args.selection_rules {
        let mut global = (0..features.len()).collect::<Vec<_>>();
        sort_selection_indices(&mut global, features, rule);
        let rule_label = rule.label(args.use_legacy_default_rule_alias);
        for budget in &rule.global_top {
            sets.push(SelectionSet {
                id: format!("global_{rule_label}_top_{budget}"),
                kind: format!("global_{rule_label}"),
                feature: rule.feature,
                direction: rule.direction,
                requested_budget: budget.to_string(),
                indices: global.iter().take(*budget).copied().collect(),
                require_disjoint_baseline: false,
            });
        }
        for percentile in &rule.percentile_cutoffs {
            let count = ((*percentile * features.len() as f64).ceil() as usize).max(1);
            sets.push(SelectionSet {
                id: format!(
                    "global_{rule_label}_percentile_{}",
                    format!("{percentile:.4}").replace('.', "p")
                ),
                kind: format!("global_{rule_label}_percentile"),
                feature: rule.feature,
                direction: rule.direction,
                requested_budget: format!("{percentile:.4}"),
                indices: global.iter().take(count).copied().collect(),
                require_disjoint_baseline: false,
            });
        }
        for budget in &rule.per_bucket_top {
            let mut indices = Vec::new();
            for (k, m) in PRODUCT_PAIRS {
                let mut bucket = (0..features.len())
                    .filter(|&idx| features[idx].product_k == *k && features[idx].product_m == *m)
                    .collect::<Vec<_>>();
                sort_selection_indices(&mut bucket, features, rule);
                indices.extend(bucket.into_iter().take(*budget));
            }
            sets.push(SelectionSet {
                id: format!("per_bucket_{rule_label}_top_{budget}"),
                kind: format!("per_bucket_{rule_label}"),
                feature: rule.feature,
                direction: rule.direction,
                requested_budget: budget.to_string(),
                indices,
                require_disjoint_baseline: false,
            });
        }
    }
    sets
}

fn hash_for_baseline(selection_id: &str, replicate: usize, candidate_id: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"feature-first-scalar-rejection-baseline-v1");
    hasher.update(selection_id.as_bytes());
    hasher.update(&(replicate as u64).to_le_bytes());
    hasher.update(candidate_id.as_bytes());
    *hasher.finalize().as_bytes()
}

fn baseline_indices_for_selection(
    features: &[CandidateFeatureRow],
    selection: &SelectionSet,
    replicate: usize,
) -> Vec<usize> {
    let selected = selection.indices.iter().copied().collect::<BTreeSet<_>>();
    let mut counts = BTreeMap::<String, usize>::new();
    for idx in &selection.indices {
        *counts.entry(features[*idx].bucket_id.clone()).or_default() += 1;
    }
    let mut out = Vec::new();
    for (bucket_id, count) in counts {
        let mut pool = (0..features.len())
            .filter(|idx| features[*idx].bucket_id == bucket_id && !selected.contains(idx))
            .collect::<Vec<_>>();
        if pool.len() < count && !selection.require_disjoint_baseline {
            pool = (0..features.len())
                .filter(|idx| features[*idx].bucket_id == bucket_id)
                .collect::<Vec<_>>();
        }
        assert!(
            pool.len() >= count,
            "bucket {bucket_id} has {count} selected rows but only {} eligible baseline candidates",
            pool.len()
        );
        pool.sort_by(|&left, &right| {
            hash_for_baseline(&selection.id, replicate, &features[left].candidate_id).cmp(
                &hash_for_baseline(&selection.id, replicate, &features[right].candidate_id),
            )
        });
        let matched = pool.into_iter().take(count).collect::<Vec<_>>();
        assert_eq!(matched.len(), count, "baseline count must match selection");
        if selection.require_disjoint_baseline {
            assert!(
                matched.iter().all(|idx| !selected.contains(idx)),
                "baseline must be disjoint from selected rows"
            );
        }
        out.extend(matched);
    }
    out
}

fn selection_rule_values(
    row: &CandidateFeatureRow,
    rules: &[SelectionRule],
) -> Vec<SelectionRuleValue> {
    rules
        .iter()
        .map(|rule| SelectionRuleValue {
            selection_feature: rule.feature.name().to_string(),
            selection_direction: rule.direction.as_str().to_string(),
            selection_feature_value: rule.value(row),
        })
        .collect()
}

fn pre_target_selection_rows(
    features: &[CandidateFeatureRow],
    selections: &[SelectionSet],
    args: &Args,
    baseline_replicates: usize,
) -> Vec<PreTargetSelectionRow> {
    let mut selection_memberships = BTreeMap::<usize, Vec<String>>::new();
    for selection in selections {
        for idx in &selection.indices {
            selection_memberships
                .entry(*idx)
                .or_default()
                .push(selection.id.clone());
        }
    }
    let mut baseline_memberships = BTreeMap::<usize, Vec<String>>::new();
    for selection in selections {
        for replicate in 0..baseline_replicates {
            let baseline_id = format!("{}__baseline_rep_{replicate}", selection.id);
            for idx in baseline_indices_for_selection(features, selection, replicate) {
                baseline_memberships
                    .entry(idx)
                    .or_default()
                    .push(baseline_id.clone());
            }
        }
    }
    let mut indices = BTreeSet::<usize>::new();
    indices.extend(selection_memberships.keys().copied());
    indices.extend(baseline_memberships.keys().copied());
    indices
        .into_iter()
        .map(|idx| {
            let row = &features[idx];
            let rule_values = selection_rule_values(row, &args.selection_rules);
            let primary_rule_value = rule_values
                .first()
                .expect("at least one selection rule is configured");
            let selection_ids = selection_memberships.get(&idx).cloned().unwrap_or_default();
            let baseline_ids = baseline_memberships.get(&idx).cloned().unwrap_or_default();
            let mut evaluation_roles = Vec::new();
            if !selection_ids.is_empty() {
                evaluation_roles.push("selected".to_string());
            }
            if !baseline_ids.is_empty() {
                evaluation_roles.push("baseline".to_string());
            }
            PreTargetSelectionRow {
                schema: "sys-datascience.extreme-scalar-rejection-proposer.pre-target-selection.v2"
                    .to_string(),
                candidate_id: row.candidate_id.clone(),
                name: row.name.clone(),
                poly_id: row.poly_id.clone(),
                producer: row.producer.clone(),
                source: row.source.clone(),
                bucket_id: row.bucket_id.clone(),
                selection_feature: primary_rule_value.selection_feature.clone(),
                selection_direction: primary_rule_value.selection_direction.clone(),
                selection_feature_value: primary_rule_value.selection_feature_value,
                selection_rule_values: rule_values,
                selection_ids,
                baseline_ids,
                evaluation_roles,
                stage_order: "selected_before_target_evaluation".to_string(),
            }
        })
        .collect()
}

fn evaluate_one_selection_row(
    geometry: &CandidateGeometryRow,
    selected: &PreTargetSelectionRow,
) -> EvaluatedRow {
    let dual_vertices = arrays_to_vectors(&geometry.dual_vertices);
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
        .unwrap_or_else(|| panic!("reconstruct evaluation candidate {}", geometry.candidate_id));
    let start = std::time::Instant::now();
    let capacity_result = capacity_billiard(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .unwrap_or_else(|_| panic!("capacity for {}", selected.candidate_id));
    let time_capacity_ms = start.elapsed().as_secs_f64() * 1000.0;
    let capacity = capacity_result.min_action;
    let sys = systolic_ratio(capacity, geometry.volume);
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        .unwrap_or_else(|_| panic!("classify facets for {}", selected.candidate_id));
    let bounces = bounce_count_from_sigma_for_facets(
        &classification.q_indices,
        &classification.p_indices,
        capacity_result.best_sigma(),
    )
    .unwrap_or_else(|| panic!("bounce count for {}", selected.candidate_id));
    EvaluatedRow {
        schema: "sys-datascience.extreme-scalar-rejection-proposer.evaluated-target.v2".to_string(),
        candidate_id: selected.candidate_id.clone(),
        name: selected.name.clone(),
        poly_id: selected.poly_id.clone(),
        bucket_id: selected.bucket_id.clone(),
        product_k: geometry.product_k,
        product_m: geometry.product_m,
        sample_index: geometry.sample_index,
        attempt: geometry.attempt,
        evaluation_roles: selected.evaluation_roles.clone(),
        selection_ids: selected.selection_ids.clone(),
        baseline_ids: selected.baseline_ids.clone(),
        selection_feature: selected.selection_feature.clone(),
        selection_direction: selected.selection_direction.clone(),
        selection_feature_value: selected.selection_feature_value,
        selection_rule_values: selected.selection_rule_values.clone(),
        capacity,
        sys,
        bounces,
        time_capacity_ms,
    }
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn quantile(sorted_values: &[f64], q: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let position = q.clamp(0.0, 1.0) * (sorted_values.len().saturating_sub(1) as f64);
    let lo = position.floor() as usize;
    let hi = position.ceil() as usize;
    if lo == hi {
        sorted_values[lo]
    } else {
        let weight = position - lo as f64;
        sorted_values[lo] * (1.0 - weight) + sorted_values[hi] * weight
    }
}

fn bucket_counts_from_selection_rows(rows: &[&PreTargetSelectionRow]) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for row in rows {
        *counts.entry(row.bucket_id.clone()).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(bucket, count)| format!("{bucket}:{count}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn evaluated_sys_by_candidate_id(evaluated_rows: &[EvaluatedRow]) -> BTreeMap<String, f64> {
    evaluated_rows
        .iter()
        .map(|row| (row.candidate_id.clone(), row.sys))
        .collect()
}

fn baseline_membership_count(row: &PreTargetSelectionRow, selection_id: &str) -> usize {
    let prefix = format!("{selection_id}__baseline_rep_");
    row.baseline_ids
        .iter()
        .filter(|baseline_id| baseline_id.starts_with(&prefix))
        .count()
}

fn validate_selection_artifact_matches_plan(
    plan: &SelectionPlanReport,
    selection_rows: &[PreTargetSelectionRow],
) {
    let planned_ids = plan
        .selections
        .iter()
        .map(|selection| selection.selection_id.as_str())
        .collect::<BTreeSet<_>>();
    for row in selection_rows {
        for selection_id in &row.selection_ids {
            assert!(
                planned_ids.contains(selection_id.as_str()),
                "selection artifact contains selection id {selection_id} that is absent from selection-plan.json"
            );
        }
        for baseline_id in &row.baseline_ids {
            let Some((selection_id, _)) = baseline_id.split_once("__baseline_rep_") else {
                panic!("baseline id {baseline_id} does not use the per_selection_matched naming convention");
            };
            assert!(
                planned_ids.contains(selection_id),
                "selection artifact contains baseline id {baseline_id} whose selection is absent from selection-plan.json"
            );
        }
    }
    for selection in &plan.selections {
        let selected_rows = selection_rows
            .iter()
            .filter(|row| row.selection_ids.contains(&selection.selection_id))
            .count();
        assert_eq!(
            selected_rows, selection.selected_rows,
            "selection-plan.json and selected-candidates-before-sys.jsonl disagree for {}",
            selection.selection_id
        );
    }
}

fn selection_summary_rows_from_plan(
    plan: &SelectionPlanReport,
    selection_rows: &[PreTargetSelectionRow],
    evaluated_rows: &[EvaluatedRow],
) -> Vec<SelectionSummaryRow> {
    let sys_by_candidate_id = evaluated_sys_by_candidate_id(evaluated_rows);
    let mut out = Vec::new();
    for selection in &plan.selections {
        let selected_rows = selection_rows
            .iter()
            .filter(|row| row.selection_ids.contains(&selection.selection_id))
            .collect::<Vec<_>>();
        let selected_sys = selected_rows
            .iter()
            .map(|row| {
                *sys_by_candidate_id
                    .get(&row.candidate_id)
                    .expect("selected row was evaluated")
            })
            .collect::<Vec<_>>();
        let mut baseline_sys = Vec::new();
        for row in selection_rows {
            let count = baseline_membership_count(row, &selection.selection_id);
            for _ in 0..count {
                baseline_sys.push(
                    *sys_by_candidate_id
                        .get(&row.candidate_id)
                        .expect("baseline row was evaluated"),
                );
            }
        }
        baseline_sys.sort_by(f64::total_cmp);
        let baseline_mean = mean(&baseline_sys);
        let selected_mean = mean(&selected_sys);
        let baseline_p95 = quantile(&baseline_sys, 0.95);
        out.push(SelectionSummaryRow {
            selection_id: selection.selection_id.clone(),
            selection_kind: selection.selection_kind.clone(),
            requested_budget: selection.requested_budget.clone(),
            selected_rows: selected_rows.len(),
            selected_max_sys: selected_sys.iter().copied().reduce(f64::max).unwrap_or(0.0),
            selected_mean_sys: selected_mean,
            baseline_replicates: plan.baseline_replicates,
            baseline_rows_total: baseline_sys.len(),
            baseline_p90_sys: quantile(&baseline_sys, 0.90),
            baseline_p95_sys: baseline_p95,
            selected_rows_above_baseline_p95: selected_sys
                .iter()
                .filter(|value| **value > baseline_p95)
                .count(),
            baseline_mean_sys: baseline_mean,
            improvement_vs_baseline_mean_sys: selected_mean - baseline_mean,
            selected_bucket_counts: bucket_counts_from_selection_rows(&selected_rows),
        });
    }
    if plan.rules.len() > 1 && plan.per_bucket_cascade.is_none() {
        let selected_rows = selection_rows
            .iter()
            .filter(|row| !row.selection_ids.is_empty())
            .collect::<Vec<_>>();
        let baseline_rows = selection_rows
            .iter()
            .filter(|row| row.selection_ids.is_empty() && !row.baseline_ids.is_empty())
            .collect::<Vec<_>>();
        let selected_sys = selected_rows
            .iter()
            .map(|row| {
                *sys_by_candidate_id
                    .get(&row.candidate_id)
                    .expect("union selected row was evaluated")
            })
            .collect::<Vec<_>>();
        let mut baseline_sys = baseline_rows
            .iter()
            .map(|row| {
                *sys_by_candidate_id
                    .get(&row.candidate_id)
                    .expect("union baseline row was evaluated")
            })
            .collect::<Vec<_>>();
        baseline_sys.sort_by(f64::total_cmp);
        let baseline_mean = mean(&baseline_sys);
        let selected_mean = mean(&selected_sys);
        let baseline_p95 = quantile(&baseline_sys, 0.95);
        out.push(SelectionSummaryRow {
            selection_id: "union_all_selection_rules".to_string(),
            selection_kind: "union_all_selection_rules".to_string(),
            requested_budget: "unique_selected_union".to_string(),
            selected_rows: selected_rows.len(),
            selected_max_sys: selected_sys.iter().copied().reduce(f64::max).unwrap_or(0.0),
            selected_mean_sys: selected_mean,
            baseline_replicates: plan.baseline_replicates,
            baseline_rows_total: baseline_rows.len(),
            baseline_p90_sys: quantile(&baseline_sys, 0.90),
            baseline_p95_sys: baseline_p95,
            selected_rows_above_baseline_p95: selected_sys
                .iter()
                .filter(|value| **value > baseline_p95)
                .count(),
            baseline_mean_sys: baseline_mean,
            improvement_vs_baseline_mean_sys: selected_mean - baseline_mean,
            selected_bucket_counts: bucket_counts_from_selection_rows(&selected_rows),
        });
    }
    out
}

fn warn_if_report_args_differ_from_plan(args: &Args, plan: &SelectionPlanReport) {
    let current_rules = selection_rule_reports(&args.selection_rules);
    let differs = args.seed != plan.seed
        || args.candidates_per_bucket != plan.candidates_per_bucket
        || args.limit_total != plan.limit_total
        || args.baseline_policy != plan.baseline_policy
        || args.baseline_replicates != plan.baseline_replicates
        || args.rule_set != plan.rule_set
        || current_rules != plan.rules
        || args
            .per_bucket_cascade
            .as_ref()
            .map(PerBucketCascade::report)
            != plan.per_bucket_cascade
        || args.global_top != plan.global_top
        || args.per_bucket_top != plan.per_bucket_top
        || args.percentile_cutoffs != plan.percentile_cutoffs;
    if differs {
        eprintln!(
            "warning: current CLI/config selection arguments differ from selection-plan.json; reports use the frozen selection plan and selected-candidates-before-sys.jsonl"
        );
    }
}

fn unique_selected_indices(selections: &[SelectionSet]) -> BTreeSet<usize> {
    selections
        .iter()
        .flat_map(|selection| selection.indices.iter().copied())
        .collect()
}

fn unique_baseline_indices(
    features: &[CandidateFeatureRow],
    selections: &[SelectionSet],
    baseline_replicates: usize,
) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    for selection in selections {
        for replicate in 0..baseline_replicates {
            out.extend(baseline_indices_for_selection(
                features, selection, replicate,
            ));
        }
    }
    out
}

fn selection_rule_reports(rules: &[SelectionRule]) -> Vec<SelectionRuleReport> {
    rules
        .iter()
        .map(|rule| SelectionRuleReport {
            selection_feature: rule.feature.name().to_string(),
            selection_direction: rule.direction.as_str().to_string(),
            global_top: rule.global_top.clone(),
            per_bucket_top: rule.per_bucket_top.clone(),
            percentile_cutoffs: rule.percentile_cutoffs.clone(),
        })
        .collect()
}

fn geometry_path(out_dir: &Path) -> PathBuf {
    out_dir.join("candidate-geometry-cache.jsonl")
}

fn feature_path(out_dir: &Path) -> PathBuf {
    out_dir.join("candidate-feature-table.jsonl")
}

fn selection_path(out_dir: &Path) -> PathBuf {
    out_dir.join("selected-candidates-before-sys.jsonl")
}

fn evaluation_path(out_dir: &Path) -> PathBuf {
    out_dir.join("sys-evaluation-cache.jsonl")
}

fn selection_summary_path(out_dir: &Path) -> PathBuf {
    out_dir.join("selection-summary.tsv")
}

fn selection_plan_path(out_dir: &Path) -> PathBuf {
    out_dir.join("selection-plan.json")
}

fn resolved_run_config_path(out_dir: &Path) -> PathBuf {
    out_dir.join("resolved-run-config.json")
}

fn resolved_run_config(args: &Args) -> ResolvedRunConfig {
    ResolvedRunConfig {
        schema: "sys-datascience.extreme-scalar-rejection-proposer.resolved-run-config.v1"
            .to_string(),
        config_path: args
            .config_path
            .as_ref()
            .map(|path| path.display().to_string()),
        stage: args.stage.as_str().to_string(),
        out_dir: args.out_dir.display().to_string(),
        seed: args.seed,
        h_min: H_MIN,
        h_max: H_MAX,
        product_pairs: PRODUCT_PAIRS.iter().map(|&(k, m)| [k, m]).collect(),
        candidates_per_bucket: args.candidates_per_bucket,
        limit_total: args.limit_total,
        baseline_policy: args.baseline_policy.clone(),
        baseline_replicates: args.baseline_replicates,
        selection: ResolvedSelectionConfig {
            rule_set: args.rule_set.clone(),
            selection_feature: args.selection_feature.name().to_string(),
            selection_direction: args.selection_direction.as_str().to_string(),
            global_top: args.global_top.clone(),
            per_bucket_top: args.per_bucket_top.clone(),
            percentile_cutoffs: args.percentile_cutoffs.clone(),
            rules: selection_rule_reports(&args.selection_rules),
            per_bucket_cascade: args
                .per_bucket_cascade
                .as_ref()
                .map(PerBucketCascade::report),
            frozen_covariance_validation: args
                .frozen_covariance_validation
                .as_ref()
                .map(FrozenCovarianceValidation::report),
        },
        jobs: args.jobs,
        chunk_rows: args.chunk_rows,
        metadata: run_metadata(&args.baseline_policy),
    }
}

fn write_jsonl_rows<T: Serialize>(writer: &mut BufWriter<File>, rows: &[T], path: &Path) {
    for row in rows {
        serde_json::to_writer(&mut *writer, row)
            .unwrap_or_else(|err| panic!("serialize {}: {err}", path.display()));
        writeln!(writer).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
    }
    writer
        .flush()
        .unwrap_or_else(|err| panic!("flush {}: {err}", path.display()));
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    write_jsonl_rows(&mut writer, rows, path);
}

fn append_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|err| panic!("open {} for append: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    write_jsonl_rows(&mut writer, rows, path);
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    BufReader::new(file)
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("parse {} line {}: {err}", path.display(), idx + 1)
                })
            })
        })
        .collect()
}

fn write_json<T: Serialize>(path: &Path, value: &T) {
    let file = File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    serde_json::to_writer_pretty(BufWriter::new(file), value)
        .unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

fn write_selection_summary(path: &Path, rows: &[SelectionSummaryRow]) {
    let mut writer = BufWriter::new(
        File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display())),
    );
    writeln!(
        writer,
        "selection_id\tselection_kind\trequested_budget\tselected_rows\tselected_max_sys\tselected_mean_sys\tbaseline_replicates\tbaseline_rows_total\tbaseline_p90_sys\tbaseline_p95_sys\tselected_rows_above_baseline_p95\tbaseline_mean_sys\timprovement_vs_baseline_mean_sys\tselected_bucket_counts"
    )
    .expect("write header");
    for row in rows {
        writeln!(
            writer,
            "{}\t{}\t{}\t{}\t{:.15}\t{:.15}\t{}\t{}\t{:.15}\t{:.15}\t{}\t{:.15}\t{:.15}\t{}",
            row.selection_id,
            row.selection_kind,
            row.requested_budget,
            row.selected_rows,
            row.selected_max_sys,
            row.selected_mean_sys,
            row.baseline_replicates,
            row.baseline_rows_total,
            row.baseline_p90_sys,
            row.baseline_p95_sys,
            row.selected_rows_above_baseline_p95,
            row.baseline_mean_sys,
            row.improvement_vs_baseline_mean_sys,
            row.selected_bucket_counts
        )
        .expect("write row");
    }
    writer.flush().expect("flush selection summary");
}

fn blake3_file(path: &Path) -> String {
    let mut file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    hasher.finalize().to_hex().to_string()
}

fn count_jsonl_rows(path: &Path) -> usize {
    let file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    BufReader::new(file)
        .lines()
        .filter(|line| line.as_ref().is_ok_and(|line| !line.trim().is_empty()))
        .count()
}

fn candidate_tasks(args: &Args) -> Vec<(usize, usize, usize)> {
    let mut tasks = Vec::new();
    for (k, m) in PRODUCT_PAIRS {
        for sample_index in 0..args.candidates_per_bucket {
            tasks.push((*k, *m, sample_index));
            if args.limit_total.is_some_and(|limit| tasks.len() >= limit) {
                return tasks;
            }
        }
    }
    tasks
}

fn run_geometry_stage(args: &Args) {
    let started = std::time::Instant::now();
    let path = geometry_path(&args.out_dir);
    let file = File::create(&path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    let tasks = candidate_tasks(args);
    let total = tasks.len();
    println!(
        "stage=geometry_cache status=started rows={total} jobs={} chunk_rows={}",
        args.jobs, args.chunk_rows
    );
    let mut completed = 0usize;
    for chunk in tasks.chunks(args.chunk_rows) {
        let rows = chunk
            .par_iter()
            .map(|&(k, m, sample_index)| build_geometry_row(args.seed, k, m, sample_index))
            .collect::<Vec<_>>();
        write_jsonl_rows(&mut writer, &rows, &path);
        completed += rows.len();
        println!("stage=geometry_cache completed={completed}/{total}");
    }
    println!(
        "stage=geometry_cache status=wrote rows={total} wall_seconds={:.3}",
        started.elapsed().as_secs_f64()
    );
}

fn process_feature_chunk(
    writer: &mut BufWriter<File>,
    path: &Path,
    chunk: &mut Vec<CandidateGeometryRow>,
) -> usize {
    let rows = chunk
        .par_iter()
        .map(feature_row_from_geometry)
        .collect::<Vec<_>>();
    let count = rows.len();
    write_jsonl_rows(writer, &rows, path);
    chunk.clear();
    count
}

fn run_feature_stage(args: &Args) {
    let started = std::time::Instant::now();
    let in_path = geometry_path(&args.out_dir);
    let out_path = feature_path(&args.out_dir);
    let input =
        File::open(&in_path).unwrap_or_else(|err| panic!("open {}: {err}", in_path.display()));
    let output = File::create(&out_path)
        .unwrap_or_else(|err| panic!("create {}: {err}", out_path.display()));
    let mut writer = BufWriter::new(output);
    let mut chunk = Vec::with_capacity(args.chunk_rows);
    let mut completed = 0usize;
    println!(
        "stage=feature_table status=started jobs={} chunk_rows={}",
        args.jobs, args.chunk_rows
    );
    for (line_index, line) in BufReader::new(input).lines().enumerate() {
        let line = line.unwrap_or_else(|err| panic!("read {}: {err}", in_path.display()));
        if line.trim().is_empty() {
            continue;
        }
        let row = serde_json::from_str::<CandidateGeometryRow>(&line).unwrap_or_else(|err| {
            panic!(
                "parse {} line {} as geometry: {err}",
                in_path.display(),
                line_index + 1
            )
        });
        chunk.push(row);
        if chunk.len() == args.chunk_rows {
            completed += process_feature_chunk(&mut writer, &out_path, &mut chunk);
            println!("stage=feature_table completed={completed}");
        }
    }
    if !chunk.is_empty() {
        completed += process_feature_chunk(&mut writer, &out_path, &mut chunk);
    }
    println!(
        "stage=feature_table status=wrote rows={completed} wall_seconds={:.3}",
        started.elapsed().as_secs_f64()
    );
}

fn run_selection_stage(args: &Args) {
    let started = std::time::Instant::now();
    let features = read_jsonl::<CandidateFeatureRow>(&feature_path(&args.out_dir));
    let selections = selection_sets(&features, args);
    let baseline_replicates = if args.frozen_covariance_validation.is_some() {
        0
    } else {
        args.baseline_replicates
    };
    let selection_rows =
        pre_target_selection_rows(&features, &selections, args, baseline_replicates);
    write_jsonl(&selection_path(&args.out_dir), &selection_rows);
    let unique_selected = unique_selected_indices(&selections);
    let unique_baseline = unique_baseline_indices(&features, &selections, baseline_replicates);
    let unique_selected_or_baseline = unique_selected
        .union(&unique_baseline)
        .copied()
        .collect::<BTreeSet<_>>();
    let selected_rows_sum_over_sets = selections
        .iter()
        .map(|selection| selection.indices.len())
        .sum::<usize>();
    let plan = SelectionPlanReport {
        schema: if args.frozen_covariance_validation.is_some() {
            "sys-datascience.extreme-scalar-rejection-proposer.selection-plan.v4"
        } else if args.per_bucket_cascade.is_some() {
            "sys-datascience.extreme-scalar-rejection-proposer.selection-plan.v3"
        } else {
            "sys-datascience.extreme-scalar-rejection-proposer.selection-plan.v2"
        }
        .to_string(),
        metadata: Some(run_metadata(&args.baseline_policy)),
        seed: args.seed,
        candidates_per_bucket: args.candidates_per_bucket,
        limit_total: args.limit_total,
        feature_rows: features.len(),
        baseline_policy: args.baseline_policy.clone(),
        baseline_replicates,
        rule_set: args.rule_set.clone(),
        rules: selection_rule_reports(&args.selection_rules),
        per_bucket_cascade: args
            .per_bucket_cascade
            .as_ref()
            .map(PerBucketCascade::report),
        frozen_covariance_validation: args
            .frozen_covariance_validation
            .as_ref()
            .map(FrozenCovarianceValidation::report),
        selection_feature: args.selection_feature.name().to_string(),
        selection_direction: args.selection_direction.as_str().to_string(),
        global_top: args.global_top.clone(),
        per_bucket_top: args.per_bucket_top.clone(),
        percentile_cutoffs: args.percentile_cutoffs.clone(),
        selection_sets: selections.len(),
        selected_rows_sum_over_sets,
        unique_selected_rows: unique_selected.len(),
        unique_baseline_rows: unique_baseline.len(),
        unique_selected_or_baseline_rows: unique_selected_or_baseline.len(),
        selections: selections
            .iter()
            .map(|selection| SelectionPlanEntry {
                selection_id: selection.id.clone(),
                selection_kind: selection.kind.clone(),
                selection_feature: selection.feature.name().to_string(),
                selection_direction: selection.direction.as_str().to_string(),
                requested_budget: selection.requested_budget.clone(),
                selected_rows: selection.indices.len(),
            })
            .collect(),
    };
    write_json(&selection_plan_path(&args.out_dir), &plan);
    println!(
        "stage=pre_sys_selection status=wrote feature_rows={} selection_sets={} selected_rows_sum_over_sets={} unique_selected_rows={} unique_baseline_rows={} selection_rows={} wall_seconds={:.3}",
        features.len(),
        selections.len(),
        selected_rows_sum_over_sets,
        unique_selected.len(),
        unique_baseline.len(),
        selection_rows.len(),
        started.elapsed().as_secs_f64()
    );
}

fn existing_evaluation_ids(path: &Path) -> BTreeSet<String> {
    if !path.exists() {
        return BTreeSet::new();
    }
    read_jsonl::<EvaluatedRow>(path)
        .into_iter()
        .map(|row| row.candidate_id)
        .collect()
}

fn geometry_for_candidate_ids(
    path: &Path,
    wanted_ids: &BTreeSet<String>,
) -> BTreeMap<String, CandidateGeometryRow> {
    let input = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    let mut rows = BTreeMap::new();
    for (line_index, line) in BufReader::new(input).lines().enumerate() {
        let line = line.unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        if line.trim().is_empty() {
            continue;
        }
        let row = serde_json::from_str::<CandidateGeometryRow>(&line).unwrap_or_else(|err| {
            panic!(
                "parse {} line {} as geometry: {err}",
                path.display(),
                line_index + 1
            )
        });
        if wanted_ids.contains(&row.candidate_id) {
            rows.insert(row.candidate_id.clone(), row);
            if rows.len() == wanted_ids.len() {
                break;
            }
        }
    }
    rows
}

fn run_sys_stage(args: &Args) {
    let started = std::time::Instant::now();
    let selection_rows = read_jsonl::<PreTargetSelectionRow>(&selection_path(&args.out_dir));
    let eval_path = evaluation_path(&args.out_dir);
    let existing_ids = existing_evaluation_ids(&eval_path);
    let missing_selection_rows = selection_rows
        .into_iter()
        .filter(|row| !existing_ids.contains(&row.candidate_id))
        .collect::<Vec<_>>();
    if missing_selection_rows.is_empty() {
        println!(
            "stage=sys_evaluation status=skipped reason=all_selected_or_baseline_candidates_already_cached cached_rows={}",
            existing_ids.len()
        );
        return;
    }
    let wanted_ids = missing_selection_rows
        .iter()
        .map(|row| row.candidate_id.clone())
        .collect::<BTreeSet<_>>();
    let geometry_by_id = geometry_for_candidate_ids(&geometry_path(&args.out_dir), &wanted_ids);
    assert_eq!(
        geometry_by_id.len(),
        wanted_ids.len(),
        "geometry cache is missing selected or baseline candidates"
    );
    println!(
        "stage=sys_evaluation status=started missing_rows={} cached_rows={} jobs={}",
        missing_selection_rows.len(),
        existing_ids.len(),
        args.jobs
    );
    let evaluated = missing_selection_rows
        .par_iter()
        .map(|selected| {
            let geometry = geometry_by_id
                .get(&selected.candidate_id)
                .unwrap_or_else(|| panic!("missing geometry for {}", selected.candidate_id));
            evaluate_one_selection_row(geometry, selected)
        })
        .collect::<Vec<_>>();
    append_jsonl(&eval_path, &evaluated);
    println!(
        "stage=sys_evaluation status=appended rows={} wall_seconds={:.3}",
        evaluated.len(),
        started.elapsed().as_secs_f64()
    );
}

fn run_report_stage(args: &Args) {
    let started = std::time::Instant::now();
    let plan = serde_json::from_reader::<_, SelectionPlanReport>(BufReader::new(
        File::open(selection_plan_path(&args.out_dir))
            .unwrap_or_else(|err| panic!("open selection-plan.json: {err}")),
    ))
    .unwrap_or_else(|err| panic!("parse selection-plan.json: {err}"));
    warn_if_report_args_differ_from_plan(args, &plan);
    let selection_rows = read_jsonl::<PreTargetSelectionRow>(&selection_path(&args.out_dir));
    validate_selection_artifact_matches_plan(&plan, &selection_rows);
    let evaluated_all = read_jsonl::<EvaluatedRow>(&evaluation_path(&args.out_dir));
    let current_ids = selection_rows
        .iter()
        .map(|row| row.candidate_id.clone())
        .collect::<BTreeSet<_>>();
    let evaluated_current = evaluated_all
        .iter()
        .filter(|row| current_ids.contains(&row.candidate_id))
        .cloned()
        .collect::<Vec<_>>();
    let evaluated_current_ids = evaluated_current
        .iter()
        .map(|row| row.candidate_id.clone())
        .collect::<BTreeSet<_>>();
    let missing_sys_rows_for_current_selection = current_ids
        .iter()
        .filter(|candidate_id| !evaluated_current_ids.contains(*candidate_id))
        .count();
    assert_eq!(
        missing_sys_rows_for_current_selection, 0,
        "reports require sys rows for all current selected and baseline candidates"
    );
    let summary_rows = selection_summary_rows_from_plan(&plan, &selection_rows, &evaluated_current);
    let summary_path = selection_summary_path(&args.out_dir);
    write_selection_summary(&summary_path, &summary_rows);
    let max_sys_selected = summary_rows
        .iter()
        .filter(|row| row.selection_id != "union_all_selection_rules")
        .map(|row| row.selected_max_sys)
        .reduce(f64::max)
        .unwrap_or(0.0);
    let report = EvaluationReport {
        schema: "sys-datascience.extreme-scalar-rejection-proposer.evaluation-report.v2"
            .to_string(),
        question: format!(
            "Does the {} scalar-filter rule configuration enrich high sys random-product candidates when selection is made before target evaluation?",
            plan.rule_set
        ),
        metadata: plan
            .metadata
            .clone()
            .unwrap_or_else(|| run_metadata(&plan.baseline_policy)),
        architecture_status: "stage artifacts are rerunnable independently; geometry and feature construction stream in chunks; selection holds only compact feature rows; sys evaluation streams geometry and appends only missing selected/baseline candidate ids".to_string(),
        stage_interface: "--config <path> --stage all|geometry|features|selection|sys|reports; CLI flags remain smoke/override conveniences".to_string(),
        deterministic_output_status: "row ordering, candidate ids, selections, baselines, and hashes are deterministic for fixed args; time_*_ms and wall-clock log fields are timing-only and vary with jobs/load".to_string(),
        feature_cache_usage_status: "features use cached vertices, vertex_facet_incidence, and volume from candidate-geometry-cache.jsonl; feature generation does not reconstruct SysLandscapePolytopeCache from dual_vertices".to_string(),
        sys_cache_status: "sys evaluation must reconstruct SysLandscapePolytopeCache from cached dual_vertices because capacity_billiard requires exact dual vertices, facet-intersection, and omega-sign matrices not stored in the f64 geometry JSONL; reruns skip candidate ids already present in sys-evaluation-cache.jsonl; this cache is intentionally reusable by candidate_id across compatible selection configs".to_string(),
        seed: plan.seed,
        h_min: H_MIN,
        h_max: H_MAX,
        candidates_per_bucket: plan.candidates_per_bucket,
        limit_total: plan.limit_total,
        product_buckets: PRODUCT_PAIRS.len(),
        jobs: args.jobs,
        chunk_rows: args.chunk_rows,
        candidate_geometry_rows: count_jsonl_rows(&geometry_path(&args.out_dir)),
        feature_rows: plan.feature_rows,
        pre_target_selection_rows: selection_rows.len(),
        evaluated_rows_for_current_selection: evaluated_current.len(),
        sys_evaluation_cache_rows_total: evaluated_all.len(),
        baseline_policy: plan.baseline_policy.clone(),
        baseline_replicates: plan.baseline_replicates,
        rule_set: plan.rule_set.clone(),
        selection_rules: plan.rules.clone(),
        selection_sets: plan.selection_sets,
        selected_rows_sum_over_sets: plan.selected_rows_sum_over_sets,
        unique_selected_rows: plan.unique_selected_rows,
        unique_baseline_rows: plan.unique_baseline_rows,
        unique_selected_or_baseline_rows: plan.unique_selected_or_baseline_rows,
        selection_feature: plan.selection_feature.clone(),
        selection_direction: plan.selection_direction.clone(),
        max_sys_evaluated_current_selection: evaluated_current
            .iter()
            .map(|row| row.sys)
            .reduce(f64::max)
            .unwrap_or(0.0),
        max_sys_selected,
        missing_sys_rows_for_current_selection,
        candidate_geometry_cache_blake3: blake3_file(&geometry_path(&args.out_dir)),
        candidate_feature_table_blake3: blake3_file(&feature_path(&args.out_dir)),
        selected_candidates_before_sys_blake3: blake3_file(&selection_path(&args.out_dir)),
        sys_evaluation_cache_blake3: blake3_file(&evaluation_path(&args.out_dir)),
        selection_summary_blake3: blake3_file(&summary_path),
        selection_plan_blake3: blake3_file(&selection_plan_path(&args.out_dir)),
        resolved_run_config_blake3: blake3_file(&resolved_run_config_path(&args.out_dir)),
        pipeline_summary_status: "pipeline-summary.json is a duplicate legacy alias of evaluation-report.json for older packet readers".to_string(),
    };
    write_json(&args.out_dir.join("evaluation-report.json"), &report);
    write_json(&args.out_dir.join("pipeline-summary.json"), &report);
    println!(
        "stage=report status=wrote evaluated_current={} wall_seconds={:.3}",
        evaluated_current.len(),
        started.elapsed().as_secs_f64()
    );
}

fn run_stage(args: &Args, stage: Stage) {
    match stage {
        Stage::All => unreachable!("all is expanded in main"),
        Stage::Geometry => run_geometry_stage(args),
        Stage::Features => run_feature_stage(args),
        Stage::Selection => run_selection_stage(args),
        Stage::Sys => run_sys_stage(args),
        Stage::Reports => run_report_stage(args),
    }
}

fn main() {
    let args = parse_args();
    std::fs::create_dir_all(&args.out_dir).expect("create output directory");
    write_json(
        &resolved_run_config_path(&args.out_dir),
        &resolved_run_config(&args),
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.jobs)
        .build_global()
        .expect("initialize rayon global thread pool");

    println!(
        "feature-first candidate-cache pipeline stage={:?} config_path={} seed={} candidates_per_bucket={} limit_total={:?} baseline_policy={} baseline_replicates={} rule_set={} selection_rules={} selection_feature={} selection_direction={} jobs={} chunk_rows={} out_dir={}",
        args.stage,
        args.config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "none".to_string()),
        args.seed,
        args.candidates_per_bucket,
        args.limit_total,
        args.baseline_policy,
        args.baseline_replicates,
        args.rule_set,
        args.selection_rules.len(),
        args.selection_feature.name(),
        args.selection_direction.as_str(),
        args.jobs,
        args.chunk_rows,
        args.out_dir.display()
    );

    match args.stage {
        Stage::All => {
            for stage in [
                Stage::Geometry,
                Stage::Features,
                Stage::Selection,
                Stage::Sys,
                Stage::Reports,
            ] {
                run_stage(&args, stage);
            }
        }
        stage => run_stage(&args, stage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axis_vertices_for_diagonal_covariance(diagonal: [f64; 4]) -> Vec<Vector4<f64>> {
        let mut vertices = Vec::new();
        for coordinate in 0..4 {
            let mut vertex = Vector4::zeros();
            vertex[coordinate] = (4.0 * diagonal[coordinate]).sqrt();
            vertices.push(vertex);
            vertices.push(-vertex);
        }
        vertices
    }

    fn assert_close(left: f64, right: f64, relative_tolerance: f64) {
        let scale = left.abs().max(right.abs()).max(1.0);
        assert!(
            (left - right).abs() <= relative_tolerance * scale,
            "left={left:.17e} right={right:.17e} tolerance={relative_tolerance:.3e}"
        );
    }

    fn feature_row(
        bucket: &str,
        candidate: &str,
        primary: f64,
        secondary: f64,
    ) -> CandidateFeatureRow {
        CandidateFeatureRow {
            candidate_id: candidate.to_string(),
            name: candidate.to_string(),
            poly_id: candidate.to_string(),
            bucket_id: bucket.to_string(),
            ridge_symp_area_sum_over_volume_sqrt: primary,
            ridge_symp_area_max_share: secondary,
            vertex_covariance_rho: Some(primary),
            vertex_covariance_status: "eligible".to_string(),
            ..CandidateFeatureRow::default()
        }
    }

    #[test]
    fn vertex_covariance_matches_analytic_diagonal_fixture() {
        let vertices = axis_vertices_for_diagonal_covariance([1.0, 4.0, 9.0, 16.0]);
        let diagnostics = vertex_covariance_diagnostics(&vertices, 8);
        assert_eq!(diagnostics.status, "eligible");
        assert_close(diagnostics.nu1.unwrap(), 3.0, 1.0e-13);
        assert_close(diagnostics.nu2.unwrap(), 8.0, 1.0e-13);
        assert_close(diagnostics.rho.unwrap(), 8.0 / 3.0, 1.0e-13);
        assert_close(diagnostics.condition.unwrap(), 16.0, 1.0e-13);
    }

    #[test]
    fn vertex_covariance_is_translation_scale_order_and_symplectic_invariant() {
        let vertices = axis_vertices_for_diagonal_covariance([1.0, 4.0, 9.0, 16.0]);
        let reference = vertex_covariance_diagnostics(&vertices, 8);
        let translation = Vector4::new(11.0, -7.0, 3.5, 2.25);
        let mut transformed = vertices
            .iter()
            .map(|vertex| 2.75 * vertex + translation)
            .collect::<Vec<_>>();
        transformed.reverse();
        let translated_scaled = vertex_covariance_diagnostics(&transformed, 8);
        assert_close(
            translated_scaled.rho.unwrap(),
            reference.rho.unwrap(),
            1.0e-12,
        );

        // q -> q + Bp with symmetric B is symplectic in (q1,q2,p1,p2).
        let shear = Matrix4::new(
            1.0, 0.0, 0.3, -0.2, 0.0, 1.0, -0.2, 0.4, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let sheared = vertices
            .iter()
            .map(|vertex| shear * vertex)
            .collect::<Vec<_>>();
        let symplectic = vertex_covariance_diagnostics(&sheared, 8);
        assert_close(symplectic.nu1.unwrap(), reference.nu1.unwrap(), 2.0e-12);
        assert_close(symplectic.nu2.unwrap(), reference.nu2.unwrap(), 2.0e-12);
        assert_close(symplectic.rho.unwrap(), reference.rho.unwrap(), 2.0e-12);

        // Swap the two canonical pairs simultaneously.
        let pair_swap = Matrix4::new(
            0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        );
        let permuted = vertices
            .iter()
            .map(|vertex| pair_swap * vertex)
            .collect::<Vec<_>>();
        let permutation = vertex_covariance_diagnostics(&permuted, 8);
        assert_close(permutation.rho.unwrap(), reference.rho.unwrap(), 1.0e-13);
    }

    #[test]
    fn vertex_covariance_agrees_with_independent_python_reference_fixture() {
        // Reference values were computed from numpy.linalg.eigvals(1j * J @ C)
        // for C=diag(2,3,5,7), independently of the invariant implementation.
        let vertices = axis_vertices_for_diagonal_covariance([2.0, 3.0, 5.0, 7.0]);
        let diagnostics = vertex_covariance_diagnostics(&vertices, 8);
        assert_close(diagnostics.nu1.unwrap(), 3.162_277_660_168_379_5, 1.0e-13);
        assert_close(diagnostics.nu2.unwrap(), 4.582_575_694_955_84, 1.0e-13);
        assert_close(diagnostics.rho.unwrap(), 1.449_137_674_618_944, 1.0e-13);
    }

    #[test]
    fn cached_product_vertices_match_reconstructed_product_geometry() {
        let geometry_row = build_geometry_row(90210, 4, 5, 3);
        let cached = vertex_covariance_diagnostics(
            &arrays_to_vectors(&geometry_row.vertices),
            geometry_row.product_k * geometry_row.product_m,
        );
        let reconstructed = generate_product_geometry(90210, 4, 5, 3).unwrap();
        let direct = vertex_covariance_diagnostics(&reconstructed.vertices, 4 * 5);
        assert_eq!(cached.status, "eligible");
        assert_eq!(direct.status, "eligible");
        assert_close(cached.nu1.unwrap(), direct.nu1.unwrap(), 1.0e-14);
        assert_close(cached.nu2.unwrap(), direct.nu2.unwrap(), 1.0e-14);
        assert_close(cached.rho.unwrap(), direct.rho.unwrap(), 1.0e-14);
    }

    #[test]
    fn frozen_multi_arm_selection_has_exact_counts_and_shared_disjoint_control() {
        let mut features = Vec::with_capacity(PRODUCT_PAIRS.len() * 5000);
        for (k, m) in PRODUCT_PAIRS {
            let bucket = bucket_id(*k, *m);
            for sample in 0..5000 {
                let mut row = feature_row(
                    &bucket,
                    &format!("seed7-{k}x{m}-{sample:04}"),
                    sample as f64,
                    sample as f64,
                );
                row.product_k = *k;
                row.product_m = *m;
                features.push(row);
            }
        }
        let design = FrozenCovarianceValidation {
            rho_fraction: 0.005,
            ridge_primary_fraction: 0.01,
            ridge_secondary_fraction: 0.5,
            control_count_per_bucket: 25,
            control_seed: 2026071299,
        };
        let sets = frozen_covariance_validation_selection_sets(&features, &design);
        assert_eq!(sets.len(), 3);
        assert_eq!(sets[0].indices.len(), 250);
        assert_eq!(sets[1].indices.len(), 250);
        assert_eq!(sets[2].indices.len(), 250);
        let rho = sets[0].indices.iter().copied().collect::<BTreeSet<_>>();
        let ridge = sets[1].indices.iter().copied().collect::<BTreeSet<_>>();
        let control = sets[2].indices.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(rho.intersection(&ridge).count(), 250);
        assert!(rho.is_disjoint(&control));
        assert!(ridge.is_disjoint(&control));
        let repeated = frozen_covariance_validation_selection_sets(&features, &design);
        assert_eq!(sets[2].indices, repeated[2].indices);
    }

    fn cascade(primary_fraction: f64, secondary_fraction: f64) -> PerBucketCascade {
        PerBucketCascade {
            primary_feature: ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
            primary_direction: SelectionDirection::Low,
            primary_fraction,
            secondary_feature: ScalarFeature::RidgeSympAreaMaxShare,
            secondary_direction: SelectionDirection::Low,
            secondary_fraction,
            emit_stage_1_comparator: true,
        }
    }

    fn selected_candidate_ids(
        features: &[CandidateFeatureRow],
        selection: &SelectionSet,
    ) -> Vec<String> {
        selection
            .indices
            .iter()
            .map(|index| features[*index].candidate_id.clone())
            .collect()
    }

    #[test]
    fn cascade_is_bucket_local_and_secondary_only_ranks_primary_tail() {
        let features = vec![
            feature_row("a", "a1", 1.0, 9.0),
            feature_row("a", "a2", 2.0, 8.0),
            feature_row("a", "a3", 3.0, 0.0),
            feature_row("a", "a4", 4.0, 1.0),
            feature_row("b", "b1", 1.0, 7.0),
            feature_row("b", "b2", 2.0, 6.0),
            feature_row("b", "b3", 3.0, 0.0),
            feature_row("b", "b4", 4.0, 1.0),
        ];
        let selections = per_bucket_cascade_selection_sets(&features, &cascade(0.5, 0.5));
        assert_eq!(selections.len(), 2);
        assert_eq!(
            selected_candidate_ids(&features, &selections[0]),
            vec!["a1", "a2", "b1", "b2"]
        );
        assert_eq!(
            selected_candidate_ids(&features, &selections[1]),
            vec!["a2", "b2"]
        );
    }

    #[test]
    fn cascade_uses_ceil_min_one_and_candidate_id_ties() {
        let mut features = Vec::new();
        for index in (0..100).rev() {
            features.push(feature_row("n100", &format!("n100-{index:03}"), 1.0, 1.0));
        }
        for index in (0..101).rev() {
            features.push(feature_row("n101", &format!("n101-{index:03}"), 1.0, 1.0));
        }
        let selections = per_bucket_cascade_selection_sets(&features, &cascade(0.01, 0.5));
        assert_eq!(selected_candidate_ids(&features, &selections[0]).len(), 3);
        assert_eq!(
            selected_candidate_ids(&features, &selections[1]),
            vec!["n100-000", "n101-000"]
        );
    }

    #[test]
    fn cascade_membership_is_invariant_under_input_reordering() {
        let forward = vec![
            feature_row("a", "a", 1.0, 2.0),
            feature_row("a", "b", 2.0, 1.0),
            feature_row("a", "c", 3.0, 0.0),
            feature_row("a", "d", 4.0, 3.0),
        ];
        let mut reversed = forward.clone();
        reversed.reverse();
        let forward_sets = per_bucket_cascade_selection_sets(&forward, &cascade(0.5, 0.5));
        let reversed_sets = per_bucket_cascade_selection_sets(&reversed, &cascade(0.5, 0.5));
        for set_index in 0..2 {
            assert_eq!(
                selected_candidate_ids(&forward, &forward_sets[set_index]),
                selected_candidate_ids(&reversed, &reversed_sets[set_index])
            );
        }
    }

    #[test]
    fn matched_baseline_is_exact_bucket_matched_disjoint_and_deterministic() {
        let features = (0..8)
            .map(|index| {
                let bucket = if index < 4 { "a" } else { "b" };
                feature_row(bucket, &format!("candidate-{index}"), index as f64, 0.0)
            })
            .collect::<Vec<_>>();
        let selection = SelectionSet {
            id: "test-selection".to_string(),
            kind: "test".to_string(),
            feature: ScalarFeature::RidgeSympAreaSumOverVolumeSqrt,
            direction: SelectionDirection::Low,
            requested_budget: "two-per-bucket".to_string(),
            indices: vec![0, 1, 4, 5],
            require_disjoint_baseline: true,
        };
        let first = baseline_indices_for_selection(&features, &selection, 0);
        let repeated = baseline_indices_for_selection(&features, &selection, 0);
        assert_eq!(first, repeated);
        assert_eq!(first.len(), selection.indices.len());
        assert!(first.iter().all(|index| !selection.indices.contains(index)));
        let counts = first.iter().fold(BTreeMap::new(), |mut counts, index| {
            *counts
                .entry(features[*index].bucket_id.clone())
                .or_insert(0usize) += 1;
            counts
        });
        assert_eq!(counts.get("a"), Some(&2));
        assert_eq!(counts.get("b"), Some(&2));
    }

    #[test]
    fn cascade_config_denies_unknown_fields_and_scalar_conflicts() {
        let unknown = r#"{
            "per_bucket_cascade": {
                "primary_feature": "ridge_symp_area_sum_over_volume_sqrt",
                "primary_direction": "low",
                "primary_fraction": 0.01,
                "secondary_feature": "ridge_symp_area_max_share",
                "secondary_direction": "low",
                "secondary_fraction": 0.5,
                "emit_stage_1_comparator": true,
                "unexpected": 1
            }
        }"#;
        assert!(serde_json::from_str::<SelectionConfigFile>(unknown).is_err());

        let conflict = r#"{
            "selection_feature": "ridge_symp_area_sum_over_volume_sqrt",
            "per_bucket_cascade": {
                "primary_feature": "ridge_symp_area_sum_over_volume_sqrt",
                "primary_direction": "low",
                "primary_fraction": 0.01,
                "secondary_feature": "ridge_symp_area_max_share",
                "secondary_direction": "low",
                "secondary_fraction": 0.5,
                "emit_stage_1_comparator": true
            }
        }"#;
        let config = serde_json::from_str::<SelectionConfigFile>(conflict).unwrap();
        assert!(std::panic::catch_unwind(|| validate_cascade_conflicts(&config)).is_err());
    }

    #[test]
    fn cascade_metadata_records_scope_rounding_steps_and_comparator() {
        let report = cascade(0.01, 0.5).report();
        assert_eq!(report.scope, "actual_bucket_id");
        assert_eq!(report.rounding, "ceil_min_one");
        assert_eq!(report.primary_fraction, 0.01);
        assert_eq!(report.secondary_fraction, 0.5);
        assert!(report.emit_stage_1_comparator);
        assert_eq!(report.secondary_feature, "ridge_symp_area_max_share");
    }
}
