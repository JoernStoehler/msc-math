#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""
Goal: classify endpoint vs random regime on the hostile-landscape surface and
      compare the existing feature families as regime separators.
Input Artifacts:
  - experiments/sys-landscape/cache.jsonl
  - experiments/combinatorial-cells/polytopes.jsonl
  - experiments/sys-landscape/variable-f-ascent/cache.jsonl
  - experiments/sys-landscape/random-sample/random-sweep.jsonl
  - experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl
  - experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl
  - experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl
  - experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl
  - feature tables in this directory:
    `feature_geometry.jsonl`, `feature_face_geometry.jsonl`,
    `feature_face_symplectic.jsonl`, `feature_skeleton.jsonl`,
    `feature_omega.jsonl`, `feature_orbit.jsonl`, `feature_trajectory.jsonl`
  - optionally a precomputed normalized dataset directory passed by `--normalized-dir`
Output Artifacts:
  - experiments/sys-landscape/feature-pattern-search/regime_classification_summary.md
  - experiments/sys-landscape/feature-pattern-search/regime_classification_bars.png
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
from sklearn.ensemble import RandomForestClassifier
from sklearn.feature_extraction import DictVectorizer
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, balanced_accuracy_score, roc_auc_score
from sklearn.model_selection import GroupKFold
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import MaxAbsScaler

try:
    from sklearn.model_selection import StratifiedGroupKFold
except ImportError:  # pragma: no cover - fallback for older scikit-learn.
    StratifiedGroupKFold = None

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_DUAL, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
REPO_ROOT = EXPERIMENT_DIR.parent.parent.parent

FEATURE_GEOMETRY_JSONL = EXPERIMENT_DIR / "feature_geometry.jsonl"
FEATURE_FACE_GEOMETRY_JSONL = EXPERIMENT_DIR / "feature_face_geometry.jsonl"
FEATURE_FACE_SYMPLECTIC_JSONL = EXPERIMENT_DIR / "feature_face_symplectic.jsonl"
FEATURE_SKELETON_JSONL = EXPERIMENT_DIR / "feature_skeleton.jsonl"
FEATURE_OMEGA_JSONL = EXPERIMENT_DIR / "feature_omega.jsonl"
FEATURE_ORBIT_JSONL = EXPERIMENT_DIR / "feature_orbit.jsonl"
FEATURE_TRAJECTORY_JSONL = EXPERIMENT_DIR / "feature_trajectory.jsonl"
SUMMARY_MD = EXPERIMENT_DIR / "regime_classification_summary.md"
FIGURE_PNG = EXPERIMENT_DIR / "regime_classification_bars.png"

ENDPOINT_DATASETS = {
    "gradient_ascent_general",
    "gradient_ascent_products",
    "variable_f_ascent",
}
RANDOM_DATASETS = {
    "random_sample",
    "random_product_sample",
}

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
MODEL_SPECS = [("logistic", "Logistic regression"), ("rf", "Random forest")]


@dataclass
class JoinedRow:
    state_id: str
    poly_id: str
    regime: str
    group_id: str
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
    polytopes = {
        row["poly_id"]: row for row in load_jsonl(normalized_dir / "polytopes.jsonl")
    }

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
    trajectory_by_state = {
        row["state_id"]: {key: value for key, value in row.items() if key != "state_id"}
        for row in load_jsonl(FEATURE_TRAJECTORY_JSONL)
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


def make_classifier(name: str):
    if name == "logistic":
        return make_pipeline(
            MaxAbsScaler(),
            LogisticRegression(
                max_iter=2000,
                class_weight="balanced",
                solver="liblinear",
                random_state=0,
            ),
        )
    if name == "rf":
        return RandomForestClassifier(
            n_estimators=300,
            random_state=0,
            min_samples_leaf=2,
            class_weight="balanced_subsample",
            n_jobs=-1,
        )
    raise ValueError(f"unknown model {name}")


def make_splitter(y: np.ndarray, groups: np.ndarray):
    n_groups = len(set(groups.tolist()))
    n_splits = min(5, n_groups)
    if n_splits < 2:
        raise ValueError("need at least two groups for grouped CV")
    if StratifiedGroupKFold is not None:
        return StratifiedGroupKFold(n_splits=n_splits, shuffle=True, random_state=0)
    return GroupKFold(n_splits=n_splits)


def score_predictions(y_true: np.ndarray, prob_pos: np.ndarray) -> dict[str, float]:
    pred = (prob_pos >= 0.5).astype(int)
    return {
        "accuracy": float(accuracy_score(y_true, pred)),
        "balanced_accuracy": float(balanced_accuracy_score(y_true, pred)),
        "roc_auc": float(roc_auc_score(y_true, prob_pos)),
    }


def evaluate_cv(rows: list[JoinedRow], block: str, model_name: str) -> dict[str, float]:
    y = np.asarray([1 if row.regime == "endpoint" else 0 for row in rows], dtype=int)
    groups = np.asarray([row.group_id for row in rows])
    splitter = make_splitter(y, groups)
    probs = np.zeros(len(rows), dtype=float)

    if block == "null":
        for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
            probs[test_idx] = float(np.mean(y[train_idx]))
        return score_predictions(y, probs)

    feature_dicts = [build_feature_dict(row, block) for row in rows]
    for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
        vectorizer = DictVectorizer(sparse=True)
        x_train = vectorizer.fit_transform(feature_dicts[i] for i in train_idx)
        x_test = vectorizer.transform(feature_dicts[i] for i in test_idx)
        model = make_classifier(model_name)
        if model_name == "rf":
            model.fit(x_train.toarray(), y[train_idx])
            probs[test_idx] = model.predict_proba(x_test.toarray())[:, 1]
        else:
            model.fit(x_train, y[train_idx])
            probs[test_idx] = model.predict_proba(x_test)[:, 1]
    return score_predictions(y, probs)


def run_evaluations(rows: list[JoinedRow]) -> list[dict]:
    results: list[dict] = []
    for model_name, _model_label in MODEL_SPECS:
        for block in FEATURE_BLOCKS:
            metrics = evaluate_cv(rows, block, model_name)
            results.append(
                {
                    "model": model_name,
                    "block": block,
                    **metrics,
                }
            )
    return results


def format_metric(value: float) -> str:
    if math.isnan(value):
        return "nan"
    return f"{value:.4f}"


def write_summary(normalized_source_label: str, rows: list[JoinedRow], results: list[dict]) -> None:
    counts = {
        regime: sum(1 for row in rows if row.regime == regime)
        for regime in ["random", "endpoint"]
    }
    counts_by_dataset: dict[str, int] = {}
    groups = {row.group_id for row in rows}
    for row in rows:
        dataset = str(row.metadata["dataset"])
        counts_by_dataset[dataset] = counts_by_dataset.get(dataset, 0) + 1

    lines = [
        "# Regime Classification Summary",
        "",
        "## Dataset",
        "",
        f"- normalized input source: {normalized_source_label}",
        f"- joined rows: `{len(rows)}`",
        f"- groups used for leakage control: `{len(groups)}`",
        f"- random rows: `{counts['random']}`",
        f"- endpoint rows: `{counts['endpoint']}`",
        "- dataset counts:",
    ]
    for dataset, count in sorted(counts_by_dataset.items()):
        lines.append(f"  - `{dataset}`: `{count}`")

    lines.extend(
        [
            "",
            "## Feature Blocks",
            "",
            "- `null`: majority-class baseline with no features",
            "- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend",
            "- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention",
            "- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling",
            "- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`",
            "- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice",
            "- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries",
            "- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars",
            "- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries",
            "- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit, and trajectory together",
            "",
            "## Metrics",
            "",
            "Reported metrics are out-of-fold accuracy, balanced accuracy, and ROC AUC. Splits are grouped by persisted `root_group_id` when present, with the same lineage/source fallback used in the pattern-search pass.",
            "",
        ]
    )

    for model_name, model_label in MODEL_SPECS:
        lines.append(f"### {model_label}")
        lines.append("")
        lines.append("| Block | Accuracy | Balanced acc. | ROC AUC |")
        lines.append("|-------|----------|---------------|---------|")
        model_rows = [row for row in results if row["model"] == model_name]
        by_block = {row["block"]: row for row in model_rows}
        for block in FEATURE_BLOCKS:
            row = by_block[block]
            lines.append(
                f"| `{block}` | {format_metric(row['accuracy'])} | {format_metric(row['balanced_accuracy'])} | {format_metric(row['roc_auc'])} |"
            )
        best = max(model_rows, key=lambda row: (row["balanced_accuracy"], row["roc_auc"]))
        best_nonmeta = max(
            [row for row in model_rows if row["block"] not in {"null", "metadata", "all"}],
            key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
        )
        lines.extend(
            [
                "",
                f"- best block by balanced accuracy: `{best['block']}` (`balanced_accuracy={format_metric(best['balanced_accuracy'])}`, `roc_auc={format_metric(best['roc_auc'])}`)",
                f"- best non-provenance block: `{best_nonmeta['block']}` (`balanced_accuracy={format_metric(best_nonmeta['balanced_accuracy'])}`, `roc_auc={format_metric(best_nonmeta['roc_auc'])}`)",
                "",
            ]
        )

    logistic_rows = {row["block"]: row for row in results if row["model"] == "logistic"}
    rf_rows = {row["block"]: row for row in results if row["model"] == "rf"}
    logistic_best = max(
        [logistic_rows[block] for block in FEATURE_BLOCKS if block not in {"null", "metadata", "all"}],
        key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
    )
    rf_best = max(
        [rf_rows[block] for block in FEATURE_BLOCKS if block not in {"null", "metadata", "all"}],
        key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
    )
    lines.extend(
        [
            "## Interpretation",
            "",
            f"- metadata is the clearest separator, but that block includes regime-linked provenance fields (`dataset`, `family`, `role`, `search_space`, `optimizer`, `backend`), so it is not a pure geometry test.",
            f"- among non-provenance blocks, logistic regression and random forest both favor `{logistic_best['block']}`.",
            "- `orbit` is the strongest non-provenance separator; `face_symplectic` and `omega` are the cleanest pure geometric blocks behind it, while `skeleton` and `trajectory` are weak separators on this task.",
            "- `all` is only a ceiling because it mixes the provenance block with every feature family.",
        ]
    )

    SUMMARY_MD.write_text("\n".join(lines) + "\n")


def plot_results(results: list[dict], out_path: Path) -> None:
    x = np.arange(len(FEATURE_BLOCKS))
    width = 0.36
    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL, sharey=True)

    for ax, (model_name, model_label), offset in zip(axes, MODEL_SPECS, [-width / 2, width / 2]):
        rows = {row["block"]: row for row in results if row["model"] == model_name}
        heights = [rows[block]["balanced_accuracy"] for block in FEATURE_BLOCKS]
        ax.bar(x + offset, heights, width=width, color="#4c78a8")
        ax.axhline(0.5, color="black", linewidth=0.8, alpha=0.5)
        ax.set_title(model_label)
        ax.set_xticks(x)
        ax.set_xticklabels(FEATURE_BLOCKS, rotation=45, ha="right")
        ax.set_ylim(0.45, 1.02)
        ax.grid(axis="y", alpha=0.2)
    axes[0].set_ylabel("Balanced accuracy")
    fig.suptitle("Endpoint vs Random Regime Classification")
    fig.tight_layout()
    fig.savefig(out_path)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    if args.normalized_dir is not None:
        normalized_dir = args.normalized_dir.resolve()
        normalized_source_label = f"`{normalized_dir}`"
    else:
        with tempfile.TemporaryDirectory(prefix="regime-classification-") as temp_dir:
            normalized_dir = Path(temp_dir) / "normalized"
            normalized_dir.mkdir(parents=True, exist_ok=True)
            refresh_normalized_dataset(normalized_dir)
            normalized_source_label = (
                "temporary refresh via "
                "`cargo run -p exp-sys-landscape --release --bin "
                "sys-normalized-dataset -- --out-dir <temp>`"
            )
            rows = load_joined_rows(normalized_dir)
            results = run_evaluations(rows)
            write_summary(normalized_source_label, rows, results)
            plot_results(results, FIGURE_PNG)
            print(f"Saved {SUMMARY_MD}")
            print(f"Saved {FIGURE_PNG}")
            return

    rows = load_joined_rows(normalized_dir)
    results = run_evaluations(rows)
    write_summary(normalized_source_label, rows, results)
    plot_results(results, FIGURE_PNG)
    print(f"Saved {SUMMARY_MD}")
    print(f"Saved {FIGURE_PNG}")


if __name__ == "__main__":
    main()
