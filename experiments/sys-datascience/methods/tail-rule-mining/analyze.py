#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""Random-only high-tail rule-mining diagnostic."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path
import sys

import numpy as np
from sklearn.metrics import precision_score, recall_score
from sklearn.model_selection import GroupShuffleSplit
from sklearn.tree import DecisionTreeClassifier, _tree

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    dataset_label,
    load_trusted_random_tables,
    matrix_for,
    numeric_feature_names,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-depth", type=int, default=4)
    parser.add_argument("--min-leaf-fraction", type=float, default=0.015)
    parser.add_argument("--random-state", type=int, default=20260627)
    parser.add_argument("--stability-runs", type=int, default=8)
    parser.add_argument("--permutations", type=int, default=32)
    parser.add_argument(
        "--min-bucket-rows",
        type=int,
        default=100,
        help="Minimum rows for within-bucket scalar interpretation diagnostics.",
    )
    parser.add_argument(
        "--stability-depths",
        type=int,
        nargs="+",
        default=[3, 4, 5],
        help="Tree depths for split/hyperparameter stability checks.",
    )
    parser.add_argument(
        "--stability-min-leaf-fractions",
        type=float,
        nargs="+",
        default=[0.01, 0.015, 0.025],
        help="Minimum leaf fractions for split/hyperparameter stability checks.",
    )
    return parser.parse_args()


def first_numeric_field(provenance_rows: list[dict[str, object]], field: str) -> str:
    values = sorted(
        {
            row[field]
            for row in provenance_rows
            if isinstance(row.get(field), int | float)
        }
    )
    if len(values) == 1:
        return str(values[0])
    if len(values) > 1:
        return "multi:" + ",".join(str(value) for value in values)
    return "missing"


def first_height_range(provenance_rows: list[dict[str, object]]) -> str:
    ranges = sorted(
        {
            (float(row["sample_h_min"]), float(row["sample_h_max"]))
            for row in provenance_rows
            if isinstance(row.get("sample_h_min"), int | float)
            and isinstance(row.get("sample_h_max"), int | float)
        }
    )
    if len(ranges) == 1:
        low, high = ranges[0]
        return f"{low:g}:{high:g}"
    if len(ranges) > 1:
        return "multi:" + ",".join(f"{low:g}:{high:g}" for low, high in ranges)
    return "missing"


def categorical_feature_rows(
    rows: list[dict[str, object]],
    provenance_rows: list[dict[str, object]],
    fields: list[str],
) -> list[dict[str, str]]:
    provenance = provenance_by_poly_id(provenance_rows)
    feature_rows: list[dict[str, str]] = []
    for row in rows:
        provenance_for_row = provenance.get(str(row["poly_id"]), [])
        is_product = row.get("capacity_source") == "random_product_sample"
        available = {
            "capacity_source": str(row.get("capacity_source", "missing")),
            "dataset_label": dataset_label(row, provenance_for_row),
            "facet_count": f"F{row.get('facet_count')}",
            "product_bucket": product_bucket(provenance_for_row)
            if is_product
            else "not_product",
            "product_bounces": first_numeric_field(provenance_for_row, "product_bounces")
            if is_product
            else "not_product",
            "sample_height_range": first_height_range(provenance_for_row),
        }
        feature_rows.append({field: available[field] for field in fields})
    return feature_rows


def one_hot_matrix(
    feature_rows: list[dict[str, str]], fields: list[str]
) -> tuple[np.ndarray, list[str]]:
    feature_names: list[str] = []
    columns: list[np.ndarray] = []
    for field in fields:
        values = sorted({row[field] for row in feature_rows})
        for value in values:
            feature_names.append(f"{field}={value}")
            columns.append(
                np.array([1.0 if row[field] == value else 0.0 for row in feature_rows])
            )
    if not columns:
        return np.zeros((len(feature_rows), 0), dtype=float), feature_names
    return np.column_stack(columns), feature_names


def feature_family(name: str) -> str | None:
    if (
        name.startswith("ridge_symp_area_")
        or name.startswith("ridge_symp_over_euclidean_area_")
        or name.startswith("ridge_abs_omega_")
        or name.startswith("ridge_abs_normalized_omega_")
        or name.startswith("allpair_abs_omega_")
        or name.startswith("allpair_abs_normalized_omega_")
        or name.startswith("omega_matrix_")
        or name.startswith("omega_sign_")
        or name.startswith("allpair_zero_fraction")
        or name.startswith("ridge_zero_fraction")
    ):
        return "symplectic_omega"
    if (
        name.startswith("geom_vol1_")
        or name.startswith("geom_cosine_")
        or name.startswith("edge_length_")
        or name.startswith("facet_volume_")
        or name.startswith("ridge_euclidean_area_")
    ):
        return "euclidean_size_spread"
    if (
        name in {"dual_vertex_count", "vertex_count", "edge_count", "ridge_count"}
        or name.startswith("facet_neighbor_count_")
        or name.startswith("facet_vertex_count_")
        or name.startswith("ridge_size_")
        or name == "edge_density"
    ):
        return "combinatorial_counts"
    if name.startswith("transition_"):
        return "transition_graph"
    return None


def matrix_for_names(rows: list[dict[str, object]], names: list[str]) -> np.ndarray:
    if not names:
        return np.zeros((len(rows), 0), dtype=float)
    return np.array(matrix_for(rows, names), dtype=float)


def tree_rule_paths(model: DecisionTreeClassifier, feature_names: list[str]) -> dict[int, str]:
    tree = model.tree_
    paths: dict[int, str] = {}

    def visit(node: int, parts: list[str]) -> None:
        if tree.feature[node] == _tree.TREE_UNDEFINED:
            paths[node] = " and ".join(parts) if parts else "all rows"
            return
        name = feature_names[tree.feature[node]]
        threshold = tree.threshold[node]
        visit(tree.children_left[node], [*parts, f"{name} <= {threshold:.6g}"])
        visit(tree.children_right[node], [*parts, f"{name} > {threshold:.6g}"])

    visit(0, [])
    return paths


def confusion_metrics(y_true: np.ndarray, y_pred: np.ndarray) -> dict[str, float | int]:
    positives = int(np.sum(y_true))
    selected = int(np.sum(y_pred))
    hits = int(np.sum((y_true == 1) & (y_pred == 1)))
    base_rate = float(np.mean(y_true)) if len(y_true) else 0.0
    precision = float(precision_score(y_true, y_pred, zero_division=0))
    recall = float(recall_score(y_true, y_pred, zero_division=0))
    return {
        "rows": int(len(y_true)),
        "positives": positives,
        "selected": selected,
        "hits": hits,
        "base_rate": base_rate,
        "precision": precision,
        "recall": recall,
        "enrichment": precision / base_rate if base_rate else 0.0,
    }


def ranks(values: np.ndarray) -> np.ndarray:
    order = np.argsort(values, kind="mergesort")
    ranked = np.empty(len(values), dtype=float)
    index = 0
    while index < len(values):
        next_index = index + 1
        while (
            next_index < len(values)
            and values[order[next_index]] == values[order[index]]
        ):
            next_index += 1
        average_rank = (index + next_index - 1) / 2.0 + 1.0
        ranked[order[index:next_index]] = average_rank
        index = next_index
    return ranked


def spearman_rank_correlation(left: np.ndarray, right: np.ndarray) -> float | None:
    left_ranks = ranks(left)
    right_ranks = ranks(right)
    left_std = float(np.std(left_ranks))
    right_std = float(np.std(right_ranks))
    if left_std == 0.0 or right_std == 0.0:
        return None
    return float(
        np.mean((left_ranks - np.mean(left_ranks)) * (right_ranks - np.mean(right_ranks)))
        / (left_std * right_std)
    )


INTERPRETABLE_FEATURES = {
    "ridge_symp_area_volnorm_sum": (
        "sum over cyclically ordered primal two-faces R of "
        "0.5 * |sum_i omega0(v_i, v_{i+1})| / sqrt(volume)"
    ),
    "ridge_symp_area_volnorm_mean": (
        "mean over cyclically ordered primal two-faces R of "
        "0.5 * |sum_i omega0(v_i, v_{i+1})| / sqrt(volume)"
    ),
    "ridge_euclidean_area_volnorm_sum": (
        "sum over cyclically ordered primal two-faces R of Euclidean polygon "
        "area in R^4 divided by sqrt(volume)"
    ),
    "ridge_euclidean_area_volnorm_mean": (
        "mean over cyclically ordered primal two-faces R of Euclidean polygon "
        "area in R^4 divided by sqrt(volume)"
    ),
    "ridge_symp_over_euclidean_area_mean": (
        "mean over cyclically ordered primal two-faces R with nonzero "
        "Euclidean area of symplectic polygon area divided by Euclidean "
        "polygon area"
    ),
    "ridge_symp_over_euclidean_area_q25": (
        "25th percentile over cyclically ordered primal two-faces R with "
        "nonzero Euclidean area of symplectic polygon area divided by "
        "Euclidean polygon area"
    ),
    "ridge_symp_over_euclidean_area_median": (
        "median over cyclically ordered primal two-faces R with nonzero "
        "Euclidean area of symplectic polygon area divided by Euclidean "
        "polygon area"
    ),
    "ridge_symp_over_euclidean_area_q75": (
        "75th percentile over cyclically ordered primal two-faces R with "
        "nonzero Euclidean area of symplectic polygon area divided by "
        "Euclidean polygon area"
    ),
    "omega_matrix_vol1_spectral_norm": (
        "largest singular value of the facet-normal matrix "
        "sqrt(volume) * omega0(a_i, a_j)"
    ),
    "ridge_abs_omega_vol1_mean": (
        "mean over adjacent facet pairs of sqrt(volume) * |omega0(a_i, a_j)|"
    ),
    "facet_volume_vol1_sum": (
        "sum over facets of Euclidean facet volume divided by volume^(3/4)"
    ),
    "geom_vol1_pairwise_dist_mean": (
        "mean pairwise Euclidean distance between dual vertices after multiplying "
        "dual coordinates by volume^(1/4)"
    ),
}


def bucket_interpretation_diagnostics(
    rows: list[dict[str, object]], *, min_bucket_rows: int
) -> list[dict[str, object]]:
    diagnostics: list[dict[str, object]] = []
    bucket_keys = sorted(
        {
            (str(row.get("capacity_source", "missing")), int(row.get("facet_count", 0)))
            for row in rows
        }
    )
    for capacity_source, facet_count in bucket_keys:
        bucket_rows = [
            row
            for row in rows
            if str(row.get("capacity_source", "missing")) == capacity_source
            and int(row.get("facet_count", 0)) == facet_count
        ]
        if len(bucket_rows) < min_bucket_rows:
            continue
        sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
        for feature, mathematical_quantity in INTERPRETABLE_FEATURES.items():
            if feature not in bucket_rows[0]:
                continue
            feature_values = np.array([float(row[feature]) for row in bucket_rows], dtype=float)
            rho = spearman_rank_correlation(feature_values, sys_values)
            if rho is None:
                continue
            for quantile_name, quantile_value in [
                ("top_decile", 0.9),
                ("top_five_percent", 0.95),
                ("top_one_percent", 0.99),
            ]:
                threshold = float(np.quantile(sys_values, quantile_value))
                target = sys_values >= threshold
                if rho >= 0.0:
                    selected = feature_values >= np.quantile(feature_values, 0.85)
                    feature_tail = "highest_15_percent"
                else:
                    selected = feature_values <= np.quantile(feature_values, 0.15)
                    feature_tail = "lowest_15_percent"
                metrics = confusion_metrics(target.astype(int), selected.astype(int))
                target_mean = float(np.mean(feature_values[target])) if np.any(target) else None
                rest_mean = float(np.mean(feature_values[~target])) if np.any(~target) else None
                diagnostics.append(
                    {
                        "capacity_source": capacity_source,
                        "facet_count": facet_count,
                        "rows": len(bucket_rows),
                        "label": quantile_name,
                        "sys_threshold": threshold,
                        "feature": feature,
                        "mathematical_quantity": mathematical_quantity,
                        "spearman_with_sys": rho,
                        "feature_tail_rule": feature_tail,
                        "target_feature_mean": target_mean,
                        "rest_feature_mean": rest_mean,
                        **metrics,
                    }
                )
    diagnostics.sort(
        key=lambda row: (
            str(row["label"]),
            -float(row["enrichment"]),
            -abs(float(row["spearman_with_sys"])),
            str(row["capacity_source"]),
            int(row["facet_count"]),
            str(row["feature"]),
        )
    )
    return diagnostics


def summarize_leaves(
    *,
    model: DecisionTreeClassifier,
    feature_names: list[str],
    x_train: np.ndarray,
    y_train_label: np.ndarray,
    sys_train: np.ndarray,
    x_test: np.ndarray,
    y_test_label: np.ndarray,
    sys_test: np.ndarray,
    source: str,
    label_name: str,
) -> list[dict[str, object]]:
    train_leaves = model.apply(x_train)
    test_leaves = model.apply(x_test)
    paths = tree_rule_paths(model, feature_names)
    rows: list[dict[str, object]] = []
    for leaf in sorted(set(train_leaves) | set(test_leaves)):
        train_mask = train_leaves == leaf
        test_mask = test_leaves == leaf
        if not np.any(train_mask) and not np.any(test_mask):
            continue
        train_rate = float(np.mean(y_train_label[train_mask])) if np.any(train_mask) else 0.0
        test_rate = float(np.mean(y_test_label[test_mask])) if np.any(test_mask) else 0.0
        rows.append(
            {
                "label": label_name,
                "source": source,
                "leaf": int(leaf),
                "rule": paths.get(int(leaf), ""),
                "train_rows": int(np.sum(train_mask)),
                "train_positive_rate": train_rate,
                "train_mean_sys": float(np.mean(sys_train[train_mask]))
                if np.any(train_mask)
                else None,
                "train_max_sys": float(np.max(sys_train[train_mask]))
                if np.any(train_mask)
                else None,
                "test_rows": int(np.sum(test_mask)),
                "test_positive_rate": test_rate,
                "test_mean_sys": float(np.mean(sys_test[test_mask])) if np.any(test_mask) else None,
                "test_max_sys": float(np.max(sys_test[test_mask])) if np.any(test_mask) else None,
            }
        )
    rows.sort(
        key=lambda row: (
            float(row["test_positive_rate"]),
            int(row["test_rows"]),
            float(row["train_positive_rate"]),
        ),
        reverse=True,
    )
    return rows


def fit_and_evaluate(
    *,
    x: np.ndarray,
    feature_names: list[str],
    y: np.ndarray,
    train_idx: np.ndarray,
    test_idx: np.ndarray,
    threshold: float,
    label_name: str,
    source: str,
    max_depth: int,
    min_samples_leaf: int,
    random_state: int,
) -> tuple[dict[str, object], list[dict[str, object]]]:
    labels = (y >= threshold).astype(int)
    model = DecisionTreeClassifier(
        max_depth=max_depth,
        min_samples_leaf=min_samples_leaf,
        class_weight="balanced",
        random_state=random_state,
    )
    if x.shape[1] == 0:
        raise SystemExit(f"{source} has no features")
    model.fit(x[train_idx], labels[train_idx])
    pred = model.predict(x[test_idx])
    metrics = confusion_metrics(labels[test_idx], pred)
    summary = {
        "threshold": float(threshold),
        "source": source,
        "label": label_name,
        "feature_count": len(feature_names),
        "max_depth": max_depth,
        "min_samples_leaf": min_samples_leaf,
        "test_metrics": metrics,
        "tree_depth": int(model.get_depth()),
        "tree_leaf_count": int(model.get_n_leaves()),
        "split_features": tree_split_features(model, feature_names),
    }
    leaves = summarize_leaves(
        model=model,
        feature_names=feature_names,
        x_train=x[train_idx],
        y_train_label=labels[train_idx],
        sys_train=y[train_idx],
        x_test=x[test_idx],
        y_test_label=labels[test_idx],
        sys_test=y[test_idx],
        source=source,
        label_name=label_name,
    )
    summary["top_test_leaves"] = leaves[:8]
    return summary, leaves


def tree_split_features(
    model: DecisionTreeClassifier, feature_names: list[str]
) -> list[dict[str, object]]:
    tree = model.tree_
    used: dict[str, int] = {}
    for feature_index in tree.feature:
        if feature_index == _tree.TREE_UNDEFINED:
            continue
        name = feature_names[int(feature_index)]
        used[name] = used.get(name, 0) + 1
    return [
        {"feature": name, "split_count": count}
        for name, count in sorted(used.items(), key=lambda item: (-item[1], item[0]))
    ]


def stability_sweep(
    *,
    x_by_source: dict[str, np.ndarray],
    feature_names_by_source: dict[str, list[str]],
    y: np.ndarray,
    groups: np.ndarray,
    labels: dict[str, float],
    run_count: int,
    depths: list[int],
    min_leaf_fractions: list[float],
    base_random_state: int,
) -> tuple[list[dict[str, object]], list[dict[str, object]]]:
    rows: list[dict[str, object]] = []
    feature_rows: list[dict[str, object]] = []
    for run_index in range(run_count):
        random_state = base_random_state + 1000 + run_index
        splitter = GroupShuffleSplit(n_splits=1, test_size=0.25, random_state=random_state)
        train_idx, test_idx = next(splitter.split(next(iter(x_by_source.values())), y, groups))
        for depth in depths:
            for min_leaf_fraction in min_leaf_fractions:
                min_samples_leaf = max(10, int(len(train_idx) * min_leaf_fraction))
                for label_name, threshold in labels.items():
                    target = (y >= threshold).astype(int)
                    base_rate = float(np.mean(target[test_idx]))
                    for source, x in x_by_source.items():
                        model = DecisionTreeClassifier(
                            max_depth=depth,
                            min_samples_leaf=min_samples_leaf,
                            class_weight="balanced",
                            random_state=random_state,
                        )
                        model.fit(x[train_idx], target[train_idx])
                        pred = model.predict(x[test_idx])
                        metrics = confusion_metrics(target[test_idx], pred)
                        row = {
                            "run_index": run_index,
                            "random_state": random_state,
                            "label": label_name,
                            "source": source,
                            "max_depth": depth,
                            "min_leaf_fraction": min_leaf_fraction,
                            "min_samples_leaf": min_samples_leaf,
                            "test_base_rate": base_rate,
                            **metrics,
                        }
                        rows.append(row)
                        for used in tree_split_features(model, feature_names_by_source[source]):
                            feature_rows.append(
                                {
                                    "run_index": run_index,
                                    "label": label_name,
                                    "source": source,
                                    "max_depth": depth,
                                    "min_leaf_fraction": min_leaf_fraction,
                                    **used,
                                }
                            )
    return rows, feature_rows


def summarize_stability(rows: list[dict[str, object]]) -> dict[str, object]:
    grouped: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in rows:
        grouped.setdefault((str(row["label"]), str(row["source"])), []).append(row)
    summary: dict[str, object] = {}
    for (label, source), group_rows in sorted(grouped.items()):
        metrics = {}
        for field in ["precision", "recall", "enrichment", "selected", "hits"]:
            values = np.array([float(row[field]) for row in group_rows], dtype=float)
            metrics[field] = {
                "min": float(np.min(values)),
                "median": float(np.median(values)),
                "max": float(np.max(values)),
                "q25": float(np.quantile(values, 0.25)),
                "q75": float(np.quantile(values, 0.75)),
            }
        summary[f"{label}:{source}"] = {
            "runs": len(group_rows),
            "metrics": metrics,
        }
    labels = sorted({str(row["label"]) for row in rows})
    comparisons = [
        ("geometry_only", "symplectic_omega_only"),
        ("geometry_only", "euclidean_size_spread_only"),
        ("geometry_only", "strata_only"),
        ("geometry_only", "generator_provenance_only"),
        ("symplectic_omega_only", "euclidean_size_spread_only"),
        ("symplectic_omega_only", "strata_only"),
        ("symplectic_omega_only", "generator_provenance_only"),
    ]
    for label in labels:
        for left, right in comparisons:
            left_values = [
                float(row["enrichment"])
                for row in rows
                if row["label"] == label and row["source"] == left
            ]
            right_values = [
                float(row["enrichment"])
                for row in rows
                if row["label"] == label and row["source"] == right
            ]
            paired_count = min(len(left_values), len(right_values))
            if paired_count:
                wins = sum(
                    1
                    for index in range(paired_count)
                    if left_values[index] > right_values[index]
                )
                summary[f"{label}:{left}_vs_{right}"] = {
                    "paired_runs": paired_count,
                    "left_enrichment_win_fraction": wins / paired_count,
                    "median_enrichment_difference": float(
                        np.median(
                            np.array(left_values[:paired_count])
                            - np.array(right_values[:paired_count])
                        )
                    ),
                }
    return summary


def summarize_feature_stability(
    rows: list[dict[str, object]], *, limit: int = 20
) -> dict[str, list[dict[str, object]]]:
    grouped: dict[tuple[str, str], dict[str, int]] = {}
    run_counts: dict[tuple[str, str], set[int]] = {}
    for row in rows:
        key = (str(row["label"]), str(row["source"]))
        feature = str(row["feature"])
        grouped.setdefault(key, {})[feature] = grouped.setdefault(key, {}).get(feature, 0) + int(
            row["split_count"]
        )
        run_counts.setdefault(key, set()).add(int(row["run_index"]))
    result: dict[str, list[dict[str, object]]] = {}
    for key, counts in sorted(grouped.items()):
        label, source = key
        result[f"{label}:{source}"] = [
            {
                "feature": feature,
                "total_split_count": count,
                "mean_splits_per_run": count / max(1, len(run_counts[key])),
            }
            for feature, count in sorted(counts.items(), key=lambda item: (-item[1], item[0]))[
                :limit
            ]
        ]
    return result


def permutation_null(
    *,
    x_by_source: dict[str, np.ndarray],
    y: np.ndarray,
    labels: dict[str, float],
    train_idx: np.ndarray,
    test_idx: np.ndarray,
    max_depth: int,
    min_samples_leaf: int,
    random_state: int,
    permutations: int,
) -> dict[str, object]:
    rng = np.random.default_rng(random_state + 2000)
    summary: dict[str, object] = {}
    for label_name, threshold in labels.items():
        true_target = (y >= threshold).astype(int)
        for source, x in x_by_source.items():
            observed_model = DecisionTreeClassifier(
                max_depth=max_depth,
                min_samples_leaf=min_samples_leaf,
                class_weight="balanced",
                random_state=random_state,
            )
            observed_model.fit(x[train_idx], true_target[train_idx])
            observed_pred = observed_model.predict(x[test_idx])
            observed = confusion_metrics(true_target[test_idx], observed_pred)
            null_enrichments = []
            for permutation_index in range(permutations):
                shuffled_train = true_target[train_idx].copy()
                rng.shuffle(shuffled_train)
                model = DecisionTreeClassifier(
                    max_depth=max_depth,
                    min_samples_leaf=min_samples_leaf,
                    class_weight="balanced",
                    random_state=random_state + 3000 + permutation_index,
                )
                model.fit(x[train_idx], shuffled_train)
                pred = model.predict(x[test_idx])
                null_enrichments.append(confusion_metrics(true_target[test_idx], pred)["enrichment"])
            null = np.array(null_enrichments, dtype=float)
            p_value = (float(np.sum(null >= float(observed["enrichment"]))) + 1.0) / (
                len(null) + 1.0
            )
            summary[f"{label_name}:{source}"] = {
                "permutations": permutations,
                "observed_enrichment": observed["enrichment"],
                "observed_precision": observed["precision"],
                "observed_recall": observed["recall"],
                "null_enrichment_min": float(np.min(null)) if len(null) else None,
                "null_enrichment_median": float(np.median(null)) if len(null) else None,
                "null_enrichment_max": float(np.max(null)) if len(null) else None,
                "permutation_p_ge_observed_enrichment": p_value,
            }
    return summary


def coarse_baselines(
    *,
    rows: list[dict[str, object]],
    y: np.ndarray,
    labels: dict[str, float],
    test_idx: np.ndarray,
) -> dict[str, object]:
    product_mask = np.array(
        [row.get("capacity_source") == "random_product_sample" for row in rows],
        dtype=bool,
    )
    generic_mask = np.array(
        [row.get("capacity_source") == "random_sample" for row in rows],
        dtype=bool,
    )
    facet_count = np.array([int(row.get("facet_count", 0)) for row in rows], dtype=int)
    facet_ge_10_mask = facet_count >= 10
    predictors = {
        "product_rows": product_mask,
        "generic_rows": generic_mask,
        "facet_count_ge_10": facet_ge_10_mask,
        "product_or_facet_count_ge_10": product_mask | facet_ge_10_mask,
        "product_and_facet_count_ge_10": product_mask & facet_ge_10_mask,
    }
    summary: dict[str, object] = {}
    for label_name, threshold in labels.items():
        target = (y >= threshold).astype(int)
        for name, mask in predictors.items():
            summary[f"{label_name}:{name}"] = {
                "full_table": confusion_metrics(target, mask.astype(int)),
                "grouped_holdout": confusion_metrics(
                    target[test_idx], mask[test_idx].astype(int)
                ),
            }
    return summary


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    fields = list(rows[0].keys())
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        for row in rows:
            writer.writerow(row)


def write_leaf_table(path: Path, rows: list[dict[str, object]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fields = [
        "label",
        "source",
        "leaf",
        "test_positive_rate",
        "test_rows",
        "test_mean_sys",
        "test_max_sys",
        "train_positive_rate",
        "train_rows",
        "train_mean_sys",
        "train_max_sys",
        "rule",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: row.get(field) for field in fields})


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    geometry_names = numeric_feature_names(rows, geometry_only=True)
    family_names = {
        family: [name for name in geometry_names if feature_family(name) == family]
        for family in [
            "symplectic_omega",
            "euclidean_size_spread",
            "combinatorial_counts",
            "transition_graph",
        ]
    }
    x_geometry = matrix_for_names(rows, geometry_names)
    strata_fields = ["capacity_source", "facet_count", "product_bucket"]
    generator_provenance_fields = [
        "capacity_source",
        "product_bounces",
        "sample_height_range",
    ]
    strata_rows = categorical_feature_rows(rows, provenance_rows, strata_fields)
    generator_provenance_rows = categorical_feature_rows(
        rows, provenance_rows, generator_provenance_fields
    )
    x_strata, strata_names = one_hot_matrix(strata_rows, strata_fields)
    x_generator_provenance, generator_provenance_names = one_hot_matrix(
        generator_provenance_rows, generator_provenance_fields
    )
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    groups = np.array(
        [
            str(row.get("capacity_source", "")) + ":" + str(row.get("facet_count", ""))
            for row in rows
        ]
    )
    splitter = GroupShuffleSplit(n_splits=1, test_size=0.25, random_state=args.random_state)
    train_idx, test_idx = next(splitter.split(x_geometry, y, groups))
    min_samples_leaf = max(10, int(len(train_idx) * args.min_leaf_fraction))
    labels = {
        "top_decile": float(np.quantile(y, 0.9)),
        "top_five_percent": float(np.quantile(y, 0.95)),
        "top_one_percent": float(np.quantile(y, 0.99)),
    }

    method_summaries: dict[str, dict[str, object]] = {}
    all_leaves: list[dict[str, object]] = []
    x_by_source = {
        "geometry_only": x_geometry,
        "symplectic_omega_only": matrix_for_names(rows, family_names["symplectic_omega"]),
        "euclidean_size_spread_only": matrix_for_names(
            rows, family_names["euclidean_size_spread"]
        ),
        "combinatorial_counts_only": matrix_for_names(
            rows, family_names["combinatorial_counts"]
        ),
        "transition_graph_only": matrix_for_names(rows, family_names["transition_graph"]),
        "strata_only": x_strata,
        "generator_provenance_only": x_generator_provenance,
    }
    feature_names_by_source = {
        "geometry_only": geometry_names,
        "symplectic_omega_only": family_names["symplectic_omega"],
        "euclidean_size_spread_only": family_names["euclidean_size_spread"],
        "combinatorial_counts_only": family_names["combinatorial_counts"],
        "transition_graph_only": family_names["transition_graph"],
        "strata_only": strata_names,
        "generator_provenance_only": generator_provenance_names,
    }
    for label_name, threshold in labels.items():
        for source, x in x_by_source.items():
            names = feature_names_by_source[source]
            summary, leaves = fit_and_evaluate(
                x=x,
                feature_names=names,
                y=y,
                train_idx=train_idx,
                test_idx=test_idx,
                threshold=threshold,
                label_name=label_name,
                source=source,
                max_depth=args.max_depth,
                min_samples_leaf=min_samples_leaf,
                random_state=args.random_state,
            )
            method_summaries[f"{label_name}:{source}"] = summary
            all_leaves.extend(leaves)

    stability_rows, stability_feature_rows = stability_sweep(
        x_by_source=x_by_source,
        feature_names_by_source=feature_names_by_source,
        y=y,
        groups=groups,
        labels=labels,
        run_count=args.stability_runs,
        depths=args.stability_depths,
        min_leaf_fractions=args.stability_min_leaf_fractions,
        base_random_state=args.random_state,
    )
    bucket_diagnostic_rows = bucket_interpretation_diagnostics(
        rows, min_bucket_rows=args.min_bucket_rows
    )

    all_leaves.sort(
        key=lambda row: (
            str(row["label"]),
            str(row["source"]),
            -float(row["test_positive_rate"]),
            -int(row["test_rows"]),
        )
    )
    summary = {
        "row_count": len(rows),
        "provenance_rows": len(provenance_rows),
        "geometry_feature_count": len(geometry_names),
        "feature_family_counts": {
            family: len(names) for family, names in sorted(family_names.items())
        },
        "strata_fields": strata_fields,
        "strata_feature_count": len(strata_names),
        "generator_provenance_fields": generator_provenance_fields,
        "generator_provenance_feature_count": len(generator_provenance_names),
        "grouping": "capacity_source:facet_count",
        "train_rows": int(len(train_idx)),
        "test_rows": int(len(test_idx)),
        "max_depth": args.max_depth,
        "min_samples_leaf": min_samples_leaf,
        "thresholds": labels,
        "methods": method_summaries,
        "stability_runs": args.stability_runs,
        "stability_depths": args.stability_depths,
        "stability_min_leaf_fractions": args.stability_min_leaf_fractions,
        "min_bucket_rows": args.min_bucket_rows,
        "stability_summary": summarize_stability(stability_rows),
        "stability_feature_summary": summarize_feature_stability(stability_feature_rows),
        "coarse_baselines": coarse_baselines(
            rows=rows,
            y=y,
            labels=labels,
            test_idx=test_idx,
        ),
        "bucket_interpretation_diagnostic_count": len(bucket_diagnostic_rows),
        "top_bucket_interpretation_diagnostics": bucket_diagnostic_rows[:40],
        "permutation_null": permutation_null(
            x_by_source=x_by_source,
            y=y,
            labels=labels,
            train_idx=train_idx,
            test_idx=test_idx,
            max_depth=args.max_depth,
            min_samples_leaf=min_samples_leaf,
            random_state=args.random_state,
            permutations=args.permutations,
        ),
        "candidate_proposer_disposition": (
            "no validated candidate-proposer: rules were mined and evaluated "
            "inside an already evaluated table"
        ),
    }
    write_json(args.out_dir / "summary.json", summary)
    write_leaf_table(args.out_dir / "leaf-rules.tsv", all_leaves)
    write_tsv(args.out_dir / "stability-runs.tsv", stability_rows)
    write_tsv(args.out_dir / "stability-split-features.tsv", stability_feature_rows)
    write_tsv(args.out_dir / "bucket-interpretation-diagnostics.tsv", bucket_diagnostic_rows)

    print("# tail-rule-mining")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- geometry features: `{len(geometry_names)}`")
    for family, names in sorted(family_names.items()):
        print(f"- {family} features: `{len(names)}`")
    print(f"- strata features: `{len(strata_names)}`")
    print(f"- generator provenance features: `{len(generator_provenance_names)}`")
    print(f"- permutations: `{args.permutations}`")
    for key, result in method_summaries.items():
        metrics = result["test_metrics"]
        print(
            f"- {key}: precision=`{metrics['precision']}`, recall=`{metrics['recall']}`, "
            f"enrichment=`{metrics['enrichment']}`, selected=`{metrics['selected']}`"
        )
    print(f"- stability configurations: `{len(stability_rows)}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
