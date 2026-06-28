#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Bucket-local Euclidean controls for the two-face symplectic-area pattern."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path
import sys

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import TABLES_DIR, load_trusted_random_tables, write_json  # noqa: E402


FEATURES = {
    "symp_area_sum": "ridge_symp_area_sum",
    "symp_area_mean": "ridge_symp_area_mean",
    "euclidean_area_sum": "ridge_euclidean_area_sum",
    "euclidean_area_mean": "ridge_euclidean_area_mean",
    "symp_over_euclidean_mean": "ridge_symp_over_euclidean_area_mean",
    "symp_over_euclidean_median": "ridge_symp_over_euclidean_area_median",
    "omega_spectral_norm": "omega_matrix_spectral_norm",
    "facet_volume_sum": "facet_volume_sum",
    "pairwise_distance_mean": "geom_pairwise_dist_mean",
}

LABELS = {
    "top_decile": 0.9,
    "top_five_percent": 0.95,
    "top_one_percent": 0.99,
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "two-face-control-artifacts")
    parser.add_argument("--min-bucket-rows", type=int, default=100)
    parser.add_argument("--tail-fraction", type=float, default=0.15)
    parser.add_argument("--euclidean-bands", type=int, default=10)
    return parser.parse_args()


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fields = sorted({field for row in rows for field in row})
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: row.get(field) for field in fields})


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
        ranked[order[index:next_index]] = (index + next_index - 1) / 2.0 + 1.0
        index = next_index
    return ranked


def spearman(left: np.ndarray, right: np.ndarray) -> float | None:
    left_ranks = ranks(left)
    right_ranks = ranks(right)
    left_std = float(np.std(left_ranks))
    right_std = float(np.std(right_ranks))
    if left_std == 0.0 or right_std == 0.0:
        return None
    return float(
        np.mean(
            (left_ranks - np.mean(left_ranks)) * (right_ranks - np.mean(right_ranks))
        )
        / (left_std * right_std)
    )


def residual(values: np.ndarray, controls: list[np.ndarray]) -> tuple[np.ndarray, float]:
    columns = [np.ones(len(values))]
    for control in controls:
        std = float(np.std(control))
        columns.append((control - np.mean(control)) / std if std else control * 0.0)
    design = np.column_stack(columns)
    coefficients = np.linalg.lstsq(design, values, rcond=None)[0]
    fitted = design @ coefficients
    residual_values = values - fitted
    variance = float(np.var(values))
    r_squared = 1.0 - float(np.var(residual_values)) / variance if variance else 0.0
    return residual_values, r_squared


def select_tail(
    values: np.ndarray, sys_values: np.ndarray, tail_fraction: float
) -> tuple[np.ndarray, str, float | None]:
    rho = spearman(values, sys_values)
    if rho is None:
        return np.zeros(len(values), dtype=bool), "constant", None
    if rho >= 0.0:
        return (
            values >= np.quantile(values, 1.0 - tail_fraction),
            "highest",
            rho,
        )
    return values <= np.quantile(values, tail_fraction), "lowest", rho


def select_within_euclidean_bands(
    *,
    values: np.ndarray,
    euclidean_values: np.ndarray,
    sys_values: np.ndarray,
    band_count: int,
    tail_fraction: float,
) -> tuple[np.ndarray, str, float | None]:
    rho = spearman(values, sys_values)
    selected = np.zeros(len(values), dtype=bool)
    quantiles = np.quantile(euclidean_values, np.linspace(0.0, 1.0, band_count + 1))
    for band_index in range(band_count):
        if band_index == band_count - 1:
            in_band = (euclidean_values >= quantiles[band_index]) & (
                euclidean_values <= quantiles[band_index + 1]
            )
        else:
            in_band = (euclidean_values >= quantiles[band_index]) & (
                euclidean_values < quantiles[band_index + 1]
            )
        band_indices = np.where(in_band)[0]
        if len(band_indices) == 0:
            continue
        select_count = max(1, int(round(len(band_indices) * tail_fraction)))
        if rho is not None and rho >= 0.0:
            ordered = band_indices[np.argsort(values[band_indices], kind="mergesort")]
            chosen = ordered[-select_count:]
            tail = "highest_within_euclidean_bands"
        else:
            ordered = band_indices[np.argsort(values[band_indices], kind="mergesort")]
            chosen = ordered[:select_count]
            tail = "lowest_within_euclidean_bands"
        selected[chosen] = True
    return selected, tail, rho


def metrics(target: np.ndarray, selected: np.ndarray) -> dict[str, float | int]:
    hits = int(np.sum(target & selected))
    selected_count = int(np.sum(selected))
    positives = int(np.sum(target))
    base_rate = float(np.mean(target)) if len(target) else 0.0
    precision = hits / selected_count if selected_count else 0.0
    recall = hits / positives if positives else 0.0
    return {
        "hits": hits,
        "selected": selected_count,
        "positives": positives,
        "base_rate": base_rate,
        "precision": precision,
        "recall": recall,
        "enrichment": precision / base_rate if base_rate else 0.0,
    }


def control_table_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
    tail_fraction: float,
    euclidean_bands: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    values = {
        name: np.array([float(row[field]) for row in bucket_rows], dtype=float)
        for name, field in FEATURES.items()
    }
    symp_resid_euclidean, symp_r2_euclidean = residual(
        values["symp_area_sum"], [values["euclidean_area_sum"]]
    )
    symp_resid_roundness, symp_r2_roundness = residual(
        values["symp_area_sum"],
        [
            values["euclidean_area_sum"],
            values["facet_volume_sum"],
            values["pairwise_distance_mean"],
        ],
    )
    omega_resid_euclidean, omega_r2_euclidean = residual(
        values["omega_spectral_norm"], [values["euclidean_area_sum"]]
    )
    feature_specs = [
        ("raw_euclidean_area_sum", values["euclidean_area_sum"], None),
        ("raw_symplectic_area_sum", values["symp_area_sum"], None),
        ("raw_symplectic_over_euclidean_mean", values["symp_over_euclidean_mean"], None),
        ("raw_omega_spectral_norm", values["omega_spectral_norm"], None),
        (
            "symplectic_area_residual_after_euclidean_area",
            symp_resid_euclidean,
            symp_r2_euclidean,
        ),
        (
            "symplectic_area_residual_after_euclidean_area_facet_volume_pairwise_distance",
            symp_resid_roundness,
            symp_r2_roundness,
        ),
        (
            "omega_spectral_norm_residual_after_euclidean_area",
            omega_resid_euclidean,
            omega_r2_euclidean,
        ),
    ]
    rows: list[dict[str, object]] = []
    for label, quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, quantile)
        for feature_name, feature_values, residual_r2 in feature_specs:
            selected, tail, rho = select_tail(feature_values, sys_values, tail_fraction)
            rows.append(
                {
                    "capacity_source": capacity_source,
                    "facet_count": facet_count,
                    "rows": len(bucket_rows),
                    "label": label,
                    "sys_threshold": float(np.quantile(sys_values, quantile)),
                    "rule": feature_name,
                    "selection": f"{tail}_{tail_fraction:g}",
                    "spearman_with_sys": rho,
                    "residual_model_r_squared": residual_r2,
                    "target_feature_mean": float(np.mean(feature_values[target]))
                    if np.any(target)
                    else None,
                    "rest_feature_mean": float(np.mean(feature_values[~target]))
                    if np.any(~target)
                    else None,
                    **metrics(target, selected),
                }
            )
        selected, tail, rho = select_within_euclidean_bands(
            values=values["symp_area_sum"],
            euclidean_values=values["euclidean_area_sum"],
            sys_values=sys_values,
            band_count=euclidean_bands,
            tail_fraction=tail_fraction,
        )
        rows.append(
            {
                "capacity_source": capacity_source,
                "facet_count": facet_count,
                "rows": len(bucket_rows),
                "label": label,
                "sys_threshold": float(np.quantile(sys_values, quantile)),
                "rule": "symplectic_area_sum_matched_within_euclidean_area_bands",
                "selection": tail,
                "spearman_with_sys": rho,
                "residual_model_r_squared": None,
                "target_feature_mean": float(np.mean(values["symp_area_sum"][target]))
                if np.any(target)
                else None,
                "rest_feature_mean": float(np.mean(values["symp_area_sum"][~target]))
                if np.any(~target)
                else None,
                **metrics(target, selected),
            }
        )
    return rows


def euclidean_decile_table_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
    band_count: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    euclidean_values = np.array(
        [float(row[FEATURES["euclidean_area_sum"]]) for row in bucket_rows], dtype=float
    )
    symplectic_values = np.array(
        [float(row[FEATURES["symp_area_sum"]]) for row in bucket_rows], dtype=float
    )
    rows: list[dict[str, object]] = []
    quantiles = np.quantile(euclidean_values, np.linspace(0.0, 1.0, band_count + 1))
    for label, sys_quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, sys_quantile)
        for band_index in range(band_count):
            if band_index == band_count - 1:
                in_band = (euclidean_values >= quantiles[band_index]) & (
                    euclidean_values <= quantiles[band_index + 1]
                )
            else:
                in_band = (euclidean_values >= quantiles[band_index]) & (
                    euclidean_values < quantiles[band_index + 1]
                )
            if not np.any(in_band):
                continue
            band_target = target[in_band]
            rows.append(
                {
                    "capacity_source": capacity_source,
                    "facet_count": facet_count,
                    "label": label,
                    "euclidean_area_sum_decile": band_index + 1,
                    "rows": int(np.sum(in_band)),
                    "positives": int(np.sum(band_target)),
                    "base_rate": float(np.mean(target)),
                    "positive_rate": float(np.mean(band_target)),
                    "enrichment": float(np.mean(band_target)) / float(np.mean(target))
                    if np.mean(target)
                    else 0.0,
                    "euclidean_area_sum_min": float(np.min(euclidean_values[in_band])),
                    "euclidean_area_sum_max": float(np.max(euclidean_values[in_band])),
                    "symplectic_area_sum_mean": float(np.mean(symplectic_values[in_band])),
                    "sys_mean": float(np.mean(sys_values[in_band])),
                }
            )
    return rows


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    del provenance_rows
    bucket_keys = sorted(
        {
            (str(row.get("capacity_source", "missing")), int(row.get("facet_count", 0)))
            for row in rows
        }
    )
    control_rows: list[dict[str, object]] = []
    decile_rows: list[dict[str, object]] = []
    bucket_counts: list[dict[str, object]] = []
    for capacity_source, facet_count in bucket_keys:
        bucket_rows = [
            row
            for row in rows
            if str(row.get("capacity_source", "missing")) == capacity_source
            and int(row.get("facet_count", 0)) == facet_count
        ]
        if len(bucket_rows) < args.min_bucket_rows:
            continue
        bucket_counts.append(
            {
                "capacity_source": capacity_source,
                "facet_count": facet_count,
                "rows": len(bucket_rows),
                "max_sys": max(float(row["sys"]) for row in bucket_rows),
            }
        )
        control_rows.extend(
            control_table_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
                tail_fraction=args.tail_fraction,
                euclidean_bands=args.euclidean_bands,
            )
        )
        decile_rows.extend(
            euclidean_decile_table_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
                band_count=args.euclidean_bands,
            )
        )
    control_rows.sort(
        key=lambda row: (
            str(row["label"]),
            str(row["capacity_source"]),
            int(row["facet_count"]),
            -float(row["enrichment"]),
            str(row["rule"]),
        )
    )
    write_tsv(args.out_dir / "bucket-control-rules.tsv", control_rows)
    write_tsv(args.out_dir / "euclidean-area-deciles.tsv", decile_rows)
    write_json(
        args.out_dir / "summary.json",
        {
            "row_count": len(rows),
            "bucket_counts": bucket_counts,
            "labels": LABELS,
            "tail_fraction": args.tail_fraction,
            "euclidean_bands": args.euclidean_bands,
            "control_rule_count": len(control_rows),
            "euclidean_decile_row_count": len(decile_rows),
        },
    )
    print("# two-face Euclidean control diagnostics")
    print(f"- rows: `{len(rows)}`")
    print(f"- buckets: `{len(bucket_counts)}`")
    print(f"- control rule rows: `{len(control_rows)}`")
    print(f"- euclidean decile rows: `{len(decile_rows)}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
