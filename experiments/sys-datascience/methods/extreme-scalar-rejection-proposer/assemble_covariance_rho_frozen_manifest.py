#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Assemble and verify the two-seed target-free covariance-rho manifest."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path

RHO_ID = "frozen_low_vertex_covariance_rho_bottom_0p005"
RIDGE_ID = "frozen_ridge_bottom_0p01_then_bottom_0p5"
CONTROL_ID = "frozen_shared_disjoint_control_25_per_bucket"
EXPECTED_SEEDS = (2026071201, 2026071202)
EXPECTED_BUCKETS = 10
EXPECTED_FEATURE_ROWS = 50_000


def jsonl(path: Path):
    with path.open() as handle:
        for line in handle:
            if line.strip():
                yield json.loads(line)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--code", type=Path, required=True)
    parser.add_argument("--config", type=Path, action="append", required=True)
    args = parser.parse_args()
    if len(args.config) != 2:
        raise SystemExit("exactly two --config paths are required")

    rows_by_id: dict[str, dict] = {}
    arm_counts = Counter()
    invalid_counts = Counter()
    source_hashes: dict[str, str] = {str(args.code): sha256(args.code)}
    for config in args.config:
        source_hashes[str(config)] = sha256(config)

    for seed in EXPECTED_SEEDS:
        directory = args.root / f"seed-{seed}"
        features_path = directory / "candidate-feature-table.jsonl"
        selected_path = directory / "selected-candidates-before-sys.jsonl"
        plan_path = directory / "selection-plan.json"
        geometry_path = directory / "candidate-geometry-cache.jsonl"
        resolved_path = directory / "resolved-run-config.json"
        for path in (features_path, selected_path, plan_path, geometry_path, resolved_path):
            if not path.is_file():
                raise SystemExit(f"missing required file: {path}")
            source_hashes[str(path)] = sha256(path)

        feature_rows = 0
        seed_buckets = set()
        for row in jsonl(features_path):
            feature_rows += 1
            if row["seed"] != seed:
                raise SystemExit(f"feature seed mismatch in {features_path}")
            seed_buckets.add(row["bucket_id"])
            invalid_counts[(seed, row["vertex_covariance_status"])] += 1
        if feature_rows != EXPECTED_FEATURE_ROWS or len(seed_buckets) != EXPECTED_BUCKETS:
            raise SystemExit(
                f"seed {seed}: feature_rows={feature_rows}, buckets={len(seed_buckets)}"
            )

        for row in jsonl(selected_path):
            if row["source"]["seed"] != seed:
                raise SystemExit(f"selection seed mismatch in {selected_path}")
            if row["baseline_ids"]:
                raise SystemExit(f"unexpected baseline membership: {row['candidate_id']}")
            memberships = set(row["selection_ids"])
            if not memberships or not memberships <= {RHO_ID, RIDGE_ID, CONTROL_ID}:
                raise SystemExit(f"unexpected memberships for {row['candidate_id']}: {memberships}")
            if CONTROL_ID in memberships and len(memberships) != 1:
                raise SystemExit(f"control overlaps selected arm: {row['candidate_id']}")
            for arm in memberships:
                arm_counts[(seed, row["bucket_id"], arm)] += 1
            candidate_id = row["candidate_id"]
            if candidate_id in rows_by_id:
                raise SystemExit(f"duplicate candidate id across seeds: {candidate_id}")
            rows_by_id[candidate_id] = row

    bucket_names = sorted({row["bucket_id"] for row in rows_by_id.values()})
    if len(bucket_names) != EXPECTED_BUCKETS:
        raise SystemExit(f"combined manifest has {len(bucket_names)} buckets")
    for seed in EXPECTED_SEEDS:
        for bucket in bucket_names:
            for arm in (RHO_ID, RIDGE_ID, CONTROL_ID):
                count = arm_counts[(seed, bucket, arm)]
                if count != 25:
                    raise SystemExit(f"seed={seed} bucket={bucket} arm={arm} count={count}")

    output = args.root / "frozen-selected-candidates-before-sys.jsonl"
    with output.open("w") as handle:
        for candidate_id in sorted(rows_by_id):
            handle.write(json.dumps(rows_by_id[candidate_id], sort_keys=True, separators=(",", ":")))
            handle.write("\n")
    source_hashes[str(output)] = sha256(output)

    rho_ids = {candidate for candidate, row in rows_by_id.items() if RHO_ID in row["selection_ids"]}
    ridge_ids = {candidate for candidate, row in rows_by_id.items() if RIDGE_ID in row["selection_ids"]}
    control_ids = {candidate for candidate, row in rows_by_id.items() if CONTROL_ID in row["selection_ids"]}
    summary = {
        "schema": "sys-datascience.extreme-scalar-rejection-proposer.covariance-rho-frozen-manifest.v1",
        "producer_seeds": list(EXPECTED_SEEDS),
        "control_seed": 2026071299,
        "feature_rows": 100_000,
        "arm_memberships": {"rho": len(rho_ids), "ridge": len(ridge_ids), "control": len(control_ids)},
        "rho_ridge_overlap": len(rho_ids & ridge_ids),
        "rho_ridge_jaccard": len(rho_ids & ridge_ids) / len(rho_ids | ridge_ids),
        "unique_rows": len(rows_by_id),
        "control_disjoint": control_ids.isdisjoint(rho_ids | ridge_ids),
        "per_seed_bucket_arm_count": 25,
        "invalid_status_counts": {
            f"seed-{seed}": {
                status: invalid_counts[(seed, status)]
                for status in sorted({key[1] for key in invalid_counts if key[0] == seed})
            }
            for seed in EXPECTED_SEEDS
        },
        "sha256": dict(sorted(source_hashes.items())),
    }
    summary_path = args.root / "frozen-manifest-summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print(json.dumps(summary, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
