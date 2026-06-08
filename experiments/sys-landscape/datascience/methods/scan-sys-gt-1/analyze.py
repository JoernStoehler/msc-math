#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Baseline EDA scan for retained table rows with `sys > 1`."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any


HERE = Path(__file__).resolve().parent
TABLES_DIR = HERE.parent.parent / "tables"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--polytope-table",
        type=Path,
        default=TABLES_DIR / "polytope-table.jsonl",
    )
    parser.add_argument(
        "--provenance-table",
        type=Path,
        default=TABLES_DIR / "polytope-provenance-table.jsonl",
    )
    parser.add_argument("--top-k", type=int, default=10)
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            row = json.loads(line)
            if not isinstance(row, dict):
                raise SystemExit(f"Expected JSON object in {path}:{line_number}")
            rows.append(row)
    return rows


def provenance_by_poly_id(rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    by_poly_id: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_poly_id[str(row["poly_id"])].append(row)
    return dict(by_poly_id)


def source_label(provenance_rows: list[dict[str, Any]]) -> str:
    datasets = sorted({str(row.get("dataset", "")) for row in provenance_rows})
    return ", ".join(dataset for dataset in datasets if dataset) or "-"


def main() -> None:
    args = parse_args()
    if args.top_k <= 0:
        raise SystemExit("--top-k must be positive")

    polytope_rows = load_jsonl(args.polytope_table)
    provenance_rows = load_jsonl(args.provenance_table)
    provenance = provenance_by_poly_id(provenance_rows)

    for row in polytope_rows:
        value = row.get("sys")
        if not isinstance(value, int | float):
            raise SystemExit(f"Missing or non-numeric `sys` for poly_id={row.get('poly_id')}")

    sorted_rows = sorted(polytope_rows, key=lambda row: float(row["sys"]), reverse=True)
    positives = [row for row in sorted_rows if float(row["sys"]) > 1.0]
    max_sys = float(sorted_rows[0]["sys"]) if sorted_rows else None

    source_summary: dict[str, dict[str, Any]] = {}
    for row in polytope_rows:
        poly_id = str(row["poly_id"])
        sys_value = float(row["sys"])
        label = source_label(provenance.get(poly_id, []))
        entry = source_summary.setdefault(
            label,
            {"rows": 0, "sys_gt_1": 0, "max_sys": None},
        )
        entry["rows"] += 1
        if sys_value > 1.0:
            entry["sys_gt_1"] += 1
        if entry["max_sys"] is None or sys_value > entry["max_sys"]:
            entry["max_sys"] = sys_value

    print("# scan-sys-gt-1")
    print()
    print(f"- polytope rows: `{len(polytope_rows)}`")
    print(f"- provenance rows: `{len(provenance_rows)}`")
    print(f"- rows with `sys > 1`: `{len(positives)}`")
    print(f"- max `sys`: `{max_sys}`")
    print()
    print(f"## Top {min(args.top_k, len(sorted_rows))} Rows")
    print()
    print("| rank | poly_id | sys | dataset |")
    print("| ---: | --- | ---: | --- |")
    for rank, row in enumerate(sorted_rows[: args.top_k], start=1):
        poly_id = str(row["poly_id"])
        print(
            f"| {rank} | `{poly_id}` | `{float(row['sys'])}` | "
            f"{source_label(provenance.get(poly_id, []))} |"
        )
    print()
    print("## Source Summary")
    print()
    print("| dataset | rows | sys > 1 | max sys |")
    print("| --- | ---: | ---: | ---: |")
    for label in sorted(source_summary):
        entry = source_summary[label]
        print(
            f"| {label} | `{entry['rows']}` | `{entry['sys_gt_1']}` | "
            f"`{entry['max_sys']}` |"
        )


if __name__ == "__main__":
    main()
