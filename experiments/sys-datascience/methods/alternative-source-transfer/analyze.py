#!/usr/bin/env python3
"""Manifest-gated post-target analyzer for the frozen 91-row union.

This command is intentionally not run during the target-free repair. It only
reads a complete authorized evaluator artifact and writes an analysis report.
"""
from __future__ import annotations

import argparse
import json
import math
import random
from pathlib import Path
from validate_packet import validate, rows

BOOTSTRAP_SEED = 2026071602
PERMUTATION_SEED = 2026071603
BOOTSTRAP_REPETITIONS = 10000
PERMUTATION_REPETITIONS = 10000
TARGET_SCHEMA = "alternative-source-transfer-target-v1"
TARGET_FIELDS = {
    "schema", "candidate_id", "logical_cell", "bucket", "selection_memberships",
    "geometry_fingerprint", "source_sha256", "feature_sha256", "selection_sha256",
    "evaluator_source", "evaluator_build", "volume", "capacity", "sys",
}


def finite(value: object) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(float(value))


def target_rows(out: Path, target_path: Path) -> tuple[list[dict], dict, dict]:
    gate = validate(out)
    source = {row["candidate_id"]: row for row in rows(out / "source.jsonl")}
    selection = {row["candidate_id"]: row for row in rows(out / "selection.jsonl")}
    data = list(rows(target_path))
    if len(data) != len(selection) or len({row.get("candidate_id") for row in data}) != len(data):
        raise ValueError("target artifact must contain exactly one row per frozen ID")
    for row in data:
        if set(row) != TARGET_FIELDS or row.get("schema") != TARGET_SCHEMA:
            raise ValueError("target schema or unexpected field mismatch")
        cid = row.get("candidate_id")
        if cid not in selection:
            raise ValueError("unknown target candidate ID")
        src, pick = source[cid], selection[cid]
        for key in ("logical_cell", "bucket", "geometry_fingerprint"):
            if row.get(key) != src.get(key) or row.get(key) != pick.get(key):
                raise ValueError(f"target identity mismatch in {key}")
        if row.get("selection_memberships") != pick.get("memberships"):
            raise ValueError("target membership mismatch")
        if row.get("source_sha256") != gate_source_hash(out, "source_sha256") or row.get("feature_sha256") != gate_source_hash(out, "feature_sha256") or row.get("selection_sha256") != gate_source_hash(out, "selection_sha256"):
            raise ValueError("target frozen artifact provenance mismatch")
        if not all(finite(row.get(key)) for key in ("volume", "capacity", "sys")) or row["volume"] <= 0:
            raise ValueError("target numeric field is malformed or nonfinite")
        if not isinstance(row.get("evaluator_source"), str) or not row["evaluator_source"] or not isinstance(row.get("evaluator_build"), str) or not row["evaluator_build"]:
            raise ValueError("target evaluator identity is missing")
    return data, source, selection


def gate_source_hash(out: Path, field: str) -> str:
    return json.loads((out / "manifest.json").read_text())[field]


def arm_values(data: list[dict], selection: dict, bucket: str, arm: str) -> list[float]:
    return [float(row["sys"]) for row in data if row["bucket"] == bucket and arm in selection[row["candidate_id"]]["memberships"]]


def percentile(values: list[float], q: float) -> float:
    ordered = sorted(values)
    if not ordered:
        raise ValueError("empty bootstrap sample")
    index = q * (len(ordered) - 1)
    lo, hi = math.floor(index), math.ceil(index)
    return ordered[lo] if lo == hi else ordered[lo] + (ordered[hi] - ordered[lo]) * (index - lo)


def estimand(data: list[dict], selection: dict, arm: str) -> dict:
    bucket_effects = {}
    for bucket in ("4x6", "6x6"):
        selected = arm_values(data, selection, bucket, arm)
        control = arm_values(data, selection, bucket, "control")
        bucket_effects[bucket] = {
            "selected_mean": sum(selected) / len(selected),
            "control_mean": sum(control) / len(control),
            "effect": sum(selected) / len(selected) - sum(control) / len(control),
            "selected_median": percentile(selected, .5),
            "control_median": percentile(control, .5),
            "selected_range": [min(selected), max(selected)],
            "control_range": [min(control), max(control)],
            "n_selected": len(selected), "n_control": len(control),
            "overlap_rows": len({r["candidate_id"] for r in data if r["bucket"] == bucket and arm in selection[r["candidate_id"]]["memberships"] and "ridge" in selection[r["candidate_id"]]["memberships"]}),
        }
    point = sum(bucket_effects[b]["effect"] for b in bucket_effects) / 2
    return {"bucket_effects": bucket_effects, "equal_bucket_effect": point}


def bootstrap(data: list[dict], selection: dict, arm: str) -> list[float]:
    rng = random.Random(BOOTSTRAP_SEED + (1 if arm == "ridge" else 0))
    result = []
    for _ in range(BOOTSTRAP_REPETITIONS):
        effects = []
        for bucket in ("4x6", "6x6"):
            selected, control = arm_values(data, selection, bucket, arm), arm_values(data, selection, bucket, "control")
            effects.append(sum(rng.choice(selected) for _ in selected) / len(selected) - sum(rng.choice(control) for _ in control) / len(control))
        result.append(sum(effects) / 2)
    return result


def permutation(data: list[dict], selection: dict, arm: str) -> dict:
    rng = random.Random(PERMUTATION_SEED + (1 if arm == "ridge" else 0))
    observed = estimand(data, selection, arm)["equal_bucket_effect"]
    null = []
    for _ in range(PERMUTATION_REPETITIONS):
        effects = []
        for bucket in ("4x6", "6x6"):
            chosen = [r["sys"] for r in data if r["bucket"] == bucket and (arm in selection[r["candidate_id"]]["memberships"] or "control" in selection[r["candidate_id"]]["memberships"])]
            n = len(arm_values(data, selection, bucket, arm)); rng.shuffle(chosen)
            effects.append(sum(chosen[:n]) / n - sum(chosen[n:]) / (len(chosen) - n))
        null.append(sum(effects) / 2)
    return {"observed": observed, "null_ge_abs_observed": sum(abs(x) >= abs(observed) for x in null) / len(null)}


def summarize(data: list[dict], selection: dict) -> dict:
    result = {"schema": "alternative-source-transfer-analysis-v1", "bootstrap_seed": BOOTSTRAP_SEED, "bootstrap_repetitions": BOOTSTRAP_REPETITIONS, "permutation_seed": PERMUTATION_SEED, "permutation_repetitions": PERMUTATION_REPETITIONS, "all_sys_gt_1": [row for row in data if row["sys"] > 1], "selectors": {}}
    for arm in ("rho", "ridge"):
        estimate = estimand(data, selection, arm)
        interval = [percentile(bootstrap(data, selection, arm), .025), percentile(bootstrap(data, selection, arm), .975)]
        estimate["bootstrap_95_percent_interval"] = interval
        estimate["permutation"] = permutation(data, selection, arm)
        estimate["sys_gt_1"] = [r for r in data if arm in selection[r["candidate_id"]]["memberships"] and r["sys"] > 1]
        if all(estimate["bucket_effects"][b]["effect"] > 0 for b in ("4x6", "6x6")) and estimate["equal_bucket_effect"] >= .08 and interval[0] > 0:
            verdict = "strong_transfer"
        elif interval[1] < .08:
            verdict = "material_negative"
        else:
            verdict = "ambiguous"
        estimate["classification"] = verdict
        result["selectors"][arm] = estimate
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("out", type=Path)
    parser.add_argument("--targets", type=Path, required=True)
    parser.add_argument("--write", type=Path, help="write analysis JSON only after full validation")
    args = parser.parse_args()
    data, _, selection = target_rows(args.out, args.targets)
    result = summarize(data, selection)
    if args.write:
        tmp = args.write.with_suffix(args.write.suffix + ".tmp")
        tmp.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
        tmp.replace(args.write)
    else:
        print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
