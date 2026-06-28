#!/usr/bin/env python3
"""Select a compact deterministic facet-count panel from the prepared table."""

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
DEFAULT_TABLE = ROOT / "experiments" / "sys-datascience" / "prepare" / "polytope-table.jsonl"
DEFAULT_OUT = Path(__file__).resolve().parent / "polytope-panel.jsonl"


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--polytope-table", type=Path, default=DEFAULT_TABLE)
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--facet-counts", default="6,10,12")
    parser.add_argument("--rows-per-facet", type=int, default=2)
    return parser.parse_args()


def load_jsonl(path):
    with path.open() as handle:
        for line in handle:
            if line.strip():
                yield json.loads(line)


def display_path(path):
    path = path.resolve()
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def main():
    args = parse_args()
    facet_counts = [int(value) for value in args.facet_counts.split(",") if value]
    by_facet = {facet_count: [] for facet_count in facet_counts}

    for row in load_jsonl(args.polytope_table):
        facet_count = int(row["facet_count"])
        if facet_count in by_facet:
            by_facet[facet_count].append(row)

    selected = []
    summary = {}
    for facet_count in facet_counts:
        rows = sorted(
            by_facet[facet_count],
            key=lambda row: (-float(row["sys"]), row["poly_id"]),
        )
        chosen = rows[: args.rows_per_facet]
        selected.extend(chosen)
        summary[str(facet_count)] = {
            "available_rows": len(rows),
            "selected_rows": len(chosen),
            "selected_poly_ids": [row["poly_id"] for row in chosen],
            "selected_sys": [row["sys"] for row in chosen],
        }

    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.open("w") as handle:
        for row in selected:
            json.dump(row, handle, sort_keys=True)
            handle.write("\n")

    summary_path = args.out.with_suffix(".summary.json")
    with summary_path.open("w") as handle:
        json.dump(
            {
                "source_table": display_path(args.polytope_table),
                "facet_counts": facet_counts,
                "rows_per_facet": args.rows_per_facet,
                "total_selected_rows": len(selected),
                "by_facet": summary,
            },
            handle,
            indent=2,
            sort_keys=True,
        )
        handle.write("\n")

    print(args.out)
    print(summary_path)


if __name__ == "__main__":
    main()
