#!/usr/bin/env python3
"""Target-free atlas for support processes on frozen random normal fans.

The producer is deliberately copy-local. It reuses the probability formulas
from alternative-generator-smoke at reviewed commit 6cc64c8f, but does not
import that unstable experiment interface. No target or capacity code is
reachable from this packet.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import platform
import random
import subprocess
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable, Sequence


LAW_VERSION = "generator-support-process-atlas-v1"
DEFAULT_SEEDS = (1201, 2203, 3209)
DEFAULT_SIDE_COUNTS = (4, 6, 8)
DEFAULT_FANS_PER_STRATUM = 48
SUPPORT_GRID_SIZE = 64
CLIPPED_GAUSSIAN_BOUND = 2.0
SMOOTH_EMPIRICAL_LOG_SD = 0.1
CV_MATCH_ABS_TOLERANCE = 0.02
GEOMETRY_TOLERANCE = 1.0e-9

ARMS: tuple[str, ...] = (
    "equal",
    "current_uniform_0.8_1.2",
    "iid_log_sigma_0.1",
    "iid_log_sigma_0.2",
    "smooth_log_r2_sd_0.1",
    "smooth_log_r3_sd_0.1",
)
SMOOTH_IID_CV_COMPARISONS: tuple[tuple[str, str], ...] = (
    ("iid_log_sigma_0.1", "smooth_log_r2_sd_0.1"),
    ("iid_log_sigma_0.1", "smooth_log_r3_sd_0.1"),
)
SIGMA_LADDER = ("equal", "iid_log_sigma_0.1", "iid_log_sigma_0.2")
SUMMARY_METRICS = (
    "support_cv",
    "log_support_sd",
    "log_support_adjacency_correlation",
    "log_support_roughness",
    "gap_cv",
    "isoperimetric_ratio",
    "width_anisotropy",
    "max_vertex_radius",
    "source_support_l2",
    "source_support_linf",
    "source_vertex_rms",
)
SOURCE_FILES = (
    "experiments/sys-datascience/methods/generator-support-process-atlas/run.py",
    "experiments/sys-datascience/methods/generator-support-process-atlas/test_run.py",
    "experiments/sys-datascience/methods/generator-support-process-atlas/README.md",
)


def stable_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def stable_hash(*parts: Any) -> str:
    digest = hashlib.sha256()
    for part in parts:
        digest.update(stable_json(part).encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def deterministic_rng(*parts: Any) -> random.Random:
    seed = int(stable_hash(LAW_VERSION, *parts)[:16], 16)
    return random.Random(seed)


def standard_normal(rng: random.Random) -> float:
    """One Box--Muller normal; algorithm is pinned in this source."""
    u1 = max(rng.random(), 2.0**-53)
    u2 = rng.random()
    return math.sqrt(-2.0 * math.log(u1)) * math.cos(2.0 * math.pi * u2)


def mean(values: Sequence[float]) -> float:
    return sum(values) / len(values)


def population_sd(values: Sequence[float]) -> float:
    center = mean(values)
    return math.sqrt(sum((value - center) ** 2 for value in values) / len(values))


def coefficient_of_variation(values: Sequence[float]) -> float | None:
    center = mean(values)
    if not math.isfinite(center) or center <= 0.0:
        return None
    return population_sd(values) / center


def percentile(values: Sequence[float], probability: float) -> float | None:
    finite = sorted(value for value in values if math.isfinite(value))
    if not finite:
        return None
    if len(finite) == 1:
        return finite[0]
    position = probability * (len(finite) - 1)
    lower = math.floor(position)
    upper = math.ceil(position)
    weight = position - lower
    return finite[lower] * (1.0 - weight) + finite[upper] * weight


def summarize_values(values: Iterable[float | None]) -> dict[str, Any]:
    finite = [float(value) for value in values if value is not None and math.isfinite(value)]
    if not finite:
        return {"count": 0, "mean": None, "p05": None, "p50": None, "p95": None}
    return {
        "count": len(finite),
        "mean": mean(finite),
        "p05": percentile(finite, 0.05),
        "p50": percentile(finite, 0.50),
        "p95": percentile(finite, 0.95),
    }


def dot(a: Sequence[float], b: Sequence[float]) -> float:
    return a[0] * b[0] + a[1] * b[1]


def shoelace(vertices: Sequence[Sequence[float]]) -> float:
    return 0.5 * sum(
        a[0] * vertices[(index + 1) % len(vertices)][1]
        - vertices[(index + 1) % len(vertices)][0] * a[1]
        for index, a in enumerate(vertices)
    )


def polygon_centroid(vertices: Sequence[Sequence[float]]) -> tuple[float, float] | None:
    twice_area = 0.0
    x_sum = 0.0
    y_sum = 0.0
    for index, a in enumerate(vertices):
        b = vertices[(index + 1) % len(vertices)]
        cross = a[0] * b[1] - b[0] * a[1]
        twice_area += cross
        x_sum += (a[0] + b[0]) * cross
        y_sum += (a[1] + b[1]) * cross
    if abs(twice_area) <= 1.0e-14:
        return None
    return (x_sum / (3.0 * twice_area), y_sum / (3.0 * twice_area))


def cyclic_gaps(angles: Sequence[float]) -> list[float]:
    return [
        (angles[(index + 1) % len(angles)] - angle) % (2.0 * math.pi)
        for index, angle in enumerate(angles)
    ]


def cyclic_adjacency_correlation(values: Sequence[float]) -> float | None:
    center = mean(values)
    variance = mean([(value - center) ** 2 for value in values])
    if variance <= 1.0e-16:
        return None
    covariance = mean(
        [
            (value - center) * (values[(index + 1) % len(values)] - center)
            for index, value in enumerate(values)
        ]
    )
    return covariance / variance


def cyclic_roughness(values: Sequence[float]) -> float:
    return math.sqrt(
        mean(
            [
                (values[(index + 1) % len(values)] - value) ** 2
                for index, value in enumerate(values)
            ]
        )
    )


def support_process_metrics(supports: Sequence[float]) -> dict[str, float | None]:
    logs = [math.log(value) for value in supports]
    return {
        "support_cv": coefficient_of_variation(supports),
        "log_support_sd": population_sd(logs),
        "log_support_adjacency_correlation": cyclic_adjacency_correlation(logs),
        "log_support_roughness": cyclic_roughness(logs),
    }


def make_supports(arm: str, angles: Sequence[float], rng: random.Random) -> tuple[list[float] | None, dict[str, Any], str | None]:
    n = len(angles)
    if arm == "equal":
        return [1.0] * n, {"construction": "h_i=1"}, None
    if arm == "current_uniform_0.8_1.2":
        return (
            [0.8 + 0.4 * rng.random() for _ in range(n)],
            {"construction": "h_i iid Uniform[0.8,1.2)"},
            None,
        )
    if arm.startswith("iid_log_sigma_"):
        sigma = float(arm.rsplit("_", 1)[1])
        latent = [max(-CLIPPED_GAUSSIAN_BOUND, min(CLIPPED_GAUSSIAN_BOUND, standard_normal(rng))) for _ in range(n)]
        latent_mean = mean(latent)
        centered = [value - latent_mean for value in latent]
        supports = [math.exp(sigma * value) for value in centered]
        return (
            supports,
            {
                "construction": "h_i=exp(sigma*(clip(Z_i,-2,2)-sample_mean))",
                "sigma": sigma,
                "latent": latent,
            },
            None,
        )
    if arm.startswith("smooth_log_r"):
        modes = int(arm.split("_r", 1)[1].split("_", 1)[0])
        coefficients = [(standard_normal(rng), standard_normal(rng)) for _ in range(modes)]
        field = []
        for theta in angles:
            value = 0.0
            for index, (a, b) in enumerate(coefficients, start=1):
                value += (a * math.cos(index * theta) + b * math.sin(index * theta)) / index
            field.append(value)
        field_mean = mean(field)
        centered = [value - field_mean for value in field]
        empirical_sd = population_sd(centered)
        if not math.isfinite(empirical_sd) or empirical_sd <= 1.0e-12:
            return None, {"coefficients": coefficients}, "smooth_field_zero_empirical_sd"
        logs = [SMOOTH_EMPIRICAL_LOG_SD * value / empirical_sd for value in centered]
        return (
            [math.exp(value) for value in logs],
            {
                "construction": "inverse-frequency Fourier log field, centered and rescaled to empirical SD 0.1",
                "modes": modes,
                "coefficients": coefficients,
                "pre_rescale_empirical_sd": empirical_sd,
            },
            None,
        )
    raise ValueError(f"unknown arm: {arm}")


def construct_polygon(angles: Sequence[float], supports: Sequence[float]) -> tuple[dict[str, Any] | None, str | None]:
    n = len(angles)
    gaps = cyclic_gaps(angles)
    if any(gap >= math.pi - 1.0e-12 for gap in gaps):
        return None, "unbounded_normal_fan"
    normals = [(math.cos(theta), math.sin(theta)) for theta in angles]
    vertices: list[tuple[float, float]] = []
    for index, normal in enumerate(normals):
        other = normals[(index + 1) % n]
        determinant = normal[0] * other[1] - normal[1] * other[0]
        if abs(determinant) <= 1.0e-12:
            return None, "adjacent_normals_numerically_parallel"
        x = (supports[index] * other[1] - supports[(index + 1) % n] * normal[1]) / determinant
        y = (normal[0] * supports[(index + 1) % n] - other[0] * supports[index]) / determinant
        vertices.append((x, y))
    for vertex in vertices:
        scale = 1.0 + math.hypot(*vertex) + max(supports)
        if any(dot(normal, vertex) > support + GEOMETRY_TOLERANCE * scale for normal, support in zip(normals, supports)):
            return None, "inactive_facet_or_infeasible_intersection"
    area = shoelace(vertices)
    if not math.isfinite(area) or area <= 1.0e-12:
        return None, "nonpositive_or_nonfinite_area"
    factor_scale = 1.0 / math.sqrt(area)
    normalized_vertices = [(factor_scale * x, factor_scale * y) for x, y in vertices]
    normalized_supports = [factor_scale * support for support in supports]
    centroid = polygon_centroid(normalized_vertices)
    if centroid is None:
        return None, "undefined_area_centroid"
    centered_vertices = [(x - centroid[0], y - centroid[1]) for x, y in normalized_vertices]
    centered_supports = [
        support - dot(normal, centroid) for normal, support in zip(normals, normalized_supports)
    ]
    if any(value <= 0.0 or not math.isfinite(value) for value in centered_supports):
        return None, "nonpositive_centered_support"
    support_grid = []
    for index in range(SUPPORT_GRID_SIZE):
        theta = 2.0 * math.pi * index / SUPPORT_GRID_SIZE
        direction = (math.cos(theta), math.sin(theta))
        support_grid.append(max(dot(direction, vertex) for vertex in centered_vertices))
    widths = [
        support_grid[index] + support_grid[(index + SUPPORT_GRID_SIZE // 2) % SUPPORT_GRID_SIZE]
        for index in range(SUPPORT_GRID_SIZE // 2)
    ]
    if min(widths) <= 0.0:
        return None, "nonpositive_sampled_width"
    perimeter = sum(
        math.dist(vertex, centered_vertices[(index + 1) % n])
        for index, vertex in enumerate(centered_vertices)
    )
    metrics = support_process_metrics(supports)
    metrics.update(
        {
            "gap_cv": coefficient_of_variation(gaps),
            "isoperimetric_ratio": 4.0 * math.pi / (perimeter * perimeter),
            "width_anisotropy": max(widths) / min(widths),
            "max_vertex_radius": max(math.hypot(x, y) for x, y in centered_vertices),
        }
    )
    return (
        {
            "vertices_centered_area1": centered_vertices,
            "supports_centered_area1": centered_supports,
            "support_grid_centered_area1": support_grid,
            "metrics": metrics,
        },
        None,
    )


def vector_l2(a: Sequence[float], b: Sequence[float]) -> float:
    return math.sqrt(mean([(x - y) ** 2 for x, y in zip(a, b)]))


def paired_distances(row: dict[str, Any], source: dict[str, Any]) -> dict[str, float]:
    support_a = row["support_grid_centered_area1"]
    support_b = source["support_grid_centered_area1"]
    vertices_a = row["vertices_centered_area1"]
    vertices_b = source["vertices_centered_area1"]
    support_differences = [a - b for a, b in zip(support_a, support_b)]
    return {
        "source_support_l2": math.sqrt(mean([value * value for value in support_differences])),
        "source_support_linf": max(abs(value) for value in support_differences),
        "source_vertex_rms": math.sqrt(
            mean(
                [
                    (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2
                    for a, b in zip(vertices_a, vertices_b)
                ]
            )
        ),
    }


def generate_rows(seeds: Sequence[int], side_counts: Sequence[int], fans_per_stratum: int) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    fans: list[dict[str, Any]] = []
    attempts: list[dict[str, Any]] = []
    for seed in seeds:
        for side_count in side_counts:
            for fan_index in range(fans_per_stratum):
                fan_rng = deterministic_rng("fan", seed, side_count, fan_index)
                angles = sorted(2.0 * math.pi * fan_rng.random() for _ in range(side_count))
                fan_id = stable_hash("fan", seed, side_count, fan_index, angles)
                fan_record = {
                    "schema": "generator-support-process-fan-v1",
                    "law_version": LAW_VERSION,
                    "fan_id": fan_id,
                    "seed": seed,
                    "side_count": side_count,
                    "fan_index": fan_index,
                    "normal_angles": angles,
                    "gap_cv": coefficient_of_variation(cyclic_gaps(angles)),
                }
                fans.append(fan_record)
                fan_attempts: list[dict[str, Any]] = []
                for arm in ARMS:
                    latent_id = stable_hash("support-latent", seed, side_count, fan_index, arm)
                    rng = deterministic_rng("support-latent", seed, side_count, fan_index, arm)
                    supports, latent, support_failure = make_supports(arm, angles, rng)
                    row: dict[str, Any] = {
                        "schema": "generator-support-process-attempt-v1",
                        "law_version": LAW_VERSION,
                        "sample_id": stable_hash("sample", fan_id, arm, latent_id),
                        "fan_id": fan_id,
                        "latent_id": latent_id,
                        "seed": seed,
                        "side_count": side_count,
                        "fan_index": fan_index,
                        "arm": arm,
                        "accepted": False,
                        "failure_reason": support_failure,
                        "complete_paired_subset": False,
                        "supports_raw": supports,
                        "latent": latent,
                    }
                    if supports is not None and support_failure is None:
                        polygon, geometry_failure = construct_polygon(angles, supports)
                        row["failure_reason"] = geometry_failure
                        if polygon is not None:
                            row.update(polygon)
                            row["accepted"] = True
                    fan_attempts.append(row)
                complete = all(row["accepted"] for row in fan_attempts)
                for row in fan_attempts:
                    row["complete_paired_subset"] = complete
                equal = next((row for row in fan_attempts if row["arm"] == "equal" and row["accepted"]), None)
                if equal is not None:
                    for row in fan_attempts:
                        if row["accepted"]:
                            distances = paired_distances(row, equal)
                            row["metrics"].update(distances)
                attempts.extend(fan_attempts)
    return fans, attempts


def mark_complete_fans(attempts: Sequence[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in attempts:
        grouped[row["fan_id"]].append(row)
    complete = []
    for fan_id, rows in grouped.items():
        if len(rows) == len(ARMS) and all(row["accepted"] for row in rows):
            complete.append(
                {
                    "schema": "generator-support-process-complete-fan-v1",
                    "law_version": LAW_VERSION,
                    "fan_id": fan_id,
                    "seed": rows[0]["seed"],
                    "side_count": rows[0]["side_count"],
                    "fan_index": rows[0]["fan_index"],
                    "conditioning": "all_requested_arms_accepted_on_the_frozen_fan",
                    "sample_ids_by_arm": {row["arm"]: row["sample_id"] for row in rows},
                }
            )
    return sorted(complete, key=lambda row: (row["seed"], row["side_count"], row["fan_index"]))


def metric_summary_rows(attempts: Sequence[dict[str, Any]], conditioning: str) -> list[dict[str, Any]]:
    grouped: dict[tuple[int, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in attempts:
        if conditioning == "complete_paired" and not row["complete_paired_subset"]:
            continue
        grouped[(row["side_count"], row["seed"], row["arm"])].append(row)
    summaries = []
    for (side_count, seed, arm), rows in sorted(grouped.items()):
        accepted = [row for row in rows if row["accepted"]]
        summaries.append(
            {
                "schema": "generator-support-process-metric-summary-v1",
                "law_version": LAW_VERSION,
                "conditioning": (
                    "arm_marginal_frozen_fan_attempts"
                    if conditioning == "marginal"
                    else "complete_paired_conditioned_on_every_arm_succeeding"
                ),
                "side_count": side_count,
                "seed": seed,
                "arm": arm,
                "attempted": len(rows),
                "accepted": len(accepted),
                "acceptance_rate": len(accepted) / len(rows) if rows else None,
                "failure_counts": dict(sorted(failure_counts(rows).items())),
                "metrics": {
                    metric: summarize_values(row.get("metrics", {}).get(metric) for row in accepted)
                    for metric in SUMMARY_METRICS
                },
            }
        )
    return summaries


def failure_counts(rows: Sequence[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = defaultdict(int)
    for row in rows:
        if not row["accepted"]:
            counts[row["failure_reason"] or "unknown_failure"] += 1
    return counts


def group_accepted(attempts: Sequence[dict[str, Any]], complete_only: bool) -> dict[tuple[int, int, str], list[dict[str, Any]]]:
    grouped: dict[tuple[int, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in attempts:
        if not row["accepted"] or (complete_only and not row["complete_paired_subset"]):
            continue
        grouped[(row["side_count"], row["seed"], row["arm"])].append(row)
    return grouped


def within_diversity_rows(attempts: Sequence[dict[str, Any]], complete_only: bool) -> list[dict[str, Any]]:
    result = []
    for (side_count, seed, arm), rows in sorted(group_accepted(attempts, complete_only).items()):
        distances = [
            vector_l2(a["support_grid_centered_area1"], b["support_grid_centered_area1"])
            for index, a in enumerate(rows)
            for b in rows[index + 1 :]
        ]
        result.append(
            {
                "schema": "generator-support-process-within-diversity-v1",
                "conditioning": (
                    "complete_paired_conditioned_on_every_arm_succeeding"
                    if complete_only
                    else "arm_marginal_accepted_shapes"
                ),
                "side_count": side_count,
                "seed": seed,
                "arm": arm,
                "shape_count": len(rows),
                "pair_count": len(distances),
                "support_l2": summarize_values(distances),
            }
        )
    return result


def nearest_distances(source_rows: Sequence[dict[str, Any]], target_rows: Sequence[dict[str, Any]]) -> list[float]:
    distances = []
    for source in source_rows:
        candidates = [
            vector_l2(source["support_grid_centered_area1"], target["support_grid_centered_area1"])
            for target in target_rows
            if target["fan_id"] != source["fan_id"]
        ]
        if candidates:
            distances.append(min(candidates))
    return distances


def directed_overlap_rows(attempts: Sequence[dict[str, Any]], complete_only: bool) -> list[dict[str, Any]]:
    grouped = group_accepted(attempts, complete_only)
    result = []
    for side_count in sorted({key[0] for key in grouped}):
        for seed in sorted({key[1] for key in grouped if key[0] == side_count}):
            for source_arm in ARMS:
                source = grouped.get((side_count, seed, source_arm), [])
                within = nearest_distances(source, source)
                within_mean = mean(within) if within else None
                for target_arm in ARMS:
                    if target_arm == source_arm:
                        continue
                    target = grouped.get((side_count, seed, target_arm), [])
                    directed = nearest_distances(source, target)
                    directed_mean = mean(directed) if directed else None
                    result.append(
                        {
                            "schema": "generator-support-process-directed-overlap-v1",
                            "conditioning": (
                                "complete_paired_conditioned_on_every_arm_succeeding"
                                if complete_only
                                else "arm_marginal_accepted_shapes"
                            ),
                            "same_fan_targets_excluded": True,
                            "side_count": side_count,
                            "seed": seed,
                            "source_arm": source_arm,
                            "target_arm": target_arm,
                            "source_shape_count": len(source),
                            "target_shape_count": len(target),
                            "directed_nearest_support_l2": summarize_values(directed),
                            "source_within_nearest_mean": within_mean,
                            "directed_to_within_mean_ratio": (
                                directed_mean / within_mean
                                if directed_mean is not None and within_mean is not None and within_mean > 0.0
                                else None
                            ),
                        }
                    )
    return result


def paired_source_distance_rows(attempts: Sequence[dict[str, Any]], complete_only: bool) -> list[dict[str, Any]]:
    grouped: dict[tuple[int, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in attempts:
        if not row["accepted"] or "source_support_l2" not in row.get("metrics", {}):
            continue
        if complete_only and not row["complete_paired_subset"]:
            continue
        grouped[(row["side_count"], row["seed"], row["arm"])].append(row)
    result = []
    for (side_count, seed, arm), rows in sorted(grouped.items()):
        result.append(
            {
                "schema": "generator-support-process-paired-distance-v1",
                "conditioning": (
                    "complete_paired_conditioned_on_every_arm_succeeding"
                    if complete_only
                    else "source_equal_and_arm_accepted_on_frozen_fan"
                ),
                "side_count": side_count,
                "seed": seed,
                "arm": arm,
                "pair_count": len(rows),
                "source_support_l2": summarize_values(row["metrics"]["source_support_l2"] for row in rows),
                "source_support_linf": summarize_values(row["metrics"]["source_support_linf"] for row in rows),
                "source_vertex_rms": summarize_values(row["metrics"]["source_vertex_rms"] for row in rows),
            }
        )
    return result


def cv_matching_rows(attempts: Sequence[dict[str, Any]], complete_only: bool) -> list[dict[str, Any]]:
    grouped = group_accepted(attempts, complete_only)
    result = []
    strata = sorted({(side_count, seed) for side_count, seed, _ in grouped})
    for side_count, seed in strata:
        for iid_arm, smooth_arm in SMOOTH_IID_CV_COMPARISONS:
            iid_values = [row["metrics"]["support_cv"] for row in grouped.get((side_count, seed, iid_arm), [])]
            smooth_values = [row["metrics"]["support_cv"] for row in grouped.get((side_count, seed, smooth_arm), [])]
            iid_mean = mean(iid_values) if iid_values else None
            smooth_mean = mean(smooth_values) if smooth_values else None
            difference = abs(iid_mean - smooth_mean) if iid_mean is not None and smooth_mean is not None else None
            result.append(
                {
                    "schema": "generator-support-process-cv-match-v1",
                    "conditioning": (
                        "complete_paired_conditioned_on_every_arm_succeeding"
                        if complete_only
                        else "arm_marginal_accepted_shapes"
                    ),
                    "side_count": side_count,
                    "seed": seed,
                    "iid_arm": iid_arm,
                    "smooth_arm": smooth_arm,
                    "predeclared_absolute_tolerance": CV_MATCH_ABS_TOLERANCE,
                    "iid_mean_support_cv": iid_mean,
                    "smooth_mean_support_cv": smooth_mean,
                    "absolute_difference": difference,
                    "matched": difference is not None and difference <= CV_MATCH_ABS_TOLERANCE,
                    "post_hoc_tuning_performed": False,
                }
            )
    return result


def sigma_monotonicity_rows(attempts: Sequence[dict[str, Any]], complete_only: bool) -> list[dict[str, Any]]:
    grouped = group_accepted(attempts, complete_only)
    result = []
    strata = sorted({(side_count, seed) for side_count, seed, _ in grouped})
    for side_count, seed in strata:
        for metric in ("support_cv", "log_support_roughness", "source_support_l2", "width_anisotropy"):
            medians = []
            counts = []
            for arm in SIGMA_LADDER:
                values = [row["metrics"].get(metric) for row in grouped.get((side_count, seed, arm), [])]
                finite = [value for value in values if value is not None and math.isfinite(value)]
                medians.append(percentile(finite, 0.5))
                counts.append(len(finite))
            evaluable = all(value is not None for value in medians)
            nondecreasing = evaluable and all(
                medians[index] <= medians[index + 1] + 1.0e-12 for index in range(len(medians) - 1)
            )
            result.append(
                {
                    "schema": "generator-support-process-sigma-monotonicity-v1",
                    "conditioning": (
                        "complete_paired_conditioned_on_every_arm_succeeding"
                        if complete_only
                        else "arm_marginal_accepted_shapes"
                    ),
                    "side_count": side_count,
                    "seed": seed,
                    "metric": metric,
                    "arms_in_order": list(SIGMA_LADDER),
                    "counts": counts,
                    "medians": medians,
                    "evaluable": evaluable,
                    "nondecreasing": nondecreasing,
                    "post_hoc_tuning_performed": False,
                }
            )
    return result


def aggregate_report(
    seeds: Sequence[int],
    side_counts: Sequence[int],
    fans_per_stratum: int,
    fans: Sequence[dict[str, Any]],
    attempts: Sequence[dict[str, Any]],
    complete_fans: Sequence[dict[str, Any]],
    summaries: dict[str, list[dict[str, Any]]],
) -> dict[str, Any]:
    expected_fans = len(seeds) * len(side_counts) * fans_per_stratum
    expected_attempts = expected_fans * len(ARMS)
    accepted = sum(bool(row["accepted"]) for row in attempts)
    failures_by_arm: dict[str, dict[str, int]] = {}
    for arm in ARMS:
        failures_by_arm[arm] = dict(sorted(failure_counts([row for row in attempts if row["arm"] == arm]).items()))
    cv_rows = summaries["cv_matching"]
    monotonic_rows = summaries["sigma_monotonicity"]
    return {
        "schema": "generator-support-process-atlas-report-v1",
        "law_version": LAW_VERSION,
        "question": "On identical frozen normal fans, how do facetwise IID and low-frequency correlated support processes change factor geometry, acceptance, diversity, and tails?",
        "design": {
            "seeds": list(seeds),
            "side_counts": list(side_counts),
            "fans_per_seed_side_count": fans_per_stratum,
            "arms": list(ARMS),
            "normal_fan_policy": "one deterministic IID-uniform sorted fan per fan_id; never redrawn after any arm failure",
            "support_latent_policy": "one deterministic independent latent per fan_id/arm; never redrawn after failure",
            "marginal_boundary": "all arm attempts on all requested frozen fans",
            "complete_boundary": "subset of fan_ids on which every requested arm accepted; not the original marginal laws",
            "support_grid_size": SUPPORT_GRID_SIZE,
            "area_and_translation_normalization": "accepted polygons scaled to area one and translated by their area centroid",
        },
        "predeclared_questions": {
            "smooth_vs_iid_cv_pairs": [list(pair) for pair in SMOOTH_IID_CV_COMPARISONS],
            "cv_absolute_match_tolerance": CV_MATCH_ABS_TOLERANCE,
            "sigma_ladder": list(SIGMA_LADDER),
            "sigma_metrics": ["support_cv", "log_support_roughness", "source_support_l2", "width_anisotropy"],
            "post_hoc_tuning_performed": False,
        },
        "counts": {
            "expected_fans": expected_fans,
            "observed_fans": len(fans),
            "expected_arm_attempts": expected_attempts,
            "observed_arm_attempts": len(attempts),
            "accepted_arm_attempts": accepted,
            "failed_arm_attempts": len(attempts) - accepted,
            "complete_fans": len(complete_fans),
            "failures_by_arm": failures_by_arm,
        },
        "calibration_results": {
            "cv_match_evaluable": sum(row["absolute_difference"] is not None for row in cv_rows),
            "cv_match_passed": sum(bool(row["matched"]) for row in cv_rows),
            "cv_match_failed": sum(row["absolute_difference"] is not None and not row["matched"] for row in cv_rows),
            "sigma_monotonic_evaluable": sum(bool(row["evaluable"]) for row in monotonic_rows),
            "sigma_monotonic_passed": sum(bool(row["nondecreasing"]) for row in monotonic_rows),
            "sigma_monotonic_failed": sum(row["evaluable"] and not row["nondecreasing"] for row in monotonic_rows),
        },
        "artifacts": {
            "fans": "fans.jsonl",
            "attempts": "attempts.jsonl",
            "complete_fans": "complete-fans.jsonl",
            "metric_summaries": "metric-summaries.jsonl",
            "paired_distances": "paired-distances.jsonl",
            "within_diversity": "within-diversity.jsonl",
            "directed_overlap": "directed-overlap.jsonl",
            "cv_matching": "cv-matching.jsonl",
            "sigma_monotonicity": "sigma-monotonicity.jsonl",
        },
        "allowed_interpretation": "Target-free descriptive geometry and conditioning-aware generator calibration on the declared finite panel.",
        "prohibited_interpretation": "No sys/capacity, population support or density, natural-law ranking, causal mechanism, or thesis theorem claim.",
    }


def run_git(*args: str) -> str | None:
    try:
        result = subprocess.run(["git", *args], check=True, capture_output=True, text=True)
    except (OSError, subprocess.CalledProcessError):
        return None
    return result.stdout.strip()


def source_provenance() -> dict[str, Any]:
    revision = run_git("rev-parse", "HEAD")
    tree = run_git("rev-parse", "HEAD^{tree}")
    status = run_git("status", "--porcelain=v1", "--untracked-files=no")
    source_hashes = {}
    for relative in SOURCE_FILES:
        path = Path(relative)
        source_hashes[relative] = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
    return {
        "source_revision": revision,
        "source_tree": tree,
        "tracked_source_dirty": status is None or bool(status),
        "tracked_status": status,
        "tracked_clean_command": "git status --porcelain=v1 --untracked-files=no",
        "snapshot_timing": "captured before artifact directory creation",
        "python_implementation": platform.python_implementation(),
        "python_version": platform.python_version(),
        "stdlib_only": True,
        "source_hashes": source_hashes,
    }


def write_jsonl(path: Path, rows: Sequence[dict[str, Any]]) -> None:
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        for row in rows:
            handle.write(stable_json(row))
            handle.write("\n")


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def produce(out_dir: Path, seeds: Sequence[int], side_counts: Sequence[int], fans_per_stratum: int) -> dict[str, Any]:
    provenance = source_provenance()
    out_dir.mkdir(parents=True, exist_ok=True)
    fans, attempts = generate_rows(seeds, side_counts, fans_per_stratum)
    complete_fans = mark_complete_fans(attempts)
    summaries = {
        "metric_summaries": metric_summary_rows(attempts, "marginal") + metric_summary_rows(attempts, "complete_paired"),
        "paired_distances": paired_source_distance_rows(attempts, False) + paired_source_distance_rows(attempts, True),
        "within_diversity": within_diversity_rows(attempts, False) + within_diversity_rows(attempts, True),
        "directed_overlap": directed_overlap_rows(attempts, False) + directed_overlap_rows(attempts, True),
        "cv_matching": cv_matching_rows(attempts, False) + cv_matching_rows(attempts, True),
        "sigma_monotonicity": sigma_monotonicity_rows(attempts, False) + sigma_monotonicity_rows(attempts, True),
    }
    paths = {
        "fans.jsonl": fans,
        "attempts.jsonl": attempts,
        "complete-fans.jsonl": complete_fans,
        "metric-summaries.jsonl": summaries["metric_summaries"],
        "paired-distances.jsonl": summaries["paired_distances"],
        "within-diversity.jsonl": summaries["within_diversity"],
        "directed-overlap.jsonl": summaries["directed_overlap"],
        "cv-matching.jsonl": summaries["cv_matching"],
        "sigma-monotonicity.jsonl": summaries["sigma_monotonicity"],
    }
    for filename, rows in paths.items():
        write_jsonl(out_dir / filename, rows)
    report = aggregate_report(seeds, side_counts, fans_per_stratum, fans, attempts, complete_fans, summaries)
    report["provenance"] = provenance
    report["command"] = " ".join(sys.argv)
    report_path = out_dir / "report.json"
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True, allow_nan=False) + "\n", encoding="utf-8")
    hashed_names = sorted([*paths.keys(), "report.json"])
    checksums = "".join(f"{file_sha256(out_dir / name)}  {name}\n" for name in hashed_names)
    (out_dir / "checksums.sha256").write_text(checksums, encoding="utf-8")
    manifest = {
        "schema": "generator-support-process-atlas-manifest-v1",
        "law_version": LAW_VERSION,
        "provenance": provenance,
        "artifact_hashes": {name: file_sha256(out_dir / name) for name in [*hashed_names, "checksums.sha256"]},
        "artifact_directory": out_dir.as_posix(),
    }
    (out_dir / "manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True, allow_nan=False) + "\n", encoding="utf-8"
    )
    expected_attempts = len(seeds) * len(side_counts) * fans_per_stratum * len(ARMS)
    complete_contract = (
        len(fans) == len(seeds) * len(side_counts) * fans_per_stratum
        and len(attempts) == expected_attempts
        and all(row["failure_reason"] is not None for row in attempts if not row["accepted"])
        and all(len(record["sample_ids_by_arm"]) == len(ARMS) for record in complete_fans)
    )
    if provenance["tracked_source_dirty"] or provenance["source_revision"] is None or provenance["source_tree"] is None:
        raise RuntimeError("source provenance is not a clean pinned repository snapshot; artifacts were written for diagnosis")
    if not complete_contract:
        raise RuntimeError("row/failure/complete-pair contract failed; artifacts were written for diagnosis")
    return report


def parse_ints(value: str) -> tuple[int, ...]:
    parsed = tuple(int(part) for part in value.split(",") if part)
    if not parsed:
        raise argparse.ArgumentTypeError("need at least one integer")
    return parsed


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=Path("experiments/sys-datascience/methods/generator-support-process-atlas/artifacts"),
    )
    parser.add_argument("--seeds", type=parse_ints, default=DEFAULT_SEEDS)
    parser.add_argument("--side-counts", type=parse_ints, default=DEFAULT_SIDE_COUNTS)
    parser.add_argument("--fans-per-stratum", type=int, default=DEFAULT_FANS_PER_STRATUM)
    args = parser.parse_args()
    if args.fans_per_stratum <= 0:
        parser.error("--fans-per-stratum must be positive")
    if any(side_count < 3 for side_count in args.side_counts):
        parser.error("side counts must be at least three")
    return args


def main() -> int:
    args = parse_args()
    try:
        report = produce(args.out_dir, args.seeds, args.side_counts, args.fans_per_stratum)
    except Exception as error:  # fail closed after diagnostic artifacts when possible
        print(f"producer failed: {error}", file=sys.stderr)
        return 1
    print(
        stable_json(
            {
                "artifact_directory": args.out_dir.as_posix(),
                "counts": report["counts"],
                "calibration_results": report["calibration_results"],
            }
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
