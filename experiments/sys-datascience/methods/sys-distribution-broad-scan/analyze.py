#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scipy"]
# ///

"""Broad distribution scan for fixed-bucket sys marginals."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
from pathlib import Path
import re
import signal
import sys
import warnings
from typing import Any

import matplotlib.pyplot as plt
import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    dataset_label,
    load_trusted_random_tables,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


SCIPY_LOGIT_FAMILIES = [
    "alpha",
    "anglit",
    "beta",
    "betaprime",
    "burr",
    "burr12",
    "cauchy",
    "chi",
    "chi2",
    "cosine",
    "crystalball",
    "dgamma",
    "dweibull",
    "expon",
    "exponnorm",
    "exponpow",
    "exponweib",
    "f",
    "fatiguelife",
    "fisk",
    "gamma",
    "genexpon",
    "genextreme",
    "gengamma",
    "genlogistic",
    "gennorm",
    "genpareto",
    "gibrat",
    "gompertz",
    "gumbel_l",
    "gumbel_r",
    "halfcauchy",
    "halfgennorm",
    "halflogistic",
    "halfnorm",
    "hypsecant",
    "invgamma",
    "invgauss",
    "invweibull",
    "jf_skew_t",
    "johnsonsb",
    "johnsonsu",
    "kappa3",
    "kappa4",
    "ksone",
    "kstwo",
    "laplace",
    "laplace_asymmetric",
    "loggamma",
    "logistic",
    "loglaplace",
    "lognorm",
    "lomax",
    "maxwell",
    "mielke",
    "moyal",
    "nakagami",
    "norm",
    "norminvgauss",
    "pareto",
    "pearson3",
    "powerlaw",
    "powerlognorm",
    "powernorm",
    "rayleigh",
    "rdist",
    "recipinvgauss",
    "semicircular",
    "skewcauchy",
    "skewnorm",
    "t",
    "trapezoid",
    "triang",
    "truncexpon",
    "truncnorm",
    "tukeylambda",
    "uniform",
    "wald",
    "weibull_max",
    "weibull_min",
]


class Timeout(Exception):
    pass


def timeout_handler(signum: int, frame: Any) -> None:
    raise Timeout()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--max-fit-seconds", type=int, default=8)
    parser.add_argument("--rng-seed", type=int, default=20260625)
    parser.add_argument("--test-fraction", type=float, default=0.25)
    return parser.parse_args()


def safe_slug(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9_.-]+", "_", value).strip("_")


def bucket_label(row: dict[str, Any], provenance_rows: list[dict[str, Any]]) -> str:
    dataset = dataset_label(row, provenance_rows)
    if dataset == "random_product_sample":
        return f"product:{product_bucket(provenance_rows)}"
    return f"generic:F{row.get('facet_count')}"


def load_groups(tables_dir: Path) -> dict[str, np.ndarray]:
    polytope_rows, provenance_rows = load_trusted_random_tables(tables_dir)
    provenance = provenance_by_poly_id(provenance_rows)
    groups: dict[str, list[float]] = defaultdict(list)
    for row in polytope_rows:
        key = bucket_label(row, provenance.get(str(row["poly_id"]), []))
        groups[key].append(float(row["sys"]))
    return {key: np.array(values, dtype=float) for key, values in sorted(groups.items())}


def logit(values: np.ndarray) -> np.ndarray:
    clipped = np.clip(values, 1e-9, 1.0 - 1e-9)
    return np.log(clipped / (1.0 - clipped))


def transformed_logpdf(dist: Any, params: tuple[Any, ...], values: np.ndarray) -> np.ndarray:
    clipped = np.clip(values, 1e-9, 1.0 - 1e-9)
    y = logit(clipped)
    return dist.logpdf(y, *params) - np.log(clipped * (1.0 - clipped))


def transformed_cdf(dist: Any, params: tuple[Any, ...], values: np.ndarray) -> np.ndarray:
    return dist.cdf(logit(np.asarray(values)), *params)


def fit_one_family(name: str, train: np.ndarray, max_fit_seconds: int) -> dict[str, Any]:
    dist = getattr(stats, name)
    y_train = logit(train)
    signal.signal(signal.SIGALRM, timeout_handler)
    signal.alarm(max_fit_seconds)
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            params = dist.fit(y_train)
        signal.alarm(0)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            logpdf = transformed_logpdf(dist, params, train)
        if not np.all(np.isfinite(logpdf)):
            raise ValueError("non-finite train logpdf")
        return {
            "family": f"logit_{name}",
            "params": [float(x) for x in params],
            "fit_status": "ok",
        }
    except Timeout:
        signal.alarm(0)
        return {"family": f"logit_{name}", "fit_status": "timeout"}
    except Exception as exc:
        signal.alarm(0)
        return {"family": f"logit_{name}", "fit_status": "failed", "error": str(exc)[:240]}


def evaluate_fit(
    fit: dict[str, Any],
    train: np.ndarray,
    test: np.ndarray,
    all_values: np.ndarray,
) -> dict[str, Any]:
    name = fit["family"].removeprefix("logit_")
    dist = getattr(stats, name)
    params = tuple(fit["params"])
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        train_ll = float(np.mean(transformed_logpdf(dist, params, train)))
        test_ll = float(np.mean(transformed_logpdf(dist, params, test)))
        all_ll = float(np.mean(transformed_logpdf(dist, params, all_values)))
    sorted_values = np.sort(all_values)
    empirical = np.arange(1, len(sorted_values) + 1) / len(sorted_values)
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        fitted = transformed_cdf(dist, params, sorted_values)
    if not np.all(np.isfinite(fitted)):
        raise ValueError("non-finite fitted CDF")
    cdf_error = float(np.max(np.abs(empirical - fitted)))
    dkw95 = float(np.sqrt(np.log(2 / 0.05) / (2 * len(sorted_values))))
    return {
        **fit,
        "train_per_row_loglik": train_ll,
        "test_per_row_loglik": test_ll,
        "all_per_row_loglik": all_ll,
        "max_abs_cdf_error": cdf_error,
        "dkw_epsilon_95": dkw95,
        "cdf_error_over_dkw95": cdf_error / dkw95,
    }


def scan_group(
    values: np.ndarray,
    rng: np.random.Generator,
    test_fraction: float,
    max_fit_seconds: int,
) -> dict[str, Any]:
    n = len(values)
    test_count = max(1, int(round(n * test_fraction)))
    perm = rng.permutation(n)
    test = values[perm[:test_count]]
    train = values[perm[test_count:]]
    ok = []
    failures = []
    for name in SCIPY_LOGIT_FAMILIES:
        fit = fit_one_family(name, train, max_fit_seconds)
        if fit["fit_status"] == "ok":
            try:
                ok.append(evaluate_fit(fit, train, test, values))
            except Exception as exc:
                failures.append(
                    {
                        "family": fit["family"],
                        "fit_status": "failed_evaluation",
                        "error": str(exc)[:240],
                    }
                )
        else:
            failures.append(fit)
    ranked = sorted(ok, key=lambda row: row["test_per_row_loglik"], reverse=True)
    return {
        "rows": n,
        "train_rows": len(train),
        "test_rows": len(test),
        "fit_count": len(ok),
        "failure_count": len(failures),
        "all_fit_evaluations": sorted(ok, key=lambda row: row["family"]),
        "top_by_test_loglik": ranked[:15],
        "top_by_cdf_error": sorted(ok, key=lambda row: row["max_abs_cdf_error"])[:15],
        "failures": failures,
    }


def write_screening_tables(out_dir: Path, summary: dict[str, Any]) -> dict[str, str]:
    buckets = list(summary["groups"])
    rows = []
    matrix_rows = []
    for family in summary["candidate_families"]:
        technical_failures = []
        cdf_failures = []
        weak_fits = []
        status_cells = []
        for bucket in buckets:
            item = summary["groups"][bucket]
            failure = next((row for row in item["failures"] if row["family"] == family), None)
            fit = next(
                (
                    row
                    for row in item["all_fit_evaluations"]
                    if row["family"] == family
                ),
                None,
            )
            if failure is not None:
                technical_failures.append(bucket)
                status_cells.append("technical_fail")
                continue
            if fit is None:
                technical_failures.append(bucket)
                status_cells.append("missing")
                continue
            ratio = float(fit["cdf_error_over_dkw95"])
            if ratio > 1.0:
                cdf_failures.append(bucket)
                status_cells.append(f"cdf_fail:{ratio:.3g}")
            elif ratio > 0.75:
                weak_fits.append(bucket)
                status_cells.append(f"weak:{ratio:.3g}")
            else:
                status_cells.append(f"ok:{ratio:.3g}")
        rows.append(
            {
                "family": family,
                "technical_failure_count": len(technical_failures),
                "cdf_failure_count": len(cdf_failures),
                "weak_fit_count": len(weak_fits),
                "technical_failure_buckets": ", ".join(technical_failures),
                "cdf_failure_buckets": ", ".join(cdf_failures),
                "weak_fit_buckets": ", ".join(weak_fits),
            }
        )
        matrix_rows.append([family, *status_cells])

    out_dir.mkdir(parents=True, exist_ok=True)
    summary_tsv = out_dir / "family-screening-summary.tsv"
    with summary_tsv.open("w") as handle:
        handle.write(
            "\t".join(
                [
                    "family",
                    "technical_failure_count",
                    "cdf_failure_count",
                    "weak_fit_count",
                    "technical_failure_buckets",
                    "cdf_failure_buckets",
                    "weak_fit_buckets",
                ]
            )
        )
        handle.write("\n")
        for row in rows:
            handle.write(
                "\t".join(
                    [
                        str(row["family"]),
                        str(row["technical_failure_count"]),
                        str(row["cdf_failure_count"]),
                        str(row["weak_fit_count"]),
                        str(row["technical_failure_buckets"]),
                        str(row["cdf_failure_buckets"]),
                        str(row["weak_fit_buckets"]),
                    ]
                )
            )
            handle.write("\n")

    matrix_tsv = out_dir / "family-bucket-screening-matrix.tsv"
    with matrix_tsv.open("w") as handle:
        handle.write("\t".join(["family", *buckets]))
        handle.write("\n")
        for row in matrix_rows:
            handle.write("\t".join(row))
            handle.write("\n")

    markdown = out_dir / "family-screening-summary.md"
    sorted_rows = sorted(
        rows,
        key=lambda row: (
            -row["cdf_failure_count"],
            -row["technical_failure_count"],
            -row["weak_fit_count"],
            row["family"],
        ),
    )
    with markdown.open("w") as handle:
        handle.write("# Family screening summary\n\n")
        handle.write(
            "This is an exploratory screening table, not a formal rule-out table. "
            "`cdf_flag` means max ECDF/CDF error exceeds the bucket's 95% DKW band "
            "in this all-data diagnostic. "
            "Weak means the error is within the band but above 75% of the band. "
            "Technical failure means SciPy fit/evaluation failed or timed out.\n\n"
        )
        handle.write(
            "| family | technical fails | CDF flags | weak fits | CDF-flag buckets | weak buckets |\n"
        )
        handle.write("| --- | ---: | ---: | ---: | --- | --- |\n")
        for row in sorted_rows:
            handle.write(
                "| "
                + " | ".join(
                    [
                        row["family"],
                        str(row["technical_failure_count"]),
                        str(row["cdf_failure_count"]),
                        str(row["weak_fit_count"]),
                        row["cdf_failure_buckets"] or "-",
                        row["weak_fit_buckets"] or "-",
                    ]
                )
                + " |\n"
            )

    return {
        "family_screening_summary_tsv": str(summary_tsv),
        "family_bucket_screening_matrix_tsv": str(matrix_tsv),
        "family_screening_summary_markdown": str(markdown),
    }


def plot_overview(groups: dict[str, np.ndarray], summary_groups: dict[str, Any], out_path: Path) -> None:
    cols = 2
    rows = (len(groups) + cols - 1) // cols
    fig, axes = plt.subplots(rows, cols, figsize=(12, 3.3 * rows), dpi=160)
    axes_flat = np.ravel(axes)
    for ax, (key, values) in zip(axes_flat, groups.items()):
        sorted_values = np.sort(values)
        empirical = np.arange(1, len(sorted_values) + 1) / len(sorted_values)
        best = summary_groups[key]["top_by_test_loglik"][0]
        name = best["family"].removeprefix("logit_")
        dist = getattr(stats, name)
        params = tuple(best["params"])
        grid = np.linspace(0.0, 1.0, 600)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            cdf = transformed_cdf(dist, params, grid)
        dkw = best["dkw_epsilon_95"]
        ax.step(sorted_values, empirical, where="post", color="#4c78a8", linewidth=1.0)
        ax.plot(grid, cdf, color="#f58518", linewidth=1.1)
        ax.fill_between(
            grid,
            np.maximum(0.0, cdf - dkw),
            np.minimum(1.0, cdf + dkw),
            color="#f58518",
            alpha=0.13,
        )
        ax.set_title(f"{key}: {best['family']}", fontsize=8)
        ax.set_xlim(0.0, 1.0)
        ax.set_ylim(0.0, 1.0)
        ax.grid(alpha=0.15, linewidth=0.5)
    for ax in axes_flat[len(groups):]:
        ax.axis("off")
    fig.tight_layout()
    out_path.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(out_path)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    rng = np.random.default_rng(args.rng_seed)
    groups = load_groups(args.tables_dir)
    summary = {
        "status": "broad_post_hoc_logit_scipy_scan",
        "tables_dir": str(args.tables_dir),
        "row_count": int(sum(len(values) for values in groups.values())),
        "group_count": len(groups),
        "candidate_family_count": len(SCIPY_LOGIT_FAMILIES),
        "candidate_families": [f"logit_{name}" for name in SCIPY_LOGIT_FAMILIES],
        "max_fit_seconds": args.max_fit_seconds,
        "rng_seed": args.rng_seed,
        "test_fraction": args.test_fraction,
        "groups": {},
    }
    for key, values in groups.items():
        summary["groups"][key] = scan_group(
            values,
            rng=rng,
            test_fraction=args.test_fraction,
            max_fit_seconds=args.max_fit_seconds,
        )
    generic_groups = {key: groups[key] for key in groups if key.startswith("generic:")}
    product_groups = {key: groups[key] for key in groups if key.startswith("product:")}
    plot_overview(
        generic_groups,
        summary["groups"],
        args.out_dir / "overview" / "generic-broad-scan-ecdf.png",
    )
    plot_overview(
        product_groups,
        summary["groups"],
        args.out_dir / "overview" / "product-broad-scan-ecdf.png",
    )
    summary["overview_figures"] = {
        "generic": str(args.out_dir / "overview" / "generic-broad-scan-ecdf.png"),
        "product": str(args.out_dir / "overview" / "product-broad-scan-ecdf.png"),
    }
    summary["screening_tables"] = write_screening_tables(args.out_dir / "tables", summary)
    write_json(args.out_dir / "summary.json", summary)

    print("# sys-distribution-broad-scan")
    print()
    print(f"- rows: `{summary['row_count']}`")
    print(f"- groups: `{summary['group_count']}`")
    print(f"- candidate families: `{summary['candidate_family_count']}`")
    print()
    print("| group | fit ok/fail | best held-out | second | best-second | best CDF error / DKW95 |")
    print("| --- | ---: | --- | --- | ---: | ---: |")
    for key, item in summary["groups"].items():
        top = item["top_by_test_loglik"]
        best = top[0]
        second = top[1]
        print(
            "| "
            + " | ".join(
                [
                    key,
                    f"{item['fit_count']}/{item['failure_count']}",
                    best["family"],
                    second["family"],
                    f"{best['test_per_row_loglik'] - second['test_per_row_loglik']:.4g}",
                    f"{best['max_abs_cdf_error']:.4g}/{best['dkw_epsilon_95']:.4g}",
                ]
            )
            + " |"
        )
    print()
    print(f"Wrote `{args.out_dir / 'summary.json'}`")
    print(f"Wrote `{summary['overview_figures']['generic']}`")
    print(f"Wrote `{summary['overview_figures']['product']}`")
    print(f"Wrote `{summary['screening_tables']['family_screening_summary_markdown']}`")


if __name__ == "__main__":
    main()
