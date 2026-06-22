#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""Random-only held-out prediction and candidate-ranking checks."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

import numpy as np
from sklearn.ensemble import RandomForestRegressor
from sklearn.linear_model import Ridge
from sklearn.metrics import mean_absolute_error, r2_score
from sklearn.model_selection import GroupShuffleSplit
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import StandardScaler

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
    parser.add_argument("--max-features", type=int, default=None)
    parser.add_argument("--forest-trees", type=int, default=80)
    parser.add_argument("--permutations", type=int, default=10)
    return parser.parse_args()


def top_decile_enrichment(y_true: np.ndarray, y_score: np.ndarray) -> float:
    cutoff = np.quantile(y_score, 0.9)
    selected = y_true[y_score >= cutoff]
    if len(selected) == 0:
        return 0.0
    return float(np.mean(selected >= np.quantile(y_true, 0.9)))


def evaluate_model(model, x_train, y_train, x_test, y_test) -> dict[str, float | None]:
    model.fit(x_train, y_train)
    pred = model.predict(x_test)
    return {
        "r2": float(r2_score(y_test, pred)) if len(y_test) >= 2 else None,
        "mae": float(mean_absolute_error(y_test, pred)),
        "top_decile_enrichment": top_decile_enrichment(y_test, pred),
        "max_predicted_row_sys": float(np.max(y_test[np.argsort(pred)[-25:]])),
    }


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


def metadata_feature_rows(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> tuple[list[dict[str, str]], list[str]]:
    provenance = provenance_by_poly_id(provenance_rows)
    fields = [
        "capacity_source",
        "dataset_label",
        "facet_count",
        "product_bucket",
        "product_bounces",
        "sample_height_range",
    ]
    metadata_rows: list[dict[str, str]] = []
    for row in rows:
        provenance_for_row = provenance.get(str(row["poly_id"]), [])
        is_product = row.get("capacity_source") == "random_product_sample"
        metadata_rows.append(
            {
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
        )
    return metadata_rows, fields


def one_hot_matrix(
    metadata_rows: list[dict[str, str]], fields: list[str]
) -> tuple[np.ndarray, list[str]]:
    feature_names: list[str] = []
    columns: list[np.ndarray] = []
    for field in fields:
        values = sorted({row[field] for row in metadata_rows})
        for value in values:
            feature_names.append(f"{field}={value}")
            columns.append(
                np.array([1.0 if row[field] == value else 0.0 for row in metadata_rows])
            )
    if not columns:
        return np.zeros((len(metadata_rows), 0), dtype=float), feature_names
    return np.column_stack(columns), feature_names


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    names = numeric_feature_names(rows, geometry_only=True)
    if args.max_features is not None:
        names = names[: args.max_features]
    x = np.array(matrix_for(rows, names), dtype=float)
    metadata_rows, metadata_fields = metadata_feature_rows(rows, provenance_rows)
    x_metadata, metadata_feature_names = one_hot_matrix(metadata_rows, metadata_fields)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    groups = np.array(
        [
            str(row.get("capacity_source", "")) + ":" + str(row.get("facet_count", ""))
            for row in rows
        ]
    )

    splitter = GroupShuffleSplit(n_splits=1, test_size=0.25, random_state=20260621)
    train_idx, test_idx = next(splitter.split(x, y, groups))
    x_train, x_test = x[train_idx], x[test_idx]
    x_metadata_train, x_metadata_test = x_metadata[train_idx], x_metadata[test_idx]
    y_train, y_test = y[train_idx], y[test_idx]

    models = {
        "ridge_geometry_only": make_pipeline(StandardScaler(), Ridge(alpha=1.0)),
        "random_forest_geometry_only": RandomForestRegressor(
            n_estimators=args.forest_trees,
            min_samples_leaf=10,
            random_state=20260621,
            n_jobs=-1,
        ),
        "ridge_metadata_only": make_pipeline(StandardScaler(), Ridge(alpha=1.0)),
        "random_forest_metadata_only": RandomForestRegressor(
            n_estimators=args.forest_trees,
            min_samples_leaf=10,
            random_state=20260622,
            n_jobs=-1,
        ),
    }
    results = {}
    for name, model in models.items():
        if name.endswith("_metadata_only"):
            results[name] = evaluate_model(
                model, x_metadata_train, y_train, x_metadata_test, y_test
            )
        else:
            results[name] = evaluate_model(model, x_train, y_train, x_test, y_test)

    rng = np.random.default_rng(20260621)
    null_enrichment = []
    for _ in range(args.permutations):
        shuffled = y_train.copy()
        rng.shuffle(shuffled)
        model = RandomForestRegressor(
            n_estimators=max(20, args.forest_trees // 2),
            min_samples_leaf=10,
            random_state=int(rng.integers(0, 2**31 - 1)),
            n_jobs=-1,
        )
        model.fit(x_train, shuffled)
        pred = model.predict(x_test)
        null_enrichment.append(top_decile_enrichment(y_test, pred))
    observed = results["random_forest_geometry_only"]["top_decile_enrichment"]
    null_p = (sum(1 for value in null_enrichment if value >= observed) + 1) / (
        len(null_enrichment) + 1
    )

    summary = {
        "row_count": len(rows),
        "provenance_rows": len(provenance_rows),
        "geometry_feature_count": len(names),
        "metadata_fields": metadata_fields,
        "metadata_feature_count": len(metadata_feature_names),
        "metadata_feature_names": metadata_feature_names,
        "grouping": "capacity_source:facet_count",
        "train_rows": int(len(train_idx)),
        "test_rows": int(len(test_idx)),
        "forest_trees": args.forest_trees,
        "permutations": args.permutations,
        "results": results,
        "random_forest_top_decile_enrichment_permutation_p": float(null_p),
        "candidate_proposer_disposition": (
            "no validated candidate-proposer: held-out ranking is an in-table "
            "geometry-only signal and did not generate or validate new rows"
        ),
        "metadata_baseline_disposition": (
            "metadata-only models are leakage/source diagnostics, not geometry "
            "candidate-proposers; compare them to geometry-only models after "
            "retained-table reruns"
        ),
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# prediction-ranking")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- geometry features: `{len(names)}`")
    print(f"- metadata features: `{len(metadata_feature_names)}`")
    for name, result in results.items():
        print(
            f"- {name}: R2=`{result['r2']}`, MAE=`{result['mae']}`, "
            f"top-decile enrichment=`{result['top_decile_enrichment']}`"
        )
    print(f"- RF enrichment permutation p: `{null_p}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
