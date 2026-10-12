#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def rows(path: Path):
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def distribution_summary(path: Path, threshold: float):
    values = sorted(row["sys"] for row in rows(path))
    if not values:
        raise ValueError(f"empty source distribution: {path}")
    return {
        "path": str(path),
        "count": len(values),
        "maximum": values[-1],
        "count_strictly_above_rotated_product": sum(value > threshold for value in values),
    }


def observed_interpretation(bodies, distributions):
    observations = []
    for body in bodies:
        outcome = "improves" if body["delta"] > 0 else "does not improve"
        observations.append(
            f"The selected {body['source_kind']}-source body {outcome} "
            "in the supplied finite orientation scan."
        )
    if distributions:
        exceeds_all = all(
            body["best_sys"] > source["maximum"]
            for body in bodies
            if body["source_kind"] == "product"
            for source in distributions
        )
        observations.append(
            "The best scanned product value "
            + ("exceeds" if exceeds_all else "does not exceed")
            + " every row in both supplied source distributions."
        )
    return {
        "observation": " ".join(observations),
        "claim_boundary": "Post-selection finite scan of the supplied bodies; not a population comparison, proposer validation, or exhaustive orientation optimum.",
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument(
        "--generic-source",
        type=Path,
        default=Path("topics/empirical-studies/polytope-datasets/random.jsonl"),
    )
    parser.add_argument(
        "--product-source",
        type=Path,
        default=Path("topics/empirical-studies/polytope-datasets/random-product.jsonl"),
    )
    args = parser.parse_args()

    evaluations = rows(args.input)
    if not evaluations:
        raise ValueError(f"empty orientation scan: {args.input}")
    bodies = []
    for source_kind in sorted({row["source_kind"] for row in evaluations}):
        body_rows = [row for row in evaluations if row["source_kind"] == source_kind]
        if len({row["source_name"] for row in body_rows}) != 1:
            raise ValueError(f"multiple source bodies for {source_kind}")
        identities = [row for row in body_rows if row["stage"] == "identity"]
        if len(identities) != 1:
            raise ValueError(f"expected one identity evaluation for {source_kind}")
        identity = identities[0]
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
    rotated_product = next(
        (body for body in bodies if body["source_kind"] == "product"), None
    )
    distributions = (
        [
            distribution_summary(args.generic_source, rotated_product["best_sys"]),
            distribution_summary(args.product_source, rotated_product["best_sys"]),
        ]
        if rotated_product is not None
        else []
    )
    result = {
        "schema": "fixed-shape-orientation-search-analysis-v1",
        "input": str(args.input),
        "evaluation_count": len(evaluations),
        "bodies": bodies,
        "source_distributions": distributions,
        "interpretation": observed_interpretation(bodies, distributions),
    }
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
