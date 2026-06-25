#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""Random-only tail summaries and simple EDA plots."""

from __future__ import annotations

import argparse
from collections import defaultdict
import math
from pathlib import Path
import statistics
import sys

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    dataset_label,
    load_trusted_random_tables,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    return parser.parse_args()


def quantiles(values: list[float]) -> dict[str, float]:
    qs = [0.0, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 1.0]
    result = np.quantile(np.array(values, dtype=float), qs)
    return {str(q): float(value) for q, value in zip(qs, result, strict=True)}


def distribution_summary(values: list[float]) -> dict[str, float | int]:
    return {
        "rows": len(values),
        "mean": float(statistics.mean(values)),
        "quantiles": quantiles(values),
        "gap_from_one_at_max": float(1.0 - max(values)),
    }


def zero_positive_upper_probability(row_count: int, confidence: float) -> float:
    return 1.0 - math.exp(math.log(1.0 - confidence) / row_count)


def tail_summary(values: list[float]) -> dict[str, object]:
    upper_p_95 = zero_positive_upper_probability(len(values), 0.95)
    return {
        **distribution_summary(values),
        "sys_gt_one": sum(1 for value in values if value > 1.0),
        "zero_positive_95pct_upper_probability": upper_p_95,
        "zero_positive_95pct_samples_per_hit_lower_bound": float(1.0 / upper_p_95),
        "crude_exponential_tail_extrapolations": exponential_tail_extrapolations(values),
    }


def exponential_tail_extrapolations(values: list[float]) -> list[dict[str, float | int]]:
    """Crude diagnostic only: fit log empirical survival against gap from 1."""
    ordered = sorted(values)
    n = len(ordered)
    results: list[dict[str, float | int]] = []
    for tail_count in [50, 100, 250, 500, 1000]:
        if tail_count >= n:
            continue
        xs: list[float] = []
        ys: list[float] = []
        for rank, value in enumerate(ordered[-tail_count:], start=n - tail_count + 1):
            survival = (n - rank + 1) / n
            gap = 1.0 - value
            if survival > 0.0 and gap > 0.0:
                xs.append(gap)
                ys.append(math.log(survival))
        mean_x = statistics.mean(xs)
        mean_y = statistics.mean(ys)
        denominator = sum((x - mean_x) ** 2 for x in xs)
        if denominator == 0.0:
            continue
        slope = sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys, strict=True))
        slope /= denominator
        intercept = mean_y - slope * mean_x
        probability_at_one = math.exp(intercept)
        results.append(
            {
                "tail_count": tail_count,
                "estimated_probability_sys_gt_one": float(probability_at_one),
                "estimated_samples_per_hit": float(1.0 / probability_at_one),
                "slope": float(slope),
            }
        )
    return results


def source_parameter_availability(provenance_rows: list[dict[str, object]]) -> dict[str, object]:
    fields = [
        "sample_seed",
        "sample_attempt",
        "sample_h_min",
        "sample_h_max",
        "product_k",
        "product_m",
        "product_bounces",
    ]
    by_field = {
        field: sum(1 for row in provenance_rows if field in row and row[field] is not None)
        for field in fields
    }
    height_ranges = sorted(
        {
            (float(row["sample_h_min"]), float(row["sample_h_max"]))
            for row in provenance_rows
            if isinstance(row.get("sample_h_min"), int | float)
            and isinstance(row.get("sample_h_max"), int | float)
        }
    )
    product_buckets = sorted(
        {
            f"{int(row['product_k'])}x{int(row['product_m'])}"
            for row in provenance_rows
            if isinstance(row.get("product_k"), int) and isinstance(row.get("product_m"), int)
        }
    )
    explicit_buckets_by_poly_id: dict[str, set[str]] = defaultdict(set)
    for row in provenance_rows:
        poly_id = str(row.get("poly_id", ""))
        if (
            poly_id
            and isinstance(row.get("product_k"), int)
            and isinstance(row.get("product_m"), int)
        ):
            explicit_buckets_by_poly_id[poly_id].add(
                f"{int(row['product_k'])}x{int(row['product_m'])}"
            )
    return {
        "provenance_rows": len(provenance_rows),
        "rows_with_field": by_field,
        "explicit_height_ranges": height_ranges,
        "explicit_product_buckets": product_buckets,
        "poly_ids_with_multiple_explicit_product_buckets": sum(
            1 for buckets in explicit_buckets_by_poly_id.values() if len(buckets) > 1
        ),
    }


def generator_contract() -> dict[str, object]:
    return {
        "source_truth": [
            "experiments/sys-datascience/produce/main.rs",
            "crates/euclidean-polytopes/src/random.rs",
            "symplectic::geom::polygon::random_polygon_2d",
        ],
        "producer_mode": "production",
        "generic_random": {
            "varied_parameters": ["facet_count"],
            "facet_counts": [5, 6, 7, 8, 9, 10, 11, 12],
            "accepted_samples_per_facet_count": 512,
            "seed": 42,
            "height_range": [0.8, 1.2],
            "normal_distribution": "independent unit normals sampled uniformly on S^3",
            "height_distribution": "independent uniform heights in [0.8, 1.2)",
            "acceptance_rule": "reject generated candidates until SysLandscapePolytopeCache::sample_random returns a valid polytope",
        },
        "random_lagrangian_product": {
            "varied_parameters": ["polygon_side_count_k", "polygon_side_count_m"],
            "polygon_pair_buckets": [
                "3x3",
                "3x4",
                "3x5",
                "3x6",
                "4x4",
                "4x5",
                "4x6",
                "5x5",
                "5x6",
                "6x6",
            ],
            "accepted_samples_per_bucket": 1024,
            "seed": 42,
            "height_range": [0.8, 1.2],
            "factor_distribution": "two independent random 2D polygons from random_polygon_2d, then Lagrangian product",
            "acceptance_rule": "reject generated candidates until SysLandscapePolytopeCache::from_lagrangian_product returns a valid polytope",
        },
        "not_varied": [
            "height range",
            "seed or independent rerun",
            "generic facet counts outside 5..12",
            "product side counts outside 3..6",
            "alternative random distributions",
            "optimizer or trajectory parameters",
        ],
    }


def research_questions() -> list[dict[str, str]]:
    return [
        {
            "question": "What is the empirical marginal distribution of sys(a) on the retained finite random/product sample?",
            "answered_by": "pooled histogram, quantiles, top rows, and top-tail summaries",
            "status": "answered for the retained finite sample",
        },
        {
            "question": "Do any retained random/product rows, or obvious overlapping source/parameter subsets, contain sys > 1?",
            "answered_by": "pooled and filtered tail summaries",
            "status": "answered negatively for the retained rows and implemented filters",
        },
        {
            "question": "Where is the high tail concentrated inside the retained sample?",
            "answered_by": "dataset, facet-count, dataset+facet-count, and product-bucket summaries",
            "status": "descriptive ranking only; not a validated proposer",
        },
        {
            "question": "Are the observed statements universal across arbitrary random polytope distributions?",
            "answered_by": "generator contract and not-varied fields",
            "status": "not answered; this packet only covers the documented retained generators",
        },
        {
            "question": "At what sample size should one expect the first sys > 1 row under a specified generator?",
            "answered_by": "zero-positive bound and crude tail extrapolation diagnostics",
            "status": "not stably answered by the current finite sample",
        },
    ]


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    provenance = provenance_by_poly_id(provenance_rows)
    args.out_dir.mkdir(parents=True, exist_ok=True)

    by_dataset: dict[str, list[float]] = defaultdict(list)
    by_facet_count: dict[str, list[float]] = defaultdict(list)
    by_dataset_and_facet: dict[str, list[float]] = defaultdict(list)
    by_product_bucket: dict[str, list[float]] = defaultdict(list)
    generic_by_facet_count: dict[str, list[float]] = defaultdict(list)
    for row in rows:
        provenance_for_row = provenance.get(str(row["poly_id"]), [])
        label = dataset_label(row, provenance_for_row)
        sys_value = float(row["sys"])
        by_dataset[label].append(sys_value)
        by_facet_count[str(row["facet_count"])].append(sys_value)
        by_dataset_and_facet[f"{label}:F{row['facet_count']}"].append(sys_value)
        if row.get("capacity_source") == "random_sample":
            generic_by_facet_count[f"F{row['facet_count']}"].append(sys_value)
        if row.get("capacity_source") == "random_product_sample":
            by_product_bucket[product_bucket(provenance_for_row)].append(sys_value)

    top_rows = sorted(rows, key=lambda row: float(row["sys"]), reverse=True)[:25]
    sys_values = [float(row["sys"]) for row in rows]
    upper_p_95 = zero_positive_upper_probability(len(sys_values), 0.95)
    filtered_tail_summaries = {
        "pooled": {"all_trusted_random_product": tail_summary(sys_values)},
        "by_dataset": {
            label: tail_summary(values) for label, values in sorted(by_dataset.items())
        },
        "by_facet_count": {
            label: tail_summary(values) for label, values in sorted(by_facet_count.items())
        },
        "by_dataset_and_facet": {
            label: tail_summary(values) for label, values in sorted(by_dataset_and_facet.items())
        },
        "by_product_bucket": {
            label: tail_summary(values) for label, values in sorted(by_product_bucket.items())
        },
    }
    ranked_filtered_slices = sorted(
        [
            {
                "filter_family": family,
                "filter": label,
                "rows": summary["rows"],
                "p99": summary["quantiles"]["0.99"],
                "max": summary["quantiles"]["1.0"],
                "sys_gt_one": summary["sys_gt_one"],
            }
            for family, summaries in filtered_tail_summaries.items()
            for label, summary in summaries.items()
        ],
        key=lambda row: (float(row["p99"]), float(row["max"])),
        reverse=True,
    )
    summary = {
        "row_count": len(rows),
        "research_questions": research_questions(),
        "generator_contract": generator_contract(),
        "source_parameter_availability": source_parameter_availability(provenance_rows),
        "sys_gt_one": sum(1 for row in rows if float(row["sys"]) > 1.0),
        "overall_distribution": distribution_summary(sys_values),
        "overall_quantiles": quantiles(sys_values),
        "dataset_quantiles": {
            label: quantiles(values) for label, values in sorted(by_dataset.items())
        },
        "dataset_distributions": {
            label: distribution_summary(values) for label, values in sorted(by_dataset.items())
        },
        "facet_count_distributions": {
            label: distribution_summary(values) for label, values in sorted(by_facet_count.items())
        },
        "dataset_and_facet_distributions": {
            label: distribution_summary(values)
            for label, values in sorted(by_dataset_and_facet.items())
        },
        "zero_positive_95pct_upper_probability": upper_p_95,
        "zero_positive_95pct_samples_per_hit_lower_bound": 1.0 / upper_p_95,
        "crude_exponential_tail_extrapolations": exponential_tail_extrapolations(sys_values),
        "filtered_tail_summaries": filtered_tail_summaries,
        "top_filtered_slices_by_p99": ranked_filtered_slices[:20],
        "interpretation": {
            "pooled_tail_forecast_decision_grade": False,
            "reason": (
                "The retained random-only sample has no sys > 1 rows, max sys is "
                "about 0.137 below threshold, and the 99th percentile is about "
                "0.248 below threshold. Tail extrapolations vary by orders of "
                "magnitude and are not decision-grade."
            ),
            "important_open_questions": [
                "How stable are the high-tail estimates under independent reruns?",
                "Which parameter families should be sampled if the goal is to increase tail mass near sys=1?",
            ],
        },
        "top_rows": [
            {
                "poly_id": row["poly_id"],
                "dataset": dataset_label(row, provenance.get(str(row["poly_id"]), [])),
                "facet_count": row.get("facet_count"),
                "sys": row["sys"],
            }
            for row in top_rows
        ],
    }
    write_json(args.out_dir / "summary.json", summary)

    import matplotlib.pyplot as plt

    fig, ax = plt.subplots(figsize=(7.2, 4.2))
    ax.hist(sys_values, bins=60, color="#4c78a8", alpha=0.85)
    ax.axvline(1.0, color="#c44e52", linewidth=1.2)
    ax.set_xlabel("sys")
    ax.set_ylabel("random-only rows")
    ax.set_title("Trusted random/product sys distribution")
    fig.tight_layout()
    fig.savefig(args.out_dir / "sys-histogram.png", dpi=160)
    plt.close(fig)

    fig, ax = plt.subplots(figsize=(7.2, 4.2))
    ordered = sorted(sys_values)
    tail = ordered[max(0, len(ordered) - 500) :]
    ax.plot(range(len(tail)), tail, color="#4c78a8")
    ax.axhline(1.0, color="#c44e52", linewidth=1.2)
    ax.set_xlabel("tail rank within top 500")
    ax.set_ylabel("sys")
    ax.set_title("Trusted random/product top tail")
    fig.tight_layout()
    fig.savefig(args.out_dir / "sys-tail.png", dpi=160)
    plt.close(fig)

    def plot_bucket_p99(
        summaries: dict[str, dict[str, object]], title: str, out_name: str
    ) -> None:
        ordered_summaries = sorted(
            summaries.items(),
            key=lambda item: float(item[1]["quantiles"]["0.99"]),
            reverse=True,
        )
        labels = [label for label, _ in ordered_summaries]
        p99 = [float(summary["quantiles"]["0.99"]) for _, summary in ordered_summaries]
        max_values = [float(summary["quantiles"]["1.0"]) for _, summary in ordered_summaries]
        fig, ax = plt.subplots(figsize=(9.0, max(3.8, 0.35 * len(labels) + 1.2)))
        y = np.arange(len(labels))
        ax.barh(y, p99, color="#4c78a8", label="p99")
        ax.scatter(max_values, y, color="#c44e52", s=18, label="max")
        ax.axvline(1.0, color="black", linewidth=1.0)
        ax.set_yticks(y, labels)
        ax.invert_yaxis()
        ax.set_xlabel("sys")
        ax.set_title(title)
        ax.legend(loc="lower right")
        fig.tight_layout()
        fig.savefig(args.out_dir / out_name, dpi=160)
        plt.close(fig)

    plot_bucket_p99(
        {
            label: tail_summary(values)
            for label, values in sorted(generic_by_facet_count.items())
        },
        "Generic random sys by facet count",
        "generic-by-f.png",
    )
    plot_bucket_p99(
        {label: tail_summary(values) for label, values in sorted(by_product_bucket.items())},
        "Random product sys by polygon-pair bucket",
        "product-by-bucket.png",
    )

    print("# random-tail-eda")
    print()
    print(f"- rows: `{summary['row_count']}`")
    print(f"- `sys > 1`: `{summary['sys_gt_one']}`")
    print(f"- max `sys`: `{summary['overall_quantiles']['1.0']}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
