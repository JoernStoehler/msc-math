#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Summarize the current numerical exact-minimum surface for HKO2024.

Goal: produce a durable bookkeeping summary of the current exact minima from the
      committed numerical artifact so Packet 2 can reconcile stale count surfaces.
Input Artifacts: experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl
Output Artifacts: experiments/hko-local-maximum/exact-clarke/numerical-minima-summary.json
"""

from __future__ import annotations

import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
INPUT_PATH = (
    ROOT
    / "experiments"
    / "hko-local-maximum"
    / "gradient-analysis"
    / "hko-neighborhood-sensitivity.jsonl"
)
OUTPUT_PATH = Path(__file__).resolve().parent / "numerical-minima-summary.json"


def rounded_gradient_key(gradient: list[float]) -> tuple[float, ...]:
    return tuple(round(value, 12) for value in gradient)


def main() -> None:
    row = json.loads(INPUT_PATH.read_text().splitlines()[0])
    orbits = row["orbits"]
    gradients = row["per_orbit_d_sys_h"]
    best_action = min(orbit["action"] for orbit in orbits)

    exact_indices = [
        index
        for index, orbit in enumerate(orbits)
        if abs(orbit["action"] - best_action) < 1e-12
    ]

    subset_counter = Counter(tuple(orbits[index]["subset"]) for index in exact_indices)
    permutation_length_counter = Counter(
        len(orbits[index]["permutation"]) for index in exact_indices
    )
    gradient_counter = Counter(
        rounded_gradient_key(gradients[index]) for index in exact_indices
    )

    payload = {
        "input_artifact": str(INPUT_PATH.relative_to(ROOT)),
        "best_action": best_action,
        "n_exact_action_orbits": len(exact_indices),
        "n_distinct_visited_subsets": len(subset_counter),
        "n_distinct_height_gradients": len(gradient_counter),
        "permutation_length_histogram": {
            str(length): count for length, count in sorted(permutation_length_counter.items())
        },
        "visited_subset_histogram": [
            {"subset": list(subset), "count": count}
            for subset, count in subset_counter.most_common()
        ],
        "height_gradient_histogram": [
            {"gradient": list(gradient), "count": count}
            for gradient, count in gradient_counter.most_common()
        ],
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
