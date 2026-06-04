#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Merge datascience LICCA fixed-F ascent shards into producer JSONL files."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


NAME_RE = re.compile(r"_(\d+)$")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--produce-dir",
        type=Path,
        default=Path(__file__).resolve().parent,
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write merged outputs. Without this flag, only print a review report.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as exc:
                raise SystemExit(f"{path}:{line_number}: invalid JSON: {exc}") from exc
    return rows


def name_index(row: dict[str, Any]) -> int:
    name = str(row.get("name", ""))
    match = NAME_RE.search(name)
    if match is None:
        raise SystemExit(f"cannot parse trailing seed index from row name: {name!r}")
    return int(match.group(1))


def row_key(row: dict[str, Any]) -> tuple[int, str]:
    return (name_index(row), str(row["name"]))


def trace_key(row: dict[str, Any]) -> tuple[int, str, int, int]:
    return (
        name_index(row),
        str(row["name"]),
        int(row.get("phase", 0)),
        int(row.get("iteration", 0)),
    )


def collect_paths(produce_dir: Path, canonical_name: str, shard_globs: list[str]) -> list[Path]:
    paths = [produce_dir / canonical_name]
    for pattern in shard_globs:
        paths.extend(sorted(produce_dir.glob(pattern)))
    return [path for path in paths if path.exists()]


def reject_trace_paths(paths: list[Path]) -> list[Path]:
    return [path for path in paths if not path.name.endswith("-trace.jsonl")]


def keep_trace_paths(paths: list[Path]) -> list[Path]:
    return [path for path in paths if path.name.endswith("-trace.jsonl")]


def dedup_rows(
    paths: list[Path],
    key_field: str,
    sort_key: Any,
) -> tuple[list[dict[str, Any]], dict[str, list[Path]]]:
    rows_by_key: dict[str, dict[str, Any]] = {}
    sources_by_key: dict[str, list[Path]] = {}
    for path in paths:
        for row in load_jsonl(path):
            key = key_field + ":" + repr(sort_key(row))
            previous = rows_by_key.get(key)
            if previous is not None and previous != row:
                raise SystemExit(f"conflicting duplicate row for {key!r} in {path}")
            rows_by_key[key] = row
            sources_by_key.setdefault(key, []).append(path)
    return sorted(rows_by_key.values(), key=sort_key), sources_by_key


def missing_indices(rows: list[dict[str, Any]], start: int, stop_inclusive: int) -> list[int]:
    seen = {name_index(row) for row in rows}
    return [idx for idx in range(start, stop_inclusive + 1) if idx not in seen]


def compact_indices(indices: list[int]) -> str:
    if not indices:
        return "[]"
    ranges: list[str] = []
    start = previous = indices[0]
    for value in indices[1:]:
        if value == previous + 1:
            previous = value
            continue
        ranges.append(f"{start}" if start == previous else f"{start}..{previous}")
        start = previous = value
    ranges.append(f"{start}" if start == previous else f"{start}..{previous}")
    return "[" + ", ".join(ranges) + "]"


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=False, separators=(",", ":")))
            handle.write("\n")


def report_family(
    label: str,
    rows: list[dict[str, Any]],
    paths: list[Path],
    expected_start: int,
    expected_stop: int,
) -> None:
    max_sys = max((float(row["final_sys"]) for row in rows), default=None)
    gt_one = [row["name"] for row in rows if float(row["final_sys"]) > 1.0]
    print(f"## {label}")
    print(f"- input files: `{len(paths)}`")
    for path in paths:
        print(f"  - `{path}`")
    print(f"- merged rows: `{len(rows)}`")
    print(f"- expected covered seed range: `{expected_start}..{expected_stop}`")
    missing = missing_indices(rows, expected_start, expected_stop)
    print(f"- missing expected seed count: `{len(missing)}`")
    print(f"- missing expected seeds: `{compact_indices(missing)}`")
    print(f"- max final_sys: `{max_sys}`")
    print(f"- final_sys > 1 rows: `{gt_one}`")
    print()


def main() -> None:
    args = parse_args()
    produce_dir = args.produce_dir

    general_paths = reject_trace_paths(collect_paths(
        produce_dir,
        "ascent.jsonl",
        [
            "licca-shards/general/general-shard-*.jsonl",
            "licca-shards/general-production-1024/general-shard-*.jsonl",
        ],
    ))
    general_trace_paths = keep_trace_paths(collect_paths(
        produce_dir,
        "ascent-trace.jsonl",
        [
            "licca-shards/general/general-shard-*-trace.jsonl",
            "licca-shards/general-production-1024/general-shard-*-trace.jsonl",
        ],
    ))
    product_paths = reject_trace_paths(collect_paths(
        produce_dir,
        "ascent-product.jsonl",
        [
            "licca-shards/product/product-shard-*.jsonl",
            "licca-shards/product-production-1024/product-shard-*.jsonl",
        ],
    ))
    product_trace_paths = keep_trace_paths(collect_paths(
        produce_dir,
        "ascent-product-trace.jsonl",
        [
            "licca-shards/product/product-shard-*-trace.jsonl",
            "licca-shards/product-production-1024/product-shard-*-trace.jsonl",
        ],
    ))

    general_rows, _ = dedup_rows(general_paths, "summary", row_key)
    general_trace_rows, _ = dedup_rows(general_trace_paths, "trace", trace_key)
    product_rows, _ = dedup_rows(product_paths, "summary", row_key)
    product_trace_rows, _ = dedup_rows(product_trace_paths, "trace", trace_key)

    print("# LICCA Ascent Shard Merge Report")
    print()
    report_family("general summary", general_rows, general_paths, 0, max(509, name_index(general_rows[-1]) if general_rows else 0))
    report_family("product summary", product_rows, product_paths, 0, max(511, name_index(product_rows[-1]) if product_rows else 0))
    print(f"## trace rows")
    print(f"- general trace rows: `{len(general_trace_rows)}`")
    print(f"- product trace rows: `{len(product_trace_rows)}`")
    print()

    if args.write:
        outputs = [
            (produce_dir / "ascent-licca-merged.jsonl", general_rows),
            (produce_dir / "ascent-licca-merged-trace.jsonl", general_trace_rows),
            (produce_dir / "ascent-product-licca-merged.jsonl", product_rows),
            (produce_dir / "ascent-product-licca-merged-trace.jsonl", product_trace_rows),
        ]
        for path, rows in outputs:
            write_jsonl(path, rows)
            print(f"wrote `{path}` rows=`{len(rows)}`")
    else:
        print("Dry run only. Pass `--write` to create `*-licca-merged*.jsonl` outputs.")


if __name__ == "__main__":
    main()
