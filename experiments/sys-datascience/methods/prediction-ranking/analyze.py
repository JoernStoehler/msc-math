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
    load_trusted_random_tables,
    matrix_for,
    numeric_feature_names,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-features", type=int, default=80)
    parser.add_argument("--forest-trees", type=int, default=80)
    parser.add_argument("--permutations", type=int, default=10)
    return parser.parse_args()


def top_decile_enrichment(y_true: np.ndarray, y_score: np.ndarray) -> float:
    cutoff = np.quantile(y_score, 0.9)
    selected = y_true[y_score >= cutoff]
    if len(selected) == 0:
        return 0.0
    return float(np.mean(selected >= np.quantile(y_true, 0.9)))


def evaluate_model(model, x_train, y_train, x_test, y_test) -> dict[str, float]:
    model.fit(x_train, y_train)
    pred = model.predict(x_test)
    return {
        "r2": float(r2_score(y_test, pred)),
        "mae": float(mean_absolute_error(y_test, pred)),
        "top_decile_enrichment": top_decile_enrichment(y_test, pred),
        "max_predicted_row_sys": float(np.max(y_test[np.argsort(pred)[-25:]])),
    }


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    names = numeric_feature_names(rows, geometry_only=True)[: args.max_features]
    x = np.array(matrix_for(rows, names), dtype=float)
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
    y_train, y_test = y[train_idx], y[test_idx]

    models = {
        "ridge_geometry_only": make_pipeline(StandardScaler(), Ridge(alpha=1.0)),
        "random_forest_geometry_only": RandomForestRegressor(
            n_estimators=args.forest_trees,
            min_samples_leaf=10,
            random_state=20260621,
            n_jobs=-1,
        ),
    }
    results = {
        name: evaluate_model(model, x_train, y_train, x_test, y_test)
        for name, model in models.items()
    }

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
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# prediction-ranking")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- geometry features: `{len(names)}`")
    for name, result in results.items():
        print(
            f"- {name}: R2=`{result['r2']}`, MAE=`{result['mae']}`, "
            f"top-decile enrichment=`{result['top_decile_enrichment']}`"
        )
    print(f"- RF enrichment permutation p: `{null_p}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
