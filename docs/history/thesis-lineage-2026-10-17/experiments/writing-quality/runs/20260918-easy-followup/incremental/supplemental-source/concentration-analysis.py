#!/usr/bin/env python3
"""Analyze a frozen two-stage proposer against its stage-1 complement."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import defaultdict
from pathlib import Path


def read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def mean(values: list[float]) -> float:
    if not values:
        raise ValueError("mean requires at least one value")
    return sum(values) / len(values)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def selection_ids(plan: dict) -> tuple[str, str]:
    by_kind = {row["selection_kind"]: row["selection_id"] for row in plan["selections"]}
    return (
        by_kind["per_bucket_cascade_stage_1_comparator"],
        by_kind["per_bucket_two_stage_cascade"],
    )


def analyze(artifact_dir: Path) -> tuple[list[dict], dict]:
    plan_path = artifact_dir / "selection-plan.json"
    selection_path = artifact_dir / "selected-candidates-before-sys.jsonl"
    evaluation_path = artifact_dir / "sys-evaluation-cache.jsonl"
    plan = json.loads(plan_path.read_text())
    selection_rows = read_jsonl(selection_path)
    evaluation_rows = read_jsonl(evaluation_path)
    stage_1_id, cascade_id = selection_ids(plan)

    selection_candidate_ids = [row["candidate_id"] for row in selection_rows]
    evaluation_candidate_ids = [row["candidate_id"] for row in evaluation_rows]
    if len(set(selection_candidate_ids)) != len(selection_candidate_ids):
        raise ValueError("selected-candidates artifact contains duplicate candidate ids")
    if len(set(evaluation_candidate_ids)) != len(evaluation_candidate_ids):
        raise ValueError("sys-evaluation cache contains duplicate candidate ids")

    selected_memberships = {
        row["candidate_id"]: set(row.get("selection_ids", [])) for row in selection_rows
    }
    baseline_memberships = {
        row["candidate_id"]: set(row.get("baseline_ids", [])) for row in selection_rows
    }
    evaluation_by_id = {row["candidate_id"]: row for row in evaluation_rows}
    missing_current = set(selection_candidate_ids) - evaluation_by_id.keys()
    if missing_current:
        raise ValueError(f"missing sys evaluations for {len(missing_current)} current rows")
    current_evaluations = {
        candidate_id: evaluation_by_id[candidate_id]
        for candidate_id in selection_candidate_ids
    }
    selection_by_id = {row["candidate_id"]: row for row in selection_rows}
    for candidate_id, evaluated in current_evaluations.items():
        selected = selection_by_id[candidate_id]
        for field in ("poly_id", "bucket_id", "selection_ids", "baseline_ids"):
            if evaluated[field] != selected[field]:
                raise ValueError(f"{field} mismatch for current candidate {candidate_id}")
    sys_by_id = {
        candidate_id: float(row["sys"])
        for candidate_id, row in current_evaluations.items()
    }
    bucket_by_id = {
        candidate_id: row["bucket_id"]
        for candidate_id, row in current_evaluations.items()
    }

    stage_1 = {cid for cid, ids in selected_memberships.items() if stage_1_id in ids}
    cascade = {cid for cid, ids in selected_memberships.items() if cascade_id in ids}
    if not cascade < stage_1:
        raise ValueError("cascade must be a proper subset of the stage-1 comparator")
    complement = stage_1 - cascade
    cascade_baseline_id = f"{cascade_id}__baseline_rep_0"
    cascade_baseline = {
        cid for cid, ids in baseline_memberships.items() if cascade_baseline_id in ids
    }
    buckets = sorted({bucket_by_id[cid] for cid in stage_1})
    rows: list[dict] = []
    for bucket in [*buckets, "all"]:
        in_bucket = (
            (lambda cid: bucket_by_id[cid] == bucket)
            if bucket != "all"
            else (lambda cid: True)
        )
        cascade_values = [sys_by_id[cid] for cid in cascade if in_bucket(cid)]
        complement_values = [sys_by_id[cid] for cid in complement if in_bucket(cid)]
        baseline_values = [sys_by_id[cid] for cid in cascade_baseline if in_bucket(cid)]
        if not (cascade_values and complement_values and baseline_values):
            raise ValueError(f"empty comparison group for bucket {bucket}")
        rows.append(
            {
                "bucket": bucket,
                "cascade_n": len(cascade_values),
                "complement_n": len(complement_values),
                "matched_baseline_n": len(baseline_values),
                "cascade_mean_sys": mean(cascade_values),
                "complement_mean_sys": mean(complement_values),
                "matched_baseline_mean_sys": mean(baseline_values),
                "cascade_minus_complement_mean_sys": mean(cascade_values)
                - mean(complement_values),
                "cascade_minus_matched_baseline_mean_sys": mean(cascade_values)
                - mean(baseline_values),
                "cascade_max_sys": max(cascade_values),
                "complement_max_sys": max(complement_values),
                "matched_baseline_max_sys": max(baseline_values),
            }
        )

    bucket_rows = [row for row in rows if row["bucket"] != "all"]
    overall = next(row for row in rows if row["bucket"] == "all")
    positive_bucket_count = sum(
        row["cascade_minus_complement_mean_sys"] > 0.0 for row in bucket_rows
    )
    max_sys = max(sys_by_id.values())
    verdict = {
        "schema": "sys-datascience.ridge-concentration-validation.v1",
        "question": "Does low ridge-area max share add independent pre-target enrichment inside a frozen low ridge-sum tail on a new generated sample?",
        "criteria_frozen_before_sys": {
            "overall_cascade_minus_complement_mean_sys_positive": True,
            "minimum_positive_product_buckets": 7,
            "target_escalation": "any evaluated sys > 1",
        },
        "observed": {
            "positive_product_buckets": positive_bucket_count,
            "overall_cascade_minus_complement_mean_sys": overall[
                "cascade_minus_complement_mean_sys"
            ],
            "maximum_evaluated_sys": max_sys,
            "evaluated_sys_gt_1": sum(value > 1.0 for value in sys_by_id.values()),
        },
        "concentration_add_on_descriptively_validated": (
            positive_bucket_count >= 7
            and overall["cascade_minus_complement_mean_sys"] > 0.0
        ),
        "boundary": "Generated-candidate evidence for this frozen random-product rule only; not a mechanism, arbitrary-generator result, or calibrated hit rate.",
        "selection_ids": {"stage_1": stage_1_id, "cascade": cascade_id},
        "input_sha256": {
            "selection_plan": sha256(plan_path),
            "selected_candidates_before_sys": sha256(selection_path),
            "sys_evaluation_cache": sha256(evaluation_path),
        },
    }
    return rows, verdict


def write_tsv(path: Path, rows: list[dict]) -> None:
    columns = list(rows[0])
    lines = ["\t".join(columns)]
    lines.extend("\t".join(str(row[column]) for column in columns) for row in rows)
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifact_dir", type=Path)
    args = parser.parse_args()
    rows, verdict = analyze(args.artifact_dir)
    write_tsv(args.artifact_dir / "incremental-validation.tsv", rows)
    (args.artifact_dir / "validation-verdict.json").write_text(
        json.dumps(verdict, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps(verdict["observed"], sort_keys=True))
    print(
        "concentration_add_on_descriptively_validated="
        f"{str(verdict['concentration_add_on_descriptively_validated']).lower()}"
    )


if __name__ == "__main__":
    main()
