#!/usr/bin/env python3
"""Reconcile the frozen S0 artifacts and generate its comparison summary.

This is deliberately a verifier, not a permissive reporting script: a summary
is only meaningful when it has checked the accounting and genealogy which make
the fixed-budget comparison interpretable.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import statistics
import struct
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path
from typing import Any

ARMS = ("iid", "multistart_branch_local_phase0", "diagonal_cem")
CHECKPOINTS = (64, 128, 192, 256)
LEVELS = (0.85, 0.90, 0.95, 1.0)
TARGET_ATTEMPTS = 256
REPLICATES = range(3)
CEM_GENERATIONS = range(4)
CEM_POPULATION = 64
CEM_ELITES = 16
CEM_CONSTRUCTION_ATTEMPT_CAP = 640
CEM_DIMENSIONS = 17
PHASE_COORDINATE = 16
CEM_SMOOTHING = 0.5
CEM_VARIANCE_FLOOR_FRACTION = 0.05
CANDIDATE_ID = re.compile(r"^s0v1-[0-9a-f]{24}$")
CANONICAL_RATIONAL = re.compile(r"^-?(?:0|[1-9][0-9]*)(?:/(?:[1-9][0-9]*))?$")

# This duplicate is intentional.  It makes a changed resolved configuration
# fail closed rather than silently changing the meaning of an old analyzer.
FROZEN_CONFIG = {
    "schema_version": 1,
    "packet_version": "s0-equal-budget-product-search-v1",
    "bucket": {"q_facets": 5, "p_facets": 5, "h_min": 0.8, "h_max": 1.2},
    "master_seeds": [202607110001, 202607110002, 202607110003],
    "arms": list(ARMS),
    "target_attempts_per_arm_replicate": TARGET_ATTEMPTS,
    "checkpoints": list(CHECKPOINTS),
    "descriptive_levels": list(LEVELS),
    "local": {
        "within_step_fractions": [0.1, 0.25, 0.5, 0.75, 0.95],
        "overshoot_multipliers": [1.5, 2.0, 3.0],
        "overshoot_step_bound_cutoff": 100.0,
        "improvement_threshold": 0.000001,
        "wiggle_or_escape_rounds": False,
    },
    "cem": {
        "generations": 4,
        "population": CEM_POPULATION,
        "elites": CEM_ELITES,
        "smoothing": 0.5,
        "variance_floor_fraction_of_generation_0": 0.05,
        "construction_attempt_cap_per_generation": CEM_CONSTRUCTION_ATTEMPT_CAP,
        "phase_zero_resultant_fallback": "previous circular mean; generation 0 uses wrapped phase of lexicographically smallest candidate_id",
        "elite_order": "descending sys, then ascending candidate_id",
        "iid_fill_on_incomplete_generation": False,
    },
    "common_base_streams": {
        "iid_indices": "0..255",
        "cem_generation_0_indices": "0..63",
        "local_trajectory_starts": "successive base indices beginning at 0",
    },
    "pooled_top_eight": "best successful row per exact polytope_key across all replicates; eight largest sys; ordinary median of positions 4 and 5",
    "normalized_auc": "mean best-so-far value over charged attempts 1..256",
    "material_ahead": {
        "paired_replicates_required": 2,
        "final_best_margin_over_iid": 0.02,
        "pooled_top_eight_median_margin_over_iid": 0.01,
    },
    "stop": "flush and stop on a newly generated trusted sys > 1 unless demonstrably known-equivalent to a declared HKO/rotated-pentagon control",
}


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open(encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]


def fail(message: str) -> None:
    raise ValueError(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def value(row: dict[str, Any], field: str, context: str) -> Any:
    if field not in row:
        fail(f"{context}: missing {field}")
    return row[field]


def finite_number(raw: Any, context: str) -> float:
    require(isinstance(raw, (int, float)) and not isinstance(raw, bool) and math.isfinite(raw), f"{context}: expected finite number")
    return float(raw)


def close(actual: float, expected: float, context: str) -> None:
    require(math.isclose(actual, expected, rel_tol=1e-12, abs_tol=1e-12), f"{context}: expected {expected:.17g}, got {actual:.17g}")


def wrap_phase(phase: float) -> float:
    return phase % math.tau


def wrapped_deviation(angle: float) -> float:
    return (angle + math.pi) % math.tau - math.pi


def chart_coordinates(row: dict[str, Any], context: str) -> list[float]:
    chart = value(row, "product_chart", context)
    require(isinstance(chart, dict), f"{context}: missing canonical product chart")
    parts = (("q_gap_logits", 4), ("q_centered_log_radii", 5), ("p_gap_logits", 4), ("p_centered_log_radii", 5))
    coordinates: list[float] = []
    for field, length in parts:
        values = value(chart, field, context)
        require(isinstance(values, list) and len(values) == length, f"{context}: invalid {field}")
        coordinates.extend(finite_number(item, f"{context}: {field}") for item in values[:4])
        if length == 5:
            close(float(values[4]), -sum(float(item) for item in values[:4]), f"{context}: {field} zero-sum coordinate")
    phase = finite_number(value(chart, "relative_phase", context), f"{context}: relative_phase")
    require(0.0 <= phase < math.tau, f"{context}: relative_phase is not canonically wrapped")
    require(isinstance(value(chart, "near_tie", context), bool), f"{context}: near_tie is not boolean")
    coordinates.append(phase)
    require(len(coordinates) == CEM_DIMENSIONS, f"{context}: chart dimension mismatch")
    return coordinates


def chart_gap_radius_features(row: dict[str, Any], context: str) -> tuple[list[float], list[float]]:
    chart = value(row, "product_chart", context)
    require(isinstance(chart, dict), f"{context}: missing canonical product chart")
    gaps: list[float] = []
    radii: list[float] = []
    for gap_field, radius_field in (("q_gap_logits", "q_centered_log_radii"), ("p_gap_logits", "p_centered_log_radii")):
        logits = value(chart, gap_field, context)
        centered = value(chart, radius_field, context)
        require(isinstance(logits, list) and len(logits) == 4 and isinstance(centered, list) and len(centered) == 5, f"{context}: invalid chart feature arrays")
        logits = [finite_number(item, f"{context}: {gap_field}") for item in logits]
        shifted = [*logits, 0.0]
        maximum = max(shifted)
        weights = [math.exp(item - maximum) for item in shifted]
        total = sum(weights)
        gaps.extend(math.tau * weight / total for weight in weights)
        radii.extend(finite_number(item, f"{context}: {radius_field}") for item in centered)
    return gaps, radii


def chart_distance(left: list[float], right: list[float]) -> float:
    return math.sqrt(sum((left[index] - right[index]) ** 2 for index in range(PHASE_COORDINATE)) + wrapped_deviation(left[PHASE_COORDINATE] - right[PHASE_COORDINATE]) ** 2)


def resultant_is_zero(cosine: float, sine: float, count: int) -> bool:
    return math.hypot(cosine, sine) <= 1e-12 * count


def coordinate_moments(candidates: list[dict[str, Any]], fallback_phase: float) -> tuple[list[float], list[float]]:
    coordinates = [chart_coordinates(row, f"CEM candidate {row['candidate_id']}") for row in candidates]
    mean = [0.0] * CEM_DIMENSIONS
    variance = [0.0] * CEM_DIMENSIONS
    for index in range(PHASE_COORDINATE):
        mean[index] = statistics.fmean(point[index] for point in coordinates)
        variance[index] = statistics.fmean((point[index] - mean[index]) ** 2 for point in coordinates)
    cosine = sum(math.cos(point[PHASE_COORDINATE]) for point in coordinates)
    sine = sum(math.sin(point[PHASE_COORDINATE]) for point in coordinates)
    mean[PHASE_COORDINATE] = wrap_phase(fallback_phase) if resultant_is_zero(cosine, sine, len(coordinates)) else wrap_phase(math.atan2(sine, cosine))
    variance[PHASE_COORDINATE] = statistics.fmean(wrapped_deviation(point[PHASE_COORDINATE] - mean[PHASE_COORDINATE]) ** 2 for point in coordinates)
    return mean, variance


def distribution_arrays(distribution: dict[str, Any], context: str) -> tuple[list[float], list[float], list[float]]:
    arrays: list[list[float]] = []
    for field in ("mean", "variance", "generation_zero_variance"):
        raw = value(distribution, field, context)
        require(isinstance(raw, list) and len(raw) == CEM_DIMENSIONS, f"{context}: {field} is not a 17-vector")
        arrays.append([finite_number(item, f"{context}: {field}") for item in raw])
    require(all(item >= 0.0 for item in arrays[1] + arrays[2]), f"{context}: distribution variance is negative")
    require(0.0 <= arrays[0][PHASE_COORDINATE] < math.tau, f"{context}: distribution phase mean is not canonical")
    return arrays[0], arrays[1], arrays[2]


def expected_distribution(previous: tuple[list[float], list[float], list[float]], elites: list[dict[str, Any]]) -> tuple[list[float], list[float], list[float]]:
    previous_mean, previous_variance, generation_zero_variance = previous
    elite_mean, elite_variance = coordinate_moments(elites, previous_mean[PHASE_COORDINATE])
    mean = [0.0] * CEM_DIMENSIONS
    variance = [0.0] * CEM_DIMENSIONS
    for index in range(PHASE_COORDINATE):
        mean[index] = CEM_SMOOTHING * previous_mean[index] + (1.0 - CEM_SMOOTHING) * elite_mean[index]
        variance[index] = max(CEM_SMOOTHING * previous_variance[index] + (1.0 - CEM_SMOOTHING) * elite_variance[index], CEM_VARIANCE_FLOOR_FRACTION * generation_zero_variance[index])
    cosine = CEM_SMOOTHING * math.cos(previous_mean[PHASE_COORDINATE]) + (1.0 - CEM_SMOOTHING) * math.cos(elite_mean[PHASE_COORDINATE])
    sine = CEM_SMOOTHING * math.sin(previous_mean[PHASE_COORDINATE]) + (1.0 - CEM_SMOOTHING) * math.sin(elite_mean[PHASE_COORDINATE])
    mean[PHASE_COORDINATE] = wrap_phase(previous_mean[PHASE_COORDINATE]) if resultant_is_zero(cosine, sine, 2) else wrap_phase(math.atan2(sine, cosine))
    variance[PHASE_COORDINATE] = max(CEM_SMOOTHING * previous_variance[PHASE_COORDINATE] + (1.0 - CEM_SMOOTHING) * elite_variance[PHASE_COORDINATE], CEM_VARIANCE_FLOOR_FRACTION * generation_zero_variance[PHASE_COORDINATE])
    return mean, variance, generation_zero_variance


def require_distribution(actual: tuple[list[float], list[float], list[float]], expected: tuple[list[float], list[float], list[float]], context: str) -> None:
    for label, observed, calculated in zip(("mean", "variance", "generation_zero_variance"), actual, expected):
        for index, (actual_value, expected_value) in enumerate(zip(observed, calculated)):
            close(actual_value, expected_value, f"{context}: distribution {label}[{index}]")


def exact_config(path: Path) -> dict[str, Any]:
    try:
        config = json.loads(path.read_text(encoding="utf-8"))
    except OSError as error:
        raise ValueError(f"cannot read resolved configuration {path}: {error}") from error
    except json.JSONDecodeError as error:
        raise ValueError(f"invalid resolved configuration {path}: {error}") from error
    require(config == FROZEN_CONFIG, "resolved-config identity/constants differ from the frozen S0 contract")
    return config


def candidate_id_for(identity: dict[str, Any]) -> str:
    """Mirror Rust's serde_json compact field order and SHA-256 truncation."""
    encoded = json.dumps(identity, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return f"s0v1-{hashlib.sha256(encoded).hexdigest()[:24]}"


def expected_candidate_id(row: dict[str, Any], context: str) -> str:
    replicate = value(row, "replicate", context)
    identity = {
        "packet_version": FROZEN_CONFIG["packet_version"],
        "master_seed": FROZEN_CONFIG["master_seeds"][replicate],
        "replicate": replicate,
        "arm": value(row, "arm", context),
        "generation": value(row, "generation", context),
        "trajectory": value(row, "trajectory", context),
        "iteration": value(row, "iteration", context),
        "proposal_index": value(row, "proposal_index", context),
        "construction_attempt": value(row, "construction_attempt", context),
    }
    return candidate_id_for(identity)


def expected_elite_set_id(candidate_ids: list[str]) -> str:
    material = "\n".join(sorted(candidate_ids)).encode("utf-8")
    return f"s0v1-elite-{hashlib.sha256(material).hexdigest()[:24]}"


def rational_from_string(raw: Any, context: str) -> Fraction:
    require(isinstance(raw, str) and CANONICAL_RATIONAL.fullmatch(raw), f"{context}: rational is not canonical")
    try:
        rational = Fraction(raw)
    except (ValueError, ZeroDivisionError):
        fail(f"{context}: rational is not parseable")
    require(str(rational) == raw, f"{context}: rational is not reduced canonical syntax")
    return rational


def rational_vertices_from_key(key: Any, context: str) -> list[list[str]]:
    require(isinstance(key, str), f"{context}: polytope_key is not a string")
    vertices = [vertex.split(",") for vertex in key.split("|")]
    require(len(vertices) == 10 and all(len(vertex) == 4 for vertex in vertices), f"{context}: polytope_key is not 10x4 rational rows")
    for vertex in vertices:
        for rational in vertex:
            rational_from_string(rational, context)
    return vertices


def poly_id_from_rational_vertices(vertices: list[list[str]]) -> str:
    hasher = hashlib.sha256()
    for vertex in vertices:
        for raw in vertex:
            coordinate = float(rational_from_string(raw, "poly_id rational"))
            require(math.isfinite(coordinate), "poly_id rational does not reconstruct a finite f64")
            if coordinate == 0.0:
                coordinate = 0.0
            hasher.update(struct.pack("<d", coordinate))
    return hasher.hexdigest()


def poly_id_from_key(key: Any, context: str = "polytope_key") -> str:
    return poly_id_from_rational_vertices(rational_vertices_from_key(key, context))


def identical_config_artifact(artifacts: Path, config_path: Path) -> dict[str, Any]:
    """Require the run-local config to be the exact selected config, not a copy
    which merely happens to parse to a similar object.
    """
    config = exact_config(config_path)
    artifact_config = artifacts / "resolved-config.json"
    try:
        require(artifact_config.read_bytes() == config_path.read_bytes(), "artifacts/resolved-config.json is not byte-identical to --config")
    except OSError as error:
        raise ValueError(f"cannot compare resolved configuration artifacts: {error}") from error
    exact_config(artifact_config)
    return config


def distinct_successes(rows: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    best: dict[str, dict[str, Any]] = {}
    for row in rows:
        if row["evaluation_status"] != "success":
            continue
        key = row["polytope_key"]
        previous = best.get(key)
        if previous is None or row["sys"] > previous["sys"]:
            best[key] = row
    return best


def cyclic_rotation(word: list[Any]) -> tuple[Any, ...]:
    require(word, "cache orbit has an empty sigma")
    return min(tuple(word[start:] + word[:start]) for start in range(len(word)))


def validate_compact_payload(target: dict[str, Any], cache: dict[str, Any], context: str) -> None:
    result = value(cache, "capacity_result", context)
    require(isinstance(result, dict), f"{context}: capacity_result is not an object")
    orbits = value(result, "orbits", context)
    require(isinstance(orbits, list), f"{context}: capacity_result.orbits is not an array")
    compact = {
        "capacity": value(result, "min_action", context),
        "volume": value(cache, "volume", context),
        "sys": value(cache, "sys", context),
        "capacity_iterations": value(result, "iterations", context),
        "raw_returned_word_count": len(orbits),
        "raw_admissible_word_count": 0,
        "distinct_cyclic_class_count": 0,
        "support_lengths": [],
    }
    words: list[list[Any]] = []
    for orbit in orbits:
        require(isinstance(orbit, dict), f"{context}: cache orbit is not an object")
        sigma = value(orbit, "sigma", context)
        require(isinstance(sigma, list), f"{context}: cache orbit sigma is not an array")
        words.append(sigma)
        if value(orbit, "admissibility", context) in {"AdmissibleF64", "AdmissibleExact"}:
            compact["raw_admissible_word_count"] += 1
    compact["distinct_cyclic_class_count"] = len({cyclic_rotation(word) for word in words})
    compact["support_lengths"] = sorted({len(word) for word in words})
    for field, expected in compact.items():
        actual = value(target, field, context)
        require(actual == expected, f"{context}: target {field} disagrees with cache payload")
    for field in ("capacity", "volume", "sys"):
        finite_number(target[field], f"{context}: {field}")


def validate_target_rows(rows: list[dict[str, Any]]) -> dict[tuple[str, int], list[dict[str, Any]]]:
    grouped: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    identifiers: set[str] = set()
    for index, row in enumerate(rows):
        context = f"target row {index}"
        arm = value(row, "arm", context)
        replicate = value(row, "replicate", context)
        require(arm in ARMS and replicate in REPLICATES, f"{context}: invalid arm/replicate")
        candidate_id = value(row, "candidate_id", context)
        require(isinstance(candidate_id, str) and CANDIDATE_ID.fullmatch(candidate_id), f"{context}: invalid candidate_id")
        require(candidate_id == expected_candidate_id(row, context), f"{context}: candidate_id does not match frozen identity")
        require(candidate_id not in identifiers, f"duplicate target candidate_id {candidate_id}")
        identifiers.add(candidate_id)
        require(isinstance(value(row, "construction_attempt", context), int) and row["construction_attempt"] >= 0, f"{context}: invalid construction_attempt")
        require(isinstance(value(row, "construction_sequence_index", context), int) and row["construction_sequence_index"] >= 0, f"{context}: invalid construction_sequence_index")
        require(isinstance(value(row, "construction_rejections_before", context), int) and row["construction_rejections_before"] >= 0, f"{context}: invalid construction_rejections_before")
        finite_number(value(row, "wall_time_ms", context), context)
        status = value(row, "evaluation_status", context)
        cache_status = value(row, "cache_status", context)
        require(status in {"success", "failure"}, f"{context}: invalid evaluation_status")
        if status == "success":
            require(cache_status in {"miss", "hit"}, f"{context}: successful target has invalid cache status")
            require(isinstance(value(row, "polytope_key", context), str), f"{context}: successful target lacks exact key")
            require(isinstance(value(row, "poly_id", context), str) and row["poly_id"], f"{context}: successful target lacks poly_id")
            for field in ("capacity", "volume", "sys", "capacity_iterations", "raw_returned_word_count", "raw_admissible_word_count", "distinct_cyclic_class_count", "support_lengths"):
                require(value(row, field, context) is not None, f"{context}: successful target has unavailable {field}")
        else:
            require(cache_status == "failed_miss", f"{context}: failure must be a failed_miss")
            require(isinstance(value(row, "polytope_key", context), str), f"{context}: failed target lacks exact key")
            require(isinstance(value(row, "poly_id", context), str) and row["poly_id"], f"{context}: failed target lacks poly_id")
            for field in ("capacity", "volume", "sys", "capacity_iterations", "raw_returned_word_count", "raw_admissible_word_count", "distinct_cyclic_class_count"):
                require(value(row, field, context) is None, f"{context}: failed target must explicitly mark {field} unavailable")
            require(value(row, "support_lengths", context) == [], f"{context}: failed target must have no support lengths")
        grouped[(arm, replicate)].append(row)
    expected = {(arm, rep) for arm in ARMS for rep in REPLICATES}
    require(set(grouped) == expected, f"arm/replicate groups mismatch: {sorted(set(grouped) ^ expected)}")
    for (arm, replicate), group in grouped.items():
        attempts = [value(row, "attempt_index", f"{arm}/{replicate}") for row in group]
        require(len(group) == TARGET_ATTEMPTS and set(attempts) == set(range(1, TARGET_ATTEMPTS + 1)), f"{arm}/{replicate}: target attempt indices are not exactly 1..256")
        group.sort(key=lambda row: row["attempt_index"])
        for row in group:
            context = f"{arm}/{replicate} attempt {row['attempt_index']}"
            role = value(row, "role", context)
            generation, trajectory, iteration = row.get("generation"), row.get("trajectory"), row.get("iteration")
            if arm == "iid":
                require(role == "iid" and generation is None and trajectory is None and iteration is None and row.get("parent_candidate_id") is None and row.get("elite_set_id") is None, f"{context}: IID identity/parent fields are not frozen")
                require(0 <= row["proposal_index"] <= 255 and row["proposal_index"] == row["attempt_index"] - 1, f"{context}: IID proposal index disagrees with attempt")
            elif arm == "diagonal_cem":
                require(role == "cem_population" and generation in CEM_GENERATIONS and trajectory is None and iteration is None and row.get("parent_candidate_id") is None, f"{context}: CEM identity/parent fields are not frozen")
                require(0 <= row["proposal_index"] < CEM_POPULATION, f"{context}: CEM proposal index is out of range")
            else:
                require(generation is None and isinstance(trajectory, int) and trajectory >= 0 and row.get("elite_set_id") is None, f"{context}: local identity fields are not frozen")
                if role == "local_start":
                    require(iteration is None and row.get("parent_candidate_id") is None and row["proposal_index"] == trajectory, f"{context}: local start parent/index fields are invalid")
                elif role in {"within_step", "overshoot"}:
                    expected_range = range(0, 5) if role == "within_step" else range(5, 8)
                    require(isinstance(iteration, int) and iteration >= 0 and isinstance(row.get("parent_candidate_id"), str) and row["construction_attempt"] == 0 and row["proposal_index"] in expected_range, f"{context}: local step parent/identity fields are invalid")
                else:
                    fail(f"{context}: invalid local role")
    poly_ids: dict[str, str] = {}
    for row in rows:
        previous = poly_ids.setdefault(row["polytope_key"], row["poly_id"])
        require(previous == row["poly_id"], f"exact key {row['polytope_key']}: inconsistent poly_id")
    for replicate in REPLICATES:
        local_rows = grouped[("multistart_branch_local_phase0", replicate)]
        by_id = {row["candidate_id"]: row for row in local_rows}
        successful_starts = {row["trajectory"]: row for row in local_rows if row["role"] == "local_start" and row["evaluation_status"] == "success"}
        for row in local_rows:
            if row["role"] == "local_start":
                continue
            context = f"local {replicate} step {row['candidate_id']}"
            start = successful_starts.get(row["trajectory"])
            parent = by_id.get(row["parent_candidate_id"])
            require(start is not None and start["attempt_index"] < row["attempt_index"], f"{context}: no preceding successful same-trajectory start")
            require(parent is not None and parent["trajectory"] == row["trajectory"] and parent["evaluation_status"] == "success" and parent["attempt_index"] < row["attempt_index"], f"{context}: parent is not reachable within same trajectory")
            ancestor = parent
            while ancestor["role"] != "local_start":
                ancestor = by_id.get(ancestor["parent_candidate_id"])
                require(ancestor is not None and ancestor["trajectory"] == row["trajectory"], f"{context}: orphaned local parent chain")
    return grouped


def validate_cache(rows: list[dict[str, Any]], grouped: dict[tuple[str, int], list[dict[str, Any]]]) -> dict[tuple[str, int, str], dict[str, Any]]:
    cache: dict[tuple[str, int, str], dict[str, Any]] = {}
    for index, row in enumerate(rows):
        context = f"cache row {index}"
        arm, replicate, key = (value(row, "arm", context), value(row, "replicate", context), value(row, "polytope_key", context))
        require(arm in ARMS and replicate in REPLICATES and isinstance(key, str), f"{context}: invalid private cache ownership")
        dual = value(row, "dual_vertices_rational", context)
        require(isinstance(dual, list) and len(dual) == 10, f"{context}: dual_vertices_rational is not 10x4")
        for vertex in dual:
            require(isinstance(vertex, list) and len(vertex) == 4, f"{context}: dual vertex is not length four")
            for rational in vertex:
                require(isinstance(rational, str) and rational.strip() == rational and rational, f"{context}: dual coordinate is not a rational string")
                try:
                    Fraction(rational)
                except (ValueError, ZeroDivisionError):
                    fail(f"{context}: dual coordinate is not parseable as a rational")
        require(key == "|".join(",".join(vertex) for vertex in dual), f"{context}: polytope_key does not exactly encode dual_vertices_rational")
        require(isinstance(value(row, "poly_id", context), str) and row["poly_id"], f"{context}: missing poly_id")
        identity = (arm, replicate, key)
        require(identity not in cache, f"duplicate arm-private cache row {identity}")
        cache[identity] = row
    for (arm, replicate), group in grouped.items():
        known: dict[str, dict[str, Any]] = {}
        for row in group:
            context = f"{arm}/{replicate} attempt {row['attempt_index']}"
            key = row.get("polytope_key")
            if row["evaluation_status"] == "failure":
                require((arm, replicate, key) not in cache, f"{context}: failure illegally owns a cache payload")
                continue
            require(isinstance(key, str), f"{context}: success without key")
            cache_row = cache.get((arm, replicate, key))
            require(cache_row is not None, f"{context}: success refers to absent arm-private cache payload")
            if row["cache_status"] == "miss":
                require(key not in known, f"{context}: miss repeats an owned cache key")
                known[key] = cache_row
            else:
                require(key in known, f"{context}: cache hit precedes its same-arm/private miss")
            validate_compact_payload(row, cache_row, context)
        expected_keys = set(known)
        actual_keys = {key for owner_arm, owner_rep, key in cache if (owner_arm, owner_rep) == (arm, replicate)}
        require(actual_keys == expected_keys, f"{arm}/{replicate}: cache rows do not reconcile with successful target misses")
    cache_poly_ids: dict[str, str] = {}
    for cache_row in cache.values():
        key, poly_id = cache_row["polytope_key"], cache_row["poly_id"]
        previous = cache_poly_ids.setdefault(key, poly_id)
        require(previous == poly_id, f"exact key {key}: cache poly_id is inconsistent")
    for group in grouped.values():
        for target in group:
            if target["polytope_key"] in cache_poly_ids:
                require(target["poly_id"] == cache_poly_ids[target["polytope_key"]], f"target {target['candidate_id']}: poly_id disagrees with cache")
    return cache


def validate_cem(rows: list[dict[str, Any]], cem_rows: list[dict[str, Any]], grouped: dict[tuple[str, int], list[dict[str, Any]]]) -> dict[tuple[int, int], dict[str, Any]]:
    records: dict[tuple[int, int], dict[str, Any]] = {}
    for index, record in enumerate(cem_rows):
        context = f"CEM generation row {index}"
        replicate, generation = value(record, "replicate", context), value(record, "generation", context)
        require(replicate in REPLICATES and generation in CEM_GENERATIONS, f"{context}: invalid replicate/generation")
        identity = (replicate, generation)
        require(identity not in records, f"duplicate CEM generation row {identity}")
        require(value(record, "complete", context) is True, f"{context}: incomplete CEM generation")
        records[identity] = record
    expected = {(rep, generation) for rep in REPLICATES for generation in CEM_GENERATIONS}
    require(set(records) == expected, f"CEM generations must be exactly 0..3 per replicate")
    for replicate in REPLICATES:
        previous_elite_set_id = None
        previous_distribution: tuple[list[float], list[float], list[float]] | None = None
        previous_elites: list[dict[str, Any]] | None = None
        for generation in CEM_GENERATIONS:
            record = records[(replicate, generation)]
            context = f"CEM {replicate}/{generation}"
            population = [row for row in grouped[("diagonal_cem", replicate)] if row.get("generation") == generation]
            require(len(population) == CEM_POPULATION, f"{context}: population is not exactly {CEM_POPULATION}")
            require(all(row.get("role") == "cem_population" and row["evaluation_status"] == "success" for row in population), f"{context}: population has non-success/non-CEM target")
            population.sort(key=lambda row: row["proposal_index"])
            require([row["proposal_index"] for row in population] == list(range(CEM_POPULATION)), f"{context}: population proposal indices are not exactly 0..63")
            member_ids = value(record, "member_candidate_ids", context)
            require(member_ids == [row["candidate_id"] for row in population] and len(set(member_ids)) == CEM_POPULATION, f"{context}: member IDs do not exactly identify the population")
            elite_ids = value(record, "elite_candidate_ids", context)
            ranked = sorted(population, key=lambda row: (-row["sys"], row["candidate_id"]))[:CEM_ELITES]
            require(elite_ids == [row["candidate_id"] for row in ranked] and len(set(elite_ids)) == CEM_ELITES, f"{context}: elite IDs do not match the ranked population")
            require(isinstance(value(record, "elite_set_id", context), str) and record["elite_set_id"], f"{context}: missing elite_set_id")
            require(record["elite_set_id"] == expected_elite_set_id(elite_ids), f"{context}: elite_set_id does not match sorted elite IDs")
            distribution = value(record, "distribution", context)
            require(isinstance(distribution, dict), f"{context}: missing distribution")
            actual_distribution = distribution_arrays(distribution, context)
            if generation == 0:
                fallback = chart_coordinates(min(population, key=lambda row: row["candidate_id"]), context)[PHASE_COORDINATE]
                moments = coordinate_moments(population, fallback)
                expected_distribution_value = (moments[0], moments[1], moments[1])
            else:
                require(previous_distribution is not None and previous_elites is not None, f"{context}: missing predecessor distribution")
                expected_distribution_value = expected_distribution(previous_distribution, previous_elites)
            require_distribution(actual_distribution, expected_distribution_value, context)
            rejections = sum(row["construction_rejections_before"] for row in population)
            require(value(record, "construction_rejections", context) == rejections, f"{context}: construction rejection count disagrees with targets")
            require(value(record, "construction_attempts", context) == CEM_POPULATION + rejections and record["construction_attempts"] <= CEM_CONSTRUCTION_ATTEMPT_CAP, f"{context}: construction attempts violate the frozen cap")
            parent = value(record, "parent_elite_set_id", context)
            require(parent == previous_elite_set_id, f"{context}: parent elite-set genealogy is broken")
            for row in population:
                require(row.get("parent_candidate_id") is None and row.get("elite_set_id") == parent, f"{context}: distribution parent does not agree with target row")
            previous_elite_set_id = record["elite_set_id"]
            previous_distribution = actual_distribution
            previous_elites = ranked
    return records


def validate_lineages(rows: list[dict[str, Any]], lineage_rows: list[dict[str, Any]], cem_records: dict[tuple[int, int], dict[str, Any]]) -> None:
    targets = {row["candidate_id"]: row for row in rows}
    lineage = {value(row, "candidate_id", "lineage row"): row for row in lineage_rows}
    require(len(lineage) == len(lineage_rows), "duplicate lineage candidate_id")
    require(set(lineage) == set(targets), "lineage candidate IDs do not reconcile with target rows")
    for candidate_id, target in targets.items():
        record = lineage[candidate_id]
        context = f"lineage {candidate_id}"
        kind = value(record, "parent_kind", context)
        parent_candidate_id = value(record, "parent_candidate_id", context)
        elite_set_id = value(record, "elite_set_id", context)
        require(parent_candidate_id == target.get("parent_candidate_id") and elite_set_id == target.get("elite_set_id"), f"{context}: lineage parent fields disagree with target row")
        if kind == "none":
            require(parent_candidate_id is None and elite_set_id is None, f"{context}: none parent has an ID")
        elif kind == "candidate":
            parent = targets.get(parent_candidate_id)
            require(elite_set_id is None and parent is not None, f"{context}: candidate parent is missing or mixed with elite parent")
            require((parent["arm"], parent["replicate"]) == (target["arm"], target["replicate"]) and parent["attempt_index"] < target["attempt_index"], f"{context}: candidate parent is not an earlier same-arm target")
        elif kind == "distribution":
            require(target["arm"] == "diagonal_cem" and parent_candidate_id is None, f"{context}: distribution parent is not a CEM target")
            generation = target.get("generation")
            expected = cem_records[(target["replicate"], generation)].get("parent_elite_set_id")
            require(elite_set_id == expected and elite_set_id is not None, f"{context}: distribution parent is not the preceding elite set")
        else:
            fail(f"{context}: invalid parent_kind")
        if target["arm"] == "diagonal_cem":
            generation = target.get("generation")
            require(kind == ("none" if generation == 0 else "distribution"), f"{context}: CEM parent kind does not match generation")


def validate_rejections_and_arm_runs(rows: list[dict[str, Any]], rejection_rows: list[dict[str, Any]], arm_runs: list[dict[str, Any]], grouped: dict[tuple[str, int], list[dict[str, Any]]], trajectory_records: dict[tuple[int, int], dict[str, Any]]) -> None:
    rejection_counts: Counter[tuple[str, int]] = Counter()
    rejection_ids: set[str] = set()
    target_ids = {row["candidate_id"] for row in rows}
    rejections_by_group: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for index, row in enumerate(rejection_rows):
        context = f"construction rejection row {index}"
        arm, replicate = value(row, "arm", context), value(row, "replicate", context)
        require(arm in ARMS and replicate in REPLICATES, f"{context}: invalid arm/replicate")
        candidate_id = value(row, "candidate_id", context)
        require(isinstance(candidate_id, str) and CANDIDATE_ID.fullmatch(candidate_id), f"{context}: invalid candidate_id")
        require(candidate_id == expected_candidate_id(row, context), f"{context}: candidate_id does not match frozen identity")
        require(candidate_id not in rejection_ids, f"duplicate construction rejection candidate_id {candidate_id}")
        require(candidate_id not in target_ids, f"{context}: rejected construction reuses a charged target candidate_id")
        rejection_ids.add(candidate_id)
        for field in ("generation", "trajectory", "iteration", "proposal_index", "role", "construction_sequence_index"):
            value(row, field, context)
        require(isinstance(row["proposal_index"], int) and row["proposal_index"] >= 0, f"{context}: invalid proposal_index")
        require(isinstance(value(row, "construction_attempt", context), int) and row["construction_attempt"] >= 0, f"{context}: invalid construction_attempt")
        require(isinstance(row["construction_sequence_index"], int) and row["construction_sequence_index"] >= 0, f"{context}: invalid construction_sequence_index")
        require(isinstance(value(row, "reason", context), str) and row["reason"], f"{context}: missing rejection reason")
        if arm == "iid":
            require(row["role"] == "iid" and row["generation"] is None and row["trajectory"] is None and row["iteration"] is None and 0 <= row["proposal_index"] <= 255, f"{context}: IID rejection identity is invalid")
        elif arm == "diagonal_cem":
            require(row["role"] == "cem_population" and row["generation"] in CEM_GENERATIONS and row["trajectory"] is None and row["iteration"] is None and 0 <= row["proposal_index"] < CEM_POPULATION, f"{context}: CEM rejection identity is invalid")
        else:
            require(row["generation"] is None and isinstance(row["trajectory"], int) and row["trajectory"] >= 0, f"{context}: local rejection identity is invalid")
            if row["role"] == "local_start":
                require(row["iteration"] is None and row["proposal_index"] == row["trajectory"], f"{context}: local-start rejection identity is invalid")
            else:
                expected_range = range(0, 5) if row["role"] == "within_step" else range(5, 8)
                require(row["role"] in {"within_step", "overshoot"} and isinstance(row["iteration"], int) and row["iteration"] >= 0 and row["construction_attempt"] == 0 and row["proposal_index"] in expected_range, f"{context}: local-step rejection identity is invalid")
        rejection_counts[(arm, replicate)] += 1
        rejections_by_group[(arm, replicate)].append(row)

    for replicate in REPLICATES:
        local_targets = grouped[("multistart_branch_local_phase0", replicate)]
        starts = {row["trajectory"]: row for row in local_targets if row["role"] == "local_start" and row["evaluation_status"] == "success"}
        for rejection in rejections_by_group[("multistart_branch_local_phase0", replicate)]:
            if rejection["role"] == "local_start":
                continue
            trajectory = rejection["trajectory"]
            context = f"local rejection {rejection['candidate_id']}"
            record = trajectory_records.get((replicate, trajectory))
            start = starts.get(trajectory)
            require(record is not None and start is not None, f"{context}: no successful trajectory start")
            require(rejection["iteration"] <= record["accepted_iterations"], f"{context}: iteration exceeds accepted trajectory iterations")
            trajectory_events = [(row["construction_sequence_index"], "target", row) for row in local_targets if row["trajectory"] == trajectory] + [(row["construction_sequence_index"], "rejection", row) for row in rejections_by_group[("multistart_branch_local_phase0", replicate)] if row["trajectory"] == trajectory]
            ordered = sorted(trajectory_events)
            position = next(index for index, event in enumerate(ordered) if event[1] == "rejection" and event[2] is rejection)
            require(start["construction_sequence_index"] < rejection["construction_sequence_index"], f"{context}: rejection precedes its successful start")
            step_key = (rejection["iteration"], rejection["proposal_index"])
            next_targets = [event[2] for event in ordered[position + 1:] if event[1] == "target"]
            if next_targets:
                next_target = next_targets[0]
                if next_target["role"] != "local_start":
                    require(step_key < (next_target["iteration"], next_target["proposal_index"]), f"{context}: sequence is not before the next lexicographic grid target")
            else:
                require(record["complete"] is False, f"{context}: complete trajectory has an unfinished terminal grid")

    run_by_group: dict[tuple[str, int], dict[str, Any]] = {}
    for index, run in enumerate(arm_runs):
        context = f"arm-run row {index}"
        identity = (value(run, "arm", context), value(run, "replicate", context))
        require(identity in grouped and identity not in run_by_group, f"{context}: invalid or duplicate arm-run ownership")
        run_by_group[identity] = run
    require(set(run_by_group) == set(grouped), "arm-runs do not cover every arm/replicate")
    for identity, targets in grouped.items():
        arm, replicate = identity
        run = run_by_group[identity]
        context = f"arm-run {arm}/{replicate}"
        target_rejections = sum(row["construction_rejections_before"] for row in targets)
        require(rejection_counts[identity] == target_rejections, f"{context}: detailed rejection rows disagree with target rejection accounting")
        events = [(row["construction_sequence_index"], "target", row) for row in targets] + [(row["construction_sequence_index"], "rejection", row) for row in rejections_by_group[identity]]
        sequence_indices = [event[0] for event in events]
        require(len(sequence_indices) == len(set(sequence_indices)) and set(sequence_indices) == set(range(len(events))), f"{context}: construction sequence is not exactly 0..N-1")
        rejections_since_target = 0
        for _, kind, event in sorted(events):
            if kind == "rejection":
                rejections_since_target += 1
            elif arm != "diagonal_cem":
                require(event["construction_rejections_before"] == rejections_since_target, f"{context}: target construction_rejections_before does not equal prior rejection events")
                rejections_since_target = 0
        if arm == "iid":
            by_proposal: dict[int, list[dict[str, Any]]] = defaultdict(list)
            for rejection in rejections_by_group[identity]:
                by_proposal[rejection["proposal_index"]].append(rejection)
            for target in targets:
                prior = sorted(by_proposal[target["proposal_index"]], key=lambda row: row["construction_attempt"])
                require([row["construction_attempt"] for row in prior] == list(range(target["construction_attempt"])), f"{context}: IID rejection attempts do not consecutively precede target proposal")
                require(len(prior) == target["construction_rejections_before"], f"{context}: IID rejection proposal does not equal following target proposal")
        elif arm == "diagonal_cem":
            for generation in CEM_GENERATIONS:
                generation_targets = [row for row in targets if row["generation"] == generation]
                generation_rejections = [row for row in rejections_by_group[identity] if row["generation"] == generation]
                events_by_attempt = sorted([(row["construction_attempt"], "target", row) for row in generation_targets] + [(row["construction_attempt"], "rejection", row) for row in generation_rejections])
                construction_attempts = [event[0] for event in events_by_attempt]
                require(len(construction_attempts) == len(set(construction_attempts)) and construction_attempts == list(range(len(events_by_attempt))), f"{context} generation {generation}: CEM construction attempts are not exactly ordered")
                accepted_count = 0
                consecutive_rejections = 0
                for _, kind, event in events_by_attempt:
                    require(accepted_count < CEM_POPULATION, f"{context} generation {generation}: construction occurred after completed population")
                    require(event["proposal_index"] == accepted_count, f"{context} generation {generation}: proposal index does not equal accepted-count-so-far")
                    if kind == "rejection":
                        consecutive_rejections += 1
                    else:
                        require(event["construction_rejections_before"] == consecutive_rejections, f"{context} generation {generation}: target rejection count is not same-generation consecutive rejections")
                        consecutive_rejections = 0
                        accepted_count += 1
                require(accepted_count == CEM_POPULATION, f"{context} generation {generation}: CEM population is incomplete")
        expected = {
            "target_attempts": TARGET_ATTEMPTS,
            "successful_new_computations": sum(row["cache_status"] == "miss" for row in targets),
            "cache_hits": sum(row["cache_status"] == "hit" for row in targets),
            "failed_new_computations": sum(row["cache_status"] == "failed_miss" for row in targets),
            "construction_rejections": target_rejections,
            "construction_attempts": TARGET_ATTEMPTS + target_rejections,
        }
        for field, expected_value in expected.items():
            require(value(run, field, context) == expected_value, f"{context}: {field} disagrees with target/rejection accounting")
        require(value(run, "status", context) == "complete", f"{context}: fixed packet arm run is not complete")
        target_wall = sum(row["wall_time_ms"] for row in targets)
        require(value(run, "target_wall_time_ms", context) == target_wall, f"{context}: target wall time disagrees with target rows")
        require(finite_number(value(run, "total_wall_time_ms", context), context) >= target_wall, f"{context}: total wall time is below charged target wall time")


def validate_run_status(run_status: dict[str, Any], arm_runs: list[dict[str, Any]], row_count: int) -> dict[str, float]:
    require(value(run_status, "packet_version", "run-status") == FROZEN_CONFIG["packet_version"], "run-status: packet version differs from resolved config")
    require(value(run_status, "complete", "run-status") is True, "run-status: fixed packet is incomplete")
    require(value(run_status, "charged_target_attempts", "run-status") == row_count == len(ARMS) * len(REPLICATES) * TARGET_ATTEMPTS, "run-status: charged target count disagrees with artifacts")
    overall = finite_number(value(run_status, "overall_wall_time_ms", "run-status"), "run-status overall wall time")
    totals = {arm: sum(finite_number(run["total_wall_time_ms"], f"arm-run {arm}") for run in arm_runs if run["arm"] == arm) for arm in ARMS}
    arm_total = sum(totals.values())
    require(overall + max(1e-9, 1e-12 * arm_total) >= arm_total, "run-status: overall wall time is less than sequential arm totals")
    return {**{f"{arm}_total_wall_time_ms": total for arm, total in totals.items()}, "overall_wall_time_ms": overall}


def validate_local_trajectories(grouped: dict[tuple[str, int], list[dict[str, Any]]], local_trajectories: list[dict[str, Any]]) -> dict[tuple[int, int], dict[str, Any]]:
    records: dict[tuple[int, int], dict[str, Any]] = {}
    allowed_stops = {"no_direction", "invalid_step_bound", "no_improvement", "improvement_below_threshold", "incomplete_grid"}
    for index, record in enumerate(local_trajectories):
        context = f"local trajectory row {index}"
        require(value(record, "arm", context) == "multistart_branch_local_phase0", f"{context}: invalid arm")
        replicate, trajectory = value(record, "replicate", context), value(record, "trajectory", context)
        require(replicate in REPLICATES and isinstance(trajectory, int) and trajectory >= 0, f"{context}: invalid replicate/trajectory")
        identity = (replicate, trajectory)
        require(identity not in records, f"duplicate local trajectory {identity}")
        require(value(record, "stop", context) in allowed_stops, f"{context}: invalid stop")
        require(value(record, "complete", context) == (record["stop"] != "incomplete_grid"), f"{context}: completeness disagrees with stop")
        records[identity] = record
    for replicate in REPLICATES:
        local_rows = grouped[("multistart_branch_local_phase0", replicate)]
        starts = {row["trajectory"]: row for row in local_rows if row["role"] == "local_start" and row["evaluation_status"] == "success"}
        require(set(records_key for records_key in records if records_key[0] == replicate) == {(replicate, trajectory) for trajectory in starts}, f"local {replicate}: trajectory records do not exactly cover successful starts")
        by_id = {row["candidate_id"]: row for row in local_rows}
        for trajectory, start in starts.items():
            record = records[(replicate, trajectory)]
            context = f"local {replicate}/{trajectory}"
            require(value(record, "start_candidate_id", context) == start["candidate_id"] and value(record, "start_sys", context) == start["sys"], f"{context}: start record disagrees with target")
            accepted = sorted((row for row in local_rows if row["trajectory"] == trajectory and row["became_next_state"]), key=lambda row: row["attempt_index"])
            final = accepted[-1] if accepted else start
            require(value(record, "accepted_iterations", context) == len(accepted), f"{context}: accepted iteration count disagrees with targets")
            require(value(record, "final_candidate_id", context) == final["candidate_id"] and value(record, "final_sys", context) == final["sys"], f"{context}: final state disagrees with accepted local target")
    return records


def validate_local_policy(grouped: dict[tuple[str, int], list[dict[str, Any]]], rejection_rows: list[dict[str, Any]], trajectory_records: dict[tuple[int, int], dict[str, Any]]) -> None:
    """Reconstruct the frozen local line-search state machine from events."""
    for replicate in REPLICATES:
        targets = grouped[("multistart_branch_local_phase0", replicate)]
        rejections = [row for row in rejection_rows if row["arm"] == "multistart_branch_local_phase0" and row["replicate"] == replicate]
        starts = {row["trajectory"]: row for row in targets if row["role"] == "local_start"}
        for trajectory, start in starts.items():
            trajectory_targets = [row for row in targets if row["trajectory"] == trajectory]
            record = trajectory_records.get((replicate, trajectory))
            if start["evaluation_status"] != "success":
                require(record is None and len(trajectory_targets) == 1, f"local {replicate}/{trajectory}: failed start has later steps or trajectory record")
                continue
            require(record is not None, f"local {replicate}/{trajectory}: successful start has no trajectory record")
            events = [(row["construction_sequence_index"], "target", row) for row in trajectory_targets if row["role"] != "local_start"] + [(row["construction_sequence_index"], "rejection", row) for row in rejections if row["trajectory"] == trajectory and row["role"] != "local_start"]
            by_iteration: dict[int, list[tuple[int, str, dict[str, Any]]]] = defaultdict(list)
            for event in events:
                iteration = event[2]["iteration"]
                require(isinstance(iteration, int) and iteration >= 0, f"local {replicate}/{trajectory}: step lacks iteration")
                by_iteration[iteration].append(event)
            iteration_numbers = sorted(by_iteration)
            require(iteration_numbers == list(range(len(iteration_numbers))), f"local {replicate}/{trajectory}: step iterations are not contiguous")
            stop = record["stop"]
            accepted_goal = record["accepted_iterations"]
            if stop in {"no_direction", "invalid_step_bound"}:
                require(len(iteration_numbers) == accepted_goal, f"local {replicate}/{trajectory}: direction/step-bound stop has terminal grid")
            elif stop == "no_improvement":
                require(len(iteration_numbers) == accepted_goal + 1, f"local {replicate}/{trajectory}: no-improvement terminal grid missing")
            elif stop == "improvement_below_threshold":
                require(accepted_goal > 0 and len(iteration_numbers) == accepted_goal, f"local {replicate}/{trajectory}: improvement-threshold terminal grid inconsistent")
            else:
                require(len(iteration_numbers) == accepted_goal + 1, f"local {replicate}/{trajectory}: incomplete terminal grid missing")
            current = start
            accepted_count = 0
            for iteration in iteration_numbers:
                grid = sorted(by_iteration[iteration])
                indices = [event[2]["proposal_index"] for event in grid]
                full_indices = list(range(8)) if any(index >= 5 for index in indices) else list(range(5))
                terminal = iteration == iteration_numbers[-1]
                full = indices == full_indices
                require(indices == list(range(len(indices))), f"local {replicate}/{trajectory} iteration {iteration}: grid is not an ordered prefix")
                for _, kind, event in grid:
                    require((event["proposal_index"] < 5 and event["role"] == "within_step") or (event["proposal_index"] >= 5 and event["role"] == "overshoot"), f"local {replicate}/{trajectory} iteration {iteration}: role/index mismatch")
                    if kind == "target":
                        require(event["parent_candidate_id"] == current["candidate_id"], f"local {replicate}/{trajectory} iteration {iteration}: target parent is not current state")
                best = None
                for _, kind, event in grid:
                    if kind == "target" and event["evaluation_status"] == "success" and event["sys"] > current["sys"] and (best is None or event["sys"] > best["sys"]):
                        best = event
                accepted = [event for _, kind, event in grid if kind == "target" and event["became_next_state"]]
                if not terminal or stop in {"no_direction", "invalid_step_bound"} or (stop == "incomplete_grid" and len(iteration_numbers) == accepted_goal):
                    require(full and best is not None and accepted == [best], f"local {replicate}/{trajectory} iteration {iteration}: accepted complete grid invalid")
                    current, accepted_count = best, accepted_count + 1
                    continue
                if stop == "no_improvement":
                    require(full and best is None and not accepted, f"local {replicate}/{trajectory}: no-improvement terminal grid invalid")
                elif stop == "improvement_below_threshold":
                    require(full and best is not None and accepted == [best] and 0.0 < best["sys"] - current["sys"] < 1e-6, f"local {replicate}/{trajectory}: improvement-threshold terminal grid invalid")
                    current, accepted_count = best, accepted_count + 1
                elif stop == "incomplete_grid":
                    require(not full and not accepted, f"local {replicate}/{trajectory}: incomplete terminal grid is not strict prefix")
                else:
                    fail(f"local {replicate}/{trajectory}: unexpected terminal stop")
            require(accepted_count == accepted_goal and current["candidate_id"] == record["final_candidate_id"], f"local {replicate}/{trajectory}: reconstructed final state disagrees with record")


def metrics(rows: list[dict[str, Any]], expected_attempts: int) -> dict[str, Any]:
    require([row["attempt_index"] for row in rows] == list(range(1, expected_attempts + 1)), "metrics input is not an exact attempt prefix")
    unique = distinct_successes(rows)
    if len(unique) < 8:
        fail(f"fewer than eight distinct successful keys: {len(unique)}")
    top = sorted((row["sys"] for row in unique.values()), reverse=True)[:8]
    running: list[float] = []
    best = None
    for row in rows:
        if row["evaluation_status"] == "success":
            best = row["sys"] if best is None else max(best, row["sys"])
        if best is None:
            fail("normalized AUC undefined before first successful target row")
        running.append(best)
    return {
        "attempts": len(rows),
        "distinct_successful_keys": len(unique),
        "best_sys": max(top),
        "top_eight_median_sys": statistics.median(top),
        "counts_above": {str(level): sum(score >= level for score in (row["sys"] for row in unique.values())) for level in LEVELS},
        "normalized_best_so_far_auc": statistics.fmean(running),
        "successful_new_computations": sum(row["cache_status"] == "miss" for row in rows),
        "cache_hits": sum(row["cache_status"] == "hit" for row in rows),
        "failed_new_computations": sum(row["cache_status"] == "failed_miss" for row in rows),
        "construction_attempts": len(rows) + sum(row["construction_rejections_before"] for row in rows),
        "construction_rejections": sum(row["construction_rejections_before"] for row in rows),
        "target_wall_time_ms": sum(row["wall_time_ms"] for row in rows),
        "capacity_iterations": {
            "count": sum(row.get("capacity_iterations") is not None for row in rows),
            "sum": sum(row.get("capacity_iterations") or 0 for row in rows),
            "max": max((row.get("capacity_iterations") or 0 for row in rows), default=0),
        },
    }


def action_gap_signature(cache: dict[str, Any], context: str, active_only: bool = False) -> list[tuple[tuple[Any, ...], str, float, float, float]]:
    result = value(cache, "capacity_result", context)
    minimum = finite_number(value(result, "min_action", context), context)
    active_upper = finite_number(value(result, "min_action_upper", context), context)
    signature = []
    for orbit in value(result, "orbits", context):
        sigma = value(orbit, "sigma", context)
        if active_only and finite_number(value(orbit, "action_lower", context), context) > active_upper:
            continue
        signature.append((cyclic_rotation(sigma), value(orbit, "admissibility", context), finite_number(value(orbit, "action", context), context) - minimum, finite_number(value(orbit, "action_lower", context), context) - minimum, finite_number(value(orbit, "action_upper", context), context) - minimum))
    return sorted(signature)


def top_eight_signatures(unique: dict[str, dict[str, Any]], cache_index: dict[tuple[str, int, str], dict[str, Any]]) -> list[dict[str, Any]]:
    result = []
    for row in sorted(unique.values(), key=lambda item: (-item["sys"], item["polytope_key"]))[:8]:
        context = f"top-eight {row['candidate_id']}"
        cache = cache_index[(row["arm"], row["replicate"], row["polytope_key"])]
        result.append({
            "polytope_key": row["polytope_key"],
            "candidate_id": row["candidate_id"],
            "sys": row["sys"],
            "support_lengths": row["support_lengths"],
            "orbit_action_gap_signature": [
                {"canonical_sigma": list(word), "admissibility": admissibility, "action_gap": action_gap, "action_lower_gap": lower_gap, "action_upper_gap": upper_gap}
                for word, admissibility, action_gap, lower_gap, upper_gap in action_gap_signature(cache, context)
            ],
            "active_orbit_canonical_signatures": [list(word) for word, _, _, _, _ in action_gap_signature(cache, context, active_only=True)],
        })
    return result


def key_and_chart_diagnostics(rows: list[dict[str, Any]], cache_index: dict[tuple[str, int, str], dict[str, Any]]) -> dict[str, Any]:
    unique = distinct_successes(rows)
    selected = sorted(unique.values(), key=lambda row: row["polytope_key"])
    distances: list[tuple[float, str, str]] = []
    for left_index, left in enumerate(selected):
        left_chart = chart_coordinates(left, f"chart {left['candidate_id']}")
        for right in selected[left_index + 1:]:
            distances.append((chart_distance(left_chart, chart_coordinates(right, f"chart {right['candidate_id']}")), left["polytope_key"], right["polytope_key"]))
    require(distances, "pairwise canonical-chart distance requires at least two distinct successful keys")
    closest = min(distances)
    values = [item[0] for item in distances]
    return {
        "all_exact_polytope_keys": [row["polytope_key"] for row in selected],
        "top_eight": top_eight_signatures(unique, cache_index),
        "pairwise_canonical_chart_distance": {"pair_count": len(distances), "minimum": min(values), "median": statistics.median(values), "maximum": max(values), "closest_pair_exact_keys": [closest[1], closest[2]]},
    }


def cem_path_metrics(cem_records: dict[tuple[int, int], dict[str, Any]], targets: dict[str, dict[str, Any]], cache_index: dict[tuple[str, int, str], dict[str, Any]]) -> dict[str, Any]:
    by_replicate: dict[str, Any] = {}
    for replicate in REPLICATES:
        generations = []
        previous_elite_ids: set[str] | None = None
        previous_elite_keys: set[str] | None = None
        previous_variance: list[float] | None = None
        previous_gaps: list[float] | None = None
        previous_radii: list[float] | None = None
        previous_action_signature = None
        previous_active_signature = None
        previous_support_lengths = None
        for generation in CEM_GENERATIONS:
            record = cem_records[(replicate, generation)]
            _, variance, _ = distribution_arrays(record["distribution"], f"CEM {replicate}/{generation}")
            elite_ids = set(record["elite_candidate_ids"])
            elite_keys = {targets[candidate_id]["polytope_key"] for candidate_id in elite_ids}
            elite_rows = [targets[candidate_id] for candidate_id in record["elite_candidate_ids"]]
            gap_vectors, radius_vectors = zip(*(chart_gap_radius_features(row, f"CEM elite {row['candidate_id']}") for row in elite_rows))
            mean_gaps = [statistics.fmean(vector[index] for vector in gap_vectors) for index in range(10)]
            mean_radii = [statistics.fmean(vector[index] for vector in radius_vectors) for index in range(10)]
            representative = elite_rows[0]
            representative_cache = cache_index[(representative["arm"], representative["replicate"], representative["polytope_key"])]
            action_signature = action_gap_signature(representative_cache, f"CEM elite {representative['candidate_id']}")
            active_signature = action_gap_signature(representative_cache, f"CEM elite {representative['candidate_id']}", active_only=True)
            overlap = None if previous_elite_keys is None else len(elite_keys & previous_elite_keys)
            union = None if previous_elite_keys is None else len(elite_keys | previous_elite_keys)
            support_lengths = representative["support_lengths"]
            generations.append({"generation": generation, "interpretation_level": "G", "elite_candidate_id_overlap_with_previous_generation": None if previous_elite_ids is None else len(elite_ids & previous_elite_ids), "elite_exact_polytope_key_overlap_with_previous_generation": overlap, "elite_exact_polytope_key_jaccard_with_previous_generation": None if union is None else overlap / union, "elite_mean_cyclic_gaps": mean_gaps, "elite_mean_centered_log_radii": mean_radii, "elite_cyclic_gap_shift_from_previous": None if previous_gaps is None else math.sqrt(sum((current - previous) ** 2 for current, previous in zip(mean_gaps, previous_gaps))), "elite_centered_radius_shift_from_previous": None if previous_radii is None else math.sqrt(sum((current - previous) ** 2 for current, previous in zip(mean_radii, previous_radii))), "elite_representative_support_lengths": support_lengths, "elite_representative_support_recurred": None if previous_support_lengths is None else support_lengths == previous_support_lengths, "elite_representative_support_changed": None if previous_support_lengths is None else support_lengths != previous_support_lengths, "elite_representative_orbit_action_gap_signature": [{"canonical_sigma": list(word), "admissibility": status, "action_gap": gap} for word, status, gap, _, _ in action_signature], "elite_representative_active_orbit_canonical_signatures": [list(word) for word, _, _, _, _ in active_signature], "elite_representative_action_signature_recurred": None if previous_action_signature is None else action_signature == previous_action_signature, "elite_representative_active_branch_recurred": None if previous_active_signature is None else active_signature == previous_active_signature, "variance_by_coordinate": variance, "variance_change_by_coordinate_from_previous": None if previous_variance is None else [current - previous for current, previous in zip(variance, previous_variance)], "variance_sum": sum(variance), "variance_min": min(variance), "variance_max": max(variance), "variance_sum_change_from_previous": None if previous_variance is None else sum(variance) - sum(previous_variance)})
            previous_elite_ids, previous_elite_keys, previous_variance = elite_ids, elite_keys, variance
            previous_gaps, previous_radii = mean_gaps, mean_radii
            previous_action_signature, previous_active_signature = action_signature, active_signature
            previous_support_lengths = support_lengths
        by_replicate[str(replicate)] = generations
    return by_replicate


def local_path_metrics(grouped: dict[tuple[str, int], list[dict[str, Any]]], targets: dict[str, dict[str, Any]], cache_index: dict[tuple[str, int, str], dict[str, Any]], trajectory_records: dict[tuple[int, int], dict[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for replicate in REPLICATES:
        rows = grouped[("multistart_branch_local_phase0", replicate)]
        starts = sorted((row for row in rows if row.get("role") == "local_start" and row["evaluation_status"] == "success"), key=lambda row: row["attempt_index"])
        accepted = [row for row in rows if row.get("became_next_state")]
        transitions = []
        for row in accepted:
            parent_id = row.get("parent_candidate_id")
            parent = targets.get(parent_id)
            require(parent is not None, f"accepted local row {row['candidate_id']} has no target parent")
            require(row["evaluation_status"] == "success" and parent["evaluation_status"] == "success", f"accepted local row {row['candidate_id']} is not a successful transition")
            child_cache = cache_index[(row["arm"], row["replicate"], row["polytope_key"])]
            parent_cache = cache_index[(parent["arm"], parent["replicate"], parent["polytope_key"])]
            support_changed = row["support_lengths"] != parent["support_lengths"]
            parent_chart = chart_coordinates(parent, parent_id)
            child_chart = chart_coordinates(row, row["candidate_id"])
            parent_gaps, parent_radii = chart_gap_radius_features(parent, parent_id)
            child_gaps, child_radii = chart_gap_radius_features(row, row["candidate_id"])
            action_changed = action_gap_signature(child_cache, row["candidate_id"]) != action_gap_signature(parent_cache, parent["candidate_id"])
            parent_active = action_gap_signature(parent_cache, parent["candidate_id"], active_only=True)
            child_active = action_gap_signature(child_cache, row["candidate_id"], active_only=True)
            cyclic_gap_change = math.sqrt(sum((left - right) ** 2 for left, right in zip(parent_gaps, child_gaps)))
            centered_radius_change = math.sqrt(sum((left - right) ** 2 for left, right in zip(parent_radii, child_radii)))
            transitions.append({"candidate_id": row["candidate_id"], "parent_candidate_id": parent_id, "trajectory": row.get("trajectory"), "iteration": row.get("iteration"), "canonical_chart_distance": chart_distance(parent_chart, child_chart), "cyclic_gap_coordinate_change": cyclic_gap_change, "centered_radius_coordinate_change": centered_radius_change, "parent_cyclic_gaps": parent_gaps, "cyclic_gaps": child_gaps, "parent_centered_log_radii": parent_radii, "centered_log_radii": child_radii, "support_signature_changed": support_changed, "orbit_action_gap_signature_changed": action_changed, "parent_active_orbit_canonical_signatures": [list(word) for word, _, _, _, _ in parent_active], "active_orbit_canonical_signatures": [list(word) for word, _, _, _, _ in child_active], "active_orbit_canonical_signature_changed": parent_active != child_active})
        recurring = Counter((item["cyclic_gap_coordinate_change"] > 0.0, item["centered_radius_coordinate_change"] > 0.0, item["active_orbit_canonical_signature_changed"], item["support_signature_changed"], item["orbit_action_gap_signature_changed"]) for item in transitions)
        records = [trajectory_records[(replicate, start["trajectory"])] for start in starts]
        result[str(replicate)] = {"started_trajectories": len(starts), "completed_trajectories": sum(record["complete"] for record in records), "accepted_steps": len(accepted), "terminal_stops": dict(Counter(record["stop"] for record in records)), "accepted_transitions": transitions, "recurrent_accepted_change_patterns": [{"cyclic_gap_change_nonzero": gap_changed, "centered_radius_change_nonzero": radius_changed, "active_branch_changed": active_changed, "support_signature_changed": support_changed, "orbit_action_gap_signature_changed": action_changed, "count": count, "interpretation_level": "G"} for (gap_changed, radius_changed, active_changed, support_changed, action_changed), count in sorted(recurring.items())]}
    return result


def summarize(rows: list[dict[str, Any]], cache_rows: list[dict[str, Any]], cem_rows: list[dict[str, Any]], lineage_rows: list[dict[str, Any]], rejection_rows: list[dict[str, Any]], arm_runs: list[dict[str, Any]], local_trajectories: list[dict[str, Any]], run_status: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
    """Validate all artifacts then return only metrics derivable from them."""
    if config is not None:
        require(config == FROZEN_CONFIG, "resolved-config identity/constants differ from the frozen S0 contract")
    grouped = validate_target_rows(rows)
    cache_index = validate_cache(cache_rows, grouped)
    cem_records = validate_cem(rows, cem_rows, grouped)
    validate_lineages(rows, lineage_rows, cem_records)
    trajectory_records = validate_local_trajectories(grouped, local_trajectories)
    validate_local_policy(grouped, rejection_rows, trajectory_records)
    validate_rejections_and_arm_runs(rows, rejection_rows, arm_runs, grouped, trajectory_records)
    wall_time_accounting = validate_run_status(run_status, arm_runs, len(rows))

    per_replicate: dict[str, dict[str, dict[str, dict[str, Any]]]] = {}
    for arm in ARMS:
        per_replicate[arm] = {}
        for replicate in REPLICATES:
            ordered = grouped[(arm, replicate)]
            per_replicate[arm][str(replicate)] = {str(checkpoint): metrics(ordered[:checkpoint], checkpoint) for checkpoint in CHECKPOINTS}

    pooled: dict[str, dict[str, Any]] = {}
    for arm in ARMS:
        unique = distinct_successes([row for row in rows if row["arm"] == arm])
        if len(unique) < 8:
            fail(f"{arm}: fewer than eight pooled distinct successful keys")
        top = sorted((row["sys"] for row in unique.values()), reverse=True)[:8]
        pooled[arm] = {"attempts": len([row for row in rows if row["arm"] == arm]), "distinct_successful_keys": len(unique), "top_eight_median_sys": statistics.median(top), "best_sys": max(top)}

    material_ahead: dict[str, dict[str, Any]] = {}
    for arm in ARMS[1:]:
        paired = sum(per_replicate[arm][str(replicate)]["256"]["best_sys"] >= per_replicate["iid"][str(replicate)]["256"]["best_sys"] + 0.02 for replicate in REPLICATES)
        pooled_margin = pooled[arm]["top_eight_median_sys"] - pooled["iid"]["top_eight_median_sys"]
        material_ahead[arm] = {"paired_replicates_with_final_best_margin_at_least_0_02": paired, "pooled_top_eight_median_margin_over_iid": pooled_margin, "is_materially_ahead": paired >= 2 and pooled_margin >= 0.01}

    return {
        "schema_version": 1,
        "packet_version": FROZEN_CONFIG["packet_version"],
        "resolved_config_identity": {"schema_version": FROZEN_CONFIG["schema_version"], "master_seeds": FROZEN_CONFIG["master_seeds"], "target_attempts_per_arm_replicate": TARGET_ATTEMPTS},
        "question": "equal charged-target-budget search on fixed 5x5 Lagrangian products",
        "evidence_level": "operational numerical/exact-geometry; path and branch observations are G only",
        "per_replicate_checkpoints": per_replicate,
        "pooled": pooled,
        "material_ahead": material_ahead,
        "path_metrics": {"local": local_path_metrics(grouped, {row["candidate_id"]: row for row in rows}, cache_index, trajectory_records), "cem": cem_path_metrics(cem_records, {row["candidate_id"]: row for row in rows}, cache_index)},
        "key_and_chart_diagnostics": {arm: key_and_chart_diagnostics([row for row in rows if row["arm"] == arm], cache_index) for arm in ARMS},
        "unavailable_metrics": {"inferential_statistics": "unavailable: three fixed replicates are a robustness check, not an inferential sample", "causal_path_or_branch_attribution": "unavailable: descriptive path and lineage records do not identify causal mechanisms"},
        "accounting": {"target_rows": len(rows), "cache_rows": len(cache_rows), "cem_generation_rows": len(cem_rows), "lineage_rows": len(lineage_rows), "construction_rejection_rows": len(rejection_rows), "arm_run_rows": len(arm_runs), "local_trajectory_rows": len(local_trajectories), "charged_attempts_expected": len(ARMS) * len(REPLICATES) * TARGET_ATTEMPTS, "wall_time_ms": wall_time_accounting},
    }


def write_tsv(path: Path, summary: dict[str, Any]) -> None:
    with path.open("w", encoding="utf-8") as handle:
        handle.write("arm\treplicate\tcheckpoint\tbest_sys\ttop_eight_median_sys\tdistinct_successful_keys\tnormalized_auc\tcount_ge_0.85\tcount_ge_0.90\tcount_ge_0.95\tcount_ge_1.0\tnew_computations\tcache_hits\tfailed_new_computations\tconstruction_attempts\tconstruction_rejections\n")
        for arm in ARMS:
            for replicate in REPLICATES:
                for checkpoint in CHECKPOINTS:
                    row = summary["per_replicate_checkpoints"][arm][str(replicate)][str(checkpoint)]
                    counts = row["counts_above"]
                    handle.write(f"{arm}\t{replicate}\t{checkpoint}\t{row['best_sys']:.17g}\t{row['top_eight_median_sys']:.17g}\t{row['distinct_successful_keys']}\t{row['normalized_best_so_far_auc']:.17g}\t{counts['0.85']}\t{counts['0.9']}\t{counts['0.95']}\t{counts['1.0']}\t{row['successful_new_computations']}\t{row['cache_hits']}\t{row['failed_new_computations']}\t{row['construction_attempts']}\t{row['construction_rejections']}\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifacts", type=Path, default=Path(__file__).parent / "artifacts")
    parser.add_argument("--config", type=Path, default=Path(__file__).parent / "resolved-config.json")
    args = parser.parse_args()
    artifacts = args.artifacts
    config = identical_config_artifact(artifacts, args.config)
    result = summarize(read_jsonl(artifacts / "target-evaluations.jsonl"), read_jsonl(artifacts / "expensive-computation-cache.jsonl"), read_jsonl(artifacts / "cem-generations.jsonl"), read_jsonl(artifacts / "lineages.jsonl"), read_jsonl(artifacts / "construction-rejections.jsonl"), read_jsonl(artifacts / "arm-runs.jsonl"), read_jsonl(artifacts / "local-trajectories.jsonl"), json.loads((artifacts / "run-status.json").read_text(encoding="utf-8")), config)
    (artifacts / "comparison-summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    write_tsv(artifacts / "comparison-summary.tsv", result)


if __name__ == "__main__":
    main()
