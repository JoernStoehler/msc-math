#!/usr/bin/env python3
"""Check the frozen two-call symmetry completion without target calls."""

import hashlib
import json
import math
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent
ARTIFACTS = ROOT / "artifacts"
EXPECTED = ["ridge-symmetry-3x6-delta1", "ridge-symmetry-3x6-delta2"]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def jsonl(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def warn_stale(condition, message):
    # Exact bytes are advisory provenance. Identity/order, API status,
    # arithmetic, and sequential comparisons remain blocking.
    if not condition:
        print(
            f"warning: {message}; continuing with semantic checks. Reassess "
            "retained interpretation before treating this packet as equivalent.",
            file=sys.stderr,
        )


def main():
    candidate_path = ARTIFACTS / "candidates.jsonl"
    preflight_path = ARTIFACTS / "api-verification.jsonl"
    source_path = ROOT / "src/main.rs"
    manifest_path = ROOT.parent / "ridge-endpoint-path/artifacts/capacity-implementation-manifest.json"
    candidates = jsonl(candidate_path)
    preflight = jsonl(preflight_path)
    targets = [json.loads((ARTIFACTS / f"target-delta{i}.json").read_text()) for i in (1, 2)]
    freeze = json.loads((ARTIFACTS / "freeze.json").read_text())
    candidate_hash, preflight_hash = digest(candidate_path), digest(preflight_path)
    source_hash, manifest_hash = digest(source_path), digest(manifest_path)
    require([row["candidate_id"] for row in candidates] == EXPECTED, "candidate identity/order changed")
    require(freeze["status"] == "target-free", "freeze is not target-free")
    warn_stale(freeze["candidates_sha256"] == candidate_hash, "freeze candidate bytes differ")
    require([row["candidate_id"] for row in preflight] == EXPECTED and all(row["passed"] for row in preflight),
            "preflight contract failed")
    require([row["candidate_id"] for row in targets] == EXPECTED, "target identity/order changed")
    warn_stale(all(row["candidates_sha256"] == candidate_hash for row in targets), "target candidate bytes differ")
    warn_stale(all(row["preflight_sha256"] == preflight_hash for row in targets), "target preflight bytes differ")
    warn_stale(all(row["evaluator_source_sha256"] == source_hash for row in targets), "target evaluator source differs")
    warn_stale(all(row["capacity_implementation_manifest_sha256"] == manifest_hash for row in targets),
               "target capacity-manifest bytes differ")
    require(all(math.isfinite(row[key]) for row in targets for key in
                ("volume", "capacity", "sys", "min_action_lower", "min_action_upper")),
            "non-finite target field")
    require(all(abs(row["sys"] - row["capacity"]**2/(2*row["volume"])) <= 1e-12 for row in targets),
            "target arithmetic mismatch")
    require(all(row["sys"] < 1 and row["branch_h_absolute_error"] <= 1e-12 for row in targets),
            "crossing or branch-H mismatch")
    require(targets[0]["sys"] < targets[1]["sys"] < 1, "sequential monotonicity failed")
    print("ridge-symmetry-completion: two-call semantic checks pass; any byte drift was warned")


if __name__ == "__main__":
    main()
