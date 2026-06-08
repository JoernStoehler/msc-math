#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Baseline EDA scan for recorded rows with `sys > 1`."""

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
    parser.add_argument(
        "--computed-polytopes",
        type=Path,
        action="append",
        default=[],
        help="Ascent producer computed-polytopes JSONL file to scan.",
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


def provenance_by_poly_id(rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    by_poly_id: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_poly_id[str(row["poly_id"])].append(row)
    return dict(by_poly_id)


def source_label(provenance_rows: list[dict[str, Any]]) -> str:
    datasets = sorted({str(row.get("dataset", "")) for row in provenance_rows})
    return ", ".join(dataset for dataset in datasets if dataset) or "-"


def scan_rows(rows: list[dict[str, Any]], *, id_field: str) -> list[dict[str, Any]]:
    for row in rows:
        value = row.get("sys")
        if not isinstance(value, int | float):
            raise SystemExit(f"Missing or non-numeric `sys` for {id_field}={row.get(id_field)}")

    return [row for row in rows if float(row["sys"]) > 1.0]


def main() -> None:
    args = parse_args()

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
        label = source_label(provenance.get(poly_id, []))
        entry = source_summary.setdefault(
            label,
            {"rows": 0, "sys_gt_1": 0},
        )
        entry["rows"] += 1
        if sys_value > 1.0:
            entry["sys_gt_1"] += 1

    computed_polytope_rows: list[dict[str, Any]] = []
    for path in args.computed_polytopes:
        computed_polytope_rows.extend(load_jsonl(path))

    computed_polytope_positives: list[dict[str, Any]] = []
    if computed_polytope_rows:
        computed_polytope_positives = scan_rows(
            computed_polytope_rows,
            id_field="result_id",
        )

    print("# scan-sys-gt-1")
    print()
    print(f"- polytope rows: `{len(polytope_rows)}`")
    print(f"- provenance rows: `{len(provenance_rows)}`")
    print(f"- rows with `sys > 1`: `{len(positives)}`")
    if positives:
        print()
        print("## Positive Rows")
        print()
        print("| poly_id | sys | dataset |")
        print("| --- | ---: | --- |")
        for row in positives:
            poly_id = str(row["poly_id"])
            print(
                f"| `{poly_id}` | `{float(row['sys'])}` | "
                f"{source_label(provenance.get(poly_id, []))} |"
            )
    print()
    print("## Source Summary")
    print()
    print("| dataset | rows | sys > 1 |")
    print("| --- | ---: | ---: |")
    for label in sorted(source_summary):
        entry = source_summary[label]
        print(f"| {label} | `{entry['rows']}` | `{entry['sys_gt_1']}` |")

    if computed_polytope_rows:
        print()
        print("## Computed-Polytope Inputs")
        print()
        print(f"- computed-polytope rows: `{len(computed_polytope_rows)}`")
        print(f"- rows with `sys > 1`: `{len(computed_polytope_positives)}`")
        if computed_polytope_positives:
            print()
            print("### Positive Computed-Polytope Rows")
            print()
            print("| result_id | sys | dataset | role |")
            print("| --- | ---: | --- | --- |")
            for row in computed_polytope_positives:
                print(
                    f"| `{row.get('result_id', '-')}` | `{float(row['sys'])}` | "
                    f"{row.get('dataset', '-')} | {row.get('role', '-')} |"
                )


if __name__ == "__main__":
    main()
