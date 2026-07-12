#!/usr/bin/env python3
"""Check the promoted frozen eight-row endpoint packet without target calls."""

import hashlib
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parent
ARTIFACTS = ROOT / "artifacts"
EXPECTED_IDS = {
    f"ridge-endpoint-{bucket}-{label}"
    for bucket in ("3x6", "4x4")
    for label in ("q01", "q001", "q0001", "endpoint")
}


def load_json(path):
    with path.open() as handle:
        return json.load(handle)


def load_jsonl(path):
    with path.open() as handle:
        return [json.loads(line) for line in handle]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def main():
    candidates_path = ARTIFACTS / "candidates.jsonl"
    api_path = ARTIFACTS / "api-verification.jsonl"
    target_path = ARTIFACTS / "target-evaluation.jsonl"
    manifest_path = ARTIFACTS / "capacity-implementation-manifest.json"
    certificate_path = ARTIFACTS / "q01-certified-minimizers.json"
    summary_path = ARTIFACTS / "target-summary.json"
    source_path = ARTIFACTS / "evaluator-source-v2-before-promotion.rs"

    candidates = load_jsonl(candidates_path)
    api_rows = load_jsonl(api_path)
    target_rows = load_jsonl(target_path)
    manifest = load_json(manifest_path)
    certificate = load_json(certificate_path)
    summary = load_json(summary_path)

    candidate_ids = {row["candidate_id"] for row in candidates}
    target_ids = {row["candidate_id"] for row in target_rows}
    require(candidate_ids == EXPECTED_IDS, "candidate IDs are not the frozen eight")
    require(target_ids == EXPECTED_IDS and len(target_rows) == 8, "target rows are not the frozen eight")
    require(len(api_rows) == 8 and {row["candidate_id"] for row in api_rows} == EXPECTED_IDS,
            "API verification does not cover the frozen eight")
    require(all(row["passed"] for row in api_rows), "a geometry/API row failed")

    candidate_digest = digest(candidates_path)
    api_digest = digest(api_path)
    manifest_digest = digest(manifest_path)
    source_digest = digest(source_path)
    require(all(row["candidates_sha256"] == candidate_digest for row in target_rows),
            "target rows do not identify the retained candidates")
    require(all(row["api_verification_sha256"] == api_digest for row in target_rows),
            "target rows do not identify API verification")
    require(all(row["capacity_manifest_sha256"] == manifest_digest for row in target_rows),
            "target rows do not identify the implementation manifest")
    require(all(row["evaluator_source_sha256"] == source_digest for row in target_rows),
            "target rows do not identify their archived evaluator source")
    require(all(row["result_is_finite"] and row["sys"] <= 1 for row in target_rows),
            "target invariants failed")
    require(all(math.isfinite(row[key]) for row in target_rows
                for key in ("volume", "capacity", "sys", "min_action_lower", "min_action_upper")),
            "a target numeric field is non-finite")
    require(all(abs(row["sys"] - row["capacity"] ** 2 / (2 * row["volume"])) <= 1e-12
                for row in target_rows), "target arithmetic mismatch")
    require(all(0 <= row["exact_fallback_rejected_count"]
                <= row["exact_fallback_resolution_count"]
                <= row["f64_indeterminate_candidate_count_before_aggregation"]
                for row in target_rows), "invalid pre-trim exact-fallback counters")

    q01 = next(row for row in target_rows if row["candidate_id"] == "ridge-endpoint-3x6-q01")
    require(certificate["candidate_id"] == q01["candidate_id"], "certificate scopes the wrong row")
    require(certificate["candidates_sha256"] == candidate_digest, "certificate candidate identity mismatch")
    require(certificate["capacity_manifest_sha256"] == manifest_digest, "certificate manifest identity mismatch")
    require(abs(certificate["ordinary_capacity"] - q01["capacity"]) <= certificate["agreement_tolerance"],
            "q01 certificate does not agree with ordinary capacity")

    require(not any(Path(item["path"]).is_absolute() for item in manifest["implementation_files"]),
            "manifest stores an absolute implementation path")
    require(manifest["cargo_lock_sha256"] == digest(ROOT / "Cargo.lock"),
            "manifest Cargo.lock hash mismatch")
    repo_root = ROOT.parents[3]
    closure_parts = []
    for item in manifest["implementation_files"]:
        implementation_path = repo_root / item["path"]
        require(implementation_path.is_file(), f'manifest source missing: {item["path"]}')
        require(digest(implementation_path) == item["sha256"],
                f'manifest source hash mismatch: {item["path"]}')
        closure_parts.extend((item["path"], "\n", item["sha256"], "\n"))
    require(hashlib.sha256("".join(closure_parts).encode()).hexdigest()
            == manifest["implementation_closure_sha256"],
            "manifest implementation-closure digest mismatch")
    cdf = load_json(ARTIFACTS / "cdf-placement.json")
    require("cache" not in cdf and cdf["cache_sha256"],
            "CDF artifact stores a local cache path or lacks its cache identity")
    require(summary["target_evaluation_sha256"] == digest(target_path), "summary target hash mismatch")
    require(summary["certificate_sha256"] == digest(certificate_path), "summary certificate hash mismatch")
    require(summary["candidates_sha256"] == candidate_digest, "summary candidate hash mismatch")
    require(summary["api_verification_sha256"] == api_digest, "summary API hash mismatch")
    require(summary["cdf_placement_json_sha256"] == digest(ARTIFACTS / "cdf-placement.json"),
            "summary CDF JSON hash mismatch")
    require(summary["cdf_placement_tsv_sha256"] == digest(ARTIFACTS / "cdf-placement.tsv"),
            "summary CDF TSV hash mismatch")
    require(summary["capacity_manifest_sha256"] == manifest_digest,
            "summary manifest hash mismatch")
    require(summary["evaluator_source_sha256"] == source_digest,
            "summary evaluator source hash mismatch")
    print("ridge-endpoint-path: frozen eight-row identity and linkage checks pass")


if __name__ == "__main__":
    main()
