#!/usr/bin/env python3
"""Independent fail-closed verifier for the 64-request AMS readiness smoke.

Only a complete, valid 48-adaptive/16-IID artifact can pass readiness. A
timeout, failure, or sys > 1 stop can be internally auditable but never passes
the readiness gate. This verifier makes no arm-quality, probability, or
scientific-negative claim.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import subprocess
from fractions import Fraction
from pathlib import Path
from typing import Any, Iterable


class ArtifactError(RuntimeError):
    pass


JSONL_FILES = (
    "charged-requests.jsonl",
    "target-evaluations.jsonl",
    "cache.jsonl",
    "construction-rejections.jsonl",
    "mutation-transitions.jsonl",
    "levels.jsonl",
    "arm-runs.jsonl",
)
MANIFEST_FIELDS = {
    "artifact_kind", "run_id", "start_unix_ms", "launch_process_id", "artifact_directory",
    "config_identity", "exact_config", "source", "adaptive_budget", "iid_budget",
    "target_probability_estimate", "tail_probability_supported", "mutation_kernel",
    "generation_schedule", "factor_exchange_quotiented",
}
SOURCE_FIELDS = {
    "git_revision", "reviewed_revision", "source_tree_clean", "executable_sha256",
    "cargo_lock_sha256", "production_target",
}
STATUS_FIELDS = {
    "run_id", "disposition", "error", "terminal_error", "end_unix_ms",
    "total_monotonic_wall_time_ms", "adaptive_charged_requests", "iid_charged_requests",
    "total_charged_requests", "artifact_sha256",
}
TERMINAL_ERROR_FIELDS = {
    "kind", "arm", "global_request_index", "candidate_id", "evaluation_status",
    "failure_reason", "next_schedule_identity", "level", "observed_distinct_geometry_keys",
    "required_distinct_geometry_keys",
}
CHARGED_FIELDS = {
    "global_request_index", "candidate_id", "identity", "arm", "attempt_index",
    "exact_geometry_key", "geometry_identity", "dual_vertices_rational", "dual_vertices_f64",
    "facet_count", "parent_candidate_id", "root_candidate_id", "level_threshold",
    "raw_proposed_chart", "product_chart", "charged_monotonic_ms",
}
TARGET_FIELDS = CHARGED_FIELDS - {"charged_monotonic_ms"} | {
    "cache_status", "evaluation_status", "failure_reason", "capacity", "volume", "sys",
    "diagnostics", "audit_kind", "started_monotonic_ms", "wall_time_ms",
    "cumulative_monotonic_ms",
}
CACHE_FIELDS = {
    "arm", "exact_geometry_key", "geometry_identity", "dual_vertices_rational",
    "dual_vertices_f64", "facet_count", "product_chart", "capacity", "volume", "sys",
    "diagnostics", "capacity_result", "audit_kind",
}
REJECTION_FIELDS = {
    "candidate_id", "identity", "arm", "reason", "parent_candidate_id",
    "root_candidate_id", "raw_proposed_chart",
}
TRANSITION_FIELDS = {
    "level", "clone_index", "mutation_step", "frozen_threshold",
    "state_before_candidate_id", "proposal_candidate_id", "proposal_sys", "accepted",
    "state_after_candidate_id", "root_candidate_id",
}
LEVEL_FIELDS = {
    "level", "frozen_threshold", "survivor_candidate_ids", "survivor_root_candidate_ids",
    "clone_parent_candidate_ids", "post_level_population_candidate_ids",
    "post_level_population_geometry_keys", "post_level_distinct_geometry_keys",
}
ARM_RUN_FIELDS = {
    "arm", "target_attempts", "construction_rejections", "cache_misses", "cache_hits",
    "failed_misses", "distinct_successful_keys", "wall_time_ms", "cumulative_monotonic_ms",
    "started_monotonic_ms", "complete",
}
ROW_FIELDS = {
    "charged-requests.jsonl": CHARGED_FIELDS,
    "target-evaluations.jsonl": TARGET_FIELDS,
    "cache.jsonl": CACHE_FIELDS,
    "construction-rejections.jsonl": REJECTION_FIELDS,
    "mutation-transitions.jsonl": TRANSITION_FIELDS,
    "levels.jsonl": LEVEL_FIELDS,
    "arm-runs.jsonl": ARM_RUN_FIELDS,
}
STOP_ACTION = (
    "artifacts_flushed_stop_unrelated_search_independent_validation_required"
)
CHART_TOLERANCE = 2.0e-10
RAW_CHART_TOLERANCE = 2.0e-12
WALL_CLOCK_RECONCILIATION_TOLERANCE_MS = 100.0
MONOTONIC_INTERVAL_TOLERANCE_MS = 5.0
DEADLINE_RECONCILIATION_TOLERANCE_MS = 100.0
MUTATION_GEOMETRY_TOLERANCE = 2.0e-12
IDENTITY_FIELDS = {
    "packet_version",
    "config_identity",
    "source_revision",
    "parent_candidate_id",
    "master_seed",
    "replicate",
    "arm",
    "level",
    "clone_index",
    "mutation_step",
    "base_index",
    "construction_attempt",
}


def fail(message: str) -> None:
    raise ArtifactError(message)


def require_fields(value: Any, expected: set[str], label: str) -> None:
    if not isinstance(value, dict) or set(value) != expected:
        fail(f"{label} has missing or extra fields")


def read_json(path: Path) -> Any:
    try:
        return strict_json_loads(path.read_text())
    except (OSError, json.JSONDecodeError, ValueError) as error:
        fail(f"cannot read valid JSON {path}: {error}")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    try:
        lines = path.read_text().splitlines()
    except OSError as error:
        fail(f"cannot read {path}: {error}")
    rows: list[dict[str, Any]] = []
    for number, line in enumerate(lines, 1):
        if not line.strip():
            fail(f"blank JSONL row in {path}:{number}")
        try:
            row = strict_json_loads(line)
        except (json.JSONDecodeError, ValueError) as error:
            fail(f"invalid JSON in {path}:{number}: {error}")
        if not isinstance(row, dict):
            fail(f"non-object JSONL row in {path}:{number}")
        rows.append(row)
    return rows


def strict_json_loads(text: str) -> Any:
    def object_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"duplicate JSON object key {key!r}")
            result[key] = value
        return result

    def reject_constant(value: str) -> None:
        raise ValueError(f"nonstandard JSON constant {value}")

    return json.loads(text, object_pairs_hook=object_pairs, parse_constant=reject_constant)


def compact_json(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode()


def sha256(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def file_sha256(path: Path) -> str:
    try:
        return sha256(path.read_bytes())
    except OSError as error:
        fail(f"cannot hash {path}: {error}")


def option(value: Any) -> str:
    return "none" if value is None else str(value)


def expected_candidate_id(identity: dict[str, Any]) -> str:
    if set(identity) != IDENTITY_FIELDS:
        fail("candidate identity has missing or extra fields")
    material = (
        f"packet={identity['packet_version']}\n"
        f"config={identity['config_identity']}\n"
        f"source={identity['source_revision']}\n"
        f"parent={option(identity['parent_candidate_id'])}\n"
        f"seed={identity['master_seed']}\n"
        f"replicate={identity['replicate']}\n"
        f"arm={identity['arm']}\n"
        f"level={option(identity['level'])}\n"
        f"clone={option(identity['clone_index'])}\n"
        f"step={option(identity['mutation_step'])}\n"
        f"base={option(identity['base_index'])}\n"
        f"construction={identity['construction_attempt']}\n"
    )
    return "amsv1-" + sha256(material.encode())[:24]


def validate_identity(
    identity: Any,
    *,
    config: dict[str, Any],
    config_id: str,
    source_revision: str,
) -> None:
    if not isinstance(identity, dict):
        fail("row lacks candidate identity")
    fixed = {
        "packet_version": config["packet_version"],
        "config_identity": config_id,
        "source_revision": source_revision,
        "master_seed": config["master_seed"],
        "replicate": config["replicate"],
    }
    if any(identity.get(field) != expected for field, expected in fixed.items()):
        fail("candidate identity changes a fixed packet/source field")
    if not exact_integer(identity.get("master_seed")) or not exact_integer(identity.get("replicate")):
        fail("candidate identity seed/replicate has a non-integer JSON type")
    arm = identity.get("arm")
    if arm not in {"adaptive", "iid"}:
        fail("candidate identity has an invalid arm")
    attempt = identity.get("construction_attempt")
    if not exact_integer(attempt) or attempt >= config["construction_retry_cap"]:
        fail("candidate identity construction attempt is outside the retry cap")
    if identity.get("level") is None:
        if (
            identity.get("parent_candidate_id") is not None
            or identity.get("clone_index") is not None
            or identity.get("mutation_step") is not None
            or not exact_integer(identity.get("base_index"))
        ):
            fail("base candidate identity has mutation fields")
    else:
        if (
            arm != "adaptive"
            or not exact_integer(identity.get("level"))
            or not exact_integer(identity.get("clone_index"))
            or not exact_integer(identity.get("mutation_step"))
            or identity.get("base_index") is not None
            or not isinstance(identity.get("parent_candidate_id"), str)
        ):
            fail("mutation candidate identity has invalid fields")


def exact_key(vertices: list[list[str]]) -> str:
    return "|".join(",".join(row) for row in vertices)


def finite_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def finite_positive(value: Any) -> bool:
    return finite_number(value) and value > 0


def exact_integer(value: Any, minimum: int = 0) -> bool:
    return isinstance(value, int) and not isinstance(value, bool) and value >= minimum


def parse_exact_vertices(value: Any, label: str) -> list[list[Fraction]]:
    if not isinstance(value, list) or len(value) != 10:
        fail(f"{label} lacks ten exact dual vertices")
    parsed: list[list[Fraction]] = []
    for row in value:
        if not isinstance(row, list) or len(row) != 4 or not all(isinstance(x, str) for x in row):
            fail(f"{label} has malformed exact dual vertices")
        try:
            parsed_row = [Fraction(x) for x in row]
        except (ValueError, ZeroDivisionError) as error:
            fail(f"{label} has invalid rational coordinate: {error}")
        for text, rational in zip(row, parsed_row):
            canonical = f"{rational.numerator}/{rational.denominator}"
            if text != canonical:
                fail(f"{label} has noncanonical or unreduced rational coordinate")
        parsed.append(parsed_row)
    return parsed


def cross(a: tuple[Fraction, Fraction], b: tuple[Fraction, Fraction], c: tuple[Fraction, Fraction]) -> Fraction:
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])


def convex_hull(points: list[tuple[Fraction, Fraction]]) -> list[tuple[Fraction, Fraction]]:
    ordered = sorted(set(points))
    if len(ordered) < 3:
        return ordered
    lower: list[tuple[Fraction, Fraction]] = []
    for point in ordered:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], point) <= 0:
            lower.pop()
        lower.append(point)
    upper: list[tuple[Fraction, Fraction]] = []
    for point in reversed(ordered):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], point) <= 0:
            upper.pop()
        upper.append(point)
    return lower[:-1] + upper[:-1]


def verify_factor(points: list[tuple[Fraction, Fraction]], label: str) -> None:
    if len(set(points)) != 5:
        fail(f"{label} factor has duplicate dual vertices")
    hull = convex_hull(points)
    if len(hull) != 5:
        fail(f"{label} factor is not irredundant/extreme")
    origin = (Fraction(0), Fraction(0))
    signs = [cross(hull[i], hull[(i + 1) % 5], origin) for i in range(5)]
    if any(value <= 0 for value in signs):
        fail(f"{label} factor does not strictly contain the origin")


def exact_primal_area(dual_points: list[tuple[Fraction, Fraction]], label: str) -> Fraction:
    hull = convex_hull(dual_points)
    if len(hull) != 5:
        fail(f"{label} factor cannot reconstruct an exact primal polygon")
    primal: list[tuple[Fraction, Fraction]] = []
    for left, right in zip(hull, hull[1:] + hull[:1]):
        determinant = left[0] * right[1] - left[1] * right[0]
        if determinant == 0:
            fail(f"{label} adjacent dual supports do not have a unique intersection")
        primal.append(
            (
                (right[1] - left[1]) / determinant,
                (left[0] - right[0]) / determinant,
            )
        )
    twice_area = sum(
        left[0] * right[1] - left[1] * right[0]
        for left, right in zip(primal, primal[1:] + primal[:1])
    )
    area = abs(twice_area) / 2
    if area <= 0:
        fail(f"{label} reconstructed primal factor has nonpositive area")
    return area


def exact_product_volume(parsed: list[list[Fraction]]) -> Fraction:
    q_area = exact_primal_area([(row[0], row[1]) for row in parsed[:5]], "q")
    p_area = exact_primal_area([(row[2], row[3]) for row in parsed[5:]], "p")
    return q_area * p_area


def wrap_phase(value: float) -> float:
    return value % math.tau if math.isfinite(value) else value


def rotations_near(gaps: list[float], radii: list[float], left: int, right: int) -> bool:
    for offset in range(5):
        li = (left + offset) % 5
        ri = (right + offset) % 5
        if gaps[li] != gaps[ri]:
            return abs(gaps[li] - gaps[ri]) <= 1.0e-10
        if radii[li] != radii[ri]:
            return abs(radii[li] - radii[ri]) <= 1.0e-10
        if offset == 0:
            return True
    return True


def encode_factor(points: list[tuple[Fraction, Fraction]]) -> tuple[list[float], list[float], float, bool]:
    entries = sorted(
        (wrap_phase(math.atan2(float(y), float(x))), math.log(math.hypot(float(x), float(y))))
        for x, y in points
    )
    mean = sum(radius for _, radius in entries) / 5.0
    angles = [angle for angle, _ in entries]
    radii = [radius - mean for _, radius in entries]
    gaps = [wrap_phase(angles[(i + 1) % 5] - angles[i]) for i in range(5)]
    best = min(
        range(5),
        key=lambda start: tuple(
            component
            for offset in range(5)
            for component in (gaps[(start + offset) % 5], radii[(start + offset) % 5])
        ),
    )
    near = any(rotations_near(gaps, radii, best, other) for other in range(5) if other != best)
    return (
        [gaps[(best + i) % 5] for i in range(5)],
        [radii[(best + i) % 5] for i in range(5)],
        angles[best],
        near,
    )


def chart_from_exact(parsed: list[list[Fraction]]) -> dict[str, Any]:
    q_points = [(row[0], row[1]) for row in parsed[:5]]
    p_points = [(row[2], row[3]) for row in parsed[5:]]
    q_gaps, q_radii, q_origin, q_near = encode_factor(q_points)
    p_gaps, p_radii, p_origin, p_near = encode_factor(p_points)
    return {
        "q_gap_logits": [math.log(q_gaps[i] / q_gaps[4]) for i in range(4)],
        "q_centered_log_radii": q_radii,
        "p_gap_logits": [math.log(p_gaps[i] / p_gaps[4]) for i in range(4)],
        "p_centered_log_radii": p_radii,
        "relative_phase": wrap_phase(p_origin - q_origin),
        "near_tie": q_near or p_near,
    }


def validate_chart_shape(chart: Any, label: str) -> None:
    if not isinstance(chart, dict) or set(chart) != {
        "q_gap_logits",
        "q_centered_log_radii",
        "p_gap_logits",
        "p_centered_log_radii",
        "relative_phase",
        "near_tie",
    }:
        fail(f"{label} has malformed chart fields")
    for name, length in (
        ("q_gap_logits", 4),
        ("q_centered_log_radii", 5),
        ("p_gap_logits", 4),
        ("p_centered_log_radii", 5),
    ):
        values = chart[name]
        if not isinstance(values, list) or len(values) != length or not all(finite_number(x) for x in values):
            fail(f"{label} has invalid finite chart field {name}")
    if not finite_number(chart["relative_phase"]) or not isinstance(chart["near_tie"], bool):
        fail(f"{label} has invalid phase/tie chart fields")


def charts_close(actual: Any, expected: Any, tolerance: float, label: str) -> None:
    validate_chart_shape(actual, label)
    validate_chart_shape(expected, f"expected {label}")
    for name in (
        "q_gap_logits",
        "q_centered_log_radii",
        "p_gap_logits",
        "p_centered_log_radii",
    ):
        if any(abs(a - b) > tolerance for a, b in zip(actual[name], expected[name])):
            fail(f"{label} disagrees with independently reconstructed {name}")
    if abs((actual["relative_phase"] - expected["relative_phase"] + math.pi) % math.tau - math.pi) > tolerance:
        fail(f"{label} disagrees with independently reconstructed phase")
    if actual["near_tie"] != expected["near_tie"]:
        fail(f"{label} disagrees with independently reconstructed tie diagnostic")


def verify_geometry(row: dict[str, Any], label: str) -> Fraction:
    raw_vertices = row.get("dual_vertices_rational")
    parsed = parse_exact_vertices(raw_vertices, label)
    zero = Fraction(0)
    for index, vertex in enumerate(parsed):
        if index < 5:
            if vertex[2:] != [zero, zero] or vertex[:2] == [zero, zero]:
                fail(f"{label} violates the five-q/five-p product structure")
        elif vertex[:2] != [zero, zero] or vertex[2:] == [zero, zero]:
            fail(f"{label} violates the five-q/five-p product structure")
    verify_factor([(row[0], row[1]) for row in parsed[:5]], "q")
    verify_factor([(row[2], row[3]) for row in parsed[5:]], "p")
    if not exact_integer(row.get("facet_count")) or row.get("facet_count") != 10:
        fail(f"{label} facet count is not fixed 5 x 5")
    if exact_key(raw_vertices) != row.get("exact_geometry_key"):
        fail(f"{label} exact key disagrees with geometry")
    if sha256(compact_json(raw_vertices)) != row.get("geometry_identity"):
        fail(f"{label} stable geometry identity mismatch")
    f64_vertices = row.get("dual_vertices_f64")
    if not isinstance(f64_vertices, list) or len(f64_vertices) != 10:
        fail(f"{label} lacks f64 child geometry")
    for exact_row, float_row in zip(parsed, f64_vertices):
        if not isinstance(float_row, list) or len(float_row) != 4 or not all(finite_number(x) for x in float_row):
            fail(f"{label} has malformed f64 child geometry")
        if any(
            not math.isclose(float(exact), value, rel_tol=2.0e-15, abs_tol=2.0e-17)
            for exact, value in zip(exact_row, float_row)
        ):
            fail(f"{label} exact and f64 child geometry disagree")
    charts_close(row.get("product_chart"), chart_from_exact(parsed), CHART_TOLERANCE, f"{label} canonical chart")
    return exact_product_volume(parsed)


def verify_artifact_hashes(directory: Path, manifest: dict[str, Any], status: dict[str, Any]) -> None:
    require_fields(status, STATUS_FIELDS, "run status")
    if status.get("run_id") != manifest.get("run_id"):
        fail("manifest and final status run IDs disagree")
    expected_names = {"manifest.json", *JSONL_FILES}
    if (directory / "stop-event.json").exists():
        expected_names.add("stop-event.json")
    try:
        actual_names = {path.name for path in directory.iterdir() if path.is_file()}
    except OSError as error:
        fail(f"cannot enumerate artifact directory: {error}")
    if actual_names != expected_names | {"run-status.json"}:
        fail("artifact directory contains missing or unexpected files")
    hashes = status.get("artifact_sha256")
    if not isinstance(hashes, dict) or set(hashes) != expected_names:
        fail("final status does not bind exactly the owning artifact files")
    for name, expected in hashes.items():
        if not isinstance(expected, str) or len(expected) != 64:
            fail(f"final status has malformed hash for {name}")
        if file_sha256(directory / name) != expected:
            fail(f"final status hash mismatch for {name}")


def git_output(root: Path, *args: str) -> str:
    try:
        result = subprocess.run(
            ["git", "-C", str(root), *args],
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        fail(f"cannot verify current repository identity: {error}")
    return result.stdout.strip()


def verify_production_identity(
    source: dict[str, Any],
    expected_reviewed_revision: str | None,
    repo_root: Path | None,
    cargo_lock: Path | None,
    executable: Path | None,
) -> None:
    if any(value is None for value in (expected_reviewed_revision, repo_root, cargo_lock, executable)):
        fail("production analysis requires reviewed revision, repo root, Cargo.lock, and executable")
    assert expected_reviewed_revision is not None
    assert repo_root is not None and cargo_lock is not None and executable is not None
    if len(expected_reviewed_revision) != 40 or any(c not in "0123456789abcdefABCDEF" for c in expected_reviewed_revision):
        fail("expected reviewed revision is not full 40-hex")
    if source.get("reviewed_revision") != expected_reviewed_revision:
        fail("manifest reviewed revision differs from analyzer expectation")
    if source.get("git_revision") != expected_reviewed_revision:
        fail("manifest source revision differs from reviewed revision")
    if git_output(repo_root, "rev-parse", "HEAD") != expected_reviewed_revision:
        fail("current repository HEAD differs from reviewed revision")
    if git_output(repo_root, "status", "--porcelain", "--untracked-files=normal"):
        fail("current repository is dirty during production analysis")
    if file_sha256(cargo_lock) != source.get("cargo_lock_sha256"):
        fail("current packet Cargo.lock hash differs from manifest")
    if file_sha256(executable) != source.get("executable_sha256"):
        fail("current executable hash differs from manifest")


def validate_manifest(
    manifest: dict[str, Any],
    *,
    expected_reviewed_revision: str | None,
    repo_root: Path | None,
    cargo_lock: Path | None,
    executable: Path | None,
) -> tuple[dict[str, Any], str, str]:
    require_fields(manifest, MANIFEST_FIELDS, "manifest")
    config = manifest.get("exact_config")
    if not isinstance(config, dict):
        fail("manifest exact_config is missing")
    config_id = sha256(compact_json(config))
    if manifest.get("config_identity") != config_id:
        fail("manifest config_identity does not bind exact_config")
    fixed_fields = {
        "packet_version": "ams-readiness-smoke-v1",
        "master_seed": 202607150101,
        "replicate": 0,
        "initial_particles": 16,
        "levels": 2,
        "survivors_per_level": 8,
        "clones_per_level": 8,
        "mutation_steps_per_clone": 2,
        "iid_requests": 16,
        "construction_retry_cap": 64,
        "abort_wall_time_seconds": 900,
        "gap_logit_scale": 0.08,
        "centered_log_radius_scale": 0.04,
        "phase_scale": 0.08,
        "tie_rule": "sys_desc_candidate_id_asc",
        "clone_assignment": "seeded_uniform_with_replacement",
        "acceptance_rule": "successful_sys_at_least_frozen_level_threshold",
        "factor_exchange_quotiented": False,
    }
    if set(config) != set(fixed_fields):
        fail("config has missing or extra frozen fields")
    integer_config_fields = {
        "master_seed", "replicate", "initial_particles", "levels", "survivors_per_level",
        "clones_per_level", "mutation_steps_per_clone", "iid_requests",
        "construction_retry_cap", "abort_wall_time_seconds",
    }
    if any(not exact_integer(config[field]) for field in integer_config_fields):
        fail("config integer fields have non-integer JSON types")
    if not isinstance(config["factor_exchange_quotiented"], bool):
        fail("config factor-exchange flag is not Boolean")
    for field, expected in fixed_fields.items():
        if config.get(field) != expected:
            fail(f"config changes frozen field {field}")
    manifest_fixed = {
        "adaptive_budget": 48,
        "iid_budget": 16,
        "target_probability_estimate": None,
        "tail_probability_supported": False,
        "mutation_kernel": "non_invariant_threshold_only_gaussian",
        "generation_schedule": "sha256_counter_box_muller_v1",
        "factor_exchange_quotiented": False,
    }
    if any(manifest.get(field) != expected for field, expected in manifest_fixed.items()):
        fail("manifest changes the fixed budget, policy, or claim boundary")
    if not exact_integer(manifest.get("adaptive_budget")) or not exact_integer(manifest.get("iid_budget")):
        fail("manifest budgets have non-integer JSON types")
    if not isinstance(manifest["tail_probability_supported"], bool) or not isinstance(
        manifest["factor_exchange_quotiented"], bool
    ):
        fail("manifest claim-boundary flags are not Boolean")
    if not exact_integer(manifest.get("start_unix_ms"), 1):
        fail("manifest lacks a pre-exposure start timestamp")
    process_id = manifest.get("launch_process_id")
    artifact_directory = manifest.get("artifact_directory")
    if not exact_integer(process_id, 1) or not isinstance(artifact_directory, str):
        fail("manifest lacks recomputable launch identity fields")
    kind = manifest.get("artifact_kind")
    if kind not in {"synthetic_target_free", "production_target"}:
        fail("unknown artifact kind")
    source = manifest.get("source")
    require_fields(source, SOURCE_FIELDS, "manifest source identity")
    if not isinstance(source["source_tree_clean"], bool) or not isinstance(source["production_target"], bool):
        fail("source identity flags are not Boolean")
    if source.get("production_target") != (kind == "production_target"):
        fail("source identity disagrees with artifact kind")
    if not isinstance(source.get("git_revision"), str):
        fail("manifest lacks source revision")
    for field in ("executable_sha256", "cargo_lock_sha256"):
        value = source.get(field)
        if not isinstance(value, str) or (kind == "production_target" and len(value) != 64):
            fail(f"invalid source identity field {field}")
    if kind == "production_target":
        if source.get("source_tree_clean") is not True:
            fail("production artifacts came from a dirty source tree")
        verify_production_identity(source, expected_reviewed_revision, repo_root, cargo_lock, executable)
    material = (
        f"ams-readiness-run-v1\n{manifest['start_unix_ms']}\n{process_id}\n"
        f"{source['git_revision']}\n{artifact_directory}\n"
    )
    expected_run_id = "amsrun-" + sha256(material.encode())[:24]
    if manifest.get("run_id") != expected_run_id:
        fail("manifest run ID is not recomputable from launch identity")
    return config, config_id, kind


def logical_identity(identity: dict[str, Any]) -> tuple[Any, ...]:
    return tuple(identity[field] for field in sorted(IDENTITY_FIELDS - {"construction_attempt"}))


def copy_identity_with_attempt(identity: dict[str, Any], attempt: int) -> dict[str, Any]:
    result = dict(identity)
    result["construction_attempt"] = attempt
    return result


def request_schedule_token(identity: dict[str, Any]) -> tuple[Any, ...]:
    return (
        identity["arm"],
        identity["level"],
        identity["clone_index"],
        identity["mutation_step"],
        identity["base_index"],
    )


def frozen_request_schedule() -> list[tuple[Any, ...]]:
    schedule = [("adaptive", None, None, None, base) for base in range(16)]
    schedule.extend(
        ("adaptive", level, clone, step, None)
        for level in range(2)
        for clone in range(8)
        for step in range(2)
    )
    schedule.extend(("iid", None, None, None, base) for base in range(16))
    return schedule


def sha_u64(material: str) -> int:
    return int.from_bytes(hashlib.sha256(material.encode()).digest()[:8], "big")


def expected_clone_parent(config_id: str, master_seed: int, level: int, clone: int, survivors: list[str]) -> str:
    material = f"ams-clone-assignment-v1\n{config_id}\n{master_seed}\n{level}\n{clone}\n"
    return survivors[sha_u64(material) % len(survivors)]


def unit_from_digest(chunk: bytes) -> float:
    bits = int.from_bytes(chunk, "big") >> 11
    return (bits + 0.5) / float(1 << 53)


def standard_normal(candidate: str, coordinate: int) -> float:
    pair = coordinate // 2
    digest = hashlib.sha256(f"ams-mutation-gaussian-v1\n{candidate}\n{pair}\n".encode()).digest()
    radius = math.sqrt(-2.0 * math.log(unit_from_digest(digest[:8])))
    angle = math.tau * unit_from_digest(digest[8:16])
    return radius * (math.cos(angle) if coordinate % 2 == 0 else math.sin(angle))


def continuous_coordinates(chart: dict[str, Any]) -> list[float]:
    return [
        *chart["q_gap_logits"],
        *chart["q_centered_log_radii"][:4],
        *chart["p_gap_logits"],
        *chart["p_centered_log_radii"][:4],
        chart["relative_phase"],
    ]


def chart_from_coordinates(values: list[float]) -> dict[str, Any]:
    return {
        "q_gap_logits": values[:4],
        "q_centered_log_radii": [*values[4:8], -sum(values[4:8])],
        "p_gap_logits": values[8:12],
        "p_centered_log_radii": [*values[12:16], -sum(values[12:16])],
        "relative_phase": wrap_phase(values[16]),
        "near_tie": False,
    }


def expected_raw_mutation(config: dict[str, Any], before_chart: dict[str, Any], identity: dict[str, Any]) -> dict[str, Any]:
    candidate = expected_candidate_id(identity)
    values = continuous_coordinates(before_chart)
    for index in range(17):
        if index in {*range(4), *range(8, 12)}:
            scale = config["gap_logit_scale"]
        elif index == 16:
            scale = config["phase_scale"]
        else:
            scale = config["centered_log_radius_scale"]
        values[index] += scale * standard_normal(candidate, index)
    return chart_from_coordinates(values)


def decode_chart_factor(logits: list[float], log_radii: list[float], origin: float) -> list[list[float]]:
    maximum = max(0.0, *logits)
    weights = [math.exp(value - maximum) for value in logits] + [math.exp(-maximum)]
    weight_sum = sum(weights)
    gaps = [math.tau * weight / weight_sum for weight in weights]
    angle = wrap_phase(origin)
    points: list[list[float]] = []
    for gap, log_radius in zip(gaps, log_radii):
        radius = math.exp(log_radius)
        points.append([radius * math.cos(angle), radius * math.sin(angle)])
        angle = wrap_phase(angle + gap)
    return points


def dual_geometry_from_raw_chart(chart: dict[str, Any]) -> list[list[float]]:
    validate_chart_shape(chart, "raw chart geometry decoder")
    q = decode_chart_factor(chart["q_gap_logits"], chart["q_centered_log_radii"], 0.0)
    p = decode_chart_factor(
        chart["p_gap_logits"], chart["p_centered_log_radii"], chart["relative_phase"]
    )
    return [[x, y, 0.0, 0.0] for x, y in q] + [[0.0, 0.0, x, y] for x, y in p]


def verify_mutation_chart_geometry(row: dict[str, Any], label: str) -> None:
    expected = dual_geometry_from_raw_chart(row["raw_proposed_chart"])
    actual = row["dual_vertices_f64"]
    for expected_vertex, actual_vertex in zip(expected, actual):
        if any(
            not math.isclose(left, right, rel_tol=MUTATION_GEOMETRY_TOLERANCE, abs_tol=MUTATION_GEOMETRY_TOLERANCE)
            for left, right in zip(expected_vertex, actual_vertex)
        ):
            fail(f"{label} resulting geometry is not decoded from its raw mutation chart")


def verify_diversity_terminal_payload(
    terminal: dict[str, Any], levels: list[dict[str, Any]], targets: list[dict[str, Any]]
) -> None:
    if not levels:
        fail("diversity-gate terminal evidence has no completed level")
    final_level = levels[-1]
    if (
        terminal["arm"] != "adaptive"
        or terminal["global_request_index"]
        != (targets[-1]["global_request_index"] if targets else None)
        or terminal["candidate_id"] != (targets[-1]["candidate_id"] if targets else None)
        or terminal["evaluation_status"] is not None
        or terminal["failure_reason"] is not None
        or terminal["next_schedule_identity"] is not None
        or terminal["level"] != final_level["level"]
        or terminal["observed_distinct_geometry_keys"]
        != final_level["post_level_distinct_geometry_keys"]
        or terminal["required_distinct_geometry_keys"] != 8
        or terminal["observed_distinct_geometry_keys"] >= 8
    ):
        fail("diversity-gate terminal evidence disagrees with final completed level")


def validate_diagnostics(row: dict[str, Any], label: str) -> None:
    diagnostics = row.get("diagnostics")
    if not isinstance(diagnostics, dict) or set(diagnostics) != {
        "iterations",
        "returned_orbit_count",
        "action_lower",
        "action_upper",
        "exact_admissible_count",
        "indeterminate_count",
    }:
        fail(f"{label} lacks compact target diagnostics")
    if not all(exact_integer(diagnostics[field]) for field in (
        "iterations", "returned_orbit_count", "exact_admissible_count", "indeterminate_count"
    )):
        fail(f"{label} has invalid diagnostic counts")
    if not finite_number(diagnostics["action_lower"]) or not finite_number(diagnostics["action_upper"]):
        fail(f"{label} has invalid action interval")
    if diagnostics["action_lower"] > diagnostics["action_upper"]:
        fail(f"{label} has reversed action interval")


def verify_full_cache_audit(row: dict[str, Any], kind: str) -> None:
    validate_diagnostics(row, "cache row")
    result = row.get("capacity_result")
    if kind == "synthetic_target_free":
        if result is not None or row.get("audit_kind") != "synthetic_formula_fixture":
            fail("synthetic cache row misrepresents a production orbit audit")
        return
    if not isinstance(result, dict) or row.get("audit_kind") != "full_orbit_search_result":
        fail("production cache miss lacks full OrbitSearchResult")
    require_fields(
        result,
        {"orbits", "min_action", "min_action_lower", "min_action_upper", "iterations"},
        "full OrbitSearchResult",
    )
    orbits = result.get("orbits")
    diagnostics = row["diagnostics"]
    if not isinstance(orbits, list) or not orbits or len(orbits) != diagnostics["returned_orbit_count"]:
        fail("full cache audit orbit count disagrees with compact diagnostics")
    if not exact_integer(result.get("iterations")) or result["iterations"] != diagnostics["iterations"]:
        fail("full cache audit iteration count disagrees with compact diagnostics")
    if any(not finite_number(result.get(field)) for field in ("min_action", "min_action_lower", "min_action_upper")):
        fail("full cache audit has invalid aggregate action values")
    if result.get("min_action") != row.get("capacity"):
        fail("full cache audit minimum action disagrees with capacity")
    if result.get("min_action_lower") != diagnostics["action_lower"] or result.get("min_action_upper") != diagnostics["action_upper"]:
        fail("full cache audit action interval disagrees with compact diagnostics")
    orbit_fields = {
        "sigma", "beta", "beta_margin", "action", "action_lower", "action_upper", "q",
        "q_error_bound", "mu", "xi", "admissibility",
    }
    seen_sigma: set[tuple[int, ...]] = set()
    admissible_actions: list[float] = []
    lower_values: list[float] = []
    upper_values: list[float] = []
    previous_lower = -math.inf
    exact = 0
    indeterminate = 0
    for index, orbit in enumerate(orbits):
        require_fields(orbit, orbit_fields, f"production orbit {index}")
        sigma = orbit["sigma"]
        beta = orbit["beta"]
        if (
            not isinstance(sigma, list) or not sigma
            or any(not isinstance(value, int) or isinstance(value, bool) or not 0 <= value < 10 for value in sigma)
            or len(set(sigma)) != len(sigma)
            or not isinstance(beta, list) or len(beta) != len(sigma)
            or any(not finite_number(value) for value in beta)
        ):
            fail(f"production orbit {index} has invalid sigma/beta payload")
        sigma_key = tuple(sigma)
        if sigma_key in seen_sigma:
            fail("full cache audit contains duplicate orbit sigma")
        seen_sigma.add(sigma_key)
        if orbit["beta_margin"] != min(beta):
            fail(f"production orbit {index} beta margin is not derived from beta")
        if any(not finite_number(orbit[field]) for field in (
            "beta_margin", "action", "action_lower", "action_upper", "q", "q_error_bound"
        )):
            fail(f"production orbit {index} has nonfinite scalar")
        if orbit["action_lower"] > orbit["action_upper"] or not (
            orbit["action_lower"] <= orbit["action"] <= orbit["action_upper"]
        ) or orbit["q"] <= 0 or orbit["q_error_bound"] < 0:
            fail(f"production orbit {index} has invalid action/q interval")
        if not math.isclose(orbit["action"], 0.5 / orbit["q"], rel_tol=4.0e-15, abs_tol=0.0):
            fail(f"production orbit {index} action is not derived from q")
        q_upper = orbit["q"] + orbit["q_error_bound"]
        q_lower = orbit["q"] - orbit["q_error_bound"]
        expected_lower = 0.5 / q_upper
        if q_lower <= 0:
            fail(f"production orbit {index} has nonpositive lower q bound")
        expected_upper = 0.5 / q_lower
        if not math.isclose(orbit["action_lower"], expected_lower, rel_tol=4.0e-15, abs_tol=0.0) or not math.isclose(
            orbit["action_upper"], expected_upper, rel_tol=4.0e-15, abs_tol=0.0
        ):
            fail(f"production orbit {index} action interval is not derived from q bounds")
        if orbit["action_lower"] < previous_lower:
            fail("full cache audit orbits are not ordered by action_lower")
        previous_lower = orbit["action_lower"]
        mu = orbit["mu"]
        if mu is not None and (
            not isinstance(mu, list) or len(mu) != 4 or any(not finite_number(value) for value in mu)
        ):
            fail(f"production orbit {index} has invalid mu")
        if orbit["xi"] is not None and not finite_number(orbit["xi"]):
            fail(f"production orbit {index} has invalid xi")
        admissibility = orbit["admissibility"]
        if admissibility not in {"AdmissibleF64", "AdmissibleExact", "IndeterminateF64"}:
            fail(f"production orbit {index} has unknown admissibility")
        if admissibility == "AdmissibleF64" and orbit["beta_margin"] <= 1.0e-9:
            fail(f"production orbit {index} contradicts f64 admissibility classification")
        if admissibility == "IndeterminateF64" and abs(orbit["beta_margin"]) > 1.0e-9:
            fail(f"production orbit {index} contradicts indeterminate classification")
        if admissibility == "AdmissibleExact" and (
            orbit["beta_margin"] <= 0
            or orbit["q_error_bound"] != 0
            or orbit["action_lower"] != orbit["action"]
            or orbit["action_upper"] != orbit["action"]
        ):
            fail(f"production orbit {index} contradicts exact fallback payload contract")
        if admissibility != "IndeterminateF64":
            admissible_actions.append(orbit["action"])
        exact += admissibility == "AdmissibleExact"
        indeterminate += admissibility == "IndeterminateF64"
        lower_values.append(orbit["action_lower"])
        upper_values.append(orbit["action_upper"])
    if not admissible_actions:
        fail("full cache audit has no admissible capacity candidate")
    if (
        result["min_action"] != min(admissible_actions)
        or result["min_action_lower"] != min(lower_values)
        or result["min_action_upper"] != min(upper_values)
    ):
        fail("full cache audit aggregate actions are not derived from orbit payloads")
    if exact != diagnostics["exact_admissible_count"] or indeterminate != diagnostics["indeterminate_count"]:
        fail("full cache audit admissibility counts disagree with compact diagnostics")


def verify(
    directory: Path,
    *,
    expected_reviewed_revision: str | None = None,
    repo_root: Path | None = None,
    cargo_lock: Path | None = None,
    executable: Path | None = None,
) -> dict[str, Any]:
    manifest = read_json(directory / "manifest.json")
    status_path = directory / "run-status.json"
    interrupted = not status_path.exists()
    status = None if interrupted else read_json(status_path)
    if not isinstance(manifest, dict) or (status is not None and not isinstance(status, dict)):
        fail("manifest and run status must be JSON objects")
    if status is not None:
        verify_artifact_hashes(directory, manifest, status)
    else:
        expected_names = {"manifest.json", *JSONL_FILES}
        if (directory / "stop-event.json").exists():
            expected_names.add("stop-event.json")
        actual_names = {path.name for path in directory.iterdir() if path.is_file()}
        if actual_names != expected_names:
            fail("interrupted artifact directory contains missing or unexpected files")
    config, config_id, kind = validate_manifest(
        manifest,
        expected_reviewed_revision=expected_reviewed_revision,
        repo_root=repo_root,
        cargo_lock=cargo_lock,
        executable=executable,
    )
    rows = {name: read_jsonl(directory / name) for name in JSONL_FILES}
    for name, file_rows in rows.items():
        for index, row in enumerate(file_rows, 1):
            require_fields(row, ROW_FIELDS[name], f"{name} row {index}")
    ledger = rows["charged-requests.jsonl"]
    targets = rows["target-evaluations.jsonl"]
    cache_rows = rows["cache.jsonl"]
    rejections = rows["construction-rejections.jsonl"]
    transitions = rows["mutation-transitions.jsonl"]
    levels = rows["levels.jsonl"]
    arm_runs = rows["arm-runs.jsonl"]
    source_revision = manifest["source"]["git_revision"]

    if any(not exact_integer(row["global_request_index"], 1) for row in ledger) or [
        row["global_request_index"] for row in ledger
    ] != list(range(1, len(ledger) + 1)):
        fail("charged-request ledger indices are not contiguous from one")
    if len(ledger) < len(targets) or len(ledger) > len(targets) + (1 if interrupted else 0):
        fail("charged-request ledger does not reconcile to target rows")
    ledger_match_fields = CHARGED_FIELDS - {"charged_monotonic_ms"}
    for index, target in enumerate(targets):
        charge = ledger[index]
        if any(target.get(field) != charge.get(field) for field in ledger_match_fields):
            fail(f"target row {index + 1} does not exactly complete its charged ledger row")
    ledger_times = [row["charged_monotonic_ms"] for row in ledger]
    if any(not finite_number(value) or value < 0 for value in ledger_times) or ledger_times != sorted(ledger_times):
        fail("charged-request ledger lacks ordered cumulative monotonic times")
    ledger_schedule = [request_schedule_token(row["identity"]) for row in ledger]
    expected_schedule = frozen_request_schedule()
    if len(ledger_schedule) > len(expected_schedule) or ledger_schedule != expected_schedule[:len(ledger_schedule)]:
        fail("charged-request ledger does not follow the frozen global schedule")
    ledger_attempts = {"adaptive": [], "iid": []}
    for row in ledger:
        validate_identity(row["identity"], config=config, config_id=config_id, source_revision=source_revision)
        if row["candidate_id"] != expected_candidate_id(row["identity"]):
            fail("charged-request ledger candidate ID mismatch")
        if row["arm"] != row["identity"]["arm"]:
            fail("charged-request ledger arm mismatch")
        if not exact_integer(row["attempt_index"], 1):
            fail("charged-request ledger attempt index has a non-integer JSON type")
        if not exact_integer(row["facet_count"]):
            fail("charged-request ledger facet count has a non-integer JSON type")
        ledger_attempts[row["arm"]].append(row["attempt_index"])
        verify_geometry(row, f"charged ledger row {row['global_request_index']}")
    for arm, indices in ledger_attempts.items():
        if indices != list(range(1, len(indices) + 1)):
            fail(f"{arm} ledger attempt indices are not contiguous from one")

    if [row.get("global_request_index") for row in targets] != list(range(1, len(targets) + 1)):
        fail("global charged request indices are not contiguous from one")
    candidate_rows: dict[str, dict[str, Any]] = {}
    attempts: dict[str, list[int]] = {"adaptive": [], "iid": []}
    seen_success: dict[tuple[str, str], dict[str, Any]] = {}
    status_counts = {arm: {"miss": 0, "hit": 0, "failed_miss": 0} for arm in attempts}
    failures: list[dict[str, Any]] = []
    for row in targets:
        identity = row.get("identity")
        validate_identity(identity, config=config, config_id=config_id, source_revision=source_revision)
        candidate = row.get("candidate_id")
        if candidate != expected_candidate_id(identity):
            fail(f"target candidate ID mismatch: {candidate}")
        if candidate in candidate_rows:
            fail(f"duplicate target candidate identity {candidate}")
        candidate_rows[candidate] = row
        arm = row.get("arm")
        if arm not in attempts or identity["arm"] != arm:
            fail(f"target {candidate} has invalid arm")
        if identity["parent_candidate_id"] != row.get("parent_candidate_id"):
            fail(f"target {candidate} identity does not bind parent")
        attempts[arm].append(row.get("attempt_index"))
        cache_status = row.get("cache_status")
        if cache_status not in status_counts[arm]:
            fail(f"target {candidate} has invalid cache status")
        status_counts[arm][cache_status] += 1
        geometry_volume = verify_geometry(row, f"target {candidate}")
        raw_chart = row.get("raw_proposed_chart")
        if raw_chart is not None:
            validate_chart_shape(raw_chart, f"target {candidate} raw proposal")
        if not finite_number(row.get("wall_time_ms")) or row["wall_time_ms"] < 0:
            fail(f"target {candidate} has invalid wall time")
        if not finite_number(row.get("started_monotonic_ms")) or row["started_monotonic_ms"] < 0:
            fail(f"target {candidate} has invalid monotonic start time")
        if (
            not finite_number(row.get("cumulative_monotonic_ms"))
            or row["cumulative_monotonic_ms"] < ledger[row["global_request_index"] - 1]["charged_monotonic_ms"]
            or ledger[row["global_request_index"] - 1]["charged_monotonic_ms"] < row["started_monotonic_ms"]
            or row["wall_time_ms"] > row["cumulative_monotonic_ms"] - row["started_monotonic_ms"] + 5.0
        ):
            fail(f"target {candidate} has invalid cumulative completion time")
        evaluation_status = row.get("evaluation_status")
        success = evaluation_status == "success"
        scalar_fields = ("capacity", "volume", "sys")
        if success:
            if row.get("failure_reason") is not None:
                fail(f"successful target {candidate} carries a failure reason")
            if not all(finite_positive(row.get(field)) for field in scalar_fields):
                fail(f"successful target {candidate} has invalid scalar payload")
            expected_sys = row["capacity"] ** 2 / (2.0 * row["volume"])
            if not math.isclose(row["sys"], expected_sys, rel_tol=4e-15, abs_tol=0.0):
                fail(f"target {candidate} violates sys = c^2/(2V)")
            if not math.isclose(
                row["volume"], float(geometry_volume), rel_tol=5.0e-13, abs_tol=1.0e-14
            ):
                fail(f"target {candidate} volume disagrees with exact product geometry")
            validate_diagnostics(row, f"target {candidate}")
            if not isinstance(row.get("audit_kind"), str):
                fail(f"target {candidate} lacks audit kind")
        else:
            failures.append(row)
            if evaluation_status not in {"target_unavailable", "invalid_output", "child_failure", "timeout"}:
                fail(f"target {candidate} has unknown failure status")
            if cache_status != "failed_miss" or not isinstance(row.get("failure_reason"), str) or not row["failure_reason"]:
                fail(f"failed target {candidate} lacks retained failure disposition")
            if any(row.get(field) is not None for field in (*scalar_fields, "diagnostics", "audit_kind")):
                fail(f"failed target {candidate} pretends target success")
        arm_key = (arm, row["exact_geometry_key"])
        if cache_status == "hit":
            source = seen_success.get(arm_key)
            if source is None or not success:
                fail(f"target {candidate} cache hit precedes successful arm-private miss")
            for field in (*scalar_fields, "diagnostics", "audit_kind", "geometry_identity"):
                if row.get(field) != source.get(field):
                    fail(f"target {candidate} cache hit payload disagrees with miss")
        elif cache_status == "miss":
            if not success or arm_key in seen_success:
                fail(f"target {candidate} has invalid successful miss")
            seen_success[arm_key] = row
        elif success:
            fail(f"target {candidate} success has failed-miss cache status")

    for arm, indices in attempts.items():
        if any(not exact_integer(index, 1) for index in indices) or indices != list(range(1, len(indices) + 1)):
            fail(f"{arm} charged attempt indices are not contiguous from one")
    target_cumulative = [row["cumulative_monotonic_ms"] for row in targets]
    if target_cumulative != sorted(target_cumulative):
        fail("target rows lack ordered cumulative completion times")
    for index, row in enumerate(targets):
        charge = ledger[index]["charged_monotonic_ms"]
        start = row["started_monotonic_ms"]
        finish = row["cumulative_monotonic_ms"]
        if not (start <= charge <= finish):
            fail("target charge/start/completion interval is reversed")
        if abs(row["wall_time_ms"] - (finish - start)) > MONOTONIC_INTERVAL_TOLERANCE_MS:
            fail("target wall duration does not reconcile with its monotonic interval")
        if index and targets[index - 1]["cumulative_monotonic_ms"] > start:
            fail("target charge/start/completion intervals overlap or are out of sequence")
    if len(ledger) > len(targets) and targets and ledger[-1]["charged_monotonic_ms"] < target_cumulative[-1]:
        fail("unmatched interrupted charge precedes the previous target completion")
    actual_schedule = [request_schedule_token(row["identity"]) for row in targets]
    if len(actual_schedule) > len(expected_schedule) or actual_schedule != expected_schedule[: len(actual_schedule)]:
        fail("charged target rows do not follow the frozen global request schedule")
    positions = {row["candidate_id"]: index for index, row in enumerate(targets)}
    for index, row in enumerate(targets):
        parent = row.get("parent_candidate_id")
        if parent is not None and (parent not in positions or positions[parent] >= index):
            fail(f"mutation candidate {row['candidate_id']} does not follow its parent")

    caches: dict[tuple[str, str], dict[str, Any]] = {}
    expected_cache_order = [
        (row["arm"], row["exact_geometry_key"])
        for row in targets
        if row["cache_status"] == "miss"
    ]
    if [(row["arm"], row["exact_geometry_key"]) for row in cache_rows] != expected_cache_order:
        fail("cache JSONL order is not the successful-miss target order")
    for row in cache_rows:
        if not exact_integer(row.get("facet_count")):
            fail("cache row facet count has a non-integer JSON type")
        arm_key = (row.get("arm"), row.get("exact_geometry_key"))
        if arm_key in caches or arm_key not in seen_success:
            fail(f"duplicate or orphan cache row {arm_key}")
        geometry_volume = verify_geometry(row, f"cache row {arm_key}")
        if not math.isclose(
            row.get("volume"), float(geometry_volume), rel_tol=5.0e-13, abs_tol=1.0e-14
        ):
            fail(f"cache row {arm_key} volume disagrees with exact product geometry")
        verify_full_cache_audit(row, kind)
        source = seen_success[arm_key]
        for field in (
            "geometry_identity", "dual_vertices_rational", "dual_vertices_f64", "facet_count", "product_chart",
            "capacity", "volume", "sys", "diagnostics", "audit_kind",
        ):
            if row.get(field) != source.get(field):
                fail(f"cache/target disagreement for {arm_key} field {field}")
        caches[arm_key] = row
    if set(caches) != set(seen_success):
        fail("cache rows are not exactly successful arm-private misses")

    rejection_by_group: dict[tuple[Any, ...], list[dict[str, Any]]] = {}
    rejected_candidates: set[str] = set()
    rejection_counts = {"adaptive": 0, "iid": 0}
    for row in rejections:
        identity = row.get("identity")
        validate_identity(identity, config=config, config_id=config_id, source_revision=source_revision)
        candidate = row.get("candidate_id")
        if candidate != expected_candidate_id(identity) or candidate in candidate_rows or candidate in rejected_candidates:
            fail(f"construction rejection candidate identity mismatch: {candidate}")
        rejected_candidates.add(candidate)
        arm = row.get("arm")
        if arm != identity["arm"] or arm not in rejection_counts:
            fail(f"construction rejection {candidate} has invalid arm")
        rejection_counts[arm] += 1
        if not isinstance(row.get("reason"), str) or not row["reason"]:
            fail(f"construction rejection {candidate} lacks reason")
        if row.get("parent_candidate_id") != identity["parent_candidate_id"]:
            fail(f"construction rejection {candidate} parent mismatch")
        if identity["level"] is None:
            if row.get("root_candidate_id") is not None or row.get("raw_proposed_chart") is not None:
                fail(f"base rejection {candidate} has mutation-only fields")
        else:
            if not isinstance(row.get("root_candidate_id"), str):
                fail(f"mutation rejection {candidate} lacks root")
            validate_chart_shape(row.get("raw_proposed_chart"), f"rejection {candidate} raw proposal")
        rejection_by_group.setdefault(logical_identity(identity), []).append(row)

    schedule_positions = {token: index for index, token in enumerate(frozen_request_schedule())}
    rejection_positions = [schedule_positions.get(request_schedule_token(row["identity"])) for row in rejections]
    if any(position is None for position in rejection_positions) or rejection_positions != sorted(rejection_positions):
        fail("construction rejection JSONL order does not follow the frozen schedule")

    for candidate, row in candidate_rows.items():
        identity = row["identity"]
        prior = [
            entry["identity"]["construction_attempt"]
            for entry in rejection_by_group.pop(logical_identity(identity), [])
        ]
        if prior != list(range(identity["construction_attempt"])):
            fail(f"candidate {candidate} lacks exact contiguous construction retry history")
    unmatched_rejection_groups = rejection_by_group

    initial_adaptive: dict[str, dict[str, Any]] = {}
    for candidate, row in candidate_rows.items():
        identity = row["identity"]
        parent = row.get("parent_candidate_id")
        root = row.get("root_candidate_id")
        if identity["level"] is None:
            if parent is not None or root != candidate or row.get("level_threshold") is not None or row.get("raw_proposed_chart") is not None:
                fail(f"base candidate {candidate} has invalid root/parent/threshold")
            if row["arm"] == "adaptive":
                initial_adaptive[candidate] = row
        else:
            if parent not in candidate_rows or root not in initial_adaptive:
                fail(f"mutation candidate {candidate} has broken genealogy")
            if candidate_rows[parent].get("root_candidate_id") != root:
                fail(f"mutation candidate {candidate} breaks root transitivity")

    if any(not exact_integer(row.get("level")) for row in levels) or [
        row.get("level") for row in levels
    ] != list(range(len(levels))) or len(levels) > 2:
        fail("completed level records are not a zero-based prefix")
    transition_keys: set[tuple[int, int, int]] = set()
    transitions_by_key: dict[tuple[int, int, int], dict[str, Any]] = {}
    mutation_targets = [row for row in targets if row["identity"]["level"] is not None]
    mutation_target_keys = [
        (row["identity"]["level"], row["identity"]["clone_index"], row["identity"]["mutation_step"])
        for row in mutation_targets
    ]
    mutation_by_key = dict(zip(mutation_target_keys, mutation_targets))
    for row in transitions:
        key = (row.get("level"), row.get("clone_index"), row.get("mutation_step"))
        if key in transition_keys or not all(exact_integer(value) for value in key):
            fail(f"duplicate or invalid mutation transition {key}")
        level, clone, step = key
        if not (0 <= level < 2 and 0 <= clone < 8 and 0 <= step < 2):
            fail(f"mutation transition index outside frozen grid {key}")
        if not isinstance(row.get("accepted"), bool):
            fail(f"mutation transition {key} acceptance has a non-Boolean JSON type")
        transition_keys.add(key)
        transitions_by_key[key] = row
    transition_file_keys = [
        (row["level"], row["clone_index"], row["mutation_step"]) for row in transitions
    ]
    if transition_file_keys != mutation_target_keys[:len(transition_file_keys)]:
        fail("mutation transition file order is not the charged mutation-target prefix")
    if len(transitions) > len(mutation_targets) or (
        not interrupted and len(transitions) != len(mutation_targets)
    ) or (interrupted and len(mutation_targets) - len(transitions) > 1):
        fail("mutation transitions do not exactly reconcile to the target prefix")

    population = sorted(
        initial_adaptive,
        key=lambda candidate: initial_adaptive[candidate]["identity"]["base_index"],
    )
    if population and [initial_adaptive[c]["identity"]["base_index"] for c in population] != list(range(len(population))):
        fail("adaptive base indices are not a zero-based prefix")
    completed_populations: list[list[str]] = []
    diversity_failures: list[dict[str, Any]] = []
    expected_state_by_mutation_key: dict[tuple[int, int, int], str] = {}
    for level in range(2):
        if len(population) < 16:
            break
        survivors = sorted(population, key=lambda candidate: (-candidate_rows[candidate]["sys"], candidate))[:8]
        threshold = candidate_rows[survivors[-1]]["sys"]
        clone_states: dict[int, str] = {}
        for clone in range(8):
            parent = expected_clone_parent(config_id, config["master_seed"], level, clone, survivors)
            state = parent
            clone_complete = True
            for step in range(2):
                expected_state_by_mutation_key[(level, clone, step)] = state
                proposal_row = mutation_by_key.get((level, clone, step))
                transition = transitions_by_key.get((level, clone, step))
                if proposal_row is None:
                    clone_complete = False
                    break
                proposal = proposal_row["candidate_id"]
                identity = proposal_row["identity"]
                if (identity["level"], identity["clone_index"], identity["mutation_step"]) != (level, clone, step):
                    fail(f"transition {(level, clone, step)} disagrees with proposal identity")
                if identity["parent_candidate_id"] != state or proposal_row["parent_candidate_id"] != state:
                    fail(f"transition {(level, clone, step)} proposal parent is not state-before")
                if proposal_row.get("level_threshold") != threshold:
                    fail(f"transition {(level, clone, step)} changes frozen target threshold")
                if proposal_row["root_candidate_id"] != candidate_rows[state]["root_candidate_id"]:
                    fail(f"transition {(level, clone, step)} breaks root transitivity")
                expected_raw = expected_raw_mutation(config, candidate_rows[state]["product_chart"], identity)
                charts_close(proposal_row.get("raw_proposed_chart"), expected_raw, RAW_CHART_TOLERANCE, f"proposal {proposal} raw mutation")
                verify_mutation_chart_geometry(proposal_row, f"proposal {proposal}")
                proposal_sys = proposal_row.get("sys")
                accepted = proposal_sys is not None and proposal_sys >= threshold
                after = proposal if accepted else state
                if transition is not None:
                    if transition.get("proposal_candidate_id") != proposal:
                        fail(f"transition {(level, clone, step)} lacks its charged proposal target")
                    if transition.get("state_before_candidate_id") != state:
                        fail(f"transition {(level, clone, step)} state chain is broken")
                    if transition.get("frozen_threshold") != threshold:
                        fail(f"transition {(level, clone, step)} changes frozen target threshold")
                    if transition.get("proposal_sys") != proposal_sys:
                        fail(f"transition {(level, clone, step)} proposal scalar mismatch")
                    if transition.get("accepted") != accepted:
                        fail(f"transition {(level, clone, step)} acceptance rule mismatch")
                    if transition.get("state_after_candidate_id") != after:
                        fail(f"transition {(level, clone, step)} next state mismatch")
                    if transition.get("root_candidate_id") != proposal_row["root_candidate_id"]:
                        fail(f"transition {(level, clone, step)} root mismatch")
                state = after
                if proposal_row.get("evaluation_status") != "success" or (
                    proposal_sys is not None and proposal_sys > 1.0
                ):
                    clone_complete = False
                    break
            if clone_complete:
                clone_states[clone] = state
        if len(clone_states) != 8:
            break
        population = survivors + [clone_states[clone] for clone in range(8)]
        completed_populations.append(population)
        if level >= len(levels):
            fail(f"completed transition grid for level {level} lacks population record")
        row = levels[level]
        if not exact_integer(row.get("post_level_distinct_geometry_keys")):
            fail(f"completed level {level} distinct count has a non-integer JSON type")
        expected_parents = [
            expected_clone_parent(config_id, config["master_seed"], level, clone, survivors)
            for clone in range(8)
        ]
        expected_roots = [candidate_rows[candidate]["root_candidate_id"] for candidate in survivors]
        keys = [candidate_rows[candidate]["exact_geometry_key"] for candidate in population]
        if (
            row.get("frozen_threshold") != threshold
            or row.get("survivor_candidate_ids") != survivors
            or row.get("survivor_root_candidate_ids") != expected_roots
            or row.get("clone_parent_candidate_ids") != expected_parents
            or row.get("post_level_population_candidate_ids") != population
            or row.get("post_level_population_geometry_keys") != keys
            or row.get("post_level_distinct_geometry_keys") != len(set(keys))
        ):
            fail(f"completed level {level} population/assignment evidence mismatch")
        if len(set(keys)) < 8:
            diversity_failures.append(row)
        if len(set(expected_roots)) < 2:
            fail(f"readiness gate requires two survivor roots at level {level}")
    if len(levels) != len(completed_populations):
        fail("level records do not equal completed rejuvenation populations")

    for row in rejections:
        identity = row["identity"]
        if identity["level"] is not None:
            parent = identity["parent_candidate_id"]
            if parent not in candidate_rows or row["root_candidate_id"] != candidate_rows[parent]["root_candidate_id"]:
                fail(f"mutation rejection {row['candidate_id']} has wrong parent/root")
            expected_raw = expected_raw_mutation(config, candidate_rows[parent]["product_chart"], identity)
            charts_close(row["raw_proposed_chart"], expected_raw, RAW_CHART_TOLERANCE, f"rejection {row['candidate_id']} raw mutation")

    stop_path = directory / "stop-event.json"
    stop = read_json(stop_path) if stop_path.exists() else None
    sys_hits = [row for row in targets if row.get("sys") is not None and row["sys"] > 1.0]
    if stop is None:
        if sys_hits and not interrupted:
            fail("target sys > 1 exists without a stop event")
        if interrupted and (len(sys_hits) > 1 or (sys_hits and sys_hits[0] is not targets[-1])):
            fail("interrupted target prefix has nonterminal sys > 1 evidence")
    else:
        if not isinstance(stop, dict) or len(sys_hits) != 1:
            fail("stop event requires exactly one sys > 1 target")
        hit = sys_hits[0]
        if hit is not targets[-1]:
            fail("a charged request follows the sys > 1 target")
        expected = {
            "event": "sys_gt_one_flush_and_stop",
            "global_request_index": hit["global_request_index"],
            "arm": hit["arm"],
            "candidate_id": hit["candidate_id"],
            "exact_geometry_key": hit["exact_geometry_key"],
            "sys": hit["sys"],
            "action": STOP_ACTION,
        }
        if stop != expected:
            fail("stop event does not exactly match the final sys > 1 target")
        if not exact_integer(stop.get("global_request_index"), 1):
            fail("stop event request index has a non-integer JSON type")

    terminal = None
    if interrupted:
        disposition = "externally_interrupted"
        if unmatched_rejection_groups:
            fail("interrupted prefix has unaudited unmatched construction rejections")
    else:
        assert status is not None
        disposition = status.get("disposition")
        if disposition not in {"complete", "sys_gt_one_stop", "timeout", "error"}:
            fail("final status has unknown disposition")
        if (disposition == "sys_gt_one_stop") != (stop is not None):
            fail("final status and stop-event disposition disagree")
        if disposition == "complete" and (stop is not None or failures):
            fail("complete status contains a stop or failed target")
        if disposition in {"timeout", "error"} and (
            not isinstance(status.get("error"), str) or not status["error"]
        ):
            fail("failed final status lacks retained error")
        if disposition in {"complete", "sys_gt_one_stop"} and status.get("error") is not None:
            fail("successful/stopped final status carries an error")
        terminal = status.get("terminal_error")
        if disposition in {"complete", "sys_gt_one_stop"}:
            if terminal is not None:
                fail("successful/stopped final status carries terminal-error evidence")
            if failures:
                fail("successful/stopped status contains a failed target")
            if unmatched_rejection_groups:
                fail("successful/stopped status has unmatched construction rejections")
        else:
            require_fields(terminal, TERMINAL_ERROR_FIELDS, "terminal error evidence")
            if terminal["global_request_index"] is not None and not exact_integer(
                terminal["global_request_index"], 1
            ):
                fail("terminal error request index has a non-integer JSON type")
            for field in (
                "level", "observed_distinct_geometry_keys", "required_distinct_geometry_keys"
            ):
                if terminal[field] is not None and not exact_integer(terminal[field]):
                    fail(f"terminal error {field} has a non-integer JSON type")
            kind_name = terminal["kind"]
            diversity_fields = (
                terminal["level"], terminal["observed_distinct_geometry_keys"],
                terminal["required_distinct_geometry_keys"],
            )
            if kind_name == "failed_target":
                if not status["error"].startswith("failed_target:"):
                    fail("failed-target status lacks its structured error prefix")
                if unmatched_rejection_groups or len(failures) != 1 or failures[-1] is not targets[-1]:
                    fail("failed-target terminal evidence lacks one final failed target")
                final = failures[-1]
                if (
                    terminal["arm"] != final["arm"]
                    or terminal["global_request_index"] != final["global_request_index"]
                    or terminal["candidate_id"] != final["candidate_id"]
                    or terminal["evaluation_status"] != final["evaluation_status"]
                    or terminal["failure_reason"] != final["failure_reason"]
                    or terminal["next_schedule_identity"] is not None
                    or any(value is not None for value in diversity_fields)
                ):
                    fail("failed-target terminal evidence disagrees with final target")
                if (disposition == "timeout") != (final["evaluation_status"] == "timeout"):
                    fail("timeout disposition disagrees with failed-target kind")
            elif kind_name == "construction_exhaustion":
                if not status["error"].startswith("construction_exhaustion:"):
                    fail("construction-exhaustion status lacks its structured error prefix")
                if disposition != "error" or failures or len(unmatched_rejection_groups) != 1:
                    fail("construction-exhaustion status has wrong terminal evidence")
                group = next(iter(unmatched_rejection_groups.values()))
                attempts_in_group = [row["identity"]["construction_attempt"] for row in group]
                if attempts_in_group != list(range(64)):
                    fail("construction exhaustion is not exactly 64 ordered rejections")
                first_identity = copy_identity_with_attempt(group[0]["identity"], 0)
                next_token_index = len(targets)
                mutation_key = (
                    first_identity["level"], first_identity["clone_index"], first_identity["mutation_step"]
                )
                if (
                    next_token_index >= len(expected_schedule)
                    or request_schedule_token(first_identity) != expected_schedule[next_token_index]
                    or terminal["next_schedule_identity"] != first_identity
                    or terminal["arm"] != first_identity["arm"]
                    or terminal["global_request_index"] is not None
                    or terminal["candidate_id"] is not None
                    or terminal["evaluation_status"] is not None
                    or terminal["failure_reason"] is not None
                    or any(value is not None for value in diversity_fields)
                    or (
                        first_identity["level"] is not None
                        and first_identity["parent_candidate_id"]
                        != expected_state_by_mutation_key.get(mutation_key)
                    )
                ):
                    fail("construction exhaustion does not evidence the exact next schedule slot")
            elif kind_name == "wall_termination":
                if not status["error"].startswith("wall_termination:"):
                    fail("wall-termination status lacks its structured error prefix")
                if disposition != "error" or unmatched_rejection_groups or failures:
                    fail("wall-termination status has incompatible terminal evidence")
                if (
                    terminal["next_schedule_identity"] is not None
                    or terminal["evaluation_status"] is not None
                    or terminal["failure_reason"] is not None
                    or any(value is not None for value in diversity_fields)
                    or terminal["global_request_index"] != (targets[-1]["global_request_index"] if targets else None)
                    or terminal["candidate_id"] != (targets[-1]["candidate_id"] if targets else None)
                    or terminal["arm"] not in {"adaptive", "iid"}
                ):
                    fail("wall-termination evidence disagrees with completed prefix")
                if not arm_runs or arm_runs[-1]["arm"] != terminal["arm"]:
                    fail("wall-termination evidence lacks its active incomplete arm row")
            elif kind_name == "post_level_diversity_gate":
                if not status["error"].startswith("post_level_diversity_gate:"):
                    fail("diversity-gate status lacks its structured error prefix")
                if (
                    disposition != "error"
                    or failures
                    or unmatched_rejection_groups
                    or len(diversity_failures) != 1
                    or diversity_failures[0] is not levels[-1]
                ):
                    fail("diversity-gate status has incompatible terminal evidence")
                verify_diversity_terminal_payload(terminal, levels, targets)
            else:
                fail("terminal error evidence has unknown kind")

    if diversity_failures and (
        interrupted
        or terminal is None
        or terminal.get("kind") != "post_level_diversity_gate"
    ):
        fail("low-diversity completed level lacks matching terminal evidence")

    charged_counts = {arm: len(indices) for arm, indices in attempts.items()}
    if status is not None:
        if any(
            not exact_integer(status.get(field))
            for field in (
                "adaptive_charged_requests", "iid_charged_requests", "total_charged_requests"
            )
        ):
            fail("final status charged counts have non-integer JSON types")
        if (
            status.get("adaptive_charged_requests") != charged_counts["adaptive"]
            or status.get("iid_charged_requests") != charged_counts["iid"]
            or status.get("total_charged_requests") != len(targets)
        ):
            fail("final status charged counts disagree with target rows")
        total_ms = status.get("total_monotonic_wall_time_ms")
        if not finite_number(total_ms) or total_ms < 0:
            fail("final status has invalid monotonic wall time")
        end_ms = status.get("end_unix_ms")
        if not exact_integer(end_ms, 1) or end_ms < manifest["start_unix_ms"]:
            fail("run status has invalid end wall-clock timestamp")
        wall_elapsed = end_ms - manifest["start_unix_ms"]
        if abs(wall_elapsed - total_ms) > WALL_CLOCK_RECONCILIATION_TOLERANCE_MS:
            fail("wall-clock and monotonic run durations do not reconcile")
        if target_cumulative and target_cumulative[-1] > total_ms:
            fail("target cumulative completion exceeds total monotonic run time")
        if ledger_times and ledger_times[-1] > total_ms:
            fail("charged cumulative time exceeds total monotonic run time")
        if sum(row["wall_time_ms"] for row in targets) > total_ms + 20.0:
            fail("target row wall times exceed total monotonic run time")
    else:
        total_ms = target_cumulative[-1] if target_cumulative else (ledger_times[-1] if ledger_times else 0.0)

    runs_by_arm: dict[str, dict[str, Any]] = {}
    for row in arm_runs:
        arm = row.get("arm")
        if arm in runs_by_arm or arm not in attempts:
            fail("duplicate or invalid arm-run row")
        runs_by_arm[arm] = row
        for field in (
            "target_attempts", "construction_rejections", "cache_misses", "cache_hits",
            "failed_misses", "distinct_successful_keys",
        ):
            if not exact_integer(row.get(field)):
                fail(f"{arm} arm-run {field} has a non-integer JSON type")
        if not isinstance(row.get("complete"), bool):
            fail(f"{arm} arm-run completion has a non-Boolean JSON type")
        if row.get("target_attempts") != charged_counts[arm]:
            fail(f"{arm} arm-run target count mismatch")
        if row.get("construction_rejections") != rejection_counts[arm]:
            fail(f"{arm} arm-run construction count mismatch")
        if row.get("cache_misses") != status_counts[arm]["miss"]:
            fail(f"{arm} arm-run miss count mismatch")
        if row.get("cache_hits") != status_counts[arm]["hit"]:
            fail(f"{arm} arm-run hit count mismatch")
        if row.get("failed_misses") != status_counts[arm]["failed_miss"]:
            fail(f"{arm} arm-run failure count mismatch")
        distinct = len({key for row_arm, key in seen_success if row_arm == arm})
        if row.get("distinct_successful_keys") != distinct:
            fail(f"{arm} arm-run distinct key count mismatch")
        if not finite_number(row.get("wall_time_ms")) or row["wall_time_ms"] < 0:
            fail(f"{arm} arm-run wall time invalid")
        if not finite_number(row.get("started_monotonic_ms")) or row["started_monotonic_ms"] < 0:
            fail(f"{arm} arm-run monotonic start invalid")
        if not finite_number(row.get("cumulative_monotonic_ms")) or row["cumulative_monotonic_ms"] < 0:
            fail(f"{arm} arm-run cumulative time invalid")
        if abs(
            row["wall_time_ms"]
            - (row["cumulative_monotonic_ms"] - row["started_monotonic_ms"])
        ) > MONOTONIC_INTERVAL_TOLERANCE_MS:
            fail(f"{arm} arm-run duration does not reconcile with its cumulative interval")
        arm_target_ms = sum(target["wall_time_ms"] for target in targets if target["arm"] == arm)
        if arm_target_ms > row["wall_time_ms"] + 20.0:
            fail(f"{arm} target times exceed arm-run time")
    if sum(row["wall_time_ms"] for row in arm_runs) > total_ms + 20.0:
        fail("arm-run wall times exceed total monotonic run time")
    if [row["arm"] for row in arm_runs] != ["adaptive", "iid"][:len(arm_runs)]:
        fail("arm-run rows are not in exact phase order")
    arm_cumulative = [row["cumulative_monotonic_ms"] for row in arm_runs]
    if arm_cumulative != sorted(arm_cumulative) or (status is not None and arm_cumulative and arm_cumulative[-1] > total_ms):
        fail("arm-run cumulative endpoints are not ordered within total time")
    for index, row in enumerate(arm_runs):
        if index and arm_runs[index - 1]["cumulative_monotonic_ms"] > row["started_monotonic_ms"]:
            fail("arm-run monotonic intervals overlap")
        arm_targets = [target for target in targets if target["arm"] == row["arm"]]
        if any(
            target["started_monotonic_ms"] < row["started_monotonic_ms"]
            or target["cumulative_monotonic_ms"] > row["cumulative_monotonic_ms"]
            for target in arm_targets
        ):
            fail(f"{row['arm']} target interval lies outside its arm interval")

    if not interrupted:
        if disposition == "complete":
            expected_arm_runs = [("adaptive", True), ("iid", True)]
        else:
            if stop is not None:
                active_arm = stop["arm"]
            else:
                assert terminal is not None
                active_arm = terminal["arm"]
            expected_arm_runs = (
                [("adaptive", False)]
                if active_arm == "adaptive"
                else [("adaptive", True), ("iid", False)]
            )
        if [(row["arm"], row["complete"]) for row in arm_runs] != expected_arm_runs:
            fail("arm-run rows do not exactly match the terminal phase")
    else:
        if any(row["complete"] is not True for row in arm_runs[:-1]):
            fail("interrupted arm-run prefix has an incomplete closed phase")
        if len(arm_runs) == 2 and arm_runs[0]["complete"] is not True:
            fail("interrupted IID phase lacks completed adaptive arm")

    deadline_ms = config["abort_wall_time_seconds"] * 1_000.0
    if not interrupted and terminal is not None and terminal["kind"] == "wall_termination":
        active_endpoint = arm_runs[-1]["cumulative_monotonic_ms"]
        if abs(active_endpoint - deadline_ms) > DEADLINE_RECONCILIATION_TOLERANCE_MS:
            fail("wall-termination arm endpoint does not reconcile with the frozen deadline")
        if total_ms < active_endpoint or total_ms - active_endpoint > DEADLINE_RECONCILIATION_TOLERANCE_MS:
            fail("wall-termination finalization does not reconcile with its arm endpoint")
    if (
        not interrupted
        and terminal is not None
        and terminal["kind"] == "failed_target"
        and terminal["evaluation_status"] == "timeout"
    ):
        final_timeout_ms = targets[-1]["cumulative_monotonic_ms"]
        if final_timeout_ms > deadline_ms + DEADLINE_RECONCILIATION_TOLERANCE_MS:
            fail("timeout completion exceeds the frozen deadline tolerance")
        if kind == "production_target" and abs(final_timeout_ms - deadline_ms) > DEADLINE_RECONCILIATION_TOLERANCE_MS:
            fail("production timeout does not reconcile with the frozen global deadline")

    complete = disposition == "complete"
    readiness_passed = complete and kind == "production_target"
    if complete:
        if len(targets) != 64 or attempts != {"adaptive": list(range(1, 49)), "iid": list(range(1, 17))}:
            fail("complete smoke does not close the exact 48/16 charged budgets")
        if len(transitions) != 32 or transition_keys != {
            (level, clone, step) for level in range(2) for clone in range(8) for step in range(2)
        }:
            fail("complete smoke does not contain the exact mutation index grid")
        if len(levels) != 2:
            fail("complete smoke lacks two completed post-level populations")
        base_grid = {
            arm: sorted(
                row["identity"]["base_index"]
                for row in targets
                if row["arm"] == arm and row["identity"]["level"] is None
            )
            for arm in attempts
        }
        if base_grid != {"adaptive": list(range(16)), "iid": list(range(16))}:
            fail("complete smoke changes the exact base index grids")
        if not any(row.get("accepted") for row in transitions):
            fail("readiness gate requires at least one accepted valid mutation")
        if set(runs_by_arm) != {"adaptive", "iid"} or not all(row.get("complete") is True for row in arm_runs):
            fail("complete smoke lacks two complete arm-run records")
        if total_ms > config["abort_wall_time_seconds"] * 1000:
            fail("complete smoke exceeds the frozen 900-second envelope")
    return {
        "verified": True,
        "readiness_passed": readiness_passed,
        "artifact_kind": kind,
        "disposition": disposition,
        "adaptive_attempts": charged_counts["adaptive"],
        "iid_attempts": charged_counts["iid"],
        "ledger_charged_requests": len(ledger),
        "outcome_unknown_requests": len(ledger) - len(targets),
        "construction_rejections": len(rejections),
        "post_level_distinct_states": [row["post_level_distinct_geometry_keys"] for row in levels],
        "stopped_on_sys_gt_one": stop is not None,
        "tail_probability_supported": False,
        "probability_estimate": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifacts", type=Path)
    parser.add_argument("--expected-reviewed-revision")
    parser.add_argument("--repo-root", type=Path)
    parser.add_argument("--cargo-lock", type=Path)
    parser.add_argument("--executable", type=Path)
    args = parser.parse_args()
    try:
        result = verify(
            args.artifacts,
            expected_reviewed_revision=args.expected_reviewed_revision,
            repo_root=args.repo_root,
            cargo_lock=args.cargo_lock,
            executable=args.executable,
        )
    except ArtifactError as error:
        raise SystemExit(f"artifact verification failed: {error}") from error
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
