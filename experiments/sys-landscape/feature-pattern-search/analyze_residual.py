#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""
Goal: test whether existing endpoint feature blocks add signal beyond the
      metadata baseline on grouped endpoint CV.
Context: this residual packet is a sibling to `analyze.py`. It keeps the
      original analyzer intact, reuses the existing feature JSONL outputs, and
      evaluates additive metadata-first models on the endpoint regime only.
Input Artifacts:
  - experiments/sys-landscape/feature-pattern-search/feature_geometry.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_face_geometry.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_face_symplectic.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_skeleton.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_omega.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_orbit.jsonl
  - experiments/sys-landscape/feature-pattern-search/feature_trajectory.jsonl
  - optionally a precomputed normalized dataset directory passed by
    `--normalized-dir`
Output Artifacts:
  - research/sys-landscape-feature-pattern-search-residual-summary.md
  - experiments/sys-landscape/feature-pattern-search/feature_pattern_search_residual.png
"""

import argparse
import json
import math
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
SUMMARY_MD = REPO_ROOT / "research" / "sys-landscape-feature-pattern-search-residual-summary.md"
RESIDUAL_PNG = EXPERIMENT_DIR / "feature_pattern_search_residual.png"

ENDPOINT_DATASETS = {
    "gradient_ascent_general",
    "gradient_ascent_products",
    "variable_f_ascent",
}

MODEL_SPECS = [("ridge", "Ridge"), ("rf", "Random forest")]
TESTED_BLOCKS = [
    "geometry",
    "face_geometry",
    "face_symplectic",
    "skeleton",
    "omega",
    "orbit",
    "trajectory",
    "all_non_metadata",
]


@dataclass
class JoinedRow:
    state_id: str
    poly_id: str
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


def cv_group_id(state: dict) -> str:
    if state.get("root_group_id"):
        return str(state["root_group_id"])
    if state.get("source_name"):
        return str(state["source_name"])
    return str(state.get("lineage_id") or state["state_id"])


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


def load_joined_rows(normalized_dir: Path) -> list[JoinedRow]:
    states = load_jsonl(normalized_dir / "states.jsonl")
    capacities = {
        row["poly_id"]: row for row in load_jsonl(normalized_dir / "capacity_results.jsonl")
    }
    polytopes = {
        row["poly_id"]: row for row in load_jsonl(normalized_dir / "polytopes.jsonl")
    }

    feature_geometry_path = FEATURE_JSONL
    if not feature_geometry_path.exists():
        raise FileNotFoundError(
            f"{feature_geometry_path} is missing; run "
            "`experiments/sys-landscape/feature-pattern-search/analyze.py` first "
            "to refresh the canonical geometry feature packet."
        )

    geometry_by_poly = {
        row["poly_id"]: {key: value for key, value in row.items() if key != "poly_id"}
        for row in load_jsonl(feature_geometry_path)
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
    trajectory_by_state = {
        row["state_id"]: {key: value for key, value in row.items() if key != "state_id"}
        for row in load_jsonl(FEATURE_TRAJECTORY_JSONL)
    }

    rows: list[JoinedRow] = []
    for state in states:
        dataset = state["dataset"]
        if dataset not in ENDPOINT_DATASETS:
            continue
        rows.append(
            JoinedRow(
                state_id=state["state_id"],
                poly_id=state["poly_id"],
                group_id=cv_group_id(state),
                sys=capacities[state["poly_id"]]["sys"],
                metadata={
                    "facet_count": float(polytopes[state["poly_id"]]["facet_count"]),
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
    return rows


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
    if block == "all_non_metadata":
        return {
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


def evaluate_additive_cv(
    rows: list[JoinedRow],
    block: str,
    model_name: str,
) -> dict[str, float]:
    groups = np.asarray([row.group_id for row in rows])
    y = np.asarray([row.sys for row in rows], dtype=float)
    unique_groups = list(dict.fromkeys(groups.tolist()))
    if len(unique_groups) < 2:
        return {
            "baseline_r2": float("nan"),
            "baseline_rmse": float("nan"),
            "combined_r2": float("nan"),
            "combined_rmse": float("nan"),
            "residual_r2": float("nan"),
            "residual_rmse": float("nan"),
        }

    splitter = GroupKFold(n_splits=min(5, len(unique_groups)))
    folds = list(splitter.split(np.zeros(len(rows)), y, groups))
    metadata_dicts = [build_feature_dict(row, "metadata") for row in rows]
    block_dicts = [build_feature_dict(row, block) for row in rows]

    meta_preds = np.zeros_like(y)
    combined_preds = np.zeros_like(y)
    residual_true = np.zeros_like(y)
    residual_preds = np.zeros_like(y)

    for train_idx, test_idx in folds:
        meta_model = make_regressor(model_name)
        meta_vectorizer = DictVectorizer(sparse=True)
        x_meta_train = meta_vectorizer.fit_transform(metadata_dicts[i] for i in train_idx)
        x_meta_test = meta_vectorizer.transform(metadata_dicts[i] for i in test_idx)
        if model_name == "rf":
            meta_model.fit(x_meta_train.toarray(), y[train_idx])
            meta_train_pred = meta_model.predict(x_meta_train.toarray())
            meta_test_pred = meta_model.predict(x_meta_test.toarray())
        else:
            meta_model.fit(x_meta_train, y[train_idx])
            meta_train_pred = meta_model.predict(x_meta_train)
            meta_test_pred = meta_model.predict(x_meta_test)
        meta_preds[test_idx] = meta_test_pred

        residual_train = y[train_idx] - meta_train_pred
        residual_true[test_idx] = y[test_idx] - meta_test_pred

        block_model = make_regressor(model_name)
        block_vectorizer = DictVectorizer(sparse=True)
        x_block_train = block_vectorizer.fit_transform(block_dicts[i] for i in train_idx)
        x_block_test = block_vectorizer.transform(block_dicts[i] for i in test_idx)
        if model_name == "rf":
            block_model.fit(x_block_train.toarray(), residual_train)
            block_test_pred = block_model.predict(x_block_test.toarray())
        else:
            block_model.fit(x_block_train, residual_train)
            block_test_pred = block_model.predict(x_block_test)
        residual_preds[test_idx] = block_test_pred
        combined_preds[test_idx] = meta_test_pred + block_test_pred

    baseline_r2, baseline_rmse = score_predictions(y, meta_preds)
    combined_r2, combined_rmse = score_predictions(y, combined_preds)
    residual_r2, residual_rmse = score_predictions(residual_true, residual_preds)
    return {
        "baseline_r2": baseline_r2,
        "baseline_rmse": baseline_rmse,
        "combined_r2": combined_r2,
        "combined_rmse": combined_rmse,
        "residual_r2": residual_r2,
        "residual_rmse": residual_rmse,
    }


def format_metric(value: float) -> str:
    if math.isnan(value):
        return "nan"
    return f"{value:.4f}"


def write_summary(rows: list[JoinedRow], results: dict[str, dict[str, dict[str, float]]]) -> None:
    counts_by_dataset: dict[str, int] = {}
    counts_by_group: dict[str, int] = {}
    for row in rows:
        dataset = str(row.metadata["dataset"])
        counts_by_dataset[dataset] = counts_by_dataset.get(dataset, 0) + 1
        counts_by_group[row.group_id] = counts_by_group.get(row.group_id, 0) + 1

    blocks = TESTED_BLOCKS
    lines = [
        "# Feature Pattern Search Residual Summary",
        "",
        "## Dataset",
        "",
        "- endpoint regime only: `gradient_ascent_general`, `gradient_ascent_products`, and `variable_f_ascent`",
        f"- endpoint rows: `{len(rows)}`",
        f"- grouped endpoint folds: `{min(5, len(counts_by_group))}`",
        "- dataset counts:",
    ]
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(f"  - `{dataset}`: `{count}`")

    lines.extend(
        [
            "",
            "## Method",
            "",
            "- baseline: grouped CV model on metadata only",
            "- residual packet: fit metadata on each train fold, subtract its train-fold predictions, then fit one block model on the residuals",
            "- signal criterion: `combined R^2 > metadata R^2`; `residual R^2 > 0` is the direct residual check",
            "- metrics: grouped-CV `R^2` and RMSE",
            "",
            "## Blocks",
            "",
            "- `geometry`: cheap dual-vertex summaries from the existing geometry packet",
            "- `face_geometry`: edge-length and facet-volume summaries",
            "- `face_symplectic`: ridge-polygon symplectic-area summaries",
            "- `skeleton`: combinatorial face-lattice summaries",
            "- `omega`: dual-side `omega_0` magnitudes, sign structure, and transition graph summaries",
            "- `orbit`: cached-best-sigma support and bounded orbit/KKT summaries",
            "- `trajectory`: endpoint trace aggregates",
            "- `all_non_metadata`: concatenation of the seven non-metadata blocks above",
            "",
            "## Metrics",
            "",
            "Reported values are out-of-fold endpoint scores. `Delta R^2` is `combined R^2 - metadata R^2`.",
            "",
        ]
    )

    for model_name, model_label in MODEL_SPECS:
        lines.append(f"### {model_label}")
        lines.append("")
        lines.append("| Block | Metadata R^2 | Combined R^2 | Delta R^2 | Residual R^2 | Metadata RMSE | Combined RMSE | Adds signal? |")
        lines.append("|-------|-------------|--------------|-----------|--------------|---------------|---------------|-------------|")
        for block in blocks:
            result = results[model_name][block]
            delta = result["combined_r2"] - result["baseline_r2"]
            adds_signal = "yes" if delta > 0 else "no"
            lines.append(
                "| "
                f"`{block}` | {format_metric(result['baseline_r2'])} | {format_metric(result['combined_r2'])} | "
                f"{format_metric(delta)} | {format_metric(result['residual_r2'])} | "
                f"{format_metric(result['baseline_rmse'])} | {format_metric(result['combined_rmse'])} | {adds_signal} |"
            )
        lines.append("")

    SUMMARY_MD.write_text("\n".join(lines) + "\n")


def plot_residual_deltas(results: dict[str, dict[str, dict[str, float]]]) -> None:
    x = np.arange(len(TESTED_BLOCKS))
    width = 0.35
    fig, ax = plt.subplots(figsize=FIGSIZE_DUAL)

    for idx, (model_name, model_label) in enumerate(MODEL_SPECS):
        deltas = [
            results[model_name][block]["combined_r2"] - results[model_name][block]["baseline_r2"]
            for block in TESTED_BLOCKS
        ]
        ax.bar(x + (idx - 0.5) * width, deltas, width=width, label=model_label)

    ax.axhline(0.0, color="black", linewidth=0.8, alpha=0.5)
    ax.set_xticks(x)
    ax.set_xticklabels(TESTED_BLOCKS, rotation=20, ha="right")
    ax.set_ylabel(r"$\Delta R^2$ vs metadata baseline")
    ax.set_title("Endpoint residual signal beyond metadata")
    ax.legend()
    fig.tight_layout()
    fig.savefig(RESIDUAL_PNG)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    if args.normalized_dir is not None:
        normalized_dir = args.normalized_dir.resolve()
    else:
        temp_dir = tempfile.TemporaryDirectory(prefix="feature-pattern-search-residual-")
        normalized_dir = Path(temp_dir.name) / "normalized"
        normalized_dir.mkdir(parents=True, exist_ok=True)
        refresh_normalized_dataset(normalized_dir)

    rows = load_joined_rows(normalized_dir)
    if not rows:
        raise RuntimeError("no endpoint rows were loaded")

    results: dict[str, dict[str, dict[str, float]]] = {}
    for model_name, _model_label in MODEL_SPECS:
        results[model_name] = {}
        for block in TESTED_BLOCKS:
            results[model_name][block] = evaluate_additive_cv(rows, block, model_name)

    write_summary(rows, results)
    plot_residual_deltas(results)

    print(f"Saved {SUMMARY_MD}")
    print(f"Saved {RESIDUAL_PNG}")


if __name__ == "__main__":
    main()
