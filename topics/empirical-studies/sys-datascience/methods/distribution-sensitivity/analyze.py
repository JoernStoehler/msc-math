#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Compare random/product prepared-table variants for distribution sensitivity."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
from pathlib import Path
import statistics
import sys
from typing import Any

import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    active_invariant_numeric_feature_names,
    dataset_label,
    load_trusted_random_tables,
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
        help="Variant as label=/path/to/prepared-tables. Pass at least two.",
    )
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--top-n", type=int, default=10)
    parser.add_argument("--top-features", type=int, default=10)
    return parser.parse_args()


def parse_variant(value: str) -> tuple[str, Path]:
    if "=" not in value:
        raise SystemExit(f"--variant must be label=path, got {value!r}")
    label, path = value.split("=", 1)
    if not label:
        raise SystemExit(f"--variant label is empty in {value!r}")
    return label, Path(path)


def quantiles(values: list[float]) -> dict[str, float]:
    qs = [0.0, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 1.0]
    array = np.array(values, dtype=float)
    return {str(q): float(v) for q, v in zip(qs, np.quantile(array, qs), strict=True)}


def sample_height_ranges(provenance_rows: list[dict[str, Any]]) -> list[list[float]]:
    ranges = sorted(
        {
            (float(row["sample_h_min"]), float(row["sample_h_max"]))
            for row in provenance_rows
            if isinstance(row.get("sample_h_min"), int | float)
            and isinstance(row.get("sample_h_max"), int | float)
        }
    )
    return [[low, high] for low, high in ranges]


def row_group(
    row: dict[str, Any], provenance_rows: list[dict[str, Any]]
) -> dict[str, str | None]:
    label = dataset_label(row, provenance_rows)
    product = product_bucket(provenance_rows) if label == "random_product_sample" else None
    return {
        "dataset": label,
        "facet_count": f"F{row.get('facet_count')}",
        "dataset_by_facet_count": f"{label}:F{row.get('facet_count')}",
        "product_bucket": product,
    }


def basic_summary(values: list[float]) -> dict[str, Any]:
    return {
        "rows": len(values),
        "mean": float(statistics.mean(values)),
        "median": float(statistics.median(values)),
        "std": float(statistics.pstdev(values)) if len(values) > 1 else 0.0,
        "max": float(max(values)),
        "gap_from_one_at_max": float(1.0 - max(values)),
        "sys_gt_one": sum(1 for value in values if value > 1.0),
        "quantiles": quantiles(values),
    }


def top_rows(
    label: str,
    rows: list[dict[str, Any]],
    provenance: dict[str, list[dict[str, Any]]],
    top_n: int,
) -> list[dict[str, Any]]:
    ordered = sorted(rows, key=lambda row: float(row["sys"]), reverse=True)
    result = []
    for row in ordered[:top_n]:
        poly_id = str(row["poly_id"])
        prov = provenance.get(poly_id, [])
        groups = row_group(row, prov)
        result.append(
            {
                "variant": label,
                "poly_id": poly_id,
                "sys": float(row["sys"]),
                "gap_from_one": float(1.0 - float(row["sys"])),
                "facet_count": row.get("facet_count"),
                "capacity_source": row.get("capacity_source"),
                "dataset": groups["dataset"],
                "product_bucket": groups["product_bucket"],
                "height_ranges": sample_height_ranges(prov),
            }
        )
    return result


def grouped_summaries(
    rows: list[dict[str, Any]], provenance: dict[str, list[dict[str, Any]]]
) -> dict[str, dict[str, Any]]:
    grouped: dict[str, dict[str, list[float]]] = defaultdict(lambda: defaultdict(list))
    for row in rows:
        groups = row_group(row, provenance.get(str(row["poly_id"]), []))
        sys_value = float(row["sys"])
        for group_name, group_value in groups.items():
            if group_value is not None:
                grouped[group_name][group_value].append(sys_value)
    return {
        group_name: {
            group_value: basic_summary(values)
            for group_value, values in sorted(groups.items())
        }
        for group_name, groups in sorted(grouped.items())
    }


def feature_associations(
    rows: list[dict[str, Any]], top_features: int
) -> list[dict[str, Any]]:
    names = active_invariant_numeric_feature_names(rows)
    sys_values = np.array([float(row["sys"]) for row in rows], dtype=float)
    scored = []
    for name in names:
        values = np.array([float(row.get(name, 0.0)) for row in rows], dtype=float)
        if len(set(values.tolist())) < 2:
            continue
        result = stats.spearmanr(values, sys_values)
        rho = float(result.statistic)
        if not np.isfinite(rho):
            continue
        scored.append(
            {
                "feature": name,
                "spearman_rho": rho,
                "spearman_abs_rho": abs(rho),
                "spearman_p": float(result.pvalue),
            }
        )
    return sorted(scored, key=lambda row: row["spearman_abs_rho"], reverse=True)[:top_features]


def pairwise_tests(variants: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    labels = sorted(variants)
    rows = []
    for i, left in enumerate(labels):
        for right in labels[i + 1 :]:
            left_values = variants[left]["sys_values"]
            right_values = variants[right]["sys_values"]
            ks = stats.ks_2samp(left_values, right_values)
            mann = stats.mannwhitneyu(left_values, right_values, alternative="two-sided")
            rows.append(
                {
                    "left": left,
                    "right": right,
                    "left_rows": len(left_values),
                    "right_rows": len(right_values),
                    "left_max": float(max(left_values)),
                    "right_max": float(max(right_values)),
                    "max_delta_left_minus_right": float(max(left_values) - max(right_values)),
                    "mean_delta_left_minus_right": float(
                        statistics.mean(left_values) - statistics.mean(right_values)
                    ),
                    "ks_statistic": float(ks.statistic),
                    "ks_p": float(ks.pvalue),
                    "mann_whitney_u": float(mann.statistic),
                    "mann_whitney_p": float(mann.pvalue),
                }
            )
    return rows


def paired_sys_by_source(
    rows: list[dict[str, Any]], provenance: dict[str, list[dict[str, Any]]]
) -> dict[tuple[str, int | None, str], float]:
    result = {}
    for row in rows:
        poly_id = str(row["poly_id"])
        for provenance_row in provenance.get(poly_id, []):
            dataset = str(provenance_row.get("dataset", ""))
            source_name = str(provenance_row.get("source_name", ""))
            sample_seed = provenance_row.get("sample_seed")
            seed = int(sample_seed) if isinstance(sample_seed, int) else None
            if dataset and source_name and seed is not None:
                result[(dataset, seed, source_name)] = float(row["sys"])
    return result


def paired_delta_tests(variants: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    labels = sorted(variants)
    rows = []
    for i, left in enumerate(labels):
        for right in labels[i + 1 :]:
            left_values = variants[left]["paired_sys_by_source"]
            right_values = variants[right]["paired_sys_by_source"]
            common = sorted(set(left_values) & set(right_values))
            if not common:
                continue
            by_dataset: dict[str, list[float]] = defaultdict(list)
            by_bucket: dict[str, list[float]] = defaultdict(list)
            for key in common:
                dataset, _, source_name = key
                delta = right_values[key] - left_values[key]
                by_dataset[dataset].append(delta)
                if dataset == "random_product_sample":
                    parts = source_name.split("_")
                    bucket = parts[1] if len(parts) >= 3 else "unknown"
                else:
                    parts = source_name.split("_")
                    bucket = parts[1] if len(parts) >= 3 else "unknown"
                by_bucket[f"{dataset}:{bucket}"].append(delta)
            rows.append(
                {
                    "left": left,
                    "right": right,
                    "paired_rows": len(common),
                    "by_dataset": summarize_deltas(by_dataset),
                    "by_bucket": summarize_deltas(by_bucket),
                }
            )
    return rows


def summarize_deltas(groups: dict[str, list[float]]) -> dict[str, dict[str, Any]]:
    result = {}
    for label, values in sorted(groups.items()):
        abs_values = [abs(value) for value in values]
        result[label] = {
            "rows": len(values),
            "mean_delta_right_minus_left": float(statistics.mean(values)),
            "median_abs_delta": float(statistics.median(abs_values)),
            "max_abs_delta": float(max(abs_values)),
            "changed_gt_1e_9": sum(value > 1e-9 for value in abs_values),
        }
    return result


def main() -> None:
    args = parse_args()
    variants = dict(parse_variant(value) for value in args.variant)
    if len(variants) < 2:
        raise SystemExit("pass at least two --variant label=path arguments")

    analyzed: dict[str, dict[str, Any]] = {}
    all_top_rows = []
    for label, tables_dir in variants.items():
        polytope_rows, provenance_rows = load_trusted_random_tables(tables_dir)
        provenance = provenance_by_poly_id(provenance_rows)
        sys_values = [float(row["sys"]) for row in polytope_rows]
        analyzed[label] = {
            "tables_dir": str(tables_dir),
            "rows": len(polytope_rows),
            "provenance_rows": len(provenance_rows),
            "height_ranges": sample_height_ranges(provenance_rows),
            "overall": basic_summary(sys_values),
            "groups": grouped_summaries(polytope_rows, provenance),
            "top_feature_associations": feature_associations(
                polytope_rows, args.top_features
            ),
            "sys_values": sys_values,
            "paired_sys_by_source": paired_sys_by_source(polytope_rows, provenance),
        }
        all_top_rows.extend(top_rows(label, polytope_rows, provenance, args.top_n))

    feature_sets = {
        label: {
            row["feature"]
            for row in analyzed[label]["top_feature_associations"][: args.top_features]
        }
        for label in analyzed
    }
    common_top_features = sorted(set.intersection(*feature_sets.values())) if feature_sets else []
    kruskal = stats.kruskal(*(row["sys_values"] for row in analyzed.values()))

    serializable_variants = {
        label: {
            key: value
            for key, value in row.items()
            if key not in {"sys_values", "paired_sys_by_source"}
        }
        for label, row in analyzed.items()
    }
    summary = {
        "variant_count": len(analyzed),
        "variants": serializable_variants,
        "pairwise_tests": pairwise_tests(analyzed),
        "paired_delta_tests": paired_delta_tests(analyzed),
        "overall_kruskal_h": float(kruskal.statistic),
        "overall_kruskal_p": float(kruskal.pvalue),
        "top_rows": sorted(all_top_rows, key=lambda row: row["sys"], reverse=True)[
            : args.top_n
        ],
        "common_top_features": common_top_features,
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# distribution-sensitivity")
    print()
    print(f"- variants: `{len(analyzed)}`")
    print(f"- overall Kruskal p: `{summary['overall_kruskal_p']:.6g}`")
    print(f"- common top features: `{', '.join(common_top_features) or '-'}`")
    print()
    print("| variant | rows | max sys | q95 sys | mean sys | sys > 1 | height ranges |")
    print("| --- | ---: | ---: | ---: | ---: | ---: | --- |")
    for label, row in sorted(serializable_variants.items()):
        overall = row["overall"]
        print(
            "| "
            + " | ".join(
                [
                    label,
                    str(overall["rows"]),
                    f"{overall['max']:.12g}",
                    f"{overall['quantiles']['0.95']:.12g}",
                    f"{overall['mean']:.12g}",
                    str(overall["sys_gt_one"]),
                    json.dumps(row["height_ranges"]),
                ]
            )
            + " |"
        )
    print()
    print(f"Wrote `{args.out_dir / 'summary.json'}`")


if __name__ == "__main__":
    main()
