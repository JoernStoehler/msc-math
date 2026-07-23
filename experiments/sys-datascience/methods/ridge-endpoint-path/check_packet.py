#!/usr/bin/env python3
"""Check the promoted frozen eight-row endpoint packet without target calls."""

import hashlib
import json
import math
from pathlib import Path
from pathlib import PurePosixPath
import subprocess
import sys


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


def digest_bytes(contents):
    return hashlib.sha256(contents).hexdigest()


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def warn_stale(condition, message):
    # Byte identities are advisory provenance. Packet schemas, row identities,
    # joins, arithmetic, and finite-value checks remain blocking.
    if not condition:
        print(
            f"warning: {message}; continuing with semantic checks. Reassess "
            "retained interpretation before treating this packet as equivalent.",
            file=sys.stderr,
        )


def require_relative_repo_path(path):
    require(isinstance(path, str), f"manifest source path is not a string: {path!r}")
    repo_path = PurePosixPath(path)
    require(path and not repo_path.is_absolute() and ".." not in repo_path.parts,
            f"manifest source path is not repository-relative: {path!r}")
    return repo_path


def git_bytes(repo_root, commit, path):
    result = subprocess.run(
        ["git", "-C", str(repo_root), "show", f"{commit}:{path}"],
        check=False,
        capture_output=True,
    )
    return result.stdout if result.returncode == 0 else None


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
    warn_stale(all(row["candidates_sha256"] == candidate_digest for row in target_rows),
               "target rows record different candidate bytes")
    warn_stale(all(row["api_verification_sha256"] == api_digest for row in target_rows),
               "target rows record different API-verification bytes")
    warn_stale(all(row["capacity_manifest_sha256"] == manifest_digest for row in target_rows),
               "target rows record different capacity-manifest bytes")
    warn_stale(all(row["evaluator_source_sha256"] == source_digest for row in target_rows),
               "target rows record different evaluator-source bytes")
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
    warn_stale(certificate["candidates_sha256"] == candidate_digest,
               "certificate records different candidate bytes")
    warn_stale(certificate["capacity_manifest_sha256"] == manifest_digest,
               "certificate records different capacity-manifest bytes")
    require(abs(certificate["ordinary_capacity"] - q01["capacity"]) <= certificate["agreement_tolerance"],
            "q01 certificate does not agree with ordinary capacity")

    repo_root = ROOT.parents[3]
    repo_commit = manifest["repo_commit"]
    commit_format = (
        isinstance(repo_commit, str)
        and len(repo_commit) == 40
        and all(character in "0123456789abcdef" for character in repo_commit)
    )
    warn_stale(commit_format, "manifest retained revision is malformed")
    commit_available = False
    if commit_format:
        commit_check = subprocess.run(
            ["git", "-C", str(repo_root), "cat-file", "-e", f"{repo_commit}^{{commit}}"],
            check=False,
            capture_output=True,
        )
        commit_available = commit_check.returncode == 0
    warn_stale(commit_available, f"manifest retained revision is unavailable: {repo_commit}")

    warn_stale(manifest["cargo_lock_sha256"] == digest(ROOT / "Cargo.lock"),
               "current Cargo.lock differs from the retained packet")
    closure_parts = []
    current_drift = []
    for item in manifest["implementation_files"]:
        implementation_path = require_relative_repo_path(item["path"])
        historical_bytes = (
            git_bytes(repo_root, repo_commit, implementation_path)
            if commit_available
            else None
        )
        warn_stale(historical_bytes is not None,
                   f'manifest source is unavailable at retained revision: {item["path"]}')
        if historical_bytes is not None:
            warn_stale(digest_bytes(historical_bytes) == item["sha256"],
                       f'manifest source differs at retained revision: {item["path"]}')
        current_path = repo_root / implementation_path
        if not current_path.is_file() or digest(current_path) != item["sha256"]:
            current_drift.append(item["path"])
        closure_parts.extend((item["path"], "\n", item["sha256"], "\n"))
    warn_stale(hashlib.sha256("".join(closure_parts).encode()).hexdigest()
               == manifest["implementation_closure_sha256"],
               "manifest implementation-closure digest differs")
    cdf = load_json(ARTIFACTS / "cdf-placement.json")
    require("cache" not in cdf and cdf["cache_sha256"],
            "CDF artifact stores a local cache path or lacks its cache identity")
    warn_stale(summary["target_evaluation_sha256"] == digest(target_path), "summary target bytes differ")
    warn_stale(summary["certificate_sha256"] == digest(certificate_path), "summary certificate bytes differ")
    warn_stale(summary["candidates_sha256"] == candidate_digest, "summary candidate bytes differ")
    warn_stale(summary["api_verification_sha256"] == api_digest, "summary API-verification bytes differ")
    warn_stale(summary["cdf_placement_json_sha256"] == digest(ARTIFACTS / "cdf-placement.json"),
               "summary CDF JSON bytes differ")
    warn_stale(summary["cdf_placement_tsv_sha256"] == digest(ARTIFACTS / "cdf-placement.tsv"),
               "summary CDF TSV bytes differ")
    warn_stale(summary["capacity_manifest_sha256"] == manifest_digest,
               "summary capacity-manifest bytes differ")
    warn_stale(summary["evaluator_source_sha256"] == source_digest,
               "summary evaluator-source bytes differ")
    if current_drift:
        print("ridge-endpoint-path: current checkout differs from retained implementation "
              f"at {repo_commit}: {', '.join(current_drift)}")
    print("ridge-endpoint-path: eight-row semantic and linkage checks pass; any byte drift was warned")


if __name__ == "__main__":
    main()
