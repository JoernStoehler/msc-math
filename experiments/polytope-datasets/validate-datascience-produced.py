#!/usr/bin/env python3
"""Validate run-local datascience producer outputs."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
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
    "known-hko-reference": "reference-samples.jsonl",
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


def plan_number(row: dict[str, Any], field: str, default: float) -> float:
    value = row.get(field, default)
    require(isinstance(value, int | float), f"plan row has non-numeric {field}: {row}")
    return float(value)


def plan_rows(plan: dict[str, Any], producer: str) -> list[dict[str, Any]]:
    if "buckets" in plan:
        rows = []
        for row in plan.get("buckets", []):
            require(isinstance(row, dict), f"plan bucket must be an object: {row!r}")
            if row.get("producer") == producer:
                rows.append(row)
        return rows
    key = "random_product" if producer == "random-product" else producer.replace("-", "_")
    rows = plan.get(key, [])
    require(isinstance(rows, list), f"plan field {key!r} must be a list")
    for row in rows:
        require(isinstance(row, dict), f"plan row must be an object: {row!r}")
    return rows


def expected_buckets_from_plan(plan_path: Path) -> dict[str, dict[tuple[Any, ...], int]]:
    plan = load_json(plan_path)
    expected: dict[str, dict[tuple[Any, ...], int]] = {
        "random": {},
        "random-product": {},
    }
    for row in plan_rows(plan, "random"):
        facet_count = row.get("facet_count")
        rows = row.get("rows")
        require(isinstance(facet_count, int), f"random plan row lacks integer facet_count: {row}")
        require(isinstance(rows, int) and rows >= 0, f"random plan row has invalid rows: {row}")
        key = (
            facet_count,
            plan_number(row, "h_min", 0.8),
            plan_number(row, "h_max", 1.2),
        )
        require(key not in expected["random"], f"duplicate random plan bucket: {key}")
        expected["random"][key] = rows
    for row in plan_rows(plan, "random-product"):
        k = row.get("k")
        m = row.get("m")
        rows = row.get("rows")
        require(isinstance(k, int) and isinstance(m, int), f"product plan row lacks k/m: {row}")
        require(
            isinstance(rows, int) and rows >= 0,
            f"product plan row has invalid rows: {row}",
        )
        key = (
            k,
            m,
            plan_number(row, "h_min", 0.8),
            plan_number(row, "h_max", 1.2),
        )
        require(key not in expected["random-product"], f"duplicate product plan bucket: {key}")
        expected["random-product"][key] = rows
    return expected


def source_number(source: dict[str, Any], field: str) -> float:
    value = source.get(field)
    require(isinstance(value, int | float), f"source has non-numeric {field}: {source}")
    return float(value)


def source_bucket_key(row: dict[str, Any], expected_producer: str) -> tuple[Any, ...]:
    source = sample_source(row, expected_producer)
    if expected_producer == "random":
        return (
            source["facet_count"],
            source_number(source, "h_min"),
            source_number(source, "h_max"),
        )
    if expected_producer == "random-product":
        return (
            source["k"],
            source["m"],
            source_number(source, "h_min"),
            source_number(source, "h_max"),
        )
    raise AssertionError(f"unexpected producer for plan validation: {expected_producer}")


def validate_plan_bucket_counts(
    rows: list[dict[str, Any]],
    expected_producer: str,
    expected: dict[tuple[Any, ...], int],
) -> dict[str, dict[str, int]]:
    counts: dict[tuple[Any, ...], int] = defaultdict(int)
    sample_indices: dict[tuple[Any, ...], set[int]] = defaultdict(set)
    for row in rows:
        key = source_bucket_key(row, expected_producer)
        counts[key] += 1
        source = row["source"]
        sample_index = source["sample_index"]
        require(
            isinstance(sample_index, int),
            f"sample {row.get('name')} has non-integer sample_index",
        )
        require(
            sample_index not in sample_indices[key],
            f"duplicate sample_index {sample_index} for {expected_producer} bucket {key}",
        )
        sample_indices[key].add(sample_index)

    require(set(counts) == set(expected), f"{expected_producer} buckets {dict(counts)} != {expected}")
    for key, expected_rows in expected.items():
        require(
            counts[key] == expected_rows,
            f"{expected_producer} bucket {key} rows {counts[key]} != expected {expected_rows}",
        )
        expected_indices = set(range(expected_rows))
        require(
            sample_indices[key] == expected_indices,
            f"{expected_producer} bucket {key} sample indices "
            f"{sorted(sample_indices[key])} != expected {sorted(expected_indices)}",
        )
    return {"bucket_count": len(expected), "row_count": sum(expected.values())}


def sample_source(row: dict[str, Any], expected_producer: str) -> dict[str, Any]:
    source = row.get("source")
    require(isinstance(source, dict), f"sample {row.get('name')} lacks source object")
    require(
        source.get("producer") == expected_producer,
        f"sample {row.get('name')} source producer {source.get('producer')!r} != {expected_producer!r}",
    )
    if expected_producer == "known-hko-reference":
        require(
            source.get("fixture") == "hko_pentagon",
            f"sample {row.get('name')} reference source fixture is not hko_pentagon",
        )
        require(
            source.get("role") == "reference_holdout",
            f"sample {row.get('name')} reference source role is not reference_holdout",
        )
        return source
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
    expected_plan_file: Path | None = None,
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
    reference_rows = (
        load_jsonl(produce_dir / PRODUCER_FILES["known-hko-reference"])
        if "known-hko-reference" in producers
        else []
    )
    sample_rows = [*random_rows, *product_rows, *reference_rows]

    expected = dict(EXPECTED[mode])
    plan_bucket_summary = None
    if expected_plan_file is not None:
        plan_buckets = expected_buckets_from_plan(expected_plan_file)
        plan_random_rows = sum(plan_buckets["random"].values())
        plan_product_rows = sum(plan_buckets["random-product"].values())
        if "random" in producers:
            if expected_random_rows is not None:
                require(
                    expected_random_rows == plan_random_rows,
                    f"--expected-random-rows {expected_random_rows} disagrees with "
                    f"--expected-plan-file total {plan_random_rows}",
                )
            expected_random_rows = plan_random_rows
        if "random-product" in producers:
            if expected_random_product_rows is not None:
                require(
                    expected_random_product_rows == plan_product_rows,
                    f"--expected-random-product-rows {expected_random_product_rows} disagrees with "
                    f"--expected-plan-file total {plan_product_rows}",
                )
            expected_random_product_rows = plan_product_rows
        plan_bucket_summary = {
            "random": validate_plan_bucket_counts(
                random_rows,
                "random",
                plan_buckets["random"],
            )
            if "random" in producers
            else {"bucket_count": 0, "row_count": 0},
            "random_product": validate_plan_bucket_counts(
                product_rows,
                "random-product",
                plan_buckets["random-product"],
            )
            if "random-product" in producers
            else {"bucket_count": 0, "row_count": 0},
        }
    if expected_random_rows is not None:
        expected[PRODUCER_FILES["random"]] = expected_random_rows
    if expected_random_product_rows is not None:
        expected[PRODUCER_FILES["random-product"]] = expected_random_product_rows
    for producer in producers:
        filename = PRODUCER_FILES[producer]
        if producer == "random":
            row_count = len(random_rows)
        elif producer == "random-product":
            row_count = len(product_rows)
        else:
            row_count = len(reference_rows)
            expected[filename] = 1
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
        require(sys == sys, f"computed payload {poly_id} has non-finite sys")
        require(row.get("capacity", 0.0) > 0.0, f"computed payload {poly_id} lacks capacity")
        require(row.get("volume", 0.0) > 0.0, f"computed payload {poly_id} lacks volume")
        require(
            row.get("capacity_method") == "certified-qp-minimizers-v1",
            f"computed payload {poly_id} does not use the current certified capacity method",
        )
        lower = float(row.get("capacity_lower", float("nan")))
        upper = float(row.get("capacity_upper", float("nan")))
        capacity = float(row["capacity"])
        require(
            0.0 < lower <= capacity <= upper,
            f"computed payload {poly_id} has invalid capacity bounds [{lower}, {upper}] around {capacity}",
        )
        require(row.get("capacity_exact"), f"computed payload {poly_id} lacks exact capacity")
        require(
            row.get("candidate_family") in {"general-hk", "product-closure-vertex"},
            f"computed payload {poly_id} lacks a recognized candidate family",
        )
        require(row.get("sigmas"), f"computed payload {poly_id} lacks sigmas")
        require(
            all(lower <= float(sigma["action"]) <= upper for sigma in row["sigmas"]),
            f"computed payload {poly_id} has a minimizing action outside its capacity bounds",
        )
        if row.get("backend") == "product":
            require(
                row["candidate_family"] == "product-closure-vertex",
                f"computed payload {poly_id} requested product output but used {row['candidate_family']}",
            )

    for row in sample_rows:
        poly_id = str(row["poly_id"])
        if row in product_rows:
            expected_producer = "random-product"
        elif row in reference_rows:
            expected_producer = "known-hko-reference"
        else:
            expected_producer = "random"
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
        stats.get("reference_rows", 0) == len(reference_rows),
        "produce-stats reference_rows mismatch",
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
    require(
        sorted(stats.get("producers", [])) == sorted(producers),
        "produce-stats producers mismatch",
    )
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

    result: dict[str, Any] = {
        "produce_dir": str(produce_dir),
        "mode": mode,
        "producers": ",".join(producers),
        "random_rows": len(random_rows),
        "random_product_rows": len(product_rows),
        "reference_rows": len(reference_rows),
        "computed_payload_rows": len(payload_rows),
        "cache_hits": stats.get("cache_hits"),
        "cache_misses": stats.get("cache_misses"),
        "max_sys": max_sys,
        "sys_gt_one": sum(1 for row in payload_rows if float(row["sys"]) > 1.0),
    }
    if plan_bucket_summary is not None:
        result["plan_bucket_summary"] = plan_bucket_summary
        result["expected_plan_file"] = str(expected_plan_file)
    return result


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
    parser.add_argument(
        "--expected-plan-file",
        type=Path,
        help="Plan JSON whose exact per-bucket row counts and sample indices must match.",
    )
    args = parser.parse_args()

    result = validate(
        args.produce_dir,
        args.mode,
        args.producers,
        args.expected_random_rows,
        args.expected_random_product_rows,
        args.expected_plan_file,
    )
    print("# Datascience Produce Validation")
    print()
    for key, value in result.items():
        if isinstance(value, dict | list):
            value = json.dumps(value, sort_keys=True)
        print(f"- {key.replace('_', ' ')}: `{value}`")


if __name__ == "__main__":
    main()
