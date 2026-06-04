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
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_geometry.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_face_geometry.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_face_symplectic.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_skeleton.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_omega.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_orbit.jsonl
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_trajectory.jsonl
  - active datascience dataset, or an override passed by `--dataset-dir`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_residual.png
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_residual_summary.md
"""

import argparse
import math
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

from common import DEFAULT_DATASET_DIR, FIGSIZE_DUAL, JoinedRow, load_joined_rows, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
RESIDUAL_PNG = EXPERIMENT_DIR / "feature_pattern_search_residual.png"
RESIDUAL_REPORT = EXPERIMENT_DIR / "feature_pattern_search_residual_summary.md"

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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset-dir",
        type=Path,
        default=DEFAULT_DATASET_DIR,
        help="Dataset directory. Defaults to experiments/sys-landscape/datascience/dataset.",
    )
    return parser.parse_args()


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

    lines.extend(
        [
            "## Verdict",
            "",
            "This endpoint-only residual check records endpoint-side association beyond metadata, not a candidate-proposer.",
            "It does not produce a validated new `sys > 1` row and does not give a rule for proposing fresh candidates before inspecting `sys`, endpoint labels, producer identity, optimizer provenance, or HKO2024-derived status.",
            "Use it as supporting/caveat evidence only.",
            "",
            "Packet verdict: `no-search-output`.",
            "",
        ]
    )

    RESIDUAL_REPORT.write_text("\n".join(lines), encoding="utf-8")

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
    dataset_dir = args.dataset_dir.resolve()

    rows = load_joined_rows(dataset_dir, endpoint_only=True)
    if not rows:
        raise RuntimeError("no endpoint rows were loaded")

    results: dict[str, dict[str, dict[str, float]]] = {}
    for model_name, _model_label in MODEL_SPECS:
        results[model_name] = {}
        for block in TESTED_BLOCKS:
            results[model_name][block] = evaluate_additive_cv(rows, block, model_name)

    plot_residual_deltas(results)
    write_summary(rows, results)

    print(f"Saved {RESIDUAL_PNG}")
    print(f"Saved {RESIDUAL_REPORT}")


if __name__ == "__main__":
    main()
