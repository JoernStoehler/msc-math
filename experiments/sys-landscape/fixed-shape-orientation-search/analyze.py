#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def rows(path: Path):
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def distribution_summary(path: Path, threshold: float):
    values = sorted(row["sys"] for row in rows(path))
    return {
        "path": str(path),
        "count": len(values),
        "maximum": values[-1],
        "count_strictly_above_rotated_product": sum(value > threshold for value in values),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument(
        "--generic-source",
        type=Path,
        default=Path("experiments/polytope-datasets/random.jsonl"),
    )
    parser.add_argument(
        "--product-source",
        type=Path,
        default=Path("experiments/polytope-datasets/random-product.jsonl"),
    )
    args = parser.parse_args()

    evaluations = rows(args.input)
    bodies = []
    for source_kind in sorted({row["source_kind"] for row in evaluations}):
        body_rows = [row for row in evaluations if row["source_kind"] == source_kind]
        identity = next(row for row in body_rows if row["stage"] == "identity")
        best = max(body_rows, key=lambda row: row["sys"])
        bodies.append(
            {
                "source_kind": source_kind,
                "source_name": identity["source_name"],
                "evaluation_count": len(body_rows),
                "identity_sys": identity["sys"],
                "best_sys": best["sys"],
                "delta": best["sys"] - identity["sys"],
                "best_theta": best["theta"],
                "best_phi": best["phi"],
                "best_stage": best["stage"],
            }
        )
    rotated_product = next(body for body in bodies if body["source_kind"] == "product")
    result = {
        "schema": "fixed-shape-orientation-search-analysis-v1",
        "input": str(args.input),
        "evaluation_count": len(evaluations),
        "bodies": bodies,
        "source_distributions": [
            distribution_summary(args.generic_source, rotated_product["best_sys"]),
            distribution_summary(args.product_source, rotated_product["best_sys"]),
        ],
        "interpretation": {
            "observation": "The selected product-source champion improves under a non-symplectic orthogonal rotation and exceeds every retained source row; the selected generic-source champion does not improve.",
            "claim_boundary": "Post-selection two-body scan; not a population comparison, proposer validation, or exhaustive orientation optimum.",
        },
    }
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
