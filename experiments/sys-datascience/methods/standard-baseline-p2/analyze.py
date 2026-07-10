#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""P2 retained-table missing standard baselines."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path
import sys
from typing import Any

import numpy as np
from sklearn.base import clone
from sklearn.ensemble import HistGradientBoostingClassifier, HistGradientBoostingRegressor
from sklearn.linear_model import ElasticNet, Lasso, LogisticRegression
from sklearn.metrics import (
    average_precision_score,
    mean_absolute_error,
    r2_score,
    roc_auc_score,
)
from sklearn.model_selection import GroupShuffleSplit
from sklearn.pipeline import Pipeline, make_pipeline
from sklearn.preprocessing import StandardScaler

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    active_invariant_numeric_feature_names,
    load_trusted_random_tables,
    matrix_for,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--test-size", type=float, default=0.25)
    parser.add_argument("--random-state", type=int, default=20260708)
    parser.add_argument("--tail-quantile", type=float, default=0.90)
    parser.add_argument("--top-score-fraction", type=float, default=0.10)
    parser.add_argument("--max-iter", type=int, default=300)
    return parser.parse_args()


def write_tsv(path: Path, rows: list[dict[str, Any]], fields: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: row.get(field, "") for field in fields})


def json_safe(value: Any) -> Any:
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    if isinstance(value, np.ndarray):
        return [json_safe(item) for item in value.tolist()]
    if isinstance(value, dict):
        return {str(key): json_safe(item) for key, item in value.items()}
    if isinstance(value, list | tuple):
        return [json_safe(item) for item in value]
    return value


def feature_family(name: str) -> str:
    if name.startswith("ridge_symp_area_"):
        return "ridge_symp_area"
    if (
        name
        in {
            "facet_count",
            "vertex_count",
            "edge_count",
            "ridge_count",
            "is_simple",
            "simple_vertex_fraction",
            "edge_density",
        }
        or name.startswith("vertex_incident_facets_")
        or name.startswith("vertex_degree_")
        or name.startswith("ridge_size_")
        or name.startswith("facet_vertex_count_")
        or name.startswith("facet_neighbor_count_")
    ):
        return "combinatorial_counts"
    return "other_invariant"


def group_labels(rows: list[dict[str, Any]]) -> np.ndarray:
    return np.array(
        [
            f"{row.get('capacity_source', 'missing')}:F{row.get('facet_count', 'missing')}"
            for row in rows
        ]
    )


def top_fraction_mask(scores: np.ndarray, fraction: float) -> np.ndarray:
    if len(scores) == 0:
        return np.zeros(0, dtype=bool)
    selected = max(1, int(np.ceil(len(scores) * fraction)))
    order = np.argsort(scores, kind="mergesort")
    mask = np.zeros(len(scores), dtype=bool)
    mask[order[-selected:]] = True
    return mask


def top_score_summary(
    y_true: np.ndarray, scores: np.ndarray, *, fraction: float, tail_cutoff: float
) -> dict[str, float | int | None]:
    selected_mask = top_fraction_mask(scores, fraction)
    selected_y = y_true[selected_mask]
    target = y_true >= tail_cutoff
    selected_target = target[selected_mask]
    base_rate = float(np.mean(target)) if len(target) else 0.0
    precision = float(np.mean(selected_target)) if len(selected_target) else 0.0
    return {
        "top_score_fraction": fraction,
        "selected_rows": int(np.sum(selected_mask)),
        "tail_positive_rows": int(np.sum(target)),
        "tail_base_rate": base_rate,
        "top_score_tail_precision": precision,
        "top_score_tail_recall": (
            float(np.sum(selected_target) / np.sum(target)) if np.sum(target) else 0.0
        ),
        "top_score_tail_enrichment": precision / base_rate if base_rate else None,
        "top_score_max_sys": float(np.max(selected_y)) if len(selected_y) else None,
        "top_score_mean_sys": float(np.mean(selected_y)) if len(selected_y) else None,
    }


def evaluate_regression(
    *,
    name: str,
    model: Any,
    x_train: np.ndarray,
    x_test: np.ndarray,
    y_train: np.ndarray,
    y_test: np.ndarray,
    top_fraction: float,
    tail_cutoff: float,
) -> tuple[dict[str, Any], Any]:
    fitted = clone(model)
    fitted.fit(x_train, y_train)
    pred = fitted.predict(x_test)
    row = {
        "model": name,
        "task": "regression",
        "train_rows": len(y_train),
        "test_rows": len(y_test),
        "r2": float(r2_score(y_test, pred)) if len(y_test) >= 2 else None,
        "mae": float(mean_absolute_error(y_test, pred)),
        **top_score_summary(
            y_test, np.array(pred, dtype=float), fraction=top_fraction, tail_cutoff=tail_cutoff
        ),
    }
    return row, fitted


def evaluate_classification(
    *,
    name: str,
    model: Any,
    x_train: np.ndarray,
    x_test: np.ndarray,
    y_train_sys: np.ndarray,
    y_test_sys: np.ndarray,
    tail_cutoff: float,
    top_fraction: float,
) -> tuple[dict[str, Any], Any]:
    y_train = y_train_sys >= tail_cutoff
    y_test = y_test_sys >= tail_cutoff
    fitted = clone(model)
    fitted.fit(x_train, y_train.astype(int))
    if hasattr(fitted, "predict_proba"):
        scores = fitted.predict_proba(x_test)[:, 1]
    elif hasattr(fitted, "decision_function"):
        scores = fitted.decision_function(x_test)
    else:
        scores = fitted.predict(x_test)
    scores = np.array(scores, dtype=float)
    unique_test = set(int(value) for value in y_test.astype(int))
    row = {
        "model": name,
        "task": "high_tail_classification",
        "train_rows": len(y_train_sys),
        "test_rows": len(y_test_sys),
        "tail_cutoff_from_train": tail_cutoff,
        "train_tail_positive_rows": int(np.sum(y_train)),
        "test_tail_positive_rows": int(np.sum(y_test)),
        "roc_auc": float(roc_auc_score(y_test, scores)) if unique_test == {0, 1} else None,
        "average_precision": (
            float(average_precision_score(y_test, scores)) if unique_test == {0, 1} else None
        ),
        **top_score_summary(
            y_test_sys, scores, fraction=top_fraction, tail_cutoff=tail_cutoff
        ),
    }
    return row, fitted


def linear_coefficients(
    *,
    model_name: str,
    model: Any,
    feature_names: list[str],
    count: int = 20,
) -> list[dict[str, Any]]:
    final = model
    if isinstance(model, Pipeline):
        final = model.steps[-1][1]
    coef = getattr(final, "coef_", None)
    if coef is None:
        return []
    coef_array = np.ravel(np.array(coef, dtype=float))
    order = np.argsort(np.abs(coef_array))[::-1][:count]
    return [
        {
            "model": model_name,
            "rank": rank,
            "feature": feature_names[int(index)],
            "family": feature_family(feature_names[int(index)]),
            "coefficient": float(coef_array[int(index)]),
            "abs_coefficient": float(abs(coef_array[int(index)])),
        }
        for rank, index in enumerate(order, start=1)
    ]


def feature_sets(names: list[str]) -> dict[str, list[int]]:
    families = {name: feature_family(name) for name in names}
    all_indices = list(range(len(names)))
    combinatorial = [
        index for index, name in enumerate(names) if families[name] == "combinatorial_counts"
    ]
    ridge = [index for index, name in enumerate(names) if families[name] == "ridge_symp_area"]
    other = [index for index, name in enumerate(names) if families[name] == "other_invariant"]
    return {
        "all_invariant": all_indices,
        "combinatorial_counts_only": combinatorial,
        "ridge_symp_area_only": ridge,
        "other_invariant_only": other,
        "without_combinatorial_counts": [
            index for index in all_indices if index not in set(combinatorial)
        ],
        "without_ridge_symp_area": [index for index in all_indices if index not in set(ridge)],
    }


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    names = active_invariant_numeric_feature_names(rows)
    x = np.array(matrix_for(rows, names), dtype=float)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    groups = group_labels(rows)
    splitter = GroupShuffleSplit(
        n_splits=1, test_size=args.test_size, random_state=args.random_state
    )
    train_idx, test_idx = next(splitter.split(x, y, groups))
    x_train, x_test = x[train_idx], x[test_idx]
    y_train, y_test = y[train_idx], y[test_idx]
    train_groups = groups[train_idx]
    test_groups = groups[test_idx]
    tail_cutoff = float(np.quantile(y_train, args.tail_quantile))

    regression_models = {
        "lasso_regression_alpha_1e-3": make_pipeline(
            StandardScaler(), Lasso(alpha=0.001, max_iter=50000, random_state=args.random_state)
        ),
        "elastic_net_regression_alpha_1e-3_l1_0.5": make_pipeline(
            StandardScaler(),
            ElasticNet(
                alpha=0.001,
                l1_ratio=0.5,
                max_iter=50000,
                random_state=args.random_state,
            ),
        ),
        "hist_gradient_boosting_regression": HistGradientBoostingRegressor(
            learning_rate=0.05,
            max_iter=args.max_iter,
            max_leaf_nodes=15,
            l2_regularization=0.01,
            random_state=args.random_state,
        ),
    }
    classification_models = {
        "elastic_net_logistic_high_tail": make_pipeline(
            StandardScaler(),
            LogisticRegression(
                solver="saga",
                l1_ratio=0.5,
                C=0.5,
                class_weight="balanced",
                max_iter=20000,
                tol=1e-3,
                random_state=args.random_state,
            ),
        ),
        "hist_gradient_boosting_high_tail": HistGradientBoostingClassifier(
            learning_rate=0.05,
            max_iter=args.max_iter,
            max_leaf_nodes=15,
            l2_regularization=0.01,
            random_state=args.random_state,
        ),
    }

    regression_rows: list[dict[str, Any]] = []
    classification_rows: list[dict[str, Any]] = []
    coefficient_rows: list[dict[str, Any]] = []
    for name, model in regression_models.items():
        row, fitted = evaluate_regression(
            name=name,
            model=model,
            x_train=x_train,
            x_test=x_test,
            y_train=y_train,
            y_test=y_test,
            top_fraction=args.top_score_fraction,
            tail_cutoff=tail_cutoff,
        )
        regression_rows.append(row)
        coefficient_rows.extend(
            linear_coefficients(model_name=name, model=fitted, feature_names=names)
        )
    for name, model in classification_models.items():
        row, fitted = evaluate_classification(
            name=name,
            model=model,
            x_train=x_train,
            x_test=x_test,
            y_train_sys=y_train,
            y_test_sys=y_test,
            tail_cutoff=tail_cutoff,
            top_fraction=args.top_score_fraction,
        )
        classification_rows.append(row)
        coefficient_rows.extend(
            linear_coefficients(model_name=name, model=fitted, feature_names=names)
        )

    ablation_rows: list[dict[str, Any]] = []
    sets = feature_sets(names)
    for set_name, indices in sets.items():
        if not indices:
            continue
        set_names = [names[index] for index in indices]
        set_x_train = x_train[:, indices]
        set_x_test = x_test[:, indices]
        reg_row, _ = evaluate_regression(
            name=f"{set_name}:hist_gradient_boosting_regression",
            model=regression_models["hist_gradient_boosting_regression"],
            x_train=set_x_train,
            x_test=set_x_test,
            y_train=y_train,
            y_test=y_test,
            top_fraction=args.top_score_fraction,
            tail_cutoff=tail_cutoff,
        )
        cls_row, _ = evaluate_classification(
            name=f"{set_name}:hist_gradient_boosting_high_tail",
            model=classification_models["hist_gradient_boosting_high_tail"],
            x_train=set_x_train,
            x_test=set_x_test,
            y_train_sys=y_train,
            y_test_sys=y_test,
            tail_cutoff=tail_cutoff,
            top_fraction=args.top_score_fraction,
        )
        for row in (reg_row, cls_row):
            row["feature_set"] = set_name
            row["feature_count"] = len(set_names)
            row["families"] = ",".join(sorted({feature_family(name) for name in set_names}))
            ablation_rows.append(row)

    family_counts: dict[str, int] = {}
    for name in names:
        family = feature_family(name)
        family_counts[family] = family_counts.get(family, 0) + 1
    train_group_counts = {
        str(group): int(np.sum(train_groups == group)) for group in sorted(set(train_groups))
    }
    test_group_counts = {
        str(group): int(np.sum(test_groups == group)) for group in sorted(set(test_groups))
    }
    summary = {
        "packet": "standard-baseline-p2",
        "tables_dir": str(args.tables_dir),
        "row_count": len(rows),
        "provenance_row_count": len(provenance_rows),
        "feature_count": len(names),
        "feature_family_counts": family_counts,
        "train_rows": int(len(train_idx)),
        "test_rows": int(len(test_idx)),
        "grouping": "capacity_source:facet_count",
        "train_group_counts": train_group_counts,
        "test_group_counts": test_group_counts,
        "tail_quantile": args.tail_quantile,
        "tail_cutoff_from_train": tail_cutoff,
        "top_score_fraction": args.top_score_fraction,
        "max_iter": args.max_iter,
        "max_sys": float(np.max(y)),
        "rows_with_sys_gt_1": int(np.sum(y > 1.0)),
        "artifacts": [
            "summary.json",
            "regression-metrics.tsv",
            "high-tail-classification-metrics.tsv",
            "feature-family-ablation.tsv",
            "linear-top-coefficients.tsv",
            "command.txt",
        ],
        "candidate_proposer_disposition": (
            "no validated candidate-proposer: P2 uses already evaluated retained "
            "rows and does not rank unevaluated generated candidates"
        ),
        "thesis_boundary": (
            "standard-method retained-table coverage only; no arbitrary random "
            "distribution, calibrated hit-rate, or generated-candidate claim"
        ),
    }

    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_json(args.out_dir / "summary.json", json_safe(summary))
    write_tsv(
        args.out_dir / "regression-metrics.tsv",
        [json_safe(row) for row in regression_rows],
        [
            "model",
            "task",
            "train_rows",
            "test_rows",
            "r2",
            "mae",
            "top_score_fraction",
            "selected_rows",
            "tail_positive_rows",
            "tail_base_rate",
            "top_score_tail_precision",
            "top_score_tail_recall",
            "top_score_tail_enrichment",
            "top_score_max_sys",
            "top_score_mean_sys",
        ],
    )
    write_tsv(
        args.out_dir / "high-tail-classification-metrics.tsv",
        [json_safe(row) for row in classification_rows],
        [
            "model",
            "task",
            "train_rows",
            "test_rows",
            "tail_cutoff_from_train",
            "train_tail_positive_rows",
            "test_tail_positive_rows",
            "roc_auc",
            "average_precision",
            "top_score_fraction",
            "selected_rows",
            "tail_positive_rows",
            "tail_base_rate",
            "top_score_tail_precision",
            "top_score_tail_recall",
            "top_score_tail_enrichment",
            "top_score_max_sys",
            "top_score_mean_sys",
        ],
    )
    write_tsv(
        args.out_dir / "feature-family-ablation.tsv",
        [json_safe(row) for row in ablation_rows],
        [
            "feature_set",
            "families",
            "feature_count",
            "model",
            "task",
            "train_rows",
            "test_rows",
            "r2",
            "mae",
            "tail_cutoff_from_train",
            "train_tail_positive_rows",
            "test_tail_positive_rows",
            "roc_auc",
            "average_precision",
            "top_score_fraction",
            "selected_rows",
            "tail_positive_rows",
            "tail_base_rate",
            "top_score_tail_precision",
            "top_score_tail_recall",
            "top_score_tail_enrichment",
            "top_score_max_sys",
            "top_score_mean_sys",
        ],
    )
    write_tsv(
        args.out_dir / "linear-top-coefficients.tsv",
        [json_safe(row) for row in coefficient_rows],
        ["model", "rank", "feature", "family", "coefficient", "abs_coefficient"],
    )
    with (args.out_dir / "command.txt").open("w") as handle:
        handle.write(" ".join(sys.argv))
        handle.write("\n")

    best_regression = max(
        regression_rows,
        key=lambda row: (
            float(row["r2"]) if row["r2"] is not None else float("-inf"),
            -float(row["mae"]),
        ),
    )
    best_classification = max(
        classification_rows,
        key=lambda row: (
            float(row["average_precision"])
            if row["average_precision"] is not None
            else float("-inf")
        ),
    )

    print("# standard-baseline-p2")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- invariant features: `{len(names)}`")
    print(f"- train/test rows: `{len(train_idx)}` / `{len(test_idx)}`")
    print(f"- train-derived tail cutoff: `{tail_cutoff}`")
    print(
        f"- best regression by R2: `{best_regression['model']}` "
        f"R2=`{best_regression['r2']}`"
    )
    print(
        f"- best classifier by average precision: `{best_classification['model']}` "
        f"AP=`{best_classification['average_precision']}`"
    )
    print(f"- rows with sys > 1: `{int(np.sum(y > 1.0))}`")
    print(f"- wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
