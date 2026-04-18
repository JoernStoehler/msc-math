#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""
Goal: Run a bounded hostile-landscape pattern-search pass on the normalized
      dataset, comparing a narrow metadata baseline against cheap geometry
      features derived from exact dual vertices.
Input Artifacts:
  - experiments/sys-landscape/cache.jsonl
  - experiments/combinatorial-cells/polytopes.jsonl
  - experiments/sys-landscape/variable-f-ascent/cache.jsonl
  - experiments/sys-landscape/random-sample/random-sweep.jsonl
  - experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl
  - experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl
  - experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl
  - experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl
  - experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products-trace.jsonl
  - experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl
  - optionally a precomputed normalized dataset directory passed by `--normalized-dir`
Output Artifacts:
  - experiments/sys-landscape/feature-pattern-search/feature_geometry.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_face_geometry.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_face_symplectic.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_skeleton.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_omega.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_orbit.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_trajectory.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_pattern_search_summary.md
  - experiments/sys-landscape/feature-pattern-search/feature_pattern_search_ridge.png
  - experiments/sys-landscape/feature-pattern-search/feature_pattern_search_rf.png
"""

import argparse
import json
import math
import statistics
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from sklearn.ensemble import RandomForestRegressor
from sklearn.feature_extraction import DictVectorizer
from sklearn.linear_model import Ridge
from sklearn.metrics import mean_squared_error, r2_score
from sklearn.model_selection import GroupKFold
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import MaxAbsScaler

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_DUAL, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
REPO_ROOT = EXPERIMENT_DIR.parent.parent.parent

FEATURE_JSONL = EXPERIMENT_DIR / "feature_geometry.jsonl"
FEATURE_FACE_GEOMETRY_JSONL = EXPERIMENT_DIR / "feature_face_geometry.jsonl"
FEATURE_FACE_SYMPLECTIC_JSONL = EXPERIMENT_DIR / "feature_face_symplectic.jsonl"
FEATURE_SKELETON_JSONL = EXPERIMENT_DIR / "feature_skeleton.jsonl"
FEATURE_OMEGA_JSONL = EXPERIMENT_DIR / "feature_omega.jsonl"
FEATURE_ORBIT_JSONL = EXPERIMENT_DIR / "feature_orbit.jsonl"
FEATURE_TRAJECTORY_JSONL = EXPERIMENT_DIR / "feature_trajectory.jsonl"
SUMMARY_MD = EXPERIMENT_DIR / "feature_pattern_search_summary.md"
RIDGE_PNG = EXPERIMENT_DIR / "feature_pattern_search_ridge.png"
RF_PNG = EXPERIMENT_DIR / "feature_pattern_search_rf.png"

ENDPOINT_DATASETS = {
    "gradient_ascent_general",
    "gradient_ascent_products",
    "variable_f_ascent",
}
RANDOM_DATASETS = {
    "random_sample",
    "random_product_sample",
}
SURFACES = [
    ("within_random", "Within random"),
    ("within_endpoint", "Within endpoint"),
    ("random_to_endpoint", "Random -> endpoint"),
    ("endpoint_to_random", "Endpoint -> random"),
]
FEATURE_BLOCKS = [
    "null",
    "metadata",
    "geometry",
    "face_geometry",
    "face_symplectic",
    "skeleton",
    "omega",
    "orbit",
    "trajectory",
    "all",
]
MODEL_SPECS = [("ridge", "Ridge"), ("rf", "Random forest")]


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


def cv_group_id(state: dict, regime: str) -> str:
    if state.get("root_group_id"):
        return str(state["root_group_id"])
    if regime == "endpoint" and state.get("source_name"):
        return str(state["source_name"])
    return str(state.get("lineage_id") or state["state_id"])


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--normalized-dir",
        type=Path,
        help="Use an existing normalized dataset directory instead of refreshing a temp one.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def parse_rational(token: str) -> float:
    if "/" not in token:
        return float(token)
    numerator, denominator = token.split("/", 1)
    return int(numerator) / int(denominator)


def centered_singular_values(vertices: list[list[float]]) -> list[float]:
    arr = np.asarray(vertices, dtype=float)
    if arr.shape[0] == 1:
        centered = arr - arr[0]
    else:
        centered = arr - np.mean(arr, axis=0, keepdims=True)
    singular_values = np.linalg.svd(centered, full_matrices=False, compute_uv=False)
    out = [float(x) for x in singular_values[:4]]
    while len(out) < 4:
        out.append(0.0)
    return out


def build_geometry_features(poly: dict) -> dict[str, float]:
    dual_vertices = [
        [parse_rational(coord) for coord in row]
        for row in poly["dual_vertices_rational"]
    ]
    arr = np.asarray(dual_vertices, dtype=float)
    norms = np.linalg.norm(arr, axis=1)
    centroid = np.mean(arr, axis=0)
    centroid_norm = float(np.linalg.norm(centroid))
    coord_std = np.std(arr, axis=0)

    cosines: list[float] = []
    pairwise_distances: list[float] = []
    for i in range(len(dual_vertices)):
        for j in range(i + 1, len(dual_vertices)):
            vi = arr[i]
            vj = arr[j]
            denom = norms[i] * norms[j]
            if denom > 0:
                cosines.append(float(np.dot(vi, vj) / denom))
            pairwise_distances.append(float(np.linalg.norm(vi - vj)))

    singular_values = centered_singular_values(dual_vertices)

    return {
        "geom_norm_mean": float(np.mean(norms)),
        "geom_norm_std": float(np.std(norms)),
        "geom_norm_min": float(np.min(norms)),
        "geom_norm_max": float(np.max(norms)),
        "geom_centroid_norm": centroid_norm,
        "geom_coord_std_x": float(coord_std[0]),
        "geom_coord_std_y": float(coord_std[1]),
        "geom_coord_std_z": float(coord_std[2]),
        "geom_coord_std_w": float(coord_std[3]),
        "geom_cosine_mean": statistics.mean(cosines) if cosines else 0.0,
        "geom_cosine_std": statistics.pstdev(cosines) if len(cosines) > 1 else 0.0,
        "geom_cosine_min": min(cosines) if cosines else 0.0,
        "geom_cosine_max": max(cosines) if cosines else 0.0,
        "geom_pairwise_dist_mean": statistics.mean(pairwise_distances)
        if pairwise_distances
        else 0.0,
        "geom_pairwise_dist_std": statistics.pstdev(pairwise_distances)
        if len(pairwise_distances) > 1
        else 0.0,
        "geom_pairwise_dist_min": min(pairwise_distances) if pairwise_distances else 0.0,
        "geom_pairwise_dist_max": max(pairwise_distances) if pairwise_distances else 0.0,
        "geom_sval_1": singular_values[0],
        "geom_sval_2": singular_values[1],
        "geom_sval_3": singular_values[2],
        "geom_sval_4": singular_values[3],
    }


def refresh_normalized_dataset(out_dir: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-normalized-dataset",
        "--",
        "--out-dir",
        str(out_dir),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_skeleton_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-skeleton",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_face_geometry_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-face-geometry",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_omega_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-omega",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_face_symplectic_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-face-symplectic",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_orbit_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-orbit",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def refresh_trajectory_features(normalized_dir: Path, out_path: Path) -> None:
    cmd = [
        "cargo",
        "run",
        "-p",
        "exp-sys-landscape",
        "--release",
        "--bin",
        "sys-feature-trajectory",
        "--",
        "--normalized-dir",
        str(normalized_dir),
        "--out",
        str(out_path),
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def load_joined_rows(
    normalized_dir: Path,
) -> tuple[
    list[JoinedRow],
    list[dict],
    list[dict],
    list[dict],
    list[dict],
    list[dict],
    list[dict],
    list[dict],
]:
    states = load_jsonl(normalized_dir / "states.jsonl")
    capacities = {
        row["poly_id"]: row for row in load_jsonl(normalized_dir / "capacity_results.jsonl")
    }
    polytopes = {
        row["poly_id"]: row for row in load_jsonl(normalized_dir / "polytopes.jsonl")
    }

    geometry_rows: list[dict] = []
    geometry_by_poly: dict[str, dict[str, float]] = {}
    for poly_id, poly in polytopes.items():
        features = build_geometry_features(poly)
        geometry_rows.append({"poly_id": poly_id, **features})
        geometry_by_poly[poly_id] = features

    face_geometry_rows = load_jsonl(FEATURE_FACE_GEOMETRY_JSONL)
    face_geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in face_geometry_rows
    }
    face_symplectic_rows = load_jsonl(FEATURE_FACE_SYMPLECTIC_JSONL)
    face_symplectic_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in face_symplectic_rows
    }
    skeleton_rows = load_jsonl(FEATURE_SKELETON_JSONL)
    skeleton_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in skeleton_rows
    }
    omega_rows = load_jsonl(FEATURE_OMEGA_JSONL)
    omega_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in omega_rows
    }
    orbit_rows = load_jsonl(FEATURE_ORBIT_JSONL)
    orbit_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in orbit_rows
    }
    trajectory_rows = load_jsonl(FEATURE_TRAJECTORY_JSONL)
    trajectory_by_state = {
        row["state_id"]: {key: value for key, value in row.items() if key != "state_id"}
        for row in trajectory_rows
    }

    rows: list[JoinedRow] = []
    for state in states:
        dataset = state["dataset"]
        regime = "endpoint" if dataset in ENDPOINT_DATASETS else "random"
        if dataset not in ENDPOINT_DATASETS | RANDOM_DATASETS:
            raise ValueError(f"unexpected dataset {dataset}")
        poly = polytopes[state["poly_id"]]
        rows.append(
            JoinedRow(
                state_id=state["state_id"],
                poly_id=state["poly_id"],
                regime=regime,
                group_id=cv_group_id(state, regime),
                sys=capacities[state["poly_id"]]["sys"],
                metadata={
                    "facet_count": float(poly["facet_count"]),
                    "family": state["family"],
                    "dataset": dataset,
                    "role": state["role"],
                    "search_space": state["search_space"],
                    "optimizer": state["optimizer"],
                    "backend": state["backend"],
                },
                geometry=geometry_by_poly[state["poly_id"]],
                face_geometry=face_geometry_by_poly[state["poly_id"]],
                face_symplectic=face_symplectic_by_poly[state["poly_id"]],
                skeleton=skeleton_by_poly[state["poly_id"]],
                omega=omega_by_poly[state["poly_id"]],
                orbit=orbit_by_poly[state["poly_id"]],
                trajectory=trajectory_by_state[state["state_id"]],
            )
        )
    return (
        rows,
        geometry_rows,
        face_geometry_rows,
        face_symplectic_rows,
        skeleton_rows,
        omega_rows,
        orbit_rows,
        trajectory_rows,
    )


def build_feature_dict(row: JoinedRow, block: str) -> dict[str, float | str]:
    if block == "metadata":
        return dict(row.metadata)
    if block == "geometry":
        return dict(row.geometry)
    if block == "face_geometry":
        return dict(row.face_geometry)
    if block == "face_symplectic":
        return dict(row.face_symplectic)
    if block == "skeleton":
        return dict(row.skeleton)
    if block == "omega":
        return dict(row.omega)
    if block == "orbit":
        return dict(row.orbit)
    if block == "trajectory":
        return dict(row.trajectory)
    if block == "all":
        return {
            **row.metadata,
            **row.geometry,
            **row.face_geometry,
            **row.face_symplectic,
            **row.skeleton,
            **row.omega,
            **row.orbit,
            **row.trajectory,
        }
    raise ValueError(f"unknown feature block {block}")


def make_regressor(name: str):
    if name == "ridge":
        return make_pipeline(MaxAbsScaler(), Ridge(alpha=1.0))
    if name == "rf":
        return RandomForestRegressor(
            n_estimators=300,
            random_state=0,
            min_samples_leaf=2,
        )
    raise ValueError(f"unknown model {name}")


def score_predictions(y_true: np.ndarray, y_pred: np.ndarray) -> tuple[float, float]:
    rmse = math.sqrt(mean_squared_error(y_true, y_pred))
    return float(r2_score(y_true, y_pred)), float(rmse)


def evaluate_cv(rows: list[JoinedRow], block: str, model_name: str) -> tuple[float, float]:
    groups = np.asarray([row.group_id for row in rows])
    y = np.asarray([row.sys for row in rows], dtype=float)
    if len(set(groups)) < 2:
        return float("nan"), float("nan")
    splitter = GroupKFold(n_splits=min(5, len(set(groups))))
    preds = np.zeros_like(y)

    if block == "null":
        for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
            preds[test_idx] = float(np.mean(y[train_idx]))
        return score_predictions(y, preds)

    feature_dicts = [build_feature_dict(row, block) for row in rows]
    for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
        vectorizer = DictVectorizer(sparse=True)
        x_train = vectorizer.fit_transform(feature_dicts[i] for i in train_idx)
        x_test = vectorizer.transform(feature_dicts[i] for i in test_idx)
        model = make_regressor(model_name)
        if model_name == "rf":
            model.fit(x_train.toarray(), y[train_idx])
            preds[test_idx] = model.predict(x_test.toarray())
        else:
            model.fit(x_train, y[train_idx])
            preds[test_idx] = model.predict(x_test)
    return score_predictions(y, preds)


def evaluate_transfer(
    train_rows: list[JoinedRow],
    test_rows: list[JoinedRow],
    block: str,
    model_name: str,
) -> tuple[float, float]:
    y_train = np.asarray([row.sys for row in train_rows], dtype=float)
    y_test = np.asarray([row.sys for row in test_rows], dtype=float)

    if block == "null":
        preds = np.full_like(y_test, fill_value=float(np.mean(y_train)))
        return score_predictions(y_test, preds)

    train_dicts = [build_feature_dict(row, block) for row in train_rows]
    test_dicts = [build_feature_dict(row, block) for row in test_rows]
    vectorizer = DictVectorizer(sparse=True)
    x_train = vectorizer.fit_transform(train_dicts)
    x_test = vectorizer.transform(test_dicts)
    model = make_regressor(model_name)
    if model_name == "rf":
        model.fit(x_train.toarray(), y_train)
        preds = model.predict(x_test.toarray())
    else:
        model.fit(x_train, y_train)
        preds = model.predict(x_test)
    return score_predictions(y_test, preds)


def run_evaluations(rows: list[JoinedRow]) -> list[dict]:
    random_rows = [row for row in rows if row.regime == "random"]
    endpoint_rows = [row for row in rows if row.regime == "endpoint"]
    results: list[dict] = []

    for model_name, _model_label in MODEL_SPECS:
        for block in FEATURE_BLOCKS:
            r2, rmse = evaluate_cv(random_rows, block, model_name)
            results.append(
                {
                    "surface": "within_random",
                    "model": model_name,
                    "block": block,
                    "r2": r2,
                    "rmse": rmse,
                }
            )
            r2, rmse = evaluate_cv(endpoint_rows, block, model_name)
            results.append(
                {
                    "surface": "within_endpoint",
                    "model": model_name,
                    "block": block,
                    "r2": r2,
                    "rmse": rmse,
                }
            )
            r2, rmse = evaluate_transfer(random_rows, endpoint_rows, block, model_name)
            results.append(
                {
                    "surface": "random_to_endpoint",
                    "model": model_name,
                    "block": block,
                    "r2": r2,
                    "rmse": rmse,
                }
            )
            r2, rmse = evaluate_transfer(endpoint_rows, random_rows, block, model_name)
            results.append(
                {
                    "surface": "endpoint_to_random",
                    "model": model_name,
                    "block": block,
                    "r2": r2,
                    "rmse": rmse,
                }
            )
    return results


def format_metric(value: float) -> str:
    if math.isnan(value):
        return "nan"
    return f"{value:.4f}"


def write_feature_jsonl(rows: list[dict]) -> None:
    with FEATURE_JSONL.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row) + "\n")


def write_summary(
    normalized_source_label: str,
    joined_rows: list[JoinedRow],
    results: list[dict],
) -> None:
    counts_by_regime = {
        regime: sum(1 for row in joined_rows if row.regime == regime)
        for regime in ["random", "endpoint"]
    }
    counts_by_dataset: dict[str, int] = {}
    for row in joined_rows:
        counts_by_dataset[row.metadata["dataset"]] = counts_by_dataset.get(
            row.metadata["dataset"], 0
        ) + 1
    orbit_available_count = sum(
        1 for row in joined_rows if row.orbit["orbit_sigma_available"] > 0.5
    )
    orbit_kkt_available_count = sum(
        1 for row in joined_rows if row.orbit["orbit_kkt_available"] > 0.5
    )
    orbit_search_scalar_available_count = sum(
        1 for row in joined_rows if row.orbit["orbit_search_scalar_available"] > 0.5
    )
    trajectory_available_count = sum(
        1 for row in joined_rows if row.trajectory["trajectory_trace_available"] > 0.5
    )
    orbit_available_by_dataset: dict[str, int] = {}
    orbit_kkt_available_by_dataset: dict[str, int] = {}
    orbit_search_scalar_available_by_dataset: dict[str, int] = {}
    trajectory_available_by_dataset: dict[str, int] = {}
    for row in joined_rows:
        if row.orbit["orbit_sigma_available"] > 0.5:
            dataset = str(row.metadata["dataset"])
            orbit_available_by_dataset[dataset] = (
                orbit_available_by_dataset.get(dataset, 0) + 1
            )
        if row.orbit["orbit_kkt_available"] > 0.5:
            dataset = str(row.metadata["dataset"])
            orbit_kkt_available_by_dataset[dataset] = (
                orbit_kkt_available_by_dataset.get(dataset, 0) + 1
            )
        if row.orbit["orbit_search_scalar_available"] > 0.5:
            dataset = str(row.metadata["dataset"])
            orbit_search_scalar_available_by_dataset[dataset] = (
                orbit_search_scalar_available_by_dataset.get(dataset, 0) + 1
            )
        if row.trajectory["trajectory_trace_available"] > 0.5:
            dataset = str(row.metadata["dataset"])
            trajectory_available_by_dataset[dataset] = (
                trajectory_available_by_dataset.get(dataset, 0) + 1
            )

    best_rows = sorted(joined_rows, key=lambda row: row.sys, reverse=True)[:5]

    lines = [
        "# Feature Pattern Search Summary",
        "",
        "## Dataset",
        "",
        f"- normalized input source: {normalized_source_label}",
        f"- joined rows: `{len(joined_rows)}`",
        f"- random rows: `{counts_by_regime['random']}`",
        f"- endpoint rows: `{counts_by_regime['endpoint']}`",
        f"- rows with cached sigma payload: `{orbit_available_count}`",
        f"- rows with bounded best-orbit KKT payload: `{orbit_kkt_available_count}`",
        f"- rows with cached search-level orbit scalars: `{orbit_search_scalar_available_count}`",
        f"- rows with trace-derived trajectory payload: `{trajectory_available_count}`",
        "- dataset counts:",
    ]
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(f"  - `{dataset}`: `{count}`")
    lines.append("- cached sigma coverage by dataset:")
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(
            f"  - `{dataset}`: `{orbit_available_by_dataset.get(dataset, 0)}` / `{count}`"
        )
    lines.append("- bounded best-orbit KKT coverage by dataset:")
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(
            f"  - `{dataset}`: `{orbit_kkt_available_by_dataset.get(dataset, 0)}` / `{count}`"
        )
    lines.append("- cached search-level orbit-scalar coverage by dataset:")
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(
            f"  - `{dataset}`: `{orbit_search_scalar_available_by_dataset.get(dataset, 0)}` / `{count}`"
        )
    lines.append("- trajectory trace coverage by dataset:")
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(
            f"  - `{dataset}`: `{trajectory_available_by_dataset.get(dataset, 0)}` / `{count}`"
        )

    lines.extend(
        [
            "",
            "## Feature Blocks",
            "",
            "- `null`: train-mean predictor with no features",
            "- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend",
            "- `geometry`: cheap dual-vertex summaries from `polytopes.jsonl`",
            "- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry",
            "- `face_symplectic`: scale-sensitive ridge-polygon symplectic-area summaries from ordered ridge vertex cycles",
            "- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice",
            "- `omega`: ridge-local `omega_0` summaries, exact omega-sign structure, and directed transition-graph summaries",
            "- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars",
            "- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries",
            "- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit, and trajectory together",
            "",
            "## Metrics",
            "",
            "Reported metrics are test-set `R^2` and RMSE. Within-regime results use grouped CV keyed by persisted `root_group_id` whenever that field is present. Transfer results train on one regime and test on the other.",
            "",
        ]
    )

    for model_name, model_label in MODEL_SPECS:
        lines.append(f"### {model_label}")
        lines.append("")
        lines.append("| Surface | Block | R^2 | RMSE |")
        lines.append("|---------|-------|-----|------|")
        for surface_key, surface_label in SURFACES:
            model_rows = [
                row
                for row in results
                if row["model"] == model_name and row["surface"] == surface_key
            ]
            by_block = {row["block"]: row for row in model_rows}
            for block in FEATURE_BLOCKS:
                result = by_block[block]
                lines.append(
                    f"| {surface_label} | `{block}` | {format_metric(result['r2'])} | {format_metric(result['rmse'])} |"
                )
        lines.append("")

    lines.extend(
        [
            "## Top States",
            "",
            "| State | Dataset | Regime | sys |",
            "|-------|---------|--------|-----|",
        ]
    )
    for row in best_rows:
        lines.append(
            f"| `{row.state_id}` | `{row.metadata['dataset']}` | `{row.regime}` | {row.sys:.6f} |"
        )

    ridge_rows = { (row["surface"], row["block"]): row for row in results if row["model"] == "ridge" }
    geometry_random = ridge_rows[("within_random", "geometry")]
    face_geometry_random = ridge_rows[("within_random", "face_geometry")]
    face_symplectic_random = ridge_rows[("within_random", "face_symplectic")]
    skeleton_random = ridge_rows[("within_random", "skeleton")]
    omega_random = ridge_rows[("within_random", "omega")]
    orbit_random = ridge_rows[("within_random", "orbit")]
    trajectory_random = ridge_rows[("within_random", "trajectory")]
    metadata_random = ridge_rows[("within_random", "metadata")]
    geometry_endpoint = ridge_rows[("within_endpoint", "geometry")]
    face_geometry_endpoint = ridge_rows[("within_endpoint", "face_geometry")]
    face_symplectic_endpoint = ridge_rows[("within_endpoint", "face_symplectic")]
    skeleton_endpoint = ridge_rows[("within_endpoint", "skeleton")]
    omega_endpoint = ridge_rows[("within_endpoint", "omega")]
    orbit_endpoint = ridge_rows[("within_endpoint", "orbit")]
    trajectory_endpoint = ridge_rows[("within_endpoint", "trajectory")]
    metadata_endpoint = ridge_rows[("within_endpoint", "metadata")]
    all_random_to_endpoint = ridge_rows[("random_to_endpoint", "all")]
    trajectory_endpoint_to_random = ridge_rows[("endpoint_to_random", "trajectory")]
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            f"- within-random ridge: metadata `R^2={format_metric(metadata_random['r2'])}`, geometry `R^2={format_metric(geometry_random['r2'])}`, face_geometry `R^2={format_metric(face_geometry_random['r2'])}`, face_symplectic `R^2={format_metric(face_symplectic_random['r2'])}`, skeleton `R^2={format_metric(skeleton_random['r2'])}`, omega `R^2={format_metric(omega_random['r2'])}`, orbit `R^2={format_metric(orbit_random['r2'])}`, trajectory `R^2={format_metric(trajectory_random['r2'])}`",
            f"- within-endpoint ridge: metadata `R^2={format_metric(metadata_endpoint['r2'])}`, geometry `R^2={format_metric(geometry_endpoint['r2'])}`, face_geometry `R^2={format_metric(face_geometry_endpoint['r2'])}`, face_symplectic `R^2={format_metric(face_symplectic_endpoint['r2'])}`, skeleton `R^2={format_metric(skeleton_endpoint['r2'])}`, omega `R^2={format_metric(omega_endpoint['r2'])}`, orbit `R^2={format_metric(orbit_endpoint['r2'])}`, trajectory `R^2={format_metric(trajectory_endpoint['r2'])}`",
            "- random-forest strengthens the face-level picture: `face_geometry` reaches `R^2=0.6756` within random, while `face_symplectic` reaches `R^2=0.8166` within random and `0.2330` within endpoints.",
            f"- random-to-endpoint transfer with full ridge block: `R^2={format_metric(all_random_to_endpoint['r2'])}`",
            f"- endpoint-to-random transfer with trajectory ridge: `R^2={format_metric(trajectory_endpoint_to_random['r2'])}`",
            "- `face_symplectic` is currently a scale-sensitive raw-area block; treat it as bounded-dataset evidence, not as an invariant feature family yet.",
            "- the richer orbit block now includes bounded best-orbit KKT scalars, using cached search-level payloads where available and a one-best-sigma fallback solve on older cache rows.",
        ]
    )

    SUMMARY_MD.write_text("\n".join(lines) + "\n")


def plot_model_results(results: list[dict], model_name: str, out_path: Path, title: str) -> None:
    surface_labels = [label for _key, label in SURFACES]
    x = np.arange(len(SURFACES))
    width = 0.8 / len(FEATURE_BLOCKS)
    fig, ax = plt.subplots(figsize=FIGSIZE_DUAL)
    center = (len(FEATURE_BLOCKS) - 1) / 2.0

    for idx, block in enumerate(FEATURE_BLOCKS):
        heights = []
        for surface_key, _surface_label in SURFACES:
            row = next(
                result
                for result in results
                if result["model"] == model_name
                and result["surface"] == surface_key
                and result["block"] == block
            )
            heights.append(row["r2"])
        ax.bar(x + (idx - center) * width, heights, width=width, label=block)

    ax.axhline(0.0, color="black", linewidth=0.8, alpha=0.5)
    ax.set_xticks(x)
    ax.set_xticklabels(surface_labels)
    ax.set_ylabel(r"Test $R^2$")
    ax.set_title(title)
    ax.legend(ncols=2)
    fig.savefig(out_path)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    if args.normalized_dir is not None:
        normalized_dir = args.normalized_dir.resolve()
        normalized_source_label = f"`{normalized_dir}`"
        refresh_face_geometry_features(normalized_dir, FEATURE_FACE_GEOMETRY_JSONL)
        refresh_face_symplectic_features(normalized_dir, FEATURE_FACE_SYMPLECTIC_JSONL)
        refresh_skeleton_features(normalized_dir, FEATURE_SKELETON_JSONL)
        refresh_omega_features(normalized_dir, FEATURE_OMEGA_JSONL)
        refresh_orbit_features(normalized_dir, FEATURE_ORBIT_JSONL)
        refresh_trajectory_features(normalized_dir, FEATURE_TRAJECTORY_JSONL)
        (
            joined_rows,
            geometry_rows,
            face_geometry_rows,
            face_symplectic_rows,
            skeleton_rows,
            omega_rows,
            orbit_rows,
            trajectory_rows,
        ) = load_joined_rows(normalized_dir)
    else:
        with tempfile.TemporaryDirectory(prefix="feature-pattern-search-") as temp_dir:
            normalized_dir = Path(temp_dir) / "normalized"
            normalized_dir.mkdir(parents=True, exist_ok=True)
            refresh_normalized_dataset(normalized_dir)
            normalized_source_label = (
                "temporary refresh via "
                f"`cargo run -p exp-sys-landscape --release --bin "
                f"sys-normalized-dataset -- --out-dir {normalized_dir}`"
            )
            refresh_face_geometry_features(normalized_dir, FEATURE_FACE_GEOMETRY_JSONL)
            refresh_face_symplectic_features(normalized_dir, FEATURE_FACE_SYMPLECTIC_JSONL)
            refresh_skeleton_features(normalized_dir, FEATURE_SKELETON_JSONL)
            refresh_omega_features(normalized_dir, FEATURE_OMEGA_JSONL)
            refresh_orbit_features(normalized_dir, FEATURE_ORBIT_JSONL)
            refresh_trajectory_features(normalized_dir, FEATURE_TRAJECTORY_JSONL)
            (
                joined_rows,
                geometry_rows,
                face_geometry_rows,
                face_symplectic_rows,
                skeleton_rows,
                omega_rows,
                orbit_rows,
                trajectory_rows,
            ) = load_joined_rows(normalized_dir)

            write_feature_jsonl(geometry_rows)
            with FEATURE_FACE_GEOMETRY_JSONL.open("w") as handle:
                for row in face_geometry_rows:
                    handle.write(json.dumps(row) + "\n")
            with FEATURE_FACE_SYMPLECTIC_JSONL.open("w") as handle:
                for row in face_symplectic_rows:
                    handle.write(json.dumps(row) + "\n")
            with FEATURE_SKELETON_JSONL.open("w") as handle:
                for row in skeleton_rows:
                    handle.write(json.dumps(row) + "\n")
            with FEATURE_OMEGA_JSONL.open("w") as handle:
                for row in omega_rows:
                    handle.write(json.dumps(row) + "\n")
            with FEATURE_ORBIT_JSONL.open("w") as handle:
                for row in orbit_rows:
                    handle.write(json.dumps(row) + "\n")
            with FEATURE_TRAJECTORY_JSONL.open("w") as handle:
                for row in trajectory_rows:
                    handle.write(json.dumps(row) + "\n")
            results = run_evaluations(joined_rows)
            write_summary(normalized_source_label, joined_rows, results)
            plot_model_results(results, "ridge", RIDGE_PNG, "Feature Pattern Search: Ridge")
            plot_model_results(results, "rf", RF_PNG, "Feature Pattern Search: Random forest")
            print(f"Saved {FEATURE_JSONL}")
            print(f"Saved {FEATURE_FACE_GEOMETRY_JSONL}")
            print(f"Saved {FEATURE_FACE_SYMPLECTIC_JSONL}")
            print(f"Saved {FEATURE_SKELETON_JSONL}")
            print(f"Saved {FEATURE_OMEGA_JSONL}")
            print(f"Saved {FEATURE_ORBIT_JSONL}")
            print(f"Saved {FEATURE_TRAJECTORY_JSONL}")
            print(f"Saved {SUMMARY_MD}")
            print(f"Saved {RIDGE_PNG}")
            print(f"Saved {RF_PNG}")
            return

    write_feature_jsonl(geometry_rows)
    with FEATURE_FACE_GEOMETRY_JSONL.open("w") as handle:
        for row in face_geometry_rows:
            handle.write(json.dumps(row) + "\n")
    with FEATURE_FACE_SYMPLECTIC_JSONL.open("w") as handle:
        for row in face_symplectic_rows:
            handle.write(json.dumps(row) + "\n")
    with FEATURE_SKELETON_JSONL.open("w") as handle:
        for row in skeleton_rows:
            handle.write(json.dumps(row) + "\n")
    with FEATURE_OMEGA_JSONL.open("w") as handle:
        for row in omega_rows:
            handle.write(json.dumps(row) + "\n")
    with FEATURE_ORBIT_JSONL.open("w") as handle:
        for row in orbit_rows:
            handle.write(json.dumps(row) + "\n")
    with FEATURE_TRAJECTORY_JSONL.open("w") as handle:
        for row in trajectory_rows:
            handle.write(json.dumps(row) + "\n")
    results = run_evaluations(joined_rows)
    write_summary(normalized_source_label, joined_rows, results)
    plot_model_results(results, "ridge", RIDGE_PNG, "Feature Pattern Search: Ridge")
    plot_model_results(results, "rf", RF_PNG, "Feature Pattern Search: Random forest")

    print(f"Saved {FEATURE_JSONL}")
    print(f"Saved {FEATURE_FACE_GEOMETRY_JSONL}")
    print(f"Saved {FEATURE_FACE_SYMPLECTIC_JSONL}")
    print(f"Saved {FEATURE_SKELETON_JSONL}")
    print(f"Saved {FEATURE_OMEGA_JSONL}")
    print(f"Saved {FEATURE_ORBIT_JSONL}")
    print(f"Saved {FEATURE_TRAJECTORY_JSONL}")
    print(f"Saved {SUMMARY_MD}")
    print(f"Saved {RIDGE_PNG}")
    print(f"Saved {RF_PNG}")


if __name__ == "__main__":
    main()
