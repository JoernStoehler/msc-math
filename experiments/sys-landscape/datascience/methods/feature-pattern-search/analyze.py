#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""
Goal: Run a bounded hostile-landscape pattern-search pass on the dataset
      surface, comparing a narrow metadata baseline against cheap geometry
      features derived from exact dual vertices.
Input Artifacts:
  - active datascience dataset, or an override passed by `--dataset-dir`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_geometry.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_face_geometry.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_face_symplectic.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_skeleton.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_omega.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_orbit.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_trajectory.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_ridge.png
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_rf.png
"""

import argparse
import math
import statistics
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

from common import (
    ENDPOINT_DATASETS,
    EXPERIMENT_DIR,
    DEFAULT_DATASET_DIR,
    FEATURE_FACE_GEOMETRY_JSONL,
    FEATURE_FACE_SYMPLECTIC_JSONL,
    FEATURE_GEOMETRY_JSONL,
    FEATURE_OMEGA_JSONL,
    FEATURE_ORBIT_JSONL,
    FEATURE_SKELETON_JSONL,
    FEATURE_TRAJECTORY_JSONL,
    FIGSIZE_DUAL,
    FIGSIZE_SQUARE,
    JoinedRow,
    RANDOM_DATASETS,
    cv_group_id,
    load_jsonl,
    setup,
    write_jsonl,
)

setup()

RIDGE_PNG = EXPERIMENT_DIR / "feature_pattern_search_ridge.png"
RF_PNG = EXPERIMENT_DIR / "feature_pattern_search_rf.png"
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
    "orbit_combinatorics",
    "orbit_geometry",
    "orbit_search",
    "orbit",
    "trajectory",
    "all",
]
MODEL_SPECS = [("ridge", "Ridge"), ("rf", "Random forest")]

ORBIT_COMBINATORICS_KEYS = [
    "orbit_sigma_available",
    "orbit_sigma_count",
    "orbit_sigma_gap_cutoff",
    "orbit_sigma_len",
    "orbit_sigma_fraction",
    "orbit_selected_out_degree_mean",
    "orbit_selected_out_degree_std",
    "orbit_selected_out_degree_min",
    "orbit_selected_out_degree_max",
    "orbit_cycle_zero_fraction",
    "orbit_cycle_transition_fraction",
    "orbit_cycle_bidirectional_fraction",
    "orbit_cycle_facet_intersection_fraction",
]
ORBIT_GEOMETRY_KEYS = [
    "orbit_selected_norm_mean",
    "orbit_selected_norm_std",
    "orbit_selected_norm_min",
    "orbit_selected_norm_max",
    "orbit_cycle_abs_omega_mean",
    "orbit_cycle_abs_omega_std",
    "orbit_cycle_abs_omega_min",
    "orbit_cycle_abs_omega_max",
    "orbit_cycle_abs_omega_le_1e3_fraction",
    "orbit_cycle_abs_omega_le_1e2_fraction",
    "orbit_cycle_abs_omega_le_1e1_fraction",
]
ORBIT_SEARCH_KEYS = [
    "orbit_kkt_available",
    "orbit_search_scalar_available",
    "orbit_result_iterations_log1p",
    "orbit_result_returned_orbit_count",
    "orbit_best_beta_margin",
    "orbit_best_q_error_bound",
    "orbit_best_has_mu",
    "orbit_best_has_xi",
    "orbit_best_is_admissible_exact",
    "orbit_best_is_indeterminate_f64",
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset-dir",
        type=Path,
        default=DEFAULT_DATASET_DIR,
        help="Dataset directory. Defaults to experiments/sys-landscape/datascience/dataset.",
    )
    return parser.parse_args()


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


def build_geometry_features(poly: dict, volume: float) -> dict[str, float]:
    dual_vertices = [
        [parse_rational(coord) for coord in row]
        for row in poly["dual_vertices_rational"]
    ]
    dual_scale = volume ** 0.25
    arr = np.asarray(dual_vertices, dtype=float) * dual_scale
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

    singular_values = centered_singular_values(arr.tolist())

    return {
        "geom_vol1_norm_mean": float(np.mean(norms)),
        "geom_vol1_norm_std": float(np.std(norms)),
        "geom_vol1_norm_min": float(np.min(norms)),
        "geom_vol1_norm_max": float(np.max(norms)),
        "geom_vol1_centroid_norm": centroid_norm,
        "geom_vol1_coord_std_x": float(coord_std[0]),
        "geom_vol1_coord_std_y": float(coord_std[1]),
        "geom_vol1_coord_std_z": float(coord_std[2]),
        "geom_vol1_coord_std_w": float(coord_std[3]),
        "geom_cosine_mean": statistics.mean(cosines) if cosines else 0.0,
        "geom_cosine_std": statistics.pstdev(cosines) if len(cosines) > 1 else 0.0,
        "geom_cosine_min": min(cosines) if cosines else 0.0,
        "geom_cosine_max": max(cosines) if cosines else 0.0,
        "geom_vol1_pairwise_dist_mean": statistics.mean(pairwise_distances)
        if pairwise_distances
        else 0.0,
        "geom_vol1_pairwise_dist_std": statistics.pstdev(pairwise_distances)
        if len(pairwise_distances) > 1
        else 0.0,
        "geom_vol1_pairwise_dist_min": min(pairwise_distances)
        if pairwise_distances
        else 0.0,
        "geom_vol1_pairwise_dist_max": max(pairwise_distances)
        if pairwise_distances
        else 0.0,
        "geom_vol1_sval_1": singular_values[0],
        "geom_vol1_sval_2": singular_values[1],
        "geom_vol1_sval_3": singular_values[2],
        "geom_vol1_sval_4": singular_values[3],
    }

def prefixed_rows(rows: list[dict], prefixes: tuple[str, ...], id_key: str) -> list[dict]:
    return [
        {
            id_key: row[id_key],
            **{
                key: value
                for key, value in row.items()
                if key != id_key and any(key.startswith(prefix) for prefix in prefixes)
            },
        }
        for row in rows
    ]


def split_feature_rows(
    polytope_rows: list[dict], observation_rows: list[dict]
) -> tuple[list[dict], list[dict], list[dict], list[dict], list[dict], list[dict], list[dict]]:
    return (
        prefixed_rows(
            polytope_rows,
            ("facet_count", "geom_"),
            "poly_id",
        ),
        prefixed_rows(
            polytope_rows,
            ("facet_count", "vertex_count", "edge_count", "edge_length_", "facet_volume_"),
            "poly_id",
        ),
        prefixed_rows(polytope_rows, ("facet_count", "ridge_count", "ridge_symp_"), "poly_id"),
        prefixed_rows(
            polytope_rows,
            (
                "facet_count",
                "vertex_count",
                "edge_count",
                "ridge_count",
                "is_simple",
                "simple_vertex_fraction",
                "edge_density",
                "vertex_incident_",
                "vertex_degree_",
                "ridge_size_",
                "facet_vertex_count_",
                "facet_neighbor_count_",
            ),
            "poly_id",
        ),
        prefixed_rows(
            polytope_rows, ("facet_count", "allpair_", "ridge_abs_omega_", "ridge_zero_fraction", "transition_"), "poly_id",
        ),
        prefixed_rows(polytope_rows, ("facet_count", "orbit_"), "poly_id"),
        prefixed_rows(observation_rows, ("trajectory_",), "observation_id"),
    )


def load_joined_rows(
    dataset_dir: Path,
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
    observations = load_jsonl(dataset_dir / "observation-table.jsonl")
    polytope_rows = load_jsonl(dataset_dir / "polytope-table.jsonl")
    polytopes = {row["poly_id"]: row for row in polytope_rows}
    (
        geometry_rows,
        face_geometry_rows,
        face_symplectic_rows,
        skeleton_rows,
        omega_rows,
        orbit_rows,
        trajectory_rows,
    ) = split_feature_rows(polytope_rows, observations)
    geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in geometry_rows
    }
    face_geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in face_geometry_rows
    }
    face_symplectic_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in face_symplectic_rows
    }
    skeleton_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in skeleton_rows
    }
    omega_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in omega_rows
    }
    orbit_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in orbit_rows
    }
    trajectory_by_observation = {
        row["observation_id"]: {key: value for key, value in row.items() if key != "observation_id"}
        for row in trajectory_rows
    }

    rows: list[JoinedRow] = []
    for observation in observations:
        dataset = observation["dataset"]
        regime = "endpoint" if dataset in ENDPOINT_DATASETS else "random"
        if dataset not in ENDPOINT_DATASETS | RANDOM_DATASETS:
            raise ValueError(f"unexpected dataset {dataset}")
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
                geometry=geometry_by_poly[observation["poly_id"]],
                face_geometry=face_geometry_by_poly[observation["poly_id"]],
                face_symplectic=face_symplectic_by_poly[observation["poly_id"]],
                skeleton=skeleton_by_poly[observation["poly_id"]],
                omega=omega_by_poly[observation["poly_id"]],
                orbit=orbit_by_poly[observation["poly_id"]],
                trajectory=trajectory_by_observation[observation["observation_id"]],
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
    if block == "orbit_combinatorics":
        return {key: row.orbit[key] for key in ORBIT_COMBINATORICS_KEYS}
    if block == "orbit_geometry":
        return {key: row.orbit[key] for key in ORBIT_GEOMETRY_KEYS}
    if block == "orbit_search":
        return {key: row.orbit[key] for key in ORBIT_SEARCH_KEYS}
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


def write_summary(
    dataset_source_label: str,
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
        f"- dataset source: {dataset_source_label}",
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
            "- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention",
            "- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling",
            "- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`",
            "- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice",
            "- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries",
            "- `orbit_combinatorics`: cached-`best_sigma` support-size and cycle-structure summaries",
            "- `orbit_geometry`: sigma-local dual-norm and cycle `omega_0` magnitude summaries",
            "- `orbit_search`: bounded best-orbit KKT and search-scalar availability summaries",
            "- `orbit`: the merged orbit packet kept as a reference aggregate",
            "- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries",
            "- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit_combinatorics, orbit_geometry, orbit_search, orbit, and trajectory together",
            "",
            "## Symmetry Status",
            "",
            "| Block | `vol(K)=1` convention | Translation invariant | `Sp(4)`-invariant | Notes |",
            "|-------|------------------------|-----------------------|-------------------|-------|",
            "| `metadata` | no | no | no | Search provenance and family labels, not geometry invariants. |",
            "| `geometry` | yes | no | no | Uses dual-coordinate norms, centroids, and singular values after `vol(K)=1` rescaling. |",
            "| `face_geometry` | yes | yes | no | Euclidean edge/facet sizes on the rescaled polytope. |",
            "| `face_symplectic` | yes | yes | yes | Ridge-polygon symplectic areas divided by `vol(K)^(1/2)`. |",
            "| `skeleton` | yes | yes | yes | Pure combinatorics; unaffected by translation, `Sp(4)`, or scaling. |",
            "| `omega` | yes | no | mixed | `omega_0` magnitudes are volume-normalized, but the dual-coordinate packet still depends on translation gauge; transition graph and zero-sign structure do not. |",
            "| `orbit_combinatorics` | yes | yes | yes | Sigma support counts and cycle-structure summaries. |",
            "| `orbit_geometry` | yes | no | mixed | Sigma-local norms and cycle `omega_0` magnitudes. |",
            "| `orbit_search` | no | no | no | Search-procedure diagnostics and cached KKT scalars. |",
            "| `orbit` | mixed | mixed | mixed | Legacy aggregate of the three orbit sub-blocks above. |",
            "| `trajectory` | no | no | no | Search-procedure diagnostics, not geometry invariants. |",
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
    orbit_combinatorics_random = ridge_rows[("within_random", "orbit_combinatorics")]
    orbit_geometry_random = ridge_rows[("within_random", "orbit_geometry")]
    orbit_search_random = ridge_rows[("within_random", "orbit_search")]
    orbit_random = ridge_rows[("within_random", "orbit")]
    trajectory_random = ridge_rows[("within_random", "trajectory")]
    metadata_random = ridge_rows[("within_random", "metadata")]
    geometry_endpoint = ridge_rows[("within_endpoint", "geometry")]
    face_geometry_endpoint = ridge_rows[("within_endpoint", "face_geometry")]
    face_symplectic_endpoint = ridge_rows[("within_endpoint", "face_symplectic")]
    skeleton_endpoint = ridge_rows[("within_endpoint", "skeleton")]
    omega_endpoint = ridge_rows[("within_endpoint", "omega")]
    orbit_combinatorics_endpoint = ridge_rows[("within_endpoint", "orbit_combinatorics")]
    orbit_geometry_endpoint = ridge_rows[("within_endpoint", "orbit_geometry")]
    orbit_search_endpoint = ridge_rows[("within_endpoint", "orbit_search")]
    orbit_endpoint = ridge_rows[("within_endpoint", "orbit")]
    trajectory_endpoint = ridge_rows[("within_endpoint", "trajectory")]
    metadata_endpoint = ridge_rows[("within_endpoint", "metadata")]
    all_random_to_endpoint = ridge_rows[("random_to_endpoint", "all")]
    trajectory_endpoint_to_random = ridge_rows[("endpoint_to_random", "trajectory")]
    orbit_endpoint_subblocks = {
        "combinatorics": orbit_combinatorics_endpoint,
        "geometry": orbit_geometry_endpoint,
        "search": orbit_search_endpoint,
    }
    best_orbit_endpoint_name, best_orbit_endpoint_row = max(
        orbit_endpoint_subblocks.items(),
        key=lambda item: item[1]["r2"],
    )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            f"- within-random ridge: metadata `R^2={format_metric(metadata_random['r2'])}`, geometry `R^2={format_metric(geometry_random['r2'])}`, face_geometry `R^2={format_metric(face_geometry_random['r2'])}`, face_symplectic `R^2={format_metric(face_symplectic_random['r2'])}`, skeleton `R^2={format_metric(skeleton_random['r2'])}`, omega `R^2={format_metric(omega_random['r2'])}`, orbit `R^2={format_metric(orbit_random['r2'])}`, trajectory `R^2={format_metric(trajectory_random['r2'])}`",
            f"- within-endpoint ridge: metadata `R^2={format_metric(metadata_endpoint['r2'])}`, geometry `R^2={format_metric(geometry_endpoint['r2'])}`, face_geometry `R^2={format_metric(face_geometry_endpoint['r2'])}`, face_symplectic `R^2={format_metric(face_symplectic_endpoint['r2'])}`, skeleton `R^2={format_metric(skeleton_endpoint['r2'])}`, omega `R^2={format_metric(omega_endpoint['r2'])}`, orbit `R^2={format_metric(orbit_endpoint['r2'])}`, trajectory `R^2={format_metric(trajectory_endpoint['r2'])}`",
            f"- orbit split on within-endpoint ridge: combinatorics `R^2={format_metric(orbit_combinatorics_endpoint['r2'])}`, geometry `R^2={format_metric(orbit_geometry_endpoint['r2'])}`, search `R^2={format_metric(orbit_search_endpoint['r2'])}`",
            f"- endpoint-side orbit signal, if any, sits in `{best_orbit_endpoint_name}` (`R^2={format_metric(best_orbit_endpoint_row['r2'])}`)",
            "- random-forest strengthens the face-level picture: `face_geometry` remains strong within random, while volume-normalized `face_symplectic` stays the strongest non-metadata endpoint-side face block.",
            f"- random-to-endpoint transfer with full ridge block: `R^2={format_metric(all_random_to_endpoint['r2'])}`",
            f"- endpoint-to-random transfer with trajectory ridge: `R^2={format_metric(trajectory_endpoint_to_random['r2'])}`",
            "- All geometric magnitude blocks in this packet now use the `vol(K)=1` convention; other symmetry-aware normalizations remain possible and are not ruled out by this packet.",
            "- the richer orbit packet is now split into combinatorics, geometry, and search sub-blocks while keeping the merged `orbit` aggregate for reference.",
        ]
    )

def plot_model_results(results: list[dict], model_name: str, out_path: Path, title: str) -> None:
    fig, axes = plt.subplots(2, 2, figsize=FIGSIZE_SQUARE, sharex=False, sharey=False)
    axes_flat = axes.flatten()
    y = np.arange(len(FEATURE_BLOCKS))

    for ax, (surface_key, surface_label) in zip(axes_flat, SURFACES):
        surface_rows = {
            row["block"]: row
            for row in results
            if row["model"] == model_name and row["surface"] == surface_key
        }
        values = [surface_rows[block]["r2"] for block in FEATURE_BLOCKS]
        colors = [
            "#d62728" if value < 0 else "#4c78a8"
            for value in values
        ]
        ax.barh(y, values, color=colors)
        ax.axvline(0.0, color="black", linewidth=0.8, alpha=0.5)
        ax.set_yticks(y)
        ax.set_yticklabels(FEATURE_BLOCKS)
        ax.invert_yaxis()
        ax.set_title(surface_label)
        ax.set_xlabel(r"Test $R^2$")

    fig.suptitle(title)
    fig.savefig(out_path)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    dataset_dir = args.dataset_dir.resolve()
    (
        joined_rows,
        geometry_rows,
        face_geometry_rows,
        face_symplectic_rows,
        skeleton_rows,
        omega_rows,
        orbit_rows,
        trajectory_rows,
    ) = load_joined_rows(dataset_dir)

    write_jsonl(FEATURE_GEOMETRY_JSONL, geometry_rows)
    write_jsonl(FEATURE_FACE_GEOMETRY_JSONL, face_geometry_rows)
    write_jsonl(FEATURE_FACE_SYMPLECTIC_JSONL, face_symplectic_rows)
    write_jsonl(FEATURE_SKELETON_JSONL, skeleton_rows)
    write_jsonl(FEATURE_OMEGA_JSONL, omega_rows)
    write_jsonl(FEATURE_ORBIT_JSONL, orbit_rows)
    write_jsonl(FEATURE_TRAJECTORY_JSONL, trajectory_rows)
    results = run_evaluations(joined_rows)
    plot_model_results(results, "ridge", RIDGE_PNG, "Feature Pattern Search: Ridge")
    plot_model_results(results, "rf", RF_PNG, "Feature Pattern Search: Random forest")

    print(f"Saved {FEATURE_GEOMETRY_JSONL}")
    print(f"Saved {FEATURE_FACE_GEOMETRY_JSONL}")
    print(f"Saved {FEATURE_FACE_SYMPLECTIC_JSONL}")
    print(f"Saved {FEATURE_SKELETON_JSONL}")
    print(f"Saved {FEATURE_OMEGA_JSONL}")
    print(f"Saved {FEATURE_ORBIT_JSONL}")
    print(f"Saved {FEATURE_TRAJECTORY_JSONL}")
    print(f"Saved {RIDGE_PNG}")
    print(f"Saved {RF_PNG}")


if __name__ == "__main__":
    main()
