#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""Random/product retained-table high-tail rule-mining diagnostic."""

from __future__ import annotations

import argparse
import csv
import hashlib
from pathlib import Path
import sys

import numpy as np
from sklearn.linear_model import LogisticRegression
from sklearn.model_selection import GroupShuffleSplit
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import StandardScaler
from sklearn.tree import DecisionTreeClassifier, _tree

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    active_invariant_numeric_feature_names,
    dataset_label,
    load_trusted_random_tables,
    matrix_for,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


CURRENT_SCHEMA_REQUIRED_FEATURES = {
    "vertex_count",
    "edge_count",
    "ridge_count",
    "facet_count",
    "ridge_symp_area_sum_over_volume_sqrt",
    "ridge_symp_area_le_1em3_over_volume_sqrt_fraction",
    "ridge_symp_area_le_1em2_over_volume_sqrt_fraction",
    "ridge_symp_area_le_1em1_over_volume_sqrt_fraction",
    "ridge_symp_area_effective_face_count",
    "ridge_symp_area_normalized_entropy",
}
ALLOWED_CAPACITY_SOURCES = {"random_sample", "random_product_sample"}
PRODUCT_PROVENANCE_INT_FIELDS = {"product_k", "product_m", "product_bounces"}
PROVENANCE_FINITE_FIELDS = {"sample_h_min", "sample_h_max"}

TAIL_CUTOFFS = {
    "top_10_percent": 0.90,
    "top_15_percent": 0.85,
    "top_20_percent": 0.80,
    "top_30_percent": 0.70,
}

FILTER_SELECTION_FRACTIONS = [0.10, 0.15, 0.20, 0.30]
ATTRIBUTION_SCOPE_TYPES = {"full_table", "capacity_source"}
SEARCH_BUDGET_FRACTIONS = [0.001, 0.002, 0.005, 0.01, 0.02, 0.05, 0.10]
SEARCH_TARGET_FRACTIONS = [0.001, 0.002, 0.005, 0.01]
SEARCH_TARGET_COUNTS = [1, 5, 10]


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-depth", type=int, default=4)
    parser.add_argument("--min-leaf-fraction", type=float, default=0.015)
    parser.add_argument("--random-state", type=int, default=20260627)
    parser.add_argument("--stability-runs", type=int, default=0)
    parser.add_argument("--permutations", type=int, default=0)
    parser.add_argument(
        "--max-rows",
        type=int,
        default=None,
        help="Deterministic smoke limit after retained random/product filtering. Omit for full table.",
    )
    parser.add_argument(
        "--max-filter-features",
        type=int,
        default=None,
        help="Development smoke cap for single-feature filter diagnostics.",
    )
    parser.add_argument(
        "--min-bucket-rows",
        type=int,
        default=100,
        help="Minimum rows for within-bucket filter-overlap diagnostics.",
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
    return parser.parse_args(argv)


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


def input_preflight(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> dict[str, object]:
    if not rows:
        raise SystemExit("No rows loaded; cannot validate inputs")
    poly_ids = [str(row.get("poly_id", "")) for row in rows]
    missing_poly_ids = sum(1 for poly_id in poly_ids if not poly_id)
    duplicate_poly_ids = len(poly_ids) - len(set(poly_ids))
    if missing_poly_ids or duplicate_poly_ids:
        raise SystemExit(
            "Input validation failed; poly_id must be present and unique "
            f"(missing={missing_poly_ids}, duplicates={duplicate_poly_ids})"
        )
    provenance_ids = [str(row.get("poly_id", "")) for row in provenance_rows]
    provenance_counts: dict[str, int] = {}
    for poly_id in provenance_ids:
        provenance_counts[poly_id] = provenance_counts.get(poly_id, 0) + 1
    bad_provenance = {
        poly_id: provenance_counts.get(poly_id, 0)
        for poly_id in poly_ids
        if provenance_counts.get(poly_id, 0) != 1
    }
    if bad_provenance:
        examples = ", ".join(
            f"{poly_id}:{count}" for poly_id, count in list(bad_provenance.items())[:5]
        )
        raise SystemExit(
            "Input validation failed; expected exactly one provenance row per "
            f"retained polytope ({len(bad_provenance)} bad rows; examples: {examples})"
        )
    row_by_poly_id = {str(row["poly_id"]): row for row in rows}
    provenance_by_id = {str(row["poly_id"]): row for row in provenance_rows}
    bad_structural_counts = {
        "sys": sum(
            1
            for row in rows
            if not isinstance(row.get("sys"), int | float)
            or not np.isfinite(float(row.get("sys")))
        ),
        "capacity_source": sum(
            1 for row in rows if row.get("capacity_source") not in ALLOWED_CAPACITY_SOURCES
        ),
        "facet_count": sum(
            1
            for row in rows
            if not isinstance(row.get("facet_count"), int)
            or isinstance(row.get("facet_count"), bool)
        ),
    }
    bad_structural_counts = {
        key: count for key, count in bad_structural_counts.items() if count
    }
    if bad_structural_counts:
        details = ", ".join(
            f"{field}:{count}" for field, count in sorted(bad_structural_counts.items())
        )
        raise SystemExit(f"Input validation failed; bad structural columns: {details}")
    bad_provenance_fields: dict[str, int] = {}
    for poly_id, provenance in provenance_by_id.items():
        row = row_by_poly_id[poly_id]
        for field in PROVENANCE_FINITE_FIELDS:
            value = provenance.get(field)
            if not isinstance(value, int | float) or not np.isfinite(float(value)):
                bad_provenance_fields[field] = bad_provenance_fields.get(field, 0) + 1
        if row.get("capacity_source") == "random_product_sample":
            for field in PRODUCT_PROVENANCE_INT_FIELDS:
                value = provenance.get(field)
                if not isinstance(value, int) or isinstance(value, bool):
                    bad_provenance_fields[field] = bad_provenance_fields.get(field, 0) + 1
    if bad_provenance_fields:
        details = ", ".join(
            f"{field}:{count}" for field, count in sorted(bad_provenance_fields.items())
        )
        raise SystemExit(
            "Input validation failed; provenance fields are missing or invalid: "
            + details
        )
    keys = {key for row in rows for key in row}
    missing_current = sorted(CURRENT_SCHEMA_REQUIRED_FEATURES - keys)
    if missing_current:
        raise SystemExit(
            "Input validation failed; missing required prepared features: "
            + ", ".join(missing_current)
        )
    bad_required_counts = {
        feature: sum(
            1
            for row in rows
            if not isinstance(row.get(feature), int | float)
            or not np.isfinite(float(row.get(feature)))
        )
        for feature in CURRENT_SCHEMA_REQUIRED_FEATURES
    }
    bad_required_counts = {
        feature: count for feature, count in sorted(bad_required_counts.items()) if count
    }
    if bad_required_counts:
        details = ", ".join(
            f"{feature}:{count}" for feature, count in bad_required_counts.items()
        )
        raise SystemExit(
            "Input validation failed; required features are missing or non-finite "
            f"in some rows: {details}"
        )
    return {
        "status": "passed_current_invariant_schema_structural_provenance_feature_check",
        "row_count": len(rows),
        "provenance_row_count": len(provenance_rows),
        "poly_id_unique": True,
        "allowed_capacity_sources": sorted(ALLOWED_CAPACITY_SOURCES),
        "structural_columns_checked": ["poly_id", "sys", "capacity_source", "facet_count"],
        "provenance_coverage": "exactly_one_row_per_retained_polytope",
        "required_features_checked": sorted(CURRENT_SCHEMA_REQUIRED_FEATURES),
        "required_features_finite_in_all_rows": True,
    }


def feature_family(name: str) -> str | None:
    if name.startswith("ridge_symp_area_"):
        return "ridge_symp_area"
    if (
        name
        in {
            "vertex_count",
            "edge_count",
            "ridge_count",
            "facet_count",
            "is_simple",
            "simple_vertex_fraction",
        }
        or name.startswith("facet_neighbor_count_")
        or name.startswith("facet_vertex_count_")
        or name.startswith("ridge_size_")
        or name.startswith("vertex_degree_")
        or name.startswith("vertex_incident_facets_")
        or name == "edge_density"
    ):
        return "combinatorial_counts"
    return None


def filter_family(name: str) -> str:
    return feature_family(name) or "other_prepared_feature"


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
    precision = hits / selected if selected else 0.0
    recall = hits / positives if positives else 0.0
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
    sorted_values = values[order]
    run_start = np.r_[True, sorted_values[1:] != sorted_values[:-1]]
    run_ids = np.cumsum(run_start) - 1
    starts = np.flatnonzero(run_start)
    ends = np.r_[starts[1:], len(values)]
    average_ranks = (starts + ends - 1) / 2.0 + 1.0
    ranked = np.empty(len(values), dtype=float)
    ranked[order] = average_ranks[run_ids]
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


def stable_train_mask(rows: list[dict[str, object]], *, salt: str) -> np.ndarray:
    mask = []
    for row in rows:
        key = f"{row.get('poly_id', '')}:{salt}".encode()
        digest = hashlib.blake2b(key, digest_size=8).digest()
        value = int.from_bytes(digest, "big") / float(2**64)
        mask.append(value < 0.5)
    return np.array(mask, dtype=bool)


def safe_feature_values(rows: list[dict[str, object]], feature: str) -> np.ndarray:
    return np.array(
        [
            float(row[feature])
            if isinstance(row.get(feature), int | float) and row.get(feature) == row.get(feature)
            else np.nan
            for row in rows
        ],
        dtype=float,
    )


def filter_feature_family(name: str) -> str:
    if name.startswith("control:"):
        return "source_facet_provenance_control"
    return filter_family(name)


def filter_value_arrays(
    rows: list[dict[str, object]],
    provenance_rows: list[dict[str, object]],
    numeric_feature_names_for_rows: list[str],
) -> dict[str, np.ndarray]:
    values = {
        feature: safe_feature_values(rows, feature)
        for feature in numeric_feature_names_for_rows
    }
    control_fields = [
        "capacity_source",
        "dataset_label",
        "facet_count",
        "product_bucket",
        "product_bounces",
        "sample_height_range",
    ]
    control_rows = categorical_feature_rows(rows, provenance_rows, control_fields)
    for field in control_fields:
        for value in sorted({row[field] for row in control_rows}):
            name = f"control:{field}={value}"
            values[name] = np.array(
                [1.0 if row[field] == value else 0.0 for row in control_rows],
                dtype=float,
            )
    return values


def rule_metrics(target: np.ndarray, selected: np.ndarray) -> dict[str, float | int]:
    return confusion_metrics(target.astype(int), selected.astype(int))


def tail_filter_rule(
    *,
    values: np.ndarray,
    sys_values: np.ndarray,
    target: np.ndarray,
    selection_fraction: float,
    fit_mask: np.ndarray | None,
    eval_mask: np.ndarray,
) -> dict[str, object] | None:
    finite = np.isfinite(values) & np.isfinite(sys_values)
    fit = finite & eval_mask if fit_mask is None else finite & fit_mask
    evaluate = finite & eval_mask
    if int(np.sum(fit)) < 10 or int(np.sum(evaluate)) < 10:
        return None
    rho = spearman_rank_correlation(values[fit], sys_values[fit])
    if rho is None:
        return None
    finite_fit_values = values[fit]
    unique_values = set(float(value) for value in np.unique(finite_fit_values))
    if unique_values <= {0.0, 1.0}:
        threshold = 0.5
        if rho >= 0.0:
            selected = values >= threshold
            direction = "binary_present"
        else:
            selected = values < threshold
            direction = "binary_absent"
        rule_type = "binary_indicator"
    elif rho >= 0.0:
        threshold = float(np.quantile(values[fit], 1.0 - selection_fraction))
        selected = values >= threshold
        direction = "highest"
        rule_type = "quantile_tail"
    else:
        threshold = float(np.quantile(values[fit], selection_fraction))
        selected = values <= threshold
        direction = "lowest"
        rule_type = "quantile_tail"
    selected &= finite
    metrics = rule_metrics(target[evaluate], selected[evaluate])
    return {
        "direction": direction,
        "rule_type": rule_type,
        "feature_threshold": threshold,
        "requested_selection_fraction": selection_fraction,
        "actual_eval_selection_fraction": (
            metrics["selected"] / metrics["rows"] if metrics["rows"] else 0.0
        ),
        "spearman_fit": rho,
        **metrics,
    }


def tail_selection_from_rho(
    *,
    values: np.ndarray,
    fit_mask: np.ndarray,
    selection_fraction: float,
    rho: float,
) -> tuple[np.ndarray, str, str, float] | None:
    finite_fit = np.isfinite(values) & fit_mask
    if int(np.sum(finite_fit)) < 10:
        return None
    finite_fit_values = values[finite_fit]
    unique_values = set(float(value) for value in np.unique(finite_fit_values))
    if unique_values <= {0.0, 1.0}:
        threshold = 0.5
        if rho >= 0.0:
            selected = values >= threshold
            direction = "binary_present"
        else:
            selected = values < threshold
            direction = "binary_absent"
        rule_type = "binary_indicator"
    elif rho >= 0.0:
        threshold = float(np.quantile(values[finite_fit], 1.0 - selection_fraction))
        selected = values >= threshold
        direction = "highest"
        rule_type = "quantile_tail"
    else:
        threshold = float(np.quantile(values[finite_fit], selection_fraction))
        selected = values <= threshold
        direction = "lowest"
        rule_type = "quantile_tail"
    selected &= np.isfinite(values)
    return selected, direction, rule_type, threshold


def scope_masks(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> list[dict[str, object]]:
    product_rows = categorical_feature_rows(rows, provenance_rows, ["product_bucket"])
    product_buckets = np.array([row["product_bucket"] for row in product_rows])
    capacity_sources = np.array([str(row.get("capacity_source", "missing")) for row in rows])
    facet_counts = np.array([int(row.get("facet_count", 0)) for row in rows], dtype=int)
    scopes: list[dict[str, object]] = [
        {
            "scope": "full_table",
            "scope_type": "full_table",
            "capacity_source": "all",
            "facet_count": "all",
            "product_bucket": "all",
            "mask": np.ones(len(rows), dtype=bool),
        }
    ]
    for source in sorted(set(capacity_sources)):
        scopes.append(
            {
                "scope": f"capacity_source={source}",
                "scope_type": "capacity_source",
                "capacity_source": source,
                "facet_count": "all",
                "product_bucket": "all",
                "mask": capacity_sources == source,
            }
        )
    for source in sorted(set(capacity_sources)):
        for facet_count in sorted(set(facet_counts[capacity_sources == source])):
            scopes.append(
                {
                    "scope": f"capacity_source={source};facet_count={facet_count}",
                    "scope_type": "capacity_source_facet_count",
                    "capacity_source": source,
                    "facet_count": int(facet_count),
                    "product_bucket": "all",
                    "mask": (capacity_sources == source) & (facet_counts == facet_count),
                }
            )
    for bucket in sorted(set(product_buckets)):
        if bucket == "not_product":
            continue
        scopes.append(
            {
                "scope": f"product_bucket={bucket}",
                "scope_type": "product_bucket",
                "capacity_source": "random_product_sample",
                "facet_count": "all",
                "product_bucket": bucket,
                "mask": product_buckets == bucket,
            }
        )
    return scopes


def single_feature_filter_rows(
    *,
    rows: list[dict[str, object]],
    provenance_rows: list[dict[str, object]],
    feature_values: dict[str, np.ndarray],
    min_scope_rows: int,
) -> tuple[list[dict[str, object]], list[dict[str, object]], list[dict[str, object]]]:
    sys_values = np.array([float(row["sys"]) for row in rows], dtype=float)
    train = stable_train_mask(rows, salt="tail-rule-single-feature-filter-holdout-v1")
    descriptive_rows: list[dict[str, object]] = []
    holdout_rows: list[dict[str, object]] = []
    scopes = scope_masks(rows, provenance_rows)
    for scope in scopes:
        scope_mask = np.array(scope["mask"], dtype=bool)
        if int(np.sum(scope_mask)) < min_scope_rows:
            continue
        scope_train = scope_mask & train
        scope_test = scope_mask & ~train
        scope_train_rows = int(np.sum(scope_train))
        scope_test_rows = int(np.sum(scope_test))
        if scope_train_rows < 20 or scope_test_rows < 20:
            continue
        targets: list[tuple[str, str, float, np.ndarray]] = []
        for label_scope, threshold_values in [
            ("global", sys_values),
            ("scope_local", sys_values[scope_mask]),
        ]:
            for label_name, quantile in TAIL_CUTOFFS.items():
                threshold = float(np.quantile(threshold_values, quantile))
                target = sys_values >= threshold
                if int(np.sum(target & scope_mask)) > 0:
                    targets.append((label_scope, label_name, threshold, target))
        for feature, values in feature_values.items():
            finite_scope = np.isfinite(values) & scope_mask
            finite_train = np.isfinite(values) & scope_train
            if int(np.sum(finite_scope)) < 10 or int(np.sum(finite_train)) < 10:
                continue
            descriptive_rho = spearman_rank_correlation(
                values[finite_scope], sys_values[finite_scope]
            )
            holdout_rho = spearman_rank_correlation(
                values[finite_train], sys_values[finite_train]
            )
            if descriptive_rho is None and holdout_rho is None:
                continue
            for selection_fraction in FILTER_SELECTION_FRACTIONS:
                descriptive_selection = (
                    tail_selection_from_rho(
                        values=values,
                        fit_mask=scope_mask,
                        selection_fraction=selection_fraction,
                        rho=descriptive_rho,
                    )
                    if descriptive_rho is not None
                    else None
                )
                holdout_selection = (
                    tail_selection_from_rho(
                        values=values,
                        fit_mask=scope_train,
                        selection_fraction=selection_fraction,
                        rho=holdout_rho,
                    )
                    if holdout_rho is not None
                    else None
                )
                for label_scope, label_name, threshold, target in targets:
                    if descriptive_selection is not None:
                        selected, direction, rule_type, feature_threshold = descriptive_selection
                        metrics = rule_metrics(target[scope_mask], selected[scope_mask])
                        descriptive_rows.append(
                            {
                                **{key: value for key, value in scope.items() if key != "mask"},
                                "label_scope": label_scope,
                                "tail_label": label_name,
                                "sys_threshold": threshold,
                                "target_threshold_scope": "full_retained_scope_including_eval_rows",
                                "feature": feature,
                                "feature_family": filter_feature_family(feature),
                                "requested_selection_fraction": selection_fraction,
                                "actual_eval_selection_fraction": (
                                    metrics["selected"] / metrics["rows"]
                                    if metrics["rows"]
                                    else 0.0
                                ),
                                "guard": "descriptive_in_table_not_validation",
                                "direction": direction,
                                "rule_type": rule_type,
                                "feature_threshold": feature_threshold,
                                "spearman_fit": descriptive_rho,
                                **metrics,
                            }
                        )
                    if holdout_selection is not None:
                        selected, direction, rule_type, feature_threshold = holdout_selection
                        train_metrics = rule_metrics(target[scope_train], selected[scope_train])
                        test_metrics = rule_metrics(target[scope_test], selected[scope_test])
                        holdout_rows.append(
                            {
                                **{key: value for key, value in scope.items() if key != "mask"},
                                "label_scope": label_scope,
                                "tail_label": label_name,
                                "sys_threshold": threshold,
                                "target_threshold_scope": "full_retained_scope_including_eval_rows",
                                "feature": feature,
                                "feature_family": filter_feature_family(feature),
                                "requested_selection_fraction": selection_fraction,
                                "actual_eval_selection_fraction": (
                                    test_metrics["selected"] / test_metrics["rows"]
                                    if test_metrics["rows"]
                                    else 0.0
                                ),
                                "guard": "threshold_and_direction_fit_on_train_evaluated_on_disjoint_poly_id_hash_split",
                                "train_rows": scope_train_rows,
                                "test_rows": scope_test_rows,
                                "train_enrichment": train_metrics["enrichment"],
                                "train_precision": train_metrics["precision"],
                                "train_recall": train_metrics["recall"],
                                "direction": direction,
                                "rule_type": rule_type,
                                "feature_threshold": feature_threshold,
                                "spearman_fit": holdout_rho,
                                **test_metrics,
                            }
                        )
    sort_key = lambda row: (
        str(row["scope"]),
        str(row["label_scope"]),
        str(row["tail_label"]),
        -float(row["enrichment"]),
        -float(row["precision"]),
        str(row["feature"]),
    )
    descriptive_rows.sort(key=sort_key)
    holdout_rows.sort(key=sort_key)
    grouped_counts: dict[tuple[str, str, str], int] = {}
    top_rows: list[dict[str, object]] = []
    for row in holdout_rows:
        key = (str(row["scope"]), str(row["label_scope"]), str(row["tail_label"]))
        count = grouped_counts.get(key, 0)
        if count < 10:
            top_rows.append(row)
            grouped_counts[key] = count + 1
    return descriptive_rows, holdout_rows, top_rows


def residualize(values: np.ndarray, controls: list[np.ndarray], mask: np.ndarray) -> np.ndarray:
    finite_all = np.isfinite(values)
    for control in controls:
        finite_all &= np.isfinite(control)
    finite_fit = finite_all & mask
    residuals = np.full(len(values), np.nan, dtype=float)
    if int(np.sum(finite_fit)) < 10:
        return residuals
    fit_columns = [np.ones(int(np.sum(finite_fit)))]
    all_columns = [np.ones(int(np.sum(finite_all)))]
    for control in controls:
        scoped = control[finite_fit]
        std = float(np.std(scoped))
        mean = float(np.mean(scoped))
        fit_columns.append((scoped - mean) / std if std else scoped * 0.0)
        all_scoped = control[finite_all]
        all_columns.append((all_scoped - mean) / std if std else all_scoped * 0.0)
    fit_design = np.column_stack(fit_columns)
    coefficients = np.linalg.lstsq(fit_design, values[finite_fit], rcond=None)[0]
    all_design = np.column_stack(all_columns)
    residuals[finite_all] = values[finite_all] - all_design @ coefficients
    return residuals


def score_threshold_selection(
    *,
    train_scores: np.ndarray,
    test_scores: np.ndarray,
    selection_fraction: float,
) -> tuple[np.ndarray, float]:
    threshold = float(np.quantile(train_scores, 1.0 - selection_fraction))
    return test_scores >= threshold, threshold


def feature_set_predictive_power_rows(
    *,
    x_by_source: dict[str, np.ndarray],
    y: np.ndarray,
    labels: dict[str, float],
    train_idx: np.ndarray,
    test_idx: np.ndarray,
    random_state: int,
    max_depth: int,
    min_samples_leaf: int,
) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    model_specs = [
        (
            "logistic_l2_balanced",
            lambda: make_pipeline(
                StandardScaler(),
                LogisticRegression(
                    class_weight="balanced",
                    max_iter=1000,
                    random_state=random_state,
                    solver="liblinear",
                ),
            ),
        ),
        (
            "shallow_tree_balanced",
            lambda: DecisionTreeClassifier(
                max_depth=max_depth,
                min_samples_leaf=min_samples_leaf,
                class_weight="balanced",
                random_state=random_state,
            ),
        ),
    ]
    for label_name, threshold in labels.items():
        target = (y >= threshold).astype(int)
        for source, x in x_by_source.items():
            if x.shape[1] == 0:
                continue
            for model_name, model_factory in model_specs:
                model = model_factory()
                model.fit(x[train_idx], target[train_idx])
                if hasattr(model, "predict_proba"):
                    train_scores = model.predict_proba(x[train_idx])[:, 1]
                    test_scores = model.predict_proba(x[test_idx])[:, 1]
                else:
                    train_scores = model.decision_function(x[train_idx])
                    test_scores = model.decision_function(x[test_idx])
                for selection_fraction in FILTER_SELECTION_FRACTIONS:
                    selected, score_threshold = score_threshold_selection(
                        train_scores=train_scores,
                        test_scores=test_scores,
                        selection_fraction=selection_fraction,
                    )
                    metrics = rule_metrics(target[test_idx], selected)
                    rows.append(
                        {
                            "tail_label": label_name,
                            "sys_threshold": threshold,
                            "target_threshold_scope": "full_retained_table_including_holdout_rows",
                            "feature_set": source,
                            "model": model_name,
                            "feature_count": int(x.shape[1]),
                            "train_score_selection_fraction": selection_fraction,
                            "actual_test_selection_fraction": (
                                metrics["selected"] / metrics["rows"]
                                if metrics["rows"]
                                else 0.0
                            ),
                            "score_threshold_fit_on_train": score_threshold,
                            "train_rows": int(len(train_idx)),
                            "test_rows": int(len(test_idx)),
                            "guard": (
                                "model_and_score_threshold_fit_on_grouped_train_"
                                "evaluated_on_grouped_holdout; target_threshold_is_"
                                "full_retained_table_quantile"
                            ),
                            **metrics,
                        }
                    )
    rows.sort(
        key=lambda row: (
            str(row["tail_label"]),
            str(row["model"]),
            -float(row["enrichment"]),
            -float(row["precision"]),
            str(row["feature_set"]),
        )
    )
    return rows


def model_specs(
    *,
    random_state: int,
    max_depth: int,
    min_samples_leaf: int,
) -> list[tuple[str, object]]:
    return [
        (
            "logistic_l2_balanced",
            lambda: make_pipeline(
                StandardScaler(),
                LogisticRegression(
                    class_weight="balanced",
                    max_iter=1000,
                    random_state=random_state,
                    solver="liblinear",
                ),
            ),
        ),
        (
            "shallow_tree_balanced",
            lambda: DecisionTreeClassifier(
                max_depth=max_depth,
                min_samples_leaf=min_samples_leaf,
                class_weight="balanced",
                random_state=random_state,
            ),
        ),
    ]


def model_scores(model: object, x_train: np.ndarray, x_test: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    if hasattr(model, "predict_proba"):
        return model.predict_proba(x_train)[:, 1], model.predict_proba(x_test)[:, 1]
    return model.decision_function(x_train), model.decision_function(x_test)


def target_masks_by_extreme_sys(sys_values: np.ndarray) -> dict[str, np.ndarray]:
    order = np.argsort(-sys_values, kind="mergesort")
    masks: dict[str, np.ndarray] = {}
    for count in SEARCH_TARGET_COUNTS:
        target_count = min(count, len(sys_values))
        mask = np.zeros(len(sys_values), dtype=bool)
        mask[order[:target_count]] = True
        masks[f"top_{target_count}_sys_rows"] = mask
    for fraction in SEARCH_TARGET_FRACTIONS:
        target_count = max(1, int(np.ceil(len(sys_values) * fraction)))
        mask = np.zeros(len(sys_values), dtype=bool)
        mask[order[:target_count]] = True
        label = f"top_{fraction * 100:g}_percent_sys_rows"
        masks[label] = mask
    return masks


def retained_table_budget_sanity_rows(
    *,
    x_by_source: dict[str, np.ndarray],
    y: np.ndarray,
    train_idx: np.ndarray,
    test_idx: np.ndarray,
    random_state: int,
    max_depth: int,
    min_samples_leaf: int,
) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    y_test = y[test_idx]
    if len(y_test) == 0:
        return rows
    test_best_sys = float(np.max(y_test))
    test_sys_targets = target_masks_by_extreme_sys(y_test)
    for source, x in x_by_source.items():
        if x.shape[1] == 0:
            continue
        # Fit against the top-decile label only to get a frozen cheap high-sys score.
        threshold = float(np.quantile(y, TAIL_CUTOFFS["top_10_percent"]))
        target = (y >= threshold).astype(int)
        for model_name, model_factory in model_specs(
            random_state=random_state,
            max_depth=max_depth,
            min_samples_leaf=min_samples_leaf,
        ):
            model = model_factory()
            model.fit(x[train_idx], target[train_idx])
            _, test_scores = model_scores(model, x[train_idx], x[test_idx])
            score_order = np.argsort(-test_scores, kind="mergesort")
            best_sys_positions = np.flatnonzero(y_test == test_best_sys)
            best_sys_score_ranks = [
                int(np.where(score_order == position)[0][0]) + 1
                for position in best_sys_positions
            ]
            best_sys_rank = min(best_sys_score_ranks)
            for budget_fraction in SEARCH_BUDGET_FRACTIONS:
                selected_count = max(1, int(np.ceil(len(test_idx) * budget_fraction)))
                selected = np.zeros(len(test_idx), dtype=bool)
                selected[score_order[:selected_count]] = True
                selected_sys = y_test[selected]
                max_sys_retained = float(np.max(selected_sys)) if len(selected_sys) else None
                best_sys_retained = bool(np.any(selected & (y_test == test_best_sys)))
                for target_name, target_mask in test_sys_targets.items():
                    target_total = int(np.sum(target_mask))
                    hits = int(np.sum(selected & target_mask))
                    rows.append(
                        {
                            "feature_set": source,
                            "model": model_name,
                            "score_fit_label": "top_10_percent",
                            "score_fit_sys_threshold": threshold,
                            "score_fit_target_threshold_scope": (
                                "full_retained_table_including_holdout_rows"
                            ),
                            "test_rows": int(len(test_idx)),
                            "budget_fraction": budget_fraction,
                            "selected_rows": selected_count,
                            "cost_reduction": 1.0 / budget_fraction,
                            "target": target_name,
                            "target_rows": target_total,
                            "hits": hits,
                            "recall": hits / target_total if target_total else 0.0,
                            "max_sys_retained": max_sys_retained,
                            "test_best_sys": test_best_sys,
                            "best_sys_retained": best_sys_retained,
                            "best_sys_score_rank": best_sys_rank,
                            "guard": (
                                "retained_table_budget_sanity_only_not_extreme_tail_"
                                "or_generated_candidate_validation"
                            ),
                        }
                    )
    rows.sort(
        key=lambda row: (
            float(row["budget_fraction"]),
            str(row["target"]),
            -float(row["recall"]),
            -float(row["max_sys_retained"] or 0.0),
            str(row["feature_set"]),
            str(row["model"]),
        )
    )
    return rows


def control_arrays_for_set(
    *,
    control_set: str,
    feature: str,
    feature_names: list[str],
    scored_features: list[tuple[float, str]],
    values_by_feature: dict[str, np.ndarray],
    filter_values: dict[str, np.ndarray],
) -> list[tuple[str, np.ndarray]]:
    if control_set == "source_facet_provenance_controls":
        return [
            (name, values)
            for name, values in sorted(filter_values.items())
            if name.startswith("control:")
        ]

    feature_family_name = filter_family(feature)
    combinatorial_names = [
        name
        for name in feature_names
        if name != feature
        and filter_family(name) == "combinatorial_counts"
    ]
    if control_set == "strongest_combinatorial_controls":
        candidates = combinatorial_names
    elif control_set == "strongest_other_family_prepared_controls":
        candidates = [
            name
            for name in feature_names
            if name != feature and filter_family(name) != feature_family_name
        ]
    elif control_set == "strongest_nonself_prepared_controls":
        candidates = [name for name in feature_names if name != feature]
    elif control_set == "source_facet_provenance_plus_strongest_combinatorial":
        control_rows = control_arrays_for_set(
            control_set="source_facet_provenance_controls",
            feature=feature,
            feature_names=feature_names,
            scored_features=scored_features,
            values_by_feature=values_by_feature,
            filter_values=filter_values,
        )
        combinatorial_rows = control_arrays_for_set(
            control_set="strongest_combinatorial_controls",
            feature=feature,
            feature_names=feature_names,
            scored_features=scored_features,
            values_by_feature=values_by_feature,
            filter_values=filter_values,
        )
        return [*control_rows, *combinatorial_rows]
    else:
        raise ValueError(f"unknown control set: {control_set}")

    candidate_set = set(candidates)
    return [
        (name, values_by_feature[name])
        for _, name in scored_features
        if name in candidate_set
    ][:8]


def feature_attribution_redundancy_rows(
    *,
    rows: list[dict[str, object]],
    provenance_rows: list[dict[str, object]],
    feature_names: list[str],
    filter_values: dict[str, np.ndarray],
    min_scope_rows: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in rows], dtype=float)
    train = stable_train_mask(rows, salt="tail-rule-feature-attribution-v1")
    values_by_feature = {feature: safe_feature_values(rows, feature) for feature in feature_names}
    result_rows: list[dict[str, object]] = []
    control_sets = [
        "source_facet_provenance_controls",
        "strongest_combinatorial_controls",
        "source_facet_provenance_plus_strongest_combinatorial",
        "strongest_other_family_prepared_controls",
        "strongest_nonself_prepared_controls",
    ]
    for scope in scope_masks(rows, provenance_rows):
        if str(scope["scope_type"]) not in ATTRIBUTION_SCOPE_TYPES:
            continue
        scope_mask = np.array(scope["mask"], dtype=bool)
        if int(np.sum(scope_mask)) < min_scope_rows:
            continue
        scope_train = scope_mask & train
        scope_test = scope_mask & ~train
        scope_train_rows = int(np.sum(scope_train))
        scope_test_rows = int(np.sum(scope_test))
        if scope_train_rows < 20 or scope_test_rows < 20:
            continue
        scored_features = []
        for candidate in feature_names:
            rho = spearman_rank_correlation(
                values_by_feature[candidate][scope_train], sys_values[scope_train]
            )
            if rho is not None:
                scored_features.append((abs(rho), candidate))
        scored_features.sort(reverse=True)
        targets: list[tuple[str, str, float, np.ndarray]] = []
        for label_scope, threshold_values in [
            ("global", sys_values),
            ("scope_local", sys_values[scope_mask]),
        ]:
            for label_name, quantile in TAIL_CUTOFFS.items():
                threshold = float(np.quantile(threshold_values, quantile))
                target = sys_values >= threshold
                if int(np.sum(target & scope_mask)) > 0:
                    targets.append((label_scope, label_name, threshold, target))
        for feature in feature_names:
            raw_values = values_by_feature[feature]
            variants: list[tuple[str, str, list[str], np.ndarray]] = [
                ("raw", "none", [], raw_values)
            ]
            for control_set in control_sets:
                controls = control_arrays_for_set(
                    control_set=control_set,
                    feature=feature,
                    feature_names=feature_names,
                    scored_features=scored_features,
                    values_by_feature=values_by_feature,
                    filter_values=filter_values,
                )
                if controls:
                    control_names = [name for name, _ in controls]
                    control_values = [values for _, values in controls]
                    residual_values = residualize(raw_values, control_values, scope_train)
                    variants.append((f"residual_after_{control_set}", control_set, control_names, residual_values))
            for feature_variant, control_set, control_names, values in variants:
                finite_train = np.isfinite(values) & scope_train
                finite_test = np.isfinite(values) & scope_test
                if int(np.sum(finite_train)) < 10 or int(np.sum(finite_test)) < 10:
                    continue
                rho = spearman_rank_correlation(
                    values[finite_train], sys_values[finite_train]
                )
                if rho is None:
                    continue
                for selection_fraction in FILTER_SELECTION_FRACTIONS:
                    selection = tail_selection_from_rho(
                        values=values,
                        fit_mask=scope_train,
                        selection_fraction=selection_fraction,
                        rho=rho,
                    )
                    if selection is None:
                        continue
                    selected, direction, rule_type, feature_threshold = selection
                    for label_scope, label_name, threshold, target in targets:
                        metrics = rule_metrics(target[finite_test], selected[finite_test])
                        result_rows.append(
                            {
                                **{key: value for key, value in scope.items() if key != "mask"},
                                "label_scope": label_scope,
                                "tail_label": label_name,
                                "sys_threshold": threshold,
                                "target_threshold_scope": "full_retained_scope_including_eval_rows",
                                "feature": feature,
                                "feature_family": filter_feature_family(feature),
                                "feature_variant": feature_variant,
                                "control_set": control_set,
                                "controls": ",".join(control_names),
                                "control_count": len(control_names),
                                "requested_selection_fraction": selection_fraction,
                                "actual_eval_selection_fraction": (
                                    metrics["selected"] / metrics["rows"]
                                    if metrics["rows"]
                                    else 0.0
                                ),
                                "train_rows": scope_train_rows,
                                "test_rows": scope_test_rows,
                                "guard": (
                                    "feature_variant_and_threshold_fit_on_train_"
                                    "evaluated_on_disjoint_poly_id_hash_split; "
                                    "target_threshold_is_full_retained_scope_quantile"
                                ),
                                "direction": direction,
                                "rule_type": rule_type,
                                "feature_threshold": feature_threshold,
                                "spearman_fit": rho,
                                **metrics,
                            }
                        )
    result_rows.sort(
        key=lambda row: (
            str(row["scope"]),
            str(row["label_scope"]),
            str(row["tail_label"]),
            str(row["feature"]),
            str(row["feature_variant"]),
            float(row["requested_selection_fraction"]),
        )
    )
    return result_rows


INTERPRETABLE_FEATURES = {
    "ridge_symp_area_sum_over_volume_sqrt": (
        "sum over cyclically ordered primal two-faces R of "
        "0.5 * |sum_i omega0(v_i, v_{i+1})| / sqrt(volume)"
    ),
    "ridge_symp_area_mean_over_volume_sqrt": (
        "mean over cyclically ordered primal two-faces R of "
        "0.5 * |sum_i omega0(v_i, v_{i+1})| / sqrt(volume)"
    ),
    "ridge_symp_area_le_1em2_over_volume_sqrt_fraction": (
        "fraction of cyclically ordered primal two-faces with "
        "symplectic area / sqrt(volume) at most 1e-2"
    ),
    "ridge_symp_area_effective_face_count": (
        "exp(entropy) of the normalized ridge symplectic-area mass distribution"
    ),
    "ridge_symp_area_normalized_entropy": (
        "entropy of the normalized ridge symplectic-area mass distribution "
        "divided by log(number of ordered two-faces)"
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
                        "target_threshold_scope": "capacity_source_facet_count_bucket",
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
    sys_threshold: float,
    target_threshold_scope: str,
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
                "sys_threshold": sys_threshold,
                "target_threshold_scope": target_threshold_scope,
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
        "target_threshold_scope": "full_retained_table_including_holdout_rows",
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
            sys_threshold=threshold,
            target_threshold_scope="full_retained_table_including_holdout_rows",
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
                            "sys_threshold": threshold,
                            "target_threshold_scope": (
                                "full_retained_table_including_holdout_rows"
                            ),
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
                                    "sys_threshold": threshold,
                                    "target_threshold_scope": (
                                        "full_retained_table_including_holdout_rows"
                                    ),
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
        ("all_invariant_features", "ridge_symp_area_only"),
        ("all_invariant_features", "combinatorial_invariants_only"),
        ("all_invariant_features", "strata_only"),
        ("all_invariant_features", "generator_provenance_only"),
        ("ridge_symp_area_only", "combinatorial_invariants_only"),
        ("ridge_symp_area_only", "strata_only"),
        ("ridge_symp_area_only", "generator_provenance_only"),
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
    if permutations == 0:
        return {
            "status": "not_run",
            "permutations": 0,
        }
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
                "status": "run",
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
        "sys_threshold",
        "target_threshold_scope",
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


def run(args: argparse.Namespace) -> None:
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    if args.max_rows is not None:
        rows = rows[: args.max_rows]
        kept_poly_ids = {str(row["poly_id"]) for row in rows}
        provenance_rows = [
            row for row in provenance_rows if str(row.get("poly_id", "")) in kept_poly_ids
        ]
    preflight = input_preflight(rows, provenance_rows)
    all_numeric_names = active_invariant_numeric_feature_names(rows)
    if args.max_filter_features is not None:
        required_first = [
            name for name in sorted(CURRENT_SCHEMA_REQUIRED_FEATURES) if name in all_numeric_names
        ]
        remaining = [name for name in all_numeric_names if name not in set(required_first)]
        all_numeric_names = (required_first + remaining)[: args.max_filter_features]
    filter_values = filter_value_arrays(rows, provenance_rows, all_numeric_names)
    family_names = {
        family: [name for name in all_numeric_names if feature_family(name) == family]
        for family in [
            "ridge_symp_area",
            "combinatorial_counts",
        ]
    }
    x_all_invariant = matrix_for_names(rows, all_numeric_names)
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
    train_idx, test_idx = next(splitter.split(x_all_invariant, y, groups))
    min_samples_leaf = max(10, int(len(train_idx) * args.min_leaf_fraction))
    labels = {
        label_name: float(np.quantile(y, quantile))
        for label_name, quantile in TAIL_CUTOFFS.items()
    }

    tree_filter_summaries: dict[str, dict[str, object]] = {}
    tree_filter_leaves: list[dict[str, object]] = []
    x_by_source = {
        "all_invariant_features": x_all_invariant,
        "ridge_symp_area_only": matrix_for_names(rows, family_names["ridge_symp_area"]),
        "combinatorial_invariants_only": matrix_for_names(
            rows, family_names["combinatorial_counts"]
        ),
        "strata_only": x_strata,
        "generator_provenance_only": x_generator_provenance,
    }
    predictive_x_by_source = {
        **x_by_source,
        "strata_plus_ridge_symp_area": np.column_stack(
            [x_strata, x_by_source["ridge_symp_area_only"]]
        ),
        "strata_plus_combinatorial_invariants": np.column_stack(
            [
                x_strata,
                x_by_source["combinatorial_invariants_only"],
            ]
        ),
        "strata_plus_all_invariant_features": np.column_stack(
            [
                x_strata,
                x_by_source["all_invariant_features"],
            ]
        ),
        "generator_provenance_plus_combinatorial_invariants": np.column_stack(
            [
                x_generator_provenance,
                x_by_source["combinatorial_invariants_only"],
            ]
        ),
        "generator_provenance_plus_all_invariant_features": np.column_stack(
            [
                x_generator_provenance,
                x_by_source["all_invariant_features"],
            ]
        ),
    }
    feature_names_by_source = {
        "all_invariant_features": all_numeric_names,
        "ridge_symp_area_only": family_names["ridge_symp_area"],
        "combinatorial_invariants_only": family_names["combinatorial_counts"],
        "strata_only": strata_names,
        "generator_provenance_only": generator_provenance_names,
    }
    for label_name, threshold in labels.items():
        for source, x in x_by_source.items():
            names = feature_names_by_source[source]
            tree_summary, leaves = fit_and_evaluate(
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
            tree_filter_summaries[f"{label_name}:{source}"] = tree_summary
            tree_filter_leaves.extend(leaves)

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
    single_filter_rows, single_filter_holdout_rows, single_filter_top_rows = single_feature_filter_rows(
        rows=rows,
        provenance_rows=provenance_rows,
        feature_values=filter_values,
        min_scope_rows=args.min_bucket_rows,
    )
    predictive_power_rows = feature_set_predictive_power_rows(
        x_by_source=predictive_x_by_source,
        y=y,
        labels=labels,
        train_idx=train_idx,
        test_idx=test_idx,
        random_state=args.random_state,
        max_depth=args.max_depth,
        min_samples_leaf=min_samples_leaf,
    )
    budget_sanity_rows = retained_table_budget_sanity_rows(
        x_by_source=predictive_x_by_source,
        y=y,
        train_idx=train_idx,
        test_idx=test_idx,
        random_state=args.random_state,
        max_depth=args.max_depth,
        min_samples_leaf=min_samples_leaf,
    )
    attribution_rows = feature_attribution_redundancy_rows(
        rows=rows,
        provenance_rows=provenance_rows,
        feature_names=all_numeric_names,
        filter_values=filter_values,
        min_scope_rows=args.min_bucket_rows,
    )
    tree_filter_leaves.sort(
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
        "input_preflight": preflight,
        "active_invariant_feature_count": len(all_numeric_names),
        "cheap_filter_feature_count_including_controls": len(filter_values),
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
        "tree_filter_methods": tree_filter_summaries,
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
        "single_feature_filter_grid": {
            "tail_cutoffs": TAIL_CUTOFFS,
            "requested_selection_fractions": FILTER_SELECTION_FRACTIONS,
            "descriptive_rows": len(single_filter_rows),
            "holdout_rows": len(single_filter_holdout_rows),
            "guard": (
                "single-feature-filter-leaderboard.tsv is descriptive only; "
                "single-feature-filter-holdout-rules.tsv freezes direction and threshold on a "
                "poly_id hash train split and evaluates on the disjoint split"
            ),
        },
        "top_single_feature_filter_holdout_rows": single_filter_top_rows[:40],
        "feature_set_predictive_power_rows": len(predictive_power_rows),
        "retained_table_budget_sanity_rows": len(budget_sanity_rows),
        "search_budget_fractions": SEARCH_BUDGET_FRACTIONS,
        "search_target_counts": SEARCH_TARGET_COUNTS,
        "search_target_fractions": SEARCH_TARGET_FRACTIONS,
        "feature_attribution_redundancy_rows": len(attribution_rows),
        "feature_attribution_scope_types": sorted(ATTRIBUTION_SCOPE_TYPES),
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
            "inside an already evaluated table; this packet does not estimate "
            "top-1e-6 feature-tail sys values or sys>1 hit probability"
        ),
    }
    write_json(args.out_dir / "summary.json", summary)
    write_leaf_table(args.out_dir / "tree-filter-leaves.tsv", tree_filter_leaves)
    write_tsv(args.out_dir / "stability-runs.tsv", stability_rows)
    write_tsv(args.out_dir / "stability-split-features.tsv", stability_feature_rows)
    write_tsv(args.out_dir / "bucket-interpretation-diagnostics.tsv", bucket_diagnostic_rows)
    write_tsv(args.out_dir / "single-feature-filter-leaderboard.tsv", single_filter_rows)
    write_tsv(args.out_dir / "single-feature-filter-holdout-rules.tsv", single_filter_holdout_rows)
    write_tsv(args.out_dir / "single-feature-filter-holdout-top-by-scope-label.tsv", single_filter_top_rows)
    write_tsv(args.out_dir / "feature-set-predictive-power.tsv", predictive_power_rows)
    write_tsv(args.out_dir / "retained-table-budget-sanity.tsv", budget_sanity_rows)
    write_tsv(args.out_dir / "feature-attribution-redundancy.tsv", attribution_rows)

    print("# tail-rule-mining")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- input preflight: `{preflight['status']}`")
    print(f"- active invariant features: `{len(all_numeric_names)}`")
    for family, names in sorted(family_names.items()):
        print(f"- {family} features: `{len(names)}`")
    print(f"- strata features: `{len(strata_names)}`")
    print(f"- generator provenance features: `{len(generator_provenance_names)}`")
    print(f"- permutations: `{args.permutations}`")
    for key, result in tree_filter_summaries.items():
        metrics = result["test_metrics"]
        print(
            f"- {key}: precision=`{metrics['precision']}`, recall=`{metrics['recall']}`, "
            f"enrichment=`{metrics['enrichment']}`, selected=`{metrics['selected']}`"
        )
    print(f"- stability configurations: `{len(stability_rows)}`")
    print(f"- single-feature descriptive filters: `{len(single_filter_rows)}`")
    print(f"- single-feature holdout filters: `{len(single_filter_holdout_rows)}`")
    print(f"- feature-set predictive rows: `{len(predictive_power_rows)}`")
    print(f"- retained-table budget sanity rows: `{len(budget_sanity_rows)}`")
    print(f"- feature attribution/redundancy rows: `{len(attribution_rows)}`")
    print(f"Wrote `{args.out_dir}`")


def main(argv: list[str] | None = None) -> None:
    run(parse_args(argv))


if __name__ == "__main__":
    main()
