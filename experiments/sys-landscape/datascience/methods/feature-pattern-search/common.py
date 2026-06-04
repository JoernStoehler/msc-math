#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""Shared local helpers for the sys-landscape feature-pattern-search packet."""

import json
import sys
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent.parent.parent))
from figure_config import FIGSIZE_DUAL, FIGSIZE_SQUARE, setup

EXPERIMENT_DIR = Path(__file__).resolve().parent
REPO_ROOT = EXPERIMENT_DIR.parent.parent.parent.parent.parent
DEFAULT_DATASET_DIR = REPO_ROOT / "experiments/sys-landscape/datascience/dataset"

FEATURE_GEOMETRY_JSONL = EXPERIMENT_DIR / "feature_geometry.jsonl"
FEATURE_FACE_GEOMETRY_JSONL = EXPERIMENT_DIR / "feature_face_geometry.jsonl"
FEATURE_FACE_SYMPLECTIC_JSONL = EXPERIMENT_DIR / "feature_face_symplectic.jsonl"
FEATURE_SKELETON_JSONL = EXPERIMENT_DIR / "feature_skeleton.jsonl"
FEATURE_OMEGA_JSONL = EXPERIMENT_DIR / "feature_omega.jsonl"
FEATURE_ORBIT_JSONL = EXPERIMENT_DIR / "feature_orbit.jsonl"
FEATURE_TRAJECTORY_JSONL = EXPERIMENT_DIR / "feature_trajectory.jsonl"

ENDPOINT_DATASETS = {
    "gradient_ascent_general",
    "gradient_ascent_products",
    "variable_f_ascent",
}
RANDOM_DATASETS = {
    "random_sample",
    "random_product_sample",
}


def repo_path(path: Path) -> str:
    resolved = path.resolve()
    try:
        return str(resolved.relative_to(REPO_ROOT))
    except ValueError:
        return str(resolved)


@dataclass
class JoinedRow:
    state_id: str
    poly_id: str
    regime: str
    group_id: str
    sys: float
    metadata: dict[str, str | float]
    geometry: dict[str, float]
    face_geometry: dict[str, float]
    face_symplectic: dict[str, float]
    skeleton: dict[str, float]
    omega: dict[str, float]
    orbit: dict[str, float]
    trajectory: dict[str, float]


@dataclass
class FeatureMaps:
    geometry_by_poly: dict[str, dict[str, float]]
    face_geometry_by_poly: dict[str, dict[str, float]]
    face_symplectic_by_poly: dict[str, dict[str, float]]
    skeleton_by_poly: dict[str, dict[str, float]]
    omega_by_poly: dict[str, dict[str, float]]
    orbit_by_poly: dict[str, dict[str, float]]
    trajectory_by_observation: dict[str, dict[str, float]]


def load_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def write_jsonl(path: Path, rows: list[dict]) -> None:
    with path.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row) + "\n")


def cv_group_id(state: dict, regime: str) -> str:
    if state.get("root_group_id"):
        return str(state["root_group_id"])
    if regime == "endpoint" and state.get("source_name"):
        return str(state["source_name"])
    return str(state.get("lineage_id") or state["observation_id"])


def load_dataset_tables(dataset_dir: Path) -> tuple[list[dict], dict[str, dict]]:
    observations = load_jsonl(dataset_dir / "observation-table.jsonl")
    polytopes = {
        row["poly_id"]: row for row in load_jsonl(dataset_dir / "polytope-table.jsonl")
    }
    return observations, polytopes


def load_feature_maps() -> FeatureMaps:
    geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_GEOMETRY_JSONL)
    }
    face_geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_FACE_GEOMETRY_JSONL)
    }
    face_symplectic_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_FACE_SYMPLECTIC_JSONL)
    }
    skeleton_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_SKELETON_JSONL)
    }
    omega_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_OMEGA_JSONL)
    }
    orbit_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(FEATURE_ORBIT_JSONL)
    }
    trajectory_by_observation = {
        row["observation_id"]: {
            key: value for key, value in row.items() if key != "observation_id"
        }
        for row in load_jsonl(FEATURE_TRAJECTORY_JSONL)
    }
    return FeatureMaps(
        geometry_by_poly=geometry_by_poly,
        face_geometry_by_poly=face_geometry_by_poly,
        face_symplectic_by_poly=face_symplectic_by_poly,
        skeleton_by_poly=skeleton_by_poly,
        omega_by_poly=omega_by_poly,
        orbit_by_poly=orbit_by_poly,
        trajectory_by_observation=trajectory_by_observation,
    )


def load_joined_rows(dataset_dir: Path, *, endpoint_only: bool = False) -> list[JoinedRow]:
    observations, polytopes = load_dataset_tables(dataset_dir)
    feature_maps = load_feature_maps()

    rows = []
    for observation in observations:
        dataset = observation["dataset"]
        regime = "endpoint" if dataset in ENDPOINT_DATASETS else "random"
        if dataset not in ENDPOINT_DATASETS | RANDOM_DATASETS:
            raise ValueError(f"unexpected dataset {dataset}")
        if endpoint_only and regime != "endpoint":
            continue
        poly = polytopes[observation["poly_id"]]
        rows.append(
            JoinedRow(
                state_id=observation["observation_id"],
                poly_id=observation["poly_id"],
                regime=regime,
                group_id=cv_group_id(observation, regime),
                sys=poly["sys"],
                metadata={
                    "facet_count": float(poly["facet_count"]),
                    "family": observation["family"],
                    "dataset": dataset,
                    "role": observation["role"],
                    "search_space": observation["search_space"],
                    "optimizer": observation["optimizer"],
                    "backend": observation["backend"],
                },
                geometry=feature_maps.geometry_by_poly[observation["poly_id"]],
                face_geometry=feature_maps.face_geometry_by_poly[observation["poly_id"]],
                face_symplectic=feature_maps.face_symplectic_by_poly[
                    observation["poly_id"]
                ],
                skeleton=feature_maps.skeleton_by_poly[observation["poly_id"]],
                omega=feature_maps.omega_by_poly[observation["poly_id"]],
                orbit=feature_maps.orbit_by_poly[observation["poly_id"]],
                trajectory=feature_maps.trajectory_by_observation[
                    observation["observation_id"]
                ],
            )
        )
    return rows
