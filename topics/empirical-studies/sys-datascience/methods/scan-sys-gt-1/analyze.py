#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Baseline EDA scan for recorded rows with `sys > 1`."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable
import sys


HERE = Path(__file__).resolve().parent
TABLES_DIR = HERE.parents[2] / "polytope-invariant-table"
sys.path.append(str(HERE.parent / "_shared"))
from random_only import load_trusted_random_tables  # noqa: E402


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
    parser.add_argument(
        "--computed-polytopes",
        type=Path,
        action="append",
        default=None,
        help=(
            "Producer computed-polytopes JSONL file to scan for raw sys values. "
            "No producer files are scanned unless this option is passed."
        ),
    )
    parser.add_argument(
        "--random-only",
        action="store_true",
        help="Restrict the table scan to trusted random/product rows.",
    )
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


def iter_jsonl(path: Path) -> Iterable[tuple[int, dict[str, Any]]]:
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            row = json.loads(line)
            if not isinstance(row, dict):
                raise SystemExit(f"Expected JSON object in {path}:{line_number}")
            yield line_number, row


def provenance_by_poly_id(rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    by_poly_id: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_poly_id[str(row["poly_id"])].append(row)
    return dict(by_poly_id)


def source_label(provenance_rows: list[dict[str, Any]], fallback: object = "") -> str:
    datasets = sorted({str(row.get("dataset", "")) for row in provenance_rows})
    return ", ".join(dataset for dataset in datasets if dataset) or str(fallback or "-")


def scan_rows(rows: list[dict[str, Any]], *, id_field: str) -> list[dict[str, Any]]:
    for row in rows:
        value = row.get("sys")
        if not isinstance(value, int | float):
            raise SystemExit(f"Missing or non-numeric `sys` for {id_field}={row.get(id_field)}")

    return [row for row in rows if float(row["sys"]) > 1.0]


def scan_computed_polytope_file(path: Path) -> tuple[int, int, list[dict[str, Any]]]:
    scanned = 0
    positives: list[dict[str, Any]] = []
    for line_number, row in iter_jsonl(path):
        value = row.get("sys")
        if not isinstance(value, int | float):
            raise SystemExit(f"Missing or non-numeric `sys` in {path}:{line_number}")
        scanned += 1
        if float(value) > 1.0:
            positives.append(
                {
                    "source_file": str(path),
                    "line_number": line_number,
                    "result_id": row.get("result_id", ""),
                    "dataset": row.get("dataset", ""),
                    "role": row.get("role", ""),
                    "sys": float(value),
                }
            )
    return scanned, len(positives), positives


def main() -> None:
    args = parse_args()
    if args.random_only:
        computed_polytope_paths = args.computed_polytopes or []
        polytope_rows, provenance_rows = load_trusted_random_tables(args.polytope_table.parent)
    else:
        computed_polytope_paths = args.computed_polytopes or []
        polytope_rows = load_jsonl(args.polytope_table)
        provenance_rows = load_jsonl(args.provenance_table)
    provenance = provenance_by_poly_id(provenance_rows)

    positives = scan_rows(
        polytope_rows,
        id_field="poly_id",
    )

    source_summary: dict[str, dict[str, Any]] = {}
    for row in polytope_rows:
        poly_id = str(row["poly_id"])
        sys_value = float(row["sys"])
        label = source_label(provenance.get(poly_id, []), row.get("capacity_source", ""))
        entry = source_summary.setdefault(
            label,
            {"rows": 0, "sys_gt_1": 0},
        )
        entry["rows"] += 1
        if sys_value > 1.0:
            entry["sys_gt_1"] += 1

    computed_scan_rows = 0
    computed_scan_positive_rows = 0
    computed_scan_positives: list[dict[str, Any]] = []
    for path in computed_polytope_paths:
        scanned, positive_count, positives_for_file = scan_computed_polytope_file(path)
        computed_scan_rows += scanned
        computed_scan_positive_rows += positive_count
        computed_scan_positives.extend(positives_for_file)

    print("# scan-sys-gt-1")
    print()
    if args.random_only:
        print("- scope: `trusted random/product rows only`")
    print(f"- polytope rows: `{len(polytope_rows)}`")
    print(f"- provenance rows: `{len(provenance_rows)}`")
    print(f"- table rows with `sys > 1`: `{len(positives)}`")
    if computed_polytope_paths:
        print(f"- producer computed-polytope rows scanned: `{computed_scan_rows}`")
        print(f"- producer computed-polytope rows with `sys > 1`: `{computed_scan_positive_rows}`")
    if positives:
        print()
        print("## Positive Table Rows")
        print()
        print("| poly_id | sys | dataset |")
        print("| --- | ---: | --- |")
        for row in positives:
            poly_id = str(row["poly_id"])
            print(
                f"| `{poly_id}` | `{float(row['sys'])}` | "
                f"{source_label(provenance.get(poly_id, []), row.get('capacity_source', ''))} |"
            )
    if computed_scan_positives:
        print()
        print("## Positive Producer Computed-Polytope Rows")
        print()
        print("| source_file | line | result_id | sys | dataset | role |")
        print("| --- | ---: | --- | ---: | --- | --- |")
        for row in computed_scan_positives:
            print(
                f"| `{row['source_file']}` | `{row['line_number']}` | "
                f"`{row['result_id']}` | `{row['sys']}` | "
                f"{row['dataset']} | {row['role']} |"
            )
    print()
    print("## Source Summary")
    print()
    print("| dataset | rows | sys > 1 |")
    print("| --- | ---: | ---: |")
    for label in sorted(source_summary):
        entry = source_summary[label]
        print(f"| {label} | `{entry['rows']}` | `{entry['sys_gt_1']}` |")


if __name__ == "__main__":
    main()
