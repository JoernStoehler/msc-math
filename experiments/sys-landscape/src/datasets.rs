//! Sys-landscape data-layout helpers.
//!
//! The project currently has overlapping legacy packets and newer flat entry
//! points. This module centralizes the repo paths used by:
//! - raw cache-worthy corpus producers;
//! - datascience-ready table producers;
//! - legacy packet locations that still back some of the flat binaries.

use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DatasetFamily {
    SharedCache,
    RandomGeneric,
    RandomProduct,
    RejectionCalibration,
    AscentGeneral,
    AscentProduct,
    Continuation,
    RotatedRegular,
    Normalized,
    FeatureSkeleton,
    FeatureFaceGeometry,
    FeatureFaceSymplectic,
    FeatureOmega,
    FeatureOrbit,
    FeatureTrajectory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatasetRole {
    CoreSearch,
    Calibration,
    StructuredFamily,
    DerivedJoin,
    FeatureBlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HkoProximity {
    None,
    FamilyAdjacent,
    LocalNeighborhood,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatasetSpec {
    pub stable_id: &'static str,
    pub producer_dir: &'static str,
    pub role: DatasetRole,
    pub hko_proximity: HkoProximity,
    pub spoiler_risk: bool,
}

impl DatasetFamily {
    pub fn spec(self) -> DatasetSpec {
        match self {
            DatasetFamily::SharedCache => DatasetSpec {
                stable_id: "shared-cache",
                producer_dir: ".",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::RandomGeneric => DatasetSpec {
                stable_id: "random-generic",
                producer_dir: "random-sample",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::RandomProduct => DatasetSpec {
                stable_id: "random-product",
                producer_dir: "random-product-sample",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::RejectionCalibration => DatasetSpec {
                stable_id: "rejection-calibration",
                producer_dir: "rejection-calibration",
                role: DatasetRole::Calibration,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::AscentGeneral => DatasetSpec {
                stable_id: "ascent-general",
                producer_dir: "gradient-ascent-general",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::AscentProduct => DatasetSpec {
                stable_id: "ascent-product",
                producer_dir: "gradient-ascent-products",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::Continuation => DatasetSpec {
                stable_id: "continuation-variable-f",
                producer_dir: "variable-f-ascent",
                role: DatasetRole::CoreSearch,
                hko_proximity: HkoProximity::FamilyAdjacent,
                spoiler_risk: false,
            },
            DatasetFamily::RotatedRegular => DatasetSpec {
                stable_id: "rotated-regular",
                producer_dir: "rotated-regular-products",
                role: DatasetRole::StructuredFamily,
                hko_proximity: HkoProximity::FamilyAdjacent,
                spoiler_risk: true,
            },
            DatasetFamily::Normalized => DatasetSpec {
                stable_id: "normalized",
                producer_dir: "normalized-dataset",
                role: DatasetRole::DerivedJoin,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureSkeleton => DatasetSpec {
                stable_id: "feature-skeleton",
                producer_dir: "feature-skeleton",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureFaceGeometry => DatasetSpec {
                stable_id: "feature-face-geometry",
                producer_dir: "feature-face-geometry",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureFaceSymplectic => DatasetSpec {
                stable_id: "feature-face-symplectic",
                producer_dir: "feature-face-symplectic",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureOmega => DatasetSpec {
                stable_id: "feature-omega",
                producer_dir: "feature-omega",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureOrbit => DatasetSpec {
                stable_id: "feature-orbit",
                producer_dir: "feature-orbit",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::FamilyAdjacent,
                spoiler_risk: false,
            },
            DatasetFamily::FeatureTrajectory => DatasetSpec {
                stable_id: "feature-trajectory",
                producer_dir: "feature-trajectory",
                role: DatasetRole::FeatureBlock,
                hko_proximity: HkoProximity::None,
                spoiler_risk: false,
            },
        }
    }
}

pub fn package_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn family_dir(family: DatasetFamily) -> PathBuf {
    let spec = family.spec();
    if spec.producer_dir == "." {
        package_root()
    } else {
        package_root().join(spec.producer_dir)
    }
}

pub fn dataset_path(family: DatasetFamily, file_name: &str) -> PathBuf {
    family_dir(family).join(file_name)
}

pub fn shared_family_cache_path() -> PathBuf {
    dataset_path(DatasetFamily::SharedCache, "cache.jsonl")
}

pub fn continuation_cache_path() -> PathBuf {
    dataset_path(DatasetFamily::Continuation, "cache.jsonl")
}

pub fn datasets_root() -> PathBuf {
    package_root().join("datasets")
}

pub fn raw_root() -> PathBuf {
    package_root().join("raw")
}

pub fn raw_dataset_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}.jsonl"))
}

pub fn raw_dataset_trace_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}-trace.jsonl"))
}

pub fn raw_dataset_cache_path(stem: &str) -> PathBuf {
    raw_root().join(format!("{stem}-cache.jsonl"))
}

pub fn canonical_dataset_path(stem: &str) -> PathBuf {
    datasets_root().join(format!("{stem}.jsonl"))
}

pub fn canonical_dataset_trace_path(stem: &str) -> PathBuf {
    datasets_root().join(format!("{stem}-trace.jsonl"))
}

pub fn canonical_dataset_cache_path(stem: &str) -> PathBuf {
    datasets_root().join(format!("{stem}-cache.jsonl"))
}

pub fn legacy_experiment_path(experiment_dir: &str, file_name: &str) -> PathBuf {
    package_root().join(experiment_dir).join(file_name)
}
