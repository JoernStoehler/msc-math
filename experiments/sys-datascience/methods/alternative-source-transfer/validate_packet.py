#!/usr/bin/env python3
"""Fail-closed target-free and post-target packet checks.

The current checked-out route is ``--validate-only``.  The target evaluator
never invokes capacity; a later authorized runner may provide a JSONL target
file to ``analyze.py`` after this manifest has passed independent review.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

FORBIDDEN = {"capacity", "sys", "iterations", "bounce", "target", "target_time_ms"}
ARMS = {"rho", "ridge", "control"}
EXPECTED = {
    "identity_scope": "alternative-source-transfer-v1",
    "master_seed": 2026071601,
    "control_seed": 2026071299,
    "law": "factorial-both",
    "buckets": ["4x6", "6x6"],
    "row_target_per_bucket": 3200,
    "row_cap_per_bucket": 4000,
    "attempt_cap": 128,
    "source_count": 6400,
    "feature_count": 6400,
    "selection_count": 91,
    "unique_target_rows": 91,
    "arm_overlap_rows": 5,
    "clean_commit": "fcd5546af014942b74a1e9313ee898329a507d3d",
    "lock_hash": "740441674806a1baaea966d5f8f12a66d8e2ef1229b66ca9dcf9225a02f6c45f",
}
SCHEMAS = {
    "source": "alternative-source-transfer-source-v1",
    "feature": "alternative-source-transfer-feature-v1",
}


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(path: Path):
    with path.open() as handle:
        for line in handle:
            if line.strip():
                yield json.loads(line)


def check_no_target(path: Path) -> None:
    for number, row in enumerate(rows(path), 1):
        bad = {key for key in row if any(token in key.lower() for token in FORBIDDEN)}
        if bad:
            raise ValueError(f"target leakage in {path}:{number}: {sorted(bad)}")


def validate(out: Path) -> dict:
    manifest = json.loads((out / "manifest.json").read_text())
    if manifest.get("schema") != "alternative-source-transfer-manifest-v1":
        raise ValueError("wrong manifest schema")
    source = list(rows(out / "source.jsonl"))
    feature = list(rows(out / "features.jsonl"))
    selection = list(rows(out / "selection.jsonl"))
    for key, expected in EXPECTED.items():
        if manifest.get(key) != expected:
            raise ValueError(f"manifest {key} mismatch: {manifest.get(key)!r}")
    if manifest.get("buckets") != EXPECTED["buckets"] or manifest.get("target_free") is not True:
        raise ValueError("manifest identity/target-free constants mismatch")
    if manifest.get("source_sha256") != digest(out / "source.jsonl"):
        raise ValueError("source hash mismatch")
    if manifest.get("feature_sha256") != digest(out / "features.jsonl"):
        raise ValueError("feature hash mismatch")
    if manifest.get("selection_sha256") != digest(out / "selection.jsonl"):
        raise ValueError("selection hash mismatch")
    if len(source) != EXPECTED["source_count"] or len(feature) != EXPECTED["feature_count"]:
        raise ValueError("incomplete frozen source or feature population")
    if len(selection) != EXPECTED["selection_count"] or len(selection) > 96:
        raise ValueError("selection union exceeds 96 unique rows")
    for path in (out / "source.jsonl", out / "features.jsonl", out / "selection.jsonl"):
        check_no_target(path)
    ids = [r["candidate_id"] for r in source]
    cells = [r["logical_cell"] for r in source]
    if len(ids) != len(set(ids)) or len(cells) != len(set(cells)):
        raise ValueError("duplicate source identity")
    source_by_id = {}
    for row in source:
        if row.get("schema") != SCHEMAS["source"] or row.get("identity_scope") != EXPECTED["identity_scope"] or row.get("law") != EXPECTED["law"]:
            raise ValueError("source schema or identity mismatch")
        if row.get("bucket") not in EXPECTED["buckets"] or not isinstance(row.get("geometry_fingerprint"), str):
            raise ValueError("source bucket/fingerprint malformed")
        if not isinstance(row.get("volume"), (int, float)) or not __import__("math").isfinite(float(row["volume"])) or row["volume"] <= 0:
            raise ValueError("source volume is not finite and positive")
        source_by_id[row["candidate_id"]] = row
    feature_by_id = {}
    if len({r.get("candidate_id") for r in feature}) != len(feature):
        raise ValueError("duplicate feature identity")
    for row in feature:
        if row.get("schema") != SCHEMAS["feature"] or row.get("identity_scope") != EXPECTED["identity_scope"] or row.get("law") != EXPECTED["law"]:
            raise ValueError("feature schema or identity mismatch")
        if row.get("candidate_id") not in source_by_id:
            raise ValueError("feature/source join mismatch")
        src = source_by_id[row["candidate_id"]]
        for key in ("logical_cell", "bucket", "row_index", "attempt", "source_geometry_fingerprint"):
            source_key = "geometry_fingerprint" if key == "source_geometry_fingerprint" else key
            if row.get(key) != src.get(source_key):
                raise ValueError(f"feature/source {key} mismatch")
        if row.get("vertex_covariance_status") != "eligible" or not isinstance(row.get("vertex_covariance_rho"), (int, float)):
            raise ValueError("ineligible rho feature")
        import math
        for key in ("vertex_covariance_rho", "ridge_symp_area_sum_over_volume_sqrt", "ridge_symp_area_max_share"):
            if not math.isfinite(float(row.get(key))):
                raise ValueError("nonfinite feature")
        if row.get("ridge_symp_area_ordering_failure_count") != 0:
            raise ValueError("ridge ordering failure")
        feature_by_id[row["candidate_id"]] = row
    if set(source_by_id) != set(feature_by_id):
        raise ValueError("source-feature bijection mismatch")
    if len({r.get("candidate_id") for r in selection}) != len(selection):
        raise ValueError("duplicate selection identity")
    selected_ids = {r["candidate_id"] for r in selection}
    source_ids = set(ids)
    if not selected_ids <= source_ids:
        raise ValueError("selection references unknown source row")
    fps = [r["geometry_fingerprint"] for r in selection]
    if len(fps) != len(set(fps)):
        raise ValueError("duplicate geometry fingerprint in selected/control union")
    counts = {bucket: {arm: 0 for arm in ARMS} for bucket in ("4x6", "6x6")}
    for row in selection:
        if row.get("candidate_id") not in source_by_id:
            raise ValueError("selection references unknown source row")
        src = source_by_id[row["candidate_id"]]
        for key in ("logical_cell", "bucket", "row_index", "attempt", "geometry_fingerprint"):
            if row.get(key) != src.get("geometry_fingerprint" if key == "geometry_fingerprint" else key):
                raise ValueError(f"selection/source {key} mismatch")
        if not row.get("memberships") or len(row["memberships"]) != len(set(row["memberships"])):
            raise ValueError("selection membership malformed")
        if set(row["memberships"]) - ARMS:
            raise ValueError("unknown selection arm")
        for arm in row["memberships"]:
            counts[row["bucket"]][arm] += 1
    if any(counts[b][a] != 16 for b in counts for a in ARMS):
        raise ValueError(f"membership count mismatch: {counts}")
    for row in selection:
        if "control" in row["memberships"] and len(row["memberships"]) > 1:
            raise ValueError("control overlaps selected arm")
    if counts != manifest.get("membership_counts"):
        raise ValueError("manifest membership counts mismatch")
    return {"source": len(source), "features": len(feature), "selection": len(selection), "counts": counts}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("out", type=Path)
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()
    result = validate(args.out)
    print(json.dumps({"status": "validation-only", **result}, sort_keys=True))


if __name__ == "__main__":
    main()
