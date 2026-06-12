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

PRODUCER_FILES = {
    "random": "random-samples.jsonl",
    "random-product": "random-product-samples.jsonl",
}


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def load_json(path: Path) -> dict[str, Any]:
    with path.open() as handle:
        row = json.load(handle)
    require(isinstance(row, dict), f"{path} must contain a JSON object")
    return row


def count_duplicates(values: list[str]) -> int:
    return len(values) - len(set(values))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def parse_producers(raw: str | None, produce_dir: Path) -> list[str]:
    if raw is not None:
        producers = [item.strip() for item in raw.split(",") if item.strip()]
        unknown = [item for item in producers if item not in PRODUCER_FILES]
        require(not unknown, f"unknown producers: {unknown}")
        require(bool(producers), "--producers must not be empty")
        return producers

    producers = [
        producer
        for producer, filename in PRODUCER_FILES.items()
        if (produce_dir / filename).exists()
    ]
    require(bool(producers), f"no producer sample files found in {produce_dir}")
    return producers


def validate(produce_dir: Path, mode: str | None, producers_raw: str | None) -> dict[str, Any]:
    producers = parse_producers(producers_raw, produce_dir)
    payload_path = produce_dir / "computed-polytopes.jsonl"
    stats_path = produce_dir / "produce-stats.json"

    require(payload_path.exists(), f"missing required file: {payload_path}")
    require(stats_path.exists(), f"missing required file: {stats_path}")
    for producer in producers:
        path = produce_dir / PRODUCER_FILES[producer]
        require(path.exists(), f"missing required file: {path}")

    stats = load_json(stats_path)
    payload_rows = load_jsonl(payload_path)
    random_rows = (
        load_jsonl(produce_dir / PRODUCER_FILES["random"]) if "random" in producers else []
    )
    product_rows = (
        load_jsonl(produce_dir / PRODUCER_FILES["random-product"])
        if "random-product" in producers
        else []
    )
    sample_rows = [*random_rows, *product_rows]

    if mode is not None:
        expected = EXPECTED[mode]
        for producer in producers:
            filename = PRODUCER_FILES[producer]
            row_count = len(random_rows) if producer == "random" else len(product_rows)
            require(
                row_count == expected[filename],
                f"{filename} row count {row_count} != expected {expected[filename]}",
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

    max_sys = max(float(row["sys"]) for row in payload_rows) if payload_rows else None
    require(stats.get("random_rows") == len(random_rows), "produce-stats random_rows mismatch")
    require(
        stats.get("random_product_rows") == len(product_rows),
        "produce-stats random_product_rows mismatch",
    )
    require(
        stats.get("computed_payload_rows") == len(payload_rows),
        "produce-stats computed_payload_rows mismatch",
    )
    require(stats.get("failures") == 0, "produce-stats failures must be zero")
    require(
        stats.get("cache_hits", 0) + stats.get("cache_misses", 0) == len(sample_rows),
        "produce-stats cache_hits + cache_misses must equal sample rows",
    )
    require(stats.get("mode") == (mode or stats.get("mode")), "produce-stats mode mismatch")
    require(stats.get("producers") == producers, "produce-stats producers mismatch")
    require(
        stats.get("cache_miss_volume_ms", -1.0) >= 0.0,
        "produce-stats cache_miss_volume_ms must be nonnegative",
    )
    require(
        stats.get("cache_miss_capacity_ms", -1.0) >= 0.0,
        "produce-stats cache_miss_capacity_ms must be nonnegative",
    )
    require(stats.get("wall_time_ms", 0.0) > 0.0, "produce-stats wall_time_ms must be positive")
    if max_sys is None:
        require(stats.get("max_sys") is None, "produce-stats max_sys mismatch")
    else:
        require(
            abs(float(stats.get("max_sys")) - max_sys) <= 1e-12,
            "produce-stats max_sys mismatch",
        )

    return {
        "produce_dir": str(produce_dir),
        "mode": mode or "count-only",
        "producers": ",".join(producers),
        "random_rows": len(random_rows),
        "random_product_rows": len(product_rows),
        "computed_payload_rows": len(payload_rows),
        "cache_hits": stats.get("cache_hits"),
        "cache_misses": stats.get("cache_misses"),
        "max_sys": max_sys,
        "sys_gt_one": sum(1 for row in payload_rows if float(row["sys"]) > 1.0),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--produce-dir", type=Path, required=True)
    parser.add_argument("--mode", choices=sorted(EXPECTED), default=None)
    parser.add_argument(
        "--producers",
        help="Comma-separated producer list. If omitted, validate sample files present in the directory.",
    )
    args = parser.parse_args()

    result = validate(args.produce_dir, args.mode, args.producers)
    print("# Datascience Produce Validation")
    print()
    for key, value in result.items():
        print(f"- {key.replace('_', ' ')}: `{value}`")


if __name__ == "__main__":
    main()
