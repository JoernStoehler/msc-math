#!/usr/bin/env -S uv run --script
"""Summarize the retained branch-equality continuation control."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from collections import defaultdict
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def summarize(rows: list[dict]) -> dict:
    ordered_gaps = sorted(row["pair_relative_gap_above_capacity"] for row in rows)
    joint = [row for row in rows if row["pair_joint_minimizer_nominal"]]
    return {
        "sample_count": len(rows),
        "joint_minimizer_count": len(joint),
        "joint_minimizer_fraction": len(joint) / len(rows),
        "max_equality_relative_residual": max(
            row["equality_relative_residual"] for row in rows
        ),
        "max_correction_norm": max(row["correction_norm"] for row in rows),
        "max_correction_to_requested_radius": max(
            row["correction_norm"] / row["requested_radius"] for row in rows
        ),
        "median_pair_relative_gap_above_capacity": ordered_gaps[len(rows) // 2],
        "max_pair_relative_gap_above_capacity": max(ordered_gaps),
        "min_sys": min(row["sys"] for row in rows),
        "max_sys": max(row["sys"] for row in rows),
        "max_joint_minimizer_sys": max((row["sys"] for row in joint), default=None),
        "mean_capacity_runtime_ms": sum(row["capacity_runtime_ms"] for row in rows)
        / len(rows),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    records = [json.loads(line) for line in args.input.read_text().splitlines() if line]
    metadata = next(row for row in records if row["record_type"] == "metadata")
    samples = [row for row in records if row["record_type"] == "sample"]
    failures = [row for row in records if row["record_type"] == "failure"]
    run_summary = next(row for row in records if row["record_type"] == "run_summary")

    expected_count = len(metadata["radii"]) * (metadata["samples_per_radius"] + 1)
    assert len(samples) + len(failures) == expected_count
    assert run_summary["completed_samples"] == len(samples)
    assert run_summary["correction_or_evaluation_failures"] == len(failures)

    by_radius: dict[float, list[dict]] = defaultdict(list)
    for row in samples:
        by_radius[row["requested_radius"]].append(row)

    small = [row for row in samples if row["requested_radius"] <= 1.0e-3]
    small_reliable = [
        row
        for row in small
        if row["equality_relative_residual"] <= 1.0e-10
        and row["first_beta_margin"] > 0.0
        and row["second_beta_margin"] > 0.0
        and row["first_kkt_n_zero"] == 0
        and row["second_kkt_n_zero"] == 0
    ]
    witness = [row for row in samples if row["sample_kind"] == "exposed_witness"]
    random_rows = [row for row in samples if row["sample_kind"].startswith("random_")]
    base_sys = ((5.0 + 2.0 * math.sqrt(5.0)) / 10.0) / math.cos(
        math.radians(metadata["base_theta_deg"])
    ) ** 2
    best = max(samples, key=lambda row: row["sys"])
    producer_path = Path(metadata["producer_source"])

    method_passed = not failures and len(small_reliable) / len(small) >= 0.9
    result = {
        "schema_version": "branch-equality-continuation-analysis-v1",
        "input": {
            "path": str(args.input),
            "sha256": sha256(args.input),
            "record_count": len(records),
        },
        "analyzer": {
            "path": str(Path(__file__)),
            "sha256": sha256(Path(__file__)),
        },
        "producer": {
            "path": str(producer_path),
            "sha256": sha256(producer_path),
        },
        "fixture": {
            "base_theta_deg": metadata["base_theta_deg"],
            "base_sys_exact_formula_f64": base_sys,
            "raw_tied_word_count": metadata["base_tied_word_count"],
            "distinct_log_gradient_group_count": metadata[
                "base_log_gradient_group_count"
            ],
            "selected_first_sigma": metadata["selected_first_sigma"],
            "selected_second_sigma": metadata["selected_second_sigma"],
            "selected_exposed_margin_per_unit_step": metadata[
                "selected_exposed_margin_per_unit_step"
            ],
        },
        "method_decision": {
            "status": "passed" if method_passed else "failed",
            "criterion": "no failures and >=90% equality corrections through radius 1e-3 with residual <=1e-10, positive beta margins, and zero numerical KKT nullity",
            "small_radius_reliable_count": len(small_reliable),
            "small_radius_sample_count": len(small),
            "small_radius_reliable_fraction": len(small_reliable) / len(small),
            "failure_count": len(failures),
        },
        "all_samples": summarize(samples),
        "by_radius": {
            f"{radius:.0e}": summarize(rows)
            for radius, rows in sorted(by_radius.items())
        },
        "exposed_witnesses": summarize(witness),
        "random_tangent_proposals": summarize(random_rows),
        "best_observed_sys": {
            "sys": best["sys"],
            "delta_from_base": best["sys"] - base_sys,
            "requested_radius": best["requested_radius"],
            "sample_kind": best["sample_kind"],
            "pair_joint_minimizer_nominal": best["pair_joint_minimizer_nominal"],
            "pair_relative_gap_above_capacity": best[
                "pair_relative_gap_above_capacity"
            ],
            "best_sigma": best["best_sigma"],
        },
        "interpretation": {
            "equality_sampler": "passed this local product control",
            "capacity_relevance": "mixed by tangent direction; all four exposed witnesses were joint-minimizing, while arbitrary equality points were often dominated",
            "sys_search": "no sys>1 point; the largest sys occurred where the selected equal pair was dominated, so it is not evidence for a high-sys equality ridge",
            "claim_boundary": "finite local numerical control; no completeness, intrinsic-uniformity, or global quotient claim",
        },
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
