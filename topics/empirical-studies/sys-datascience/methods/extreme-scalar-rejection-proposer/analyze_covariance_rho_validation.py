#!/usr/bin/env python3
"""Read the frozen covariance-rho manifest after, and only after, evaluation.

This is deliberately separate from the producer: it never reconstructs a
selection from current feature values or command-line selection parameters.
It accepts the immutable combined manifest plus fresh evaluation cache(s),
checks their row identities, and writes the predeclared decision report.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import tempfile
from collections import defaultdict
from pathlib import Path

RHO_ID = "frozen_low_vertex_covariance_rho_bottom_0p005"
RIDGE_ID = "frozen_ridge_bottom_0p01_then_bottom_0p5"
CONTROL_ID = "frozen_shared_disjoint_control_25_per_bucket"
EXPECTED_SEEDS = (2026071201, 2026071202)
EXPECTED_BUCKETS = 10
T_975_DF19 = 2.093_024_054_408_263
T_950_DF19 = 1.729_132_811_521_367
PREVIOUS_GENERATED_MAXIMUM = 0.950_971_838_1


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def mean(values: list[float]) -> float:
    if not values:
        raise ValueError("mean requires at least one value")
    return sum(values) / len(values)


def mean_interval(values: list[float]) -> dict:
    """Student-t intervals for the fixed 20 stratum effects."""
    if len(values) != 20:
        raise ValueError(f"the frozen design requires 20 stratum effects, got {len(values)}")
    estimate = mean(values)
    variance = sum((value - estimate) ** 2 for value in values) / (len(values) - 1)
    standard_error = math.sqrt(variance / len(values))
    return {
        "estimate": estimate,
        "standard_error": standard_error,
        "two_sided_95": [
            estimate - T_975_DF19 * standard_error,
            estimate + T_975_DF19 * standard_error,
        ],
        "upper_one_sided_95": estimate + T_950_DF19 * standard_error,
    }


def arm_ids(rows: dict[str, dict], arm: str) -> set[str]:
    return {candidate_id for candidate_id, row in rows.items() if arm in row["selection_ids"]}


def validate_manifest(rows: list[dict]) -> dict[str, dict]:
    by_id: dict[str, dict] = {}
    counts: dict[tuple[int, str, str], int] = defaultdict(int)
    for row in rows:
        candidate_id = row["candidate_id"]
        if candidate_id in by_id:
            raise ValueError(f"duplicate candidate id in frozen manifest: {candidate_id}")
        memberships = set(row["selection_ids"])
        if not memberships or not memberships <= {RHO_ID, RIDGE_ID, CONTROL_ID}:
            raise ValueError(f"unexpected arm memberships for {candidate_id}: {memberships}")
        if row.get("baseline_ids"):
            raise ValueError(f"frozen manifest must not have baselines: {candidate_id}")
        if CONTROL_ID in memberships and len(memberships) != 1:
            raise ValueError(f"control overlaps another arm: {candidate_id}")
        source = row.get("source", {})
        seed = source.get("seed")
        if seed not in EXPECTED_SEEDS:
            raise ValueError(f"unexpected producer seed for {candidate_id}: {seed}")
        bucket = row["bucket_id"]
        for arm in memberships:
            counts[(seed, bucket, arm)] += 1
        by_id[candidate_id] = row

    buckets = sorted({row["bucket_id"] for row in by_id.values()})
    if len(buckets) != EXPECTED_BUCKETS:
        raise ValueError(f"expected {EXPECTED_BUCKETS} buckets, got {len(buckets)}")
    for seed in EXPECTED_SEEDS:
        for bucket in buckets:
            for arm in (RHO_ID, RIDGE_ID, CONTROL_ID):
                if counts[(seed, bucket, arm)] != 25:
                    raise ValueError(
                        f"expected 25 rows for seed={seed} bucket={bucket} arm={arm}; "
                        f"got {counts[(seed, bucket, arm)]}"
                    )
    rho = arm_ids(by_id, RHO_ID)
    ridge = arm_ids(by_id, RIDGE_ID)
    control = arm_ids(by_id, CONTROL_ID)
    if not control.isdisjoint(rho | ridge):
        raise ValueError("control is not disjoint from rho/ridge")
    if len(by_id) != 1436 or not (len(rho) == len(ridge) == len(control) == 500):
        raise ValueError("manifest does not have the frozen 1436-row / 500-membership design")
    return by_id


def read_evaluations(paths: list[Path], manifest: dict[str, dict]) -> dict[str, dict]:
    evaluations: dict[str, dict] = {}
    for path in paths:
        for row in read_jsonl(path):
            candidate_id = row.get("candidate_id")
            if candidate_id not in manifest:
                raise ValueError(f"evaluation row not in frozen manifest: {candidate_id}")
            if candidate_id in evaluations:
                raise ValueError(f"duplicate evaluated candidate id: {candidate_id}")
            selected = manifest[candidate_id]
            for field in (
                "candidate_id",
                "poly_id",
                "bucket_id",
                "selection_ids",
                "baseline_ids",
                "selection_feature",
                "selection_direction",
                "selection_feature_value",
                "selection_rule_values",
            ):
                if row.get(field) != selected.get(field):
                    raise ValueError(f"{field} mismatch for evaluated candidate {candidate_id}")
            value = row.get("sys")
            if not isinstance(value, (int, float)) or not math.isfinite(value):
                raise ValueError(f"non-finite sys for {candidate_id}")
            evaluations[candidate_id] = row
    missing = set(manifest) - set(evaluations)
    if missing:
        raise ValueError(f"missing sys evaluations for {len(missing)} frozen-manifest rows")
    return evaluations


def analyze(manifest_path: Path, evaluation_paths: list[Path]) -> dict:
    manifest = validate_manifest(read_jsonl(manifest_path))
    evaluations = read_evaluations(evaluation_paths, manifest)
    values = {candidate_id: float(row["sys"]) for candidate_id, row in evaluations.items()}
    groups: dict[tuple[int, str], dict[str, list[float]]] = defaultdict(lambda: defaultdict(list))
    for candidate_id, row in manifest.items():
        key = (row["source"]["seed"], row["bucket_id"])
        for arm in row["selection_ids"]:
            groups[key][arm].append(values[candidate_id])

    stratum_rows = []
    rho_control_effects = []
    rho_ridge_effects = []
    ridge_control_effects = []
    for (seed, bucket), arms in sorted(groups.items()):
        if set(arms) != {RHO_ID, RIDGE_ID, CONTROL_ID} or any(len(arms[arm]) != 25 for arm in arms):
            raise ValueError(f"invalid arm group at seed={seed} bucket={bucket}")
        rho_control = mean(arms[RHO_ID]) - mean(arms[CONTROL_ID])
        rho_ridge = mean(arms[RHO_ID]) - mean(arms[RIDGE_ID])
        ridge_control = mean(arms[RIDGE_ID]) - mean(arms[CONTROL_ID])
        rho_control_effects.append(rho_control)
        rho_ridge_effects.append(rho_ridge)
        ridge_control_effects.append(ridge_control)
        stratum_rows.append({
            "seed": seed,
            "bucket": bucket,
            "rho_mean_sys": mean(arms[RHO_ID]),
            "ridge_mean_sys": mean(arms[RIDGE_ID]),
            "control_mean_sys": mean(arms[CONTROL_ID]),
            "rho_minus_control_mean_sys": rho_control,
            "rho_minus_ridge_mean_sys": rho_ridge,
            "ridge_minus_control_mean_sys": ridge_control,
        })

    rho_control = mean_interval(rho_control_effects)
    rho_ridge = mean_interval(rho_ridge_effects)
    ridge_control = mean_interval(ridge_control_effects)
    seed_effects = {
        str(seed): mean([row["rho_minus_control_mean_sys"] for row in stratum_rows if row["seed"] == seed])
        for seed in EXPECTED_SEEDS
    }
    bucket_effects = {
        bucket: mean([row["rho_minus_control_mean_sys"] for row in stratum_rows if row["bucket"] == bucket])
        for bucket in sorted({row["bucket"] for row in stratum_rows})
    }
    leave_one_bucket_out = {
        bucket: mean([row["rho_minus_control_mean_sys"] for row in stratum_rows if row["bucket"] != bucket])
        for bucket in bucket_effects
    }
    rho = arm_ids(manifest, RHO_ID)
    ridge = arm_ids(manifest, RIDGE_ID)
    control = arm_ids(manifest, CONTROL_ID)
    arm_outputs = {
        label: {
            "memberships": len(ids),
            "maximum_sys": max(values[candidate_id] for candidate_id in ids),
            "count_sys_gt_0p9": sum(values[candidate_id] > 0.9 for candidate_id in ids),
            "count_sys_gt_1": sum(values[candidate_id] > 1.0 for candidate_id in ids),
            "count_sys_gt_previous_generated_maximum": sum(
                values[candidate_id] > PREVIOUS_GENERATED_MAXIMUM for candidate_id in ids
            ),
        }
        for label, ids in (("rho", rho), ("ridge", ridge), ("control", control))
    }
    primary_success = (
        rho_control["estimate"] >= 0.08
        and rho_control["two_sided_95"][0] > 0.0
        and all(value > 0.0 for value in seed_effects.values())
        and sum(value > 0.0 for value in bucket_effects.values()) >= 7
    )
    negative = rho_control["upper_one_sided_95"] < 0.08
    rho_competitive = primary_success and rho_ridge["two_sided_95"][0] > -0.05
    rho_better_than_ridge = primary_success and rho_ridge["two_sided_95"][0] > 0.0
    return {
        "schema": "sys-datascience.covariance-rho-frozen-validation-verdict.v1",
        "question": "Does the frozen low canonical-vertex covariance rho arm enrich sys over a deterministic disjoint control on fresh random-product geometry?",
        "criteria_frozen_before_sys": {
            "rho_minus_control_estimate_at_least": 0.08,
            "rho_minus_control_two_sided_95_excludes_zero_upward": True,
            "both_seed_aggregates_positive": True,
            "minimum_positive_seed_pooled_bucket_effects": 7,
            "rho_competitive_after_primary_success_if_rho_minus_ridge_two_sided_95_lower_gt": -0.05,
            "rho_better_after_primary_success_if_rho_minus_ridge_two_sided_95_lower_gt": 0.0,
            "meaningful_negative_if_rho_minus_control_upper_one_sided_95_lt": 0.08,
            "sys_gt_1_escalation": "each unique row with sys > 1 requires independent geometry and capacity verification before interpretation",
        },
        "observed": {
            "previous_generated_maximum": PREVIOUS_GENERATED_MAXIMUM,
            "rho_minus_control": rho_control,
            "rho_minus_ridge": rho_ridge,
            "ridge_minus_control_replication": ridge_control,
            "seed_aggregates_rho_minus_control": seed_effects,
            "seed_pooled_bucket_effects_rho_minus_control": bucket_effects,
            "positive_seed_pooled_bucket_effects": sum(value > 0.0 for value in bucket_effects.values()),
            "leave_one_bucket_out_rho_minus_control": leave_one_bucket_out,
            "arms": arm_outputs,
            "rho_ridge_overlap": len(rho & ridge),
            "rho_ridge_jaccard": len(rho & ridge) / len(rho | ridge),
            "control_disjoint": control.isdisjoint(rho | ridge),
            "unique_evaluated_rows": len(values),
            "unique_sys_gt_1": sum(value > 1.0 for value in values.values()),
        },
        "verdict": {
            "primary_success": primary_success,
            "meaningful_negative": negative,
            "rho_competitive_with_ridge": rho_competitive,
            "rho_better_than_ridge": rho_better_than_ridge,
            "requires_sys_gt_1_escalation": any(value > 1.0 for value in values.values()),
        },
        "input_sha256": {
            "frozen_manifest": sha256(manifest_path),
            "sys_evaluation_caches": {str(path): sha256(path) for path in evaluation_paths},
        },
        "boundary": "Same-generator prospective selection evidence only. The reader does not authorize a direction flip, a subset rule, a capacity theorem, or transfer beyond the frozen random-product height law.",
        "strata": stratum_rows,
    }


def synthetic_fixture(directory: Path) -> tuple[Path, Path]:
    manifest_path = directory / "manifest.jsonl"
    evaluations_path = directory / "evaluations.jsonl"
    manifest_rows = []
    evaluation_rows = []
    stratum_number = 0
    for seed in EXPECTED_SEEDS:
        for bucket_number in range(EXPECTED_BUCKETS):
            bucket = f"bucket-{bucket_number}"
            # Exactly 64 rho/ridge overlaps: four in each of the first four
            # strata and three in every remaining stratum.
            overlap_count = 4 if stratum_number < 4 else 3
            memberships = ([RHO_ID, RIDGE_ID], 0.30, overlap_count)
            arm_specs = (
                memberships,
                ([RHO_ID], 0.30, 25 - overlap_count),
                ([RIDGE_ID], 0.20, 25 - overlap_count),
                ([CONTROL_ID], 0.10, 25),
            )
            for arms, sys_value, count in arm_specs:
                for index in range(count):
                    candidate_id = f"synthetic:{seed}:{bucket}:{'+'.join(arms)}:{index}"
                    selected = {
                        "candidate_id": candidate_id,
                        "poly_id": f"poly:{candidate_id}",
                        "bucket_id": bucket,
                        "source": {"seed": seed},
                        "selection_ids": arms,
                        "baseline_ids": [],
                        "selection_feature": "vertex_covariance_rho",
                        "selection_direction": "low",
                        "selection_feature_value": float(index),
                        "selection_rule_values": [],
                    }
                    manifest_rows.append(selected)
                    evaluation_rows.append({**selected, "sys": sys_value, "capacity": 1.0, "bounces": 2})
            stratum_number += 1
    manifest_path.write_text("".join(json.dumps(row) + "\n" for row in manifest_rows))
    evaluations_path.write_text("".join(json.dumps(row) + "\n" for row in evaluation_rows))
    return manifest_path, evaluations_path


def self_test() -> None:
    with tempfile.TemporaryDirectory() as temporary:
        manifest, evaluations = synthetic_fixture(Path(temporary))
        verdict = analyze(manifest, [evaluations])
        assert verdict["verdict"]["primary_success"]
        assert verdict["verdict"]["rho_better_than_ridge"]
        assert verdict["observed"]["unique_evaluated_rows"] == 1436
        assert verdict["observed"]["positive_seed_pooled_bucket_effects"] == 10


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--evaluation-cache", type=Path, action="append")
    parser.add_argument("--out-dir", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("self-test passed")
        return
    if not args.manifest or not args.evaluation_cache or not args.out_dir:
        parser.error("--manifest, --evaluation-cache, and --out-dir are required")
    if args.out_dir.exists() and any(args.out_dir.iterdir()):
        raise SystemExit(f"refusing to overwrite nonempty output directory: {args.out_dir}")
    args.out_dir.mkdir(parents=True, exist_ok=True)
    verdict = analyze(args.manifest, args.evaluation_cache)
    (args.out_dir / "covariance-rho-validation-verdict.json").write_text(
        json.dumps(verdict, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps(verdict["verdict"], sort_keys=True))


if __name__ == "__main__":
    main()
