#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Print on-demand guard facts for sys-landscape datascience tables."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "tables_dir",
        type=Path,
        help="Directory containing the retained sys-landscape datascience JSONL tables.",
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


def load_json(path: Path) -> dict[str, Any]:
    with path.open() as handle:
        row = json.load(handle)
    if not isinstance(row, dict):
        raise SystemExit(f"{path} must contain a JSON object")
    return row


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


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


FORBIDDEN_ACTIVE_POLYTOPE_FIELDS = {
    "capacity",
    "volume",
    "dual_vertex_count",
    "dual_vertices_f64",
}

FORBIDDEN_ACTIVE_POLYTOPE_PREFIXES = (
    "geom_",
    "edge_length_",
    "facet_volume_",
    "ridge_euclidean_area_",
    "ridge_symp_over_euclidean_area_",
    "allpair_",
    "omega_",
    "ridge_abs_omega_",
    "ridge_abs_normalized_omega_",
    "transition_",
)


def forbidden_active_polytope_fields(rows: list[dict[str, Any]]) -> list[str]:
    keys = {key for row in rows for key in row}
    return sorted(
        key
        for key in keys
        if key in FORBIDDEN_ACTIVE_POLYTOPE_FIELDS
        or key.startswith(FORBIDDEN_ACTIVE_POLYTOPE_PREFIXES)
    )


def fingerprint(tables_dir: Path) -> dict[str, Any]:
    polytope_path = tables_dir / "polytope-table.jsonl"
    provenance_path = tables_dir / "polytope-provenance-table.jsonl"
    polytope_rows = load_jsonl(polytope_path)
    provenance_rows = load_jsonl(provenance_path)
    sys_values = [float(row["sys"]) for row in polytope_rows]
    forbidden_fields = forbidden_active_polytope_fields(polytope_rows)
    require(
        not forbidden_fields,
        "active polytope table contains non-invariant/legacy fields: "
        + ", ".join(forbidden_fields[:20]),
    )
    hashes = {
        "polytope-table.jsonl": sha256(polytope_path),
        "polytope-provenance-table.jsonl": sha256(provenance_path),
    }
    max_sys = max(sys_values) if sys_values else None
    sys_gt_one_count = sum(1 for value in sys_values if value > 1.0)
    data = {
        "tables_dir": str(tables_dir),
        "polytope_rows": len(polytope_rows),
        "provenance_rows": len(provenance_rows),
        "polytope_union_field_count": union_field_count(polytope_rows),
        "active_polytope_forbidden_field_count": len(forbidden_fields),
        "provenance_union_field_count": union_field_count(provenance_rows),
        "dataset_counts": count_by(provenance_rows, "dataset"),
        "polytope_capacity_source_counts": count_by(polytope_rows, "capacity_source"),
        "max_sys": max_sys,
        "sys_gt_one_count": sys_gt_one_count,
        "sha256": hashes,
    }
    stats_path = tables_dir / "prepare-stats.json"
    if stats_path.exists():
        stats = load_json(stats_path)
        require(
            stats.get("polytope_rows") == len(polytope_rows),
            "prepare-stats polytope_rows mismatch",
        )
        require(
            stats.get("provenance_rows") == len(provenance_rows),
            "prepare-stats provenance_rows mismatch",
        )
        require(stats.get("sys_gt_one") == sys_gt_one_count, "prepare-stats sys_gt_one mismatch")
        if max_sys is None:
            require(stats.get("max_sys") is None, "prepare-stats max_sys mismatch")
        else:
            require(
                abs(float(stats.get("max_sys")) - max_sys) <= 1e-12,
                "prepare-stats max_sys mismatch",
            )
        require(
            stats.get("wall_time_ms", 0.0) > 0.0,
            "prepare-stats wall_time_ms must be positive",
        )
        data["prepare_stats_present"] = True
    else:
        data["prepare_stats_present"] = False
    return data


def print_markdown(data: dict[str, Any]) -> None:
    print("# Dataset Fingerprint")
    print()
    print(f"- tables dir: `{data['tables_dir']}`")
    print(f"- polytope rows: `{data['polytope_rows']}`")
    print(f"- provenance rows: `{data['provenance_rows']}`")
    print(f"- prepare stats present: `{data['prepare_stats_present']}`")
    print(f"- polytope union fields: `{data['polytope_union_field_count']}`")
    print(
        "- active polytope forbidden fields: "
        f"`{data['active_polytope_forbidden_field_count']}`"
    )
    print(f"- provenance union fields: `{data['provenance_union_field_count']}`")
    print(f"- max `sys`: `{data['max_sys']}`")
    print(f"- `sys > 1` rows: `{data['sys_gt_one_count']}`")
    print("- dataset counts:")
    for key, value in data["dataset_counts"].items():
        print(f"  - `{key}`: `{value}`")
    print("- polytope capacity-source counts:")
    for key, value in data["polytope_capacity_source_counts"].items():
        print(f"  - `{key}`: `{value}`")
    print("- sha256:")
    for key, value in data["sha256"].items():
        print(f"  - `{key}`: `{value}`")


def main() -> None:
    args = parse_args()
    data = fingerprint(args.tables_dir)
    if args.format == "json":
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        print_markdown(data)


if __name__ == "__main__":
    main()
