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


def parse_producers(raw: str) -> list[str]:
    producers = [item.strip() for item in raw.split(",") if item.strip()]
    unknown = [item for item in producers if item not in PRODUCER_FILES]
    require(not unknown, f"unknown producers: {unknown}")
    require(bool(producers), "--producers must not be empty")
    return producers


def sample_source(row: dict[str, Any], expected_producer: str) -> dict[str, Any]:
    source = row.get("source")
    require(isinstance(source, dict), f"sample {row.get('name')} lacks source object")
    require(
        source.get("producer") == expected_producer,
        f"sample {row.get('name')} source producer {source.get('producer')!r} != {expected_producer!r}",
    )
    h_min = source.get("h_min")
    h_max = source.get("h_max")
    require(
        isinstance(h_min, int | float) and isinstance(h_max, int | float) and 0.0 < h_min < h_max,
        f"sample {row.get('name')} source has invalid height interval",
    )
    require(isinstance(source.get("seed"), int), f"sample {row.get('name')} source lacks seed")
    require(
        isinstance(source.get("sample_index"), int),
        f"sample {row.get('name')} source lacks sample_index",
    )
    require(
        isinstance(source.get("attempt"), int),
        f"sample {row.get('name')} source lacks attempt",
    )
    if expected_producer == "random":
        require(
            isinstance(source.get("facet_count"), int),
            f"sample {row.get('name')} random source lacks facet_count",
        )
    else:
        require(isinstance(source.get("k"), int), f"sample {row.get('name')} source lacks k")
        require(isinstance(source.get("m"), int), f"sample {row.get('name')} source lacks m")
        require(
            isinstance(source.get("bounces"), int),
            f"sample {row.get('name')} source lacks bounces",
        )
    return source


def validate(
    produce_dir: Path,
    mode: str,
    producers_raw: str,
    expected_random_rows: int | None = None,
    expected_random_product_rows: int | None = None,
) -> dict[str, Any]:
    payload_path = produce_dir / "computed-polytopes.jsonl"
    stats_path = produce_dir / "produce-stats.json"

    require(payload_path.exists(), f"missing required file: {payload_path}")
    require(stats_path.exists(), f"missing required file: {stats_path}")
    stats = load_json(stats_path)
    producers = parse_producers(producers_raw)

    for producer in producers:
        path = produce_dir / PRODUCER_FILES[producer]
        require(path.exists(), f"missing required file: {path}")

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

    expected = dict(EXPECTED[mode])
    if expected_random_rows is not None:
        expected[PRODUCER_FILES["random"]] = expected_random_rows
    if expected_random_product_rows is not None:
        expected[PRODUCER_FILES["random-product"]] = expected_random_product_rows
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
    require(
        len(payload_rows) == len(set(sample_ids)),
        f"payload rows {len(payload_rows)} != unique sample poly_ids {len(set(sample_ids))}",
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
        expected_producer = (
            "random-product" if row in product_rows else "random"
        )
        sample_source(row, expected_producer)
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
    require(stats.get("mode") == mode, "produce-stats mode mismatch")
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
        "mode": mode,
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
    parser.add_argument("--mode", choices=sorted(EXPECTED), required=True)
    parser.add_argument(
        "--producers",
        required=True,
        help="Comma-separated producer list.",
    )
    parser.add_argument("--expected-random-rows", type=int)
    parser.add_argument("--expected-random-product-rows", type=int)
    args = parser.parse_args()

    result = validate(
        args.produce_dir,
        args.mode,
        args.producers,
        args.expected_random_rows,
        args.expected_random_product_rows,
    )
    print("# Datascience Produce Validation")
    print()
    for key, value in result.items():
        print(f"- {key.replace('_', ' ')}: `{value}`")


if __name__ == "__main__":
    main()
