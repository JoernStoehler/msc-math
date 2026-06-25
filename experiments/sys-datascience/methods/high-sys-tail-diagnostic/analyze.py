#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scipy"]
# ///

"""High-sys tail diagnostics by random/product bucket."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
from pathlib import Path
import sys
import warnings
from typing import Any

import matplotlib.pyplot as plt
import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
BROAD_SCAN = HERE.parent / "sys-distribution-broad-scan" / "analyze.py"
spec = importlib.util.spec_from_file_location("sys_distribution_broad_scan", BROAD_SCAN)
if spec is None or spec.loader is None:
    raise RuntimeError(f"Cannot load broad scan helpers from {BROAD_SCAN}")
broad = importlib.util.module_from_spec(spec)
spec.loader.exec_module(broad)

sys.path.append(str(HERE.parent / "_shared"))
from random_only import write_json  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--threshold-quantiles", nargs="+", type=float, default=[0.8, 0.9, 0.95])
    parser.add_argument("--bootstrap-count", type=int, default=200)
    parser.add_argument("--rng-seed", type=int, default=20260625)
    return parser.parse_args()


def write_tsv(path: Path, rows: list[dict[str, Any]], columns: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        handle.write("\t".join(columns))
        handle.write("\n")
        for row in rows:
            values = []
            for column in columns:
                value = row.get(column, "")
                if isinstance(value, list | dict):
                    value = json.dumps(value, sort_keys=True, separators=(",", ":"))
                values.append(str(value))
            handle.write("\t".join(values))
            handle.write("\n")


def quantile_summary(values: np.ndarray) -> dict[str, float]:
    quantiles = [0.5, 0.8, 0.9, 0.95, 0.99, 1.0]
    result = np.quantile(values, quantiles)
    return {f"q{int(q * 100):02d}": float(value) for q, value in zip(quantiles, result, strict=True)}


def gpd_survival(excess: float, shape: float, scale: float) -> float:
    if excess <= 0.0:
        return 1.0
    if scale <= 0.0:
        return float("nan")
    if abs(shape) < 1e-10:
        return math.exp(-excess / scale)
    base = 1.0 + shape * excess / scale
    if base <= 0.0:
        return 0.0
    return base ** (-1.0 / shape)


def gpd_endpoint(threshold: float, shape: float, scale: float) -> float | str:
    if shape < 0.0 and scale > 0.0:
        return float(threshold - scale / shape)
    return "infinite"


def fit_tail(bucket: str, values: np.ndarray, threshold_quantile: float) -> list[dict[str, Any]]:
    threshold = float(np.quantile(values, threshold_quantile))
    tail_values = values[values > threshold]
    excess = tail_values - threshold
    rows: list[dict[str, Any]] = []
    if len(excess) < 8:
        return [
            {
                "bucket": bucket,
                "threshold_quantile": threshold_quantile,
                "threshold": threshold,
                "tail_n": len(excess),
                "model": "insufficient_tail_rows",
                "status": "failed",
            }
        ]

    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        try:
            shape, loc, scale = stats.genpareto.fit(excess, floc=0.0)
            logpdf = stats.genpareto.logpdf(excess, shape, loc=loc, scale=scale)
            if not np.all(np.isfinite(logpdf)):
                raise ValueError("non-finite GPD logpdf")
            endpoint = gpd_endpoint(threshold, float(shape), float(scale))
            prob_gt_one = (len(excess) / len(values)) * gpd_survival(
                1.0 - threshold,
                float(shape),
                float(scale),
            )
            rows.append(
                {
                    "bucket": bucket,
                    "threshold_quantile": threshold_quantile,
                    "threshold": threshold,
                    "tail_n": len(excess),
                    "model": "gpd",
                    "status": "ok",
                    "shape": float(shape),
                    "scale": float(scale),
                    "endpoint": endpoint,
                    "prob_sys_gt_1": float(prob_gt_one),
                    "samples_per_sys_gt_1": float(1.0 / prob_gt_one) if prob_gt_one > 0.0 else "infinite",
                    "log_likelihood": float(np.sum(logpdf)),
                    "per_row_log_likelihood": float(np.mean(logpdf)),
                }
            )
        except Exception as exc:
            rows.append(
                {
                    "bucket": bucket,
                    "threshold_quantile": threshold_quantile,
                    "threshold": threshold,
                    "tail_n": len(excess),
                    "model": "gpd",
                    "status": "failed",
                    "error": str(exc)[:240],
                }
            )

        try:
            scale = float(np.mean(excess))
            logpdf = stats.expon.logpdf(excess, loc=0.0, scale=scale)
            if scale <= 0.0 or not np.all(np.isfinite(logpdf)):
                raise ValueError("non-finite exponential logpdf")
            prob_gt_one = (len(excess) / len(values)) * math.exp(-(1.0 - threshold) / scale)
            rows.append(
                {
                    "bucket": bucket,
                    "threshold_quantile": threshold_quantile,
                    "threshold": threshold,
                    "tail_n": len(excess),
                    "model": "exponential",
                    "status": "ok",
                    "shape": 0.0,
                    "scale": scale,
                    "endpoint": "infinite",
                    "prob_sys_gt_1": float(prob_gt_one),
                    "samples_per_sys_gt_1": float(1.0 / prob_gt_one) if prob_gt_one > 0.0 else "infinite",
                    "log_likelihood": float(np.sum(logpdf)),
                    "per_row_log_likelihood": float(np.mean(logpdf)),
                }
            )
        except Exception as exc:
            rows.append(
                {
                    "bucket": bucket,
                    "threshold_quantile": threshold_quantile,
                    "threshold": threshold,
                    "tail_n": len(excess),
                    "model": "exponential",
                    "status": "failed",
                    "error": str(exc)[:240],
                }
            )

    return rows


def add_likelihood_gaps(rows: list[dict[str, Any]]) -> None:
    by_key: dict[tuple[str, float], list[dict[str, Any]]] = {}
    for row in rows:
        by_key.setdefault((row["bucket"], float(row["threshold_quantile"])), []).append(row)
    for group_rows in by_key.values():
        ok_rows = [row for row in group_rows if row["status"] == "ok"]
        if not ok_rows:
            continue
        best_ll = max(float(row["log_likelihood"]) for row in ok_rows)
        for row in group_rows:
            if row["status"] != "ok":
                row["delta_log_likelihood_vs_best"] = ""
                row["log2_likelihood_ratio_vs_best"] = ""
                continue
            delta = float(row["log_likelihood"]) - best_ll
            row["delta_log_likelihood_vs_best"] = delta
            row["log2_likelihood_ratio_vs_best"] = delta / math.log(2.0)


def bootstrap_gpd_endpoint(
    values: np.ndarray,
    threshold_quantile: float,
    rng: np.random.Generator,
    bootstrap_count: int,
) -> dict[str, Any]:
    endpoints = []
    shapes = []
    for _ in range(bootstrap_count):
        sample = values[rng.integers(0, len(values), size=len(values))]
        threshold = float(np.quantile(sample, threshold_quantile))
        excess = sample[sample > threshold] - threshold
        if len(excess) < 8:
            continue
        try:
            with warnings.catch_warnings():
                warnings.simplefilter("ignore")
                shape, loc, scale = stats.genpareto.fit(excess, floc=0.0)
            shapes.append(float(shape))
            endpoint = gpd_endpoint(threshold, float(shape), float(scale))
            if isinstance(endpoint, float) and np.isfinite(endpoint):
                endpoints.append(endpoint)
        except Exception:
            continue
    result: dict[str, Any] = {
        "bootstrap_success_count": len(shapes),
        "finite_endpoint_count": len(endpoints),
        "negative_shape_fraction": float(sum(1 for shape in shapes if shape < 0.0) / len(shapes))
        if shapes
        else "",
    }
    if endpoints:
        array = np.array(endpoints, dtype=float)
        result.update(
            {
                "endpoint_bootstrap_q05": float(np.quantile(array, 0.05)),
                "endpoint_bootstrap_median": float(np.quantile(array, 0.5)),
                "endpoint_bootstrap_q95": float(np.quantile(array, 0.95)),
            }
        )
    return result


def plot_survival(groups: dict[str, np.ndarray], out_dir: Path, prefix: str) -> None:
    selected = {key: values for key, values in groups.items() if key.startswith(prefix)}
    if not selected:
        return
    plt.figure(figsize=(8, 5))
    for key, values in selected.items():
        ordered = np.sort(values)
        survival = (len(ordered) - np.arange(len(ordered))) / len(ordered)
        mask = survival <= 0.25
        plt.plot(ordered[mask], survival[mask], marker=".", linewidth=1.0, markersize=3, label=key.removeprefix(prefix))
    plt.yscale("log")
    plt.xlabel("sys")
    plt.ylabel("empirical survival P(sys >= t)")
    plt.title(f"High-sys survival: {prefix.removesuffix(':')}")
    plt.legend(fontsize=7, ncol=2)
    plt.tight_layout()
    path = out_dir / "figures" / f"{prefix.removesuffix(':')}-survival.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    plt.savefig(path, dpi=180)
    plt.close()


def plot_endpoint_summary(endpoint_rows: list[dict[str, Any]], out_dir: Path) -> None:
    rows = [
        row
        for row in endpoint_rows
        if row.get("threshold_quantile") == 0.9 and isinstance(row.get("endpoint"), float)
    ]
    if not rows:
        return
    rows = sorted(rows, key=lambda row: row["bucket"])
    labels = [row["bucket"] for row in rows]
    endpoints = [float(row["endpoint"]) for row in rows]
    maxima = [float(row["max_sys"]) for row in rows]
    plt.figure(figsize=(10, 5))
    x = np.arange(len(rows))
    plt.scatter(x, endpoints, label="GPD finite endpoint estimate")
    plt.scatter(x, maxima, label="sample max", marker="x")
    plt.axhline(1.0, color="black", linestyle="--", linewidth=1.0, label="sys = 1")
    plt.xticks(x, labels, rotation=60, ha="right", fontsize=7)
    plt.ylabel("sys")
    plt.title("q90 GPD endpoint estimates by bucket")
    plt.legend(fontsize=8)
    plt.tight_layout()
    path = out_dir / "figures" / "endpoint-by-bucket.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    plt.savefig(path, dpi=180)
    plt.close()


def format_gap(value: Any) -> str:
    if value == "":
        return ""
    gap = float(value)
    if abs(gap) < 1e-9:
        return "best"
    if gap <= -20.0:
        return "no"
    return f"{gap:.2f}"


def write_readable_tail_table(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        handle.write("# High-sys tail fit table\n\n")
        handle.write(
            "Rows fit excesses `sys - u | sys > u`. `gap` is log2 likelihood "
            "ratio versus the best model for the same bucket/threshold. `no` "
            "means <= -20 bits.\n\n"
        )
        handle.write(
            "| bucket | q | u | tail n | model | shape | endpoint | P(sys>1) | samples/hit | gap |\n"
        )
        handle.write("| --- | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |\n")
        for row in rows:
            if row["status"] != "ok":
                continue
            endpoint = row["endpoint"]
            endpoint_text = endpoint if isinstance(endpoint, str) else f"{float(endpoint):.4f}"
            samples = row["samples_per_sys_gt_1"]
            samples_text = samples if isinstance(samples, str) else f"{float(samples):.3g}"
            handle.write(
                "| "
                + " | ".join(
                    [
                        str(row["bucket"]),
                        f"{float(row['threshold_quantile']):.2f}",
                        f"{float(row['threshold']):.4f}",
                        str(row["tail_n"]),
                        str(row["model"]),
                        f"{float(row['shape']):.3f}",
                        str(endpoint_text),
                        f"{float(row['prob_sys_gt_1']):.3g}",
                        str(samples_text),
                        format_gap(row.get("log2_likelihood_ratio_vs_best", "")),
                    ]
                )
                + " |\n"
            )


def main() -> None:
    args = parse_args()
    out_dir = args.out_dir
    groups = broad.load_groups(args.tables_dir)
    rng = np.random.default_rng(args.rng_seed)

    bucket_rows = []
    fit_rows = []
    endpoint_rows = []
    for bucket, values in groups.items():
        summary = {"bucket": bucket, "n": len(values), **quantile_summary(values)}
        summary["gap_from_one_at_max"] = float(1.0 - np.max(values))
        bucket_rows.append(summary)
        for threshold_quantile in args.threshold_quantiles:
            rows = fit_tail(bucket, values, threshold_quantile)
            fit_rows.extend(rows)
            gpd = next((row for row in rows if row["model"] == "gpd" and row["status"] == "ok"), None)
            if gpd is not None:
                boot = bootstrap_gpd_endpoint(values, threshold_quantile, rng, args.bootstrap_count)
                endpoint_rows.append(
                    {
                        **gpd,
                        "max_sys": float(np.max(values)),
                        **boot,
                    }
                )

    add_likelihood_gaps(fit_rows)
    plot_survival(groups, out_dir, "generic:")
    plot_survival(groups, out_dir, "product:")
    plot_endpoint_summary(endpoint_rows, out_dir)

    write_tsv(
        out_dir / "bucket-tail-summary.tsv",
        bucket_rows,
        ["bucket", "n", "q50", "q80", "q90", "q95", "q99", "q100", "gap_from_one_at_max"],
    )
    write_tsv(
        out_dir / "tail-fit-table.tsv",
        fit_rows,
        [
            "bucket",
            "threshold_quantile",
            "threshold",
            "tail_n",
            "model",
            "status",
            "shape",
            "scale",
            "endpoint",
            "prob_sys_gt_1",
            "samples_per_sys_gt_1",
            "log_likelihood",
            "per_row_log_likelihood",
            "delta_log_likelihood_vs_best",
            "log2_likelihood_ratio_vs_best",
            "error",
        ],
    )
    write_tsv(
        out_dir / "gpd-endpoint-bootstrap.tsv",
        endpoint_rows,
        [
            "bucket",
            "threshold_quantile",
            "threshold",
            "tail_n",
            "shape",
            "scale",
            "endpoint",
            "max_sys",
            "negative_shape_fraction",
            "finite_endpoint_count",
            "bootstrap_success_count",
            "endpoint_bootstrap_q05",
            "endpoint_bootstrap_median",
            "endpoint_bootstrap_q95",
        ],
    )
    write_readable_tail_table(out_dir / "tail-fit-readable.md", fit_rows)
    write_json(
        out_dir / "summary.json",
        {
            "status": "high_sys_tail_diagnostic",
            "tables_dir": str(args.tables_dir),
            "threshold_quantiles": args.threshold_quantiles,
            "bootstrap_count": args.bootstrap_count,
            "bucket_summary": str(out_dir / "bucket-tail-summary.tsv"),
            "tail_fit_table": str(out_dir / "tail-fit-table.tsv"),
            "tail_fit_readable": str(out_dir / "tail-fit-readable.md"),
            "gpd_endpoint_bootstrap": str(out_dir / "gpd-endpoint-bootstrap.tsv"),
            "figures": [
                str(out_dir / "figures" / "generic-survival.png"),
                str(out_dir / "figures" / "product-survival.png"),
                str(out_dir / "figures" / "endpoint-by-bucket.png"),
            ],
            "bucket_rows": bucket_rows,
            "fit_rows": fit_rows,
            "endpoint_rows": endpoint_rows,
        },
    )
    print("# high-sys-tail-diagnostic")
    print(f"- buckets: `{len(groups)}`")
    print(f"- thresholds: `{args.threshold_quantiles}`")
    print(f"- wrote `{out_dir / 'tail-fit-readable.md'}`")
    print(f"- wrote `{out_dir / 'gpd-endpoint-bootstrap.tsv'}`")


if __name__ == "__main__":
    main()
