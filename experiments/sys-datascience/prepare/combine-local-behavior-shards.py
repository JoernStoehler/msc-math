# /// script
# requires-python = ">=3.12"
# ///

"""Combine completed sys-local-behavior-produce shard directories."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


JSONL_FILES = [
    "local-behavior-basepoints.jsonl",
    "local-behavior-samples.jsonl",
    "local-behavior-branch-gradients.jsonl",
    "local-behavior-candidate-branch-gradients.jsonl",
]

UNIQUE_ROW_KEYS = {
    "local-behavior-basepoints.jsonl": ["basepoint_id"],
    "local-behavior-samples.jsonl": ["sample_id"],
    "local-behavior-branch-gradients.jsonl": ["basepoint_id", "orbit_index"],
    "local-behavior-candidate-branch-gradients.jsonl": ["basepoint_id", "orbit_index"],
}


def read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def iter_jsonl(path: Path):
    if not path.exists():
        raise SystemExit(f"required shard file is missing: {path}")
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if line:
                yield json.loads(line)


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")


def compatible(first: dict[str, Any], other: dict[str, Any], key: str) -> bool:
    return first.get(key) == other.get(key)


def shard_basepoint_ranges(shard_dirs: list[Path], stats_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    ranges = []
    for shard, stats in zip(shard_dirs, stats_rows):
        start = stats.get("basepoint_start")
        selected = stats.get("selected_basepoints")
        if not isinstance(start, int) or not isinstance(selected, int):
            raise SystemExit(f"shard missing integer basepoint_start/selected_basepoints: {shard}")
        if start < 0 or selected < 0:
            raise SystemExit(f"shard has negative basepoint range values: {shard}")
        ranges.append(
            {
                "shard_dir": str(shard),
                "basepoint_start": start,
                "basepoint_end_exclusive": start + selected,
                "basepoint_limit": stats.get("basepoint_limit"),
                "selected_basepoints": selected,
            }
        )
    return ranges


def validate_disjoint_ranges(ranges: list[dict[str, Any]]) -> None:
    sorted_ranges = sorted(ranges, key=lambda row: row["basepoint_start"])
    for left, right in zip(sorted_ranges, sorted_ranges[1:]):
        if left["basepoint_end_exclusive"] > right["basepoint_start"]:
            raise SystemExit(
                "overlapping shard basepoint ranges: "
                f"{left['shard_dir']} [{left['basepoint_start']}, {left['basepoint_end_exclusive']}) "
                f"and {right['shard_dir']} [{right['basepoint_start']}, {right['basepoint_end_exclusive']})"
            )


def collect_rows(shard_dirs: list[Path], name: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for shard in shard_dirs:
        rows.extend(iter_jsonl(shard / name))
    validate_unique_rows(name, rows)
    return rows


def validate_unique_rows(name: str, rows: list[dict[str, Any]]) -> None:
    keys = UNIQUE_ROW_KEYS[name]
    seen = set()
    for row in rows:
        try:
            key = tuple(json.dumps(row[field], sort_keys=True) for field in keys)
        except KeyError as error:
            raise SystemExit(f"{name} row missing key field {error.args[0]}") from error
        if key in seen:
            raise SystemExit(f"duplicate {name} row key {dict(zip(keys, key))}")
        seen.add(key)


def combine_stats(out_dir: Path, shard_dirs: list[Path], stats_rows: list[dict[str, Any]]) -> dict[str, Any]:
    first = stats_rows[0]
    for stats in stats_rows[1:]:
        for key in [
            "polytope_table",
            "provenance_table",
            "max_top_basepoints",
            "max_hash_basepoints",
            "source_datasets",
            "starts_per_source",
            "total_selected_basepoints",
            "radii",
            "branch_threshold_relative",
            "action_window_relative",
            "random_directions",
            "seed",
        ]:
            if not compatible(first, stats, key):
                raise SystemExit(f"incompatible shard stats key {key}: {shard_dirs[0]} vs shard")

    sample_status_counts = Counter()
    for stats in stats_rows:
        sample_status_counts.update(stats.get("sample_status_counts", {}))

    ranges = shard_basepoint_ranges(shard_dirs, stats_rows)
    validate_disjoint_ranges(ranges)

    return {
        "combined_from_shards": True,
        "shard_dirs": [str(path) for path in shard_dirs],
        "shard_basepoint_ranges": ranges,
        "shard_count": len(shard_dirs),
        "polytope_table": first.get("polytope_table"),
        "provenance_table": first.get("provenance_table"),
        "output_dir": str(out_dir),
        "max_top_basepoints": first.get("max_top_basepoints"),
        "max_hash_basepoints": first.get("max_hash_basepoints"),
        "source_datasets": first.get("source_datasets", []),
        "starts_per_source": first.get("starts_per_source"),
        "total_selected_basepoints": first.get("total_selected_basepoints"),
        "selected_basepoints": sum(stats.get("selected_basepoints", 0) for stats in stats_rows),
        "basepoint_start": None,
        "basepoint_limit": None,
        "radii": first.get("radii", []),
        "branch_threshold_relative": first.get("branch_threshold_relative"),
        "action_window_relative": first.get("action_window_relative"),
        "random_directions": first.get("random_directions"),
        "seed": first.get("seed"),
        "basepoint_rows": sum(stats.get("basepoint_rows", 0) for stats in stats_rows),
        "sample_rows": sum(stats.get("sample_rows", 0) for stats in stats_rows),
        "branch_gradient_rows": sum(stats.get("branch_gradient_rows", 0) for stats in stats_rows),
        "candidate_branch_gradient_rows": sum(
            stats.get("candidate_branch_gradient_rows", 0) for stats in stats_rows
        ),
        "computed_payload_rows": None,
        "failures": sum(stats.get("failures", 0) for stats in stats_rows),
        "sample_status_counts": dict(sorted(sample_status_counts.items())),
        "max_base_sys": max_optional(stats.get("max_base_sys") for stats in stats_rows),
        "max_target_sys": max_optional(stats.get("max_target_sys") for stats in stats_rows),
        "max_observed_delta_sys": max_optional(
            stats.get("max_observed_delta_sys") for stats in stats_rows
        ),
        "local_state_volume_ms": sum(stats.get("local_state_volume_ms", 0.0) for stats in stats_rows),
        "local_state_capacity_ms": sum(
            stats.get("local_state_capacity_ms", 0.0) for stats in stats_rows
        ),
        "shard_wall_time_ms_sum": sum(stats.get("wall_time_ms", 0.0) for stats in stats_rows),
    }


def max_optional(values) -> float | None:
    finite = [value for value in values if value is not None]
    return max(finite) if finite else None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", required=True, type=Path)
    parser.add_argument("shard_dirs", nargs="+", type=Path)
    args = parser.parse_args()

    args.out_dir.mkdir(parents=True, exist_ok=True)
    stats_rows = [read_json(path / "produce-stats.json") for path in args.shard_dirs]
    stats = combine_stats(args.out_dir, args.shard_dirs, stats_rows)

    for name in JSONL_FILES:
        rows = collect_rows(args.shard_dirs, name)
        write_jsonl(args.out_dir / name, rows)

    payloads_by_poly_id: dict[str, dict[str, Any]] = {}
    for shard in args.shard_dirs:
        for row in iter_jsonl(shard / "computed-polytopes.jsonl"):
            poly_id = row["poly_id"]
            existing = payloads_by_poly_id.get(poly_id)
            if existing is not None and existing != row:
                raise SystemExit(f"inconsistent computed-polytopes payload for poly_id {poly_id}")
            payloads_by_poly_id[poly_id] = row
    write_jsonl(args.out_dir / "computed-polytopes.jsonl", list(payloads_by_poly_id.values()))

    stats["computed_payload_rows"] = len(payloads_by_poly_id)
    with (args.out_dir / "produce-stats.json").open("w", encoding="utf-8") as handle:
        json.dump(stats, handle, indent=2, sort_keys=True)
        handle.write("\n")

    print(args.out_dir)


if __name__ == "__main__":
    main()
