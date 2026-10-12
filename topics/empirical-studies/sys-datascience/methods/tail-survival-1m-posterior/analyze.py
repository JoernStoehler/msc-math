#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Tail-survival and 1M hit-probability sensitivity for retained random/product rows."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
import math
from pathlib import Path
import sys
import warnings
from typing import Any

import numpy as np
from scipy import stats

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

RUN_SIZES = [10_000, 100_000, 1_000_000]
SURVIVAL_THRESHOLDS = [0.5, 0.6, 0.7, 0.75, 0.8, 0.85, 0.8625, 0.9, 0.95, 1.0]
TAIL_QUANTILES = [0.8, 0.9, 0.95]
DENSE_TAIL_QUANTILES = [
    0.75,
    0.76,
    0.77,
    0.78,
    0.79,
    0.8,
    0.81,
    0.82,
    0.83,
    0.84,
    0.85,
    0.86,
    0.87,
    0.88,
    0.89,
    0.9,
    0.91,
    0.92,
    0.93,
    0.94,
    0.95,
    0.96,
    0.97,
    0.98,
    0.99,
]
BACKTEST_FIT_QUANTILES = [0.75, 0.8, 0.85]
BACKTEST_TEST_QUANTILES = [0.9, 0.95, 0.99]
ZERO_POSITIVE_PRIORS = {
    "jeffreys_beta_0.5_0.5": (0.5, 0.5),
    "uniform_beta_1_1": (1.0, 1.0),
    "pessimistic_beta_0.1_1000": (0.1, 1000.0),
    "skeptical_beta_0.01_10000": (0.01, 10000.0),
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--bootstrap-count", type=int, default=160)
    parser.add_argument("--rng-seed", type=int, default=20260630)
    return parser.parse_args()


def display_path(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(Path.cwd().resolve()))
    except ValueError:
        return str(path)


def write_tsv(path: Path, rows: list[dict[str, Any]], columns: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        handle.write("\t".join(columns))
        handle.write("\n")
        for row in rows:
            values: list[str] = []
            for column in columns:
                value = row.get(column, "")
                if isinstance(value, list | dict):
                    value = json.dumps(value, sort_keys=True, separators=(",", ":"))
                elif isinstance(value, float):
                    value = f"{value:.17g}"
                values.append(str(value))
            handle.write("\t".join(values))
            handle.write("\n")


def quantile(values: list[float], q: float) -> float:
    return float(np.quantile(np.array(values, dtype=float), q))


def bucket_label(row: dict[str, Any], provenance_rows: list[dict[str, Any]]) -> str:
    dataset = dataset_label(row, provenance_rows)
    if dataset == "random_product_sample":
        return f"product:{product_bucket(provenance_rows)}"
    return f"generic:F{int(row['facet_count'])}"


def load_rows(tables_dir: Path) -> tuple[list[dict[str, Any]], dict[str, list[float]]]:
    polytope_rows, provenance_rows = load_trusted_random_tables(tables_dir)
    provenance = provenance_by_poly_id(provenance_rows)
    groups: dict[str, list[float]] = defaultdict(list)
    groups["pooled:all"] = []
    for row in polytope_rows:
        sys_value = float(row["sys"])
        poly_provenance = provenance.get(str(row["poly_id"]), [])
        dataset = dataset_label(row, poly_provenance)
        fixed_bucket = bucket_label(row, poly_provenance)
        groups["pooled:all"].append(sys_value)
        groups[f"dataset:{dataset}"].append(sys_value)
        groups[fixed_bucket].append(sys_value)
    return polytope_rows, {key: values for key, values in sorted(groups.items())}


def anchor_summary(polytope_rows: list[dict[str, Any]]) -> dict[str, Any]:
    values = [float(row["sys"]) for row in polytope_rows]
    return {
        "row_count": len(values),
        "positives_sys_gt_1": sum(1 for value in values if value > 1.0),
        "max_sys": max(values),
        "p99_sys": quantile(values, 0.99),
    }


def predictive_at_least_one(point_probability: float, run_size: int) -> float:
    if point_probability <= 0.0:
        return 0.0
    if point_probability >= 1.0:
        return 1.0
    return float(-math.expm1(run_size * math.log1p(-point_probability)))


def beta_binomial_predictive(alpha: float, beta: float, failures: int, run_size: int) -> float:
    log_no_hit = (
        math.lgamma(beta + failures + run_size)
        + math.lgamma(alpha + beta + failures)
        - math.lgamma(alpha + beta + failures + run_size)
        - math.lgamma(beta + failures)
    )
    return float(1.0 - math.exp(log_no_hit))


def beta_posterior_mean(alpha: float, beta: float, failures: int) -> float:
    return float(alpha / (alpha + beta + failures))


def survival_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum, values in groups.items():
        for threshold in SURVIVAL_THRESHOLDS:
            exceedances = sum(1 for value in values if value > threshold)
            rows.append(
                {
                    "stratum": stratum,
                    "n": len(values),
                    "threshold": threshold,
                    "exceedances_sys_gt_threshold": exceedances,
                    "empirical_survival": exceedances / len(values),
                    "max_sys": max(values),
                    "q99_sys": quantile(values, 0.99),
                }
            )
    return rows


def binomial_wilson_interval(successes: int, trials: int, confidence: float = 0.95) -> tuple[float, float]:
    if trials <= 0:
        return (float("nan"), float("nan"))
    estimate = successes / trials
    z = float(stats.norm.ppf(0.5 + confidence / 2.0))
    denominator = 1.0 + z * z / trials
    center = (estimate + z * z / (2.0 * trials)) / denominator
    half_width = (
        z
        * math.sqrt(estimate * (1.0 - estimate) / trials + z * z / (4.0 * trials * trials))
        / denominator
    )
    return (max(0.0, center - half_width), min(1.0, center + half_width))


def dense_threshold_grid(values: list[float], high_quantile: float = 0.75) -> list[float]:
    array = np.array(values, dtype=float)
    low = float(np.quantile(array, high_quantile))
    high = float(np.max(array))
    grid = {1.0, low, high}
    if high > low:
        grid.update(float(value) for value in np.linspace(low, high, 160))
    grid.update(float(value) for value in array[array >= low])
    return sorted(grid)


def dense_empirical_survival_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = groups[stratum]
        sorted_values = np.sort(np.array(values, dtype=float))
        for threshold in dense_threshold_grid(values):
            exceedances = int(len(sorted_values) - np.searchsorted(sorted_values, threshold, side="right"))
            lower, upper = binomial_wilson_interval(exceedances, len(sorted_values))
            rows.append(
                {
                    "stratum": stratum,
                    "n": len(sorted_values),
                    "threshold": threshold,
                    "exceedances_sys_gt_threshold": exceedances,
                    "empirical_survival": exceedances / len(sorted_values),
                    "wilson_95_lower": lower,
                    "wilson_95_upper": upper,
                    "threshold_source": "observed_plus_grid",
                    "high_quantile": 0.75,
                    "max_sys": float(np.max(sorted_values)),
                }
            )
    return rows


def top_k_spacing_rows(groups: dict[str, list[float]], top_k: int = 30) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = np.sort(np.array(groups[stratum], dtype=float))[::-1]
        for rank, value in enumerate(values[:top_k], start=1):
            next_lower = float(values[rank]) if rank < len(values) else ""
            rows.append(
                {
                    "stratum": stratum,
                    "n": len(values),
                    "rank_desc": rank,
                    "sys": float(value),
                    "gap_from_one": float(1.0 - value),
                    "gap_to_next_lower": float(value - next_lower) if isinstance(next_lower, float) else "",
                    "gap_to_record": float(values[0] - value),
                    "empirical_tail_mass_at_or_above_rank": rank / len(values),
                }
            )
    return rows


def support_limited_future_max_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = np.array(groups[stratum], dtype=float)
        observed_max = float(np.max(values))
        top_multiplicity = int(np.sum(values == observed_max))
        top_mass = top_multiplicity / len(values)
        for run_size in RUN_SIZES:
            rows.append(
                {
                    "stratum": stratum,
                    "n": len(values),
                    "run_size": run_size,
                    "observed_max": observed_max,
                    "top_multiplicity": top_multiplicity,
                    "empirical_top_mass": top_mass,
                    "probability_rehit_observed_max": predictive_at_least_one(top_mass, run_size),
                    "support_limited_can_exceed_observed_max": "no",
                }
            )
    return rows


def zero_positive_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum, values in groups.items():
        positives = sum(1 for value in values if value > 1.0)
        if positives != 0:
            continue
        for prior_name, (alpha, beta) in ZERO_POSITIVE_PRIORS.items():
            posterior_mean = beta_posterior_mean(alpha, beta, len(values))
            for run_size in RUN_SIZES:
                rows.append(
                    {
                        "stratum": stratum,
                        "n": len(values),
                        "model": prior_name,
                        "posterior_mean_p_sys_gt_1": posterior_mean,
                        "run_size": run_size,
                        "posterior_predictive_at_least_one": beta_binomial_predictive(
                            alpha, beta, len(values), run_size
                        ),
                    }
                )
    return rows


def generic_f_density_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in sorted(key for key in groups if key.startswith("generic:F")):
        values = groups[stratum]
        facet_count = int(stratum.removeprefix("generic:F"))
        positives = sum(1 for value in values if value > 1.0)
        for prior_name, (alpha, beta) in ZERO_POSITIVE_PRIORS.items():
            rows.append(
                {
                    "facet_count": facet_count,
                    "stratum": stratum,
                    "n": len(values),
                    "positives_sys_gt_1": positives,
                    "model": prior_name,
                    "posterior_mean_p_sys_gt_1": beta_posterior_mean(alpha, beta, len(values)),
                    "posterior_predictive_at_least_one_10k": beta_binomial_predictive(
                        alpha, beta, len(values), 10_000
                    ),
                    "posterior_predictive_at_least_one_100k": beta_binomial_predictive(
                        alpha, beta, len(values), 100_000
                    ),
                    "posterior_predictive_at_least_one_1m": beta_binomial_predictive(
                        alpha, beta, len(values), 1_000_000
                    ),
                    "support_allows_exact_zero": "yes",
                    "exact_zero_support_predictive_1m": 0.0,
                    "max_sys": max(values),
                    "q99_sys": quantile(values, 0.99),
                }
            )
    return rows


def gpd_tail_survival(excess: float, shape: float, scale: float) -> float:
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


def fit_tail_probability(values: list[float], threshold_quantile: float, model: str) -> dict[str, Any]:
    array = np.array(values, dtype=float)
    threshold = float(np.quantile(array, threshold_quantile))
    tail = array[array > threshold]
    excesses = tail - threshold
    if len(excesses) < 12:
        return {"status": "insufficient_tail_rows", "threshold": threshold, "tail_n": len(excesses)}
    target_excess = 1.0 - threshold
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            if model == "exponential_excess":
                scale = float(np.mean(excesses))
                if scale <= 0.0:
                    raise ValueError("non-positive scale")
                conditional_survival = math.exp(-target_excess / scale)
                shape: float | str = 0.0
                endpoint: float | str = "infinite"
            elif model == "gpd_excess":
                shape_fit, _loc, scale_fit = stats.genpareto.fit(excesses, floc=0.0)
                shape = float(shape_fit)
                scale = float(scale_fit)
                conditional_survival = gpd_tail_survival(target_excess, shape, scale)
                endpoint = (
                    float(threshold - scale / shape)
                    if shape < 0.0 and scale > 0.0
                    else "infinite"
                )
            else:
                raise ValueError(f"unknown model {model}")
    except Exception as exc:
        return {
            "status": "fit_failed",
            "threshold": threshold,
            "tail_n": len(excesses),
            "error": str(exc)[:240],
        }

    p_sys_gt_1 = float((len(excesses) / len(array)) * max(0.0, conditional_survival))
    return {
        "status": "ok",
        "threshold": threshold,
        "tail_n": len(excesses),
        "shape": shape,
        "scale": scale,
        "endpoint": endpoint,
        "p_sys_gt_1": p_sys_gt_1,
    }


def fit_tail_probability_at_threshold(
    values: list[float], threshold: float, model: str, min_tail_n: int = 20
) -> dict[str, Any]:
    array = np.array(values, dtype=float)
    tail = array[array > threshold]
    excesses = tail - threshold
    row: dict[str, Any] = {
        "threshold": threshold,
        "tail_n": len(excesses),
        "exceedance_fraction": len(excesses) / len(array),
    }
    if len(excesses) < min_tail_n:
        return {**row, "status": "insufficient_tail_rows"}
    target_excess = 1.0 - threshold
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            if model == "exponential_excess":
                scale = float(np.mean(excesses))
                if scale <= 0.0:
                    raise ValueError("non-positive scale")
                shape: float | str = 0.0
                endpoint: float | str = "infinite"
                conditional_survival = math.exp(-max(0.0, target_excess) / scale)
            elif model == "gpd_excess":
                shape_fit, _loc, scale_fit = stats.genpareto.fit(excesses, floc=0.0)
                shape = float(shape_fit)
                scale = float(scale_fit)
                conditional_survival = gpd_tail_survival(target_excess, shape, scale)
                endpoint = (
                    float(threshold - scale / shape)
                    if shape < 0.0 and scale > 0.0
                    else "infinite"
                )
            else:
                raise ValueError(f"unknown model {model}")
    except Exception as exc:
        return {**row, "status": "fit_failed", "error": str(exc)[:240]}
    p_sys_gt_1 = float((len(excesses) / len(array)) * max(0.0, min(1.0, conditional_survival)))
    return {
        **row,
        "status": "ok",
        "shape": shape,
        "scale": scale,
        "endpoint": endpoint,
        "conditional_survival_sys_gt_1_given_exceedance": float(conditional_survival),
        "p_sys_gt_1": p_sys_gt_1,
        "predictive_at_least_one_1m": predictive_at_least_one(p_sys_gt_1, 1_000_000),
        "endpoint_below_1_flag": "yes" if isinstance(endpoint, float) and endpoint <= 1.0 else "no",
    }


def add_threshold_stability_flags(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_key: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_key[(str(row["stratum"]), str(row["model"]))].append(row)
    for key_rows in by_key.values():
        key_rows.sort(key=lambda row: float(row["threshold_quantile"]))
        for index, row in enumerate(key_rows):
            nearby = [
                candidate
                for candidate in key_rows[max(0, index - 2) : min(len(key_rows), index + 3)]
                if candidate.get("status") == "ok"
            ]
            log_ps = [
                math.log10(float(candidate["p_sys_gt_1"]))
                for candidate in nearby
                if float(candidate.get("p_sys_gt_1", 0.0)) > 0.0
            ]
            endpoints = [
                float(candidate["endpoint"])
                for candidate in nearby
                if isinstance(candidate.get("endpoint"), int | float)
            ]
            log10_range = max(log_ps) - min(log_ps) if len(log_ps) >= 2 else ""
            endpoint_range = max(endpoints) - min(endpoints) if len(endpoints) >= 2 else ""
            row["nearby_ok_fit_count"] = len(nearby)
            row["nearby_log10_p_sys_gt_1_range"] = log10_range
            row["nearby_finite_endpoint_range"] = endpoint_range
            row["threshold_probability_stable_flag"] = (
                "yes" if isinstance(log10_range, float) and log10_range <= 1.0 else "no"
            )
            row["threshold_endpoint_stable_flag"] = (
                "yes" if not isinstance(endpoint_range, float) or endpoint_range <= 0.15 else "no"
            )
            row["threshold_stable_flag"] = (
                "yes"
                if row.get("status") == "ok"
                and len(nearby) >= 3
                and row["threshold_probability_stable_flag"] == "yes"
                and row["threshold_endpoint_stable_flag"] == "yes"
                else "no"
            )
    return rows


def dense_threshold_evt_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = groups[stratum]
        for q in DENSE_TAIL_QUANTILES:
            threshold = quantile(values, q)
            for model in ["exponential_excess", "gpd_excess"]:
                fit = fit_tail_probability_at_threshold(values, threshold, model, min_tail_n=20)
                rows.append(
                    {
                        "stratum": stratum,
                        "n": len(values),
                        "model": model,
                        "threshold_quantile": q,
                        "max_sys": max(values),
                        **fit,
                    }
                )
    return add_threshold_stability_flags(rows)


def mean_residual_life_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = np.array(groups[stratum], dtype=float)
        for q in DENSE_TAIL_QUANTILES:
            threshold = float(np.quantile(values, q))
            excesses = values[values > threshold] - threshold
            rows.append(
                {
                    "stratum": stratum,
                    "n": len(values),
                    "threshold_quantile": q,
                    "threshold": threshold,
                    "tail_n": len(excesses),
                    "mean_residual_life": float(np.mean(excesses)) if len(excesses) else "",
                    "median_residual_life": float(np.median(excesses)) if len(excesses) else "",
                    "max_excess": float(np.max(excesses)) if len(excesses) else "",
                    "enough_exceedances_flag": "yes" if len(excesses) >= 20 else "no",
                }
            )
    return rows


def binomial_log_score(observed: int, n: int, p: float) -> float:
    p = min(max(p, 1e-300), 1.0 - 1e-16)
    return float(observed * math.log(p) + (n - observed) * math.log1p(-p))


def observed_tail_backtest_rows(groups: dict[str, list[float]]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = groups[stratum]
        array = np.array(values, dtype=float)
        fit_thresholds = [(q, float(np.quantile(array, q))) for q in BACKTEST_FIT_QUANTILES]
        test_thresholds = [(q, float(np.quantile(array, q))) for q in BACKTEST_TEST_QUANTILES]
        for fit_q, fit_threshold in fit_thresholds:
            for model in ["exponential_excess", "gpd_excess"]:
                fit = fit_tail_probability_at_threshold(values, fit_threshold, model, min_tail_n=20)
                for test_q, test_threshold in test_thresholds:
                    observed = int(np.sum(array > test_threshold))
                    row: dict[str, Any] = {
                        "stratum": stratum,
                        "n": len(values),
                        "model": model,
                        "fit_threshold_quantile": fit_q,
                        "fit_threshold": fit_threshold,
                        "test_threshold_quantile": test_q,
                        "test_threshold": test_threshold,
                        "observed_exceedances": observed,
                        "observed_rate": observed / len(values),
                        "fit_status": fit["status"],
                        "fit_tail_n": fit.get("tail_n", ""),
                    }
                    if test_threshold <= fit_threshold:
                        rows.append({**row, "status": "invalid_threshold_order"})
                        continue
                    if fit["status"] != "ok":
                        rows.append({**row, "status": fit["status"]})
                        continue
                    predicted_p = float(fit["exceedance_fraction"]) * gpd_tail_survival(
                        test_threshold - fit_threshold,
                        float(fit["shape"]),
                        float(fit["scale"]),
                    )
                    predicted_p = max(0.0, min(1.0, predicted_p))
                    lower, upper = binomial_wilson_interval(observed, len(values))
                    rows.append(
                        {
                            **row,
                            "status": "ok",
                            "predicted_probability": predicted_p,
                            "predicted_expected_count": len(values) * predicted_p,
                            "predicted_over_observed_count": (len(values) * predicted_p / observed)
                            if observed > 0
                            else "",
                            "observed_rate_wilson_95_lower": lower,
                            "observed_rate_wilson_95_upper": upper,
                            "predicted_inside_wilson_interval": "yes" if lower <= predicted_p <= upper else "no",
                            "binomial_log_score": binomial_log_score(observed, len(values), predicted_p),
                            "shape": fit.get("shape", ""),
                            "scale": fit.get("scale", ""),
                            "endpoint": fit.get("endpoint", ""),
                        }
                    )
    return rows


def bootstrap_tail_probability(
    values: list[float],
    threshold_quantile: float,
    model: str,
    rng: np.random.Generator,
    bootstrap_count: int,
) -> dict[str, Any]:
    array = np.array(values, dtype=float)
    probabilities: list[float] = []
    endpoints_below_one = 0
    successful = 0
    for _ in range(bootstrap_count):
        sample = array[rng.integers(0, len(array), size=len(array))].tolist()
        result = fit_tail_probability(sample, threshold_quantile, model)
        if result["status"] != "ok":
            continue
        successful += 1
        probabilities.append(float(result["p_sys_gt_1"]))
        endpoint = result.get("endpoint")
        if isinstance(endpoint, float) and endpoint <= 1.0:
            endpoints_below_one += 1
    if not probabilities:
        return {
            "bootstrap_success_count": successful,
            "bootstrap_p_q05": "",
            "bootstrap_p_median": "",
            "bootstrap_p_q95": "",
            "bootstrap_endpoint_below_one_fraction": "",
        }
    p = np.array(probabilities, dtype=float)
    return {
        "bootstrap_success_count": successful,
        "bootstrap_p_q05": float(np.quantile(p, 0.05)),
        "bootstrap_p_median": float(np.quantile(p, 0.5)),
        "bootstrap_p_q95": float(np.quantile(p, 0.95)),
        "bootstrap_endpoint_below_one_fraction": endpoints_below_one / successful
        if successful
        else "",
    }


def selected_tail_strata(groups: dict[str, list[float]]) -> list[str]:
    fixed = [key for key in groups if key.startswith("generic:") or key.startswith("product:")]
    by_max = sorted(fixed, key=lambda key: max(groups[key]), reverse=True)
    required = ["pooled:all", "dataset:random_sample", "dataset:random_product_sample"]
    return [key for key in required if key in groups] + by_max[:8]


def tail_model_rows(
    groups: dict[str, list[float]], rng: np.random.Generator, bootstrap_count: int
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for stratum in selected_tail_strata(groups):
        values = groups[stratum]
        for threshold_quantile in TAIL_QUANTILES:
            for model in ["exponential_excess", "gpd_excess"]:
                fit = fit_tail_probability(values, threshold_quantile, model)
                row: dict[str, Any] = {
                    "stratum": stratum,
                    "n": len(values),
                    "model": model,
                    "threshold_quantile": threshold_quantile,
                    "max_sys": max(values),
                    "q99_sys": quantile(values, 0.99),
                    **fit,
                }
                if fit["status"] == "ok":
                    boot = bootstrap_tail_probability(
                        values, threshold_quantile, model, rng, bootstrap_count
                    )
                    row.update(boot)
                    for run_size in RUN_SIZES:
                        row[f"predictive_at_least_one_{run_size}"] = predictive_at_least_one(
                            float(fit["p_sys_gt_1"]), run_size
                        )
                    q05 = boot.get("bootstrap_p_q05")
                    q95 = boot.get("bootstrap_p_q95")
                    row["model_dominated_flag"] = "yes"
                    row["unstable_flag"] = (
                        "yes"
                        if not isinstance(q05, float)
                        or not isinstance(q95, float)
                        or q05 <= 0.0
                        or q95 / max(q05, 1e-300) > 1_000.0
                        or stratum == "pooled:all"
                        else "no"
                    )
                rows.append(row)
    return rows


def decision_rows(
    groups: dict[str, list[float]],
    zero_rows: list[dict[str, Any]],
    tail_rows_data: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    pooled_zero = [
        row
        for row in zero_rows
        if row["stratum"] == "pooled:all" and row["run_size"] == 1_000_000
    ]
    best_tail = sorted(
        [
            row
            for row in tail_rows_data
            if row.get("status") == "ok"
            and isinstance(row.get("predictive_at_least_one_1000000"), float)
        ],
        key=lambda row: float(row["predictive_at_least_one_1000000"]),
        reverse=True,
    )
    rows = [
        {
            "question": "retained_table_anchor",
            "answer": "validated_no_positive_record_below_one",
            "row_count": len(groups["pooled:all"]),
            "max_sys": max(groups["pooled:all"]),
            "p99_sys": quantile(groups["pooled:all"], 0.99),
            "positive_count_sys_gt_1": sum(1 for value in groups["pooled:all"] if value > 1.0),
            "flag": "distribution_free_retained_sample_fact",
        }
    ]
    for row in pooled_zero:
        rows.append(
            {
                "question": "zero_positive_only_1m",
                "answer": row["model"],
                "posterior_predictive_at_least_one_1m": row[
                    "posterior_predictive_at_least_one"
                ],
                "flag": "prior_sensitive_not_distribution_free",
            }
        )
    for row in best_tail[:8]:
        rows.append(
            {
                "question": "tail_extrapolation_1m",
                "answer": f"{row['stratum']} {row['model']} q={row['threshold_quantile']}",
                "posterior_predictive_at_least_one_1m": row[
                    "predictive_at_least_one_1000000"
                ],
                "p_sys_gt_1": row["p_sys_gt_1"],
                "flag": "model_dominated"
                if row.get("model_dominated_flag") == "yes"
                else "model_sensitive",
                "unstable_flag": row.get("unstable_flag", ""),
            }
        )
    rows.append(
        {
            "question": "decision",
            "answer": "Do not treat 1M accepted samples as a well-calibrated rational blind scale-up under the current retained generator. A controlled large run is rational only as a deliberately model-dominated probe or after focusing/generator-axis work.",
            "flag": "current_strongest_conclusion",
        }
    )
    return rows


def model_class_ledger_rows(
    groups: dict[str, list[float]],
    zero_rows: list[dict[str, Any]],
    tail_rows_data: list[dict[str, Any]],
    threshold_rows: list[dict[str, Any]],
    backtest_rows: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    pooled_zero_1m = [
        row
        for row in zero_rows
        if row["stratum"] == "pooled:all" and row["run_size"] == 1_000_000
    ]
    gpd_rows = [
        row
        for row in threshold_rows
        if row.get("stratum") == "pooled:all"
        and row.get("model") == "gpd_excess"
        and row.get("status") == "ok"
    ]
    exp_rows = [
        row
        for row in threshold_rows
        if row.get("stratum") == "pooled:all"
        and row.get("model") == "exponential_excess"
        and row.get("status") == "ok"
    ]
    pooled_backtests = [
        row
        for row in backtest_rows
        if row.get("stratum") == "pooled:all" and row.get("status") == "ok"
    ]
    endpoint_below_one = [
        row for row in gpd_rows if row.get("endpoint_below_1_flag") == "yes"
    ]
    stable_gpd = [row for row in gpd_rows if row.get("threshold_stable_flag") == "yes"]
    stable_exp = [row for row in exp_rows if row.get("threshold_stable_flag") == "yes"]
    failed_backtests = [
        row
        for row in pooled_backtests
        if row.get("predicted_inside_wilson_interval") == "no"
    ]
    return [
        {
            "model_class": "endpoint_below_one",
            "stratum": "pooled:all",
            "evidence_for": f"gpd_endpoint_below_1_rows={len(endpoint_below_one)};max_sys={max(groups['pooled:all']):.17g}<1",
            "evidence_against": f"gpd_ok_rows={len(gpd_rows)};stable_gpd_rows={len(stable_gpd)}",
            "artifact_sources": "dense-threshold-tail-fit.tsv;decision-summary.tsv",
            "decision_effect": "keeps support-ending hypotheses live when extrapolating past the observed record",
            "warning_flag": "threshold_sensitive" if len(stable_gpd) == 0 else "model_sensitive",
        },
        {
            "model_class": "rare_positive_unstructured",
            "stratum": "pooled:all",
            "evidence_for": ";".join(
                f"{row['model']}_1m={float(row['posterior_predictive_at_least_one']):.6g}"
                for row in pooled_zero_1m
            ),
            "evidence_against": "zero positives do not distinguish exact-zero support from rare positive support",
            "artifact_sources": "zero-positive-posterior-predictive.tsv;generic-f-density-posterior.tsv",
            "decision_effect": "separates prior sensitivity from empirical tail-shape evidence",
            "warning_flag": "prior_sensitive",
        },
        {
            "model_class": "exponential_like_tail",
            "stratum": "pooled:all",
            "evidence_for": f"stable_exponential_rows={len(stable_exp)};ok_exponential_rows={len(exp_rows)}",
            "evidence_against": f"observed_tail_backtest_failures={len(failed_backtests)};max_sys={max(groups['pooled:all']):.17g}<1",
            "artifact_sources": "dense-threshold-tail-fit.tsv;observed-tail-backtest.tsv",
            "decision_effect": "makes 1M look attractive only if asymptotic onset is already trusted",
            "warning_flag": "model_dominated",
        },
        {
            "model_class": "gpd_or_reverse_weibull_tail",
            "stratum": "pooled:all",
            "evidence_for": f"gpd_ok_rows={len(gpd_rows)};endpoint_below_1_rows={len(endpoint_below_one)}",
            "evidence_against": f"stable_gpd_rows={len(stable_gpd)}",
            "artifact_sources": "dense-threshold-tail-fit.tsv;mean-residual-life.tsv",
            "decision_effect": "tests whether bounded-tail behavior is stable across thresholds",
            "warning_flag": "threshold_sensitive",
        },
        {
            "model_class": "mixture_or_bulk_contamination",
            "stratum": "pooled:all",
            "evidence_for": f"selected_strata={len(selected_tail_strata(groups))};pooled_fit_backtest_failures={len(failed_backtests)}",
            "evidence_against": "not a direct probability model for sys>1",
            "artifact_sources": "dense-empirical-survival.tsv;observed-tail-backtest.tsv",
            "decision_effect": "discounts pooled and low-threshold fits as finite-u/bulk-sensitive",
            "warning_flag": "bulk_or_mixture_sensitive",
        },
    ]


def naturalistic_hypothesis_rows(
    groups: dict[str, list[float]],
    zero_rows: list[dict[str, Any]],
    backtest_rows: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    pooled_zero_1m = [
        row
        for row in zero_rows
        if row["stratum"] == "pooled:all" and row["run_size"] == 1_000_000
    ]
    pooled_exp_backtests = [
        row
        for row in backtest_rows
        if row.get("stratum") == "pooled:all"
        and row.get("model") == "exponential_excess"
        and row.get("status") == "ok"
    ]
    pooled_gpd_backtests = [
        row
        for row in backtest_rows
        if row.get("stratum") == "pooled:all"
        and row.get("model") == "gpd_excess"
        and row.get("status") == "ok"
    ]
    exp_inside = sum(row.get("predicted_inside_wilson_interval") == "yes" for row in pooled_exp_backtests)
    gpd_inside = sum(row.get("predicted_inside_wilson_interval") == "yes" for row in pooled_gpd_backtests)
    return [
        {
            "hypothesis_bucket": "retained_high_tail_different_mechanism",
            "mechanism": "The retained record rows around sys 0.86 arise from a generator-specific or mixture mechanism that the pooled table does not identify.",
            "predictions": "pooled exponential extrapolation can fail observed-tail backtests; fixed-bucket tails need separate treatment",
            "observations": f"pooled_top_sys={max(groups['pooled:all']):.17g};product5x5_max_sys={max(groups.get('product:5x5', [float('nan')])):.17g};pooled_exp_backtest_inside={exp_inside}/{len(pooled_exp_backtests)};pooled_gpd_backtest_inside={gpd_inside}/{len(pooled_gpd_backtests)}",
            "update_direction": "supported_by_backtest_failure_of_exponential_tail_and_retained_table_mixture_status",
            "current_weight": "medium",
        },
        {
            "hypothesis_bucket": "broad_positive_region",
            "mechanism": "The positive region has enough measure under the current generator that 1M should probably hit.",
            "predictions": "zero-positive retained table is unlucky; tail extrapolation should backtest well; zero-positive priors should be overwhelmed by high-tail evidence",
            "observations": ";".join(
                f"{row['model']}_1m={float(row['posterior_predictive_at_least_one']):.6g}"
                for row in pooled_zero_1m
            )
            + f";pooled_exp_backtest_inside={exp_inside}/{len(pooled_exp_backtests)}",
            "update_direction": "disfavored_by_zero_positives_and_pooled_exponential_backtest_failures",
            "current_weight": "low",
        },
    ]


def validate_anchors(summary: dict[str, Any]) -> None:
    expected = {
        "row_count": 14336,
        "positives_sys_gt_1": 0,
        "max_sys": 0.86258589584944,
        "p99_sys": 0.7521020122453151,
    }
    for key, expected_value in expected.items():
        observed = summary[key]
        if isinstance(expected_value, float):
            if not math.isclose(float(observed), expected_value, rel_tol=0.0, abs_tol=5e-15):
                raise SystemExit(f"Anchor mismatch for {key}: {observed} != {expected_value}")
        elif observed != expected_value:
            raise SystemExit(f"Anchor mismatch for {key}: {observed} != {expected_value}")


def main() -> None:
    args = parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    rng = np.random.default_rng(args.rng_seed)
    polytope_rows, groups = load_rows(args.tables_dir)
    anchors = anchor_summary(polytope_rows)
    validate_anchors(anchors)

    empirical_survival = survival_rows(groups)
    dense_empirical_survival = dense_empirical_survival_rows(groups)
    top_k_spacings = top_k_spacing_rows(groups)
    future_max_support_limited = support_limited_future_max_rows(groups)
    zero_predictive = zero_positive_rows(groups)
    generic_f_posterior = generic_f_density_rows(groups)
    tail_predictive = tail_model_rows(groups, rng, args.bootstrap_count)
    dense_threshold_tail_fit = dense_threshold_evt_rows(groups)
    mean_residual_life = mean_residual_life_rows(groups)
    observed_tail_backtest = observed_tail_backtest_rows(groups)
    decision = decision_rows(groups, zero_predictive, tail_predictive)
    model_class_ledger = model_class_ledger_rows(
        groups,
        zero_predictive,
        tail_predictive,
        dense_threshold_tail_fit,
        observed_tail_backtest,
    )
    naturalistic_hypotheses = naturalistic_hypothesis_rows(
        groups,
        zero_predictive,
        observed_tail_backtest,
    )

    write_tsv(
        args.out_dir / "empirical-survival.tsv",
        empirical_survival,
        [
            "stratum",
            "n",
            "threshold",
            "exceedances_sys_gt_threshold",
            "empirical_survival",
            "max_sys",
            "q99_sys",
        ],
    )
    write_tsv(
        args.out_dir / "dense-empirical-survival.tsv",
        dense_empirical_survival,
        [
            "stratum",
            "n",
            "threshold",
            "exceedances_sys_gt_threshold",
            "empirical_survival",
            "wilson_95_lower",
            "wilson_95_upper",
            "threshold_source",
            "high_quantile",
            "max_sys",
        ],
    )
    write_tsv(
        args.out_dir / "top-k-spacings.tsv",
        top_k_spacings,
        [
            "stratum",
            "n",
            "rank_desc",
            "sys",
            "gap_from_one",
            "gap_to_next_lower",
            "gap_to_record",
            "empirical_tail_mass_at_or_above_rank",
        ],
    )
    write_tsv(
        args.out_dir / "support-limited-future-max.tsv",
        future_max_support_limited,
        [
            "stratum",
            "n",
            "run_size",
            "observed_max",
            "top_multiplicity",
            "empirical_top_mass",
            "probability_rehit_observed_max",
            "support_limited_can_exceed_observed_max",
        ],
    )
    write_tsv(
        args.out_dir / "zero-positive-posterior-predictive.tsv",
        zero_predictive,
        [
            "stratum",
            "n",
            "model",
            "posterior_mean_p_sys_gt_1",
            "run_size",
            "posterior_predictive_at_least_one",
        ],
    )
    write_tsv(
        args.out_dir / "generic-f-density-posterior.tsv",
        generic_f_posterior,
        [
            "facet_count",
            "stratum",
            "n",
            "positives_sys_gt_1",
            "model",
            "posterior_mean_p_sys_gt_1",
            "posterior_predictive_at_least_one_10k",
            "posterior_predictive_at_least_one_100k",
            "posterior_predictive_at_least_one_1m",
            "support_allows_exact_zero",
            "exact_zero_support_predictive_1m",
            "max_sys",
            "q99_sys",
        ],
    )
    write_tsv(
        args.out_dir / "tail-model-predictive.tsv",
        tail_predictive,
        [
            "stratum",
            "n",
            "model",
            "threshold_quantile",
            "threshold",
            "tail_n",
            "status",
            "shape",
            "scale",
            "endpoint",
            "p_sys_gt_1",
            "predictive_at_least_one_10000",
            "predictive_at_least_one_100000",
            "predictive_at_least_one_1000000",
            "bootstrap_success_count",
            "bootstrap_p_q05",
            "bootstrap_p_median",
            "bootstrap_p_q95",
            "bootstrap_endpoint_below_one_fraction",
            "model_dominated_flag",
            "unstable_flag",
            "max_sys",
            "q99_sys",
            "error",
        ],
    )
    write_tsv(
        args.out_dir / "dense-threshold-tail-fit.tsv",
        dense_threshold_tail_fit,
        [
            "stratum",
            "n",
            "model",
            "threshold_quantile",
            "threshold",
            "tail_n",
            "exceedance_fraction",
            "status",
            "shape",
            "scale",
            "endpoint",
            "conditional_survival_sys_gt_1_given_exceedance",
            "p_sys_gt_1",
            "predictive_at_least_one_1m",
            "endpoint_below_1_flag",
            "nearby_ok_fit_count",
            "nearby_log10_p_sys_gt_1_range",
            "nearby_finite_endpoint_range",
            "threshold_probability_stable_flag",
            "threshold_endpoint_stable_flag",
            "threshold_stable_flag",
            "max_sys",
            "error",
        ],
    )
    write_tsv(
        args.out_dir / "mean-residual-life.tsv",
        mean_residual_life,
        [
            "stratum",
            "n",
            "threshold_quantile",
            "threshold",
            "tail_n",
            "mean_residual_life",
            "median_residual_life",
            "max_excess",
            "enough_exceedances_flag",
        ],
    )
    write_tsv(
        args.out_dir / "observed-tail-backtest.tsv",
        observed_tail_backtest,
        [
            "stratum",
            "n",
            "model",
            "fit_threshold_quantile",
            "fit_threshold",
            "test_threshold_quantile",
            "test_threshold",
            "observed_exceedances",
            "observed_rate",
            "fit_status",
            "fit_tail_n",
            "status",
            "predicted_probability",
            "predicted_expected_count",
            "predicted_over_observed_count",
            "observed_rate_wilson_95_lower",
            "observed_rate_wilson_95_upper",
            "predicted_inside_wilson_interval",
            "binomial_log_score",
            "shape",
            "scale",
            "endpoint",
        ],
    )
    write_tsv(args.out_dir / "decision-summary.tsv", decision, sorted({key for row in decision for key in row}))
    write_tsv(
        args.out_dir / "model-class-ledger.tsv",
        model_class_ledger,
        [
            "model_class",
            "stratum",
            "evidence_for",
            "evidence_against",
            "artifact_sources",
            "decision_effect",
            "warning_flag",
        ],
    )
    write_tsv(
        args.out_dir / "naturalistic-hypothesis-ledger.tsv",
        naturalistic_hypotheses,
        [
            "hypothesis_bucket",
            "mechanism",
            "predictions",
            "observations",
            "update_direction",
            "current_weight",
        ],
    )
    write_json(
        args.out_dir / "summary.json",
        {
            "status": "ok",
            "tables_dir": display_path(args.tables_dir),
            "anchors": anchors,
            "run_sizes": RUN_SIZES,
            "survival_thresholds": SURVIVAL_THRESHOLDS,
            "tail_quantiles": TAIL_QUANTILES,
            "dense_tail_quantiles": DENSE_TAIL_QUANTILES,
            "backtest_fit_quantiles": BACKTEST_FIT_QUANTILES,
            "backtest_test_quantiles": BACKTEST_TEST_QUANTILES,
            "zero_positive_priors": ZERO_POSITIVE_PRIORS,
            "selected_tail_strata": selected_tail_strata(groups),
            "bootstrap_count": args.bootstrap_count,
            "artifacts": {
                "empirical_survival": display_path(args.out_dir / "empirical-survival.tsv"),
                "dense_empirical_survival": display_path(args.out_dir / "dense-empirical-survival.tsv"),
                "top_k_spacings": display_path(args.out_dir / "top-k-spacings.tsv"),
                "support_limited_future_max": display_path(args.out_dir / "support-limited-future-max.tsv"),
                "zero_positive_posterior_predictive": display_path(
                    args.out_dir / "zero-positive-posterior-predictive.tsv"
                ),
                "generic_f_density_posterior": display_path(
                    args.out_dir / "generic-f-density-posterior.tsv"
                ),
                "tail_model_predictive": display_path(args.out_dir / "tail-model-predictive.tsv"),
                "dense_threshold_tail_fit": display_path(args.out_dir / "dense-threshold-tail-fit.tsv"),
                "mean_residual_life": display_path(args.out_dir / "mean-residual-life.tsv"),
                "observed_tail_backtest": display_path(args.out_dir / "observed-tail-backtest.tsv"),
                "decision_summary": display_path(args.out_dir / "decision-summary.tsv"),
                "model_class_ledger": display_path(args.out_dir / "model-class-ledger.tsv"),
                "naturalistic_hypothesis_ledger": display_path(
                    args.out_dir / "naturalistic-hypothesis-ledger.tsv"
                ),
            },
        },
    )

    print("# tail-survival-1m-posterior")
    print(f"- rows: `{anchors['row_count']}`")
    print(f"- positives sys > 1: `{anchors['positives_sys_gt_1']}`")
    print(f"- max sys: `{anchors['max_sys']:.14f}`")
    print(f"- p99 sys: `{anchors['p99_sys']:.16f}`")
    print(f"- dense threshold fit rows: `{len(dense_threshold_tail_fit)}`")
    print(f"- observed-tail backtest rows: `{len(observed_tail_backtest)}`")
    print(f"- wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
