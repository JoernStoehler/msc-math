#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Diagnose whether height interval is a useful random-distribution axis."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
from pathlib import Path
import statistics
import sys
from typing import Any, Iterable

import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    dataset_label,
    load_trusted_random_tables,
    numeric_feature_names,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--variant",
        action="append",
        required=True,
        help="Variant as label=/path/to/prepared-tables.",
    )
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--top-features", type=int, default=20)
    parser.add_argument("--top-k", type=int, default=5)
    parser.add_argument("--bootstrap-samples", type=int, default=1000)
    parser.add_argument("--bootstrap-rng-seed", type=int, default=20260625)
    return parser.parse_args()


def parse_variant(value: str) -> tuple[str, Path]:
    if "=" not in value:
        raise SystemExit(f"--variant must be label=path, got {value!r}")
    label, path = value.split("=", 1)
    if not label:
        raise SystemExit(f"--variant label is empty in {value!r}")
    return label, Path(path)


def first_provenance(provenance_rows: list[dict[str, Any]]) -> dict[str, Any]:
    if not provenance_rows:
        return {}
    return sorted(provenance_rows, key=lambda row: str(row.get("provenance_id", "")))[0]


def height_label(provenance_row: dict[str, Any]) -> str:
    low = provenance_row.get("sample_h_min")
    high = provenance_row.get("sample_h_max")
    if isinstance(low, int | float) and isinstance(high, int | float):
        return f"{float(low):g}:{float(high):g}"
    return "unknown"


def row_metadata(
    row: dict[str, Any], provenance_rows: list[dict[str, Any]], variant: str
) -> dict[str, Any]:
    provenance = first_provenance(provenance_rows)
    dataset = dataset_label(row, provenance_rows)
    if dataset == "random_product_sample":
        bucket = product_bucket(provenance_rows)
    else:
        bucket = f"F{row.get('facet_count')}"
    seed = provenance.get("sample_seed")
    return {
        "variant": variant,
        "interval": height_label(provenance),
        "seed": int(seed) if isinstance(seed, int) else None,
        "dataset": dataset,
        "bucket": f"{dataset}:{bucket}",
        "source_name": str(provenance.get("source_name", "")),
    }


def load_variant(label: str, tables_dir: Path) -> list[dict[str, Any]]:
    polytope_rows, provenance_rows = load_trusted_random_tables(tables_dir)
    provenance = provenance_by_poly_id(provenance_rows)
    result = []
    for row in polytope_rows:
        enriched = dict(row)
        enriched["_meta"] = row_metadata(
            row, provenance.get(str(row["poly_id"]), []), label
        )
        result.append(enriched)
    return result


def eta_squared(values: list[float], labels: list[str]) -> float | None:
    if len(values) != len(labels) or len(values) < 2:
        return None
    groups: dict[str, list[float]] = defaultdict(list)
    for value, label in zip(values, labels, strict=True):
        if np.isfinite(value):
            groups[label].append(float(value))
    nonempty = [group for group in groups.values() if group]
    if len(nonempty) < 2:
        return None
    all_values = [value for group in nonempty for value in group]
    mean = statistics.mean(all_values)
    total_ss = sum((value - mean) ** 2 for value in all_values)
    if total_ss == 0.0:
        return 0.0
    between_ss = sum(
        len(group) * (statistics.mean(group) - mean) ** 2 for group in nonempty
    )
    return float(between_ss / total_ss)


def kruskal_p(values: list[float], labels: list[str]) -> float | None:
    groups: dict[str, list[float]] = defaultdict(list)
    for value, label in zip(values, labels, strict=True):
        if np.isfinite(value):
            groups[label].append(float(value))
    nonempty = [group for group in groups.values() if len(group) >= 2]
    if len(nonempty) < 2:
        return None
    try:
        return float(stats.kruskal(*nonempty).pvalue)
    except ValueError:
        return None


def factor_effect(
    rows: list[dict[str, Any]], value_name: str, factor_name: str
) -> dict[str, Any]:
    values = [float(row[value_name]) for row in rows if isinstance(row.get(value_name), int | float)]
    labels = [
        str(row["_meta"][factor_name])
        for row in rows
        if isinstance(row.get(value_name), int | float)
    ]
    return {
        "rows": len(values),
        "levels": sorted(set(labels)),
        "eta_squared": eta_squared(values, labels),
        "kruskal_p": kruskal_p(values, labels),
    }


def feature_effects(
    rows: list[dict[str, Any]],
    features: Iterable[str],
    factor_name: str,
    top_n: int,
) -> list[dict[str, Any]]:
    effects = []
    for feature in features:
        values = []
        labels = []
        for row in rows:
            value = row.get(feature)
            if isinstance(value, int | float) and np.isfinite(float(value)):
                values.append(float(value))
                labels.append(str(row["_meta"][factor_name]))
        eta = eta_squared(values, labels)
        if eta is None:
            continue
        effects.append(
            {
                "feature": feature,
                "eta_squared": eta,
                "kruskal_p": kruskal_p(values, labels),
            }
        )
    return sorted(effects, key=lambda row: row["eta_squared"], reverse=True)[:top_n]


def feature_effect_summary(
    rows: list[dict[str, Any]], features: list[str], factor_name: str
) -> dict[str, Any]:
    etas = []
    for feature in features:
        values = []
        labels = []
        for row in rows:
            value = row.get(feature)
            if isinstance(value, int | float) and np.isfinite(float(value)):
                values.append(float(value))
                labels.append(str(row["_meta"][factor_name]))
        eta = eta_squared(values, labels)
        if eta is not None:
            etas.append(eta)
    if not etas:
        return {"features": 0}
    return {
        "features": len(etas),
        "median_eta_squared": float(statistics.median(etas)),
        "q90_eta_squared": float(np.quantile(np.array(etas), 0.9)),
        "max_eta_squared": float(max(etas)),
    }


def summarize_sys_by_cell(rows: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    groups: dict[str, list[float]] = defaultdict(list)
    for row in rows:
        meta = row["_meta"]
        key = f"{meta['interval']}|{meta['bucket']}"
        groups[key].append(float(row["sys"]))
    result = {}
    for key, values in sorted(groups.items()):
        result[key] = {
            "rows": len(values),
            "mean": float(statistics.mean(values)),
            "median": float(statistics.median(values)),
            "q90": float(np.quantile(np.array(values), 0.9)),
            "max": float(max(values)),
        }
    return result


def bootstrap_ci(
    values: list[float],
    statistic: str,
    rng: np.random.Generator,
    sample_count: int,
) -> dict[str, float] | None:
    if not values:
        return None
    array = np.array(values, dtype=float)
    if statistic == "mean":
        observed = float(np.mean(array))
        reducer = np.mean
    elif statistic == "median":
        observed = float(np.median(array))
        reducer = np.median
    elif statistic == "q90":
        observed = float(np.quantile(array, 0.9))

        def reducer(sample: np.ndarray) -> float:
            return float(np.quantile(sample, 0.9))

    else:
        raise ValueError(f"unknown statistic {statistic}")
    if len(array) == 1 or sample_count <= 0:
        return {"estimate": observed, "ci_low": observed, "ci_high": observed}
    estimates = []
    for _ in range(sample_count):
        sample = rng.choice(array, size=len(array), replace=True)
        estimates.append(float(reducer(sample)))
    return {
        "estimate": observed,
        "ci_low": float(np.quantile(np.array(estimates), 0.025)),
        "ci_high": float(np.quantile(np.array(estimates), 0.975)),
    }


def bootstrap_sys_by_cell(
    rows: list[dict[str, Any]], sample_count: int, seed: int
) -> dict[str, dict[str, Any]]:
    groups: dict[str, list[float]] = defaultdict(list)
    for row in rows:
        meta = row["_meta"]
        key = f"{meta['interval']}|{meta['bucket']}"
        groups[key].append(float(row["sys"]))
    result = {}
    rng = np.random.default_rng(seed)
    for key, values in sorted(groups.items()):
        result[key] = {
            "rows": len(values),
            "mean": bootstrap_ci(values, "mean", rng, sample_count),
            "median": bootstrap_ci(values, "median", rng, sample_count),
            "q90": bootstrap_ci(values, "q90", rng, sample_count),
        }
    return result


def matched_source_topk_overlap(rows: list[dict[str, Any]], top_k: int) -> list[dict[str, Any]]:
    """Compare paired source names where provenance makes matching possible.

    This is not a seed-effect analysis. Seed is only part of the provenance key
    that prevents matching unrelated independent samples with reused names.
    """
    by_cell: dict[tuple[str, int | None, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        meta = row["_meta"]
        by_cell[(meta["interval"], meta["seed"], meta["bucket"])].append(row)
    by_seed_bucket: dict[tuple[int | None, str], dict[str, set[str]]] = defaultdict(dict)
    for (interval, seed, bucket), cell_rows in by_cell.items():
        ordered = sorted(cell_rows, key=lambda row: float(row["sys"]), reverse=True)
        by_seed_bucket[(seed, bucket)][interval] = {
            row["_meta"]["source_name"] for row in ordered[:top_k]
        }
    result = []
    for (seed, bucket), intervals in sorted(by_seed_bucket.items(), key=str):
        labels = sorted(intervals)
        for i, left in enumerate(labels):
            for right in labels[i + 1 :]:
                left_set = intervals[left]
                right_set = intervals[right]
                union = left_set | right_set
                result.append(
                    {
                        "seed": seed,
                        "bucket": bucket,
                        "left": left,
                        "right": right,
                        "top_k": top_k,
                        "intersection": len(left_set & right_set),
                        "jaccard": float(len(left_set & right_set) / len(union))
                        if union
                        else 1.0,
                    }
                )
    return result


def main() -> None:
    args = parse_args()
    variants = dict(parse_variant(value) for value in args.variant)
    rows = [row for label, path in variants.items() for row in load_variant(label, path)]
    intervals = {row["_meta"]["interval"] for row in rows}
    if len(intervals) < 2:
        raise SystemExit("need at least two height intervals for height-axis diagnostics")
    features = numeric_feature_names(rows, geometry_only=True)

    factor_rows = {
        "interval": rows,
        "dataset": rows,
        "bucket": rows,
    }
    factor_effects = {
        "height_interval_on_sys": factor_effect(
            factor_rows["interval"], "sys", "interval"
        ),
        "dataset_on_sys": factor_effect(
            factor_rows["dataset"], "sys", "dataset"
        ),
        "bucket_on_sys": factor_effect(factor_rows["bucket"], "sys", "bucket"),
    }

    bucket_interval_effects = {}
    for bucket in sorted({row["_meta"]["bucket"] for row in rows}):
        bucket_rows = [row for row in rows if row["_meta"]["bucket"] == bucket]
        bucket_interval_effects[bucket] = factor_effect(bucket_rows, "sys", "interval")

    summary = {
        "row_count": len(rows),
        "variants": sorted(variants),
        "interval_levels": sorted(intervals),
        "feature_count": len(features),
        "bootstrap_samples": args.bootstrap_samples,
        "sys_factor_effects": factor_effects,
        "sys_height_interval_effect_by_bucket": bucket_interval_effects,
        "geometry_feature_effect_summary": {
            "height_interval": feature_effect_summary(rows, features, "interval"),
            "dataset": feature_effect_summary(rows, features, "dataset"),
            "bucket": feature_effect_summary(rows, features, "bucket"),
        },
        "top_height_interval_features": feature_effects(
            rows, features, "interval", args.top_features
        ),
        "top_bucket_features": feature_effects(
            rows, features, "bucket", args.top_features
        ),
        "matched_source_topk_overlap": matched_source_topk_overlap(rows, args.top_k),
        "sys_by_cell": summarize_sys_by_cell(rows),
        "bootstrap_sys_by_cell": bootstrap_sys_by_cell(
            rows, args.bootstrap_samples, args.bootstrap_rng_seed
        ),
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# random-axis-diagnostic")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- geometry features analyzed: `{len(features)}`")
    print(f"- bootstrap samples per cell/statistic: `{args.bootstrap_samples}`")
    print()
    print("## Sys Effect Sizes")
    print()
    print("| factor | eta^2 | Kruskal p | levels |")
    print("| --- | ---: | ---: | --- |")
    for name, effect in factor_effects.items():
        eta = effect["eta_squared"]
        p = effect["kruskal_p"]
        print(
            f"| {name} | {eta if eta is not None else 'NA'} | "
            f"{p if p is not None else 'NA'} | {json.dumps(effect['levels'])} |"
        )
    print()
    print("## Geometry Feature Effect Summary")
    print()
    print("| factor | median eta^2 | q90 eta^2 | max eta^2 |")
    print("| --- | ---: | ---: | ---: |")
    for name, item in summary["geometry_feature_effect_summary"].items():
        print(
            f"| {name} | {item.get('median_eta_squared', 'NA')} | "
            f"{item.get('q90_eta_squared', 'NA')} | {item.get('max_eta_squared', 'NA')} |"
        )
    print()
    print(f"Wrote `{args.out_dir / 'summary.json'}`")


if __name__ == "__main__":
    main()
