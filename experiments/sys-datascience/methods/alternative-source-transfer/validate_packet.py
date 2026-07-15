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
    if manifest.get("source_sha256") != digest(out / "source.jsonl"):
        raise ValueError("source hash mismatch")
    if manifest.get("feature_sha256") != digest(out / "features.jsonl"):
        raise ValueError("feature hash mismatch")
    if manifest.get("selection_sha256") != digest(out / "selection.jsonl"):
        raise ValueError("selection hash mismatch")
    if len(source) != 6400 or len(feature) != 6400:
        raise ValueError("incomplete frozen source or feature population")
    if len(selection) > 96:
        raise ValueError("selection union exceeds 96 unique rows")
    for path in (out / "source.jsonl", out / "features.jsonl", out / "selection.jsonl"):
        check_no_target(path)
    ids = [r["candidate_id"] for r in source]
    cells = [r["logical_cell"] for r in source]
    if len(ids) != len(set(ids)) or len(cells) != len(set(cells)):
        raise ValueError("duplicate source identity")
    selected_ids = {r["candidate_id"] for r in selection}
    source_ids = set(ids)
    if not selected_ids <= source_ids:
        raise ValueError("selection references unknown source row")
    fps = [r["geometry_fingerprint"] for r in selection]
    if len(fps) != len(set(fps)):
        raise ValueError("duplicate geometry fingerprint in selected/control union")
    counts = {bucket: {arm: 0 for arm in ARMS} for bucket in ("4x6", "6x6")}
    for row in selection:
        if set(row["memberships"]) - ARMS:
            raise ValueError("unknown selection arm")
        for arm in row["memberships"]:
            counts[row["bucket"]][arm] += 1
    if any(counts[b][a] != 16 for b in counts for a in ARMS):
        raise ValueError(f"membership count mismatch: {counts}")
    for row in selection:
        if "control" in row["memberships"] and len(row["memberships"]) > 1:
            raise ValueError("control overlaps selected arm")
    if manifest.get("target_free") is not True:
        raise ValueError("manifest is not target-free")
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
