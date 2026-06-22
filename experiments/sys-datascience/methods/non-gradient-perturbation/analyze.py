#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Summarize non-gradient random-direction perturbation panels."""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from pathlib import Path
import sys

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import load_jsonl, write_json  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("panel_dir", type=Path)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    sample_rows = load_jsonl(args.panel_dir / "local-behavior-samples.jsonl")
    basepoint_rows = load_jsonl(args.panel_dir / "local-behavior-basepoints.jsonl")
    stats = load_jsonl(args.panel_dir / "local-behavior-branch-gradients.jsonl")

    successes = [row for row in sample_rows if row.get("target_sys") is not None]
    random_direction_successes = [
        row for row in successes if str(row.get("direction_label", "")).startswith("random_unit_direction_")
    ]
    direction_label_counts = Counter(str(row.get("direction_label", "")) for row in sample_rows)
    deltas = [float(row["observed_delta_sys"]) for row in successes]
    random_deltas = [float(row["observed_delta_sys"]) for row in random_direction_successes]
    by_radius: dict[str, list[float]] = defaultdict(list)
    random_by_radius: dict[str, list[float]] = defaultdict(list)
    for row in successes:
        by_radius[str(row["radius"])].append(float(row["observed_delta_sys"]))
    for row in random_direction_successes:
        random_by_radius[str(row["radius"])].append(float(row["observed_delta_sys"]))

    summary = {
        "panel_dir": str(args.panel_dir),
        "research_question": (
            "Smoke-check bounded perturbations whose basepoints and directions are not selected "
            "by observed sys improvements; only random_unit_direction_* rows are treated as the "
            "non-gradient subset."
        ),
        "basepoint_rows": len(basepoint_rows),
        "sample_rows": len(sample_rows),
        "branch_gradient_rows": len(stats),
        "direction_label_counts": dict(direction_label_counts),
        "status_counts": dict(Counter(str(row.get("status", "")) for row in sample_rows)),
        "successful_samples": len(successes),
        "max_target_sys": max((float(row["target_sys"]) for row in successes), default=None),
        "sys_gt_one_targets": sum(
            1 for row in successes if float(row.get("target_sys", 0.0)) > 1.0
        ),
        "positive_delta_samples": sum(1 for value in deltas if value > 0.0),
        "max_observed_delta_sys": max(deltas) if deltas else None,
        "radius_summary": {
            radius: {
                "rows": len(values),
                "mean_delta_sys": float(np.mean(values)),
                "max_delta_sys": float(np.max(values)),
                "positive_delta_fraction": float(np.mean(np.array(values) > 0.0)),
            }
            for radius, values in sorted(by_radius.items(), key=lambda item: float(item[0]))
        },
        "random_direction_subset": {
            "sample_rows": len(random_direction_successes),
            "sys_gt_one_targets": sum(
                1
                for row in random_direction_successes
                if float(row.get("target_sys", 0.0)) > 1.0
            ),
            "max_target_sys": max(
                (float(row["target_sys"]) for row in random_direction_successes), default=None
            ),
            "positive_delta_samples": sum(1 for value in random_deltas if value > 0.0),
            "max_observed_delta_sys": max(random_deltas) if random_deltas else None,
            "radius_summary": {
                radius: {
                    "rows": len(values),
                    "mean_delta_sys": float(np.mean(values)),
                    "max_delta_sys": float(np.max(values)),
                    "positive_delta_fraction": float(np.mean(np.array(values) > 0.0)),
                }
                for radius, values in sorted(random_by_radius.items(), key=lambda item: float(item[0]))
            },
        },
        "gradient_direction_rows_are_scope_diagnostics_only": (
            "Rows with single_near_active_gradient, negative_single_near_active_gradient, "
            "or near_active_maximin_direction labels are not counted as non-gradient "
            "perturbation evidence."
        ),
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# non-gradient-perturbation")
    print()
    print(f"- basepoints: `{summary['basepoint_rows']}`")
    print(f"- samples: `{summary['sample_rows']}`")
    print(f"- successful samples: `{summary['successful_samples']}`")
    print(f"- target rows with `sys > 1`: `{summary['sys_gt_one_targets']}`")
    print(f"- max target `sys`: `{summary['max_target_sys']}`")
    print(f"- max observed delta `sys`: `{summary['max_observed_delta_sys']}`")
    print(f"- random-direction samples: `{summary['random_direction_subset']['sample_rows']}`")
    print(
        "- random-direction target rows with `sys > 1`: "
        f"`{summary['random_direction_subset']['sys_gt_one_targets']}`"
    )
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
