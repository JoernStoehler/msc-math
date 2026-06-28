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
    "symp_area_sum": "ridge_symp_area_volnorm_sum",
    "symp_area_mean": "ridge_symp_area_volnorm_mean",
    "euclidean_area_sum": "ridge_euclidean_area_volnorm_sum",
    "euclidean_area_mean": "ridge_euclidean_area_volnorm_mean",
    "symp_over_euclidean_mean": "ridge_symp_over_euclidean_area_mean",
    "symp_over_euclidean_median": "ridge_symp_over_euclidean_area_median",
    "omega_spectral_norm": "omega_matrix_vol1_spectral_norm",
    "facet_volume_sum": "facet_volume_vol1_sum",
    "pairwise_distance_mean": "geom_vol1_pairwise_dist_mean",
}

LABELS = {
    "top_decile": 0.9,
    "top_five_percent": 0.95,
    "top_one_percent": 0.99,
    "top_half_percent": 0.995,
    "top_quarter_percent": 0.9975,
}

SMALL_FACE_FRACTIONS = [0.1, 0.2, 0.3, 0.4]

PROFILE_FEATURES = {
    "sys": "sys",
    "euclidean_area_sum": FEATURES["euclidean_area_sum"],
    "symp_area_sum": FEATURES["symp_area_sum"],
    "symp_over_euclidean_mean": FEATURES["symp_over_euclidean_mean"],
    "omega_spectral_norm": FEATURES["omega_spectral_norm"],
    "facet_volume_sum": FEATURES["facet_volume_sum"],
    "pairwise_distance_mean": FEATURES["pairwise_distance_mean"],
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


def profile_values(
    values: dict[str, np.ndarray], mask: np.ndarray, prefix: str
) -> dict[str, float | int | None]:
    row: dict[str, float | int | None] = {f"{prefix}_rows": int(np.sum(mask))}
    for name, feature_values in values.items():
        if np.any(mask):
            row[f"{prefix}_{name}_mean"] = float(np.mean(feature_values[mask]))
            row[f"{prefix}_{name}_median"] = float(np.median(feature_values[mask]))
        else:
            row[f"{prefix}_{name}_mean"] = None
            row[f"{prefix}_{name}_median"] = None
    return row


def euclidean_decile_indices(euclidean_values: np.ndarray, band_count: int) -> np.ndarray:
    quantiles = np.quantile(euclidean_values, np.linspace(0.0, 1.0, band_count + 1))
    deciles = np.zeros(len(euclidean_values), dtype=int)
    for band_index in range(band_count):
        if band_index == band_count - 1:
            in_band = (euclidean_values >= quantiles[band_index]) & (
                euclidean_values <= quantiles[band_index + 1]
            )
        else:
            in_band = (euclidean_values >= quantiles[band_index]) & (
                euclidean_values < quantiles[band_index + 1]
            )
        deciles[in_band] = band_index + 1
    return deciles


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


def outside_small_face_audit_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    values = {
        name: np.array([float(row[field]) for row in bucket_rows], dtype=float)
        for name, field in PROFILE_FEATURES.items()
    }
    euclidean_values = values["euclidean_area_sum"]
    rows: list[dict[str, object]] = []
    for label, quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, quantile)
        for small_fraction in SMALL_FACE_FRACTIONS:
            small_face = euclidean_values <= np.quantile(euclidean_values, small_fraction)
            high_small = target & small_face
            high_outside = target & ~small_face
            small_not_high = small_face & ~target
            outside_not_high = ~small_face & ~target
            row: dict[str, object] = {
                "capacity_source": capacity_source,
                "facet_count": facet_count,
                "rows": len(bucket_rows),
                "label": label,
                "sys_threshold": float(np.quantile(sys_values, quantile)),
                "small_face_fraction": small_fraction,
                "small_face_rows": int(np.sum(small_face)),
                "positives": int(np.sum(target)),
                "positive_small_face_rows": int(np.sum(high_small)),
                "positive_outside_small_face_rows": int(np.sum(high_outside)),
                "positive_fraction_in_small_face": float(np.sum(high_small) / np.sum(target))
                if np.any(target)
                else 0.0,
                "positive_fraction_outside_small_face": float(
                    np.sum(high_outside) / np.sum(target)
                )
                if np.any(target)
                else 0.0,
                "small_face_positive_rate": float(np.mean(target[small_face]))
                if np.any(small_face)
                else 0.0,
                "outside_small_face_positive_rate": float(np.mean(target[~small_face]))
                if np.any(~small_face)
                else 0.0,
                "small_face_enrichment": float(np.mean(target[small_face]) / np.mean(target))
                if np.any(small_face) and np.mean(target)
                else 0.0,
                "outside_small_face_enrichment": float(
                    np.mean(target[~small_face]) / np.mean(target)
                )
                if np.any(~small_face) and np.mean(target)
                else 0.0,
            }
            row.update(profile_values(values, high_small, "positive_small_face"))
            row.update(profile_values(values, high_outside, "positive_outside_small_face"))
            row.update(profile_values(values, small_not_high, "negative_small_face"))
            row.update(profile_values(values, outside_not_high, "negative_outside_small_face"))
            rows.append(row)
    return rows


def conditional_omega_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
    tail_fraction: float,
    euclidean_bands: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    euclidean_values = np.array(
        [float(row[FEATURES["euclidean_area_sum"]]) for row in bucket_rows], dtype=float
    )
    omega_values = np.array(
        [float(row[FEATURES["omega_spectral_norm"]]) for row in bucket_rows], dtype=float
    )
    rows: list[dict[str, object]] = []
    for label, quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, quantile)
        selected, tail, rho = select_within_euclidean_bands(
            values=omega_values,
            euclidean_values=euclidean_values,
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
                "scope": "all_rows_matched_within_euclidean_bands",
                "rule": "omega_spectral_norm",
                "selection": tail,
                "spearman_with_sys": rho,
                **metrics(target, selected),
            }
        )
        for small_fraction in SMALL_FACE_FRACTIONS:
            small_face = euclidean_values <= np.quantile(euclidean_values, small_fraction)
            for scope, scope_mask in [
                (f"outside_lowest_{small_fraction:g}_euclidean_area", ~small_face),
                (f"inside_lowest_{small_fraction:g}_euclidean_area", small_face),
            ]:
                if int(np.sum(scope_mask)) < 20:
                    continue
                scoped_target = target[scope_mask]
                scoped_omega = omega_values[scope_mask]
                scoped_sys = sys_values[scope_mask]
                selected_scoped, tail_scoped, rho_scoped = select_tail(
                    scoped_omega, scoped_sys, tail_fraction
                )
                rows.append(
                    {
                        "capacity_source": capacity_source,
                        "facet_count": facet_count,
                        "rows": int(np.sum(scope_mask)),
                        "label": label,
                        "sys_threshold": float(np.quantile(sys_values, quantile)),
                        "scope": scope,
                        "rule": "omega_spectral_norm",
                        "selection": f"{tail_scoped}_{tail_fraction:g}",
                        "spearman_with_sys": rho_scoped,
                        **metrics(scoped_target, selected_scoped),
                    }
                )
    return rows


def matched_outside_small_face_omega_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
    euclidean_bands: int,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    euclidean_values = np.array(
        [float(row[FEATURES["euclidean_area_sum"]]) for row in bucket_rows], dtype=float
    )
    omega_values = np.array(
        [float(row[FEATURES["omega_spectral_norm"]]) for row in bucket_rows], dtype=float
    )
    deciles = euclidean_decile_indices(euclidean_values, euclidean_bands)
    rows: list[dict[str, object]] = []
    for label, quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, quantile)
        for small_fraction in SMALL_FACE_FRACTIONS:
            small_face = euclidean_values <= np.quantile(euclidean_values, small_fraction)
            outside_target = target & ~small_face
            outside_control_pool = ~target & ~small_face
            if not np.any(outside_target):
                continue
            matched_control = np.zeros(len(bucket_rows), dtype=bool)
            for decile in sorted(set(deciles[outside_target])):
                target_indices = np.where(outside_target & (deciles == decile))[0]
                available_controls = set(
                    int(index)
                    for index in np.where(outside_control_pool & (deciles == decile))[0]
                )
                if not available_controls:
                    continue
                for target_index in target_indices:
                    if not available_controls:
                        break
                    chosen = min(
                        available_controls,
                        key=lambda control_index: abs(
                            euclidean_values[control_index]
                            - euclidean_values[target_index]
                        ),
                    )
                    available_controls.remove(chosen)
                    matched_control[chosen] = True
            comparison_mask = outside_target | matched_control
            if not np.any(matched_control):
                continue
            selected = np.zeros(len(bucket_rows), dtype=bool)
            selected[comparison_mask] = omega_values[comparison_mask] <= np.median(
                omega_values[comparison_mask]
            )
            rows.append(
                {
                    "capacity_source": capacity_source,
                    "facet_count": facet_count,
                    "rows": int(np.sum(comparison_mask)),
                    "label": label,
                    "sys_threshold": float(np.quantile(sys_values, quantile)),
                    "small_face_fraction": small_fraction,
                    "outside_positive_rows": int(np.sum(outside_target)),
                    "matched_control_rows": int(np.sum(matched_control)),
                    "positive_omega_median": float(np.median(omega_values[outside_target])),
                    "matched_control_omega_median": float(
                        np.median(omega_values[matched_control])
                    ),
                    "positive_omega_mean": float(np.mean(omega_values[outside_target])),
                    "matched_control_omega_mean": float(np.mean(omega_values[matched_control])),
                    "positive_euclidean_area_decile_mean": float(np.mean(deciles[outside_target])),
                    "matched_control_euclidean_area_decile_mean": float(
                        np.mean(deciles[matched_control])
                    ),
                    "rule": "omega_spectral_norm_below_comparison_median",
                    **metrics(outside_target[comparison_mask], selected[comparison_mask]),
                }
            )
    return rows


def outside_small_face_examples_for_bucket(
    *,
    bucket_rows: list[dict[str, object]],
    capacity_source: str,
    facet_count: int,
    euclidean_bands: int,
    limit: int = 8,
) -> list[dict[str, object]]:
    sys_values = np.array([float(row["sys"]) for row in bucket_rows], dtype=float)
    euclidean_values = np.array(
        [float(row[FEATURES["euclidean_area_sum"]]) for row in bucket_rows], dtype=float
    )
    symplectic_values = np.array(
        [float(row[FEATURES["symp_area_sum"]]) for row in bucket_rows], dtype=float
    )
    omega_values = np.array(
        [float(row[FEATURES["omega_spectral_norm"]]) for row in bucket_rows], dtype=float
    )
    deciles = euclidean_decile_indices(euclidean_values, euclidean_bands)
    rows: list[dict[str, object]] = []
    for label, quantile in LABELS.items():
        target = sys_values >= np.quantile(sys_values, quantile)
        outside_lowest_20 = euclidean_values > np.quantile(euclidean_values, 0.2)
        candidate_indices = np.where(target & outside_lowest_20)[0]
        ordered = candidate_indices[
            np.argsort(sys_values[candidate_indices], kind="mergesort")[::-1]
        ][:limit]
        for index in ordered:
            source_row = bucket_rows[int(index)]
            rows.append(
                {
                    "capacity_source": capacity_source,
                    "facet_count": facet_count,
                    "label": label,
                    "poly_id": source_row.get("poly_id"),
                    "sys": float(sys_values[index]),
                    "euclidean_area_sum": float(euclidean_values[index]),
                    "euclidean_area_sum_decile": int(deciles[index]),
                    "symplectic_area_sum": float(symplectic_values[index]),
                    "omega_spectral_norm": float(omega_values[index]),
                    "symp_over_euclidean_mean": float(
                        source_row[FEATURES["symp_over_euclidean_mean"]]
                    ),
                    "facet_volume_sum": float(source_row[FEATURES["facet_volume_sum"]]),
                    "pairwise_distance_mean": float(
                        source_row[FEATURES["pairwise_distance_mean"]]
                    ),
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
    outside_small_face_rows: list[dict[str, object]] = []
    conditional_omega_rows: list[dict[str, object]] = []
    matched_outside_omega_rows: list[dict[str, object]] = []
    outside_example_rows: list[dict[str, object]] = []
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
        outside_small_face_rows.extend(
            outside_small_face_audit_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
            )
        )
        conditional_omega_rows.extend(
            conditional_omega_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
                tail_fraction=args.tail_fraction,
                euclidean_bands=args.euclidean_bands,
            )
        )
        matched_outside_omega_rows.extend(
            matched_outside_small_face_omega_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
                euclidean_bands=args.euclidean_bands,
            )
        )
        outside_example_rows.extend(
            outside_small_face_examples_for_bucket(
                bucket_rows=bucket_rows,
                capacity_source=capacity_source,
                facet_count=facet_count,
                euclidean_bands=args.euclidean_bands,
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
    write_tsv(args.out_dir / "outside-small-face-audit.tsv", outside_small_face_rows)
    write_tsv(args.out_dir / "conditional-omega-rules.tsv", conditional_omega_rows)
    write_tsv(
        args.out_dir / "matched-outside-small-face-omega.tsv",
        matched_outside_omega_rows,
    )
    write_tsv(args.out_dir / "outside-small-face-examples.tsv", outside_example_rows)
    write_json(
        args.out_dir / "summary.json",
        {
            "row_count": len(rows),
            "bucket_counts": bucket_counts,
            "labels": LABELS,
            "small_face_fractions": SMALL_FACE_FRACTIONS,
            "tail_fraction": args.tail_fraction,
            "euclidean_bands": args.euclidean_bands,
            "control_rule_count": len(control_rows),
            "euclidean_decile_row_count": len(decile_rows),
            "outside_small_face_audit_row_count": len(outside_small_face_rows),
            "conditional_omega_rule_count": len(conditional_omega_rows),
            "matched_outside_small_face_omega_count": len(matched_outside_omega_rows),
            "outside_small_face_example_count": len(outside_example_rows),
        },
    )
    print("# two-face Euclidean control diagnostics")
    print(f"- rows: `{len(rows)}`")
    print(f"- buckets: `{len(bucket_counts)}`")
    print(f"- control rule rows: `{len(control_rows)}`")
    print(f"- euclidean decile rows: `{len(decile_rows)}`")
    print(f"- outside-small-face audit rows: `{len(outside_small_face_rows)}`")
    print(f"- conditional omega rows: `{len(conditional_omega_rows)}`")
    print(f"- matched outside-small-face omega rows: `{len(matched_outside_omega_rows)}`")
    print(f"- outside-small-face examples: `{len(outside_example_rows)}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
