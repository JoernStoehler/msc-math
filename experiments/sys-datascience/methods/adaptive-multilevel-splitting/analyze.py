#!/usr/bin/env python3
"""Fail-closed verifier for AMS readiness-smoke artifacts.

This verifies accounting, provenance links, genealogy, and state transitions.
It deliberately does not estimate a tail probability or compare arm quality.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Any


class ArtifactError(RuntimeError):
    pass


JSONL_FILES = (
    "target-evaluations.jsonl",
    "cache.jsonl",
    "construction-rejections.jsonl",
    "mutation-transitions.jsonl",
    "levels.jsonl",
    "arm-runs.jsonl",
)


def fail(message: str) -> None:
    raise ArtifactError(message)


def read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
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
            row = json.loads(line)
        except json.JSONDecodeError as error:
            fail(f"invalid JSON in {path}:{number}: {error}")
        if not isinstance(row, dict):
            fail(f"non-object JSONL row in {path}:{number}")
        rows.append(row)
    return rows


def compact_json(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode()


def sha256(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def option(value: Any) -> str:
    return "none" if value is None else str(value)


def expected_candidate_id(identity: dict[str, Any]) -> str:
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


def exact_key(vertices: list[list[str]]) -> str:
    return "|".join(",".join(row) for row in vertices)


def finite_positive(value: Any) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(value) and value > 0


def verify(directory: Path) -> dict[str, Any]:
    manifest = read_json(directory / "manifest.json")
    rows = {name: read_jsonl(directory / name) for name in JSONL_FILES}
    config = manifest.get("exact_config")
    if not isinstance(config, dict):
        fail("manifest exact_config is missing")
    config_id = sha256(compact_json(config))
    if manifest.get("config_identity") != config_id:
        fail("manifest config_identity does not bind exact_config")
    if manifest.get("adaptive_budget") != 48 or manifest.get("iid_budget") != 16:
        fail("manifest changes the fixed 48/16 charged budgets")
    if manifest.get("target_probability_estimate") is not None:
        fail("a readiness smoke must not contain a probability estimate")
    if manifest.get("factor_exchange_quotiented") is not False:
        fail("factor exchange must remain visibly unquotiented")
    if config.get("factor_exchange_quotiented") is not False:
        fail("config claims factor exchange is quotiented")
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
        "abort_wall_time_seconds": 600,
        "gap_logit_scale": 0.08,
        "centered_log_radius_scale": 0.04,
        "phase_scale": 0.08,
        "tie_rule": "sys_desc_candidate_id_asc",
        "clone_assignment": "seeded_uniform_with_replacement",
        "acceptance_rule": "successful_sys_at_least_frozen_level_threshold",
    }
    for field, expected in fixed_fields.items():
        if config.get(field) != expected:
            fail(f"config changes frozen field {field}")
    kind = manifest.get("artifact_kind")
    if kind not in {"synthetic_target_free", "production_target"}:
        fail("unknown artifact_kind")
    source = manifest.get("source", {})
    if source.get("production_target") != (kind == "production_target"):
        fail("source production_target disagrees with artifact_kind")
    if kind == "production_target" and source.get("source_tree_clean") is not True:
        fail("production artifacts came from a dirty source tree")
    for field in ("executable_sha256", "cargo_lock_sha256"):
        value = source.get(field)
        if not isinstance(value, str) or len(value) != 64:
            fail(f"invalid source identity field {field}")

    targets = rows["target-evaluations.jsonl"]
    cache_rows = rows["cache.jsonl"]
    rejections = rows["construction-rejections.jsonl"]
    transitions = rows["mutation-transitions.jsonl"]
    levels = rows["levels.jsonl"]
    arm_runs = rows["arm-runs.jsonl"]

    candidate_rows: dict[str, dict[str, Any]] = {}
    attempts: dict[str, list[int]] = {"adaptive": [], "iid": []}
    successes_by_arm_key: dict[tuple[str, str], dict[str, Any]] = {}
    seen_success_keys: set[tuple[str, str]] = set()
    status_counts = {
        arm: {"miss": 0, "hit": 0, "failed_miss": 0} for arm in attempts
    }
    for row in targets:
        identity = row.get("identity")
        if not isinstance(identity, dict):
            fail("target row lacks candidate identity")
        candidate = row.get("candidate_id")
        if candidate != expected_candidate_id(identity):
            fail(f"candidate ID mismatch for target {candidate}")
        if identity.get("config_identity") != config_id:
            fail(f"target {candidate} has another config identity")
        if identity.get("source_revision") != source.get("git_revision"):
            fail(f"target {candidate} has another source revision")
        if identity.get("parent_candidate_id") != row.get("parent_candidate_id"):
            fail(f"target {candidate} identity does not bind its parent")
        if candidate in candidate_rows:
            fail(f"duplicate target candidate identity {candidate}")
        candidate_rows[candidate] = row
        arm = row.get("arm")
        if arm not in attempts or identity.get("arm") != arm:
            fail(f"invalid arm on target {candidate}")
        attempts[arm].append(row.get("attempt_index"))
        key = row.get("exact_geometry_key")
        geometry_id = row.get("geometry_identity")
        if not isinstance(key, str) or not isinstance(geometry_id, str):
            fail(f"target {candidate} lacks geometry identity")
        cache_status = row.get("cache_status")
        if cache_status not in status_counts[arm]:
            fail(f"target {candidate} has invalid cache status")
        status_counts[arm][cache_status] += 1
        success = row.get("evaluation_status") == "success"
        if success != (row.get("sys") is not None):
            fail(f"target {candidate} success/payload disagreement")
        arm_key = (arm, key)
        if cache_status == "hit" and arm_key not in seen_success_keys:
            fail(f"target {candidate} cache hit precedes successful miss")
        if cache_status == "miss":
            if not success or arm_key in seen_success_keys:
                fail(f"target {candidate} invalid successful miss")
            seen_success_keys.add(arm_key)
        elif cache_status == "failed_miss" and success:
            fail(f"target {candidate} failed_miss carries success")
        if success:
            for field in ("capacity", "volume", "sys"):
                if field == "sys":
                    if not isinstance(row[field], (int, float)) or not math.isfinite(row[field]):
                        fail(f"target {candidate} has nonfinite sys")
                elif not finite_positive(row[field]):
                    fail(f"target {candidate} has invalid {field}")
            expected_sys = row["capacity"] * row["capacity"] / (2.0 * row["volume"])
            if not math.isclose(row["sys"], expected_sys, rel_tol=4e-15, abs_tol=0.0):
                fail(f"target {candidate} sys disagrees with capacity and volume")
            successes_by_arm_key[arm_key] = row
        if not isinstance(row.get("product_chart"), dict):
            fail(f"target {candidate} does not retain the product chart")
        if not isinstance(row.get("wall_time_ms"), (int, float)) or row["wall_time_ms"] < 0:
            fail(f"target {candidate} has invalid wall time")

    for arm, indices in attempts.items():
        if indices != list(range(1, len(indices) + 1)):
            fail(f"{arm} charged attempt indices are not contiguous from one")

    caches: dict[tuple[str, str], dict[str, Any]] = {}
    for row in cache_rows:
        arm_key = (row.get("arm"), row.get("exact_geometry_key"))
        if arm_key in caches:
            fail(f"duplicate cache row {arm_key}")
        vertices = row.get("dual_vertices_rational")
        if not isinstance(vertices, list) or len(vertices) != 10:
            fail(f"cache row {arm_key} lacks ten exact dual vertices")
        if exact_key(vertices) != arm_key[1]:
            fail(f"cache row {arm_key} exact key disagrees with geometry")
        if sha256(compact_json(vertices)) != row.get("geometry_identity"):
            fail(f"cache row {arm_key} geometry identity mismatch")
        if row.get("facet_count") != 10:
            fail(f"cache row {arm_key} is outside the fixed 5 x 5 chart")
        source_target = successes_by_arm_key.get(arm_key)
        if source_target is None:
            fail(f"orphan cache row {arm_key}")
        for field in ("capacity", "volume", "sys", "geometry_identity"):
            if row.get(field) != source_target.get(field):
                fail(f"cache/target disagreement for {arm_key} field {field}")
        caches[arm_key] = row
    miss_keys = {
        (row["arm"], row["exact_geometry_key"])
        for row in targets
        if row["cache_status"] == "miss"
    }
    if set(caches) != miss_keys:
        fail("cache rows are not exactly the successful misses")

    rejection_counts = {"adaptive": 0, "iid": 0}
    rejected_candidates: set[str] = set()
    for row in rejections:
        identity = row.get("identity")
        candidate = row.get("candidate_id")
        if not isinstance(identity, dict) or candidate != expected_candidate_id(identity):
            fail(f"construction rejection candidate identity mismatch: {candidate}")
        if identity.get("config_identity") != config_id:
            fail(f"construction rejection {candidate} has another config")
        if identity.get("source_revision") != source.get("git_revision"):
            fail(f"construction rejection {candidate} has another source revision")
        if candidate in rejected_candidates or candidate in candidate_rows:
            fail(f"duplicate construction identity {candidate}")
        rejected_candidates.add(candidate)
        arm = row.get("arm")
        if arm not in rejection_counts:
            fail(f"construction rejection {candidate} has invalid arm")
        rejection_counts[arm] += 1
        if not row.get("reason"):
            fail(f"construction rejection {candidate} has no reason")

    initial_adaptive = {
        candidate: row
        for candidate, row in candidate_rows.items()
        if row["arm"] == "adaptive" and row["identity"]["level"] is None
    }
    for candidate, row in candidate_rows.items():
        root = row.get("root_candidate_id")
        parent = row.get("parent_candidate_id")
        if row["identity"]["level"] is None:
            if parent is not None or root != candidate:
                fail(f"base candidate {candidate} has invalid root/parent")
        else:
            if parent not in candidate_rows or root not in initial_adaptive:
                fail(f"mutation candidate {candidate} has broken genealogy")

    if len(levels) not in {0, 1, 2}:
        fail("invalid number of level records")
    if [row.get("level") for row in levels] != list(range(len(levels))):
        fail("level records are not sequential")
    for row in levels:
        if len(row.get("survivor_candidate_ids", [])) != 8:
            fail("level does not retain eight survivor candidates")
        if len(row.get("survivor_root_candidate_ids", [])) != 8:
            fail("level does not retain eight survivor roots")
        if len(row.get("clone_parent_candidate_ids", [])) != 8:
            fail("level does not retain eight clone assignments")
        expected_roots = [
            candidate_rows[candidate]["root_candidate_id"]
            for candidate in row["survivor_candidate_ids"]
        ]
        if row["survivor_root_candidate_ids"] != expected_roots:
            fail("level survivor roots disagree with survivor genealogy")
        survivor_set = set(row["survivor_candidate_ids"])
        if any(parent not in survivor_set for parent in row["clone_parent_candidate_ids"]):
            fail("clone assignment does not point to a survivor")
        threshold = row.get("frozen_threshold")
        survivor_sys = [candidate_rows[candidate]["sys"] for candidate in survivor_set]
        if not survivor_sys or threshold != min(survivor_sys):
            fail("frozen threshold is not the survivor minimum")

    transition_keys: set[tuple[int, int, int]] = set()
    clone_states: dict[tuple[int, int], str] = {}
    for row in transitions:
        key = (row.get("level"), row.get("clone_index"), row.get("mutation_step"))
        if key in transition_keys:
            fail(f"duplicate mutation transition {key}")
        transition_keys.add(key)
        level, clone, step = key
        if not all(isinstance(value, int) for value in key):
            fail("mutation transition indices are not integers")
        if level >= len(levels) or clone >= 8 or step >= 2:
            fail(f"mutation transition index out of range {key}")
        proposal = row.get("proposal_candidate_id")
        proposal_row = candidate_rows.get(proposal)
        if proposal_row is None:
            fail(f"transition {key} lacks proposal target")
        identity = proposal_row["identity"]
        if (
            identity["level"] != level
            or identity["clone_index"] != clone
            or identity["mutation_step"] != step
        ):
            fail(f"transition {key} disagrees with proposal identity")
        expected_before = clone_states.get(
            (level, clone), levels[level]["clone_parent_candidate_ids"][clone]
        )
        if row.get("state_before_candidate_id") != expected_before:
            fail(f"transition {key} state chain is broken")
        threshold = levels[level]["frozen_threshold"]
        if row.get("frozen_threshold") != threshold:
            fail(f"transition {key} changes frozen threshold")
        expected_accept = proposal_row["sys"] is not None and proposal_row["sys"] >= threshold
        if row.get("accepted") != expected_accept:
            fail(f"transition {key} acceptance rule mismatch")
        expected_after = proposal if expected_accept else expected_before
        if row.get("state_after_candidate_id") != expected_after:
            fail(f"transition {key} next-state semantics mismatch")
        if row.get("root_candidate_id") != proposal_row["root_candidate_id"]:
            fail(f"transition {key} changes ancestry root")
        clone_states[(level, clone)] = expected_after

    population = [
        candidate
        for candidate, row in candidate_rows.items()
        if row["arm"] == "adaptive" and row["identity"]["level"] is None
    ]
    if levels and len(population) != 16:
        fail("first level does not begin with 16 successful initial particles")
    for level_row in levels:
        level = level_row["level"]
        expected_survivors = sorted(
            population,
            key=lambda candidate: (-candidate_rows[candidate]["sys"], candidate),
        )[:8]
        if level_row["survivor_candidate_ids"] != expected_survivors:
            fail(f"level {level} survivors violate the frozen tie rule")
        if all((level, clone) in clone_states for clone in range(8)):
            population = expected_survivors + [
                clone_states[(level, clone)] for clone in range(8)
            ]

    stop_path = directory / "stop-event.json"
    stop = read_json(stop_path) if stop_path.exists() else None
    if stop is None:
        if len(targets) != 64 or len(transitions) != 32 or len(levels) != 2:
            fail("complete smoke does not contain 64 targets, 32 transitions, and two levels")
        if attempts != {"adaptive": list(range(1, 49)), "iid": list(range(1, 17))}:
            fail("complete smoke does not close the fixed arm budgets")
        distinct_by_arm = {
            arm: len(
                {
                    row["exact_geometry_key"]
                    for row in targets
                    if row["arm"] == arm and row["evaluation_status"] == "success"
                }
            )
            for arm in attempts
        }
        if any(count < 4 for count in distinct_by_arm.values()):
            fail("readiness gate requires four distinct successful keys per arm")
        if any(len(set(row["survivor_root_candidate_ids"])) < 2 for row in levels):
            fail("readiness gate requires two surviving roots at every level")
        if not any(row["accepted"] for row in transitions):
            fail("readiness gate requires a nonzero accepted valid mutation")
    else:
        candidate = stop.get("candidate_id")
        target = candidate_rows.get(candidate)
        if (
            stop.get("event") != "sys_gt_one_flush_and_stop"
            or target is None
            or target.get("sys") != stop.get("sys")
            or stop.get("sys", 0) <= 1
        ):
            fail("invalid sys > 1 stop event")
        if targets[-1].get("candidate_id") != candidate:
            fail("target evaluation continued after sys > 1 stop event")

    runs_by_arm: dict[str, dict[str, Any]] = {}
    for row in arm_runs:
        arm = row.get("arm")
        if arm in runs_by_arm or arm not in attempts:
            fail("duplicate or invalid arm-run row")
        runs_by_arm[arm] = row
        if row.get("target_attempts") != len(attempts[arm]):
            fail(f"{arm} arm-run target count mismatch")
        if row.get("construction_rejections") != rejection_counts[arm]:
            fail(f"{arm} arm-run construction count mismatch")
        if row.get("cache_misses") != status_counts[arm]["miss"]:
            fail(f"{arm} arm-run miss count mismatch")
        if row.get("cache_hits") != status_counts[arm]["hit"]:
            fail(f"{arm} arm-run hit count mismatch")
        if row.get("failed_misses") != status_counts[arm]["failed_miss"]:
            fail(f"{arm} arm-run failed-miss count mismatch")
        distinct = len(
            {
                target["exact_geometry_key"]
                for target in targets
                if target["arm"] == arm and target["evaluation_status"] == "success"
            }
        )
        if row.get("distinct_successful_keys") != distinct:
            fail(f"{arm} arm-run distinct-key count mismatch")
        if not isinstance(row.get("wall_time_ms"), (int, float)) or row["wall_time_ms"] < 0:
            fail(f"{arm} arm-run wall time invalid")
    expected_run_arms = (
        {"adaptive", "iid"}
        if stop is None or stop["arm"] == "iid"
        else {"adaptive"}
    )
    if set(runs_by_arm) != expected_run_arms:
        fail("arm-run rows do not match completed/stopped execution")
    if stop is None and not all(row.get("complete") is True for row in arm_runs):
        fail("unstopped arm-run row is incomplete")
    if stop is None and sum(row["wall_time_ms"] for row in arm_runs) > 601_000:
        fail("complete smoke exceeds the frozen wall-time envelope")

    return {
        "verified": True,
        "artifact_kind": kind,
        "adaptive_attempts": len(attempts["adaptive"]),
        "iid_attempts": len(attempts["iid"]),
        "construction_rejections": len(rejections),
        "distinct_successful_keys": {
            arm: len(
                {
                    row["exact_geometry_key"]
                    for row in targets
                    if row["arm"] == arm and row["evaluation_status"] == "success"
                }
            )
            for arm in attempts
        },
        "stopped_on_sys_gt_one": stop is not None,
        "probability_estimate": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifacts", type=Path)
    args = parser.parse_args()
    try:
        result = verify(args.artifacts)
    except ArtifactError as error:
        raise SystemExit(f"artifact verification failed: {error}") from error
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
