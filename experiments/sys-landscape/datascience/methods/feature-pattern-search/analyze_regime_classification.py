#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""
Goal: classify endpoint vs random regime on the hostile-landscape surface and
      compare the existing feature families as regime separators.
Input Artifacts:
  - active datascience dataset, or an override passed by `--dataset-dir`
  - feature tables in this directory:
    `feature_geometry.jsonl`, `feature_face_geometry.jsonl`,
    `feature_face_symplectic.jsonl`, `feature_skeleton.jsonl`,
    `feature_omega.jsonl`, `feature_orbit.jsonl`, `feature_trajectory.jsonl`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_summary.md
  - experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_bars.png
"""

import argparse
import math
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

from common import DEFAULT_DATASET_DIR, FIGSIZE_DUAL, JoinedRow, load_joined_rows, repo_path, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
SUMMARY_MD = EXPERIMENT_DIR / "regime_classification_summary.md"
FIGURE_PNG = EXPERIMENT_DIR / "regime_classification_bars.png"

FEATURE_BLOCKS = [
    "null",
    "facet_count",
    "provenance_metadata",
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
MODEL_SPECS = [("logistic", "Logistic regression"), ("rf", "Random forest")]

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


def build_feature_dict(row: JoinedRow, block: str) -> dict[str, float | str]:
    if block == "facet_count":
        return {"facet_count": float(row.metadata["facet_count"])}
    if block == "provenance_metadata":
        return {
            key: value
            for key, value in row.metadata.items()
            if key != "facet_count"
        }
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
    class_counts = np.bincount(y.astype(int))
    min_class_count = int(class_counts.min()) if len(class_counts) > 0 else 0
    n_splits = min(5, n_groups, min_class_count)
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
        f"- dataset source: {normalized_source_label}",
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
            "- `facet_count`: one non-provenance combinatorial scalar from the polytope table",
            "- `provenance_metadata`: dataset/family/role/search-space/optimizer/backend, excluding facet count",
            "- `metadata`: facet count plus dataset/family/role/search-space/optimizer/backend",
            "- `geometry`: cheap dual-vertex summaries after rescaling each polytope to the `vol(K)=1` convention",
            "- `face_geometry`: edge-length and facet-3-volume summaries from the exact face geometry after the `vol(K)=1` rescaling",
            "- `face_symplectic`: ridge-polygon symplectic-area summaries after volume normalization by `vol(K)^(1/2)`",
            "- `skeleton`: combinatorial counts and degree summaries from the exact 4D face lattice",
            "- `omega`: volume-normalized dual-side `omega_0` magnitude summaries, exact omega-sign structure, and directed transition-graph summaries",
            "- `orbit_combinatorics`: cached-`best_sigma` support-size and cycle-structure summaries",
            "- `orbit_geometry`: sigma-local dual-norm and cycle `omega_0` magnitude summaries",
            "- `orbit_search`: bounded best-orbit KKT and search-scalar availability summaries",
            "- `orbit`: cached-`best_sigma` support size plus sigma-local geometry, `omega_0`, transition summaries, and bounded best-orbit KKT scalars",
            "- `trajectory`: endpoint-keyed step-event aggregates such as overshoot mix, phase restarts, and gradient/step-size summaries",
            "- `all`: metadata, geometry, face_geometry, face_symplectic, skeleton, omega, orbit_combinatorics, orbit_geometry, orbit_search, orbit, and trajectory together",
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
            [
                row
                for row in model_rows
                if row["block"] not in {"null", "metadata", "provenance_metadata", "all"}
            ],
            key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
        )
        best_geom_orbit = max(
            [
                row
                for row in model_rows
                if row["block"]
                in {
                    "facet_count",
                    "geometry",
                    "face_geometry",
                    "face_symplectic",
                    "skeleton",
                    "omega",
                    "orbit_combinatorics",
                    "orbit_geometry",
                }
            ],
            key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
        )
        lines.extend(
            [
                "",
                f"- best block by balanced accuracy: `{best['block']}` (`balanced_accuracy={format_metric(best['balanced_accuracy'])}`, `roc_auc={format_metric(best['roc_auc'])}`)",
                f"- best non-metadata block: `{best_nonmeta['block']}` (`balanced_accuracy={format_metric(best_nonmeta['balanced_accuracy'])}`, `roc_auc={format_metric(best_nonmeta['roc_auc'])}`)",
                f"- best non-provenance geometry/orbit block: `{best_geom_orbit['block']}` (`balanced_accuracy={format_metric(best_geom_orbit['balanced_accuracy'])}`, `roc_auc={format_metric(best_geom_orbit['roc_auc'])}`)",
                "",
            ]
        )

    logistic_rows = {row["block"]: row for row in results if row["model"] == "logistic"}
    rf_rows = {row["block"]: row for row in results if row["model"] == "rf"}
    logistic_best = max(
        [
            logistic_rows[block]
            for block in FEATURE_BLOCKS
            if block not in {"null", "metadata", "provenance_metadata", "all"}
        ],
        key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
    )
    rf_best = max(
        [
            rf_rows[block]
            for block in FEATURE_BLOCKS
            if block not in {"null", "metadata", "provenance_metadata", "all"}
        ],
        key=lambda row: (row["balanced_accuracy"], row["roc_auc"]),
    )
    lines.extend(
        [
            "## Interpretation",
            "",
            f"- metadata is the clearest separator, but that block includes regime-linked provenance fields (`dataset`, `family`, `role`, `search_space`, `optimizer`, `backend`), so it is not a pure geometry test.",
            (
                f"- among non-provenance blocks, logistic regression and random forest both favor `{logistic_best['block']}`."
                if logistic_best["block"] == rf_best["block"]
                else f"- among non-provenance blocks, logistic regression favors `{logistic_best['block']}` and random forest favors `{rf_best['block']}`."
            ),
            "- compare `provenance_metadata`, `facet_count`, and the geometry/orbit sub-blocks before treating a high score as geometric signal.",
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
    dataset_dir = args.dataset_dir.resolve()
    normalized_source_label = f"`{repo_path(dataset_dir)}`"
    rows = load_joined_rows(dataset_dir)
    results = run_evaluations(rows)
    write_summary(normalized_source_label, rows, results)
    plot_results(results, FIGURE_PNG)
    print(f"Saved {SUMMARY_MD}")
    print(f"Saved {FIGURE_PNG}")


if __name__ == "__main__":
    main()
