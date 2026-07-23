#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///

import argparse
import json
from collections import defaultdict
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    rows = [json.loads(line) for line in args.input.read_text().splitlines()]
    bodies = []
    for kind in sorted({row["source_kind"] for row in rows}):
        selected = [row for row in rows if row["source_kind"] == kind]
        control = next(row for row in selected if row["stage"] == "compact-control")
        best = max(selected, key=lambda row: row["sys"])
        by_radius = defaultdict(list)
        for row in selected:
            by_radius[row["radius"]].append(row["sys"])
        bodies.append(
            {
                "source_kind": kind,
                "source_name": control["source_name"],
                "evaluation_count": len(selected),
                "compact_best_sys": control["sys"],
                "global_best_sys": best["sys"],
                "delta": best["sys"] - control["sys"],
                "best_stage": best["stage"],
                "best_radius": best["radius"],
                "best_condition_number": best["map_condition_number"],
                "radius_summary": [
                    {
                        "radius": radius,
                        "count": len(values),
                        "maximum_sys": max(values),
                        "mean_sys": sum(values) / len(values),
                    }
                    for radius, values in sorted(by_radius.items())
                ],
            }
        )
    result = {
        "schema": "fixed-shape-linear-search-analysis-v1",
        "input": str(args.input),
        "evaluation_count": len(rows),
        "bodies": bodies,
        "claim_boundary": (
            "Deterministic finite multiscale sample of the global normalized "
            "determinant-one quotient; not an exhaustive search of its "
            "noncompact radial coordinate or four angular coordinates."
        ),
    }
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
