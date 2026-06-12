#!/usr/bin/env python3
"""Validate run-local datascience producer outputs."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


EXPECTED = {
    "smoke": {
        "random-samples.jsonl": 8,
        "random-product-samples.jsonl": 10,
    },
    "production": {
        "random-samples.jsonl": 4096,
        "random-product-samples.jsonl": 10240,
    },
}


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def count_duplicates(values: list[str]) -> int:
    return len(values) - len(set(values))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def validate(produce_dir: Path, mode: str | None) -> dict[str, Any]:
    payload_path = produce_dir / "computed-polytopes.jsonl"
    random_path = produce_dir / "random-samples.jsonl"
    product_path = produce_dir / "random-product-samples.jsonl"

    for path in [payload_path, random_path, product_path]:
        require(path.exists(), f"missing required file: {path}")

    payload_rows = load_jsonl(payload_path)
    random_rows = load_jsonl(random_path)
    product_rows = load_jsonl(product_path)
    sample_rows = [*random_rows, *product_rows]

    if mode is not None:
        expected = EXPECTED[mode]
        require(
            len(random_rows) == expected["random-samples.jsonl"],
            f"random row count {len(random_rows)} != expected {expected['random-samples.jsonl']}",
        )
        require(
            len(product_rows) == expected["random-product-samples.jsonl"],
            "random-product row count "
            f"{len(product_rows)} != expected {expected['random-product-samples.jsonl']}",
        )

    payload_ids = [str(row["poly_id"]) for row in payload_rows]
    sample_ids = [str(row["poly_id"]) for row in sample_rows]
    sample_names = [str(row["name"]) for row in sample_rows]
    require(count_duplicates(payload_ids) == 0, "duplicate poly_id in computed-polytopes.jsonl")
    require(count_duplicates(sample_names) == 0, "duplicate sample name")
    require(count_duplicates(sample_ids) == 0, "duplicate sample poly_id")
    require(
        len(payload_rows) == len(sample_rows),
        f"payload rows {len(payload_rows)} != sample rows {len(sample_rows)}",
    )

    payload_by_id = {str(row["poly_id"]): row for row in payload_rows}
    for row in payload_rows:
        poly_id = str(row["poly_id"])
        sys = float(row["sys"])
        require(sys <= 1.0, f"computed payload {poly_id} has sys > 1: {sys}")
        require(row.get("capacity", 0.0) > 0.0, f"computed payload {poly_id} lacks capacity")
        require(row.get("volume", 0.0) > 0.0, f"computed payload {poly_id} lacks volume")
        require(row.get("sigmas"), f"computed payload {poly_id} lacks sigmas")
        require(row.get("orbit_scalars"), f"computed payload {poly_id} lacks orbit_scalars")

    for row in sample_rows:
        poly_id = str(row["poly_id"])
        require(poly_id in payload_by_id, f"sample {row['name']} missing payload {poly_id}")
        payload_sys = float(payload_by_id[poly_id]["sys"])
        sample_sys = float(row["sys"])
        require(
            abs(payload_sys - sample_sys) <= 1e-8,
            f"sample {row['name']} sys {sample_sys} disagrees with payload {payload_sys}",
        )

    return {
        "produce_dir": str(produce_dir),
        "mode": mode or "count-only",
        "random_rows": len(random_rows),
        "random_product_rows": len(product_rows),
        "computed_payload_rows": len(payload_rows),
        "max_sys": max(float(row["sys"]) for row in payload_rows) if payload_rows else None,
        "sys_gt_one": sum(1 for row in payload_rows if float(row["sys"]) > 1.0),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--produce-dir", type=Path, required=True)
    parser.add_argument("--mode", choices=sorted(EXPECTED), default=None)
    args = parser.parse_args()

    result = validate(args.produce_dir, args.mode)
    print("# Datascience Produce Validation")
    print()
    for key, value in result.items():
        print(f"- {key.replace('_', ' ')}: `{value}`")


if __name__ == "__main__":
    main()
