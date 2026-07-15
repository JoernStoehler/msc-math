#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==1.26.4"]
# ///

"""Independent-pilot rare-region hit curves for target-free factor streams.

The producer rows are consumed in file order.  Regions are frozen from the
pilot seed only; confirmation streams are never used to choose thresholds.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import random
import re
import subprocess
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable

import numpy as np


SCHEMA = "generator-rare-hit-curves-v1"
ROW_SCHEMA = "factor-shape-row-v1"
VIEW_NAMES = (
    "covariance_anisotropy",
    "isoperimetric_ratio",
    "support_roughness",
    "central_symmetry_residual",
)
PREFIXES = (1, 2, 4, 8, 16, 24)
TARGET_KEYS = {"sys", "capacity", "target", "bounce", "ehz"}
EXPECTED_CONFIRMATION_SEEDS = (20260717, 20260718)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def json_hash(value: Any) -> str:
    return hashlib.sha256(
        json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()


def wilson(successes: int, trials: int, z: float = 1.959963984540054) -> tuple[float, float]:
    if trials <= 0:
        return 0.0, 1.0
    p = successes / trials
    denominator = 1.0 + z * z / trials
    center = (p + z * z / (2.0 * trials)) / denominator
    half = z * math.sqrt(p * (1.0 - p) / trials + z * z / (4.0 * trials * trials)) / denominator
    return max(0.0, center - half), min(1.0, center + half)


def polygon_area(vertices: np.ndarray) -> float:
    return float(
        0.5
        * np.sum(
            vertices[:, 0] * np.roll(vertices[:, 1], -1)
            - vertices[:, 1] * np.roll(vertices[:, 0], -1)
        )
    )


def support_vector(vertices: np.ndarray, grid: int = 64) -> np.ndarray:
    area = abs(polygon_area(vertices))
    if not math.isfinite(area) or area <= 0.0:
        raise ValueError("non-positive polygon area")
    centered = vertices - np.mean(vertices, axis=0)
    centered = centered / math.sqrt(area)
    angles = 2.0 * math.pi * np.arange(grid, dtype=float) / grid
    directions = np.column_stack((np.cos(angles), np.sin(angles)))
    support = np.max(centered @ directions.T, axis=0)
    if not np.all(np.isfinite(support)) or float(np.min(support)) <= 0.0:
        raise ValueError("invalid support vector")
    return support


def validate_geometry(row: dict[str, Any], path: Path, line_number: int) -> None:
    context = f"{path}:{line_number}"
    side = row.get("side_count")
    if not isinstance(side, int) or isinstance(side, bool) or side < 3:
        raise ValueError(f"{context}: side_count must be an integer >= 3")
    vertices = np.asarray(row.get("vertices_ccw"), dtype=float)
    if vertices.shape != (side, 2) or not np.all(np.isfinite(vertices)):
        raise ValueError(f"{context}: vertices_ccw shape/finite contract failed")
    area = polygon_area(vertices)
    scale = max(float(np.max(np.linalg.norm(vertices - vertices.mean(axis=0), axis=1))), 1.0)
    if area <= 1e-12 * scale * scale:
        raise ValueError(f"{context}: polygon area is non-positive or degenerate")
    edges = np.roll(vertices, -1, axis=0) - vertices
    turns = edges[:, 0] * np.roll(edges, -1, axis=0)[:, 1] - edges[:, 1] * np.roll(edges, -1, axis=0)[:, 0]
    if np.any(turns <= 1e-12 * scale * scale):
        raise ValueError(f"{context}: vertices_ccw must be strictly convex and CCW")
    if row.get("area_normalized") is not True or row.get("factor_role") != "single":
        raise ValueError(f"{context}: factor-only area/factor-role contract failed")


def contains_target_key(value: Any) -> bool:
    if isinstance(value, dict):
        return any(key in TARGET_KEYS or contains_target_key(nested) for key, nested in value.items())
    if isinstance(value, list):
        return any(contains_target_key(nested) for nested in value)
    return False


def rotation_distance(left: np.ndarray, right: np.ndarray) -> float:
    if left.shape != right.shape:
        raise ValueError("support grid mismatch")
    right_mean = float(np.mean(right))
    left_mean = float(np.mean(left))
    left_n = left / max(left_mean, 1e-15)
    right_n = right / max(right_mean, 1e-15)
    rolls = np.stack([np.roll(right_n, shift) for shift in range(len(right_n))])
    return float(math.sqrt(float(np.min(np.mean((rolls - left_n[None, :]) ** 2, axis=1)))))


def rotation_distances_to(left: np.ndarray, rights: list[np.ndarray]) -> np.ndarray:
    """Vectorized distances from one support to many supports."""
    left_n = left / max(float(np.mean(left)), 1e-15)
    right_n = np.stack(rights) / np.maximum(np.mean(np.stack(rights), axis=1, keepdims=True), 1e-15)
    best = np.full(len(rights), np.inf)
    for shift in range(left_n.size):
        rolled = np.roll(right_n, shift, axis=1)
        best = np.minimum(best, np.mean((rolled - left_n[None, :]) ** 2, axis=1))
    return np.sqrt(best)


def row_views(row: dict[str, Any]) -> dict[str, Any]:
    vertices = np.asarray(row.get("vertices_ccw", row.get("vertices")), dtype=float)
    if vertices.ndim != 2 or vertices.shape[1] != 2 or len(vertices) < 3:
        raise ValueError("vertices_ccw must be an n x 2 array")
    support = support_vector(vertices)
    centered = vertices - np.mean(vertices, axis=0)
    edges = np.roll(centered, -1, axis=0) - centered
    lengths = np.linalg.norm(edges, axis=1)
    perimeter = float(np.sum(lengths) / math.sqrt(abs(polygon_area(vertices))))
    covariance = np.cov(centered, rowvar=False)
    eig = np.linalg.eigvalsh(covariance)
    anisotropy = float(max(eig[-1], 0.0) / max(eig[0], 1e-15))
    isoperimetric = perimeter * perimeter / (4.0 * math.pi)
    vertex_angles = np.mod(np.arctan2(centered[:, 1], centered[:, 0]), 2.0 * math.pi)
    vertex_angles.sort()
    gaps = np.diff(np.concatenate((vertex_angles, vertex_angles[:1] + 2.0 * math.pi)))
    gap_cv = float(np.std(gaps) / max(np.mean(gaps), 1e-15))
    second = np.roll(support, -1) - 2.0 * support + np.roll(support, 1)
    roughness = float(np.std(second) / max(float(np.mean(support)), 1e-15))
    half = len(support) // 2
    symmetry = float(np.mean(np.abs(support - np.roll(support, half))) / max(float(np.mean(support)), 1e-15))
    return {
        "support": support,
        "covariance_anisotropy": anisotropy,
        "isoperimetric_ratio": isoperimetric,
        "support_roughness": roughness,
        "central_symmetry_residual": symmetry,
        "angular_gap_cv": gap_cv,
    }


def load_rows(paths: Iterable[Path]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    rows: list[dict[str, Any]] = []
    inputs: list[dict[str, Any]] = []
    seen: set[str] = set()
    for path in paths:
        input_record = {"path": str(path), "sha256": sha256(path), "bytes": path.stat().st_size, "rows": 0, "schema": ROW_SCHEMA}
        inputs.append(input_record)
        with path.open() as handle:
            for line_number, line in enumerate(handle, 1):
                if not line.strip():
                    continue
                row = json.loads(line)
                if row.get("schema") != ROW_SCHEMA:
                    raise ValueError(f"{path}:{line_number}: unexpected schema")
                sample_id = row.get("sample_id")
                if not isinstance(sample_id, str) or sample_id in seen:
                    raise ValueError(f"{path}:{line_number}: duplicate or invalid sample_id")
                if contains_target_key(row):
                    raise ValueError(f"{path}:{line_number}: target-bearing key present")
                if not isinstance(row.get("attempt"), int) or row["attempt"] < 0:
                    raise ValueError(f"{path}:{line_number}: attempt must be a non-negative integer")
                if not isinstance(row.get("seed"), int) or not isinstance(row.get("row_index"), int) or row["row_index"] < 0:
                    raise ValueError(f"{path}:{line_number}: seed/row_index contract failed")
                if not isinstance(row.get("population"), str) or not row["population"] or not isinstance(row.get("parameter"), str):
                    raise ValueError(f"{path}:{line_number}: population/parameter contract failed")
                expected_id = re.search(r"seed=(\d+)/side=(\d+)/row=(\d+)/attempt=(\d+)/", sample_id)
                if expected_id is None or tuple(map(int, expected_id.groups())) != (row["seed"], row["side_count"], row["row_index"], row["attempt"]):
                    raise ValueError(f"{path}:{line_number}: sample_id linkage contract failed")
                validate_geometry(row, path, line_number)
                # Compute views now, while preserving the producer's row order.
                row = dict(row)
                row["_source_path"] = str(path)
                row["_views"] = row_views(row)
                seen.add(sample_id)
                rows.append(row)
                input_record["rows"] += 1
    if not rows:
        raise ValueError("no rows")
    return rows, inputs


def stream_key(row: dict[str, Any]) -> tuple[int, str, int]:
    return int(row["seed"]), str(row.get("population", row.get("law"))), int(row["side_count"])


def grouped_streams(rows: list[dict[str, Any]]) -> dict[tuple[int, str, int], list[dict[str, Any]]]:
    result: dict[tuple[int, str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        result[stream_key(row)].append(row)
    for key, stream in result.items():
        row_indices = [int(row["row_index"]) for row in stream]
        if len(row_indices) != len(set(row_indices)):
            raise ValueError(f"duplicate row_index in confirmation stream {key}")
        if row_indices != sorted(row_indices):
            raise ValueError(f"confirmation stream {key} is not in original row order")
    # Never sort rows: producer order is the time axis.  Stable group ordering is
    # used only for deterministic output.
    return {key: result[key] for key in sorted(result)}


def make_regions(pilot_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_side: dict[int, list[dict[str, Any]]] = defaultdict(list)
    for row in pilot_rows:
        by_side[int(row["side_count"])].append(row)
    regions: list[dict[str, Any]] = []
    for side, members in sorted(by_side.items()):
        for view in VIEW_NAMES:
            values = np.asarray([float(row["_views"][view]) for row in members])
            low, high = float(np.quantile(values, 0.10)), float(np.quantile(values, 0.90))
            regions.extend(
                [
                    {
                        "name": f"{view}_low",
                        "side_count": side,
                        "view": view,
                        "direction": "low",
                        "threshold": low,
                        "pilot_n": len(values),
                        "pilot_hits": int(np.sum(values <= low)),
                        "quantile": 0.10,
                        "definition": "fixed lower 0.10 pooled-pilot quantile; ties count as hits",
                    },
                    {
                        "name": f"{view}_high",
                        "side_count": side,
                        "view": view,
                        "direction": "high",
                        "threshold": high,
                        "pilot_n": len(values),
                        "pilot_hits": int(np.sum(values >= high)),
                        "quantile": 0.90,
                        "definition": "fixed upper 0.90 pooled-pilot quantile; ties count as hits",
                    },
                ]
            )
        # Novelty is a separate support-shape view.  Pilot self-neighbours are
        # excluded, so the frontier threshold is not spuriously zero.
        supports = [row["_views"]["support"] for row in members]
        nearest = []
        for i, support in enumerate(supports):
            others = [other for j, other in enumerate(supports) if i != j]
            nearest.append(float(np.min(rotation_distances_to(support, others))) if others else 0.0)
        threshold = float(np.quantile(np.asarray(nearest), 0.90))
        regions.append(
            {
                "name": "support_novelty_high",
                "side_count": side,
                "view": "support_novelty_distance",
                "direction": "high",
                "threshold": threshold,
                "pilot_n": len(members),
                "pilot_hits": int(np.sum(np.asarray(nearest) >= threshold)),
                "quantile": 0.90,
                "definition": "fixed upper 0.90 quantile of pilot nearest-neighbour rotation-quotiented support distance; confirmation distance is to pooled pilot support set",
            }
        )
    return regions


def region_hit(row: dict[str, Any], region: dict[str, Any], pilot_rows: list[dict[str, Any]]) -> bool:
    if int(row["side_count"]) != int(region["side_count"]):
        return False
    if region["view"] == "support_novelty_distance":
        return float(row["_views"]["support_novelty_distance"]) >= float(region["threshold"])
    value = float(row["_views"][region["view"]])
    threshold = float(region["threshold"])
    return value <= threshold if region["direction"] == "low" else value >= threshold


def attach_novelty_distances(rows: list[dict[str, Any]], pilot_rows: list[dict[str, Any]]) -> None:
    """Cache distance to the pooled pilot support set once per confirmation row."""
    by_side: dict[int, list[np.ndarray]] = defaultdict(list)
    for row in pilot_rows:
        by_side[int(row["side_count"])].append(row["_views"]["support"])
    for row in rows:
        candidates = by_side[int(row["side_count"])]
        row["_views"]["support_novelty_distance"] = float(np.min(rotation_distances_to(row["_views"]["support"], candidates)))


def producer_costs(report_paths: Iterable[Path]) -> dict[tuple[int, str, int], dict[str, Any]]:
    costs: dict[tuple[int, str, int], dict[str, Any]] = {}
    for path in report_paths:
        report = json.loads(path.read_text())
        seed = int(report["seed"])
        max_attempts = int(report["max_attempts_per_row"])
        for item in report["per_population"]:
            key = (seed, f"{item['law']}[{item['parameter']}]", int(item["side_count"]))
            accepted = int(item["accepted"])
            exhausted = int(item["exhausted"])
            attempts = int(item.get("attempts_total", sum([])))
            # Reports do not retain the full rejection trace.  Accepted row
            # attempts are exact; exhausted rows are charged their declared cap.
            if attempts == 0:
                attempts = accepted + exhausted * max_attempts
            costs[key] = {
                "requested": int(item["requested"]),
                "accepted": accepted,
                "exhausted": exhausted,
                "attempts_total": attempts,
                "rejections": attempts - accepted,
                "max_attempts_per_row": max_attempts,
                "generator_ms": float(item["total_generation_ms"]),
                "report": str(path),
                "report_sha256": sha256(path),
                "report_schema": report.get("schema"),
                "report_source_revision": report.get("source_revision"),
                "report_source_dirty": report.get("source_dirty"),
            }
    return costs


def analyze_curves(
    pilot_rows: list[dict[str, Any]],
    confirmation_rows: list[dict[str, Any]],
    regions: list[dict[str, Any]],
    costs: dict[tuple[int, str, int], dict[str, Any]],
    roles: dict[int, str],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    streams = grouped_streams(confirmation_rows)
    curve_rows: list[dict[str, Any]] = []
    stream_rows: list[dict[str, Any]] = []
    summaries: list[dict[str, Any]] = []
    stratum_findings: list[dict[str, Any]] = []
    confirmation_seeds = tuple(sorted({key[0] for key in streams}))
    if confirmation_seeds != EXPECTED_CONFIRMATION_SEEDS:
        raise ValueError(f"expected exactly confirmation seeds {EXPECTED_CONFIRMATION_SEEDS}, got {confirmation_seeds}")
    for region in regions:
        pilot_values = [
            float(row["_views"][region["view"]])
            for row in pilot_rows
            if int(row["side_count"]) == int(region["side_count"])
            and region["view"] in row["_views"]
        ]
        pilot_min = min(pilot_values) if pilot_values else 0.0
        pilot_max = max(pilot_values) if pilot_values else 0.0
        by_stratum: dict[tuple[str, int], dict[int, tuple[list[bool], list[int], int | None, bool]]] = defaultdict(dict)
        for key, stream in streams.items():
            seed, population, side = key
            if side != int(region["side_count"]):
                continue
            events = [region_hit(row, region, pilot_rows) for row in stream]
            first = next((index + 1 for index, hit in enumerate(events) if hit), None)
            # The producer records zero-based rejection count in `attempt`;
            # charge the accepted draw itself as one generator attempt.
            attempts = [int(row["attempt"]) + 1 for row in stream]
            first_attempt = sum(attempts[:first]) if first is not None else None
            total_attempts = sum(attempts)
            cost = costs.get(key, {})
            if region["view"] == "support_novelty_distance":
                support_overlap = 1.0
            else:
                stream_values = [float(row["_views"][region["view"]]) for row in stream]
                stream_min, stream_max = min(stream_values), max(stream_values)
                overlap = max(0.0, min(pilot_max, stream_max) - max(pilot_min, stream_min))
                width = min(pilot_max - pilot_min, stream_max - stream_min)
                support_overlap = 1.0 if width <= 1e-15 else overlap / width
            eligible = support_overlap >= 0.25
            stream_rows.append(
                {
                    "region": region["name"],
                    "seed_role": roles.get(seed, "unknown"),
                    "seed": seed,
                    "population": population,
                    "side_count": side,
                    "accepted_rows": len(stream),
                    "requested_rows": cost.get("requested"),
                    "exhausted_rows": cost.get("exhausted"),
                    "attempts_total_accepted_prefix": total_attempts,
                    "rejections_in_accepted_prefix": total_attempts - len(stream),
                    "attempts_total_with_exhaustion": cost.get("attempts_total"),
                    "rejections_with_exhaustion": cost.get("rejections"),
                    "first_hit_accepted_index": first,
                    "first_hit_attempt": first_attempt,
                    "censored": first is None,
                    "generator_ms": cost.get("generator_ms"),
                    "support_overlap": support_overlap,
                    "comparison_eligible": eligible,
                }
            )
            if eligible:
                if seed in by_stratum[(population, side)]:
                    raise ValueError(f"duplicate confirmation stream for {seed}/{population}/{side}")
                by_stratum[(population, side)][seed] = (events, attempts, first, eligible)
        pairs = list(by_stratum.values())
        for (population, side), flags in sorted(by_stratum.items()):
            if tuple(sorted(flags)) != EXPECTED_CONFIRMATION_SEEDS:
                raise ValueError(f"missing or duplicate confirmation stream for {population}/{side}: {sorted(flags)}")
            ordered = [flags[seed][2] is not None for seed in EXPECTED_CONFIRMATION_SEEDS]
            if ordered and all(ordered):
                stratum_classification = "replicates-both-confirmation-seeds"
            elif any(ordered):
                stratum_classification = "partial-one-confirmation-seed"
            else:
                stratum_classification = "not-reobserved-both-confirmation-seeds-right-censored"
            stratum_findings.append(
                {
                    "region": region["name"],
                    "population": population,
                    "side_count": side,
                    "confirmation_seed_flags": json.dumps({seed: flags[seed][2] is not None for seed in EXPECTED_CONFIRMATION_SEEDS}, sort_keys=True),
                    "confirmation_seeds_with_hit": sum(flags[seed][2] is not None for seed in EXPECTED_CONFIRMATION_SEEDS),
                    "confirmation_seed_count": len(EXPECTED_CONFIRMATION_SEEDS),
                    "classification": stratum_classification,
                    "interpretation": "stratum-level pilot-region re-observation label; no-hit is right-censored at 24 accepted rows and is not a zero-probability claim",
                }
            )
            # Aggregate over exactly the two independent confirmation streams.
            for prefix in PREFIXES:
                if any(prefix > len(flags[seed][0]) for seed in EXPECTED_CONFIRMATION_SEEDS):
                    continue
                successes = sum(any(flags[seed][0][:prefix]) for seed in EXPECTED_CONFIRMATION_SEEDS)
                trials = len(EXPECTED_CONFIRMATION_SEEDS)
                low, high = wilson(successes, trials)
                curve_rows.append(
                    {
                        "region": region["name"],
                        "population": population,
                        "side_count": side,
                        "accepted_prefix": prefix,
                        "hit_count": successes,
                        "survival_count": trials - successes,
                        "stream_count": trials,
                        "hit_rate": successes / trials,
                        "survival_rate": (trials - successes) / trials,
                        "wilson_low": low,
                        "wilson_high": high,
                        "attempts_prefix_total": sum(sum(flags[seed][1][:prefix]) for seed in EXPECTED_CONFIRMATION_SEEDS),
                        "first_hit_censored_streams": sum(flags[seed][2] is None for seed in EXPECTED_CONFIRMATION_SEEDS),
                        "interval_contract": "Wilson 95% over exactly two independent confirmation seed streams; descriptive only",
                    }
                )
        both = sum(all(flags[seed][2] is not None for seed in EXPECTED_CONFIRMATION_SEEDS) for flags in pairs)
        any_hit = sum(any(flags[seed][2] is not None for seed in EXPECTED_CONFIRMATION_SEEDS) for flags in pairs)
        if both:
            classification = "replicates-both-confirmation-seeds"
        elif any_hit:
            classification = "partial-one-confirmation-seed"
        else:
            classification = "not-reobserved-both-confirmation-seeds-right-censored"
        summary = dict(region)
        overlaps = [float(row["support_overlap"]) for row in stream_rows if row["region"] == region["name"]]
        summary.update(
            {
                "confirmation_streams": 2 * len(by_stratum),
                "confirmation_streams_with_hit": sum(flags[seed][2] is not None for flags in pairs for seed in EXPECTED_CONFIRMATION_SEEDS),
                "comparison_eligible_streams": sum(1 for row in stream_rows if row["region"] == region["name"] and row["comparison_eligible"]),
                "support_overlap_min": min(overlaps) if overlaps else None,
                "support_overlap_mean": float(np.mean(overlaps)) if overlaps else None,
                "strata_with_hits_both_seeds": both,
                "strata_with_hit_any_seed": any_hit,
                "classification": classification,
            }
        )
        summaries.append(summary)
    return curve_rows, stream_rows, summaries, stratum_findings


def synthetic_controls() -> list[dict[str, Any]]:
    """Small deterministic controls; all probabilities are known by construction."""
    rows: list[dict[str, Any]] = []
    for probability in (0.10, 0.01):
        for seed in range(5):
            rng = random.Random(17000 + seed)
            values = [rng.random() < probability for _ in range(5000)]
            first = next((i + 1 for i, value in enumerate(values) if value), None)
            k = sum(values)
            low, high = wilson(k, len(values))
            rows.append(
                {
                    "control": f"bernoulli_p={probability}",
                    "replicate": seed,
                    "n": len(values),
                    "known_probability": probability,
                    "hit_count": k,
                    "hit_rate": k / len(values),
                    "wilson_low": low,
                    "wilson_high": high,
                    "first_hit_index": first,
                    "censored": first is None,
                    "duplicate_count": len(values) - len(set(values)),
                    "order_contract": "original seeded IID order",
                }
            )
    duplicate = [0.25] * 64
    rows.append(
        {
            "control": "duplicate-stream",
            "replicate": 0,
            "n": len(duplicate),
            "known_probability": 0.0,
            "hit_count": 0,
            "hit_rate": 0.0,
            "wilson_low": 0.0,
            "wilson_high": wilson(0, len(duplicate))[1],
            "first_hit_index": None,
            "censored": True,
            "duplicate_count": len(duplicate) - len(set(duplicate)),
            "order_contract": "all values equal; duplicate stream must not be mistaken for IID evidence",
        }
    )
    no_hit = [0.0] * 64
    rows.append(
        {
            "control": "no-hit-censoring",
            "replicate": 0,
            "n": len(no_hit),
            "known_probability": 0.0,
            "hit_count": 0,
            "hit_rate": 0.0,
            "wilson_low": 0.0,
            "wilson_high": wilson(0, len(no_hit))[1],
            "first_hit_index": None,
            "censored": True,
            "duplicate_count": len(no_hit) - len(set(no_hit)),
            "order_contract": "threshold=1; no event by exhaustion",
        }
    )
    original = [0] * 99 + [1]
    first_original = next(i + 1 for i, value in enumerate(original) if value)
    rows.append(
        {
            "control": "order-original",
            "replicate": 0,
            "n": 100,
            "known_probability": 0.01,
            "hit_count": 1,
            "hit_rate": 0.01,
            "wilson_low": wilson(1, 100)[0],
            "wilson_high": wilson(1, 100)[1],
            "first_hit_index": first_original,
            "censored": False,
            "duplicate_count": 98,
            "order_contract": "same multiset, event last",
        }
    )
    for replicate in range(32):
        values = original.copy()
        random.Random(18000 + replicate).shuffle(values)
        first = next(i + 1 for i, value in enumerate(values) if value)
        rows.append(
            {
                "control": "order-permutation",
                "replicate": replicate,
                "n": 100,
                "known_probability": 0.01,
                "hit_count": 1,
                "hit_rate": 0.01,
                "wilson_low": wilson(1, 100)[0],
                "wilson_high": wilson(1, 100)[1],
                "first_hit_index": first,
                "censored": False,
                "duplicate_count": 98,
                "order_contract": "same multiset, seeded permutation",
            }
        )
    return rows


def write_tsv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fields = sorted({field for row in rows for field in row})
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: "NA" if row.get(field) is None else row[field] for field in fields})


def validate_completeness(
    pilot_rows: list[dict[str, Any]],
    confirmation_rows: list[dict[str, Any]],
    costs: dict[tuple[int, str, int], dict[str, Any]],
) -> None:
    pilot_seeds = sorted({int(row["seed"]) for row in pilot_rows})
    confirmation_seeds = tuple(sorted({int(row["seed"]) for row in confirmation_rows}))
    if len(pilot_seeds) != 1 or confirmation_seeds != EXPECTED_CONFIRMATION_SEEDS or pilot_seeds[0] in confirmation_seeds:
        raise ValueError(f"incomplete seed roles: pilot={pilot_seeds}, confirmation={confirmation_seeds}")
    streams = grouped_streams(confirmation_rows)
    strata_by_seed = {seed: {(population, side) for (stream_seed, population, side) in streams if stream_seed == seed} for seed in EXPECTED_CONFIRMATION_SEEDS}
    if strata_by_seed[EXPECTED_CONFIRMATION_SEEDS[0]] != strata_by_seed[EXPECTED_CONFIRMATION_SEEDS[1]]:
        raise ValueError("confirmation seed strata/linkage mismatch")
    for key, stream in streams.items():
        expected = costs.get(key)
        if expected is None:
            raise ValueError(f"missing producer report for confirmation stream {key}")
        if len(stream) != expected["accepted"]:
            raise ValueError(f"accepted-row count mismatch for {key}: rows={len(stream)} report={expected['accepted']}")
    expected_cost_keys = {(int(row["seed"]), str(row["population"]), int(row["side_count"])) for row in [*pilot_rows, *confirmation_rows]}
    if expected_cost_keys != set(costs):
        raise ValueError("producer-cost completeness check failed")


def source_provenance() -> dict[str, Any]:
    packet = Path(__file__).resolve().parent
    repo_root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()).resolve()
    source_files = {
        "analyzer": packet / "rare_hit_curves.py",
        "tests": packet / "test_rare_hit_curves.py",
        "readme": packet / "README.md",
    }
    relative_sources = [str(path.relative_to(repo_root)) for path in source_files.values()]
    repo_revision = subprocess.check_output(["git", "log", "-1", "--format=%H", "--", *relative_sources], text=True).strip()
    repo_tree = subprocess.check_output(["git", "rev-parse", f"{repo_revision}^{{tree}}"], text=True).strip()
    dirty = subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=no", "--", *relative_sources], text=True)
    if dirty:
        raise SystemExit("source files are dirty; commit source before artifact generation")
    return {
        "source_revision": repo_revision,
        "source_tree": repo_tree,
        "source_dirty": False,
        "tracked_clean_predicate": "git status --porcelain --untracked-files=no",
        "source_file_hashes": {name: sha256(path) for name, path in source_files.items()},
        "source_files": {name: str(path.relative_to(repo_root)) for name, path in source_files.items()},
        "input_schema": ROW_SCHEMA,
        "analyzer_schema": SCHEMA,
    }


def producer_report_records(paths: Iterable[Path]) -> list[dict[str, Any]]:
    records = []
    for path in paths:
        report = json.loads(path.read_text())
        if report.get("schema") != "generator-zoo-factor-only-report-v1":
            raise ValueError(f"unexpected producer report schema in {path}")
        records.append(
            {
                "path": str(path),
                "sha256": sha256(path),
                "schema": report["schema"],
                "seed": report["seed"],
                "accepted": report["factor_rows"],
                "requested": sum(item["requested"] for item in report["per_population"]),
                "exhausted": sum(item["exhausted"] for item in report["per_population"]),
                "source_revision": report.get("source_revision"),
                "source_dirty": report.get("source_dirty"),
            }
        )
    if len(records) != 6 or len({record["path"] for record in records}) != 6:
        raise ValueError("producer report completeness requires six distinct reports")
    return records
def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pilot-input", type=Path, action="append", required=True)
    parser.add_argument("--confirmation-input", type=Path, action="append", required=True)
    parser.add_argument("--producer-report", type=Path, action="append", required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    if len(args.pilot_input) != 2 or len(args.confirmation_input) != 4 or len(args.producer_report) != 6:
        raise SystemExit("protocol requires exactly 2 pilot data shards, 4 confirmation data shards, and 6 producer reports")
    args.out_dir.mkdir(parents=True, exist_ok=True)
    pilot_rows, pilot_inputs = load_rows(args.pilot_input)
    confirmation_rows, confirmation_inputs = load_rows(args.confirmation_input)
    pilot_seeds = sorted({int(row["seed"]) for row in pilot_rows})
    confirmation_seeds = sorted({int(row["seed"]) for row in confirmation_rows})
    costs = producer_costs(args.producer_report)
    validate_completeness(pilot_rows, confirmation_rows, costs)
    source = source_provenance()
    producer_reports = producer_report_records(args.producer_report)
    roles = {pilot_seeds[0]: "pilot", **{seed: f"confirmation-{index + 1}" for index, seed in enumerate(EXPECTED_CONFIRMATION_SEEDS)}}
    regions = make_regions(pilot_rows)
    attach_novelty_distances(confirmation_rows, pilot_rows)
    curves, streams, summaries, stratum_findings = analyze_curves(pilot_rows, confirmation_rows, regions, costs, roles)
    synthetic = synthetic_controls()
    write_tsv(args.out_dir / "hit-curves.tsv", curves)
    write_tsv(args.out_dir / "stream-summary.tsv", streams)
    write_tsv(args.out_dir / "stratum-findings.tsv", stratum_findings)
    write_tsv(args.out_dir / "synthetic-controls.tsv", synthetic)
    (args.out_dir / "pilot-regions.json").write_text(json.dumps(regions, indent=2, sort_keys=True) + "\n")
    report = {
        "schema": SCHEMA,
        "protocol": {
            "pilot_seed": pilot_seeds[0],
            "confirmation_seeds": confirmation_seeds,
            "seed_roles": roles,
            "selection_boundary": "all region thresholds and support-frontier distances are computed from pilot rows only; confirmation rows are consumed in original producer file order",
            "views": {
                "covariance_anisotropy": "largest/smallest centered vertex covariance eigenvalue",
                "isoperimetric_ratio": "perimeter^2/(4*pi*area), after translation/scale normalization",
                "support_roughness": "standard deviation of 64-grid support second differences divided by mean support",
                "central_symmetry_residual": "mean absolute half-period support mismatch divided by mean support",
                "support_novelty_distance": "minimum circular-shift L2 distance of mean-normalized supports to pooled pilot support set",
            },
            "quantile_regions": "fixed pooled-pilot lower/upper 0.10 cells, with ties included",
            "uncertainty": "Wilson 95% intervals aggregate exactly two independent confirmation seed streams per law/side/prefix; intervals are intentionally wide and descriptive, with no per-row CI",
            "censoring": "first_hit_index is right-censored at accepted-prefix exhaustion when no event appears",
            "cost_contract": "generator_ms comes from producer reports; analyzer runtime is intentionally not serialized because it is volatile",
        },
        "rows": {
            "pilot": len(pilot_rows),
            "confirmation": len(confirmation_rows),
            "pilot_inputs": pilot_inputs,
            "confirmation_inputs": confirmation_inputs,
        },
        "regions": summaries,
        "producer_costs": [dict({"seed": key[0], "population": key[1], "side_count": key[2]}, **value) for key, value in sorted(costs.items())],
        "producer_reports": producer_reports,
        "synthetic_controls": {
            "known_rare_probabilities": [0.1, 0.01],
            "duplicate_stream": "64 identical values; no IID or hit-rate inference",
            "no_hit_censoring": "64 rows, region never hit; first hit is censored",
            "order_permutation": "one event in 100 rows, event-last original and 32 seeded permutations",
        },
        "provenance": {
            "source_root": "repository checkout containing this packet (resolved with git rev-parse --show-toplevel)",
            **source,
            "accepted_base_commit": "3f09eeebbcaae731d493317b63fc6ece127e804d",
            "input_target_field_scan": "passed recursively; keys sys/capacity/target/bounce/ehz absent",
            "transitive_hash_contract": "all six factor data shards and all six producer reports are listed with SHA-256, schema, seed, row/acceptance counts, and producer source provenance",
        },
        "interpretation": {
            "allowed": [
                "online accepted-prefix first-hit and censoring descriptions for named factor streams",
                "law-by-side contrasts only when the frozen region and finite support overlap are reported",
                "generator attempt/rejection/exhaustion and generator-versus-validation cost accounting",
                "replication labels across the two independent confirmation seeds",
            ],
            "prohibited": [
                "universal geometric extremes or population-support probabilities",
                "ranking laws by one aggregate rare-hit score",
                "pooling side counts, facet counts, or law knobs",
                "target/sys/capacity transfer, mechanism, or online sampler promotion",
                "using confirmation rows to invent or retune regions",
            ],
            "failed_or_deferred": [
                "no-hit streams remain right-censored rather than zero-probability claims",
                "pilot-defined regions with no confirmation event are not reobserved under two right-censored confirmation streams; this is not a zero-probability or universal-artifact conclusion",
                "small stratum counts are descriptive and not asymptotic inference",
            ],
        },
    }
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"pilot_rows": len(pilot_rows), "confirmation_rows": len(confirmation_rows), "regions": len(regions)}))


if __name__ == "__main__":
    main()
