#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Print a small fingerprint for sys-landscape datascience tables."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "dataset_dir",
        type=Path,
        help="Directory containing polytope-table.jsonl and observation-table.jsonl.",
    )
    parser.add_argument(
        "--format",
        choices=["markdown", "json"],
        default="markdown",
        help="Output format.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def sha256(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def count_by(rows: list[dict[str, Any]], key: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        value = str(row.get(key, ""))
        counts[value] = counts.get(value, 0) + 1
    return dict(sorted(counts.items()))


def union_field_count(rows: list[dict[str, Any]]) -> int:
    return len({key for row in rows for key in row})


def fingerprint(dataset_dir: Path) -> dict[str, Any]:
    poly_path = dataset_dir / "polytope-table.jsonl"
    observation_path = dataset_dir / "observation-table.jsonl"
    polytopes = load_jsonl(poly_path)
    observations = load_jsonl(observation_path)
    sys_values = [float(row["sys"]) for row in polytopes]
    return {
        "dataset_dir": str(dataset_dir),
        "polytope_rows": len(polytopes),
        "observation_rows": len(observations),
        "polytope_union_field_count": union_field_count(polytopes),
        "observation_union_field_count": union_field_count(observations),
        "dataset_counts": count_by(observations, "dataset"),
        "max_sys": max(sys_values) if sys_values else None,
        "sys_gt_one_count": sum(1 for value in sys_values if value > 1.0),
        "sha256": {
            "polytope-table.jsonl": sha256(poly_path),
            "observation-table.jsonl": sha256(observation_path),
        },
    }


def print_markdown(data: dict[str, Any]) -> None:
    print("# Dataset Fingerprint")
    print()
    print(f"- dataset dir: `{data['dataset_dir']}`")
    print(f"- polytope rows: `{data['polytope_rows']}`")
    print(f"- observation rows: `{data['observation_rows']}`")
    print(f"- polytope union fields: `{data['polytope_union_field_count']}`")
    print(f"- observation union fields: `{data['observation_union_field_count']}`")
    print(f"- max `sys`: `{data['max_sys']}`")
    print(f"- `sys > 1` rows: `{data['sys_gt_one_count']}`")
    print("- dataset counts:")
    for key, value in data["dataset_counts"].items():
        print(f"  - `{key}`: `{value}`")
    print("- sha256:")
    for key, value in data["sha256"].items():
        print(f"  - `{key}`: `{value}`")


def main() -> None:
    args = parse_args()
    data = fingerprint(args.dataset_dir)
    if args.format == "json":
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print_markdown(data)


if __name__ == "__main__":
    main()
