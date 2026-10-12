#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Compare the known HKO reference row to retained random/product feature support."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys
from typing import Any

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    active_invariant_numeric_feature_names,
    dataset_label,
    load_reference_holdout_tables,
    load_trusted_random_tables,
    matrix_for,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--random-tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--reference-tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--top-count", type=int, default=10)
    return parser.parse_args()


def standardize(reference: np.ndarray, values: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    mean = np.mean(reference, axis=0)
    scale = np.std(reference, axis=0)
    scale[scale == 0.0] = 1.0
    return (reference - mean) / scale, (values - mean) / scale


def nearest_neighbor_distances(x: np.ndarray) -> np.ndarray:
    if len(x) < 2:
        return np.array([], dtype=float)
    out = np.empty(len(x), dtype=float)
    for index, row in enumerate(x):
        distances = np.linalg.norm(x - row, axis=1)
        distances[index] = np.inf
        out[index] = float(np.min(distances))
    return out


def rank_desc(value: float, values: np.ndarray) -> int:
    return int(np.sum(values > value) + 1)


def rank_asc(value: float, values: np.ndarray) -> int:
    return int(np.sum(values < value) + 1)


def require_finite_features(rows: list[dict[str, Any]], names: list[str], label: str) -> None:
    missing = []
    for row in rows:
        poly_id = str(row.get("poly_id", "<missing poly_id>"))
        for name in names:
            value = row.get(name)
            if not isinstance(value, int | float) or not np.isfinite(float(value)):
                missing.append(f"{poly_id}:{name}")
    if missing:
        preview = ", ".join(missing[:8])
        raise SystemExit(
            f"{label} rows are missing finite retained-support features; "
            f"{len(missing)} missing values, examples: {preview}"
        )


def row_preview(
    row: dict[str, Any],
    provenance: dict[str, list[dict[str, Any]]],
    distance: float | None = None,
) -> dict[str, Any]:
    poly_id = str(row["poly_id"])
    preview: dict[str, Any] = {
        "poly_id": poly_id,
        "sys": float(row["sys"]),
        "facet_count": int(row["facet_count"]),
        "dataset": dataset_label(row, provenance.get(poly_id, [])),
        "ridge_area_score": float(row["ridge_symp_area_sum_over_volume_sqrt"]),
    }
    if distance is not None:
        preview["standardized_distance"] = float(distance)
    return preview


def coverage_statement(percentile: float, min_distance: float, nn_q95: float, nn_q99: float) -> str:
    if percentile >= 0.99 or min_distance > nn_q99:
        return "outside retained random/product invariant-feature cloud"
    if percentile >= 0.95 or min_distance > nn_q95:
        return "near the retained random/product boundary"
    return "inside retained random/product invariant-feature cloud"


def write_markdown(path: Path, summary: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    hko = summary["hko"]
    support = summary["support"]
    distance = summary["distance"]
    ridge = summary["ridge_area_score"]
    with path.open("w") as handle:
        handle.write("# HKO reference coverage\n\n")
        handle.write(f"Conclusion: {summary['conclusion']}.\n\n")
        handle.write("## Inputs\n\n")
        handle.write(f"- random/product rows: `{support['trusted_random_rows']}`\n")
        handle.write(f"- reference rows: `{support['reference_rows']}`\n")
        handle.write(f"- active invariant features: `{support['feature_count']}`\n")
        handle.write(f"- HKO `sys`: `{hko['sys']}`\n")
        handle.write(f"- retained max `sys`: `{support['max_random_sys']}`\n\n")
        handle.write("## Distance\n\n")
        handle.write(f"- HKO nearest standardized distance: `{distance['hko_min_distance']}`\n")
        handle.write(
            f"- random/product nearest-neighbor percentile: `{distance['hko_nn_percentile']}`\n"
        )
        handle.write(f"- nearest-neighbor q95/q99: `{distance['nn_q95']}` / `{distance['nn_q99']}`\n")
        handle.write(f"- HKO distance rank among rows plus HKO: `{distance['hko_distance_rank']}`\n\n")
        handle.write("## Ridge Area\n\n")
        handle.write(f"- score: `{ridge['hko_score']}`\n")
        handle.write(f"- descending rank among rows plus HKO: `{ridge['rank_descending']}`\n")
        handle.write(f"- percentile among random/product rows: `{ridge['percentile']}`\n\n")
        handle.write("## Nearest Random/Product Rows\n\n")
        handle.write("| rank | dataset | facets | sys | distance | poly_id |\n")
        handle.write("| ---: | --- | ---: | ---: | ---: | --- |\n")
        for index, row in enumerate(summary["nearest_rows"], start=1):
            handle.write(
                f"| {index} | {row['dataset']} | {row['facet_count']} | {row['sys']} | "
                f"{row['standardized_distance']} | `{row['poly_id']}` |\n"
            )
        handle.write("\n## Current High-Tail Rows\n\n")
        handle.write("| rank | dataset | facets | sys | distance_to_hko | ridge_area_score | poly_id |\n")
        handle.write("| ---: | --- | ---: | ---: | ---: | ---: | --- |\n")
        for index, row in enumerate(summary["high_tail_rows"], start=1):
            handle.write(
                f"| {index} | {row['dataset']} | {row['facet_count']} | {row['sys']} | "
                f"{row['standardized_distance']} | {row['ridge_area_score']} | `{row['poly_id']}` |\n"
            )


def main() -> None:
    args = parse_args()
    random_rows, random_provenance_rows = load_trusted_random_tables(args.random_tables_dir)
    reference_rows, reference_provenance_rows = load_reference_holdout_tables(
        args.reference_tables_dir
    )
    if not random_rows:
        raise SystemExit("No trusted random/product rows found")
    if len(reference_rows) != 1:
        raise SystemExit(f"Expected exactly one reference holdout row, found {len(reference_rows)}")

    hko = reference_rows[0]
    feature_names = active_invariant_numeric_feature_names(random_rows, require_all=False)
    require_finite_features(reference_rows, feature_names, "reference")
    random_x = np.array(matrix_for(random_rows, feature_names), dtype=float)
    hko_x = np.array(matrix_for(reference_rows, feature_names), dtype=float)
    random_z, hko_z = standardize(random_x, hko_x)
    hko_distances = np.linalg.norm(random_z - hko_z[0], axis=1)
    nearest_order = np.argsort(hko_distances)
    random_nn = nearest_neighbor_distances(random_z)
    hko_min_distance = float(np.min(hko_distances))
    nn_percentile = float(np.mean(random_nn <= hko_min_distance)) if len(random_nn) else 1.0
    nn_q95 = float(np.quantile(random_nn, 0.95)) if len(random_nn) else float("nan")
    nn_q99 = float(np.quantile(random_nn, 0.99)) if len(random_nn) else float("nan")

    random_sys = np.array([float(row["sys"]) for row in random_rows], dtype=float)
    ridge_values = np.array(
        [float(row["ridge_symp_area_sum_over_volume_sqrt"]) for row in random_rows], dtype=float
    )
    hko_ridge = float(hko["ridge_symp_area_sum_over_volume_sqrt"])
    hko_sys = float(hko["sys"])

    random_provenance = provenance_by_poly_id(random_provenance_rows)
    reference_provenance = provenance_by_poly_id(reference_provenance_rows)
    top_count = min(args.top_count, len(random_rows))
    high_tail_order = np.argsort(random_sys)[-top_count:][::-1]
    nearest_rows = [
        row_preview(random_rows[int(index)], random_provenance, float(hko_distances[int(index)]))
        for index in nearest_order[:top_count]
    ]
    high_tail_rows = [
        row_preview(random_rows[int(index)], random_provenance, float(hko_distances[int(index)]))
        for index in high_tail_order
    ]
    hko_preview = row_preview(hko, reference_provenance)

    summary: dict[str, Any] = {
        "conclusion": coverage_statement(nn_percentile, hko_min_distance, nn_q95, nn_q99),
        "support": {
            "random_tables_dir": str(args.random_tables_dir),
            "reference_tables_dir": str(args.reference_tables_dir),
            "trusted_random_rows": len(random_rows),
            "reference_rows": len(reference_rows),
            "feature_count": len(feature_names),
            "features": feature_names,
            "max_random_sys": float(np.max(random_sys)),
            "random_sys_gt_one": int(np.sum(random_sys > 1.0)),
        },
        "hko": {
            **hko_preview,
            "sys_rank_descending_among_rows_plus_hko": rank_desc(hko_sys, random_sys),
        },
        "distance": {
            "hko_min_distance": hko_min_distance,
            "hko_mean_distance": float(np.mean(hko_distances)),
            "hko_nn_percentile": nn_percentile,
            "nn_q50": float(np.quantile(random_nn, 0.5)) if len(random_nn) else None,
            "nn_q90": float(np.quantile(random_nn, 0.9)) if len(random_nn) else None,
            "nn_q95": nn_q95,
            "nn_q99": nn_q99,
            "hko_distance_rank": rank_asc(hko_min_distance, random_nn),
        },
        "ridge_area_score": {
            "feature": "ridge_symp_area_sum_over_volume_sqrt",
            "hko_score": hko_ridge,
            "random_min": float(np.min(ridge_values)),
            "random_median": float(np.median(ridge_values)),
            "random_max": float(np.max(ridge_values)),
            "rank_descending": rank_desc(hko_ridge, ridge_values),
            "percentile": float(np.mean(ridge_values <= hko_ridge)),
        },
        "nearest_rows": nearest_rows,
        "high_tail_rows": high_tail_rows,
    }

    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_json(args.out_dir / "summary.json", summary)
    write_markdown(args.out_dir / "report.md", summary)
    print("# hko-reference-coverage")
    print(f"- conclusion: {summary['conclusion']}")
    print(f"- trusted random/product rows: `{len(random_rows)}`")
    print(f"- HKO nearest standardized distance: `{hko_min_distance}`")
    print(f"- HKO NN percentile: `{nn_percentile}`")
    print(f"- retained max sys: `{float(np.max(random_sys))}`; HKO sys: `{hko_sys}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
