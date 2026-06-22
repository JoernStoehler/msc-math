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
    deltas = [float(row["observed_delta_sys"]) for row in successes]
    by_radius: dict[str, list[float]] = defaultdict(list)
    for row in successes:
        by_radius[str(row["radius"])].append(float(row["observed_delta_sys"]))

    summary = {
        "panel_dir": str(args.panel_dir),
        "basepoint_rows": len(basepoint_rows),
        "sample_rows": len(sample_rows),
        "branch_gradient_rows": len(stats),
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
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()

