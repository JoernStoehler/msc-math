#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scipy"]
# ///

"""All-data MLE likelihood table for transformed candidate sys marginal families."""

from __future__ import annotations

import argparse
import importlib.util
import json
import multiprocessing as mp
from pathlib import Path
import sys
import traceback
import warnings
from typing import Any

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
    parser.add_argument("--max-fit-seconds", type=float, default=3.0)
    parser.add_argument("--distributions", nargs="+", default=broad.SCIPY_LOGIT_FAMILIES)
    parser.add_argument(
        "--transforms",
        nargs="+",
        default=[
            "identity",
            "logit",
            "log",
            "log1m",
            "neglog1m",
            "sqrt",
            "square",
            "cloglog",
        ],
    )
    return parser.parse_args()


def transform_values(transform: str, array: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    if transform == "identity":
        return array, np.zeros_like(array)
    if transform == "logit":
        y = broad.logit(array)
        log_abs_derivative = -np.log(array) - np.log1p(-array)
        return y, log_abs_derivative
    if transform == "log":
        return np.log(array), -np.log(array)
    if transform == "log1m":
        return np.log1p(-array), -np.log1p(-array)
    if transform == "neglog1m":
        return -np.log1p(-array), -np.log1p(-array)
    if transform == "sqrt":
        return np.sqrt(array), -np.log(2.0) - 0.5 * np.log(array)
    if transform == "square":
        return array * array, np.log(2.0) + np.log(array)
    if transform == "cloglog":
        tail = -np.log1p(-array)
        return np.log(tail), -np.log1p(-array) - np.log(tail)
    raise ValueError(f"unknown transform: {transform}")


def fit_worker(transform: str, distribution: str, values: list[float], queue: mp.Queue) -> None:
    try:
        array = np.array(values, dtype=float)
        dist = getattr(stats, distribution)
        y, log_abs_derivative = transform_values(transform, array)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            params = dist.fit(y)
            logpdf = dist.logpdf(y, *params[:-2], loc=params[-2], scale=params[-1])
            logpdf = logpdf + log_abs_derivative
        if not np.all(np.isfinite(logpdf)):
            raise ValueError("non-finite logpdf")
        queue.put(
            {
                "model": f"{transform}+{distribution}",
                "transform": transform,
                "distribution": distribution,
                "status": "ok",
                "params": [float(x) for x in params],
                "parameter_count": len(params),
                "log_likelihood": float(np.sum(logpdf)),
                "per_row_log_likelihood": float(np.mean(logpdf)),
            }
        )
    except Exception as exc:
        queue.put(
            {
                "model": f"{transform}+{distribution}",
                "transform": transform,
                "distribution": distribution,
                "status": "failed",
                "error": str(exc)[:300],
                "traceback_tail": traceback.format_exc()[-800:],
            }
        )


def fit_with_timeout(
    transform: str,
    distribution: str,
    values: np.ndarray,
    timeout_seconds: float,
) -> dict[str, Any]:
    ctx = mp.get_context("fork")
    queue: mp.Queue = ctx.Queue()
    process = ctx.Process(target=fit_worker, args=(transform, distribution, values.tolist(), queue))
    process.start()
    process.join(timeout_seconds)
    if process.is_alive():
        process.terminate()
        process.join(1.0)
        return {
            "model": f"{transform}+{distribution}",
            "transform": transform,
            "distribution": distribution,
            "status": "timeout",
        }
    if queue.empty():
        return {
            "model": f"{transform}+{distribution}",
            "transform": transform,
            "distribution": distribution,
            "status": "failed",
            "error": f"worker exited with code {process.exitcode} without result",
        }
    return queue.get()


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


def median(values: list[float]) -> float:
    ordered = sorted(values)
    middle = len(ordered) // 2
    if len(ordered) % 2 == 1:
        return ordered[middle]
    return 0.5 * (ordered[middle - 1] + ordered[middle])


def build_model_summaries(
    rows: list[dict[str, Any]],
    buckets: list[str],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    by_model: dict[str, list[dict[str, Any]]] = {}
    for row in rows:
        by_model.setdefault(row["model"], []).append(row)

    model_summaries = []
    gap_matrix = []
    for model, model_rows in sorted(by_model.items()):
        ok_rows = [row for row in model_rows if row["status"] == "ok"]
        gaps = [float(row["log2_likelihood_ratio_vs_best"]) for row in ok_rows]
        best_buckets = [
            row["bucket"]
            for row in ok_rows
            if abs(float(row["log2_likelihood_ratio_vs_best"])) < 1e-12
        ]
        non_ok = [row for row in model_rows if row["status"] != "ok"]
        transform = model_rows[0]["transform"]
        distribution = model_rows[0]["distribution"]
        summary = {
            "model": model,
            "transform": transform,
            "distribution": distribution,
            "ok_bucket_count": len(ok_rows),
            "non_ok_bucket_count": len(non_ok),
            "best_bucket_count": len(best_buckets),
            "best_buckets": best_buckets,
            "non_ok_buckets": [row["bucket"] for row in non_ok],
            "non_ok_statuses": {
                row["bucket"]: row["status"]
                for row in non_ok
            },
        }
        if gaps:
            summary.update(
                {
                    "worst_log2_ratio_vs_best": min(gaps),
                    "median_log2_ratio_vs_best": median(gaps),
                    "best_log2_ratio_vs_best": max(gaps),
                }
            )
        model_summaries.append(summary)

        matrix_row = {"model": model}
        by_bucket = {row["bucket"]: row for row in model_rows}
        for bucket in buckets:
            row = by_bucket.get(bucket)
            if row is None:
                matrix_row[bucket] = ""
            elif row["status"] == "ok":
                matrix_row[bucket] = row["log2_likelihood_ratio_vs_best"]
            else:
                matrix_row[bucket] = row["status"]
        gap_matrix.append(matrix_row)

    model_summaries.sort(
        key=lambda row: (
            -row["best_bucket_count"],
            -row["ok_bucket_count"],
            -row["worst_log2_ratio_vs_best"] if row["ok_bucket_count"] else float("inf"),
            -row["median_log2_ratio_vs_best"] if row["ok_bucket_count"] else float("inf"),
            row["model"],
        )
    )
    return model_summaries, gap_matrix


def readable_gap(value: str) -> str:
    if value in ("failed", "timeout", ""):
        return "non-ok" if value else ""
    gap = float(value)
    if abs(gap) < 1e-9:
        return "best"
    if gap <= -20.0:
        return "no"
    if gap > -0.05:
        return "~best"
    if gap > -1.0:
        return f"{gap:.2f}"
    return f"{gap:.1f}"


def write_readable_tables(
    out_dir: Path,
    model_summaries: list[dict[str, Any]],
    gap_matrix: list[dict[str, Any]],
    buckets: list[str],
) -> tuple[Path, Path]:
    out_dir.mkdir(parents=True, exist_ok=True)
    summary_path = out_dir / "model-readable-summary.md"
    matrix_path = out_dir / "model-bucket-readable-gap-matrix.md"
    with summary_path.open("w") as handle:
        handle.write("# Model readable summary\n\n")
        handle.write(
            "`model` is `transform+distribution`. Counts use all-data MLE "
            "likelihood on the original `sys` scale. `<=5 bits` counts buckets "
            "within 5 log2 units of the bucket winner; `<=20 bits` is a loose "
            "not-catastrophically-dominated count, not a truth threshold.\n\n"
        )
        handle.write(
            "| model | wins | <=5 bits | <=20 bits | no/non-ok buckets | worst shown status |\n"
        )
        handle.write("| --- | ---: | ---: | ---: | ---: | --- |\n")
        by_model = {row["model"]: row for row in gap_matrix}
        for summary in model_summaries:
            row = by_model[summary["model"]]
            gaps = []
            no_or_non_ok = 0
            for bucket in buckets:
                value = row[bucket]
                if value in ("failed", "timeout", ""):
                    no_or_non_ok += 1
                    continue
                gap = float(value)
                gaps.append(gap)
                if gap <= -20.0:
                    no_or_non_ok += 1
            if not gaps:
                status = "all non-ok"
                wins = within5 = within20 = 0
            else:
                wins = sum(abs(gap) < 1e-9 for gap in gaps)
                within5 = sum(gap >= -5.0 for gap in gaps)
                within20 = sum(gap > -20.0 for gap in gaps)
                worst = min(gaps)
                if worst <= -20.0:
                    status = "has no cells"
                elif worst <= -5.0:
                    status = f"worst {worst:.1f}"
                elif worst < 0.0:
                    status = f"worst {worst:.2f}"
                else:
                    status = "all best"
            if wins or within5 or within20:
                handle.write(
                    "| "
                    + " | ".join(
                        [
                            str(summary["model"]),
                            str(wins),
                            str(within5),
                            str(within20),
                            str(no_or_non_ok),
                            status,
                        ]
                    )
                    + " |\n"
                )

    with matrix_path.open("w") as handle:
        handle.write("# Model-bucket readable likelihood-gap matrix\n\n")
        handle.write(
            "Cells are log2 MLE likelihood ratios versus the bucket winner. "
            "`best` means this model wins the bucket. `~best` means within "
            "0.05 bits. `no` means at most `2^-20` as likely as the bucket "
            "winner at MLE, so the exact huge negative number is hidden. "
            "`non-ok` is optimizer/timeout failure.\n\n"
        )
        handle.write("| model | " + " | ".join(buckets) + " |\n")
        handle.write("| --- | " + " | ".join(["---:"] * len(buckets)) + " |\n")
        for summary in model_summaries:
            row = next(item for item in gap_matrix if item["model"] == summary["model"])
            cells = [readable_gap(str(row[bucket])) for bucket in buckets]
            if any(cell not in ("no", "non-ok", "") for cell in cells):
                handle.write("| " + " | ".join([str(row["model"]), *cells]) + " |\n")
    return summary_path, matrix_path


def main() -> None:
    args = parse_args()
    groups = broad.load_groups(args.tables_dir)
    all_rows: list[dict[str, Any]] = []
    bucket_summaries: list[dict[str, Any]] = []
    candidate_models = [
        (transform, distribution)
        for transform in args.transforms
        for distribution in args.distributions
    ]
    for bucket_index, (bucket, values) in enumerate(groups.items(), start=1):
        print(f"Fitting bucket {bucket_index}/{len(groups)}: {bucket} ({len(values)} rows)", flush=True)
        bucket_rows = []
        for transform, distribution in candidate_models:
            result = fit_with_timeout(transform, distribution, values, args.max_fit_seconds)
            row = {
                "bucket": bucket,
                "n": len(values),
                **result,
            }
            bucket_rows.append(row)
        ok_rows = [row for row in bucket_rows if row["status"] == "ok"]
        best_log_likelihood = max(row["log_likelihood"] for row in ok_rows) if ok_rows else None
        best_model = None
        if best_log_likelihood is not None:
            for row in bucket_rows:
                if row["status"] == "ok":
                    delta = row["log_likelihood"] - best_log_likelihood
                    row["delta_log_likelihood_vs_best"] = float(delta)
                    row["log2_likelihood_ratio_vs_best"] = float(delta / np.log(2.0))
                else:
                    row["delta_log_likelihood_vs_best"] = ""
                    row["log2_likelihood_ratio_vs_best"] = ""
            best_model = max(ok_rows, key=lambda row: row["log_likelihood"])["model"]
        all_rows.extend(
            sorted(
                bucket_rows,
                key=lambda row: (
                    row["status"] != "ok",
                    -float(row.get("log_likelihood", float("-inf")))
                    if row["status"] == "ok"
                    else 0.0,
                    row["model"],
                ),
            )
        )
        reference_gaps = {}
        for reference in [
            "logit+beta",
            "logit+norm",
            "logit+logistic",
            "logit+johnsonsu",
            "logit+jf_skew_t",
            "logit+norminvgauss",
            "neglog1m+gamma",
            "neglog1m+lognorm",
            "square+beta",
            "identity+beta",
        ]:
            match = next((row for row in bucket_rows if row["model"] == reference), None)
            if match is not None and match["status"] == "ok":
                reference_gaps[f"{reference}_log2_ratio_vs_best"] = match[
                    "log2_likelihood_ratio_vs_best"
                ]
        bucket_summaries.append(
            {
                "bucket": bucket,
                "n": len(values),
                "ok_count": len(ok_rows),
                "non_ok_count": len(bucket_rows) - len(ok_rows),
                "best_model": best_model or "",
                "best_log_likelihood": best_log_likelihood if best_log_likelihood is not None else "",
                **reference_gaps,
            }
        )
        print(
            f"Finished {bucket}: {len(ok_rows)} ok, {len(bucket_rows) - len(ok_rows)} non-ok, "
            f"best={best_model}",
            flush=True,
        )

    out_dir = args.out_dir
    buckets = list(groups.keys())
    model_summaries, gap_matrix = build_model_summaries(all_rows, buckets)

    table_path = out_dir / "mle-likelihood-table.tsv"
    bucket_summary_path = out_dir / "bucket-best-summary.tsv"
    model_summary_path = out_dir / "model-summary.tsv"
    gap_matrix_path = out_dir / "model-bucket-log2-gap-matrix.tsv"
    readable_summary_path, readable_matrix_path = write_readable_tables(
        out_dir,
        model_summaries,
        gap_matrix,
        buckets,
    )
    write_tsv(
        table_path,
        all_rows,
        [
            "bucket",
            "n",
            "model",
            "transform",
            "distribution",
            "status",
            "log_likelihood",
            "per_row_log_likelihood",
            "delta_log_likelihood_vs_best",
            "log2_likelihood_ratio_vs_best",
            "parameter_count",
            "params",
            "error",
        ],
    )
    write_tsv(
        bucket_summary_path,
        bucket_summaries,
        [
            "bucket",
            "n",
            "ok_count",
            "non_ok_count",
            "best_model",
            "best_log_likelihood",
            "identity+beta_log2_ratio_vs_best",
            "logit+beta_log2_ratio_vs_best",
            "square+beta_log2_ratio_vs_best",
            "logit+norm_log2_ratio_vs_best",
            "logit+logistic_log2_ratio_vs_best",
            "logit+johnsonsu_log2_ratio_vs_best",
            "logit+jf_skew_t_log2_ratio_vs_best",
            "logit+norminvgauss_log2_ratio_vs_best",
            "neglog1m+gamma_log2_ratio_vs_best",
            "neglog1m+lognorm_log2_ratio_vs_best",
        ],
    )
    write_tsv(
        model_summary_path,
        model_summaries,
        [
            "model",
            "transform",
            "distribution",
            "ok_bucket_count",
            "non_ok_bucket_count",
            "best_bucket_count",
            "worst_log2_ratio_vs_best",
            "median_log2_ratio_vs_best",
            "best_log2_ratio_vs_best",
            "best_buckets",
            "non_ok_buckets",
            "non_ok_statuses",
        ],
    )
    write_tsv(
        gap_matrix_path,
        gap_matrix,
        ["model", *buckets],
    )
    write_json(
        out_dir / "summary.json",
        {
            "status": "all_data_approximate_mle_transformed_likelihood_table",
            "tables_dir": str(args.tables_dir),
            "row_count": int(sum(len(values) for values in groups.values())),
            "group_count": len(groups),
            "transform_count": len(args.transforms),
            "distribution_count": len(args.distributions),
            "candidate_model_count": len(candidate_models),
            "transforms": args.transforms,
            "max_fit_seconds": args.max_fit_seconds,
            "table": str(table_path),
            "bucket_best_summary": str(bucket_summary_path),
            "model_summary": str(model_summary_path),
            "model_bucket_log2_gap_matrix": str(gap_matrix_path),
            "model_readable_summary": str(readable_summary_path),
            "model_bucket_readable_gap_matrix": str(readable_matrix_path),
            "bucket_summaries": bucket_summaries,
            "model_summaries": model_summaries,
        },
    )

    print("# sys-distribution-mle-likelihood-table")
    print()
    print(f"- rows: `{sum(len(values) for values in groups.values())}`")
    print(f"- groups: `{len(groups)}`")
    print(f"- transforms: `{len(args.transforms)}`")
    print(f"- distributions: `{len(args.distributions)}`")
    print(f"- candidate models: `{len(candidate_models)}`")
    print()
    print("| bucket | ok/non-ok | best model | best log likelihood | log2 logit+beta/best |")
    print("| --- | ---: | --- | ---: | ---: |")
    for row in bucket_summaries:
        beta_gap = row.get("logit+beta_log2_ratio_vs_best", "")
        print(
            "| "
            + " | ".join(
                [
                    str(row["bucket"]),
                    f"{row['ok_count']}/{row['non_ok_count']}",
                    str(row["best_model"]),
                    f"{float(row['best_log_likelihood']):.6g}"
                    if row["best_log_likelihood"] != ""
                    else "",
                    f"{float(beta_gap):.6g}" if beta_gap != "" else "",
                ]
            )
            + " |"
        )
    print()
    print(f"Wrote `{table_path}`")
    print(f"Wrote `{bucket_summary_path}`")
    print(f"Wrote `{model_summary_path}`")
    print(f"Wrote `{gap_matrix_path}`")
    print(f"Wrote `{readable_summary_path}`")
    print(f"Wrote `{readable_matrix_path}`")


if __name__ == "__main__":
    main()
