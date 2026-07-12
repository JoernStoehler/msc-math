#!/usr/bin/env python3
"""Refresh only retained-artifact hashes in the endpoint target summary.

This performs no geometry, feature, capacity, or target evaluation. It exists
so provenance-only promotion changes, such as removing a machine-local input
path from the CDF report, do not require rerunning the frozen target calls.
"""

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
ARTIFACTS = ROOT / "artifacts"
SUMMARY = ARTIFACTS / "target-summary.json"
LINKS = {
    "target_evaluation_sha256": "target-evaluation.jsonl",
    "certificate_sha256": "q01-certified-minimizers.json",
    "candidates_sha256": "candidates.jsonl",
    "api_verification_sha256": "api-verification.jsonl",
    "cdf_placement_json_sha256": "cdf-placement.json",
    "cdf_placement_tsv_sha256": "cdf-placement.tsv",
    "capacity_manifest_sha256": "capacity-implementation-manifest.json",
    "evaluator_source_sha256": "evaluator-source-v2-before-promotion.rs",
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    summary = json.loads(SUMMARY.read_text())
    if summary.get("schema") != "ridge-endpoint-smoke.target-summary.v2":
        raise RuntimeError("unexpected target-summary schema")
    for field, relative_path in LINKS.items():
        summary[field] = digest(ARTIFACTS / relative_path)
    temporary = SUMMARY.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(summary, indent=2) + "\n")
    temporary.replace(SUMMARY)
    print("refreshed target-summary artifact links without target calls")


if __name__ == "__main__":
    main()
